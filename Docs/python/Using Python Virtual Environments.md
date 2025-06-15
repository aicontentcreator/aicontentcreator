# Using Python Virtual Environments (venv)

Here's a complete guide to using Python's built-in virtual environment tool:

## Basic Usage

### 1. Create a virtual environment
```
python3 -m venv myproject_env
```

### 2. Activate the environment

**On Linux/macOS:**
```
source myproject_env/bin/activate
```

**On Windows:**
```
myproject_env\Scripts\activate
```

Your prompt will change to show the active environment: `(myproject_env) $`

### 3. Install packages
```
pip install package-name
```

### 4. Deactivate when finished
```
deactivate
```

## Advanced Usage

### Creating with specific options
```
python3 -m venv --system-site-packages myenv  # Include system packages
python3 -m venv --without-pip myenv           # Create without pip
python3 -m venv --copies myenv                # Use copies instead of symlinks
```

### Managing dependencies
```
# Save requirements
pip freeze > requirements.txt

# Install from requirements
pip install -r requirements.txt
```

### Upgrading pip in the environment
```
pip install --upgrade pip
```

### Deleting a virtual environment
Simply delete the environment directory:
```
rm -rf myproject_env  # Linux/macOS
rmdir /s /q myproject_env  # Windows
```

## Project Workflow Example

```bash
# Start a new project
mkdir my_project
cd my_project

# Create and activate environment
python3 -m venv .my_project_venv
source .my_project_venv/bin/activate  # or .venv\Scripts\activate on Windows

# Install dependencies
pip install numpy pandas matplotlib

# Do your work...

# Save environment for others
pip freeze > requirements.txt

# When done
deactivate
```

Virtual environments keep your Python projects isolated, making dependency management much easier and preventing conflicts between different projects.
