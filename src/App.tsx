import React from 'react';
import { BrowserRouter as Router, Route, Routes, Link, useLocation } from 'react-router-dom';
import Home from './pages/home/Home';
import WalletPage from './pages/wallet_page/WalletPage';
import SettingsPage from './pages/settings_page/SettingsPage';

// Navigation Component
const NavLink: React.FC<{ to: string; children: React.ReactNode }> = ({ to, children }) => {
  const location = useLocation();
  const isActive = location.pathname === to;

  return (
    <Link
      to={to}
      className={`px-4 py-2 rounded-lg transition-colors ${
        isActive ? 'bg-gray-500 text-white' : 'text-white'
      } hover:bg-yellow-700`}
    >
      {children}
    </Link>
  );
};

const App: React.FC = () => {
  return (
    <Router>
      <div className="min-h-screen flex flex-col">
        {/* Navigation */}
        <nav className="bg-gray-800 text-white shadow-md z-20 relative">
          <div className="flex justify-center space-x-6 py-4">
            <ul className="flex space-x-24">
              <li>
                <NavLink to="/">Home</NavLink>
              </li>
              <li>
                <NavLink to="/wallet_page">Wallet</NavLink>
              </li>
              <li>
                <NavLink to="/settings_page">Settings</NavLink>
              </li>
            </ul>
          </div>
        </nav>

        {/* Main Content */}
        <div className="flex-1 overflow-hidden">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/wallet_page" element={<WalletPage />} />
            <Route path="/settings_page" element={<SettingsPage />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
};

export default App;
