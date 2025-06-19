import os, uuid, json

from PySide6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QHBoxLayout, QPushButton, QStackedWidget,QScrollArea
)
from .chat_page import ChatPage
from constants import CHAT_DIR

from PySide6.QtCore import QThread, Signal
import sys
import subprocess
from pathlib import Path


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("JSON-Based Chat Loader")
        self.setGeometry(100, 100, 600, 500)
        #label = QLabel("This is a dark mode window", self)
        #label.setStyleSheet("font-size: 16px; padding: 20px;")

        os.makedirs(CHAT_DIR, exist_ok=True)

        # Central layout
        central_widget = QWidget()
        self.central_layout = QVBoxLayout()
        central_widget.setLayout(self.central_layout)
        self.setCentralWidget(central_widget)

        # Nav bar
        nav_bar = QHBoxLayout()
        home_button = QPushButton("Home")
        chat_button = QPushButton("Chat")
        #settings_button = QPushButton("Settings")
        nav_bar.addWidget(home_button)
        nav_bar.addWidget(chat_button)
        #nav_bar.addWidget(settings_button)
        self.central_layout.addLayout(nav_bar)

        # Pages
        self.pages = QStackedWidget()
        self.central_layout.addWidget(self.pages)

        # --- Home Page ---
        self.home_page = QWidget()
        self.home_layout = QVBoxLayout()
        self.home_page.setLayout(self.home_layout)

        # Scrollable area for chat buttons
        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        scroll_widget = QWidget()
        self.chat_list_layout = QVBoxLayout(scroll_widget)
        scroll_widget.setLayout(self.chat_list_layout)
        scroll_area.setWidget(scroll_widget)

        # Add scroll area and new chat button to home layout
        self.new_chat_button = QPushButton("New Chat")
        self.new_chat_button.clicked.connect(self.create_new_chat)
        #
        self.new_test_button = QPushButton("New Test")
        self.new_test_button.clicked.connect(self.run_subprocess)
        #
        self.home_layout.addWidget(self.new_chat_button)
        self.home_layout.addWidget(self.new_test_button)
        self.home_layout.addWidget(scroll_area)
        

        # --- Chat Page ---
        #self.chat_page = ChatPage()
        self.chat_page = ChatPage(self.open_editor_page)
        self.pages.addWidget(self.chat_page)
        # Add pages to stacked widget
        self.pages.addWidget(self.home_page)  # index 0
        self.pages.addWidget(self.chat_page)  # index 1

        # Navigation button connections
        home_button.clicked.connect(lambda: self.pages.setCurrentIndex(0))
        chat_button.clicked.connect(lambda: self.pages.setCurrentIndex(1))
        #settings_button.clicked.connect(lambda: self.pages.setCurrentIndex(2))

        self.load_chat_buttons()

    def open_editor_page(self, page_widget):
        self.pages.addWidget(page_widget)
        self.pages.setCurrentWidget(page_widget)

    def load_chat_buttons(self):
        for i in reversed(range(self.chat_list_layout.count())):
            widget = self.chat_list_layout.itemAt(i).widget()
            if widget:
                widget.setParent(None)

        for filename in sorted(os.listdir(CHAT_DIR)):
            if filename.endswith(".json"):
                filepath = os.path.join(CHAT_DIR, filename)
                btn = QPushButton(filename)
                btn.clicked.connect(lambda checked, f=filepath: self.open_chat(f))
                self.chat_list_layout.addWidget(btn)

    def create_new_chat(self):
        chat_id = str(uuid.uuid4())[:8]
        filepath = os.path.join(CHAT_DIR, f"chat-{chat_id}.json")
        with open(filepath, "w") as f:
            json.dump([], f)
        self.load_chat_buttons()
        self.open_chat(filepath)

    def open_chat(self, filepath):
        self.chat_page.load_chat_from_file(filepath)
        self.pages.setCurrentIndex(1)

    ###########################"""
    def run_subprocess(self):
        self.worker = SubprocessWorker()
        self.worker.output_ready.connect(self.display_output)
        self.worker.start()

    def display_output(self, text):
        #self.output.append(text)
        print("output",text)
#######################""
class SubprocessWorker(QThread):
    output_ready = Signal(str)

    def run(self):
        try:
            script_path = Path("predefined_units/template/run_action.sh").resolve()
            working_dir = script_path.parent
            arg1="raw_test"
            arg2="."
            # Example command (replace with your script/command)
            result = subprocess.run(
                ["bash", str(script_path),arg1,arg2],
                cwd=str(working_dir),  # ⬅️ set working directory for script
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                check=False
            )
            self.output_ready.emit(result.stdout)
        except Exception as e:
            self.output_ready.emit(f"Error: {e}")
