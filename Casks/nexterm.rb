cask "nexterm" do
  version "1.1.0"
  sha256 "3d1a446dce035b7df61168ce00035a67e4a7a66fdf9e7a13461e819355b956bc"

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
