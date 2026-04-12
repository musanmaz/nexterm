class Nexterm < Formula
  desc "AI-powered terminal emulator and DevOps command center"
  homepage "https://github.com/musanmaz/nexterm"
  version "1.0.0"
  url "https://github.com/musanmaz/nexterm/releases/download/v1.0.0/nexterm-macos-arm64.tar.gz"
  sha256 "6449c7fa44652bb1269165eecca5eba993a8c969e30cb939260a6c06ad8c6e56"
  license "MIT"

  def install
    bin.install "nexterm"
  end

  test do
    assert_predicate bin/"nexterm", :exist?
  end
end
