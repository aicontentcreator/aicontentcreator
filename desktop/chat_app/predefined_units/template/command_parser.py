from pathlib import Path

import argparse
import time
#import os
#import sys

# Predefined allowed categories
ALLOWED_CATEGORIES = ["raw_test", "trim", "help"]

# Mapping from categories to child script paths
#CATEGORY_TO_ACTION_SCRIPT = {
#    "raw_test": "actions/raw_test.py",
#    "trim": "actions/trim.py",
#    "help": "actions/help.py",
#}

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

    args = parser.parse_args()

    # Run the job
    #run_job_with_child_action(child_script, job, args.json_file_path)
    print("running",args.category,args.details)

    for i in range(4):
        print(f"Running {i + 1}")
        time.sleep(1)

    #return args.category, args.file_path