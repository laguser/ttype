use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

// стили отображения курсора / каретки
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum VisualStyle {
    Tape,      // плавная лента со смещением
    Underline, // терминальное ретро-подчеркивание
    Blocky,    // монолитный блок ▊
    Minimal,   // призрачный поток букв
}

impl VisualStyle {
    pub fn name(&self) -> &'static str {
        match self {
            VisualStyle::Tape => "tape",
            VisualStyle::Underline => "underline",
            VisualStyle::Blocky => "blocky",
            VisualStyle::Minimal => "minimal",
        }
    }
}

// цветовые темы оформления
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum ThemeColor {
    Auto,      // берет родную палитру терминала d:
    Catppuccin,// mocha mauve
    TokyoNight,// cyan neon
    Nord,      // frost blue
    Gruvbox,   // retro orange
    Dracula,   // purple
    Amber,     // vintage typewriter amber
    White,     // pure monochrome
}

impl ThemeColor {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeColor::Auto => "auto (term)",
            ThemeColor::Catppuccin => "catppuccin",
            ThemeColor::TokyoNight => "tokyo night",
            ThemeColor::Nord => "nord",
            ThemeColor::Gruvbox => "gruvbox",
            ThemeColor::Dracula => "dracula",
            ThemeColor::Amber => "amber",
            ThemeColor::White => "white",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Default)]
pub enum DownloadSource {
    #[default]
    Moscow,        // msc-1 VPS (Россия)
    Germany,       // de-1 VPS (Европа)
    JsDelivr,      // jsDelivr Global Edge CDN
    GitHub,        // GitHub Raw CDN
    CloudflareR2,  // Cloudflare R2 / Custom CDN
}

impl DownloadSource {
    pub fn name(&self) -> &'static str {
        match self {
            DownloadSource::Moscow => "moscow (vps)",
            DownloadSource::Germany => "germany (vps)",
            DownloadSource::JsDelivr => "jsdelivr (fast cdn)",
            DownloadSource::GitHub => "github (raw)",
            DownloadSource::CloudflareR2 => "cloudflare (r2)",
        }
    }

    pub fn base_url(&self) -> &'static str {
        match self {
            DownloadSource::Moscow => "http://93.88.203.50:8089/ttype/dicts",
            DownloadSource::Germany => "http://31.76.11.45:8089/ttype/dicts",
            DownloadSource::JsDelivr => "https://cdn.jsdelivr.net/gh/laguser/ttype@main/dicts",
            DownloadSource::GitHub => "https://raw.githubusercontent.com/laguser/ttype/main/dicts",
            DownloadSource::CloudflareR2 => "https://pub-ttype.r2.dev/dicts",
        }
    }
}

// локальный конфиг юзера
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub sound_enabled: bool,
    pub punctuation: bool,
    pub numbers: bool,
    pub theme: ThemeColor,
    pub visual: VisualStyle,
    pub live_wpm: bool,
    pub sudden_death: bool,
    #[serde(default)]
    pub mirror: DownloadSource,
    #[serde(default)]
    pub active_lang: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sound_enabled: false, // по дефолту тишина :3
            punctuation: false,
            numbers: false,
            theme: ThemeColor::Auto,
            visual: VisualStyle::Tape,
            live_wpm: true,
            sudden_death: false,
            mirror: DownloadSource::Moscow,
            active_lang: String::from("en"),
        }
    }
}

// результат одного раунда
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TestResult {
    pub date: DateTime<Local>,
    pub mode_name: String,
    pub duration_secs: u64,
    pub wpm: f64,
    pub cpm: f64,
    pub accuracy: f64,
    pub correct_chars: usize,
    pub incorrect_chars: usize,
    pub total_chars: usize,
    pub wpm_sparkline: Vec<u64>,
}

// локальная бд в json :d
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DataStore {
    pub config: Config,
    pub results: Vec<TestResult>,
}

impl DataStore {
    fn file_path() -> Option<PathBuf> {
        let mut path = dirs::data_local_dir().or_else(dirs::home_dir)?;
        path.push(".ttype");
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        path.push("history.json");
        Some(path)
    }

    pub fn dicts_dir() -> Option<PathBuf> {
        let mut path = dirs::data_local_dir().or_else(dirs::home_dir)?;
        path.push(".ttype");
        path.push("dicts");
        if !path.exists() {
            let _ = fs::create_dir_all(&path);
        }
        Some(path)
    }

    pub fn load() -> Self {
        // если файла еще нет или юзер его сломал, возвращаем дефолты O_0
        if let Some(path) = Self::file_path() {
            if path.exists() {
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(store) = serde_json::from_str(&data) {
                        return store;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::file_path() {
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }

    pub fn save_result(&mut self, res: TestResult) {
        self.results.push(res);
        self.save();
    }
}
