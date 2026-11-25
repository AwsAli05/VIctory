# Run a bounded playtest and keep the frame-rate samples with its exit status.
[CmdletBinding()]
param(
    [Parameter(Mandatory)][string]$Victory,
    [Parameter(Mandatory)][string]$Title,
    [Parameter(Mandatory)][string]$Config,
    [ValidateSet('series-x', 'series-s')][string]$Console = 'series-x',
    [string]$Keys = '',
    [ValidateRange(1, 86400)][int]$Seconds = 180,
    [ValidateRange(0, 1048576)][int]$MinimumFreeMemoryMiB = 2048,
    [string]$OutputDirectory = (Join-Path $env:LOCALAPPDATA 'victory\playtests')
)

$ErrorActionPreference = 'Stop'
$launcher = Get-Item -LiteralPath $Victory
$image = Get-Item -LiteralPath $Title
$settings = Get-Item -LiteralPath $Config
if ($launcher.PSIsContainer -or $image.PSIsContainer -or $settings.PSIsContainer) {
    throw 'Victory, Title and Config must each point to a file.'
}

$run = New-Item -ItemType Directory -Path (Join-Path $OutputDirectory ('{0:yyyyMMdd-HHmmss-fffffff}-{1}' -f [datetime]::UtcNow, $PID))
$stdout = Join-Path $run.FullName 'stdout.log'
$stderr = Join-Path $run.FullName 'stderr.log'
Write-Host "Playtest logs are in $($run.FullName)"

$start = New-Object System.Diagnostics.ProcessStartInfo
$start.FileName = $launcher.FullName
$start.Arguments = 'run "{0}" --execute --console {1}' -f $image.FullName, $Console
$start.WorkingDirectory = $image.DirectoryName
$start.UseShellExecute = $false
$start.CreateNoWindow = $true
$start.RedirectStandardOutput = $true
$start.RedirectStandardError = $true
$start.EnvironmentVariables['VICTORY_CONFIG'] = $settings.FullName
$start.EnvironmentVariables['VICTORY_KEYS'] = $Keys
$start.EnvironmentVariables['VICTORY_LOG'] = 'info'

$elapsed = New-Object System.Diagnostics.Stopwatch
$reason = 'process-exited'
$lowestFreeKiB = $null
$stdoutFile = [System.IO.File]::Open($stdout, 'CreateNew', 'Write', 'Read')
try {
    $stderrFile = [System.IO.File]::Open($stderr, 'CreateNew', 'Write', 'Read')
    try {
        $game = [System.Diagnostics.Process]::Start($start)
        $elapsed.Restart()
        try {
            $stdoutCopy = $game.StandardOutput.BaseStream.CopyToAsync($stdoutFile)
            $stderrCopy = $game.StandardError.BaseStream.CopyToAsync($stderrFile)
            while (-not $game.HasExited) {
                $remainingMs = $Seconds * 1000 - $elapsed.ElapsedMilliseconds
                if ($remainingMs -le 0) {
                    $reason = 'time-limit'
                    break
                }
                if ($MinimumFreeMemoryMiB -gt 0) {
                    $freeKiB = (Get-CimInstance -ClassName Win32_OperatingSystem -Property FreeVirtualMemory).FreeVirtualMemory
                    if ($null -ne $freeKiB) {
                        if ($null -eq $lowestFreeKiB -or $freeKiB -lt $lowestFreeKiB) {
                            $lowestFreeKiB = $freeKiB
                        }
                        if ($freeKiB -lt $MinimumFreeMemoryMiB * 1024) {
                            $reason = 'low-memory'
                            break
                        }
                    }
                }
                [void]$game.WaitForExit([int][math]::Min(2000, $remainingMs))
            }
        } finally {
            if (-not $game.HasExited) {
                $game.Kill()
            }
            $game.WaitForExit()
            $elapsed.Stop()
        }
        try {
            [void]$stdoutCopy.GetAwaiter().GetResult()
            [void]$stderrCopy.GetAwaiter().GetResult()
            $exitCode = $game.ExitCode
        } finally {
            $game.Dispose()
        }
    } finally {
        $stderrFile.Dispose()
    }
} finally {
    $stdoutFile.Dispose()
}

$culture = [System.Globalization.CultureInfo]::InvariantCulture
$samples = @(Get-Content -LiteralPath $stderr | ForEach-Object {
    if ($_ -match '^(?<utc>\S+)\s+INFO\s+victory_gpu::host: (?<fps>[\d.]+) fps, (?<ms>[\d.]+) ms/frame .*?, (?<draws>\d+) draws/frame,') {
        [ordered]@{
            utc = $Matches.utc
            fps = [double]::Parse($Matches.fps, $culture)
            milliseconds_per_frame = [double]::Parse($Matches.ms, $culture)
            draws_per_frame = [int]$Matches.draws
        }
    }
})
$result = [ordered]@{
    reason = $reason
    process_exit_code = $exitCode
    elapsed_seconds = [math]::Round($elapsed.Elapsed.TotalSeconds, 2)
    lowest_free_memory_kib = $lowestFreeKiB
    samples = $samples
}
$result | ConvertTo-Json -Depth 4 | Set-Content -LiteralPath (Join-Path $run.FullName 'summary.json') -Encoding UTF8
Write-Host "$reason after $($result.elapsed_seconds) seconds, $($samples.Count) frame-rate samples."

switch ($reason) {
    'time-limit' { exit 124 }
    'low-memory' { exit 125 }
    default { exit $exitCode }
}
