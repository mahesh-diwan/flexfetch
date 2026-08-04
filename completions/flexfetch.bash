_flexfetch() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="flexfetch"
                ;;
            flexfetch,completions)
                cmd="flexfetch__subcmd__completions"
                ;;
            flexfetch,help)
                cmd="flexfetch__subcmd__help"
                ;;
            flexfetch,plugin)
                cmd="flexfetch__subcmd__plugin"
                ;;
            flexfetch__subcmd__help,completions)
                cmd="flexfetch__subcmd__help__subcmd__completions"
                ;;
            flexfetch__subcmd__help,help)
                cmd="flexfetch__subcmd__help__subcmd__help"
                ;;
            flexfetch__subcmd__help,plugin)
                cmd="flexfetch__subcmd__help__subcmd__plugin"
                ;;
            flexfetch__subcmd__help__subcmd__plugin,install)
                cmd="flexfetch__subcmd__help__subcmd__plugin__subcmd__install"
                ;;
            flexfetch__subcmd__help__subcmd__plugin,list)
                cmd="flexfetch__subcmd__help__subcmd__plugin__subcmd__list"
                ;;
            flexfetch__subcmd__help__subcmd__plugin,search)
                cmd="flexfetch__subcmd__help__subcmd__plugin__subcmd__search"
                ;;
            flexfetch__subcmd__help__subcmd__plugin,update)
                cmd="flexfetch__subcmd__help__subcmd__plugin__subcmd__update"
                ;;
            flexfetch__subcmd__plugin,help)
                cmd="flexfetch__subcmd__plugin__subcmd__help"
                ;;
            flexfetch__subcmd__plugin,install)
                cmd="flexfetch__subcmd__plugin__subcmd__install"
                ;;
            flexfetch__subcmd__plugin,list)
                cmd="flexfetch__subcmd__plugin__subcmd__list"
                ;;
            flexfetch__subcmd__plugin,search)
                cmd="flexfetch__subcmd__plugin__subcmd__search"
                ;;
            flexfetch__subcmd__plugin,update)
                cmd="flexfetch__subcmd__plugin__subcmd__update"
                ;;
            flexfetch__subcmd__plugin__subcmd__help,help)
                cmd="flexfetch__subcmd__plugin__subcmd__help__subcmd__help"
                ;;
            flexfetch__subcmd__plugin__subcmd__help,install)
                cmd="flexfetch__subcmd__plugin__subcmd__help__subcmd__install"
                ;;
            flexfetch__subcmd__plugin__subcmd__help,list)
                cmd="flexfetch__subcmd__plugin__subcmd__help__subcmd__list"
                ;;
            flexfetch__subcmd__plugin__subcmd__help,search)
                cmd="flexfetch__subcmd__plugin__subcmd__help__subcmd__search"
                ;;
            flexfetch__subcmd__plugin__subcmd__help,update)
                cmd="flexfetch__subcmd__plugin__subcmd__help__subcmd__update"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        flexfetch)
            opts="-c -m -t -f -o -h --config --modules --template --format --theme --debug --gen-config --list-modules --list-presets --list-themes --benchmark --pipe --minimal --full --dev --preset --export --output --no-gradient --no-progress --box-style --pixel-logo --palette-style --frame --watch --watch-interval --live --record --bench-cpu --bench-memory --smart --health --prompt --motd --ssh --diff --wizard --qr --import-qr --update --doctor --hook --tmux-config --update-db --auto-theme --history --history-interval --history-graph --hours --history-export --daemon --threshold --demo --bug-report --help completions plugin help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --modules)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --template)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --theme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --benchmark)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --preset)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --export)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --box-style)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --palette-style)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --frame)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --watch-interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --record)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --ssh)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --diff)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --import-qr)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --hook)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --history-interval)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --history-graph)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --hours)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --history-export)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threshold)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__completions)
            opts="-h --help bash elvish fish powershell zsh"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help)
            opts="completions plugin help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__completions)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__plugin)
            opts="search install list update"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__plugin__subcmd__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__plugin__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__plugin__subcmd__search)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__help__subcmd__plugin__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin)
            opts="-h --help search install list update help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__help)
            opts="search install list update help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__help__subcmd__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__help__subcmd__install)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__help__subcmd__list)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__help__subcmd__search)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__help__subcmd__update)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__install)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__list)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__search)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        flexfetch__subcmd__plugin__subcmd__update)
            opts="-h --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _flexfetch -o nosort -o bashdefault -o default flexfetch
else
    complete -F _flexfetch -o bashdefault -o default flexfetch
fi
