class Ttype < Formula
  desc "Minimalist terminal typing speed test in Rust with conveyor tape and mechanical audio"
  homepage "https://github.com/laguser/ttype"
  url "https://github.com/laguser/ttype/archive/refs/tags/v0.1.3.tar.gz"
  sha256 "PLACEHOLDER_SHA256"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "ttype", shell_output("#{bin}/ttype --version")
  end
end
