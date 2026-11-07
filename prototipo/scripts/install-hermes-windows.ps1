$ErrorActionPreference = "Stop"

if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) {
    throw "Este instalador é destinado ao Windows."
}

$installerUrl = "https://hermes-agent.nousresearch.com/install.ps1"
$installerPath = Join-Path $env:TEMP ("hermes-install-" + [Guid]::NewGuid().ToString("N") + ".ps1")

Write-Host "Baixando o instalador oficial do Hermes Agent..." -ForegroundColor Cyan
$installerScript = Invoke-RestMethod -Uri $installerUrl
if ([string]::IsNullOrWhiteSpace($installerScript)) {
    throw "O servidor oficial retornou um instalador vazio."
}

Set-Content -Path $installerPath -Value $installerScript -Encoding UTF8
$hash = (Get-FileHash -Path $installerPath -Algorithm SHA256).Hash
Write-Host ""
Write-Host "O instalador foi salvo para revisão em:" -ForegroundColor Yellow
Write-Host $installerPath
Write-Host "SHA-256: $hash"
Write-Host "Revise o conteúdo antes de autorizar a execução." -ForegroundColor Yellow
Start-Process -FilePath notepad.exe -ArgumentList "`"$installerPath`"" -Wait

$confirmation = Read-Host "Para executar o script revisado, digite EXECUTAR"
if ($confirmation -cne "EXECUTAR") {
    Write-Host "Instalação cancelada. O arquivo baixado foi mantido para inspeção."
    return
}

try {
    & $installerPath
} catch {
    Write-Host "A instalação do Hermes encontrou um erro:" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    throw
}

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$machinePath = [Environment]::GetEnvironmentVariable("Path", "Machine")
$env:Path = "$machinePath;$userPath"

$hermesCommand = Get-Command hermes -ErrorAction SilentlyContinue
if ($hermesCommand) {
    Write-Host ""
    Write-Host "Hermes instalado. Para concluir a configuração, execute: hermes setup" -ForegroundColor Green
    hermes --version
} else {
    Write-Host "O instalador terminou. Abra um novo PowerShell e execute: hermes --version" -ForegroundColor Yellow
}
