# editor_pages.py

from PySide6.QtWidgets import QWidget, QVBoxLayout, QLabel, QTextEdit
from PySide6.QtGui import QPixmap
from PySide6.QtCore import Qt
import os

class ImageEditorPage(QWidget):
    def __init__(self, file_path):
        super().__init__()
        self.setLayout(QVBoxLayout())
        self.layout().addWidget(QLabel("Image Editor"))

        pixmap = QPixmap(file_path)
        img_label = QLabel()
        img_label.setPixmap(pixmap.scaled(400, 400, Qt.KeepAspectRatio))
        self.layout().addWidget(img_label)

class VideoEditorPage(QWidget):
    def __init__(self, file_path):
        super().__init__()
        self.setLayout(QVBoxLayout())
        self.layout().addWidget(QLabel("Video Editor"))
        self.layout().addWidget(QLabel(f"Video file: {os.path.basename(file_path)}"))
        # Placeholder — add player later

class TextEditorPage(QWidget):
    def __init__(self, file_path):
        super().__init__()
        self.setLayout(QVBoxLayout())
        self.layout().addWidget(QLabel("Text Editor"))

        self.editor = QTextEdit()
        with open(file_path, "r") as f:
            self.editor.setText(f.read())
        self.layout().addWidget(self.editor)
