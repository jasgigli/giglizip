$ErrorActionPreference = 'Stop'
$VERSION = (Select-String -Path Cargo.toml -Pattern '^version =').Line -replace 'version = "','' -replace '"',''
$TARGETS = @('x86_64-unknown-linux-musl', 'x86_64-pc-windows-gnu', 'x86_64-apple-darwin')
$ARTIFACTS = @()

foreach ($TARGET in $TARGETS) {
    rustup target add $TARGET
    cargo build --release --target $TARGET
    $BIN_NAME = 'giglizip-compress.exe'
    $EXT = 'zip'
    $PKG_NAME = "giglizip-compress-v$VERSION-$TARGET.$EXT"
    New-Item -ItemType Directory -Force -Path dist | Out-Null
    Copy-Item "target/$TARGET/release/giglizip-compress.exe" dist/
    Compress-Archive -Path "dist/giglizip-compress.exe" -DestinationPath "dist/$PKG_NAME"
    Remove-Item "dist/giglizip-compress.exe"
    $ARTIFACTS += "dist/$PKG_NAME"
}

Set-Location dist
Get-FileHash *.zip -Algorithm SHA256 | ForEach-Object { $_.Hash + '  ' + $_.Path } | Set-Content checksums.txt
Set-Location ..

gh release create "v$VERSION" dist/*.zip dist/checksums.txt --title "giglizip-compress v$VERSION" --notes "Automated release."
