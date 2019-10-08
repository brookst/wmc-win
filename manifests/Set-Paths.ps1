# Smudge @EXECUTABLE_PATH@ in manifest JSON files

$ApplicationFolder = Get-ItemProperty -Path HKCU:\SOFTWARE\wmc-win -name ApplicationFolder
$ApplicationFolder = ($ApplicationFolder.ApplicationFolder) -replace '\\','\\'
$BinPath = Join-Path -Path $ApplicationFolder -ChildPath "bin\\wmc-win.exe"
$ChromeManifest = Join-Path -Path $ApplicationFolder -ChildPath "me.f1u77y.web_media_controller.chromium.json"
$FirefoxManifest = Join-Path -Path $ApplicationFolder -ChildPath "me.f1u77y.web_media_controller.firefox.json"

if (Test-Path $ChromeManifest) {
    (Get-Content $ChromeManifest) -replace '@EXECUTABLE_PATH@',"$BinPath" | Set-Content -Path $ChromeManifest
}
if (Test-Path $FirefoxManifest) {
    (Get-Content $FirefoxManifest) -replace '@EXECUTABLE_PATH@',"$BinPath" | Set-Content -Path $FirefoxManifest
}
