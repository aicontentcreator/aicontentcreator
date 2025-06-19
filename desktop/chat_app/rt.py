
from processing.predefined_processing import predefined_processing

# Example usage
if __name__ == "__main__":
    predefined_processing("/trim")
    predefined_processing("/resize_image")
    predefined_processing("hello world")  # ignored
    predefined_processing("/unknown_action")  # unknown