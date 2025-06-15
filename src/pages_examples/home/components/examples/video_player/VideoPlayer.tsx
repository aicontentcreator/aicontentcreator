import React from "react";

const VideoPlayer: React.FC = () => {
  return (
    <div className="video-container">
      <video controls width="100%" preload="metadata">
        <source src="http://localhost:3000/cool.mp4" type="video/mp4" />
        Your browser does not support the video tag.
      </video>
    </div>
  );
};

export default VideoPlayer;
