from PySide6.QtGui import QPalette, QColor
from PySide6.QtCore import Qt

# === Semantic Brand Colors ===
SPIRIT_COLOR = QColor(0, 127, 255)       # Blue – Trust, clarity, intelligence
ENERGY_COLOR = QColor(242, 245, 16)      # Yellow – Attention, energy, innovation

# === Neutral Background Colors ===
DARK_BACKGROUND = QColor(130, 130, 130)
LIGHT_BACKGROUND = QColor(245, 245, 245)
DARK_BASE = QColor(25, 25, 25)
LIGHT_BASE = QColor(255, 255, 255)
DARK_ALT_BASE = QColor(40, 40, 40)
LIGHT_ALT_BASE = QColor(230, 230, 230)
DARK_BUTTON = QColor(50, 50, 50)
LIGHT_BUTTON = QColor(230, 230, 230)


def set_dark_palette(app):
    palette = QPalette()

    palette.setColor(QPalette.Window, DARK_BACKGROUND)
    palette.setColor(QPalette.Base, DARK_BASE)
    palette.setColor(QPalette.AlternateBase, DARK_ALT_BASE)
    palette.setColor(QPalette.Text, Qt.white)
    palette.setColor(QPalette.WindowText, Qt.white)
    palette.setColor(QPalette.ToolTipBase, Qt.white)
    palette.setColor(QPalette.ToolTipText, Qt.black)

    palette.setColor(QPalette.Button, DARK_BUTTON)
    palette.setColor(QPalette.ButtonText, Qt.white)
    palette.setColor(QPalette.Link, SPIRIT_COLOR)
    palette.setColor(QPalette.Highlight, SPIRIT_COLOR)
    palette.setColor(QPalette.HighlightedText, Qt.white)
    palette.setColor(QPalette.BrightText, ENERGY_COLOR)

    app.setPalette(palette)
    app.setStyle("Fusion")


def set_light_palette(app):
    palette = QPalette()

    palette.setColor(QPalette.Window, LIGHT_BACKGROUND)
    palette.setColor(QPalette.Base, LIGHT_BASE)
    palette.setColor(QPalette.AlternateBase, LIGHT_ALT_BASE)
    palette.setColor(QPalette.Text, Qt.black)
    palette.setColor(QPalette.WindowText, Qt.black)
    palette.setColor(QPalette.ToolTipBase, Qt.black)
    palette.setColor(QPalette.ToolTipText, Qt.white)

    palette.setColor(QPalette.Button, LIGHT_BUTTON)
    palette.setColor(QPalette.ButtonText, Qt.black)
    palette.setColor(QPalette.Link, SPIRIT_COLOR)
    palette.setColor(QPalette.Highlight, SPIRIT_COLOR)
    palette.setColor(QPalette.HighlightedText, Qt.white)
    palette.setColor(QPalette.BrightText, ENERGY_COLOR)

    app.setPalette(palette)
    app.setStyle("Fusion")
