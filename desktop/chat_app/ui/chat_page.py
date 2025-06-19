import json
from PySide6.QtWidgets import (
    QWidget, QVBoxLayout,QHBoxLayout, QTextEdit, QLineEdit, QPushButton, QMessageBox, QFileDialog, QLabel, QScrollArea,QSizePolicy
)

from PySide6.QtCore import Qt
from PySide6.QtCore import QUrl, QSize
from PySide6.QtWidgets import QStyle

import os
from PySide6.QtGui import QIcon

from .editor_pages import ImageEditorPage, VideoEditorPage, TextEditorPage

#from processing.processing_chat_message import proce

class ChatPage(QWidget):
    def __init__(self, navigate_to_editor):
        super().__init__()
        self.navigate_to_editor = navigate_to_editor

        # Main layout
        main_layout = QVBoxLayout(self)
        self.setLayout(main_layout)

        # Scrollable chat content
        scroll_area = QScrollArea()
        scroll_area.setWidgetResizable(True)
        main_layout.addWidget(scroll_area)

        scroll_content = QWidget()
        self.scroll_layout = QVBoxLayout(scroll_content)
        scroll_content.setLayout(self.scroll_layout)
        scroll_area.setWidget(scroll_content)

        # Chat display
        self.chat_display = QTextEdit()
        self.chat_display.setReadOnly(True)
        self.scroll_layout.addWidget(self.chat_display)

        # Message input
        self.message_input = QTextEdit()
        self.message_input.setFixedHeight(60)
        self.scroll_layout.addWidget(self.message_input)

        # Horizontal scroll area for attachments
        self.attachment_scroll = QScrollArea()
        self.attachment_scroll.setWidgetResizable(True)
        self.attachment_scroll.setFixedHeight(120)
        self.attachment_scroll.setHorizontalScrollBarPolicy(Qt.ScrollBarAlwaysOn)
        self.attachment_scroll.setVerticalScrollBarPolicy(Qt.ScrollBarAlwaysOff)

        attachment_container = QWidget()
        self.attachment_icon_layout = QHBoxLayout(attachment_container)
        self.attachment_icon_layout.setContentsMargins(4, 4, 4, 4)
        self.attachment_icon_layout.setSpacing(6)
        attachment_container.setLayout(self.attachment_icon_layout)
        attachment_container.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Fixed)

        self.attachment_scroll.setWidget(attachment_container)
        main_layout.addWidget(self.attachment_scroll)
        self.attachment_scroll.hide()  # Initially hidden

        # Send and Add buttons
        button_layout = QHBoxLayout()
        self.add_attached_item_button = QPushButton("Add")
        self.add_attached_item_button.clicked.connect(self.add_attached_item)
        button_layout.addWidget(self.add_attached_item_button)

        self.send_button = QPushButton("Send")
        self.send_button.clicked.connect(self.send_message)
        button_layout.addWidget(self.send_button)

        self.scroll_layout.addLayout(button_layout)

        # State
        self.attached_files = []
        self.attachment_widgets = []
        self.current_chat_file = None



    def load_chat_from_file(self, filepath):
        self.chat_display.clear()
        self.current_chat_file = filepath
        try:
            with open(filepath, "r") as f:
                messages = json.load(f)
                for msg in messages:
                    if isinstance(msg, str):
                        self.chat_display.append(msg)
                    elif isinstance(msg, dict):
                        if msg.get("text"):
                            self.chat_display.append(f"You: {msg['text']}")
                        for file_path in msg.get("attachments", []):
                            filename = os.path.basename(file_path)
                            self.chat_display.append(f"[Attachment: {filename}]")
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Failed to load chat: {e}")

    def create_attachment_widget(self, file_path):
        self.attachment_scroll.show()
        # Container for the attachment widget
        container = QWidget()
        container_layout = QVBoxLayout(container)  # Vertical: top row + thumbnail
        container_layout.setContentsMargins(4, 4, 4, 4)
        container_layout.setSpacing(4)

        # Top row: file label + edit + remove
        top_row = QHBoxLayout()
        top_row.setSpacing(6)

        # Filename label
        full_name = os.path.basename(file_path)
        max_length = 20
        short_name = full_name if len(full_name) <= max_length else full_name[:max_length - 3] + "..."
        file_label = QLabel(short_name)
        file_label.setToolTip(full_name)
        file_label.setAlignment(Qt.AlignCenter)
        file_label.setStyleSheet("background-color: black; color: white; padding: 2px;")
        file_label.setSizePolicy(QSizePolicy.Expanding, QSizePolicy.Preferred)
        top_row.addWidget(file_label)

        # Edit button
        edit_btn = QPushButton("✏️")
        edit_btn.setFixedSize(24, 24)
        edit_btn.clicked.connect(lambda: self.edit_attachment(container, file_path))
        top_row.addWidget(edit_btn)

        # Remove button
        remove_btn = QPushButton("❌")
        remove_btn.setFixedSize(24, 24)
        remove_btn.clicked.connect(lambda: self.remove_attachment(container, file_path))
        top_row.addWidget(remove_btn)

        # Add the top row to the main layout
        container_layout.addLayout(top_row)

        # Thumbnail or icon
        thumbnail = QLabel()
        thumbnail.setFixedSize(64, 64)

        ext = os.path.splitext(file_path)[1].lower()
        if ext in [".png", ".jpg", ".jpeg", ".bmp", ".gif"]:
            pixmap = QPixmap(file_path).scaled(64, 64, Qt.KeepAspectRatio, Qt.SmoothTransformation)
            thumbnail.setPixmap(pixmap)
        else:
            thumbnail.setPixmap(self.get_icon_for_file(file_path).pixmap(32, 32))

        thumbnail.setAlignment(Qt.AlignCenter)
        container_layout.addWidget(thumbnail)

        # Add the complete container to the attachments layout
        self.attachment_icon_layout.addWidget(container)
        self.attachment_widgets.append(container)


    def get_icon_for_file(self, file_path):
        ext = os.path.splitext(file_path)[1].lower()
        if ext in [".png", ".jpg", ".jpeg", ".gif", ".bmp"]:
            return self.style().standardIcon(QStyle.StandardPixmap.SP_FileDialogContentsView)
        elif ext in [".mp4", ".avi", ".mov", ".mkv"]:
            return self.style().standardIcon(QStyle.StandardPixmap.SP_MediaPlay)
        elif ext in [".txt", ".md", ".json", ".pdf", ".docx"]:
            return self.style().standardIcon(QStyle.StandardPixmap.SP_FileDialogListView)
        else:
            return self.style().standardIcon(QStyle.StandardPixmap.SP_FileIcon)



    def remove_attachment(self, widget, file_path):
        if file_path in self.attached_files:
            self.attached_files.remove(file_path)
        if widget in self.attachment_widgets:
            self.attachment_icon_layout.removeWidget(widget)
            widget.deleteLater()
            self.attachment_widgets.remove(widget)

        # Hide if no attachments left
        if not self.attached_files:
            self.attachment_scroll.hide()

    def edit_attachment(self, widget, file_path):
        ext = os.path.splitext(file_path)[-1].lower()
        if ext in [".png", ".jpg", ".jpeg"]:
            editor = ImageEditorPage(file_path)
        elif ext in [".mp4", ".mov"]:
            editor = VideoEditorPage(file_path)
        elif ext in [".txt", ".json", ".md"]:
            editor = TextEditorPage(file_path)
        else:
            QMessageBox.warning(self, "Unsupported", "Cannot edit this file type.")
            return

        self.navigate_to_editor(editor)



    def add_attached_item(self):
        file_path, _ = QFileDialog.getOpenFileName(
            self, "Attach File", "", "All Files (*)"
        )
        if file_path:
            self.attached_files.append(file_path)
            self.create_attachment_widget(file_path)


    def send_message(self):
        message = self.message_input.toPlainText().strip()
        attachments = self.attached_files.copy()  # Get current attachments
        self.attachment_scroll.hide()

        if not message and not attachments:
            return  # Nothing to send

        display_message = f"You: {message}" if message else "You sent attachments:"
        self.chat_display.append(display_message)

        # Show attachments in the chat
        for file_path in attachments:
            filename = os.path.basename(file_path)
            self.chat_display.append(f"[Attachment: {filename}]")

        # Clear input and attachments
        self.message_input.clear()
        self.clear_attachments()

        # Save message and attachments
        if self.current_chat_file:
            try:
                with open(self.current_chat_file, "r") as f:
                    history = json.load(f)
            except Exception:
                history = []

            history.append({
                "text": message,
                "attachments": attachments
            })

            with open(self.current_chat_file, "w") as f:
                json.dump(history, f, indent=2)


    def clear_attachments(self):
        self.attached_files.clear()
        for widget in self.attachment_widgets:
            self.attachment_icon_layout.removeWidget(widget)
            widget.deleteLater()
        self.attachment_widgets.clear()

