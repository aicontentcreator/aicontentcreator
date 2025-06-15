// src/App.tsx
import { useState, useEffect } from 'react';
import { path } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/core';
import './EditPage.css';

function EditPage() {
  const [videoUrl, setVideoUrl] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  useEffect(() => {
    async function loadVideo() {
      try {
        // Get the app data directory
        const dataDir = await path.appDataDir();
        
        // Assuming video file is named "video.mp4" in the app data directory
        // You should replace this with your actual filename
        const videoPath = `${dataDir}/etam.mp4`;
        
        // Convert the path to a URL that can be used by the video element
        const convertedPath = convertFileSrc(videoPath);
        setVideoUrl(convertedPath);
        setLoading(false);
      } catch (err) {
        setError(`Error loading video: ${err instanceof Error ? err.message : String(err)}`);
        setLoading(false);
      }
    }
    
    loadVideo();
  }, []);
  
  return (
    <div className="page-container">
      <h1>Tauri Video Player</h1>
      
      {loading && <div className="loading">Loading video...</div>}
      
      {error && <div className="error">{error}</div>}
      
      {videoUrl && (
        <div className="video-container">
          <video 
            src={videoUrl} 
            controls 
            autoPlay 
            className="video-player"
          >
            Your browser does not support the video tag.
          </video>
        </div>
      )}
    </div>
  );
}

export default EditPage;