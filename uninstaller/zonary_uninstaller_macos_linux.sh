#!/bin/bash
# Zonary Interpreter Uninstaller for macOS/Linux
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
    echo -e "${BLUE}Zonary Interpreter Uninstaller v${VERSION}${NC}"
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

# Check if Zonary is installed
FOUND=false
BINARY_PATH=""

# Check default location
if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    BINARY_PATH="$INSTALL_DIR/$BINARY_NAME"
    FOUND=true
    print_info "Found Zonary at: $BINARY_PATH"
fi

# Check if it's in PATH
if [ "$FOUND" = false ]; then
    WHICH_PATH=$(which zonary 2>/dev/null || echo "")
    if [ -n "$WHICH_PATH" ]; then
        BINARY_PATH="$WHICH_PATH"
        FOUND=true
        print_info "Found Zonary at: $BINARY_PATH"
    fi
fi

# If not found, ask user
if [ "$FOUND" = false ]; then
    print_warn "Zonary not found in default location ($INSTALL_DIR) or PATH"
    echo ""
    read -p "Enter the full path to the zonary binary (or press Enter to skip): " CUSTOM_PATH

    if [ -z "$CUSTOM_PATH" ]; then
        print_info "No action taken"
        exit 0
    fi

    if [ -f "$CUSTOM_PATH" ]; then
        BINARY_PATH="$CUSTOM_PATH"
        FOUND=true
        print_info "Using custom path: $BINARY_PATH"
    else
        print_error "File not found: $CUSTOM_PATH"
        exit 1
    fi
fi

if [ "$FOUND" = false ]; then
    print_info "Zonary is not installed"
    exit 0
fi

# Confirm uninstall
echo ""
echo -e "${YELLOW}Warning: This will remove: $BINARY_PATH${NC}"
echo ""
read -p "Are you sure you want to uninstall? (y/N): " CONFIRM

if [[ ! "$CONFIRM" =~ ^[Yy]$ ]]; then
    print_info "Uninstall cancelled"
    exit 0
fi

# Remove binary
print_info "Removing $BINARY_PATH"
sudo rm -f "$BINARY_PATH"

if [ $? -eq 0 ]; then
    print_info "Binary removed successfully"
else
    print_error "Failed to remove binary"
    print_info "Try running with sudo: sudo ./uninstall.sh"
    exit 1
fi

# Option to remove from PATH in profile files
echo ""
read -p "Do you also want to remove Zonary entries from your shell profile files? (y/N): " REMOVE_PROFILE

if [[ "$REMOVE_PROFILE" =~ ^[Yy]$ ]]; then
    print_info "Checking profile files..."

    PROFILE_FILES=("$HOME/.bashrc" "$HOME/.bash_profile" "$HOME/.zshrc" "$HOME/.profile")

    for FILE in "${PROFILE_FILES[@]}"; do
        if [ -f "$FILE" ]; then
            if grep -q "Zonary\|zonary" "$FILE" 2>/dev/null; then
                cp "$FILE" "$FILE.bak"
                print_info "Backup created: $FILE.bak"

                sed -i.bak '/# Zonary Interpreter/d' "$FILE"
                sed -i.bak '/zonary/d' "$FILE"
                sed -i.bak '/ZONARY/d' "$FILE"

                print_info "Removed Zonary entries from $FILE"
            fi
        fi
    done

    print_warn "Please restart your terminal or run: source ~/.bashrc"
fi

# Check if directory is empty and ask to remove
BIN_DIR=$(dirname "$BINARY_PATH")
if [ -d "$BIN_DIR" ] && [ -z "$(ls -A "$BIN_DIR" 2>/dev/null)" ]; then
    echo ""
    read -p "The directory $BIN_DIR is now empty. Do you want to remove it? (y/N): " REMOVE_DIR
    if [[ "$REMOVE_DIR" =~ ^[Yy]$ ]]; then
        sudo rmdir "$BIN_DIR" 2>/dev/null
        if [ $? -eq 0 ]; then
            print_info "Removed directory: $BIN_DIR"
        else
            print_warn "Could not remove directory: $BIN_DIR"
        fi
    fi
fi

echo ""
print_info "Uninstall complete!"
echo ""
echo "Zonary has been removed from your system"
