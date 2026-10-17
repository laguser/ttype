<p align="center">
  <h1 align="center">ttype</h1>
  <p align="center"><b>minimalist terminal typing test</b></p>
  <p align="center">
    <a href="https://crates.io/crates/ttype-cli"><img src="https://img.shields.io/crates/v/ttype-cli?style=flat-square&color=white" alt="crates.io"></a>
    <a href="https://crates.io/crates/ttype-cli"><img src="https://img.shields.io/crates/d/ttype-cli?style=flat-square&color=white" alt="downloads"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-white?style=flat-square" alt="license"></a>
  </p>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/laguser/ttype/main/demo.gif" width="720" alt="ttype preview">
</p>

```
  f5 langs   f4 config   f3 stats   tab restart   esc quit
  ───────────────────────────────────────────────────────────
   → through  quietly  whispering  under  the  pale  moonlight
```

---

### Features

- **Single-line conveyor:** Fixed cursor tape with smooth fade transitions.
- **Pure terminal aesthetic:** Inherits background transparency and monochrome palette.
- **Procedural sound:** Built-in mechanical switch & carriage bell audio synthesis.
- **Dynamic wordlists (`F5`):**
  - Instant download: Russian, Spanish, German, French, Chinese, Japanese, etc.
  - Vocabulary tiers: 1K, 2K, 5K, 10K.
  - Code syntax: Rust, Python, JavaScript, C, Go, HTML.
  - Classic literature: 1200+ Russian and 6400+ English quotes from *War & Peace*.
- **Multi-CDN mirrors:** Moscow VPS, Germany VPS, jsDelivr Global Edge, GitHub Raw.
- **Analytics:** Real-time WPM sparkline chart (` ▂▃▅▆▇█`), accuracy, CPM, local records.

---

### Install

```bash
# Cargo (crates.io)
cargo install ttype-cli

# Quick script
curl -sSL https://raw.githubusercontent.com/laguser/ttype/main/install.sh | bash

# Source
git clone https://github.com/laguser/ttype.git && cd ttype
cargo build --release && cp target/release/ttype ~/.local/bin/
```

---

### Controls

| Key | Action |
|:---|:---|
| `tab` | Restart test / new line |
| `f1` | Cycle mode (`en` → `code` → `quotes` → installed dicts) |
| `f2` | Cycle timer (`15s` / `30s`) |
| `f3` | Local records & stats |
| `f4` | Settings (audio, live wpm, punctuation, theme) |
| `f5` | Languages manager & CDN mirrors |
| `d` / `del` | Delete dictionary *(in F5)* |
| `m` | Switch CDN mirror *(in F5 / F4)* |
| `esc` | Return to typing |
| `esc esc` | Quit |

---

### License

MIT © [laguser](https://github.com/laguser)
