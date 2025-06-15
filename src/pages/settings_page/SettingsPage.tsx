// src/pages/Settings.tsx
import DisplaySettings from './components/display_settings/DisplaySettings';
import React from 'react';

const SettingsPage: React.FC = () => {
  return (
    <div className="page-container">
      <p>SettingsPage</p>
      <DisplaySettings />
    </div>
  );
};

export default SettingsPage;
