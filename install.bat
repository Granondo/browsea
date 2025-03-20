@echo off
:: Change to the directory where the script is located
cd /d "%~dp0"

echo Installing Browser Picker...

:: Check if running as administrator
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Please run this script as Administrator
    echo Right-click on install.bat and select "Run as administrator"
    pause
    exit /b 1
)

:: Check if executable exists (try both possible names)
if exist "BrowserPicker.exe" (
    set EXENAME=BrowserPicker.exe
) else if exist "browser_picker.exe" (
    set EXENAME=browser_picker.exe
) else (
    echo ERROR: Browser picker executable not found in current directory
    echo Current directory: %CD%
    echo Please make sure:
    echo 1. You extracted the ZIP file completely
    echo 2. You're running install.bat from the extracted folder
    echo 3. The browser picker executable is in the same folder as install.bat
    pause
    exit /b 1
)

:: Create program directory with error handling
echo Creating program directory...
if not exist "%ProgramFiles%\BrowserPicker" (
    mkdir "%ProgramFiles%\BrowserPicker" 2>nul
    if %errorlevel% neq 0 (
        echo ERROR: Failed to create directory in %ProgramFiles%\BrowserPicker
        echo Please make sure you have administrator rights
        pause
        exit /b 1
    )
)

:: Copy the executable with error checking
echo Copying files...
copy /Y "%EXENAME%" "%ProgramFiles%\BrowserPicker\BrowserPicker.exe" >nul
if %errorlevel% neq 0 (
    echo ERROR: Failed to copy %EXENAME%
    echo Source: %CD%\%EXENAME%
    echo Destination: %ProgramFiles%\BrowserPicker\BrowserPicker.exe
    echo Please make sure you have proper permissions
    pause
    exit /b 1
)

:: Copy assets folders if they exist
echo Copying assets...
if exist "assets" (
    if not exist "%ProgramFiles%\BrowserPicker\assets" mkdir "%ProgramFiles%\BrowserPicker\assets"
    xcopy /E /I /Y assets "%ProgramFiles%\BrowserPicker\assets" >nul
)

if exist "src\assets" (
    if not exist "%ProgramFiles%\BrowserPicker\src\assets" mkdir "%ProgramFiles%\BrowserPicker\src\assets"
    xcopy /E /I /Y src\assets "%ProgramFiles%\BrowserPicker\src\assets" >nul
)

:: Register as browser handler
echo Registering browser handler...
"%ProgramFiles%\BrowserPicker\BrowserPicker.exe"
if %errorlevel% neq 0 (
    echo ERROR: Failed to register browser handler
    echo Please check if the executable is working properly
    pause
    exit /b 1
)

echo.
echo Installation completed successfully!
echo Please set Browser Picker as your default browser in Windows Settings
pause