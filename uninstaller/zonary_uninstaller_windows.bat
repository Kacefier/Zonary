@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Zonary Interpreter Uninstaller for Windows
:: Made by Kacefier
:: Licensed under GPLv3. See LICENSE file in the root directory.

set VERSION=2.1.1
set BINARY_NAME=zonary.exe
set INSTALL_DIR=%USERPROFILE%\bin

echo Zonary Interpreter Uninstaller v%VERSION%
echo Made by Kacefier
echo.

:: Check if Zonary is installed
set "FOUND=false"
set "BINARY_PATH="

:: Check default location
if exist "%INSTALL_DIR%\%BINARY_NAME%" (
    set "FOUND=true"
    set "BINARY_PATH=%INSTALL_DIR%\%BINARY_NAME%"
    echo [INFO] Found Zonary at: !BINARY_PATH!
)

:: Check if it's in PATH
if "%FOUND%"=="false" (
    where zonary 2>nul > temp_path.txt
    set /p PATH_RESULT=<temp_path.txt
    del temp_path.txt 2>nul

    if not "!PATH_RESULT!"=="" (
        set "FOUND=true"
        set "BINARY_PATH=!PATH_RESULT!"
        echo [INFO] Found Zonary at: !BINARY_PATH!
    )
)

:: If not found, ask user
if "%FOUND%"=="false" (
    echo [WARN] Zonary not found in default location or PATH
    echo.
    set /p "CUSTOM_PATH=Enter the full path to zonary.exe (or press Enter to skip): "

    if "!CUSTOM_PATH!"=="" (
        echo [INFO] No action taken
        pause
        exit /b 0
    )

    if exist "!CUSTOM_PATH!" (
        set "FOUND=true"
        set "BINARY_PATH=!CUSTOM_PATH!"
        echo [INFO] Using custom path: !BINARY_PATH!
    ) else (
        echo [ERROR] File not found: !CUSTOM_PATH!
        pause
        exit /b 1
    )
)

if "%FOUND%"=="false" (
    echo [INFO] Zonary is not installed
    pause
    exit /b 0
)

:: Confirm uninstall
echo.
echo [WARN] This will remove: %BINARY_PATH%
echo.
set /p "CONFIRM=Are you sure you want to uninstall? (y/N): "

if /i not "%CONFIRM%"=="y" (
    echo [INFO] Uninstall cancelled
    pause
    exit /b 0
)

:: Remove binary
echo [INFO] Removing %BINARY_PATH%
del /f "%BINARY_PATH%" 2>nul

if errorlevel 1 (
    echo [ERROR] Failed to remove binary
    echo [INFO] Make sure you have write permissions
    pause
    exit /b 1
) else (
    echo [INFO] Binary removed successfully
)

:: Get directory path
for %%f in ("%BINARY_PATH%") do set "BIN_DIR=%%~dpf"
set "BIN_DIR=!BIN_DIR:~0,-1!"

:: Check if directory is empty and ask to remove
dir /b "!BIN_DIR!" 2>nul | findstr . >nul
if errorlevel 1 (
    echo.
    set /p "REMOVE_DIR=The directory !BIN_DIR! is now empty. Do you want to remove it? (y/N): "
    if /i "!REMOVE_DIR!"=="y" (
        rmdir "!BIN_DIR!" 2>nul
        if errorlevel 1 (
            echo [WARN] Could not remove directory: !BIN_DIR!
        ) else (
            echo [INFO] Removed directory: !BIN_DIR!
        )
    )
)

echo.
echo [INFO] Uninstall complete!
echo.
echo Zonary has been removed from your system

pause
