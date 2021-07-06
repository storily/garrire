use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

use color_eyre::eyre::Result;
use structopt::StructOpt;
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Schema, INDEXED, STORED, TEXT},
    DocAddress, Index, Score,
};
use xml::reader::XmlEvent;

mod blockstore;

#[derive(StructOpt, Debug, Clone)]
enum Action {
    QueryExtract {
        #[structopt(short, long, default_value = "extract-store")]
        store_dir: PathBuf,

        searches: Vec<String>,
    },

    Extract {
        #[structopt(short = "f", long)]
        dump: PathBuf,

        #[structopt(short, long, default_value = "extract-store")]
        store_dir: PathBuf,
    },

    Index {
        #[structopt(short = "f", long)]
        dump: PathBuf,

        #[structopt(short, long)]
        index: PathBuf,
    },

    Query {
        #[structopt(short, long)]
        index: PathBuf,

        #[structopt(short = "n", long, default_value = "20")]
        limit: usize,

        search: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;
    match Action::from_args() {
        Action::QueryExtract { store_dir, searches } => {
            use rayon::prelude::*;
            use std::sync::Arc;

            let mut store = blockstore::Store::new(store_dir);
            store.open()?;

            store.blocks()?
                .par_iter()
                .flat_map(|path| {
                    let block = store.read_block(path).expect("error reading block");
                    let block = Arc::new(block);
                    (0..block.n).into_par_iter().map(move |n| {
                        let block = block.clone();
                        block.entry(n).expect("error parsing entry").open()
                    })
                })
                .filter(move |(_, text)| searches.iter().all(|search| {
                    if search.starts_with('~') {
                        !text.contains(search)
                    } else {
                        text.contains(search)
                    }
                }))
                .for_each(|(title, _)| println!("{}", title));
        }

        Action::Extract { dump, store_dir } => {
            let mut store = blockstore::Store::new(store_dir);
            store.create()?;

            let dump = File::open(dump)?;
            let xml = xml::EventReader::new(dump);

            let mut n = 0;
            let mut current = Page::None;
            let mut block = blockstore::Block::default();

            for event in xml {
                let event = event?;

                current = Page::parse(current, event);
                match current {
                    Page::Open => {
                        n += 1;
                        print!("\x1b[2K\x1b[0G{}", n);
                    }

                    Page::Texted {
                        ref title,
                        ref text,
                    } => {
                        let entry = blockstore::Entry::new(title, text);
                        block.add(entry)?;

                        if n % 10000 == 0 {
                            println!(": commit");
                            store.commit(&mut block, n)?;
                        }
                    }

                    _ => {}
                }
            }

            println!(": commit");
            store.commit(&mut block, n)?;
            println!("{}! done.", n);
        }

        Action::Index { dump, index } => {
            if !index.exists() {
                create_dir_all(&index)?;
            }

            let dir = MmapDirectory::open(index)?;
            let schema = schema();
            let index = Index::open_or_create(dir, schema.clone())?;
            let mut index_writer = index.writer(100_000_000)?;

            let dump = File::open(dump)?;
            let xml = xml::EventReader::new(dump);

            let mut n = 0;
            let mut current = Page::None;

            let s_title = schema.get_field("title").unwrap();
            let s_body = schema.get_field("body").unwrap();
            let s_gen = schema.get_field("gen").unwrap();

            for event in xml {
                let event = event?;

                current = Page::parse(current, event);
                match current {
                    Page::Open => {
                        n += 1;
                        print!("\x1b[2K\x1b[0G{}", n);
                    }

                    Page::Texted {
                        ref title,
                        ref text,
                    } => {
                        if n % 10000 == 0 {
                            println!(": commit");
                            index_writer.commit()?;
                        }

                        index_writer.add_document(doc!(
                            s_title => title.as_str(),
                            s_body => text.as_str(),
                            s_gen => 1_u64,
                        ));
                    }

                    _ => {}
                }
            }

            println!(": commit");
            index_writer.commit()?;
            println!("{}! done.", n);
        }

        Action::Query {
            index,
            search,
            limit,
        } => {
            let index = Index::open_in_dir(index)?;
            let reader = index.reader()?;
            let searcher = reader.searcher();

            let schema = schema();
            // let s_title = schema.get_field("title").unwrap();
            let s_body = schema.get_field("body").unwrap();

            let query_parser = QueryParser::for_index(&index, vec![s_body]);
            let query = query_parser.parse_query(&search)?;

            let top_docs: Vec<(Score, DocAddress)> =
                searcher.search(&query, &TopDocs::with_limit(limit))?;
            for (score, doc_address) in top_docs {
                let retrieved_doc = searcher.doc(doc_address)?;
                println!(
                    "score={}: {}",
                    score,
                    schema.to_named_doc(&retrieved_doc).0.get("title").unwrap()[0]
                        .text()
                        .unwrap()
                );
            }
        }
    }

    Ok(())
}

enum Page {
    None,
    Open,
    Title(Vec<String>),
    Titled(String),
    Text { title: String, text: Vec<String> },
    Texted { title: String, text: String },
}

impl Page {
    fn parse(page: Self, event: XmlEvent) -> Self {
        match (page, event) {
            (Page::None, XmlEvent::StartElement { name, .. }) if name.local_name == "page" => {
                Page::Open
            }

            (Page::Open, XmlEvent::StartElement { name, .. }) if name.local_name == "title" => {
                Page::Title(Vec::with_capacity(1))
            }

            (Page::Title(mut ts), XmlEvent::Characters(s))
            | (Page::Title(mut ts), XmlEvent::CData(s)) => {
                ts.push(s);
                Page::Title(ts)
            }

            (Page::Title(ts), XmlEvent::EndElement { name }) if name.local_name == "title" => {
                Page::Titled(ts.join(" "))
            }

            (Page::Titled(title), XmlEvent::StartElement { name, .. })
                if name.local_name == "text" =>
            {
                Page::Text {
                    title,
                    text: Vec::with_capacity(5),
                }
            }

            (Page::Text { title, mut text }, XmlEvent::Characters(s))
            | (Page::Text { title, mut text }, XmlEvent::CData(s)) => {
                text.push(s);
                Page::Text { title, text }
            }

            (Page::Text { title, text }, XmlEvent::EndElement { name })
                if name.local_name == "text" =>
            {
                Page::Texted {
                    title,
                    text: text.join(" "),
                }
            }

            (Page::Texted { .. }, _) => Page::None,
            (_, XmlEvent::EndElement { name }) if name.local_name == "page" => Page::None,

            (p, _) => p,
        }
    }
}

fn schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    schema_builder.add_u64_field("gen", INDEXED | STORED);
    schema_builder.build()
}
