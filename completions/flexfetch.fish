# Fish completions for flexfetch

complete -c flexfetch -s c -l config -d 'config file path' -r -F
complete -c flexfetch -s m -l modules -d 'module list, colon-separated' -r -f
complete -c flexfetch -s t -l template -d 'template file path' -r -F
complete -c flexfetch -s f -l format -d 'output format' -r -a 'text json svg html png'
complete -c flexfetch -l theme -d 'theme name' -r
complete -c flexfetch -l debug -d 'enable debug output'
complete -c flexfetch -l gen-config -d 'generate default config'
complete -c flexfetch -l list-modules -d 'list available modules'
complete -c flexfetch -l list-presets -d 'list available presets'
complete -c flexfetch -l benchmark -d 'run benchmark'
complete -c flexfetch -l pipe -d 'force pipe mode'
complete -c flexfetch -l minimal -d 'minimal module set'
complete -c flexfetch -l full -d 'all default modules'
complete -c flexfetch -l dev -d 'developer module set'
complete -c flexfetch -l preset -d 'use named preset' -r -a 'default minimal full dev server laptop'
complete -c flexfetch -l export -d 'export to file' -r -a 'svg html png'
complete -c flexfetch -s o -l output -d 'output file path' -r -F
complete -c flexfetch -l no-gradient -d 'disable gradient title'
complete -c flexfetch -l no-progress -d 'disable progress bars'
complete -c flexfetch -l box-style -d 'box rendering style' -r -a 'rounded sharp double heavy'
complete -c flexfetch -l pixel-logo -d 'enable pixel art logo'
complete -c flexfetch -l palette-style -d 'color palette style' -r -a 'gradient solid ansi'
complete -c flexfetch -l frame -d 'frame style' -r -a 'none single double rounded'
complete -c flexfetch -s V -l version -d 'show version'
complete -c flexfetch -s h -l help -d 'show help'
