import subprocess
from pathlib import Path

from PySide6.QtCore import QThread, Signal


def simple_test():
    print("Simple Test")

def run_subprocess(script_path_str, arg1, arg2):
    script_path = Path(script_path_str)
    try:
        result = subprocess.run(
            [str(script_path), arg1, arg2],
            check=True,
            capture_output=True,
            text=True
        )
        print("STDOUT:\n", result.stdout)
        print("STDERR:\n", result.stderr)
    except subprocess.CalledProcessError as e:
        print("Script failed with return code", e.returncode)
        print("Error output:\n", e.stderr)

class JobRunner(QThread):
    job_done = Signal(str)

    def __init__(self)#, script_path_str,arg1, arg2):
        super().__init__()
        #self.script_path_str = script_path_str
        #self.arg1=arg1
        #self.arg2=arg2

    def run(self):
        run_subprocess("predefined/basic/run_action.sh","raw_test","")
        self.job_done.emit("done")