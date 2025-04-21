@echo off
echo Cleaning previous builds...
if exist "dist" rmdir /s /q "dist"
if exist "target" rmdir /s /q "target"

echo Building release version...
cargo build --release

echo Creating distribution package...
mkdir dist\Browsea
copy target\release\browsea.exe dist\Browsea\Browsea.exe
copy target\release\browsea.exe.manifest dist\browsea\browsea.exe.manifest
copy install.bat dist\Browsea\
copy uninstall.bat dist\Browsea\

echo Copying assets...
mkdir dist\Browsea\assets
xcopy /E /I /Y assets dist\Browsea\assets
xcopy /E /I /Y src\assets dist\Browsea\src\assets

echo Creating ZIP archive...
set CURRENT_DIR=%CD%
powershell -Command "Compress-Archive -Path '%CURRENT_DIR%\dist\Browsea' -DestinationPath '%CURRENT_DIR%\dist\Browsea.zip' -Force"

echo Build completed successfully!
echo Distribution files are in the dist\Browsea folder
echo ZIP archive is in the dist\Browsea.zip file