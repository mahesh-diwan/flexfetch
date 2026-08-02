# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_flexfetch_global_optspecs
    string join \n c/config= m/modules= t/template= f/format= theme= debug gen-config list-modules list-presets benchmark= pipe minimal full dev preset= export= o/output= no-gradient no-progress box-style= pixel-logo palette-style= frame= watch watch-interval= live smart health prompt motd ssh= wizard h/help
end

function __fish_flexfetch_needs_command
    # Figure out if the current invocation already has a command.
    set -l cmd (commandline -opc)
    set -e cmd[1]
    argparse -s (__fish_flexfetch_global_optspecs) -- $cmd 2>/dev/null
    or return
    if set -q argv[1]
        # Also print the command, so this can be used to figure out what it is.
        echo $argv[1]
        return 1
    end
    return 0
end

function __fish_flexfetch_using_subcommand
    set -l cmd (__fish_flexfetch_needs_command)
    test -z "$cmd"
    and return 1
    contains -- $cmd[1] $argv
end

complete -c flexfetch -n "__fish_flexfetch_needs_command" -s c -l config -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -s m -l modules -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -s t -l template -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -s f -l format -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l theme -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l benchmark -d 'Micro-benchmark: `--benchmark` (per-module timing) or `--benchmark N` (run each module N times, report min/avg/total)' -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l preset -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l export -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -s o -l output -r -F
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l box-style -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l palette-style -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l frame -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l watch-interval -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l ssh -d 'Fetch remote system info via SSH (repeatable, parallel)' -r
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l debug
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l gen-config
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l list-modules
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l list-presets
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l pipe
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l minimal
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l full
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l dev
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l no-gradient
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l no-progress
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l pixel-logo
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l watch
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l live -d 'Live dashboard: real-time CPU/memory gauges, top processes, network rates'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l smart -d 'Smart fetch: add $PWD context (git branch/status, project type, container/venv/SSH)'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l health -d 'Add the system health module (score 0-100: disk/swap/load/battery)'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l prompt -d 'Single-line prompt string (e.g. `🐧 arch | CPU 12% | RAM 3.2G`)'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l motd -d 'Plain-text banner (ANSI colors stripped) for MOTD/startup'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -l wizard -d 'Interactive config wizard (writes ~/.config/flexfetch/config.toml)'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -s h -l help -d 'Print help'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -f -a "completions" -d 'Generate shell completions for the given shell'
complete -c flexfetch -n "__fish_flexfetch_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c flexfetch -n "__fish_flexfetch_using_subcommand completions" -s h -l help -d 'Print help'
complete -c flexfetch -n "__fish_flexfetch_using_subcommand help; and not __fish_seen_subcommand_from completions help" -f -a "completions" -d 'Generate shell completions for the given shell'
complete -c flexfetch -n "__fish_flexfetch_using_subcommand help; and not __fish_seen_subcommand_from completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
