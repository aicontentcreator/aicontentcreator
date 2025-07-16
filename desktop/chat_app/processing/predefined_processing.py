
import subprocess
from pathlib import Path

# Define actions with their predefined_unit and executable path
ACTIONS = {
    "trim": {
        "predefined_unit": "content_editing",
        "path": Path("predefined/content_editing/trim.sh").resolve()
    },
    "convert_to_mp3": {
        "predefined_unit": "audio_editing",
        "path": Path("predefined/audio_editing/convert_to_mp3.sh").resolve()
    },
    "resize_image": {
        "predefined_unit": "image_editing",
        "path": Path("predefined/image_editing/resize_image.sh").resolve()
    },
    "compress_pdf": {
        "predefined_unit": "document_editing",
        "path": Path("predefined/document_editing/compress_pdf.sh").resolve()
    },
}

def predefined_processing(chat_message_string: str):
    if not chat_message_string.startswith("/"):
        print("--> Not a command. Ignored.")
        return

    command = chat_message_string[1:].split()[0]  # e.g., from "/trim video.mp4" get "trim"
    action = ACTIONS.get(command)

    if action is None:
        print(f"--> Unknown action: {command}")
        return

    script_path = action["path"]
    working_dir = script_path.parent

    try:
        print(f"--> Executing {command} from predefined_unit '{action['predefined_unit']}' at path: {script_path}")
        result = subprocess.run(
            ["bash", str(script_path)],
            cwd=str(working_dir),
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True
        )
        print("Output:\n", result.stdout)
    except Exception as e:
        print(f"--> Error executing action '{command}': {e}")


