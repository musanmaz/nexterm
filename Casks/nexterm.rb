cask "nexterm" do
  version "1.1.0"
  sha256 "cac01e2167ae03ffe3c111a1618faa10fcfc9ad4aab643628cfd66593e5fa5a1"

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
