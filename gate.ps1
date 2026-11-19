<#
.SYNOPSIS
    CrawRAD's local gate. One command, and it states what "green" means.

.DESCRIPTION
    Until now the gate of this project was RITUAL: a list of commands written
    in an ADR and in a briefing, retyped by whoever remembered it. Prose does not run,
    does not abort on the first error, and leaves no output for anyone to paste into a
    report. The executable copy of that list was GitHub CI — and it stopped
    ("The job was not started because recent account payments have failed or
    your spending limit needs to be increased": zero steps, zero log).

    This file becomes the single source of truth. `.github/workflows/ci.yml`
    CALLS it instead of repeating the commands: one single truth, versioned alongside
    the code it judges. The two earlier copies had already diverged — the
    yml gained `--include-ignored` and the local gate stayed in the head of whoever
    typed it.

    CI returned after that, on a SELF-HOSTED runner on this machine, which does not
    consume minutes. It returned as ONE single job, and that job is one line: calling this
    file. The `ubuntu-latest` and `macos-latest` jobs did not return, and what
    they proved changed ownership — compilation for the two
    `cargo clippy --target` here, behavior for the Docker step.
    `ci.yml` tells that story in full.

    The steps go from cheap to expensive, and the first one to fail ABORTS: knowing
    about tests is pointless when clippy has already failed. The exception is the
    Linux in Docker step, and the reason why is written in its section below.

.PARAMETER Fast
    Skips the expensive steps: the two `#[ignore]` gates (~2.5 min), the two
    `cargo clippy --target` (~1 min cold, seconds warm), and the Linux suite
    inside Docker (8 s warm, 80 s cold). This is the mode for iterating. The
    mode appears on the first line, in the summary, and on the last line — a
    half-gate announcing itself as a gate is worse than no gate at all.

    And the last line says more than the mode: when the Linux step does not run —
    due to `-Fast`, `-NoDocker`, or because the daemon is down —, it outputs
    `WITHOUT LINUX PROOF`. Green from a run that proved Linux and green from a
    run that did not cannot be the same pasteable text.

.PARAMETER NoDocker
    Does not run the Linux suite inside Docker. Exists for whoever knows they will
    pay for cold compilation (changed `Cargo.lock`, deleted volume) and does not
    want to pay for it now. Like `-NoDeny`, the summary notes that you used it — and
    what gets left out is the only BEHAVIORAL proof outside Windows that this
    gate has.

    Do not confuse with Docker being absent: if the daemon is down, the step
    SKIPS itself, stating the reason in the summary, and the gate continues. A gate that
    requires a daemon to give green turns into a gate that nobody runs.

.PARAMETER NoDeny
    Does not run `cargo deny check licenses`. Born for CI, when a dedicated Linux job
    ran the entire `cargo-deny` with the Embark action —
    and installing the binary on the Windows runner just to repeat half the
    question would cost compilation minutes.

    That job no longer exists (07/09/2026: GitHub runners stopped due to an
    unpaid bill, and CI became a single job on the self-hosted runner of this
    machine, which HAS `cargo-deny`). CI today calls this gate WITHOUT
    `-NoDeny`. The parameter remains for a machine that does not have the binary and still
    wants to run the rest — and, if you use it, the summary says you used it.

.EXAMPLE
    .\gate.ps1 -Fast
    While working.

.EXAMPLE
    .\gate.ps1
    Before submitting. This is the one that counts as "green".

.NOTES
    Only this version in PowerShell exists because the primary environment is Windows (ADR 0001) and
    because two copies of the same list is exactly the defect this file
    came to eliminate. If someone ever works on Linux, `pwsh` runs there; if
    they still want a `gate.sh`, let it CALL this one, rather than rewrite the
    steps.

    Output is in English so that it can be used in CI, PRs, and reports across the project.
#>
[CmdletBinding()]
param(
    [Alias('Rapido')][switch]$Fast,
    [Alias('SemDocker')][switch]$NoDocker,
    [Alias('SemDeny')][switch]$NoDeny
)

$ErrorActionPreference = 'Stop'

# PowerShell 7.3+ can convert external command exit codes into exceptions.
# NOT here: the decision of what to do with cargo's exit code belongs to
# `Invoke-Step`, which needs to record the elapsed time and print the summary BEFORE
# exiting. Without this, the gate could abort without reporting which step failed.
if (Test-Path variable:PSNativeCommandUseErrorActionPreference) {
    $PSNativeCommandUseErrorActionPreference = $false
}

# Run from any directory: the gate belongs to the repository, not to the location
# from which you invoked it.
Set-Location -LiteralPath $PSScriptRoot

$TARGETS = @('x86_64-unknown-linux-gnu', 'aarch64-apple-darwin')

# ---------------------------------- the Linux that RUNS, not just compiles ---
#
# The two `cargo clippy --target` in this gate prove that the code COMPILES outside
# Windows, and without warnings. They do not prove ONE line of behavior — and the gap
# between the two is not theoretical: the entire suite passes on Windows and fails
# on Linux in ELEVEN tests, all due to platform assumptions written inside the
# tests (backslash in path, Windows reserved name, `rename` over read-only
# file, font glyph). No compilation check would find a single one.
#
# The machine for this project is Windows (ADR 0001) and will remain so. Real
# Linux enters via CONTAINER: same kernel, same `/`, same `PathBuf`.
#
# MEASURED on this machine, on 07/09/2026, with these two volumes:
#
#     cold (BOTH volumes deleted: downloads 348 crates and compiles everything) .. 80 s
#     warm (second run, nothing changed) ........................................ 8 s
#
# Eight seconds warm, compared to under 1 s for each warm `cargo clippy --target`
# (0.77 s and 0.69 s, measured on this machine on 07/09/2026): the Linux step is
# an order of magnitude more expensive than the two COMBINED, yet still fits in
# a gate and answers a question that they do not. The cold number for the
# per-target steps is deliberately omitted here — the method that produced it did not
# isolate what it promised, and the reason why is documented alongside the step below.
#
# The two named volumes are what allows the step to fit in a gate. Without them,
# EVERY run recompiles 348 crates from scratch, because `--rm` throws `/tgt`
# away. And they cannot be Windows's `target/`: the same directory with
# two OSes inside invalidates the cache for both on each switch. Hence the
# separate `CARGO_TARGET_DIR` — without it, the step pollutes the caller's `target/`.
#
# The mount point is deliberately FIXED (`/w`): the path inside the
# container is part of cargo's fingerprint.
#
# THE TARGET VOLUME IS ONE PER FOLDER (#40), not one per machine. It used to be shared,
# so this folder, an agent's worktree, and the runner's `_work` would share
# the same cache, and that sharing LIED: with the gate running in two folders at
# the same time, CI evaluated the same merge twice and disagreed with itself
# (8 failures that did not exist), and subsequently a binary from another tree
# was treated as fresh (cargo judges `/w` by file timestamps, and the trees
# have different timestamps). Now the name includes the hash of the folder path:
# repeating in the same place stays warm, and two folders never write to the same
# `/tgt`. The cost is the first cold run for each folder (80 s, above).
#
# The volume is created with the label `crawrad.linux-target=<folder>`, and before each
# run the gate removes volumes whose folders no longer exist — a worktree deleted
# after merge does not leave its target volume occupying disk.
#
# The REGISTRY remains shared: it stores downloaded crate sources, which do not
# depend on the tree.
#
# The image is PINNED to a version: the container does not follow the caller's `rustup`.
# When the project updates its toolchain, this line updates alongside it — it is the single place.
$LINUX_IMAGE = 'rust:1.95-slim'
$LINUX_MOUNT = '/w'
$LINUX_TARGET = '/tgt'
$LINUX_TARGET_LABEL = 'crawrad.linux-target'
$LINUX_TARGET_VOLUME = 'crawrad-linux-target-' + (
    [System.BitConverter]::ToString(
        [System.Security.Cryptography.SHA256]::HashData(
            [System.Text.Encoding]::UTF8.GetBytes($PSScriptRoot.ToLowerInvariant())
        )
    ) -replace '-', ''
).Substring(0, 12).ToLowerInvariant()
$LINUX_REGISTRY_VOLUME = 'crawrad-linux-registry'

# WHAT RUNS INSIDE: the entire suite, `--workspace`, with no exclusions.
#
# The question "only crates that do not depend on windows?" was MEASURED, and the
# answer is that it does not need to be asked: `crawrad-ide` compiles and runs
# entirely on `rust:1.95-slim`, without GTK, display, or X11. `eframe` only
# opens a window in `run_native`, which no test calls, and the dialog backend
# chosen in `crates/crawrad-ide/Cargo.toml` is `xdg-portal`, which speaks D-Bus
# at runtime and requires no libraries to compile — the same calculation that
# ADR 0007 made to reject `gtk3` pays off here. Measured: 245 of the 253 tests in
# `crawrad-ide` pass in the container. Excluding the crate would have thrown away 245
# proofs to avoid looking at 8.
#
# What does NOT run: `--include-ignored`. The two `#[ignore]` gates call
# `cargo add` from inside the test, which requires container networking and a second
# compilation of all of `eframe` — and what they evaluate is the GENERATED PROJECT,
# which is a Windows product and already has its own dedicated step here on Windows.
#
# `--no-fail-fast` is mandatory and not a convenience: without it, cargo stops at
# the first failing binary, and the set comparison below would treat
# binaries that never even ran as "fixed".

# THE EXPECTED RED, written down and verified by machine.
#
# This step was born red, and deliberately so: the eleven failures
# below are REAL DEBT, measured on 07/09/2026, which no gate workaround
# erases. They are not hidden behind `--skip`; they run,
# fail, appear completely in the output, and the gate checks them ONE BY ONE against
# this list:
#
#   - failure not listed here -> RED GATE. A new regression on Linux, and
#                                exactly why the step exists.
#   - failure here that vanished -> RED GATE. Someone fixed (or deleted)
#                                the test and did not remove the line; an exception that
#                                outlives the defect becomes noise within a week.
#   - exact set match         -> step proceeds, marked "EXPECTED RED", and the
#                                last line of the gate states the count aloud.
#                                Not disguised green.
#
# The name is qualified by the test BINARY (`crawrad_actions::…`) because test names
# repeat across crates and matching is by exact equality.
#
# FIXING THIS IS NOT THE TASK OF THIS FILE. Each line belongs to another crate and
# another owner; what this gate was supposed to do was FIND them, and it did.
$LINUX_EXPECTED_RED = @(
    # EMPTY since 02/10/2026: the full gate measured 0 failures on Linux. The
    # eleven from 07/09 were paid off — the path guard rejecting drive letters
    # on all OSes and portable tab/diagnostic tests (#307), with the rest moving
    # crates with their fix. Any new failure turns the gate red per the rule above,
    # and enters here only with documented justification.
)

$mode = 'FULL'
if ($Fast) { $mode = 'FAST' }

$results = New-Object System.Collections.ArrayList

# How many EXPECTED failures the Linux step confirmed. Printed on the last line so
# that nobody cites "GREEN GATE" without the rest of the sentence.
$expectedRed = 0

# And the other side of the SAME honesty: why the Linux step did NOT run.
# Empty means it ran.
#
# The defect this variable eliminates was measured by taking down the Docker daemon:
# the step skipped, printed the reason in the middle of output — and the LAST LINE
# reverted to `GREEN GATE — FULL mode`, word for word identical to a run
# that proved Linux. The last line is what gets pasted in reports and what CI
# displays; two runs that proved different things cannot end with the
# same text. `-Fast` already announced itself via `$mode`; the Docker skip
# announced itself nowhere, which is lying by omission at the exact point where
# this file claims not to lie.
$noLinuxProof = ''

function Write-Title {
    param([string]$Text)
    Write-Host ''
    Write-Host "=== $Text" -ForegroundColor Cyan
}

# THE TIME OF EACH STEP IS NOT DECORATION. Measured: two agents saw the same
# `clippy` "pass" in 0.54 s and in 3.3 s without compiling a line — it was the cache
# returning the previous result. Without elapsed time in the output, nobody
# suspects a step that did not actually run.
function Write-Summary {
    Write-Title "SUMMARY — $mode mode"
    $text = $results | Format-Table -AutoSize | Out-String
    Write-Host $text
}

# Missing tool is not a red gate: it is a gate that could not run. Exits
# with code 2 and with the exact remediation line, so the next person does not have
# to decipher a cargo error.
function Stop-Missing {
    param([string]$What, [string]$Fix)
    Write-Host ''
    Write-Host "PRE-FLIGHT: $What" -ForegroundColor Yellow
    Write-Host "  fix with:  $Fix" -ForegroundColor Yellow
    Write-Host ''
    Write-Host 'The gate did NOT run. This is neither green nor red.' -ForegroundColor Yellow
    Write-Host ''
    exit 2
}

function Record-Step {
    param([string]$Name, [string]$State, [double]$Seconds)
    $row = New-Object psobject
    $row | Add-Member -MemberType NoteProperty -Name 'Step' -Value $Name
    $row | Add-Member -MemberType NoteProperty -Name 'State' -Value $State
    $row | Add-Member -MemberType NoteProperty -Name 'Seconds' -Value $Seconds
    [void]$results.Add($row)
}

function Invoke-Step {
    param([string]$Name, [string[]]$Command)

    Write-Title $Name
    $cmdLine = $Command -join ' '
    Write-Host "    $cmdLine" -ForegroundColor DarkGray

    $exe = $Command[0]
    $cmdArgs = @($Command | Select-Object -Skip 1)

    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    & $exe $cmdArgs
    $code = $LASTEXITCODE
    $stopwatch.Stop()

    $seconds = [math]::Round($stopwatch.Elapsed.TotalSeconds, 2)
    if ($code -eq 0) {
        Record-Step $Name 'ok' $seconds
        return
    }

    Record-Step $Name 'FAILED' $seconds
    Write-Summary
    Write-Host "RED GATE — failed at: $Name (exit code $code)" -ForegroundColor Red
    Write-Host "    $cmdLine" -ForegroundColor Red
    Write-Host ''
    exit 1
}

function Mark-Skipped {
    param([string]$Name, [string]$Reason)
    Record-Step $Name "skipped ($Reason)" 0
}

# build.rs of makepad-platform v1.0.0 (crawrad-ide/makepad layer) unconditionally calls
# `clang` and `ar` when target_os == "macos" to compile metal_xpc.m into libmetal_xpc.a.
#
# In a Windows environment where we check the Darwin target (clippy without linking, ADR 0007 / ADR 0034),
# there is no macOS SDK or Apple clang/ar. We ensure a minimal shim in %TEMP% so that
# makepad's build script generates the required empty static archive/object and cargo clippy
# proceeds normally without a missing tool error.
function Ensure-DarwinCrossShim {
    $shimDir = Join-Path $env:TEMP 'crawrad-darwin-shim'
    $clangExe = Join-Path $shimDir 'clang.exe'
    $arExe = Join-Path $shimDir 'ar.exe'
    if (-not (Test-Path $clangExe) -or -not (Test-Path $arExe)) {
        New-Item -ItemType Directory -Path $shimDir -Force | Out-Null
        $stubSource = 'fn main() { let args: Vec<String> = std::env::args().collect(); for i in 0..args.len() { if args[i] == "-o" && i + 1 < args.len() { let _ = std::fs::write(&args[i+1], b""); } } if args.len() >= 3 && args[1] == "crus" { let _ = std::fs::write(&args[2], b""); } }'
        $stubRs = Join-Path $shimDir 'stub.rs'
        Set-Content -Path $stubRs -Value $stubSource -Encoding utf8
        & rustc $stubRs -o $clangExe 2>&1 | Out-Null
        Copy-Item $clangExe $arExe -Force
    }
    if ($env:PATH -split ';' -notcontains $shimDir) {
        $env:PATH = "$shimDir;$env:PATH"
    }
}

function Ensure-LinuxCrossShim {
    $shimDir = Join-Path $env:TEMP 'crawrad-linux-shim'
    $pkgExe = Join-Path $shimDir 'pkg-config.exe'
    if (-not (Test-Path $pkgExe)) {
        New-Item -ItemType Directory -Path $shimDir -Force | Out-Null
        $stubSource = 'fn main() { let args: Vec<String> = std::env::args().collect(); if args.iter().any(|a| a == "--libs" || a == "--cflags") { println!("-lasound"); } else if args.iter().any(|a| a == "--modversion") { println!("1.2.9"); } }'
        $stubRs = Join-Path $shimDir 'stub.rs'
        Set-Content -Path $stubRs -Value $stubSource -Encoding utf8
        & rustc $stubRs -o $pkgExe 2>&1 | Out-Null
    }
    if ($env:PATH -split ';' -notcontains $shimDir) {
        $env:PATH = "$shimDir;$env:PATH"
    }
    $env:PKG_CONFIG_ALLOW_CROSS = '1'
}

# --------------------------------------------- the Linux in Docker step ---

# Why the step SKIPS instead of failing: Docker is infrastructure, not our
# code. A gate that turns red because Docker Desktop is closed teaches,
# within a week, that red means nothing.
#
# `docker info` and not `docker --version`: the latter only checks the binary and
# returns identical output with the daemon stopped. It is the difference between
# "installed" and "running", and only the second suffices.
function Get-NoDockerReason {
    if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
        return '`docker` is not in PATH'
    }

    # THE `2>&1` CAGE, and it is not paranoia: with
    # `$ErrorActionPreference = 'Stop'`, PowerShell 5.1 converts the FIRST
    # line of stderr from a redirected native command into a TERMINATING
    # `NativeCommandError`. The two checks below reply via stderr when the
    # answer is "no", which is exactly the case they exist to detect.
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        # `docker info` talks to the DAEMON.
        $null = & docker info --format '{{.ServerVersion}}' 2>&1
        $daemon = $LASTEXITCODE

        # Without the image, `docker run` would reach out to the internet to fetch 325 MB in
        # the middle of the gate — and fail due to NETWORK, not code, on an offline machine.
        # Better to skip indicating the exact remediation line.
        $null = & docker image inspect $LINUX_IMAGE 2>&1
        $image = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $prev
    }

    if ($daemon -ne 0) {
        return 'Docker is not running'
    }
    if ($image -ne 0) {
        return "missing image $LINUX_IMAGE (run: docker pull $LINUX_IMAGE)"
    }

    return $null
}

# The name of each test that FAILED, qualified by the binary that ran it.
#
# Parsed from `---- <name> stdout ----` which libtest prints once per failure, and
# `Running … (/tgt/debug/deps/<binary>-<hash>)` which cargo prints before
# each binary. Reading names from there rather than the `failures:` block at the end
# is deliberate: that block appears twice per binary and double-counts.
function Get-Failures {
    param([string[]]$Output)

    $binary = '?'
    $failures = New-Object System.Collections.Generic.List[string]

    foreach ($line in $Output) {
        if ($line -match '^\s+Running .*[\\/]([A-Za-z0-9_]+)-[0-9a-f]+\)\s*$') {
            $binary = $matches[1]
            continue
        }
        if ($line -match '^\s+Doc-tests ([A-Za-z0-9_]+)\s*$') {
            $binary = "$($matches[1])(doc)"
            continue
        }
        if ($line -match '^---- (.+) stdout ----\s*$') {
            [void]$failures.Add("${binary}::$($matches[1])")
        }
    }

    return $failures.ToArray()
}

# #40: the target volumes of folders that are gone (a worktree removed after
# its merge). A volume another run is using refuses the removal, and stays.
function Remove-OrphanLinuxTargets {
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $volumes = & docker volume ls --quiet --filter "label=$LINUX_TARGET_LABEL" 2>$null
        foreach ($volume in $volumes) {
            $folder = (& docker volume inspect $volume --format '{{json .Labels}}' 2>$null | ConvertFrom-Json).$LINUX_TARGET_LABEL
            if ($folder -and -not (Test-Path -LiteralPath $folder)) {
                $null = & docker volume rm $volume 2>&1
                if ($LASTEXITCODE -eq 0) {
                    Write-Host "    volume $volume removed (folder $folder no longer exists)" -ForegroundColor DarkGray
                }
            }
        }
    } finally {
        $ErrorActionPreference = $prev
    }
}

# The Linux build leaves ~5 GB of file cache inside the Docker Desktop VM
# (`VmmemWSL`), and `autoMemoryReclaim=gradual` hands it back only slowly.
# Dropping the cache returns it to Windows at once (measured 30/10: 4.9 GB ->
# 1.2 GB) and touches nothing else: no container, volume or image. Best
# effort: without WSL or that distro it says so and the gate goes on.
function Clear-WslCache {
    if (-not (Get-Command wsl -ErrorAction SilentlyContinue)) { return }
    $null = & wsl -d docker-desktop -u root sh -c 'sync; echo 3 > /proc/sys/vm/drop_caches' 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host '    (WSL cache not cleared: docker-desktop distro not found)' -ForegroundColor DarkGray
    }
}

function Invoke-LinuxInDockerStep {
    $stepName = 'Linux tests (docker)'
    Write-Title $stepName

    Remove-OrphanLinuxTargets
    # Created with the label that says whose it is; an existing one is kept.
    $null = & docker volume create --label "$LINUX_TARGET_LABEL=$PSScriptRoot" $LINUX_TARGET_VOLUME 2>&1

    # `$PSScriptRoot` contains backslashes and docker rejects them in volume mounts.
    $rootPath = $PSScriptRoot -replace '\\', '/'

    $dockerArgs = @(
        'run', '--rm',
        '-v', "${rootPath}:$LINUX_MOUNT",
        '-w', $LINUX_MOUNT,
        '-v', "${LINUX_TARGET_VOLUME}:$LINUX_TARGET",
        '-v', "${LINUX_REGISTRY_VOLUME}:/usr/local/cargo/registry",
        '-e', "CARGO_TARGET_DIR=$LINUX_TARGET",
        $LINUX_IMAGE,
        'cargo', 'test', '--workspace', '--no-fail-fast'
    )
    Write-Host "    docker $($dockerArgs -join ' ')" -ForegroundColor DarkGray

    # Output is printed AND captured: printed because errors must be
    # readable without re-executing, captured because expected set verification
    # operates on it. `[string]$_` because PowerShell's `2>&1` delivers stderr
    # as error records, while downstream `-match` requires text.
    #
    # And the same cage as `Get-NoDockerReason`, here for a heavier reason:
    # `cargo` writes ALL its progress to stderr — "Compiling", "Running",
    # the line for each test. With `$ErrorActionPreference = 'Stop'`, the first
    # such line would become a terminating exception and the step would abort before
    # compiling the first crate.
    $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
    $prev = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $output = & docker $dockerArgs 2>&1 | ForEach-Object {
            $line = [string]$_
            Write-Host $line
            $line
        }
        $code = $LASTEXITCODE
    }
    finally {
        $ErrorActionPreference = $prev
    }
    $stopwatch.Stop()
    $seconds = [math]::Round($stopwatch.Elapsed.TotalSeconds, 2)
    Clear-WslCache

    $foundFailures = @(Get-Failures -Output $output)

    # Exited non-zero and no test failed: it was not a test failure.
    # It was compilation, docker itself, or volume mounting. This must NOT pass through
    # set comparison — with an empty list it would treat a container that did not even compile
    # as "expected".
    if ($code -ne 0 -and $foundFailures.Count -eq 0) {
        Record-Step $stepName 'FAILED' $seconds
        Write-Summary
        Write-Host "RED GATE — failed at: $stepName (exit code $code)" -ForegroundColor Red
        Write-Host '  No tests failed, so this was not a test failure: inspect output above.' -ForegroundColor Red
        Write-Host '  Compilation, volume mounting, or docker itself.' -ForegroundColor Red
        Write-Host ''
        exit 1
    }

    $newFailures = @($foundFailures | Where-Object { $LINUX_EXPECTED_RED -notcontains $_ })
    $fixedFailures = @($LINUX_EXPECTED_RED | Where-Object { $foundFailures -notcontains $_ })

    if ($newFailures.Count -eq 0 -and $fixedFailures.Count -eq 0) {
        if ($code -eq 0) {
            # Empty list and green suite: the debt is paid off. Not an
            # error case, and the step behaves as a normal step.
            Record-Step $stepName 'ok' $seconds
            return
        }

        $script:expectedRed = $foundFailures.Count
        Record-Step $stepName "EXPECTED RED ($($foundFailures.Count))" $seconds
        Write-Host ''
        Write-Host "The $($foundFailures.Count) Linux failures MATCH EXACTLY what" -ForegroundColor Yellow
        Write-Host '`$LINUX_EXPECTED_RED` lists in this file, one by one. No new' -ForegroundColor Yellow
        Write-Host 'regressions, and no expired exceptions. The gate proceeds.' -ForegroundColor Yellow
        return
    }

    Record-Step $stepName 'FAILED' $seconds
    Write-Summary
    Write-Host "RED GATE — failed at: $stepName" -ForegroundColor Red
    Write-Host ''

    if ($newFailures.Count -gt 0) {
        Write-Host "REGRESSION ON LINUX — $($newFailures.Count) unexpected failure(s):" -ForegroundColor Red
        foreach ($f in $newFailures) { Write-Host "    $f" -ForegroundColor Red }
        Write-Host ''
        Write-Host '  Fix the test (or the code). Only add to expected red list' -ForegroundColor Red
        Write-Host '  with documented reason alongside, like the others.' -ForegroundColor Red
        Write-Host ''
    }

    if ($fixedFailures.Count -gt 0) {
        Write-Host "THE LIST IS STALE — $($fixedFailures.Count) expected failure(s) did not occur:" -ForegroundColor Red
        foreach ($f in $fixedFailures) { Write-Host "    $f" -ForegroundColor Red }
        Write-Host ''
        Write-Host '  The test passed, vanished, or was renamed. Remove the line from' -ForegroundColor Red
        Write-Host '  `$LINUX_EXPECTED_RED` — an exception that outlives the defect becomes' -ForegroundColor Red
        Write-Host '  noise, and noise is what causes a gate to stop being read.' -ForegroundColor Red
        Write-Host ''
        Write-Host '  (If the entire list vanished at once, inspect output above: the' -ForegroundColor Red
        Write-Host '  container might not have compiled.)' -ForegroundColor Red
        Write-Host ''
    }

    exit 1
}

# --------------------------------------------------------------- pre-flight ---

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Stop-Missing '`cargo` is not in PATH.' 'install Rust via https://rustup.rs'
}

# Targets are only required when the steps that use them will run.
#
# And `rustup` is queried BEFORE being invoked, like `cargo` and
# `cargo-deny` right above and below. Without this line, on a machine with cargo
# but without rustup — distro-packaged Rust, or `pwsh` on Linux as contemplated in
# `.NOTES` —, the call below throws under `$ErrorActionPreference = 'Stop'` and
# exits with **1**, indistinguishable from a red gate. This is the rule documented
# in `Stop-Missing`: missing tool is not a red gate, it is a gate that could not run,
# and exits with 2.
if (-not $Fast) {
    if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
        Stop-Missing '`rustup` is not in PATH, and it installs the ADR 0007 targets.' 'install Rust via https://rustup.rs'
    }
    $installedTargets = @(rustup target list --installed)
    $missingTargets = @($TARGETS | Where-Object { $installedTargets -notcontains $_ })
    if ($missingTargets.Count -gt 0) {
        $targetList = $missingTargets -join ', '
        $targetAdd = $missingTargets -join ' '
        Stop-Missing "missing rustup targets: $targetList" "rustup target add $targetAdd"
    }
}

if (-not $NoDeny) {
    if (-not (Get-Command cargo-deny -ErrorAction SilentlyContinue)) {
        Stop-Missing '`cargo-deny` is not installed.' 'cargo install --locked cargo-deny'
    }
}

# -------------------------------------------------------------------- steps ---

Write-Host ''
Write-Host "CRAWRAD LOCAL GATE — $mode mode" -ForegroundColor Green
if ($Fast) {
    Write-Host 'The two `#[ignore]` gates, the two `cargo clippy --target`, and the Linux' -ForegroundColor Yellow
    Write-Host 'suite (docker) will NOT run.' -ForegroundColor Yellow
}

Invoke-Step 'fmt' @('cargo', 'fmt', '--all', '--', '--check')
Invoke-Step 'clippy' @('cargo', 'clippy', '--workspace', '--all-targets', '--', '-D', 'warnings')

if ($Fast) {
    Invoke-Step 'tests (without expensive gates)' @('cargo', 'test', '--workspace')
    Mark-Skipped 'tests `#[ignore]`' '-Fast'
    foreach ($target in $TARGETS) {
        Mark-Skipped "clippy $target" '-Fast'
    }
    Mark-Skipped 'Linux tests (docker)' '-Fast'
    $noLinuxProof = '-Fast'
}
else {
    # `--include-ignored` is NOT a convenience: without it the two most expensive gates
    # of the project remain inert. `gate_fase3` generates a Cargo project, compiles for
    # real and drives click headless; `gate_sem_avisos` generates six
    # documents and requires clean `clippy -D warnings` on each.
    Invoke-Step 'tests + two expensive gates' @('cargo', 'test', '--workspace', '--', '--include-ignored')

    # The shipped examples are projects the IDE created: each one must still build,
    # warning-free, so none of them rots behind a change to the runtime.
    foreach ($example in Get-ChildItem -Path (Join-Path $PSScriptRoot 'examples') -Directory) {
        $manifest = Join-Path $example.FullName 'Cargo.toml'
        if (Test-Path $manifest) {
            Invoke-Step "example $($example.Name)" @('cargo', 'clippy', '-q', '--manifest-path', $manifest, '--', '-D', 'warnings')
        }
    }

    # The SQLite drivers are behind features the workspace never turns on: their
    # tests (and one that did not even compile with them) ran nowhere (#621).
    Invoke-Step 'portadb with its SQLite drivers' @('cargo', 'test', '-q', '-p', 'portadb', '--features', 'rusqlite,sqlx-sqlite')

    # #788: a bound screen writing to real SQLite, behind the same C driver.
    Invoke-Step 'crawrad-binding round trip on SQLite' @('cargo', 'test', '-q', '-p', 'crawrad-binding', '--features', 'rusqlite', '--test', 'dataset_round_trip')

    # #621: the portadb's cost per query, as a REPORT (never a latency gate:
    # a latency gate measures the machine). Red only if the benchmark breaks.
    Invoke-Step 'portadb benchmark (report)' @('cargo', 'bench', '-q', '-p', 'portadb', '--bench', 'query_rows', '--features', 'rusqlite,sqlx-sqlite', '--', '--report')

    # Not `build`: a dependency's `build.rs` decides by TARGET, and without
    # linking no library from the foreign system needs to exist here. That was how,
    # from within Windows, the `rfd` defect on Linux was found and its fix verified (ADR 0007).
    #
    # And it IS `clippy -D warnings`, NOT `check`, since 07/09/2026. While
    # `ci.yml` had `RUSTFLAGS: -D warnings` and jobs on `ubuntu-latest` and
    # `macos-latest`, warnings outside Windows were rejected by someone. Both
    # were removed on the same day, and nobody noticed what was left: the `clippy`
    # step above compiles only for the HOST target, and the Linux container
    # runs `cargo test`, which warns and continues. There was NO ONE rejecting
    # warnings outside Windows.
    #
    # It WAS WRITTEN HERE that what remained unjudged was "all code behind
    # `#[cfg(unix)]`", and that phrase described an empty room: measured on
    # 07/09/2026, there is ONE site of CODE with `#[cfg(target_os = ...)]` in the entire
    # workspace (the other grep hits are comments about this very decision):
    # `crates/crawrad-ide/build.rs`, and it is `"windows"`. There is no `#[cfg(unix)]`,
    # `linux`, or `macos` in the code. The change remains correct, and the honest
    # reason is stronger than the inflated one: **nothing rejected warnings outside
    # Windows, the two cross-compilations were already being paid for anyway, and
    # `-D warnings` on them costs less than 1 s warm** — closing the door before
    # the first line of `#[cfg(unix)]` is written, which is the only time closing
    # doors is cheap.
    #
    # The MECHANISM was proven via mutation, and sustains the switch: with a
    # planted `#[cfg(target_os = "linux")] fn surplus() { let unused = 1; }` in
    # `crawrad-ir`, the old step (`check` per target) returns **rc=0 printing the
    # two warnings**, while this step returns **rc=101**.
    #
    # THE COST. A cold times table and a "four cold seconds" used to be here, and
    # were REMOVED: that method — deleting only `target/<target>/` — does not isolate
    # what it promises. The host's `target/debug` stays warm, with build scripts
    # and proc-macros inside, so whichever runs second gets a free ride and measurement
    # becomes execution order; redone cold with that same method, the published direction
    # inverted. What holds is the number governing daily use, measured on this machine
    # on 07/09/2026 with both targets warm:
    #
    #     clippy warm: linux 0.77 s, darwin 0.69 s
    #     check  warm: linux 0.76 s, darwin 0.54 s
    #
    # Warm, the switch costs nothing — and warm is how this gate almost always runs.
    # Cold, cargo's `clippy` only runs WORKSPACE crates through `clippy-driver`
    # (dependencies remain compiled by regular `rustc` and reused), so the switch
    # does not pay for a second tree; what it costs exactly is a figure nobody has
    # measured with an isolating method, which is why it is not stated here. Whoever
    # measures it, document the method alongside.
    foreach ($target in $TARGETS) {
        if ($target -match 'darwin' -and [System.Environment]::OSVersion.Platform -match 'Win') {
            Ensure-DarwinCrossShim
        }
        if ($target -match 'linux' -and [System.Environment]::OSVersion.Platform -match 'Win') {
            Ensure-LinuxCrossShim
        }
        # The bundled SQLite (`datagrid-sql`, `crawrad-dbprobe`, `crawrad-settings`)
        # compiles C for the TARGET. On Windows, `zig cc` is that cross compiler,
        # through the wrappers in `scripts/zigcc` (they drop the clang
        # `--target=` flag cc-rs adds, which zig rejects). Zig is required.
        if ([System.Environment]::OSVersion.Platform -match 'Win') {
            if (-not (Get-Command zig -ErrorAction SilentlyContinue)) {
                throw 'zig not found: install it (winget install zig.zig) for the cross clippy step'
            }
            $zigcc = Join-Path $PSScriptRoot 'scripts\zigcc'
            $env:CC_x86_64_unknown_linux_gnu = Join-Path $zigcc 'x86_64-linux-gnu.cmd'
            $env:AR_x86_64_unknown_linux_gnu = Join-Path $zigcc 'ar.cmd'
            $env:CC_aarch64_apple_darwin = Join-Path $zigcc 'aarch64-macos.cmd'
            $env:AR_aarch64_apple_darwin = Join-Path $zigcc 'ar.cmd'
        }
        # Apple from Windows: `--cfg crawrad_cross_check` drops `cpal` from
        # `crawrad-audio` (its `coreaudio-sys` needs the real macOS SDK, and the
        # Apple licence forbids it on a non-Apple machine, ADR 0034). The step
        # name says so, so the summary never claims the Mac audio was judged.
        $stepName = "clippy $target"
        if ($target -match 'darwin' -and [System.Environment]::OSVersion.Platform -match 'Win') {
            $env:CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS = '--cfg crawrad_cross_check'
            $stepName = "clippy $target (no Apple audio)"
        }
        Invoke-Step $stepName @('cargo', 'clippy', '--workspace', '--all-targets', '--target', $target, '--', '-D', 'warnings')
        Remove-Item Env:CARGO_TARGET_AARCH64_APPLE_DARWIN_RUSTFLAGS -ErrorAction SilentlyContinue
    }

    # And here `clippy` above ceases to be the final word on Linux: the suite
    # runs for real, on real Linux. The rationale, measured cost, and expected
    # red list are documented above, alongside `$LINUX_IMAGE`.
    if ($NoDocker) {
        Mark-Skipped 'Linux tests (docker)' '-NoDocker'
        $noLinuxProof = '-NoDocker'
    }
    else {
        $noDockerReason = Get-NoDockerReason
        if ($noDockerReason) {
            Write-Title 'Linux tests (docker)'
            Write-Host "    SKIPPED: $noDockerReason" -ForegroundColor Yellow
            Write-Host '    BEHAVIOR proof outside Windows was not performed in this run.' -ForegroundColor Yellow
            Mark-Skipped 'Linux tests (docker)' $noDockerReason
            $noLinuxProof = $noDockerReason
        }
        else {
            Invoke-LinuxInDockerStep
        }
    }
}

if ($NoDeny) {
    Mark-Skipped 'deny (licenses)' '-NoDeny'
}
else {
    Invoke-Step 'deny (licenses)' @('cargo', 'deny', 'check', 'licenses')
}

# ------------------------------------------------------------------ the end ---

Write-Summary

Write-Host 'WHAT THIS GATE DOES NOT COVER — read before calling green:' -ForegroundColor Yellow
# This line used to say "no test runs on Linux or macOS", which became a half-truth
# the day the Docker step was added. A half-truth in a list that exists to be honest
# is worse than no list at all.
Write-Host '  - macOS: nothing runs. Only `cargo clippy --target` exists there, proving that'
Write-Host '    code COMPILES without warnings, not that it behaves — and there is no Apple'
Write-Host '    machine in this setup. On Linux that ceased to be true: the suite runs in a container.'
Write-Host '  - On Linux, what does NOT run in the container are the two `#[ignore]` gates:'
Write-Host '    they invoke `cargo add`, require network access inside, and judge the GENERATED project,'
Write-Host '    which is a Windows product. The local test step verifies them here.'
# This line used to say "Nobody opens the IDE on any platform other than this one" — a
# claim about THE WORLD inside a list about THE GATE. The gate could never know what
# nobody did, and on 07/09/2026 the phrase expired: the IDE was opened on real Linux
# (WSL Ubuntu-24.04 with WSLg). What the gate can say, and says now, is what IT does.
Write-Host '  - This gate does not launch the IDE on any platform. That the window opens,'
Write-Host '    rendering matches expectations, and the folder dialog responds — on Linux it'
Write-Host '    depends on an `xdg-desktop-portal` over D-Bus — no step here proves (ADR 0007).'
Write-Host '    The container is headless and does not change this.'
Write-Host '    There was ONE manual run on Linux (WSLg, 07/09/2026): the window opens and the'
Write-Host '    GTK picker opens via portal — and on the Wayland path the IDE ABORTS (issue'
Write-Host '    #39). That was a manual, unrepeatable action, not a gate step. macOS remains'
Write-Host '    without proof of any kind.'
Write-Host '  - `cargo deny check advisories` does not run here, and what checks it DOES NOT BLOCK:'
Write-Host '    it is a dedicated CI step marked `continue-on-error`. Measured on 07/09/2026,'
Write-Host '    with the three targets in `deny.toml`, it finds three transitive issues in the'
Write-Host '    Linux/macOS graph that none of our changes fixes — and ignoring them is'
Write-Host '    an owner decision, with date and rationale, that has not yet been taken. See'
Write-Host '    `deny.toml` and `.github/workflows/ci.yml`.'
if ($Fast) {
    Write-Host '  - And this was FAST mode: expensive steps did not run.' -ForegroundColor Yellow
}

Write-Host ''
if ($expectedRed -gt 0) {
    # The last line is what gets pasted into reports. If the count is not IN IT,
    # someone will quote "GREEN GATE" without the rest of the sentence — and then
    # the gate lies by omission, which is the cheapest way to lie.
    Write-Host "GREEN GATE — $mode mode, WITH $expectedRed EXPECTED REDS ON LINUX" -ForegroundColor Green
    Write-Host 'They are tracked debt (issue #6 and Windows assumptions inside tests),' -ForegroundColor Yellow
    Write-Host 'named one by one in `$LINUX_EXPECTED_RED` in this file, and the gate' -ForegroundColor Yellow
    Write-Host 'turns RED the day one more appears — or one fewer.' -ForegroundColor Yellow
}
elseif ($noLinuxProof) {
    # The other way to lie by omission on the SAME line: with Docker daemon down,
    # the step skipped and the last line was, character for character, that of a run
    # that proved Linux. Pasting that output in review would claim more than ran.
    Write-Host "GREEN GATE — $mode mode, WITHOUT LINUX PROOF (docker skipped: $noLinuxProof)" -ForegroundColor Green
    Write-Host 'The suite did NOT run on Linux in this run, so this output says nothing about' -ForegroundColor Yellow
    Write-Host 'BEHAVIOR outside Windows — the eleven known failures were not even checked.' -ForegroundColor Yellow
    Write-Host 'Run without `-Fast` and without `-NoDocker`, with the daemon running, before' -ForegroundColor Yellow
    Write-Host 'calling this green complete.' -ForegroundColor Yellow
}
else {
    Write-Host "GREEN GATE — $mode mode" -ForegroundColor Green
}
Write-Host ''
exit 0
