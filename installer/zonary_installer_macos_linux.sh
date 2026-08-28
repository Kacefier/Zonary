#!/bin/bash
# Zonary Interpreter Installer for macOS/Linux
# Made by Kacefier
# Licensed under GPLv3. See LICENSE file in the root directory.

set -e

VERSION="2.1.1"
BINARY_NAME="zonary"
INSTALL_DIR="/usr/local/bin"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_header() {
    echo -e "${BLUE}Zonary Interpreter Installer v${VERSION}${NC}"
    echo -e "${BLUE}Made by Kacefier${NC}"
    echo ""
}

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header

# Get binary path from user
echo "Please enter the path to the Zonary binary file"
echo ""
read -p "Binary path: " BINARY_PATH

# Validate input
if [ -z "$BINARY_PATH" ]; then
    print_error "Binary path cannot be empty"
    exit 1
fi

# Expand ~ to home directory
BINARY_PATH="${BINARY_PATH/#\~/$HOME}"

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Binary not found: $BINARY_PATH"
    print_info "Please make sure the path is correct"
    exit 1
fi

# Check if it's a directory
if [ -d "$BINARY_PATH" ]; then
    print_error "Path points to a directory, expected a file: $BINARY_PATH"
    exit 1
fi

print_info "Binary found: $BINARY_PATH"

# Make binary executable
chmod +x "$BINARY_PATH"
print_info "Made binary executable"

# Check installation directory
if [ ! -d "$INSTALL_DIR" ]; then
    print_warn "$INSTALL_DIR does not exist"
    print_info "Creating $INSTALL_DIR"
    sudo mkdir -p "$INSTALL_DIR"
fi

# Install binary
print_info "Installing to $INSTALL_DIR"
sudo cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"

if [ $? -ne 0 ]; then
    print_error "Failed to copy binary to $INSTALL_DIR"
    print_info "Try running with sudo: sudo ./install.sh"
    exit 1
fi

# Verify installation
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    print_info "Installation successful!"
    print_info "Version: $($INSTALL_DIR/$BINARY_NAME -v)"
else
    print_error "Installation failed"
    exit 1
fi

# Check if INSTALL_DIR is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    print_warn "$INSTALL_DIR is not in your PATH"
    print_info "Adding $INSTALL_DIR to PATH..."
    
    # Detect shell and add to profile
    SHELL_NAME=$(basename "$SHELL")
    case "$SHELL_NAME" in
        bash)
            PROFILE_FILE="$HOME/.bashrc"
            ;;
        zsh)
            PROFILE_FILE="$HOME/.zshrc"
            ;;
        fish)
            PROFILE_FILE="$HOME/.config/fish/config.fish"
            ;;
        *)
            PROFILE_FILE="$HOME/.profile"
            ;;
    esac
    
    if [ -f "$PROFILE_FILE" ]; then
        echo "" >> "$PROFILE_FILE"
        echo "# Zonary Interpreter" >> "$PROFILE_FILE"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$PROFILE_FILE"
        print_info "Added $INSTALL_DIR to PATH in $PROFILE_FILE"
        print_warn "Please restart your terminal or run: source $PROFILE_FILE"
    else
        print_warn "Could not find profile file. Please manually add to PATH:"
        print_info "export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
else
    print_info "$INSTALL_DIR is already in PATH"
fi

echo ""
print_info "Installation complete!"
echo ""
echo "You can now use 'zonary' command:"
echo "  zonary --help"
echo "  zonary -v"
echo "  zonary example.zonary"
echo ""
echo "If 'zonary' is not found, please restart your terminal"
