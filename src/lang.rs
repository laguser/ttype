use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DictMeta {
    pub id: String,
    pub name: String,
    pub words_count: usize,
    pub file: String,
}

pub struct LangManager {
    pub items: Vec<DictMeta>,
    pub installed: Vec<String>,
}

impl LangManager {
    pub fn new(dicts_dir: &PathBuf) -> Self {
        let manifest_path = dicts_dir.join("manifest.json");
        let mut items = Vec::new();
        if manifest_path.exists() {
            if let Ok(data) = fs::read_to_string(&manifest_path) {
                if let Ok(parsed) = serde_json::from_str::<Vec<DictMeta>>(&data) {
                    items = parsed;
                }
            }
        }

        // Если локального манифеста нет, используем встроенный базовый список
        if items.is_empty() {
            items = vec![
                DictMeta { id: "russian".into(), name: "Russian".into(), words_count: 200, file: "russian.txt".into() },
                DictMeta { id: "russian_1k".into(), name: "Russian 1K".into(), words_count: 996, file: "russian_1k.txt".into() },
                DictMeta { id: "russian_10k".into(), name: "Russian 10K".into(), words_count: 9996, file: "russian_10k.txt".into() },
                DictMeta { id: "english_1k".into(), name: "English 1K".into(), words_count: 1000, file: "english_1k.txt".into() },
                DictMeta { id: "english_5k".into(), name: "English 5K".into(), words_count: 5000, file: "english_5k.txt".into() },
                DictMeta { id: "english_10k".into(), name: "English 10K".into(), words_count: 9944, file: "english_10k.txt".into() },
                DictMeta { id: "spanish".into(), name: "Spanish".into(), words_count: 197, file: "spanish.txt".into() },
                DictMeta { id: "spanish_1k".into(), name: "Spanish 1K".into(), words_count: 998, file: "spanish_1k.txt".into() },
                DictMeta { id: "german".into(), name: "German".into(), words_count: 200, file: "german.txt".into() },
                DictMeta { id: "german_1k".into(), name: "German 1K".into(), words_count: 988, file: "german_1k.txt".into() },
                DictMeta { id: "french".into(), name: "French".into(), words_count: 174, file: "french.txt".into() },
                DictMeta { id: "french_1k".into(), name: "French 1K".into(), words_count: 1394, file: "french_1k.txt".into() },
                DictMeta { id: "french_2k".into(), name: "French 2K".into(), words_count: 2041, file: "french_2k.txt".into() },
                DictMeta { id: "italian".into(), name: "Italian".into(), words_count: 199, file: "italian.txt".into() },
                DictMeta { id: "italian_1k".into(), name: "Italian 1K".into(), words_count: 1159, file: "italian_1k.txt".into() },
                DictMeta { id: "portuguese".into(), name: "Portuguese".into(), words_count: 201, file: "portuguese.txt".into() },
                DictMeta { id: "portuguese_1k".into(), name: "Portuguese 1K".into(), words_count: 1000, file: "portuguese_1k.txt".into() },
                DictMeta { id: "ukrainian".into(), name: "Ukrainian".into(), words_count: 199, file: "ukrainian.txt".into() },
                DictMeta { id: "ukrainian_1k".into(), name: "Ukrainian 1K".into(), words_count: 1000, file: "ukrainian_1k.txt".into() },
                DictMeta { id: "polish".into(), name: "Polish".into(), words_count: 183, file: "polish.txt".into() },
                DictMeta { id: "turkish".into(), name: "Turkish".into(), words_count: 299, file: "turkish.txt".into() },
                DictMeta { id: "turkish_1k".into(), name: "Turkish 1K".into(), words_count: 1026, file: "turkish_1k.txt".into() },
                DictMeta { id: "japanese_romaji".into(), name: "Japanese Romaji".into(), words_count: 153, file: "japanese_romaji.txt".into() },
            ];
        }

        let mut mgr = Self { items, installed: Vec::new() };
        mgr.refresh_installed(dicts_dir);
        mgr
    }

    pub fn refresh_installed(&mut self, dicts_dir: &PathBuf) {
        self.installed.clear();
        if let Ok(entries) = fs::read_dir(dicts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        self.installed.push(stem.to_string());
                    }
                }
            }
        }
    }

    pub fn is_installed(&self, id: &str) -> bool {
        self.installed.iter().any(|i| i == id)
    }

    pub fn download_dict(
        &mut self,
        dicts_dir: &PathBuf,
        base_url: &str,
        item: &DictMeta,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/{}", base_url.trim_end_matches('/'), item.file);
        let resp = ureq::get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .call()?;
        let body = resp.into_string()?;

        let dest = dicts_dir.join(&item.file);
        fs::write(dest, body)?;
        self.refresh_installed(dicts_dir);
        Ok(())
    }
}
