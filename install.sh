#!/bin/bash

# Configuration
APP_NAME="pass-gen"
BINARY_NAME="pass-gen"
DISPLAY_NAME="Password Generator"
COMMENT="A simple and secure password generator"
ICON_NAME="security-high" # Using a standard system icon

# Directory setup
BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"

echo "Installing $DISPLAY_NAME..."

# Create directories if they don't exist
mkdir -p "$BIN_DIR"
mkdir -p "$APP_DIR"

# Build the application
echo "Building release binary..."
cargo build --release

# Copy binary
echo "Installing binary to $BIN_DIR..."
cp "target/release/$BINARY_NAME" "$BIN_DIR/$BINARY_NAME"
chmod +x "$BIN_DIR/$BINARY_NAME"

# Create .desktop file
echo "Creating desktop entry in $APP_DIR..."
cat > "$APP_DIR/$APP_NAME.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=$DISPLAY_NAME
Comment=$COMMENT
Exec=$BIN_DIR/$BINARY_NAME
Icon=$ICON_NAME
Terminal=false
Categories=Utility;Security;
EOF

echo "Installation complete!"
echo "You may need to restart your desktop environment or log out/in for the application to appear in the menu."
