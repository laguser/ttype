class Ttype < Formula
  desc "Minimalist terminal typing speed test in Rust with conveyor tape and mechanical audio"
  homepage "https://github.com/laguser/ttype"
  url "https://github.com/laguser/ttype/archive/refs/tags/v0.1.3.tar.gz"
  sha256 "f50dd1f538262f58d2510a58e06fc3a051a6c35f94a1348f616c230e7eec81a5"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "ttype", shell_output("#{bin}/ttype --version")
  end
end
