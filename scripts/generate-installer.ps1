# PowerShell script to generate installer.iss with version from Cargo.toml
$version = (Select-String -Path "..\Cargo.toml" -Pattern '^version\s*=\s*"(.*)"').Matches[0].Groups[1].Value

@"
[Setup]
AppName=GigliZip Compress
AppVersion=$version
DefaultDirName={pf}\GigliZip
DefaultGroupName=GigliZip
OutputBaseFilename=giglizip-setup
Compression=lzma
SolidCompression=yes

[Files]
Source: "..\target\x86_64-pc-windows-gnu\release\giglizip-compress.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\GigliZip Compress"; Filename: "{app}\giglizip-compress.exe"

[Run]
Filename: "{app}\giglizip-compress.exe"; Description: "Launch GigliZip Compress"; Flags: nowait postinstall skipifsilent
"@ | Set-Content -Path "installer.iss"
