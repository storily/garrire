use fluent::{FluentBundle, FluentResource, FluentValue};
use log::{debug, error, warn};
use rand::Rng;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::settings::Settings;

#[derive(RustEmbed)]
#[folder = "locale/"]
struct LocaleAsset;

lazy_static::lazy_static! {
    static ref AVAILABLE_LOCALES: Vec<String> = {
        let locs = LocaleAsset::iter().filter_map(|filename| {
            let path = PathBuf::from(filename.to_string());
            path.iter().next().map(|s| s.to_owned())
        }).collect::<std::collections::HashSet<std::ffi::OsString>>()
        .into_iter()
        .map(|s| s.to_string_lossy().to_string())
        .collect::<Vec<String>>();

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
    resources: Vec<String>,
    locale: String,
    chain: Vec<String>,
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

pub type Args<'args> = HashMap<&'args str, FluentValue>;

fn fallback_chain(root: &String) -> Vec<String> {
    let settings = &Settings::load().locale;
    let mut chain = vec![root.clone()];
    if let Some(fallback) = settings.fallbacks.get(root) {
        for fall in fallback {
            chain.push(fall.clone());
        }
    }

    chain
}

fn actual_locale(requested: String, can_use_this: bool) -> Option<String> {
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
            warn!(
                "Unavailable locale requested: {}, falling back to {}",
                requested, fallback
            );
            actual_locale(fallback, true)
        }
        (true, false, false) | (false, _, false) => None,
    }
}

fn random_other_locale(this: &String) -> String {
    let others: Vec<&String> = AVAILABLE_LOCALES.iter().filter(|l| l != &this).collect();
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

fn actual_locale_hard(requested_locale: String, can_use_this: bool) -> String {
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

    pub fn with_locale(requested_resources: &[&str], requested_locale: String) -> Self {
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

    fn get(&self, name: &str, args: Option<&Args>, alternate: Option<fn(&str) -> bool>) -> String {
        if self.glitch() {
            let locale = random_other_locale(&self.locale);
            let chain = fallback_chain(&locale);
            return Self {
                resources: self.resources.clone(),
                locale,
                chain,
                glitchiness: 0.0,
            }
            .get(name, args, alternate);
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
                return self.fallback().get(name, args, alternate);
            }
        }

        let mut bundle = FluentBundle::new(&self.chain);
        for res in &fres {
            bundle
                .add_resource(res)
                .expect("Failed to add FTL resource. This is a bug, please report it!");
        }

        // TODO: handle alternates

        if let Some((message, partial_error)) = bundle.format(name, args) {
            if !partial_error.is_empty() {
                error!(
                    "Partial error(s) while formatting message {} with args {:?}:",
                    name, args
                );
            }

            for err in &partial_error {
                error!("{}", err);
            }

            message.to_string()
        } else {
            self.fallback().get(name, args, alternate)
        }
    }

    pub fn simple(&self, name: &str, args: Option<&Args>) -> String {
        self.get(name, args, None)
    }

    pub fn alternate(&self, name: &str, args: Option<&Args>, selector: fn(&str) -> bool) -> String {
        self.get(name, args, Some(selector))
    }

    pub fn random(&self, name: &str, args: Option<&Args>) -> String {
        self.get(name, args, Some(|_| rand::thread_rng().gen()))
    }

    pub fn glitchiness(mut self, frequency: f64) -> Self {
        self.glitchiness = frequency;
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
