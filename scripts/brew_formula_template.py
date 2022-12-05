formula_template = """# Spake-cli homebrew formula for releasing brew formula via github.

class Spake < Formula
  desc "Spake CLI tool for automatic machine translation."
  homepage "https://spake.uncommon.industries"
  version "{version}"
  depends_on :macos
  
  license "Apache-2.0"
  
  on_macos do
    if Hardware::CPU.arm?
      url "{url_arm64}"
      sha256 "{sha_arm64}"

      def install
        bin.install "spake-cli"
      end
    end
    if Hardware::CPU.intel?
      url "{url_x86_64}"
      sha256 "{sha_x86_64}"
      def install
        bin.install "spake-cli"
      end
    end
  end
end
"""
