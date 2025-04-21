@echo off
echo Uninstalling Browsea...

:: Check if running as administrator
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Please run as administrator
    pause
    exit /b 1
)

:: Remove registry entries
reg delete "HKCU\Software\Classes\Browsea" /f
reg delete "HKCU\Software\Clients\StartMenuInternet\Browsea" /f
reg delete "HKCU\Software\RegisteredApplications" /v "Browsea" /f

:: Remove program files
if exist "%ProgramFiles%\Browsea" (
    rmdir /s /q "%ProgramFiles%\Browsea"
)   

:: Clean up any remaining files in temp
del /f /q "%TEMP%\Browsea*" 2>nul

echo Uninstallation complete!
pause 