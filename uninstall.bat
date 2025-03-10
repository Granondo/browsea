@echo off
echo Uninstalling Browser Picker...

:: Check if running as administrator
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Please run as administrator
    pause
    exit /b 1
)

:: Remove registry entries
reg delete "HKCU\Software\Classes\BrowserPicker" /f
reg delete "HKCU\Software\Clients\StartMenuInternet\BrowserPicker" /f
reg delete "HKCU\Software\Classes\http\shell\openWithPicker" /f
reg delete "HKCU\Software\Classes\https\shell\openWithPicker" /f
reg delete "HKCU\Software\RegisteredApplications" /v "BrowserPicker" /f

:: Remove program files
if exist "%ProgramFiles%\BrowserPicker" (
    rmdir /s /q "%ProgramFiles%\BrowserPicker"
)   

:: Clean up any remaining files in temp
del /f /q "%TEMP%\BrowserPicker*" 2>nul

echo Uninstallation complete!
pause 