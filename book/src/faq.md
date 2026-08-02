# FAQ

**How is this different from neofetch/fastfetch?** Lua plugins, Tera templates,
and theme presets — no other tool has all three.

**How do I add info that isn't built in?** Two ways: a `[custom]` config section
(shell commands) or a Lua plugin.

**Does it work on macOS?** Yes. OS detection via `sw_vers`, macOS logo
auto-detected, and the release pipeline builds both arm64 and x86_64 binaries.

**Why is my minimal build missing templates / image logos / the live dashboard?**
Those are feature-gated for the binary diet. Build with `--features
live,image-logos,tera` to opt back in.

**Do prebuilt binaries include Lua plugins?** No — Releases/install.sh binaries
are pure Rust (no Lua) to stay lean. Source builds include Lua by default.

**What's the license?** MIT.
