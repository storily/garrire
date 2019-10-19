use fluent::{FluentBundle, FluentResource, FluentValue};
use log::{debug, error, warn};
use once_cell::sync::{Lazy, OnceCell};
use rand::Rng;
use std::collections::HashMap;
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;

use crate::Settings;

#[derive(RustEmbed)]
#[folder = "locale/"]
struct LocaleAsset;

static AVAILABLE_LOCALES: Lazy<Vec<LanguageIdentifier>> = Lazy::new(|| {
    let mut locs = LocaleAsset::iter()
        .filter_map(|filename| {
            let path = PathBuf::from(filename.to_string());
            path.iter().next().map(|s| s.to_owned())
        })
        .collect::<std::collections::HashSet<std::ffi::OsString>>()
        .into_iter()
        .map(|s| s.to_string_lossy().to_string().parse().unwrap())
        .collect::<Vec<LanguageIdentifier>>();

    // The empty locale is useful for special cases when purely-random locale
    // selection is needed. It is also hard-coded to fallback to no other
    // locale, while falling back to the default locale for dates/times.
    locs.push(LanguageIdentifier::default());

    let default = &Settings::load().locale.default;
    if !locs.iter().any(|loc| loc == default) {
        error!("Default locale is not available, hard abort!!!");
        panic!("Default locale is not available, hard abort!!!");
    }

    locs
});

static AVAILABLE_RESOURCES: Lazy<Vec<String>> = Lazy::new(|| {
    LocaleAsset::iter()
        .filter_map(|filename| {
            let path = PathBuf::from(filename.to_string());
            path.file_stem().map(|s| s.into())
        })
        .collect::<std::collections::HashSet<std::ffi::OsString>>()
        .into_iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect()
});

pub struct Locale {
    /// L10n resources this instance draws from.
    resources: Vec<String>,

    /// Locale it will use, and derive the fallbacks from. This can be any
    /// string and will be matched against the available (embedded) files.
    pub locale: LanguageIdentifier,

    /// Fallback chain used for date/times. Unlike language fallbacks, this
    /// remains constant for the particular root locale, such that the date/time
    /// formats of the preferred language are used even if the language isn't.
    pub chain: Vec<LanguageIdentifier>,

    /// Glitchiness factor. When positive, all calls to the `Locale` have a
    /// chance (0.0 to 1.0, defaults to 0.01) to use a completely different
    /// language instead. Surprise!
    pub glitchiness: f64,

    /// Bits used for grammar formatting e.g. lists
    grammar: OnceCell<Box<Self>>,
}

impl Default for Locale {
    fn default() -> Self {
        let settings = &Settings::load().locale;
        Self {
            resources: Vec::new(),
            locale: settings.default.clone(),
            chain: vec![settings.default.clone()],
            glitchiness: settings.glitchiness,
            grammar: OnceCell::new(),
        }
    }
}

pub type Args<'args> = HashMap<&'args str, FluentValue<'args>>;

fn fallback_chain(root: &LanguageIdentifier) -> Vec<LanguageIdentifier> {
    use std::convert::TryInto;

    let settings = &Settings::load().locale;
    let mut chain = vec![root.clone().try_into().unwrap()];

    if root == &LanguageIdentifier::default() {
        return chain;
    }

    if let Some(fallback) = settings.fallbacks.get(root) {
        for fall in fallback {
            chain.push(fall.clone().try_into().unwrap());
        }
    }

    chain
}

fn actual_locale(
    requested: LanguageIdentifier,
    can_use_this: bool,
) -> Result<LanguageIdentifier, String> {
    let settings = &Settings::load().locale;
    let mut fallback_to = Some(&settings.default);

    if let Some(fallback) = settings.fallbacks.get(&requested) {
        if fallback.is_empty() {
            fallback_to = None;
        } else {
            fallback_to = Some(&fallback[0]);
        }
    }

    let available = AVAILABLE_LOCALES.iter().any(|loc| loc == &requested);
    match (available, can_use_this, fallback_to.is_some()) {
        (true, true, _) => Ok(requested),
        (true, false, true) | (false, _, true) => {
            let fallback = fallback_to.unwrap().clone(); // safe unwrap bc is_some()
            if requested == fallback {
                Err(format!(
                    "Fallback ({}) is what was originally requested ({})",
                    fallback, requested
                ))
            } else {
                warn!(
                    "Unavailable locale requested: {}, falling back to {}",
                    requested, fallback
                );
                actual_locale(fallback, true)
            }
        }
        (true, false, false) | (false, _, false) => {
            error!("Unavailable locale and no fallbacks! ({})", requested);
            Err(format!(
                "Unavailable locale and no fallbacks! ({})",
                requested
            ))
        }
    }
}

fn random_other_locale(this: &LanguageIdentifier) -> LanguageIdentifier {
    let others: Vec<&LanguageIdentifier> = AVAILABLE_LOCALES
        .iter()
        .filter(|l| l != &this && l != &&LanguageIdentifier::default())
        .collect();
    debug!("Random locale :: This: {}, Others: {:?}", this, others);
    if others.is_empty() {
        this
    } else if others.len() > 1 {
        others[rand::thread_rng().gen_range(0, others.len() - 1)]
    } else {
        others[0]
    }
    .clone()
}

impl Locale {
    pub fn new(requested_resources: &[&str]) -> Result<Self, String> {
        Self::with_locale(requested_resources, Settings::load().locale.default.clone())
    }

    pub fn single(
        resource: &str,
        name: &str,
        args: Option<&Args>,
        locale: Option<LanguageIdentifier>,
    ) -> Result<String, String> {
        Self::with_locale(
            &[resource],
            locale.unwrap_or_else(|| Settings::load().locale.default.clone()),
        )
        .and_then(|l| l.get(name, args))
    }

    pub fn glitchy(requested_resources: &[&str]) -> Result<Self, String> {
        Self::with_locale(requested_resources, LanguageIdentifier::default())
            .map(|l| l.glitchiness(1))
    }

    pub fn with_locale(resources: &[&str], locale: LanguageIdentifier) -> Result<Self, String> {
        let chain = fallback_chain(&locale);
        let locale = actual_locale(locale.clone(), true)?;
        Self::with_actual_locale(resources, locale, chain)
    }

    fn with_actual_locale(
        reses: &[&str],
        locale: LanguageIdentifier,
        chain: Vec<LanguageIdentifier>,
    ) -> Result<Self, String> {
        let mut resources = Vec::with_capacity(reses.len());
        for resource in reses {
            // Checking *all* resources across locales
            if !AVAILABLE_RESOURCES.iter().any(|res| res == resource) {
                error!("Missing resource: {}. This is a bug, report it!", resource);
                return Err(format!("Missing resource: {}", resource));
            }

            resources.push(resource.to_string());
        }

        Ok(Self {
            resources,
            locale,
            chain,
            glitchiness: Settings::load().locale.glitchiness,
            grammar: OnceCell::new(),
        })
    }

    fn fallback(&self) -> Result<Self, String> {
        Ok(Self {
            resources: self.resources.clone(),
            locale: actual_locale(self.locale.clone(), false)?,
            chain: self.chain.clone(),
            glitchiness: 0.0,
            grammar: OnceCell::new(),
        })
    }

    pub fn get(&self, name: &str, args: Option<&Args>) -> Result<String, String> {
        debug!("Getting localisation for {} with args: {:?}", name, args,);

        if self.glitch() {
            let locale = random_other_locale(&self.locale);
            debug!("Glitching to another locale: {}", locale);
            let chain = fallback_chain(&locale);
            return Self {
                resources: self.resources.clone(),
                locale,
                chain,
                glitchiness: 0.0,
                grammar: OnceCell::new(),
            }
            .get(name, args);
        }

        let mut fres = Vec::with_capacity(self.resources.len());
        for resource in &self.resources {
            // Checking for this particular locale
            let asset = &format!("{}/{}.ftl", self.locale, resource);
            if let Some(ftl) = LocaleAsset::get(asset) {
                // Unwrap everything: in dev it should hard-fail for feedback,
                // and in prod the resources are embedded and so should not fail.
                // (And if they do, it's a bug, and needs to fail hard!)
                fres.push(
                    FluentResource::try_new(String::from_utf8(ftl.to_vec()).expect(&format!(
                        "Failed to decode embedded resource: {}. This is a bug, please report it!",
                        asset
                    )))
                    .expect(&format!(
                        "Failed to parse embedded FTL: {}. This is a bug, please report it!",
                        asset
                    )),
                );
            } else {
                debug!("No matching asset for resource {}, falling back", asset);
                return self.fallback()?.get(name, args);
            }
        }

        let mut bundle = FluentBundle::new(&self.chain);

        bundle
            .add_function("PREFIX_CHOOSE_VARIATE", |_, named| {
                use rand::seq::IteratorRandom;
                use std::str::FromStr;

                let variations = named
                    .into_iter()
                    .map(|(prefix, n)| {
                        let n = usize::from_str(&n.to_string())
                            .expect("PREFIX_CHOOSE_VARIATE arguments should be numbers");
                        std::iter::repeat(prefix.to_string())
                            .take(n)
                            .enumerate()
                            .map(|(n, p)| [p, (n + 1).to_string()].join("-"))
                    })
                    .flatten();

                variations
                    .choose(&mut rand::thread_rng())
                    .expect("PREFIX_CHOOSE_VARIATE needs at least one variant parameter")
                    .into()
            })
            .expect("Could not add PREFIX_CHOOSE_VARIATE");

        bundle
            .add_function("CHOOSE", |positional, _| {
                use std::str::FromStr;

                rand::thread_rng()
                    .gen_range(
                        1,
                        usize::from_str(
                            &positional
                                .iter()
                                .next()
                                .expect("CHOOSE requires an argument")
                                .to_string(),
                        )
                        .expect("CHOOSE requires a number"),
                    )
                    .into()
            })
            .expect("Could not add CHOOSE");

        for res in &fres {
            bundle
                .add_resource(res)
                .expect("Failed to add FTL resource. This is a bug, please report it!");
        }

        if let Some(msg) = bundle.get_message(name) {
            if let Some(pattern) = msg.value {
                let mut errors = Vec::new();
                let message = bundle.format_pattern(&pattern, args, &mut errors);
                if !errors.is_empty() {
                    error!(
                        "Error(s) while formatting message {} with args {:?}:",
                        name, args
                    );

                    for err in &errors {
                        error!("{}", err);
                    }
                }

                Ok(message.to_string())
            } else {
                debug!("Message for {} has no value, falling back", name);
                self.fallback()?.get(name, args)
            }
        } else {
            debug!(
                "Bundle couldn’t find the message for {}, falling back",
                name
            );
            self.fallback()?.get(name, args)
        }
    }

    pub fn glitchiness<N>(mut self, frequency: N) -> Self
    where
        N: Into<f64>,
    {
        self.glitchiness = frequency.into();
        self
    }

    fn glitch(&self) -> bool {
        if self.glitchiness > 0.0 {
            let roll = rand::thread_rng().gen_range(0.0, 1.0);
            debug!("Glitch roll: {} / threshold: {}", roll, self.glitchiness);
            roll < self.glitchiness
        } else {
            false
        }
    }

    fn grammar(&self) -> &Box<Self> {
        self.grammar.get_or_init(|| {
            Box::new(
                Self::with_actual_locale(&["grammar"], self.locale.clone(), self.chain.clone())
                    .expect("Failed to obtain grammar locale"),
            )
        })
    }

    pub fn list<'i, S: AsRef<str>>(
        &self,
        items: impl IntoIterator<Item = S>,
    ) -> Result<String, String> {
        let joiner = self.grammar().get("list-joiner", None)?;
        let items: Vec<String> = items.into_iter().map(|i| i.as_ref().to_string()).collect();
        Ok(items.join(&joiner))
    }
}

#[macro_export]
macro_rules! locale_args {
    (prefix) => {locale_args! {
        "prefix" => crate::Settings::load().discord.prefix.clone()
    }};
    (prefix, $($key:expr => $value:expr),*) => {locale_args! {
        $($key => $value),*
        ,"prefix" => crate::Settings::load().discord.prefix.clone()
    }};
    ($($key:expr => $value:expr),*$(,)?) => {{
        let mut args: crate::locale::Args = ::std::collections::HashMap::new();
        $(
            args.insert($key, ::fluent::FluentValue::from($value));
        )*
        args
    }}
}
