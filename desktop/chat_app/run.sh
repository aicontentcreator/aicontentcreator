#!/bin/bash

# Exit immediately if a command fails
set -e

# Define the virtual environment directory
VENV_DIR=".venv"

# Create virtual environment if it doesn't exist
if [ ! -d "$VENV_DIR" ]; then
  echo "Creating virtual environment..."
  python3 -m venv "$VENV_DIR"
fi

# Activate the virtual environment
source "$VENV_DIR/bin/activate"

# Install PySide6 if not already installed
if ! pip3 show PySide6 > /dev/null 2>&1; then
  echo "Installing PySide6..."
  pip3 install PySide6
fi

# Run the application
echo "Running chat app..."
python3 main.py

# Deactivate the virtual environment after the app exits
deactivate
echo "Virtual environment deactivated."
