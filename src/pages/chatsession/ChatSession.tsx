import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';


const ChatSession: React.FC = () => {
  const [messages, setMessages] = useState<string[]>([]);
  const [input, setInput] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const NAV_HEIGHT = 60; // Adjust based on the navigation bar height
  const INPUT_CONTAINER_HEIGHT = 150; // Adjust based on the input container height

  const sendMessage = async () => {
    if (input.trim()) {
      const response = await invoke<string>('send_message', { message: input });
      setMessages([...messages, response]);
      setInput('');
    }
  };

  useEffect(() => {
    // Scroll to the bottom whenever messages change
    if (messagesEndRef.current) {
      messagesEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [messages]);

  return (
    <div className="flex flex-col bg-gray-100">
      {/* Messages container with dynamic height */}
      <div
        className={`overflow-y-auto p-4 bg-white shadow-lg rounded m-4`}
        style={{
          height: `calc(100vh - ${NAV_HEIGHT + INPUT_CONTAINER_HEIGHT+20}px)`,
        }}
      >
        {messages.length === 0 ? (
          <div className="text-center text-gray-500">No messages yet</div>
        ) : (
          messages.map((msg, index) => (
            <div key={index} className="mb-2 text-left text-gray-800">
              {msg}
            </div>
          ))
        )}
        {/* Ref for automatically scrolling to the bottom */}
        <div ref={messagesEndRef} />
      </div>

      {/* Input container stays at the bottom */}
      <div
        className="bg-white shadow-lg w-full p-4 border-t fixed bottom-0 left-0 z-10"
        style={{ height: `${INPUT_CONTAINER_HEIGHT}px` }}
      >
        <div className="flex justify-center items-center">
          <div className="flex w-3/4 max-w-2xl items-start"> {/* Align items to the start */}
            <textarea
              className="flex-1 p-2 border border-gray-300 rounded-l"
              value={input}
              onChange={(e) => setInput(e.target.value)}
              placeholder="Type your message..."
            />
            <button
              className="px-4 py-2 bg-blue-500 text-white rounded-r h-auto" // Remove flex-1 or any flex properties
              onClick={sendMessage}
            >
              Send
            </button>
          </div>
        </div>
      </div>


    </div>
  );
};

export default ChatSession;

