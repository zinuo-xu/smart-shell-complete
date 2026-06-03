# smart-shell-complete - Zsh shell integration
# Install: source this file in your .zshrc or place in
#          /usr/local/share/zsh/site-functions/_smart_complete

__smart_complete_init() {
    local smart_bin

    smart_bin=$(command -v smart-shell-complete 2>/dev/null)
    if [[ -z "$smart_bin" ]]; then
        for path in ~/.cargo/bin/smart-shell-complete /usr/local/bin/smart-shell-complete /usr/bin/smart-shell-complete; do
            if [[ -x "$path" ]]; then
                smart_bin="$path"
                break
            fi
        done
    fi

    if [[ -z "$smart_bin" ]]; then
        echo "smart-shell-complete: binary not found" >&2
        return 1
    fi

    typeset -g __SMART_COMPLETE_BIN="$smart_bin"

    local cache_dir="${HOME}/.cache/smart-shell-complete"
    if [[ ! -f "${cache_dir}/initialized" ]]; then
        mkdir -p "$cache_dir"
        "$smart_bin" learn --shell zsh 2>/dev/null
        touch "${cache_dir}/initialized"
    fi

    return 0
}

__smart_complete_predict() {
    local count=${1:-5}
    if [[ -z "$__SMART_COMPLETE_BIN" ]]; then
        return 1
    fi
    "${__SMART_COMPLETE_BIN}" predict --count "$count" 2>/dev/null
}

__smart_complete_complete() {
    local prefix="$1"
    if [[ -z "$__SMART_COMPLETE_BIN" ]]; then
        return 1
    fi
    "${__SMART_COMPLETE_BIN}" complete --prefix "$prefix" --count 10 2>/dev/null
}

__smart_complete_init
