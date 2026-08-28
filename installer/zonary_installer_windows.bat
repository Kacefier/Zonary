@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

:: Zonary Interpreter Installer for Windows
:: Made by Kacefier
:: Licensed under GPLv3. See LICENSE file in the root directory.

set VERSION=2.1.1
set BINARY_NAME=zonary.exe
set INSTALL_DIR=%USERPROFILE%\bin

echo Zonary Interpreter Installer v%VERSION%
echo Made by Kacefier
echo.

:: Get binary path from user
echo Please enter the path to the Zonary binary file
echo.
set /p "BINARY_PATH=Binary path: "

:: Validate input
if "%BINARY_PATH%"=="" (
    echo [ERROR] Binary path cannot be empty
    pause
    exit /b 1
)

:: Remove quotes if present
set "BINARY_PATH=%BINARY_PATH:"=%"

:: Check if binary exists
if not exist "%BINARY_PATH%" (
    echo [ERROR] Binary not found: %BINARY_PATH%
    echo [INFO] Please make sure the path is correct
    pause
    exit /b 1
)

:: Check if it's a directory
if exist "%BINARY_PATH%\*" (
    echo [ERROR] Path points to a directory, expected a file: %BINARY_PATH%
    pause
    exit /b 1
)

echo [INFO] Binary found: %BINARY_PATH%

:: Make binary executable (Windows doesn't need chmod)
echo [INFO] Made binary executable

:: Create installation directory if it doesn't exist
if not exist "%INSTALL_DIR%" (
    echo [WARN] %INSTALL_DIR% does not exist
    echo [INFO] Creating %INSTALL_DIR%
    mkdir "%INSTALL_DIR%"
)

:: Install binary
echo [INFO] Installing to %INSTALL_DIR%
copy "%BINARY_PATH%" "%INSTALL_DIR%\%BINARY_NAME%" > nul

if errorlevel 1 (
    echo [ERROR] Failed to copy binary to %INSTALL_DIR%
    echo [INFO] Try running as Administrator
    pause
    exit /b 1
)

:: Verify installation
if exist "%INSTALL_DIR%\%BINARY_NAME%" (
    echo [INFO] Installation successful!
    echo [INFO] Version:
    "%INSTALL_DIR%\%BINARY_NAME%" -v
) else (
    echo [ERROR] Installation failed
    pause
    exit /b 1
)

:: Check if INSTALL_DIR is in PATH
echo %PATH% | findstr /C:"%INSTALL_DIR%" > nul
if errorlevel 1 (
    echo [WARN] %INSTALL_DIR% is not in your PATH
    echo [INFO] Adding %INSTALL_DIR% to PATH...
    
    setx PATH "%PATH%;%INSTALL_DIR%" > nul
    
    if errorlevel 1 (
        echo [WARN] Failed to add to PATH automatically
        echo [INFO] Please manually add to PATH:
        echo [INFO] set PATH="%INSTALL_DIR%;%%PATH%%"
    ) else (
        echo [INFO] Added %INSTALL_DIR% to PATH
        echo [WARN] Please restart your terminal or run: refreshenv
    )
) else (
    echo [INFO] %INSTALL_DIR% is already in PATH
)

echo.
echo [INFO] Installation complete!
echo.
echo You can now use 'zonary' command:
echo   zonary --help
echo   zonary -v
echo   zonary example.zonary
echo.
echo If 'zonary' is not found, please restart your terminal

pause
