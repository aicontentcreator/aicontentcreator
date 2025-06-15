from action_dispatcher import run_job_with_child_action
from pathlib import Path

import argparse
import os
import sys

# Predefined allowed categories
ALLOWED_CATEGORIES = ["raw_test", "trim", "help"]

# Mapping from categories to child script paths
CATEGORY_TO_ACTION_SCRIPT = {
    "raw_test": "actions/raw_test.py",
    "trim": "actions/trim.py",
    "help": "actions/help.py",
}

def parse_args():
    parser = argparse.ArgumentParser(description="Run a categorized job on a given file path.")

    parser.add_argument(
        "category",
        choices=ALLOWED_CATEGORIES,
        help=f"Category of action. Allowed: {', '.join(ALLOWED_CATEGORIES)}"
    )

    parser.add_argument(
        "details",
        help="Command details"
    )

    parser.add_argument(
        "json_file_path",
        help="Path to the job file (can be a future file)"
    )

    args = parser.parse_args()
    #######################
    # Ensure the parent folder of json_path exists
    json_file = Path(args.json_file_path)
    json_file.parent.mkdir(parents=True, exist_ok=True)

        # Get the child script path from the category
    child_script = CATEGORY_TO_ACTION_SCRIPT.get(args.category)
    if not child_script:
        print(f"Error: No script mapped for category '{args.category}'")
        sys.exit(1)

    # Define the job
    job = {
        "input_data": args.details,
        "result": None,
        "job_complete": False
    }

    # Run the job
    run_job_with_child_action(child_script, job, args.json_file_path)

    #return args.category, args.file_path