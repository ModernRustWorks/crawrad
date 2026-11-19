<#
.SYNOPSIS
    Builds the debug adapter CrawRAD ships: lldb-dap with Python turned off.

.DESCRIPTION
    Every ready-made LLDB for Windows links Python into liblldb.dll and starts
    it when the debugger starts (measured on 30/09/2026: the LLVM release links
    python3.dll, llvm-mingw links libpython3.14.dll; without Python on the
    machine lldb-dap dies before the first breakpoint). The owner's rule is a
    debugger without Python (ADR 0048), so the adapter is built here from the
    LLVM sources with LLDB_ENABLE_PYTHON=OFF.

    It is built with MSVC, as the LLVM release is: the llvm-mingw LLDB misreads
    Rust enums in DWARF (a `-gnu` build showed `Circle { radius: 1.97E-323 }`
    where the LLVM release shows `2.0`).

    The CRT is linked statically, so the result needs nothing but Windows.
    Output: lldb-dap.exe, liblldb.dll and lldb-server.exe — the three files
    measured as the minimum (without lldb-server `configurationDone` fails).

.EXAMPLE
    pwsh tools/lldb/build-lldb.ps1 -Source C:\lldb-build\llvm-project-23.1.2.src -Out C:\lldb-build\out
#>
param(
    # The extracted llvm-project source tree (llvm-project-<version>.src).
    [Parameter(Mandatory)] [string] $Source,
    # Where the three files are copied.
    [Parameter(Mandatory)] [string] $Out,
    # The CMake build directory; kept between runs so a rebuild is incremental.
    [string] $Build = (Join-Path (Split-Path $Source -Parent) 'build')
)
$ErrorActionPreference = 'Stop'

$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$vs = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $vs) { throw 'Visual Studio with the C++ tools (x64) is required.' }
$vcvars = Join-Path $vs 'VC\Auxiliary\Build\vcvars64.bat'
$cmake = Join-Path $vs 'Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe'
$ninja = Join-Path $vs 'Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe'

$options = @(
    '-G', 'Ninja',
    "-DCMAKE_MAKE_PROGRAM=$ninja",
    '-S', (Join-Path $Source 'llvm'),
    '-B', $Build,
    '-DCMAKE_BUILD_TYPE=Release',
    # LLDB needs Clang (its expression evaluator and type system).
    '-DLLVM_ENABLE_PROJECTS=clang;lldb',
    '-DLLVM_TARGETS_TO_BUILD=X86;AArch64',
    # The point of this build.
    '-DLLDB_ENABLE_PYTHON=OFF',
    '-DLLDB_ENABLE_LUA=OFF',
    '-DLLDB_ENABLE_LIBEDIT=OFF',
    '-DLLDB_ENABLE_CURSES=OFF',
    '-DLLDB_ENABLE_LIBXML2=OFF',
    '-DLLDB_ENABLE_LZMA=OFF',
    # PDBs are read by LLDB's native reader: no DIA SDK, no msdia140.dll to ship.
    '-DLLVM_ENABLE_DIA_SDK=OFF',
    '-DLLVM_ENABLE_ZLIB=OFF',
    '-DLLVM_ENABLE_ZSTD=OFF',
    '-DLLVM_ENABLE_LIBXML2=OFF',
    # Static CRT: nothing to install beside the three files.
    '-DCMAKE_MSVC_RUNTIME_LIBRARY=MultiThreaded',
    '-DLLVM_ENABLE_ASSERTIONS=OFF',
    '-DLLVM_INCLUDE_TESTS=OFF',
    '-DLLVM_INCLUDE_BENCHMARKS=OFF',
    '-DLLVM_INCLUDE_EXAMPLES=OFF',
    '-DLLVM_INCLUDE_DOCS=OFF',
    '-DCLANG_INCLUDE_TESTS=OFF',
    '-DCLANG_INCLUDE_DOCS=OFF',
    '-DLLDB_INCLUDE_TESTS=OFF'
)

function Invoke-InVcvars([string] $command) {
    cmd /c "`"$vcvars`" >nul && $command"
    if ($LASTEXITCODE -ne 0) { throw "failed: $command" }
}

$quoted = ($options | ForEach-Object { "`"$_`"" }) -join ' '
Invoke-InVcvars "`"$cmake`" $quoted"
Invoke-InVcvars "`"$ninja`" -C `"$Build`" lldb-dap lldb-server liblldb"

New-Item -ItemType Directory -Force $Out | Out-Null
foreach ($file in 'lldb-dap.exe', 'liblldb.dll', 'lldb-server.exe') {
    Copy-Item (Join-Path $Build "bin\$file") $Out -Force
}
Get-ChildItem $Out | Format-Table Name, Length
