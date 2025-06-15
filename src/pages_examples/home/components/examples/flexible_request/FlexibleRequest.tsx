// src/components/FlexibleRequest.tsx
import { useState } from 'react';
import { callBackend, BackendResponse } from '../../../../../lib/backend.ts';

export default function FlexibleRequest() {
  const [name, setName] = useState('');
  const [response, setResponse] = useState<BackendResponse<{ additional_text: string }> | null>(null);

  const sendRequest = async () => {
    const res = await callBackend<{ additional_text: string }>('sayHello', { name });
    setResponse(res);
  };

  return (
    <div style={{ padding: '1rem' }}>
      <input
        value={name}
        onChange={(e) => setName(e.target.value)}
        placeholder="Enter your name"
        style={{ marginRight: '0.5rem' }}
      />
      <button onClick={sendRequest}>Send</button>

      {response && (
        <div style={{ marginTop: '1rem' }}>
          <strong>Status:</strong> {response.status}
          <br />
          <strong>additional_text:</strong> {response.payload.additional_text}
        </div>
      )}
    </div>
  );
}
