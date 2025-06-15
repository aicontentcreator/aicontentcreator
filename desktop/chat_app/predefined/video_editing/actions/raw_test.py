import json
import sys
import time
import portalocker

json_path = sys.argv[1]

def safe_read_json(path):
    with open(path, "r") as f:
        portalocker.lock(f, portalocker.LOCK_SH)
        data = json.load(f)
        portalocker.unlock(f)
    return data

def safe_write_json(path, data):
    with open(path, "w") as f:
        portalocker.lock(f, portalocker.LOCK_EX)
        json.dump(data, f)
        portalocker.unlock(f)

# Step 1: Read input
job_data = safe_read_json(json_path)

# Step 2: Do work
print("Child: Working...")
time.sleep(5)

# Step 3: Update result and flag complete
job_data["result"] = job_data["input_data"].upper()
job_data["job_complete"] = True
safe_write_json(json_path, job_data)

print("Child: Done.")

