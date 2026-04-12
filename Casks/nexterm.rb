cask "nexterm" do
  version "1.0.0"
  sha256 "5695a673916c61ab217b3c3221137b16206428d3b1de06f2a145eef93dc03afa"

  url "https://github.com/musanmaz/nexterm/releases/download/v#{version}/NEXTERM-#{version}-arm64.dmg"
  name "NEXTERM"
  desc "AI-powered terminal emulator and DevOps command center"
  homepage "https://github.com/musanmaz/nexterm"

  app "NEXTERM.app"

  zap trash: [
    "~/Library/Preferences/com.nexterm.app.plist",
    "~/Library/Application Support/com.nexterm.app",
    "~/Library/Caches/com.nexterm.app",
    "~/Library/WebKit/com.nexterm.app",
  ]
end
