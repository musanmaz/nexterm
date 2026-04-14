cask "nexterm" do
  version "1.3.0"
  sha256 "4f0e0c61cc56100243066fa667a27878e3c39eeba380fc74f9fb8bbe44dbc267"

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
