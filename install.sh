#!/usr/bin/env bash
set -e

# ttype installer script (clean & minimal)
LANG_PREF="${LANG:-en}"

is_ru() {
    [[ "$LANG_PREF" =~ ^ru ]]
}

msg() {
    if is_ru; then
        echo -e "\033[1;33m==>\033[0m \033[1m$1\033[0m"
    else
        echo -e "\033[1;33m==>\033[0m \033[1m$2\033[0m"
    fi
}

err() {
    if is_ru; then
        echo -e "\033[1;31m[ошибка]\033[0m $1" >&2
    else
        echo -e "\033[1;31m[error]\033[0m $2" >&2
    fi
    exit 1
}

# check dependencies
command -v git >/dev/null 2>&1 || err "git не найден. Установите git." "git is required but not installed."
command -v cargo >/dev/null 2>&1 || err "cargo (Rust) не найден. Установите rustup." "cargo (Rust) is required. Install rustup."

TMP_DIR="$(mktemp -d /tmp/ttype-build.XXXXXX)"
cleanup() {
    rm -rf "$TMP_DIR"
}
trap cleanup EXIT

msg "Клонирование репозитория..." "Cloning repository..."
git clone --depth 1 https://github.com/laguser/ttype.git "$TMP_DIR" >/dev/null 2>&1 || {
    # fallback to local repo if running locally
    if [ -f "./Cargo.toml" ]; then
        cp -r ./* "$TMP_DIR/"
    else
        err "Не удалось загрузить репозиторий" "Failed to clone repository"
    fi
}

msg "Сборка релизного бинарника..." "Compiling release binary..."
cd "$TMP_DIR"
cargo build --release --quiet

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"
cp target/release/ttype "$INSTALL_DIR/ttype"
chmod +x "$INSTALL_DIR/ttype"

msg "Готово! Установлено в $INSTALL_DIR/ttype" "Done! Installed to $INSTALL_DIR/ttype"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    if is_ru; then
        echo -e "\033[0;33mВнимание:\033[0m $INSTALL_DIR отсутствует в вашем \$PATH."
        echo "Добавьте в ~/.bashrc или ~/.zshrc: export PATH=\"\$HOME/.local/bin:\$PATH\""
    else
        echo -e "\033[0;33mNote:\033[0m $INSTALL_DIR is not in your \$PATH."
        echo "Add to ~/.bashrc or ~/.zshrc: export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi
fi

if is_ru; then
    echo -e "Запуск: \033[1;32mttype\033[0m"
else
    echo -e "Run: \033[1;32mttype\033[0m"
fi
