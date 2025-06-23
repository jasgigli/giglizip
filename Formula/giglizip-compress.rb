class GiglizipCompress < Formula
  desc "A fast, multi-algorithm compression CLI and library"
  homepage "https://github.com/yojasgigliur/repo"
  url "https://github.com/jasgigli/giglizip/releases/download/v0.1.0/giglizip-compress-v0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "<SHA256_PLACEHOLDER>"
  version "1.0.0"

  def install
    bin.install "giglizip-setup"
  end
end
