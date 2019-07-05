use fluent::FluentBundle;
use fluent_resmgr::resource_manager::ResourceManager;
use log::{debug, error, info};
use std::collections::HashMap;
use std::path::PathBuf;

struct Lang<'settings> {
    settings: &'settings Language,
    manager: ResourceManager,
    resources: Vec<String>,
    langs: HashMap<String, FluentBundle<'settings>>,
}

impl<'settings> Lang<'settings> {
    pub fn init(settings: &'settings Language) -> Self {
        use std::collections::HashSet;

        let manager = ResourceManager::new(settings.path.join("{locale}/{res_id}").to_string_lossy().to_string());

        fn is_hidden(entry: &walkdir::DirEntry) -> bool {
            entry.file_name()
                 .to_str()
                 .map(|s| s.starts_with("."))
                 .unwrap_or(false)
        }

        let resources = walkdir::WalkDir::new(&settings.path)
            .into_iter()
            .filter_entry(|e| !is_hidden(e))
            .filter_map(|e| e.ok().map(|f| f.file_name().to_os_string()))
            .filter_map(|f| PathBuf::from(&f).extension().map(|s| s == "ftl").and(f.into_string().ok()))
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        Self {
            settings,
            manager,
            resources,
            langs: HashMap::new(),
        }
    }

    fn load_language(&mut self, lang: &str) {
        let locales = vec![lang.into(), self.settings.default.clone()];
        dbg!(&locales);

        let bundle = self.manager.get_bundle(locales, self.resources);
        self.langs.insert(lang.into(), bundle);
    }
}