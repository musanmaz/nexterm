cask "nexterm" do
  version "1.4.0"
  sha256 "9571ceca4e20b6f96c09f5b25d7c0c3a5d61e3b53ee04434d560a66c0692be01"

  url "https://github.com/musanmaz/nexterm/releases/download/v#{version}/NEXTERM_#{version}_aarch64.dmg"
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
