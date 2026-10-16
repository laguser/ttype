mod audio;
mod storage;
mod words;

use audio::SoundEngine;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{seq::SliceRandom, Rng};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use storage::{DataStore, TestResult, ThemeColor, VisualStyle};

// режимы тренировки
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PracticeMode {
    Russian,
    English,
    Code,
    Quotes,
}

impl PracticeMode {
    pub fn name(&self) -> &'static str {
        match self {
            PracticeMode::Russian => "ru",
            PracticeMode::English => "en",
            PracticeMode::Code => "code",
            PracticeMode::Quotes => "quotes",
        }
    }
}

// лимиты времени
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TestDuration {
    Sec15 = 15,
    Sec30 = 30,
}

// экраны приложения
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum AppMode {
    Typing,
    Result,
    Settings,
    HistoryView,
}

pub struct App {
    pub practice_mode: PracticeMode,
    pub duration: TestDuration,
    pub mode: AppMode,
    pub settings_cursor: usize,
    pub target_chars: Vec<char>,
    pub typed_chars: Vec<char>,
    pub start_time: Option<Instant>,
    pub end_time: Option<Instant>,
    pub wpm_samples: Vec<u64>,
    pub last_sample_sec: u64,
    pub store: DataStore,
    pub sound: SoundEngine,
    pub last_result: Option<TestResult>,
    pub last_esc_press: Option<Instant>,
}

impl App {
    pub fn new() -> Self {
        let store = DataStore::load();
        let sound = SoundEngine::new();
        let mut app = Self {
            practice_mode: PracticeMode::Russian,
            duration: TestDuration::Sec15,
            mode: AppMode::Typing,
            settings_cursor: 0,
            target_chars: Vec::new(),
            typed_chars: Vec::new(),
            start_time: None,
            end_time: None,
            wpm_samples: Vec::new(),
            last_sample_sec: 0,
            store,
            sound,
            last_result: None,
            last_esc_press: None,
        };
        app.reset_test();
        app
    }

    // подтягиваем акцентный цвет в зависимости от выбранной темы
    pub fn theme_accent(&self) -> Color {
        match self.store.config.theme {
            ThemeColor::Auto => Color::Yellow, // берет родной желтый из палитры терминала d:
            ThemeColor::Catppuccin => Color::Rgb(203, 166, 247),
            ThemeColor::TokyoNight => Color::Rgb(125, 207, 255),
            ThemeColor::Nord => Color::Rgb(136, 192, 208),
            ThemeColor::Gruvbox => Color::Rgb(254, 128, 25),
            ThemeColor::Dracula => Color::Rgb(189, 147, 249),
            ThemeColor::Amber => Color::Rgb(245, 185, 65),
            ThemeColor::White => Color::Rgb(240, 240, 240),
        }
    }

    // генерируем случайный текст для раунда
    pub fn generate_text(&self) -> String {
        let mut rng = rand::thread_rng();

        match self.practice_mode {
            PracticeMode::Quotes => {
                // берем случайную цитату, а если тест долгий — склеиваем 2-3 разные O_0
                let count = if self.duration == TestDuration::Sec30 { 3 } else { 2 };
                let mut quotes_pool = words::QUOTES.to_vec();
                quotes_pool.shuffle(&mut rng);
                quotes_pool.into_iter().take(count).collect::<Vec<_>>().join(" ")
            }
            PracticeMode::Russian => {
                let mut chosen = Vec::new();
                let mut last_word = "";
                for _ in 0..120 {
                    if let Some(&w) = words::WORDS_RU.choose(&mut rng) {
                        // избегаем дублей подряд :3
                        if w == last_word {
                            continue;
                        }
                        last_word = w;

                        let mut word = w.to_string();
                        if self.store.config.punctuation && rng.gen_bool(0.25) {
                            let punc = [",", ".", "!", "?", " —", ";", ":"].choose(&mut rng).unwrap();
                            word.push_str(punc);
                        }
                        if self.store.config.numbers && rng.gen_bool(0.12) {
                            let num = rng.gen_range(0..100);
                            word = format!("{}{}", word, num);
                        }
                        chosen.push(word);
                    }
                }
                chosen.join(" ")
            }
            PracticeMode::English => {
                let mut chosen = Vec::new();
                let mut last_word = "";
                for _ in 0..120 {
                    if let Some(&w) = words::WORDS_EN.choose(&mut rng) {
                        if w == last_word {
                            continue;
                        }
                        last_word = w;

                        let mut word = w.to_string();
                        if self.store.config.punctuation && rng.gen_bool(0.25) {
                            let punc = [",", ".", "!", "?", " -", ";", ":"].choose(&mut rng).unwrap();
                            word.push_str(punc);
                        }
                        if self.store.config.numbers && rng.gen_bool(0.12) {
                            let num = rng.gen_range(0..100);
                            word = format!("{}{}", word, num);
                        }
                        chosen.push(word);
                    }
                }
                chosen.join(" ")
            }
            PracticeMode::Code => {
                let mut chosen = Vec::new();
                for _ in 0..100 {
                    if let Some(&w) = words::CODE_WORDS.choose(&mut rng) {
                        chosen.push(w.to_string());
                    }
                }
                chosen.join(" ")
            }
        }
    }

    // сброс раунда и генерация новой ленты
    pub fn reset_test(&mut self) {
        let text = self.generate_text();
        self.target_chars = text.chars().collect();
        self.typed_chars.clear();
        self.start_time = None;
        self.end_time = None;
        self.wpm_samples.clear();
        self.last_sample_sec = 0;
        self.mode = AppMode::Typing;
    }

    // сколько секунд осталось до конца таймера
    pub fn time_remaining(&self) -> Duration {
        let limit = Duration::from_secs(self.duration as u64);
        if let Some(start) = self.start_time {
            let elapsed = self.end_time.unwrap_or_else(Instant::now).duration_since(start);
            if elapsed >= limit {
                Duration::ZERO
            } else {
                limit - elapsed
            }
        } else {
            limit
        }
    }

    // подсчет текущей скорости на лету
    pub fn current_wpm(&self) -> f64 {
        if let Some(start) = self.start_time {
            let elapsed = Instant::now().duration_since(start).as_secs_f64().max(0.1);
            let correct = self
                .typed_chars
                .iter()
                .enumerate()
                .filter(|(i, c)| *i < self.target_chars.len() && self.target_chars[*i] == **c)
                .count();
            (correct as f64 / 5.0) / (elapsed / 60.0)
        } else {
            0.0
        }
    }

    // завершение теста и сохранение рекорда
    pub fn finish_test(&mut self) {
        let end = Instant::now();
        self.end_time = Some(end);
        let start = self.start_time.unwrap_or(end);
        let elapsed_secs = (end.duration_since(start).as_secs_f64()).max(0.1);

        let mut correct = 0;
        let mut incorrect = 0;
        for (i, &t_char) in self.typed_chars.iter().enumerate() {
            if i < self.target_chars.len() {
                if t_char == self.target_chars[i] {
                    correct += 1;
                } else {
                    incorrect += 1;
                }
            }
        }

        let total = self.typed_chars.len();
        let accuracy = if total > 0 {
            (correct as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        let mins = elapsed_secs / 60.0;
        let wpm = (correct as f64 / 5.0) / mins;
        let cpm = (correct as f64) / mins;

        let mode_desc = format!(
            "{}{}{}",
            self.practice_mode.name(),
            if self.store.config.punctuation { "+punc" } else { "" },
            if self.store.config.numbers { "+num" } else { "" }
        );

        let result = TestResult {
            date: chrono::Local::now(),
            mode_name: mode_desc,
            duration_secs: self.duration as u64,
            wpm,
            cpm,
            accuracy,
            correct_chars: correct,
            incorrect_chars: incorrect,
            total_chars: total,
            wpm_sparkline: self.wpm_samples.clone(),
        };

        if self.store.config.sound_enabled {
            self.sound.play_bell(); // дзинь! d:
        }

        self.store.save_result(result.clone());
        self.last_result = Some(result);
        self.mode = AppMode::Result;
    }

    // обработка нажатий клавиш
    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::Typing => match key.code {
                KeyCode::Tab => self.reset_test(),
                KeyCode::F(1) => {
                    self.practice_mode = match self.practice_mode {
                        PracticeMode::Russian => PracticeMode::English,
                        PracticeMode::English => PracticeMode::Code,
                        PracticeMode::Code => PracticeMode::Quotes,
                        PracticeMode::Quotes => PracticeMode::Russian,
                    };
                    self.reset_test();
                }
                KeyCode::F(2) => {
                    self.duration = match self.duration {
                        TestDuration::Sec15 => TestDuration::Sec30,
                        TestDuration::Sec30 => TestDuration::Sec15,
                    };
                    self.reset_test();
                }
                KeyCode::F(3) => self.mode = AppMode::HistoryView,
                KeyCode::F(4) => self.mode = AppMode::Settings,
                KeyCode::Backspace => {
                    if !self.typed_chars.is_empty() {
                        self.typed_chars.pop();
                        if self.store.config.sound_enabled {
                            self.sound.play_click();
                        }
                    }
                }
                KeyCode::Char(c) => {
                    // стартуем таймер ровно с первой нажатой буквы
                    if self.start_time.is_none() {
                        self.start_time = Some(Instant::now());
                    }

                    let cur_idx = self.typed_chars.len();
                    let is_correct = cur_idx < self.target_chars.len() && self.target_chars[cur_idx] == c;

                    // режим внезапной смерти: ошибся — гуляй, начинай заново ):
                    if !is_correct && self.store.config.sudden_death {
                        if self.store.config.sound_enabled {
                            self.sound.play_error();
                        }
                        self.reset_test();
                        return;
                    }

                    if self.store.config.sound_enabled {
                        if !is_correct {
                            self.sound.play_error();
                        } else if c == ' ' {
                            self.sound.play_space();
                        } else {
                            self.sound.play_click();
                        }
                    }

                    self.typed_chars.push(c);
                    if self.typed_chars.len() >= self.target_chars.len() {
                        self.finish_test();
                    }
                }
                _ => {}
            },
            AppMode::Settings => match key.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.settings_cursor > 0 {
                        self.settings_cursor -= 1;
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.settings_cursor < 6 {
                        self.settings_cursor += 1;
                    }
                }
                KeyCode::Enter | KeyCode::Char(' ') | KeyCode::Right | KeyCode::Left => {
                    match self.settings_cursor {
                        0 => {
                            self.store.config.sound_enabled = !self.store.config.sound_enabled;
                            if self.store.config.sound_enabled {
                                self.sound.play_bell();
                            }
                        }
                        1 => self.store.config.punctuation = !self.store.config.punctuation,
                        2 => self.store.config.numbers = !self.store.config.numbers,
                        3 => self.store.config.live_wpm = !self.store.config.live_wpm,
                        4 => self.store.config.sudden_death = !self.store.config.sudden_death,
                        5 => {
                            self.store.config.theme = match self.store.config.theme {
                                ThemeColor::Auto => ThemeColor::Catppuccin,
                                ThemeColor::Catppuccin => ThemeColor::TokyoNight,
                                ThemeColor::TokyoNight => ThemeColor::Nord,
                                ThemeColor::Nord => ThemeColor::Gruvbox,
                                ThemeColor::Gruvbox => ThemeColor::Dracula,
                                ThemeColor::Dracula => ThemeColor::Amber,
                                ThemeColor::Amber => ThemeColor::White,
                                ThemeColor::White => ThemeColor::Auto,
                            };
                        }
                        6 => {
                            self.store.config.visual = match self.store.config.visual {
                                VisualStyle::Tape => VisualStyle::Underline,
                                VisualStyle::Underline => VisualStyle::Blocky,
                                VisualStyle::Blocky => VisualStyle::Minimal,
                                VisualStyle::Minimal => VisualStyle::Tape,
                            };
                        }
                        _ => {}
                    }
                    self.store.save();
                }
                KeyCode::Tab | KeyCode::F(4) => {
                    self.mode = AppMode::Typing;
                    self.reset_test();
                }
                _ => {}
            },
            AppMode::Result => match key.code {
                KeyCode::Tab | KeyCode::Enter | KeyCode::Char(' ') => self.reset_test(),
                KeyCode::Char('s') | KeyCode::F(4) => self.mode = AppMode::Settings,
                KeyCode::Char('h') | KeyCode::F(3) => self.mode = AppMode::HistoryView,
                _ => {}
            },
            AppMode::HistoryView => match key.code {
                KeyCode::Tab | KeyCode::Enter | KeyCode::Char(' ') | KeyCode::F(3) => {
                    self.mode = AppMode::Typing;
                    self.reset_test();
                }
                _ => {}
            },
        }
    }
}

// рисуем мини-график скорости по секундам :3
fn render_sparkline(samples: &[u64]) -> String {
    if samples.is_empty() {
        return String::new();
    }
    let chars = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let min = *samples.iter().min().unwrap_or(&0);
    let max = (*samples.iter().max().unwrap_or(&100)).max(min + 1);

    samples
        .iter()
        .map(|&v| {
            let ratio = (v.saturating_sub(min) as f64) / ((max - min) as f64);
            let idx = ((ratio * 7.0).round() as usize).min(7);
            chars[idx]
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let mut init_mode: Option<PracticeMode> = None;
    let mut init_duration: Option<TestDuration> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                println!("ttype 0.1.0 — minimalist terminal typing speed test\n");
                println!("USAGE:");
                println!("  ttype [OPTIONS]\n");
                println!("OPTIONS:");
                println!("  -m, --mode <MODE>       Practice mode: ru, en, code, quotes");
                println!("  -t, --time <SECS>       Test duration: 15, 30");
                println!("  -h, --help              Print this help message");
                println!("  -v, --version           Print version\n");
                println!("KEYBINDINGS:");
                println!("  tab                     Restart test / new sheet");
                println!("  f1                      Cycle mode (ru -> en -> code -> quotes)");
                println!("  f2                      Cycle timer (15s <-> 30s)");
                println!("  f3                      View local stats & records");
                println!("  f4                      Settings menu");
                println!("  esc                     Restart test / back to typing");
                println!("  esc esc                 Quit application");
                return Ok(());
            }
            "-v" | "--version" => {
                println!("ttype 0.1.0");
                return Ok(());
            }
            "-m" | "--mode" => {
                if i + 1 < args.len() {
                    i += 1;
                    init_mode = match args[i].to_lowercase().as_str() {
                        "ru" | "russian" => Some(PracticeMode::Russian),
                        "en" | "english" => Some(PracticeMode::English),
                        "code" => Some(PracticeMode::Code),
                        "quote" | "quotes" => Some(PracticeMode::Quotes),
                        _ => None,
                    };
                }
            }
            "-t" | "--time" => {
                if i + 1 < args.len() {
                    i += 1;
                    init_duration = match args[i].as_str() {
                        "15" | "15s" => Some(TestDuration::Sec15),
                        "30" | "30s" => Some(TestDuration::Sec30),
                        _ => None,
                    };
                }
            }
            _ => {}
        }
        i += 1;
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    if let Some(m) = init_mode {
        app.practice_mode = m;
        app.reset_test();
    }
    if let Some(d) = init_duration {
        app.duration = d;
        app.reset_test();
    }

    let tick_rate = Duration::from_millis(25);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                    break;
                }

                // умный esc: один раз сброс/назад, два раза подряд — выход O_0
                if key.code == KeyCode::Esc {
                    let now = Instant::now();
                    let is_double_esc = if let Some(last) = app.last_esc_press {
                        now.duration_since(last) <= Duration::from_millis(500)
                    } else {
                        false
                    };

                    if is_double_esc {
                        break;
                    }

                    app.last_esc_press = Some(now);

                    match app.mode {
                        AppMode::Typing => app.reset_test(),
                        AppMode::Result | AppMode::Settings | AppMode::HistoryView => {
                            app.mode = AppMode::Typing;
                            app.reset_test();
                        }
                    }
                    continue;
                }

                app.handle_key(key);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if app.mode == AppMode::Typing && app.start_time.is_some() {
                // раз в секунду замеряем wpm для графика
                let elapsed_sec = app.start_time.unwrap().elapsed().as_secs();
                if elapsed_sec > app.last_sample_sec {
                    app.last_sample_sec = elapsed_sec;
                    app.wpm_samples.push(app.current_wpm() as u64);
                }

                if app.time_remaining() == Duration::ZERO {
                    app.finish_test();
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    match app.mode {
        AppMode::Typing => {
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(1)])
                .split(size);

            let center_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Fill(1),
                ])
                .split(outer[0]);

            render_top_bar(f, app, center_area[1]);
            render_single_line_stream(f, app, center_area[3]);
            render_bottom_bar(f, app, outer[1]);
        }
        AppMode::Settings => render_settings_view(f, app, size),
        AppMode::Result => render_minimal_result(f, app, size),
        AppMode::HistoryView => render_minimal_history(f, app, size),
    }
}

// верхняя информационная строка
fn render_top_bar(f: &mut Frame, app: &App, area: Rect) {
    let time_left = app.time_remaining().as_secs();
    let accent = app.theme_accent();

    let mut mode_str = app.practice_mode.name().to_string();
    if app.store.config.punctuation {
        mode_str.push_str("+punc");
    }
    if app.store.config.numbers {
        mode_str.push_str("+num");
    }
    if app.store.config.sudden_death {
        mode_str.push_str(" [hardcore]");
    }

    let mut spans = vec![
        Span::styled(mode_str, Style::default().fg(accent)),
        Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{}s", app.duration as u64), Style::default().fg(Color::White)),
        Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{:02}s", time_left),
            Style::default()
                .fg(if time_left <= 5 && app.start_time.is_some() {
                    Color::Red
                } else if app.start_time.is_some() {
                    Color::White
                } else {
                    Color::DarkGray
                })
                .add_modifier(Modifier::BOLD),
        ),
    ];

    if app.store.config.live_wpm && app.start_time.is_some() {
        let live_wpm = app.current_wpm();
        spans.push(Span::styled("  ·  ", Style::default().fg(Color::DarkGray)));
        spans.push(Span::styled(format!("{:.0} wpm", live_wpm), Style::default().fg(Color::Rgb(130, 130, 130))));
    }

    if app.store.config.sound_enabled {
        spans.push(Span::styled("  ·  ", Style::default().fg(Color::DarkGray)));
        spans.push(Span::styled("audio", Style::default().fg(Color::Rgb(90, 90, 90))));
    }

    let p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    f.render_widget(p, area);
}

// центральная бегущая строка с плавным затуханием
fn render_single_line_stream(f: &mut Frame, app: &App, area: Rect) {
    let width = area.width as usize;
    if width == 0 {
        return;
    }

    // фиксируем курсор на 35% ширины экрана, как на конвейере
    let center_pos = (width as f32 * 0.35) as usize;
    let typed_idx = app.typed_chars.len();
    let accent = app.theme_accent();
    let visual_style = app.store.config.visual;

    let start_in_target = typed_idx.saturating_sub(center_pos);
    let leading_spaces = center_pos.saturating_sub(typed_idx);

    let mut spans = Vec::new();

    if leading_spaces > 0 {
        spans.push(Span::raw(" ".repeat(leading_spaces)));
    }

    let mut current_slot = leading_spaces;
    let mut i = start_in_target;

    while current_slot < width && i < app.target_chars.len() {
        let target_c = app.target_chars[i];

        if i < typed_idx {
            // уже набранные символы
            let typed_c = app.typed_chars[i];
            let dist_from_cursor = typed_idx - i;

            if typed_c == target_c {
                // плавный фейд влево :3
                let color = match visual_style {
                    VisualStyle::Minimal => Color::Rgb(50, 50, 50),
                    _ => {
                        if dist_from_cursor > 20 {
                            Color::Rgb(55, 55, 55)
                        } else if dist_from_cursor > 10 {
                            Color::Rgb(90, 90, 90)
                        } else {
                            Color::Rgb(140, 140, 140)
                        }
                    }
                };
                spans.push(Span::styled(target_c.to_string(), Style::default().fg(color)));
            } else {
                // опечатка подсвечивается красным
                let display = if target_c == ' ' { "·" } else { &target_c.to_string() };
                spans.push(Span::styled(
                    display.to_string(),
                    Style::default().fg(Color::Red).add_modifier(Modifier::UNDERLINED),
                ));
            }
        } else if i == typed_idx {
            // активный курсор под стиль
            match visual_style {
                VisualStyle::Tape => {
                    spans.push(Span::styled(
                        target_c.to_string(),
                        Style::default().fg(Color::Black).bg(accent).add_modifier(Modifier::BOLD),
                    ));
                }
                VisualStyle::Underline => {
                    spans.push(Span::styled(
                        target_c.to_string(),
                        Style::default().fg(accent).add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
                    ));
                }
                VisualStyle::Blocky => {
                    spans.push(Span::styled(
                        format!("▊{}", target_c),
                        Style::default().fg(accent).add_modifier(Modifier::BOLD),
                    ));
                }
                VisualStyle::Minimal => {
                    spans.push(Span::styled(
                        target_c.to_string(),
                        Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                    ));
                }
            }
        } else {
            // будущие буквы, затухающие вправо
            let dist_ahead = i - typed_idx;
            let color = match visual_style {
                VisualStyle::Minimal => {
                    if dist_ahead < 8 {
                        Color::Rgb(180, 180, 180)
                    } else if dist_ahead < 20 {
                        Color::Rgb(100, 100, 100)
                    } else {
                        Color::Rgb(40, 40, 40)
                    }
                }
                _ => {
                    if dist_ahead < 12 {
                        Color::Rgb(225, 225, 225)
                    } else if dist_ahead < 28 {
                        Color::Rgb(145, 145, 145)
                    } else if dist_ahead < 45 {
                        Color::Rgb(85, 85, 85)
                    } else {
                        Color::Rgb(45, 45, 45)
                    }
                }
            };
            spans.push(Span::styled(target_c.to_string(), Style::default().fg(color)));
        }

        current_slot += 1;
        i += 1;
    }

    let p = Paragraph::new(Line::from(spans));
    f.render_widget(p, area);
}

// аккуратная строка хоткеев в самом низу экрана
fn render_bottom_bar(f: &mut Frame, _app: &App, area: Rect) {
    let hints = vec![
        Span::styled("tab", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" restart    ", Style::default().fg(Color::Rgb(50, 50, 50))),
        Span::styled("f1", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" mode    ", Style::default().fg(Color::Rgb(50, 50, 50))),
        Span::styled("f2", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" time    ", Style::default().fg(Color::Rgb(50, 50, 50))),
        Span::styled("f3", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" stats    ", Style::default().fg(Color::Rgb(50, 50, 50))),
        Span::styled("f4", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" config    ", Style::default().fg(Color::Rgb(50, 50, 50))),
        Span::styled("esc", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" restart    ", Style::default().fg(Color::Rgb(50, 50, 50))),
        Span::styled("esc esc", Style::default().fg(Color::Rgb(95, 95, 95))),
        Span::styled(" quit", Style::default().fg(Color::Rgb(50, 50, 50))),
    ];
    let p = Paragraph::new(Line::from(hints)).alignment(Alignment::Center);
    f.render_widget(p, area);
}

// меню настроек
fn render_settings_view(f: &mut Frame, app: &App, size: Rect) {
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(18), Constraint::Fill(1)])
        .split(size);

    let accent = app.theme_accent();
    let mut lines = Vec::new();

    lines.push(Line::from(Span::styled("settings", Style::default().fg(accent).add_modifier(Modifier::BOLD))));
    lines.push(Line::from(Span::styled("────────", Style::default().fg(Color::Rgb(50, 50, 50)))));
    lines.push(Line::from(""));

    let items = [
        ("sound effects", if app.store.config.sound_enabled { "on" } else { "off" }),
        ("punctuation", if app.store.config.punctuation { "on" } else { "off" }),
        ("numbers", if app.store.config.numbers { "on" } else { "off" }),
        ("live wpm", if app.store.config.live_wpm { "on" } else { "off" }),
        ("sudden death", if app.store.config.sudden_death { "on" } else { "off" }),
        ("theme", app.store.config.theme.name()),
        ("style", app.store.config.visual.name()),
    ];

    for (idx, (label, val)) in items.iter().enumerate() {
        let is_selected = idx == app.settings_cursor;
        let prefix = if is_selected { " > " } else { "   " };
        let style_label = if is_selected {
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Rgb(120, 120, 120))
        };
        let style_val = if is_selected {
            Style::default().fg(accent).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        lines.push(Line::from(vec![
            Span::styled(prefix, Style::default().fg(accent)),
            Span::styled(format!("{:<20} ", label), style_label),
            Span::styled(format!("[{}]", val), style_val),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑ / ↓ navigate   ·   space / enter toggle   ·   esc / tab return",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(p, v[1]);
}

// экран результатов раунда
fn render_minimal_result(f: &mut Frame, app: &App, size: Rect) {
    if let Some(res) = &app.last_result {
        let accent = app.theme_accent();
        let sparkline = render_sparkline(&res.wpm_sparkline);

        let v = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Fill(1),
            ])
            .split(size);

        let main_line = Line::from(vec![
            Span::styled(format!("{:.0}", res.wpm), Style::default().fg(accent).add_modifier(Modifier::BOLD)),
            Span::styled(" wpm   ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.1}%", res.accuracy), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
            Span::styled(" acc", Style::default().fg(Color::DarkGray)),
        ]);
        f.render_widget(Paragraph::new(main_line).alignment(Alignment::Center), v[1]);

        if !sparkline.is_empty() {
            let spark_line = Line::from(vec![Span::styled(sparkline, Style::default().fg(accent))]);
            f.render_widget(Paragraph::new(spark_line).alignment(Alignment::Center), v[2]);
        }

        let detail_line = Line::from(vec![
            Span::styled(format!("{} cpm", res.cpm as u64), Style::default().fg(Color::Rgb(120, 120, 120))),
            Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} err", res.incorrect_chars),
                Style::default().fg(if res.incorrect_chars > 0 { Color::Red } else { Color::DarkGray }),
            ),
            Span::styled("  ·  ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{} / {}s", res.mode_name, res.duration_secs), Style::default().fg(Color::Rgb(120, 120, 120))),
        ]);
        f.render_widget(Paragraph::new(detail_line).alignment(Alignment::Center), v[4]);

        let act_line = Line::from(vec![
            Span::styled("tab", Style::default().fg(accent)),
            Span::styled(" retry    ", Style::default().fg(Color::DarkGray)),
            Span::styled("f4", Style::default().fg(accent)),
            Span::styled(" config    ", Style::default().fg(Color::DarkGray)),
            Span::styled("f3", Style::default().fg(accent)),
            Span::styled(" stats    ", Style::default().fg(Color::DarkGray)),
            Span::styled("esc", Style::default().fg(accent)),
            Span::styled(" restart    ", Style::default().fg(Color::DarkGray)),
            Span::styled("esc esc", Style::default().fg(accent)),
            Span::styled(" quit", Style::default().fg(Color::DarkGray)),
        ]);
        f.render_widget(Paragraph::new(act_line).alignment(Alignment::Center), v[6]);
    }
}

// экран истории и рекордов
fn render_minimal_history(f: &mut Frame, app: &App, size: Rect) {
    let accent = app.theme_accent();
    let v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(14), Constraint::Fill(1)])
        .split(size);

    let mut lines = Vec::new();
    lines.push(Line::from(vec![Span::styled(
        format!("{:<16} {:<12} {:<6} {:<8} {:<8} {:<8}", "date", "mode", "time", "wpm", "cpm", "acc"),
        Style::default().fg(Color::DarkGray),
    )]));
    lines.push(Line::from(Span::styled(
        "────────────────────────────────────────────────────────────────",
        Style::default().fg(Color::Rgb(40, 40, 40)),
    )));

    if app.store.results.is_empty() {
        lines.push(Line::from(Span::styled("no records yet ):", Style::default().fg(Color::DarkGray))));
    } else {
        for res in app.store.results.iter().rev().take(8) {
            lines.push(Line::from(vec![
                Span::styled(format!("{:<16} ", res.date.format("%m-%d %H:%M")), Style::default().fg(Color::Rgb(100, 100, 100))),
                Span::styled(format!("{:<12} ", res.mode_name), Style::default().fg(Color::Rgb(140, 140, 140))),
                Span::styled(format!("{}s    ", res.duration_secs), Style::default().fg(Color::Rgb(100, 100, 100))),
                Span::styled(format!("{:<8.0} ", res.wpm), Style::default().fg(accent)),
                Span::styled(format!("{:<8.0} ", res.cpm), Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(format!("{:.1}%", res.accuracy), Style::default().fg(Color::White)),
            ]));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "press esc or tab to return",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(p, v[1]);
}
