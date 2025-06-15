// src/pages/Settings.tsx
/*
import React from 'react';

const Home: React.FC = () => {
  return (
    <div className="container">
      <p>Home</p>
    </div>
  );
};

export default Home;

*/

// src/pages/Home.tsx
import { useNavigate } from "react-router-dom";
//import './Home.css'; // Import the CSS file

import VideoPlayer from "./components/examples/video_player/VideoPlayer";

import FlexibleRequest from './components/examples/flexible_request/FlexibleRequest';

import ThumbnailButton from './components/examples/thumbnail_button/ThumbnailButton';

export default function Home() {
  const navigate = useNavigate();

  const goToEditPage = () => {
    navigate("/edit_page");
  };

  return (
    <div className="page-container">
    <div>
      <h1>My Tauri Video App</h1>
      <VideoPlayer/>
    </div>

      <h1 className="home-title">Home Page</h1>
      <button className="navigate-button" onClick={goToEditPage}>
        Go to Edit Page
      </button>
      <div>
        <h1>FlexibleRequest</h1>
        <FlexibleRequest />
      </div>


      <div>
        <h1>ThumbnailButton</h1>
        <ThumbnailButton />
      </div>

    </div>


  );
}
