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
'--record=[Phase 5.10\: record the --live dashboard to an asciinema v2 cast file (e.g. --live --record flexfetch.cast; replay with \`asciinema play\`)]:RECORD:_files' \
'*--ssh=[Fetch remote system info via SSH (repeatable, parallel)]:SSH:_default' \
'*--diff=[Diff mode (Phase 4.9)\: compare two systems side-by-side. Each target is \`local\`, \`host@remote\`, or a path to a flexfetch JSON export file]:DIFF:_default:DIFF:_default' \
'--import-qr=[Import a config from a QR-code image (PNG/etc; decoded via rqrr) and write it to the config path (existing file is backed up)]:IMPORT_QR:_files' \
'--hook=[Print a shell hook (bash|zsh|fish) for cd-into-git-repo context fetches]:HOOK:_default' \
'--history-interval=[Phase 5.5\: record interval for --history / --daemon (seconds)]:HISTORY_INTERVAL:_default' \
'--history-graph=[Phase 5.5\: print an ASCII sparkline of the recorded metric over the last --hours (cpu|memory|disk|temp; requires the \`history\` feature)]:HISTORY_GRAPH:_default' \
'--hours=[Phase 5.5\: window for --history-graph, in hours]:HOURS:_default' \
'--history-export=[Phase 5.5\: export the snapshots table to a CSV file]:HISTORY_EXPORT:_files' \
'*--threshold=[Phase 5.6\: threshold overrides, e.g. --threshold cpu=95,mem=88,temp=80]:THRESHOLD:_default' \
'--debug[]' \
'--gen-config[]' \
'--list-modules[]' \
'--list-presets[]' \
'--list-themes[List all built-in theme presets (Phase 7.8 — pairs with \`--theme random\`)]' \
'--pipe[]' \
'--minimal[]' \
'--full[]' \
'--dev[]' \
'--no-gradient[]' \
'--no-progress[]' \
'--pixel-logo[]' \
'--watch[]' \
'--live[Live dashboard\: real-time CPU/memory gauges, top processes, network rates]' \
'--bench-cpu[SIMD CPU micro-benchmark (Phase 4.3)\: vectorized integer benchmark with runtime AVX2/SSE4/NEON detection, scalar fallback]' \
'--bench-memory[SIMD memory-bandwidth micro-benchmark (Phase 4.3)]' \
'--smart[Smart fetch\: add \$PWD context (git branch/status, project type, container/venv/SSH)]' \
'--health[Add the system health module (score 0-100\: disk/swap/load/battery)]' \
'--prompt[Single-line prompt string (e.g. \`🐧 arch | CPU 12% | RAM 3.2G\`)]' \
'--motd[Plain-text banner (ANSI colors stripped) for MOTD/startup]' \
'--wizard[Interactive config wizard (writes ~/.config/flexfetch/config.toml)]' \
'--qr[Render the effective config as a terminal QR code (base64+zstd payload, unicode blocks). Scan it with a phone to import on another machine]' \
'--update[Self-update\: check the latest GitHub release and re-run the install script if a newer version exists (requires curl)]' \
'--doctor[Environment doctor\: validate terminal, color, config, and collectors]' \
'--tmux-config[Print a tmux.conf snippet that auto-runs the fetch in new idle panes (Phase 5.3 — pair with the bundled \`flexfetch-tmux\` helper binary)]' \
'--update-db[Refresh the crowdsourced hardware database (Phase 5.8)\: downloads the latest PCI/USB name map to the cache dir; falls back to the bundled seed when offline]' \
'--auto-theme[Phase 5.4\: derive the theme from the wallpaper'\''s dominant colors (requires the \`auto-theme\` feature; falls back to catppuccin otherwise)]' \
'--history[Phase 5.5\: record cpu/mem/disk/temp snapshots to history.db every --history-interval seconds until Ctrl+C (requires the \`history\` feature)]' \
'--daemon[Phase 5.6\: critical health notifications daemon — poll every --history-interval seconds, notify on threshold breach (requires the \`notifications\` feature)]' \
'--demo[Phase 8.8\: showcase mode — every module + every visual feature, for screenshots / social previews / \`install.sh\` first-run demos]' \
'--bug-report[Phase 8.7\: print a full environment/version dump for bug reports (version, OS, kernel, terminal, shell, config, module errors)]' \
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
(plugin)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
":: :_flexfetch__subcmd__plugin_commands" \
"*::: :->plugin" \
&& ret=0

    case $state in
    (plugin)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:flexfetch-plugin-command-$line[1]:"
        case $line[1] in
            (search)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
':query:_default' \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
':name:_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_flexfetch__subcmd__plugin__subcmd__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:flexfetch-plugin-help-command-$line[1]:"
        case $line[1] in
            (search)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
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
(plugin)
_arguments "${_arguments_options[@]}" : \
":: :_flexfetch__subcmd__help__subcmd__plugin_commands" \
"*::: :->plugin" \
&& ret=0

    case $state in
    (plugin)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:flexfetch-help-plugin-command-$line[1]:"
        case $line[1] in
            (search)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(install)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
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
'plugin:Plugin registry (Phase 5.7)\: search/install/list/update Lua plugins against the hosted registry.toml (checksum + min-version verified)' \
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
'plugin:Plugin registry (Phase 5.7)\: search/install/list/update Lua plugins against the hosted registry.toml (checksum + min-version verified)' \
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
(( $+functions[_flexfetch__subcmd__help__subcmd__plugin_commands] )) ||
_flexfetch__subcmd__help__subcmd__plugin_commands() {
    local commands; commands=(
'search:Search the hosted registry by name/description' \
'install:Install a plugin (SHA-256 verified, min-version gated)' \
'list:List installed plugins + registry status' \
'update:Re-install every installed plugin still in the registry' \
    )
    _describe -t commands 'flexfetch help plugin commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help__subcmd__plugin__subcmd__install_commands] )) ||
_flexfetch__subcmd__help__subcmd__plugin__subcmd__install_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch help plugin install commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help__subcmd__plugin__subcmd__list_commands] )) ||
_flexfetch__subcmd__help__subcmd__plugin__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch help plugin list commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help__subcmd__plugin__subcmd__search_commands] )) ||
_flexfetch__subcmd__help__subcmd__plugin__subcmd__search_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch help plugin search commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__help__subcmd__plugin__subcmd__update_commands] )) ||
_flexfetch__subcmd__help__subcmd__plugin__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch help plugin update commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin_commands] )) ||
_flexfetch__subcmd__plugin_commands() {
    local commands; commands=(
'search:Search the hosted registry by name/description' \
'install:Install a plugin (SHA-256 verified, min-version gated)' \
'list:List installed plugins + registry status' \
'update:Re-install every installed plugin still in the registry' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'flexfetch plugin commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__help_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__help_commands() {
    local commands; commands=(
'search:Search the hosted registry by name/description' \
'install:Install a plugin (SHA-256 verified, min-version gated)' \
'list:List installed plugins + registry status' \
'update:Re-install every installed plugin still in the registry' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'flexfetch plugin help commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__help__subcmd__help_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__help__subcmd__help_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin help help commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__help__subcmd__install_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__help__subcmd__install_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin help install commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__help__subcmd__list_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__help__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin help list commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__help__subcmd__search_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__help__subcmd__search_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin help search commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__help__subcmd__update_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__help__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin help update commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__install_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__install_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin install commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__list_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__list_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin list commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__search_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__search_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin search commands' commands "$@"
}
(( $+functions[_flexfetch__subcmd__plugin__subcmd__update_commands] )) ||
_flexfetch__subcmd__plugin__subcmd__update_commands() {
    local commands; commands=()
    _describe -t commands 'flexfetch plugin update commands' commands "$@"
}

if [ "$funcstack[1]" = "_flexfetch" ]; then
    _flexfetch "$@"
else
    compdef _flexfetch flexfetch
fi
