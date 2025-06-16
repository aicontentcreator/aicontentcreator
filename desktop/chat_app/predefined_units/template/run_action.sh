#!/bin/bash
set -e

VENV_DIR=".venv"

if [ ! -d "$VENV_DIR" ]; then
  echo "Creating virtual environment..."
  python3 -m venv "$VENV_DIR"
fi

source "$VENV_DIR/bin/activate"

#if ! pip3 show xxxpackage > /dev/null 2>&1; then
#  echo "Installing xxxpackage..."
#  pip3 install xxxpackage
#fi

# Forward arguments to main.py
echo "Running predefined action..."
python3 main.py "$@"

deactivate
echo "Virtual environment deactivated."
