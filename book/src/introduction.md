# Introduction

**flexfetch** is a fast, flexible system info tool written in Rust. It prints
your system details — OS, kernel, CPU, memory, disks, and more — with a logo and
themeable output, and it goes well beyond what neofetch/fastfetch do:

|     | Feature            | What it means                                                                                  |
| --- | ------------------ | ---------------------------------------------------------------------------------------------- |
| 🔌  | **Lua plugins**    | Write info modules in Lua. Drop a `.lua` file in `~/.config/flexfetch/plugins/` and it appears in output. |
| 📝  | **Tera templates** | Jinja2-style templates. Variables, loops, conditionals. Default template renders side-by-side logo + info. |
| 🎭  | **5+ theme presets** | Catppuccin, Dracula, Nord, Gruvbox, Tokyo Night, and more. Switch with `--theme`.               |
| ⚡  | **Rust + Rayon**   | Parallel detection. Static binary, zero runtime deps. As small as ~1.5 MB in the minimal build. |

This documentation covers installation, configuration, every module, the
template system, plugins, themes, and the full CLI reference. The source lives
at [github.com/mahesh-diwan/flexfetch](https://github.com/mahesh-diwan/flexfetch).
