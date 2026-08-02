# Homebrew formula for flexfetch.
#
# This file ships in-repo under packaging/ so it can be published to a
# `homebrew-flexfetch` tap repo (or installed directly with:
#   brew install --formula packaging/flexfetch.rb
# or via a tap once the tap repo exists). To bump for a new release, update the
# `url` tag and `sha256` (shasum -a 256 of the GitHub source tarball).
#
# Builds the release config (--no-default-features + live,image-logos,completions)
# matching the GitHub release binaries. No assets/themes or assets/logos dirs exist
# (themes/logos are embedded consts — see ROADMAP 1.2/1.3), so only the binary,
# man page, and completions are installed.

class Flexfetch < Formula
  desc "Blazing-fast system information tool"
  homepage "https://github.com/mahesh-diwan/flexfetch"
  url "https://github.com/mahesh-diwan/flexfetch/archive/v0.18.0.tar.gz"
  sha256 "124cff5605cc69e4c7bc819b24a0b4edcd31eeb682b36735d468d0c36aa6c92b"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release", "--locked",
           "--no-default-features", "--features", "live,image-logos,completions",
           "--package", "flexfetch-cli"
    bin.install "target/release/flexfetch"
    man1.install "doc/flexfetch.1"
    bash_completion.install "completions/flexfetch.bash"
    zsh_completion.install "completions/flexfetch.zsh"
    fish_completion.install "completions/flexfetch.fish"
  end

  test do
    assert_match "flexfetch", shell_output("#{bin}/flexfetch --version")
  end
end
