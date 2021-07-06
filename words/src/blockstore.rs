use std::{
    convert::{TryFrom, TryInto},
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
    iter::once,
    mem,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};
use deku::prelude::*;
use log::{debug, error};
use zstd::{
    dict::{from_continuous, DecoderDictionary, EncoderDictionary},
    Decoder, Encoder,
};

pub struct Store {
    pub dir: PathBuf,
    pub dict_en: Option<EncoderDictionary<'static>>,
    pub dict_de: Option<DecoderDictionary<'static>>,
}

impl Store {
    pub fn commit(&mut self, block: &mut Block, n: usize) -> Result<()> {
        let block = mem::take(block);

        let dict = if let Some(ref d) = self.dict_en {
            d
        } else {
            // create dictionary from first block
            let sample_sizes: Vec<usize> = once(&0_u64)
                .chain(block.starts.iter())
                .zip(block.starts.iter())
                .fuse()
                .map(|(start, end)| end - start)
                .map(|n| usize::try_from(n).unwrap())
                .collect();

            let dict_data = from_continuous(&block.data, &sample_sizes, 150_000)?;
            let mut file = File::create(self.dir.join("zst.dictionary"))?;
            file.write_all(&dict_data)?;
            self.dict_en = Some(EncoderDictionary::copy(&dict_data, 3));
            self.dict_de = Some(DecoderDictionary::copy(&dict_data));
            self.dict_en.as_ref().unwrap()
        };

        let file = File::create(self.dir.join(format!("{}.zst", n)))?;
        let mut target = Encoder::with_prepared_dictionary(file, dict)?;

        let block_bytes = block.finish()?;
        target.write_all(&block_bytes)?;
        target.finish()?;

        Ok(())
    }

    pub fn new(dir: impl AsRef<Path>) -> Self {
        Self {
            dir: dir.as_ref().into(),
            dict_en: None,
            dict_de: None,
        }
    }

    pub fn create(&self) -> Result<()> {
        if !self.dir.exists() {
            create_dir_all(&self.dir)?;
        }

        Ok(())
    }

    pub fn open(&mut self) -> Result<()> {
        let mut dict = File::open(self.dir.join("zst.dictionary"))?;
        let mut dict_bytes = Vec::with_capacity(dict.metadata()?.len().try_into()?);
        dict.read_to_end(&mut dict_bytes)?;
        self.dict_de = Some(DecoderDictionary::copy(&dict_bytes));
        debug!("loaded dictionary size={}", dict_bytes.len());

        Ok(())
    }

    pub fn blocks(&self) -> Result<Vec<PathBuf>> {
        let mut blocks = Vec::new();
        for d in self.dir.read_dir()? {
            let d = d?;
            if !d.file_type()?.is_file() {
                continue;
            }
            if !d.path().display().to_string().ends_with(".zst") {
                continue;
            }

            blocks.push(d.path());
        }

        Ok(blocks)
    }

    /// reads a block
    ///
    /// panics if decoder dictionary isn't ready (call `open()` first)
    pub fn read_block(&self, path: impl AsRef<Path>) -> Result<Block> {
        let file = File::open(path)?;
        let filelen: usize = file.metadata()?.len().try_into()?;
        let file = BufReader::new(file);
        let mut source =
            Decoder::with_prepared_dictionary(file, self.dict_de.as_ref().unwrap())?;
        let mut block_bytes = Vec::with_capacity(filelen * 2);
        source.read_to_end(&mut block_bytes)?;

        debug!("loaded block size={}", block_bytes.len());
        let block = Block::from_bytes((&block_bytes, 0))?;
        Ok(block.1)
    }
}

#[derive(Debug, Default, DekuRead, DekuWrite)]
pub struct Block {
    #[deku(update = "self.starts.len()")]
    pub n: u64,
    #[deku(count = "n")]
    pub starts: Vec<u64>,
    #[deku(bits_read = "deku::rest.len()")]
    pub data: Vec<u8>,
}

impl Block {
    pub fn add(&mut self, entry: Entry) -> Result<()> {
        let data = entry.to_bytes()?;
        self.n += 1;
        self.starts.push(u64::try_from(self.data.len())?);
        self.data.extend(data);
        Ok(())
    }

    pub fn finish(self) -> Result<Vec<u8>> {
        Ok(self.to_bytes()?)
    }

    pub fn entry(&self, n: u64) -> Result<Entry> {
        let start = *self
            .starts
            .get(usize::try_from(n)?)
            .ok_or_else(|| eyre!("no such entry: {}", n))?;
        let start: usize = start.try_into()?;

        let title_len =usize::try_from( u32::from_ne_bytes(self.data[start..start+4].try_into()?))?;
        let body_len = usize::try_from(u32::from_ne_bytes(self.data[start+4..start+8].try_into()?))?;
        let entry_slice = &self.data[start..(start+8+title_len+body_len)];

        debug!("reading entry {}/{} start={}", n, self.n, start);
        let entry = Entry::from_bytes((entry_slice, 0)).map_err(|err| {
            error!("entry {} t={} b={} data={:?}", n, title_len, body_len, entry_slice);
            err
        })?;
        Ok(entry.1)
    }
}

#[derive(Debug, DekuRead, DekuWrite)]
pub struct Entry {
    pub title_len: u32,
    pub body_len: u32,
    #[deku(bytes_read = "title_len")]
    pub title: Vec<u8>,
    #[deku(bytes_read = "body_len")]
    pub body: Vec<u8>,
}

impl Entry {
    pub fn new(title: &str, body: &str) -> Self {
        let title = title.as_bytes();
        let body = body.as_bytes();

        Self {
            title_len: u32::try_from(title.len()).unwrap(),
            body_len: u32::try_from(body.len()).unwrap(),
            title: title.into(),
            body: body.into(),
        }
    }

    pub fn open(self) -> (String, String) {
        let title = String::from_utf8(self.title).unwrap();
        let body = String::from_utf8(self.body).unwrap();
        (title, body)
    }
}
