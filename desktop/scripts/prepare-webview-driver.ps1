# External CI setup only. Resolve WebView2 Runtime, never the Edge browser.
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)][string]$OutputDirectory,
    [string]$ReportPath = ""
)
$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') { throw 'WebView2 driver setup requires Windows.' }
$OutputDirectory = [IO.Path]::GetFullPath($OutputDirectory)
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null
if (-not $ReportPath) { $ReportPath = Join-Path $OutputDirectory 'webview-driver.json' }
$ReportPath = [IO.Path]::GetFullPath($ReportPath)
New-Item -ItemType Directory -Force -Path ([IO.Path]::GetDirectoryName($ReportPath)) | Out-Null
$client = '{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}'
$keys = @(
    "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\$client",
    "HKLM:\SOFTWARE\Microsoft\EdgeUpdate\Clients\$client",
    "HKCU:\Software\Microsoft\EdgeUpdate\Clients\$client"
)
$registry = @()
foreach ($key in $keys) {
    if (Test-Path -LiteralPath $key) {
        $pv = Get-ItemPropertyValue -LiteralPath $key -Name pv -ErrorAction SilentlyContinue
        if ($pv -match '^\d+\.\d+\.\d+\.\d+$' -and $pv -ne '0.0.0.0') {
            $registry += [pscustomobject]@{ key = $key; version = $pv }
        }
    }
}
if (-not $registry.Count) { throw 'No installed WebView2 Runtime pv found in the Microsoft runtime registry locations.' }
$runtime = ($registry | Sort-Object { [version]$_.version } -Descending | Select-Object -First 1).version
$runtimeBuild = (($runtime -split '\.')[0..2] -join '.')
$major = ($runtime -split '\.')[0]
$architecture = switch ([Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()) {
    'X64' { 'win64' }
    'X86' { 'win32' }
    'Arm64' { 'arm64' }
    default { throw 'Unsupported Windows driver architecture.' }
}
$archive = Join-Path $OutputDirectory 'edgedriver.zip'
$driverVersion = $runtime
$url = "https://msedgedriver.microsoft.com/$driverVersion/edgedriver_$architecture.zip"
try {
    Invoke-WebRequest -Uri $url -OutFile $archive -TimeoutSec 30
} catch {
    # Runtime and driver patch versions can differ. Microsoft's compatibility
    # contract requires all first three components, never just the major.
    $releasePath = Join-Path $OutputDirectory 'latest-release.txt'
    $releaseUrl = "https://msedgewebdriverstorage.blob.core.windows.net/edgewebdriver/LATEST_RELEASE_${major}_WINDOWS"
    Invoke-WebRequest -Uri $releaseUrl -OutFile $releasePath -TimeoutSec 30
    $releaseText = [IO.File]::ReadAllText($releasePath).Replace([string][char]0, '').Trim()
    if ($releaseText -notmatch '^\d+\.\d+\.\d+\.\d+$') { throw "Invalid official EdgeDriver release metadata: $releaseText" }
    $driverVersion = $releaseText
    $driverBuild = (($driverVersion -split '\.')[0..2] -join '.')
    if ($driverBuild -ne $runtimeBuild) {
        throw "Installed WebView2 $runtime has no matching published EdgeDriver at $url; latest major driver $driverVersion has a different build. No mismatched driver was used."
    }
    $url = "https://msedgedriver.microsoft.com/$driverVersion/edgedriver_$architecture.zip"
    Invoke-WebRequest -Uri $url -OutFile $archive -TimeoutSec 30
}
Expand-Archive -LiteralPath $archive -DestinationPath $OutputDirectory -Force
$driver = Join-Path $OutputDirectory 'msedgedriver.exe'
if (-not (Test-Path -LiteralPath $driver -PathType Leaf)) { throw 'Official archive did not contain msedgedriver.exe.' }
$signature = Get-AuthenticodeSignature -LiteralPath $driver
if ($signature.Status -ne 'Valid' -or $signature.SignerCertificate.Subject -notmatch 'Microsoft Corporation') {
    throw "Downloaded EdgeDriver lacks a valid Microsoft Corporation Authenticode signature: $($signature.Status)"
}
$versionOutput = (& $driver --version | Out-String).Trim()
$versionMatch = [regex]::Match($versionOutput, '(\d+\.\d+\.\d+\.\d+)')
if ($LASTEXITCODE -ne 0 -or -not $versionMatch.Success) { throw "Cannot obtain EdgeDriver version: $versionOutput" }
$actualDriverVersion = $versionMatch.Groups[1].Value
if ($actualDriverVersion -ne $driverVersion -or (($actualDriverVersion -split '\.')[0..2] -join '.') -ne $runtimeBuild) {
    throw "EdgeDriver binary $actualDriverVersion does not match selected $driverVersion / WebView2 $runtime."
}
# Detect updater races instead of silently claiming the runtime selected earlier.
$observed = @($registry | ForEach-Object {
    $current = Get-ItemPropertyValue -LiteralPath $_.key -Name pv
    [pscustomobject]@{ key = $_.key; before = $_.version; after = $current }
})
if (@($observed | Where-Object { $_.before -ne $_.after }).Count) { throw 'WebView2 Runtime changed during driver preparation; rerun setup before GUI launch.' }
$report = [ordered]@{
    schema_version = 1
    runtime_version = $runtime
    runtime_source = 'highest valid installed WebView2 Runtime pv; all detected locations retained'
    registry = $registry
    runtime_registry_recheck = $observed
    driver_path = $driver
    driver_version = $actualDriverVersion
    driver_version_output = $versionOutput
    compatible_build = $runtimeBuild
    architecture = $architecture
    download_url = $url
    archive_sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $archive).Hash.ToLowerInvariant()
    sha256 = (Get-FileHash -Algorithm SHA256 -LiteralPath $driver).Hash.ToLowerInvariant()
    signer = $signature.SignerCertificate.Subject
    signature_status = $signature.Status.ToString()
    powershell = $PSVersionTable.PSVersion.ToString()
    os = [Runtime.InteropServices.RuntimeInformation]::OSDescription
    utc = [DateTime]::UtcNow.ToString('o')
}
$json = $report | ConvertTo-Json -Depth 8
[IO.File]::WriteAllText($ReportPath, $json, [Text.UTF8Encoding]::new($false))
Write-Output $json
