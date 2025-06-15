
import React from 'react';
import { BrowserRouter as Router, Route, Routes, Link, useLocation } from 'react-router-dom';
import Home from './pages_examples/home/Home';
import EditPage from './pages/edit_page/EditPage';

import FullImagePage from "./pages_examples/full_image_page/FullImagePage";

import NewChatPage from './pages/new_chat_page/NewChatPage';
import SettingsPage from './pages/settings_page/SettingsPage';



import './App.css'; // Import the custom CSS

// Navigation Component
const NavLink: React.FC<{ to: string; children: React.ReactNode }> = ({ to, children }) => {
  const location = useLocation();
  const isActive = location.pathname === to;

  return (
    <Link
      to={to}
      className={`nav-link ${isActive ? 'active' : ''}`}
    >
      {children}
    </Link>
  );
};

const App: React.FC = () => {
  return (
    <Router>
      <div className="app-container">
        {/* Navigation */}
        <nav className="navbar">
          <div className="navbar-inner">
            <ul className="nav-list">
              <li>
                <NavLink to="/">Home</NavLink>
              </li>
              <li>
                <NavLink to="/new_chat_page">New Chat</NavLink>
              </li>
              <li>
                <NavLink to="/settings_page">Settings</NavLink>
              </li>
            </ul>
          </div>
        </nav>

        {/* Main Content */}
        <div className="main-content">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/edit_page" element={<EditPage />} />
            <Route path="/new_chat_page" element={<NewChatPage />} />
            <Route path="/settings_page" element={<SettingsPage />} />
            <Route path="/view/:imageId" element={<FullImagePage />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;

/*
import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>


      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;

*/
