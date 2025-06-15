// src/components/FlexibleRequest.tsx
//import { useState } from 'react';
import { useState, useEffect } from "react";
import { callBackend } from '../../../../lib/backend.ts';

export default function Testcount() {
  /*
  const [name, setName] = useState('');
  const [response, setResponse] = useState<BackendResponse<{ additional_text: string }> | null>(null);

  const sendRequest = async () => {
    const res = await callBackend<{ additional_text: string }>('sayHello', { name });
    setResponse(res);
  };
  */
  const [testcount, setTestcount] = useState<number>(0);

  //
  useEffect(() => {
    const interval = setInterval(() => {

      
      updateChatsCount();

    }, 2000);

    return () => clearInterval(interval);
  }, []);
  //
  async function updateChatsCount() {
    try {
      const newres =await callBackend<{ chat_messages_count: number }>('getChatMessagesCountFromBackend', {  chat_id: '**'  });
      setTestcount(newres.payload.chat_messages_count);
    } catch (error) {
      console.error("Failed to send message:", error);
    }
  }
  return (
    <div >
        <p>testcount: {testcount}</p>
    </div>

  );
}
