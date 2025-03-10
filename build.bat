@echo off
echo Cleaning previous builds...
if exist "dist" rmdir /s /q "dist"
if exist "target" rmdir /s /q "target"

echo Building release version...
cargo build --release

echo Creating distribution package...
mkdir dist\BrowserPicker
copy target\release\browser_picker.exe dist\BrowserPicker\BrowserPicker.exe
copy install.bat dist\BrowserPicker\
copy uninstall.bat dist\BrowserPicker\

echo Creating ZIP archive...
set CURRENT_DIR=%CD%
powershell -Command "Compress-Archive -Path '%CURRENT_DIR%\dist\BrowserPicker' -DestinationPath '%CURRENT_DIR%\dist\BrowserPicker.zip' -Force"

echo Build completed successfully!
echo Distribution files are in the dist\BrowserPicker folder
echo ZIP archive is in the dist\BrowserPicker.zip file 