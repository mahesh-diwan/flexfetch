{
  description = "flexfetch — blazing-fast system information tool";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;

      # Version is single-sourced from the workspace Cargo.toml so a version bump
      # can never drift from the flake.
      version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;

      # Build a flexfetch package. `features` maps directly to the real cargo
      # features of the flexfetch-cli workspace member (there is no `all`/`minimal`
      # feature — the plan's names were adapted):
      #   - default:  release config (--no-default-features + live,image-logos,completions)
      #               = the shipped binary, pure Rust, ~2 MB
      #   - full:     all default features (incl. vendored Lua, Tera, Rayon) ~6 MB
      #   - minimal:  --no-default-features ~1.5 MB
      # No C deps are needed for default/minimal (mlua/zstd-sys are gated behind
      # `lua`/`qr`), so no pkg-config/openssl/zstd buildInputs like the original
      # plan suggested.
      mkFlexfetch = { pkgs, rustPlatform, features ? [ "live" "image-logos" "completions" ], noDefault ? true }:
        rustPlatform.buildRustPackage {
          pname = "flexfetch";
          inherit version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # The workspace root has no [package]; always build just the CLI member.
          # buildNoDefaultFeatures + buildFeatures are applied to flexfetch-cli,
          # and its `image-logos`/`live` features forward the matching
          # flexfetch-core feature flags automatically.
          cargoBuildFlags = [ "--package" "flexfetch-cli" ];
          buildNoDefaultFeatures = noDefault;
          buildFeatures = features;

          # Tests need live /proc + /sys; run the workspace suite in CI instead.
          doCheck = false;

          # Explicit install: copy the built binary (cargo install --path . fails
          # on a workspace root with no [package]).
          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin
            install -Dm755 target/release/flexfetch $out/bin/flexfetch
            runHook postInstall
          '';

          meta = {
            description = "Blazing-fast, deeply introspective system information tool";
            homepage = "https://github.com/mahesh-diwan/flexfetch";
            license = nixpkgs.lib.licenses.mit;
            mainProgram = "flexfetch";
          };
        };
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = mkFlexfetch { inherit pkgs; rustPlatform = pkgs.rustPlatform; };
          full = mkFlexfetch {
            inherit pkgs;
            rustPlatform = pkgs.rustPlatform;
            features = [ ];
            noDefault = false;
          };
          minimal = mkFlexfetch {
            inherit pkgs;
            rustPlatform = pkgs.rustPlatform;
            features = [ ];
          };
        });

      # A single, system-agnostic Home Manager module (the standard
      # `homeManagerModules.<name>` shape). The package is resolved from HM's own
      # `pkgs.system` *inside* the module body, so importing the module on any
      # system always gets that system's flexfetch derivation — never a
      # flake-evaluation-time system baked in.
      homeManagerModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.programs.flexfetch;
        in
        {
          options.programs.flexfetch = {
            enable = lib.mkEnableOption "flexfetch system information tool";
            package = lib.mkOption {
              type = lib.types.package;
              default = self.packages.${pkgs.system}.default;
              description = "The flexfetch package to install.";
            };
            settings = lib.mkOption {
              type = lib.types.attrs;
              default = { };
              example = lib.literalExpression ''
                {
                  modules = [ "title" "separator" "os" "host" "kernel" "uptime" "cpu" "memory" ];
                  display = {
                    theme = "catppuccin-mocha";
                    key_width = 8;
                    separator = ": ";
                  };
                }
              '';
              description = "TOML settings written to ~/.config/flexfetch/config.toml.";
            };
          };
          config = lib.mkIf cfg.enable {
            home.packages = [ cfg.package ];
            xdg.configFile."flexfetch/config.toml".source =
              pkgs.writeText "flexfetch-config.toml" (lib.generators.toTOML { } cfg.settings);
          };
        };
    };
}
