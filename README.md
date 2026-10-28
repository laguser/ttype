# ttype

minimalist terminal typing speed test

[![crates.io](https://img.shields.io/crates/v/ttype-cli?style=flat-square&color=black)](https://crates.io/crates/ttype-cli)
[![license](https://img.shields.io/badge/license-MIT-black?style=flat-square)](LICENSE)

```
  f5 langs   f4 config   f3 stats   tab restart   esc quit
  ───────────────────────────────────────────────────────────
   → through  quietly  whispering  under  the  pale  moonlight
```

### Features

- **Conveyor tape:** smooth single-line stream with fade transitions
- **Monochrome aesthetic:** inherits terminal background transparency
- **Procedural sound:** synthesized mechanical switches and carriage bell
- **Flexible goals (`F2`):** time modes (`15s`, `30s`) and word count modes (`10w`, `25w`, `50w`, `100w`)
- **Key error analysis:** identifies most missed keys after each test
- **Keyboard Heatmap (`k` in results):** visual error map showing lifetime mistyped keys
- **Accidental exit protection:** prevents accidental skipping of result screen while typing fast
- **Wordlists on demand (`F5`):** Blind Typing dicts, 1K–10K tiers, code, War & Peace
- **Local stats:** WPM sparkline chart (` ▂▃▅▆▇█`), accuracy, CPM, history

### Install

```bash
cargo install ttype-cli
```

#### Homebrew
```bash
brew install laguser/ttype/ttype
```

#### From Source
```bash
git clone https://github.com/laguser/ttype.git
cd ttype
cargo build --release
install -m 755 target/release/ttype ~/.local/bin/
```

### Keybindings

| Key | Action |
|:---|:---|
| `tab` | Restart test / new line |
| `f1` | Cycle mode (`en` → `code` → `quotes` → dict) |
| `f2` | Cycle goal (`15s` → `30s` → `10w` → `25w` → `50w` → `100w`) |
| `f3` | History & records |
| `f4` | Settings |
| `f5` | Languages manager |
| `k` | Keyboard error heatmap *(in results)* |
| `d` / `del` | Delete dictionary *(in F5)* |
| `esc` | Back to typing |
| `esc esc` | Quit |

### License

MIT
