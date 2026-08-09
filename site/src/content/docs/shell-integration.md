---
title: Shell Integration
description: Prompt, shell hooks, MOTD, completions, and man page
order: 11
---

## Prompt

`--prompt` prints a single ANSI-free line for shell prompts:

```bash
flexfetch --prompt            # cachyos | CPU 12% | RAM 3.2 GiB/15.3 GiB
```

### Bash

```bash
PS1="$(flexfetch --prompt) $ "
```

### Zsh

```bash
PROMPT="$(flexfetch --prompt) % "
```

### Fish

```bash
function fish_prompt
    flexfetch --prompt
    echo -n " "
end
```

## Shell hooks

Print a shell hook for cd-into-git-repo context fetches:

```bash
eval "$(flexfetch --hook bash)"   # bash
eval "$(flexfetch --hook zsh)"    # zsh
eval "$(flexfetch --hook fish)"   # fish
```

## MOTD

`--motd` renders the normal output with all ANSI colors stripped — drop it in
`/etc/motd` or your shell startup:

```bash
flexfetch --motd > /etc/motd
```

## Completions

Tab completion for bash, zsh, and fish. Generate fresh copies from the binary
(the `completions` subcommand is available in default builds):

```bash
flexfetch completions bash > completions/flexfetch.bash
flexfetch completions zsh  > completions/flexfetch.zsh
flexfetch completions fish > completions/flexfetch.fish
```

Or use the pre-generated files in the repo `completions/` directory:

```bash
# Bash
source completions/flexfetch.bash

# Zsh
source completions/flexfetch.zsh

# Fish
source completions/flexfetch.fish
```

Install permanently:

```bash
# Bash (Ubuntu/Debian)
cp completions/flexfetch.bash /etc/bash_completion.d/

# Zsh
cp completions/flexfetch.zsh /usr/share/zsh/vendor-completions/

# Fish
cp completions/flexfetch.fish ~/.config/fish/completions/
```

## Putting it together

Here's a complete bash config that uses flexfetch as a prompt and MOTD:

```bash
# ~/.bashrc

# Shell prompt with system info
PS1="\[\e[36m\]$(flexfetch --prompt)\[\e[0m\] \$ "

# MOTD on login (optional — generates once, not on every shell)
if [ ! -f ~/.flexfetch-motd ] || [ "$(find ~/.flexfetch-motd -mmin +60 2>/dev/null)" ]; then
    flexfetch --motd > ~/.flexfetch-motd
fi
cat ~/.flexfetch-motd

# Source completions
source /etc/bash_completion.d/flexfetch 2>/dev/null
```

For zsh:

```bash
# ~/.zshrc

# Prompt
PROMPT="%F{cyan}$(flexfetch --prompt)%f % "

# Completions
source /usr/share/zsh/vendor-completions/_flexfetch 2>/dev/null
```

For fish:

```bash
# ~/.config/fish/config.fish

# Prompt
function fish_prompt
    flexfetch --prompt
    echo -n " > "
end

# Completions (auto-loaded from completions/ dir)
```

## Prompt output

`flexfetch --prompt` returns a compact one-liner like:

```
cachyos | CPU 12% | RAM 3.2 GiB/15.3 GiB | Disk 89% | Uptime 4h 12m
```

It's ANSI-free by default so it works in any prompt. If you want colors in
the prompt, wrap it in your shell's color escape sequences (see the examples
above).

## MOTD

`--motd` renders the normal output with all ANSI colors stripped — drop it in
`/etc/motd` or your shell startup:

```bash
flexfetch --motd > /etc/motd
```

The generated file is plain text, so it works in terminals that don't support
truecolor or in contexts where colors are stripped (SSH banners, serial
consoles).
