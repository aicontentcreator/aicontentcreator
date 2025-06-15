import sys
from PySide6.QtWidgets import QApplication
from ui.main_window import MainWindow
from theme import set_dark_palette, set_light_palette

if __name__ == "__main__":
    app = QApplication(sys.argv)
    # set_dark_palette(app)   # for dark theme
    # set_light_palette(app)  # for light theme
    set_dark_palette(app)
    window = MainWindow()
    window.show()
    sys.exit(app.exec())
