cask "nexterm" do
  version "1.5.0"
  sha256 "8160dd55d2b6b7822dd9a9e0a74ea905d2a16cec7ddb9969c91f34893530a10a"

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
