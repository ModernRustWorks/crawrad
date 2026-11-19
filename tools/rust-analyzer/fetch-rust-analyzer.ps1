<#
.SYNOPSIS
    Puts the rust-analyzer CrawRAD ships beside the IDE's executable.

.DESCRIPTION
    The IDE uses its own rust-analyzer, not the machine's (ADR 0050): the
    version the IDE was tested with is the one that runs, as with the
    compiler (ADR 0034) and lldb-dap (ADR 0048). The server is looked for in
    `<folder of crawrad-ide>\rust-analyzer\`; the PATH is only the last resort
    and the status bar says "(system copy)" when it is used.

    The version is pinned below with the SHA-256 of each asset, measured when
    it was pinned. A download whose hash differs is refused.

.EXAMPLE
    pwsh tools/rust-analyzer/fetch-rust-analyzer.ps1 -Out target\release\rust-analyzer
#>
param(
    # The folder that receives rust-analyzer.exe: `<IDE folder>\rust-analyzer`.
    [Parameter(Mandatory)] [string] $Out,
    # x86_64 or aarch64; defaults to this machine.
    [string] $Arch = $(if ($env:PROCESSOR_ARCHITECTURE -eq 'ARM64') { 'aarch64' } else { 'x86_64' })
)
$ErrorActionPreference = 'Stop'

# Pinned 01/10/2026: rust-analyzer 0.3.3065-standalone (03fcb77246 2026-09-27).
$Version = '2026-09-28'
$Sha256 = @{
    'x86_64'  = 'ad78fb368525404c6ac09c4bba33e90797902ce1f5a17db0925c695cae096ccc'
    'aarch64' = 'f63c7fc9a00a7e863b21b5e0b77cda7ff61aaa9f6f83b0affa5bbb525be7c43c'
}
if (-not $Sha256.ContainsKey($Arch)) { throw "No pinned rust-analyzer for '$Arch'." }

$asset = "rust-analyzer-$Arch-pc-windows-msvc.zip"
$url = "https://github.com/rust-lang/rust-analyzer/releases/download/$Version/$asset"
$tmp = Join-Path ([IO.Path]::GetTempPath()) "crawrad-$asset"
Invoke-WebRequest -Uri $url -OutFile $tmp -UseBasicParsing

$got = (Get-FileHash -Algorithm SHA256 $tmp).Hash.ToLowerInvariant()
if ($got -ne $Sha256[$Arch]) {
    Remove-Item $tmp -Force
    throw "SHA-256 mismatch for $asset`: expected $($Sha256[$Arch]), got $got."
}

New-Item -ItemType Directory -Force $Out | Out-Null
$unzip = Join-Path ([IO.Path]::GetTempPath()) "crawrad-ra-$Version"
Expand-Archive -Path $tmp -DestinationPath $unzip -Force
Copy-Item (Join-Path $unzip 'rust-analyzer.exe') (Join-Path $Out 'rust-analyzer.exe') -Force
Remove-Item $tmp, $unzip -Recurse -Force

& (Join-Path $Out 'rust-analyzer.exe') --version
