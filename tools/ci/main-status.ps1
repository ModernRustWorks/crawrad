# #66: the house learns that `main` turned red the moment it does.
#
# The PR check is computed against a base that may no longer exist, so a green
# PR can merge into a red `main`, and once nobody knew for three hours and four
# merges. The CI calls this after the gate on every push to `main`:
#
#   failure -> opens the issue "main is red", or comments on the one already
#              open, with the commit and the run that broke;
#   success -> closes that issue, saying which commit made `main` green again.
#
# So at most one such issue is ever open, and only while `main` is red. Branch
# protection, which would stop the merge itself, is not available on this
# plan (private repo, HTTP 403), which is why the signal is an issue.
#
# -DryRun prints what it would do and touches nothing (it still reads the open
# issues), which is how it is tried by hand; -Title points it at an issue that
# is already open, to try the comment and close paths without opening one.
param(
    [Parameter(Mandatory)][ValidateSet('success', 'failure')][string]$Result,
    [Parameter(Mandatory)][string]$Sha,
    [Parameter(Mandatory)][string]$RunUrl,
    [string]$Title = 'main is red',
    [switch]$DryRun
)
$ErrorActionPreference = 'Stop'
$title = $Title

$open = gh issue list --state open --limit 100 --json number,title |
    ConvertFrom-Json |
    Where-Object { $_.title -eq $title } |
    Select-Object -First 1

function Invoke-Gh([string[]]$ghArgs) {
    if ($DryRun) {
        Write-Output ("DRY RUN: gh " + ($ghArgs -join ' '))
    } else {
        & gh @ghArgs
        if ($LASTEXITCODE -ne 0) { throw "gh $($ghArgs[0]) $($ghArgs[1]) failed ($LASTEXITCODE)" }
    }
}

$short = $Sha.Substring(0, [Math]::Min(8, $Sha.Length))
if ($Result -eq 'failure') {
    $body = "The gate failed on ``main`` at $short.`n`nRun: $RunUrl`n`nDo not merge on top of it until ``main`` is green again; this issue closes itself on the next green push."
    if ($open) {
        Invoke-Gh @('issue', 'comment', "$($open.number)", '--body', "Still red at $short.`n`nRun: $RunUrl")
    } else {
        Invoke-Gh @('issue', 'create', '--title', $title, '--body', $body)
    }
} elseif ($open) {
    Invoke-Gh @('issue', 'close', "$($open.number)", '--comment', "``main`` is green again at $short.`n`nRun: $RunUrl")
} else {
    Write-Output "main is green at $short and no '$title' issue is open: nothing to do."
}
