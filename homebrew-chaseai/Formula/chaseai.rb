# Homebrew formula for ChaseAI
# This formula installs ChaseAI from a pre-built DMG

class Chaseai < Formula
  desc "Local control and orchestration system for AI agents"
  homepage "https://github.com/chaseai/chaseai"
  url "https://github.com/chaseai/chaseai/releases/download/v0.1.0/chase-0.1.0-macos.dmg"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  version "0.1.0"

  # Supported architectures
  on_macos do
    if Hardware::CPU.arm?
      # Apple Silicon
    elsif Hardware::CPU.intel?
      # Intel
    end
  end

  def install
    # Mount DMG and extract app
    dmg_mount_point = mount_dmg(cached_download)

    # Copy app bundle to Applications
    app_bundle = File.join(dmg_mount_point, "ChaseAI.app")
    if File.exist?(app_bundle)
      cp_r app_bundle, "/Applications/"
    end

    # Create symlink to binary in bin directory
    bin.install_symlink "/Applications/ChaseAI.app/Contents/MacOS/ChaseAI" => "chase"
  end

  def mount_dmg(dmg_path)
    mount_point = "/Volumes/ChaseAI"
    system "hdiutil", "attach", dmg_path, "-mountpoint", mount_point
    mount_point
  end

  def post_install
    puts "ChaseAI has been installed!"
    puts "You can now run: chase --help"
  end

  test do
    system "#{bin}/chase", "--version"
  end
end
