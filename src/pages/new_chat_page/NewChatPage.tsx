import { useState, useEffect, useRef } from "react";
//import { invoke } from "@tauri-apps/api/core";
import { callBackend} from '../../lib/backend.ts';

import "./NewChatPage.css";
import Testcount from "./components/testcount/Testcount.tsx";
import EditItemModal from "./components/edit_item_modal/EditItemModal";

function NewChatPage() {

  type ChatMessage = {
    sender: "user" | "bot";
    text: string;
    image?: string; // base64 or URL
  };
  
  const [messages, setChatMessages] = useState<ChatMessage[]>([]);

  const [input, setInput] = useState("");
  const lastMessageRef = useRef<HTMLDivElement | null>(null);


  //

  const [modalEditItem, setModalEditItem] = useState<string | null>(null); // Modal state

  async function sendChatMessage() {
    if (!input.trim()) return;
    const testImage="https://upload.wikimedia.org/wikipedia/en/9/94/The_Matrix.jpg";
    const userMessage: ChatMessage = { sender: "user", text: input,image:testImage };
    setChatMessages((prev) => [...prev, userMessage]);

    try {


      const res = await callBackend<{ response_text: string }>('sendChatMessageToBackend', {  chat_message_text: input  });
     
      //
      const mockImage = "https://upload.wikimedia.org/wikipedia/commons/thumb/9/95/EvanRachelWood.jpg/800px-EvanRachelWood.jpg";

      const botMessage: ChatMessage = {
        sender: "bot",
        text: res.payload.response_text,//responseText,
        image: mockImage,
      };

      setChatMessages((prev) => [...prev, botMessage]);
      //
      } catch (error) {
        console.error("Failed to send message:", error);
      }

      setInput("");
  }


  // Scroll to the last message every time messages update
  useEffect(() => {
    lastMessageRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  return (
    <main className="page-container">


      <div className="chat-box">


        {messages.map((msg, index) => (
          <div
            key={index}
            className={`message ${msg.sender === "user" ? "user" : "bot"}`}
          >
            <p>{msg.text}</p>
            {/* msg.image && <img src={msg.image} alt="chat-img" className="chat-img" /> */}

            {msg.image && (
              <div className="attacheditem-wrapper">
                {/*<img src={msg.image} alt="attacheditem-img" className="attacheditem-img" /> */}
                <img
                  src={msg.image}
                  alt="attacheditem-img"
                  className="attacheditem-img"
                  onClick={() => setModalEditItem(msg.image ?? null)}
                  style={{ cursor: 'pointer' }}
                />
                <div className="attacheditem-buttons-group">
                  <button className="attacheditem-button">Reply</button>
                  {/* <button className="attacheditem-button">Forward</button> */}
                </div>
              </div>
            )}

          </div>
        ))}

        {/* Invisible div at the bottom to scroll into view */}
        <div ref={lastMessageRef} />
      </div>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          sendChatMessage();
        }}
      >
      <textarea
        value={input}
        onChange={(e) => setInput(e.currentTarget.value)}
        placeholder="Type a message..."
        rows={2}
        onKeyDown={(e) => {
          if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            sendChatMessage();
          }
        }}
      />

        <button type="submit">Send</button>
      </form>

      <div>
        <Testcount />
      </div>
      {/* Modal */}
      {modalEditItem && (
        <EditItemModal imageUrl={modalEditItem} onClose={() => setModalEditItem(null)} />
      )}
    </main>
  );
}

export default NewChatPage;

