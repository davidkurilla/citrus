# Quick Setup Script for citrus on Ubuntu
# This script is for developers to quickly set up a citrus development enviornment on Linux
# You must have 'curl' and 'git' installed on your system

echo "Setting up citrus development enviornment"

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
echo

# Clone citrus repository
git clone https://github.com/davidkurilla/citrus.git
cd citrus/

cargo build

echo "citrus development enviornment successfully configured!"