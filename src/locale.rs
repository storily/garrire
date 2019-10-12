use fluent::{FluentBundle, FluentResource, FluentValue};
use log::{debug, error, warn};
use rand::Rng;
use std::collections::HashMap;
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;

use crate::Settings;

#[derive(RustEmbed)]
#[folder = "locale/"]
struct LocaleAsset;

lazy_static::lazy_static! {
    static ref AVAILABLE_LOCALES: Vec<LanguageIdentifier> = {
        let mut locs = LocaleAsset::iter().filter_map(|filename| {
            let path = PathBuf::from(filename.to_string());
            path.iter().next().map(|s| s.to_owned())
        }).collect::<std::collections::HashSet<std::ffi::OsString>>()
        .into_iter()
        .map(|s| s.to_string_lossy().to_string().parse().unwrap())
        .collect::<Vec<LanguageIdentifier>>();

        // The empty locale is useful for special cases when purely-random locale
        // selection is needed. It is also hard-coded to fallback to no other
        // locale, while falling back to the default locale for dates/times.
        locs.push(LanguageIdentifier::default());

        let default = &Settings::load().locale.default;
        if ! locs.iter().any(|loc| loc == default) {
            error!("Default locale is not available, hard abort!!!");
            panic!("Default locale is not available, hard abort!!!");
        }

        locs
    };

    static ref AVAILABLE_RESOURCES: Vec<String> = {
        LocaleAsset::iter().filter_map(|filename| {
            let path = PathBuf::from(filename.to_string());
            path.file_stem().map(|s| s.into())
        }).collect::<std::collections::HashSet<std::ffi::OsString>>()
        .into_iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect()
    };
}

pub struct Locale {
    /// L10n resources this instance draws from.
    resources: Vec<String>,

    /// Locale it will use, and derive the fallbacks from. This can be any
    /// string and will be matched against the available (embedded) files.
    locale: LanguageIdentifier,

    /// Fallback chain used for date/times. Unlike language fallbacks, this
    /// remains constant for the particular root locale, such that the date/time
    /// formats of the preferred language are used even if the language isn't.
    chain: Vec<LanguageIdentifier>,

    /// Glitchiness factor. When positive, all calls to the `Locale` have a
    /// chance (0.0 to 1.0, defaults to 0.01) to use a completely different
    /// language instead. Surprise!
    glitchiness: f64,
}

impl Default for Locale {
    fn default() -> Self {
        let settings = &Settings::load().locale;
        Self {
            resources: Vec::new(),
            locale: settings.default.clone(),
            chain: vec![settings.default.clone()],
            glitchiness: settings.glitchiness,
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

fn actual_locale(requested: LanguageIdentifier, can_use_this: bool) -> Option<LanguageIdentifier> {
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
        (true, true, _) => Some(requested),
        (true, false, true) | (false, _, true) => {
            let fallback = fallback_to.unwrap().clone(); // safe unwrap bc is_some()
            if requested == fallback {
                None
            } else {
                warn!(
                    "Unavailable locale requested: {}, falling back to {}",
                    requested, fallback
                );
                actual_locale(fallback, true)
            }
        }
        (true, false, false) | (false, _, false) => None,
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

fn actual_locale_hard(
    requested_locale: LanguageIdentifier,
    can_use_this: bool,
) -> LanguageIdentifier {
    match actual_locale(requested_locale.clone(), can_use_this) {
        Some(l) => l,
        None => {
            error!(
                "Unavailable locale and no fallbacks! ({}), abort",
                requested_locale
            );
            panic!(
                "Unavailable locale and no fallbacks! ({}), abort",
                requested_locale
            );
        }
    }
}

impl Locale {
    pub fn new(requested_resources: &[&str]) -> Self {
        Self::with_locale(requested_resources, Settings::load().locale.default.clone())
    }

    pub fn glitchy(requested_resources: &[&str]) -> Self {
        Self::with_locale(requested_resources, LanguageIdentifier::default()).glitchiness(1)
    }

    pub fn with_locale(requested_resources: &[&str], requested_locale: LanguageIdentifier) -> Self {
        let chain = fallback_chain(&requested_locale);
        let locale = actual_locale_hard(requested_locale.clone(), true);

        let mut resources = Vec::with_capacity(requested_resources.len());
        for resource in requested_resources {
            // Checking *all* resources across locales
            if !AVAILABLE_RESOURCES.iter().any(|res| res == resource) {
                error!("Missing resource: {}. This is a bug, report it!", resource);
                panic!("Missing resource: {}", resource);
            }

            resources.push(resource.to_string());
        }

        Self {
            resources,
            locale,
            chain,
            glitchiness: Settings::load().locale.glitchiness,
        }
    }

    fn fallback(&self) -> Self {
        Self {
            resources: self.resources.clone(),
            locale: actual_locale_hard(self.locale.clone(), false),
            chain: self.chain.clone(),
            glitchiness: 0.0,
        }
    }

    pub fn get(&self, name: &str, args: Option<&Args>) -> String {
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
                return self.fallback().get(name, args);
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

                message.to_string()
            } else {
                debug!("Message for {} has no value, falling back", name);
                self.fallback().get(name, args)
            }
        } else {
            debug!(
                "Bundle couldn’t find the message for {}, falling back",
                name
            );
            self.fallback().get(name, args)
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
}

#[macro_export]
macro_rules! locale_args {
    ($($key:expr => $value:expr),*) => {{
        let mut args: crate::locale::Args = ::std::collections::HashMap::new();
        $(
            args.insert($key, ::fluent::FluentValue::from($value));
        )*
        args
    }}
}
