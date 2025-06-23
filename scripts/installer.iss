[Setup]
AppName=GigliZip Compress
AppVersion=0.1.0
DefaultDirName={pf}\GigliZip
DefaultGroupName=GigliZip
OutputBaseFilename=giglizip-setup
Compression=lzma
SolidCompression=yes

[Files]
Source: "..\target\x86_64-pc-windows-gnu\release\giglizip.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\GigliZip Compress"; Filename: "{app}\giglizip.exe"

[Run]
Filename: "{app}\giglizip.exe"; Description: "Launch GigliZip Compress"; Flags: nowait postinstall skipifsilent
