import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface BackendResponse {
  status: string;
  description: string;
  data?: {
    daemon_operational_situation?: any; // Can be string, number, or object
  };
}

const DaemonOperationalSituationDisplay = () => {
  const [daemonStatus, setDaemonOperationalSituation] = useState<string>("Loading...");

  const getDaemonOperationalSituation = async () => {
    try {
      const response: BackendResponse = await invoke("handle_frontend_request", {
        frontendRequest: {
          action: "get_daemon_operational_situation",
          payload: {},
        },
      });

      console.log("Daemon Response:", JSON.stringify(response, null, 2)); // Debugging

      const daemonSituation = response.data?.daemon_operational_situation;

        setDaemonOperationalSituation(daemonSituation.data.daemon_operational_situation); // 
    } catch (error) {
      console.error("Error fetching daemon status:", error);
      setDaemonOperationalSituation("Error fetching daemon status");
    }
  };
  /*
  useEffect(() => {
    getDaemonOperationalSituation();
  }, []);
  */
  useEffect(() => {
    getDaemonOperationalSituation(); // Fetch on mount

    const interval = setInterval(() => {
      getDaemonOperationalSituation(); // Refresh every 5 seconds
    }, 5000);

    return () => clearInterval(interval); // Cleanup on unmount
  }, []);

  return (
    <div className="text-black p-4">
      {/*
      <h2 className="text-lg font-semibold mb-2">Daemon Status</h2>
      */}
      
      <pre className="text-black-400 whitespace-pre-wrap">{daemonStatus}</pre> {/* ✅ Displays objects correctly */}
        {/*
          <button
            onClick={getDaemonOperationalSituation}
            className="mt-4 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded-lg"
          >
            Refresh
          </button>
        */}

    </div>
  );
};

export default DaemonOperationalSituationDisplay;
