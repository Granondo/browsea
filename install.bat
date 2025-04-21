@echo off
:: Change to the directory where the script is located
cd /d "%~dp0"

echo Installing Browsea...

:: Check if running as administrator
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Please run this script as Administrator
    echo Right-click on install.bat and select "Run as administrator"
    pause
    exit /b 1
)

:: Check if executable exists (try both possible names)
if exist "Browsea.exe" (
    set EXENAME=Browsea.exe
) else if exist "browsea.exe" (
    set EXENAME=browsea.exe
) else (
    echo ERROR: Browsea executable not found in current directory
    echo Current directory: %CD%
    echo Please make sure:
    echo 1. You extracted the ZIP file completely
    echo 2. You're running install.bat from the extracted folder
    echo 3. The Browsea executable is in the same folder as install.bat
    pause
    exit /b 1
)

:: Create program directory with error handling
echo Creating program directory...
if not exist "%ProgramFiles%\Browsea" (
    mkdir "%ProgramFiles%\Browsea" 2>nul
    if %errorlevel% neq 0 (
        echo ERROR: Failed to create directory in %ProgramFiles%\Browsea
        echo Please make sure you have administrator rights
        pause
        exit /b 1
    )
)

:: Copy the executable with error checking
echo Copying files...
copy /Y "%EXENAME%" "%ProgramFiles%\Browsea\Browsea.exe" >nul
if %errorlevel% neq 0 (
    echo ERROR: Failed to copy %EXENAME%
    echo Source: %CD%\%EXENAME%
    echo Destination: %ProgramFiles%\Browsea\Browsea.exe
    echo Please make sure you have proper permissions
    pause
    exit /b 1
)

:: Copy assets folders if they exist
echo Copying assets...
if exist "assets" (
    if not exist "%ProgramFiles%\Browsea\assets" mkdir "%ProgramFiles%\Browsea\assets"
    xcopy /E /I /Y assets "%ProgramFiles%\Browsea\assets" >nul
)

if exist "src\assets" (
    if not exist "%ProgramFiles%\Browsea\src\assets" mkdir "%ProgramFiles%\Browsea\src\assets"
    xcopy /E /I /Y src\assets "%ProgramFiles%\Browsea\src\assets" >nul
)

:: Register as browser handler
echo Registering browser handler...
"%ProgramFiles%\Browsea\Browsea.exe"
if %errorlevel% neq 0 (
    echo ERROR: Failed to register browser handler
    echo Please check if the executable is working properly
    pause
    exit /b 1
)

echo.
echo Installation completed successfully!
echo Please set Browsea as your default browser in Windows Settings
pause