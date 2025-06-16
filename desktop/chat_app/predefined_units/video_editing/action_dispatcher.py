import subprocess
import json
import time
import portalocker
from typing import Dict

def safe_write_json(path: str, data: Dict):
    with open(path, "w") as f:
        portalocker.lock(f, portalocker.LOCK_EX)
        json.dump(data, f)
        portalocker.unlock(f)

def safe_read_json(path: str) -> Dict:
    with open(path, "r") as f:
        portalocker.lock(f, portalocker.LOCK_SH)
        data = json.load(f)
        portalocker.unlock(f)
    return data

def run_job_with_child_action(child_action_script: str, initial_data: Dict, json_path: str):
    """Dispatch a job to a child_action subprocess and poll for completion."""
    safe_write_json(json_path, initial_data)

    subprocess.Popen(["python3", child_action_script, json_path])

    while True:
        time.sleep(1)
        try:
            job_data = safe_read_json(json_path)
        except json.JSONDecodeError:
            continue  # File may still be locked or being written

        if job_data.get("job_complete"):
            print("Parent: Job complete. Result:", job_data.get("result"))
            break
        else:
            print("Parent: Waiting...")
