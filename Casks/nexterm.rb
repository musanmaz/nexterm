cask "nexterm" do
  version "1.6.0"
  sha256 "4fc837ae09f7877d4011186288eb8b55e3d70270d9ef6eb5c182f47e3fb58880"

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
