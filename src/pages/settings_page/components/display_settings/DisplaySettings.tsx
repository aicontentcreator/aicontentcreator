// src/components/DisplaySettings.tsx
import { useEffect, useState } from 'react';
import { callBackend } from '../../../../lib/backend';

type Settings = {
  theme: string;
  notifications: boolean;
  version: string;
};

export default function DisplaySettings() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadSettings() {
      const res = await callBackend<Settings>('requestSettings', {});
      if (res.status === 'success') {
        setSettings(res.payload);
      } else {
        setError( res.status_description);
      }
    }

    loadSettings();
  }, []);

  if (error) return <p style={{ color: 'red' }}>Error: {error}</p>;
  if (!settings) return <p>Loading settings...</p>;

  return (
    <div style={{ padding: '1rem' }}>
      <h2>App Settings</h2>
      <ul>
        <li><strong>Theme:</strong> {settings.theme}</li>
        <li><strong>Notifications:</strong> {settings.notifications ? 'Enabled' : 'Disabled'}</li>
        <li><strong>Version:</strong> {settings.version}</li>
      </ul>
    </div>
  );
}
