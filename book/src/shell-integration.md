# Shell Integration

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

## Man page

```bash
man doc/flexfetch.1
```
