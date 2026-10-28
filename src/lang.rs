use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

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
    pub fn new(dicts_dir: &Path) -> Self {
        let manifest_path = dicts_dir.join("manifest.json");
        let mut items = Vec::new();
        if manifest_path.exists() {
            if let Ok(data) = fs::read_to_string(&manifest_path) {
                if let Ok(parsed) = serde_json::from_str::<Vec<DictMeta>>(&data) {
                    items = parsed;
                }
            }
        }

        // Если локального манифеста нет, используем полный список
        if items.is_empty() {
            items = vec![
                DictMeta { id: "russian".into(), name: "Russian".into(), words_count: 200, file: "russian.txt".into() },
                DictMeta { id: "russian_1k".into(), name: "Russian 1K".into(), words_count: 996, file: "russian_1k.txt".into() },
                DictMeta { id: "russian_10k".into(), name: "Russian 10K".into(), words_count: 9996, file: "russian_10k.txt".into() },
                DictMeta { id: "english_1k".into(), name: "English 1K".into(), words_count: 1000, file: "english_1k.txt".into() },
                DictMeta { id: "english_5k".into(), name: "English 5K".into(), words_count: 5000, file: "english_5k.txt".into() },
                DictMeta { id: "english_10k".into(), name: "English 10K".into(), words_count: 9944, file: "english_10k.txt".into() },
                DictMeta { id: "war_and_peace_ru".into(), name: "War & Peace Quotes (RU)".into(), words_count: 1163, file: "war_and_peace_ru.txt".into() },
                DictMeta { id: "war_and_peace_en".into(), name: "War & Peace Quotes (EN)".into(), words_count: 6491, file: "war_and_peace_en.txt".into() },
                DictMeta { id: "code_rust".into(), name: "Code: Rust".into(), words_count: 192, file: "code_rust.txt".into() },
                DictMeta { id: "code_python".into(), name: "Code: Python".into(), words_count: 174, file: "code_python.txt".into() },
                DictMeta { id: "code_javascript".into(), name: "Code: JavaScript".into(), words_count: 126, file: "code_javascript.txt".into() },
                DictMeta { id: "code_c".into(), name: "Code: C".into(), words_count: 200, file: "code_c.txt".into() },
                DictMeta { id: "code_go".into(), name: "Code: Go".into(), words_count: 63, file: "code_go.txt".into() },
                DictMeta { id: "code_html".into(), name: "Code: HTML".into(), words_count: 232, file: "code_html.txt".into() },
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

    pub fn refresh_installed(&mut self, dicts_dir: &Path) {
        self.installed.clear();
        if let Ok(entries) = fs::read_dir(dicts_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("txt") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        self.installed.push(stem.to_string());
                    }
                }
            }
        }
        self.installed.sort();
    }

    pub fn is_installed(&self, id: &str) -> bool {
        self.installed.iter().any(|i| i == id)
    }

    pub fn delete_dict(&mut self, dicts_dir: &Path, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = dicts_dir.join(format!("{}.txt", id));
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        self.refresh_installed(dicts_dir);
        Ok(())
    }

    pub const GITHUB_BASE_URL: &'static str = "https://raw.githubusercontent.com/laguser/ttype/main/dicts";

    pub fn download_dict<F>(
        &mut self,
        dicts_dir: &Path,
        item: &DictMeta,
        mut on_progress: F,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(usize, usize), // (downloaded_bytes, total_bytes)
    {
        use std::io::Read;

        let url = format!("{}/{}", Self::GITHUB_BASE_URL.trim_end_matches('/'), item.file);
        let resp = ureq::get(&url)
            .timeout(std::time::Duration::from_secs(15))
            .call()?;

        let total_bytes: usize = resp
            .header("Content-Length")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        let mut reader = resp.into_reader();
        let mut buffer = Vec::new();
        let mut chunk = [0u8; 4096];
        let mut downloaded: usize = 0;

        loop {
            let n = reader.read(&mut chunk)?;
            if n == 0 {
                break;
            }
            buffer.extend_from_slice(&chunk[..n]);
            downloaded += n;
            on_progress(downloaded, total_bytes);
        }

        let dest = dicts_dir.join(&item.file);
        fs::write(dest, buffer)?;
        self.refresh_installed(dicts_dir);
        Ok(())
    }
}
