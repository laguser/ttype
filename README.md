<p align="center">
  <b>minimalist terminal typing speed test</b>
  <br>
  <i>smooth one-line conveyor · mechanical sounds · rust</i>
</p>

<p align="center">
  <a href="https://crates.io/crates/ttype-cli"><img src="https://img.shields.io/crates/v/ttype-cli.svg?style=flat-square&color=orange" alt="crates.io"></a>
  <a href="https://crates.io/crates/ttype-cli"><img src="https://img.shields.io/crates/d/ttype-cli.svg?style=flat-square&color=blue" alt="downloads"></a>
  <img src="https://img.shields.io/github/license/laguser/ttype?style=flat-square&color=blue" alt="License">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/laguser/ttype/main/demo.gif" alt="ttype demo" width="750">
</p>

<p align="center">
  <a href="#english">English</a> • <a href="#русский">Русский</a>
</p>

---

<a name="english"></a>
## English

A keyboard-first, distraction-free typing speed test built for terminal purists. Features a smooth single-line conveyor tape, procedural mechanical typewriter audio, local statistics persistence, and multiple practice modes.

### Features
- **Transparent & Minimal:** No clunky boxes, zero clutter. Inherits your terminal's background transparency and color palette.
- **Single-Line Conveyor Tape:** Real-time text stream with smooth fade-in/fade-out animations.
- **Synthesized Audio:** Built-in procedural mechanical switch and carriage bell sounds (no external audio files required, toggleable).
- **Lightweight by Default:** English, code keywords, and classic quotes are built-in.
- **Dynamic Languages & Wordlists (`F5`):**
  - Download all Blind Typing & Monkeytype dictionaries: Russian, Spanish, German, French, Italian, Chinese (Pinyin), Japanese (Romaji), etc.
  - Vocabulary tiers: 1K, 2K, 5K, 10K words.
  - Programming languages: Rust, Python, JavaScript, C, Go, HTML.
  - Epic literature collections: 1200+ quotes from *War & Peace* in Russian and 6400+ quotes in English.
  - Instant one-key deletion (`d` or `del`).
  - Real-time download progress bar.
- **Multi-CDN Mirror Switcher:** Fast global downloads via `moscow (vps)`, `germany (vps)`, `jsdelivr (fast edge cdn)`, or `github (raw)`.
- **Configurable Tests:** 15s / 30s timers, punctuation, numbers, and hardcore sudden death mode.
- **Local Stats & Sparkline:** Tracks WPM, CPM, accuracy, mistakes, and draws a real-time WPM sparkline chart per test.
- **Themes & Styles:** `auto (term)`, `catppuccin`, `tokyo night`, `nord`, `gruvbox`, `dracula`, `amber`, `white` across 4 cursor styles (`tape`, `underline`, `blocky`, `minimal`).

### Installation

#### Via Cargo (Recommended)
```bash
cargo install ttype-cli
```

#### Quick One-Liner (Build & Install)
```bash
curl -sSL https://raw.githubusercontent.com/laguser/ttype/main/install.sh | bash
```

#### From Source
Requires Rust (`cargo`):
```bash
git clone https://github.com/laguser/ttype.git
cd ttype
cargo build --release
cp target/release/ttype ~/.local/bin/
```

### Keybindings
| Key | Action |
| :--- | :--- |
| `tab` | Restart / New test sheet |
| `f1` | Switch mode (`en` ➔ `code` ➔ `quotes` ➔ custom downloaded language) |
| `f2` | Toggle duration (15s / 30s) |
| `f3` | Local records & history |
| `f4` | Settings menu |
| `f5` | Languages & CDN mirrors manager |
| `d` / `del` *(in F5)* | Delete downloaded dictionary |
| `m` *(in F5)* | Cycle CDN download mirrors |
| `esc` | Restart / Back to typing screen |
| `esc esc` / `ctrl+c` | Exit |

---

<a name="русский"></a>
## Русский

Ультра-минималистичный терминальный тренажёр слепой печати на Rust. Никаких рамок и громоздкого интерфейса — только одна плавная бегущая строка, процедурный звук механики и чистая прозрачность твоего терминала.

### Особенности
- **Прозрачность и минимализм:** Наследует фон и акценты твоего эмулятора терминала без хардкодных цветов.
- **Бегущая строка (лента):** Курсор зафиксирован на комфортной позиции, текст плавно уезжает влево и мягко проявляется справа.
- **Процедурный звук механики:** Аудиосинтез щелчков клавиш, глухого пробела и звонка каретки прямо в коде (без внешних .wav файлов).
- **Компактный размер «из коробки»:** Английский язык, код и цитаты встроены в бинарник.
- **Менеджер языков и словарей (`F5`):**
  - Подгрузка всех словарей из Blind Typing / Monkeytype: русский, испанский, немецкий, французский, итальянский, китайский (пиньинь), японский (ромадзи) и др.
  - Уровни сложности и словарного запаса: 1K, 2K, 5K, 10K слов.
  - Языки программирования: Rust, Python, JavaScript, C, Go, HTML.
  - Избранные цитаты из классики: 1160+ цитат из «Войны и мира» Льва Толстого на русском и 6480+ цитат на английском.
  - Удаление словарей в одно нажатие (`d` / `del`).
  - Ч/Б прогресс-бар скачивания `[■■■■□□□□]`.
- **Сеть зеркал загрузки:** Быстрое переключение серверов на лету (`moscow vps`, `germany vps`, `jsdelivr cdn`, `github raw`).
- **Гибкая настройка:** Таймеры 15с / 30с, знаки препинания, цифры и режим внезапной смерти (`sudden death`).
- **Локальная история и график:** Сохранение результатов в `~/.ttype/history.json` с выводом спарклайн-графика скорости (` ▂▃▅▆▇█`).
- **Стили и темы:** Адаптация `auto (term)`, а также `catppuccin`, `tokyo night`, `nord`, `gruvbox` и 4 стиля курсора.

### Установка

#### Через Cargo (Рекомендуется):
```bash
cargo install ttype-cli
```

#### Быстрая установка одной командой:
```bash
curl -sSL https://raw.githubusercontent.com/laguser/ttype/main/install.sh | bash
```

#### Ручная сборка из исходников:
Нужен Rust (`cargo`):
```bash
git clone https://github.com/laguser/ttype.git
cd ttype
cargo build --release
cp target/release/ttype ~/.local/bin/
```

### Управление
| Клавиша | Действие |
| :--- | :--- |
| `tab` | Рестарт теста |
| `f1` | Переключение режима (`en` ➔ `code` ➔ `quotes` ➔ скачанный язык) |
| `f2` | Время (15s / 30s) |
| `f3` | История и рекорды |
| `f4` | Настройки |
| `f5` | Меню языков и CDN-зеркал |
| `d` / `del` *(в F5)* | Удалить скачанный словарь |
| `m` *(в F5)* | Переключить зеркало загрузки (Москва / Германия / jsDelivr / GitHub) |
| `esc` | Сброс / Возврат к печати |
| `esc esc` / `ctrl+c` | Выход |

---

### License
MIT
