#compdef flexfetch

_flexfetch() {
    _arguments \
        '(-c --config)'{-c,--config}'[config file path]:file:_files' \
        '(-m --modules)'{-m,--modules}'[module list, colon-separated]:modules:_flexfetch_modules' \
        '(-t --template)'{-t,--template}'[template file path]:file:_files' \
        '(-f --format)'{-f,--format}'[output format]:format:(text json svg html png)' \
        '--theme[theme name]:theme:' \
        '--debug[enable debug output]' \
        '--gen-config[generate default config]' \
        '--list-modules[list available modules]' \
        '--list-presets[list available presets]' \
        '--benchmark[run benchmark]' \
        '--pipe[force pipe mode]' \
        '--minimal[minimal module set]' \
        '--full[all default modules]' \
        '--dev[developer module set]' \
        '--preset[use named preset]:preset:_flexfetch_presets' \
        '--export[export to file]:format:(svg html png)' \
        '(-o --output)'{-o,--output}'[output file path]:file:_files' \
        '--no-gradient[disable gradient title]' \
        '--no-progress[disable progress bars]' \
        '--box-style[box rendering style]:style:(rounded sharp double heavy)' \
        '--pixel-logo[enable pixel art logo]' \
        '--palette-style[color palette style]:style:(gradient solid ansi)' \
        '--frame[frame style]:style:(none single double rounded)' \
        '(-V --version)'{-V,--version}'[show version]' \
        '(-h --help)'{-h,--help}'[show help]' && ret=0
}

_flexfetch_modules() {
    local modules=(
        os host kernel uptime locale cpu memory disk gpu network
        battery processes packages shell terminal de wm colors custom
    )
    _values -s ':' modules[@]
}

_flexfetch_presets() {
    local presets=(
        default minimal full dev server laptop
    )
    _describe 'preset' presets
}

_flexfetch "$@"
