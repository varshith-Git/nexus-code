#!/bin/sh

set -e

REPO="varshith-Git/nexus-code"
BINARY_NAME="claw"
INSTALL_DIR="/usr/local/bin"

# Detect OS and Architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

if [ "$OS" = "darwin" ]; then
    OS_ID="macos"
elif [ "$OS" = "linux" ]; then
    OS_ID="linux"
else
    echo "Unsupported Operating System: $OS"
    exit 1
fi

if [ "$ARCH" = "x86_64" ]; then
    ARCH_ID="amd64"
elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
    ARCH_ID="arm64"
else
    echo "Unsupported Architecture: $ARCH"
    exit 1
fi

ARTIFACT_NAME="${BINARY_NAME}-${OS_ID}-${ARCH_ID}"
TARBALL="${ARTIFACT_NAME}.tar.gz"

echo "Checking for latest release of ${BINARY_NAME}..."
LATEST_RELEASE_URL=$(curl -sL https://api.github.com/repos/${REPO}/releases/latest | grep "browser_download_url" | grep "${TARBALL}" | head -n 1 | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Error: Could not find a binary release for ${ARTIFACT_NAME}. Please check the repo: https://github.com/${REPO}/releases"
    exit 1
fi

echo "Downloading ${TARBALL}..."
curl -sL "${LATEST_RELEASE_URL}" -o "${TARBALL}"

echo "Extracting binary..."
tar -xzf "${TARBALL}"

echo "Installing ${BINARY_NAME} to ${INSTALL_DIR}..."
if [ -w "${INSTALL_DIR}" ]; then
    mv "${BINARY_NAME}" "${INSTALL_DIR}/"
else
    echo "Installation directory ${INSTALL_DIR} is not writable. Attempting with sudo..."
    sudo mv "${BINARY_NAME}" "${INSTALL_DIR}/"
fi

echo "Cleaning up..."
rm "${TARBALL}"

echo "Successfully installed ${BINARY_NAME}!"
${BINARY_NAME} --version
