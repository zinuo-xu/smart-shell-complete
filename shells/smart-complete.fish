# smart-shell-complete - Fish shell integration
# Install: source this file in your config.fish or place in
#          ~/.config/fish/completions/smart-complete.fish

function __smart_complete_init -d "Initialize smart-shell-complete for fish"
    set -l smart_bin (which smart-shell-complete 2>/dev/null)

    if test -z "$smart_bin"
        for path in ~/.cargo/bin/smart-shell-complete /usr/local/bin/smart-shell-complete /usr/bin/smart-shell-complete
            if test -x "$path"
                set smart_bin "$path"
                break
            end
        end
    end

    if test -z "$smart_bin"
        echo "smart-shell-complete: binary not found" >&2
        return 1
    end

    set -g __smart_complete_bin "$smart_bin"

    if not test -f ~/.cache/smart-shell-complete/initialized
        mkdir -p ~/.cache/smart-shell-complete
        "$smart_bin" learn --shell fish 2>/dev/null
        touch ~/.cache/smart-shell-complete/initialized
    end

    return 0
end

function __smart_complete_predict -d "Get smart command predictions"
    set -l count 5
    if set -q argv[1]
        set count $argv[1]
    end

    if not set -q __smart_complete_bin
        return 1
    end

    "$__smart_complete_bin" predict --count $count 2>/dev/null
end

function __smart_complete_complete -d "Complete a command prefix"
    set -l prefix "$argv[1]"

    if not set -q __smart_complete_bin
        return 1
    end

    "$__smart_complete_bin" complete --prefix "$prefix" --count 10 2>/dev/null
end

# Initialize
__smart_complete_init
