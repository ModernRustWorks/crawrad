; CrawRAD — Windows installer (Inno Setup 6).
; Build the bundle first:   python tools/bundle/bundle.py
; Then:                     iscc /DAppVersion=0.1.0 installer\windows\crawrad.iss
; The bundle folder can be overridden with /DBundleDir=...

#ifndef AppVersion
  #define AppVersion "0.0.0-dev"
#endif
#ifndef BundleDir
  #define BundleDir "..\..\target\bundle\CrawRAD"
#endif

[Setup]
AppId={{C45BFA2D-9D7D-48D6-B7F2-48E0F9855B7A}
AppName=CrawRAD
AppVersion={#AppVersion}
AppPublisher=CrawRAD
DefaultDirName={autopf}\CrawRAD
DefaultGroupName=CrawRAD
DisableProgramGroupPage=yes
LicenseFile={#BundleDir}\LICENSE
OutputDir=..\..\target\installer
OutputBaseFilename=CrawRAD-{#AppVersion}-windows-x64-setup
SetupIconFile=..\..\crates\crawrad-ide\crawrad.ico
UninstallDisplayIcon={app}\crawrad-ide.exe
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
Compression=lzma2/max
SolidCompression=yes
WizardStyle=modern
PrivilegesRequiredOverridesAllowed=dialog

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "{#BundleDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\CrawRAD"; Filename: "{app}\crawrad-ide.exe"; WorkingDir: "{app}"
Name: "{group}\{cm:UninstallProgram,CrawRAD}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\CrawRAD"; Filename: "{app}\crawrad-ide.exe"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\crawrad-ide.exe"; Description: "{cm:LaunchProgram,CrawRAD}"; Flags: nowait postinstall skipifsilent
