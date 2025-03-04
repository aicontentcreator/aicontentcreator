

import React from "react";
import DaemonOperationalSituationDisplay from "./components/daemon_operational_situation_display/DaemonOperationalSituationDisplay";

const Home: React.FC = () => {
  return (
    <div className="p-4 flex flex-col items-left">      
      {/* Include GetTimeString Component */}
      <DaemonOperationalSituationDisplay />
    </div>
  );
};

export default Home;