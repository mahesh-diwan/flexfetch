#compdef flexfetch

autoload -U is-at-least

_flexfetch() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-c+[]:CONFIG:_default' \
'--config=[]:CONFIG:_default' \
'-m+[]:MODULES:_default' \
'--modules=[]:MODULES:_default' \
'-t+[]:TEMPLATE:_default' \
'--template=[]:TEMPLATE:_default' \
'-f+[]:FORMAT:_default' \
'--format=[]:FORMAT:_default' \
'--theme=[]:THEME:_default' \
'--benchmark=[Micro-benchmark\: \`--benchmark\` (per-module timing) or \`--benchmark N\` (run each module N times, report min/avg/total)]::BENCHMARK:_default' \
'--preset=[]:PRESET:_default' \
'--export=[]:EXPORT:_default' \
'-o+[]:OUTPUT:_files' \
'--output=[]:OUTPUT:_files' \
'--box-style=[]:BOX_STYLE:_default' \
'--palette-style=[]:PALETTE_STYLE:_default' \
'--frame=[]:FRAME:_default' \
'--watch-interval=[]:WATCH_INTERVAL:_default' \
'*--ssh=[Fetch remote system info via SSH (repeatable, parallel)]:SSH:_default' \
'--debug[]' \
'--gen-config[]' \
'--list-modules[]' \
'--list-presets[]' \
'--pipe[]' \
'--minimal[]' \
'--full[]' \
'--dev[]' \
'--no-gradient[]' \
'--no-progress[]' \
'--pixel-logo[]' \
'--watch[]' \
'--live[Live dashboard\: real-time CPU/memory gauges, top processes, network rates]' \
'--smart[Smart fetch\: add \$PWD context (git branch/status, project type, container/venv/SSH)]' \
'--health[Add the system health module (score 0-100\: disk/swap/load/battery)]' \
'--prompt[Single-line prompt string (e.g. \`🐧 arch | CPU 12% | RAM 3.2G\`)]' \
'--motd[Plain-text banner (ANSI colors stripped) for MOTD/startup]' \
'--wizard[Interactive config wizard (writes ~/.config/flexfetch/config.toml)]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_flexfetch_commands" \
"*::: :->flexfetch" \
&& ret=0
    case $state in
    (flexfetch)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:flexfetch-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
':shell:(bash elvish fish powershell zsh)' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_flexfetch__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:flexfetch-help-command-$line[1]:"
        case $line[1] in
            (completions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_flexfetch_commands] )) ||
_flexfetch_commands() {
    local commands; commands=(
'completions:Generate shell completions for the given shell' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'flexfetch commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__completions_commands] )) ||
_flexfetch__subcmd__completions_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch completions commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help_commands] )) ||
_flexfetch__subcmd__help_commands() {
    local commands; commands=(
'completions:Generate shell completions for the given shell' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'flexfetch help commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help__subcmd__completions_commands] )) ||
_flexfetch__subcmd__help__subcmd__completions_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch help completions commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help__subcmd__help_commands] )) ||
_flexfetch__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch help help commands' commands "$@"
}

if [ "$funcstack[1]" = "_flexfetch" ]; then
    _flexfetch "$@"
else
    compdef _flexfetch flexfetch
fi
