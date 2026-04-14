cask "nexterm" do
  version "1.2.0"
  sha256 "c7f23c0b57e4d05ef97bea7340516f8999a37ecd5af18c53974d7ba6cb37ebb7"

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
