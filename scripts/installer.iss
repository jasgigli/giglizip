; -- GigliZip Inno Setup Script --
#define MyAppName "GigliZip"
#define MyAppExeName "giglizip.exe"
#define MyAppPublisher "GigliZip Project"
#define MyAppVersion "1.0.0"
#define MyAppURL "https://github.com/jasgigli/giglizip"

[Setup]
AppId={{A1B2C3D4-E5F6-47A8-9B0C-1234567890AB}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
OutputDir=Output
OutputBaseFilename=giglizip-setup
Compression=lzma
SolidCompression=yes
DisableProgramGroupPage=no
PrivilegesRequired=admin

[Files]
Source: "..\target\x86_64-pc-windows-gnu\release\giglizip.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\GigliZip"; Filename: "{app}\giglizip.exe"
Name: "{group}\Uninstall GigliZip"; Filename: "{uninstallexe}"

[Run]
Filename: "{app}\giglizip.exe"; Description: "Launch GigliZip"; Flags: nowait postinstall skipifsilent
