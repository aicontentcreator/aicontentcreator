import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type BackendResponse = {
  status: string;
  description: string;
  data?: any;
};

export default function DaemonOperationalSituationDisplay() {
  const [name, setName] = useState("");
  const [result, setResult] = useState<string | null>(null);
  const [daemonStatus, setDaemonStatus] = useState<string | null>(null);
/*
  const greet = async () => {
    try {
      const response: BackendResponse = await invoke("handle_frontend_request", {
        frontendRequest: {
          action: "greet",
          payload: { name },
        },
      });
      setResult(response.description);
    } catch (error) {
      console.error("Error invoking greet:", error);
    }
  };
*/
  const calculate = async () => {
    try {
      const response: BackendResponse = await invoke("handle_frontend_request", {
        frontendRequest: {
          action: "calculate",
          payload: { a: 5, b: 10 },
        },
      });
      setResult(`Result: ${response.data?.result}`);
    } catch (error) {
      console.error("Error invoking calculate:", error);
    }
  };
/*
  const getDaemonStatus = async () => {
    try {
      const response: BackendResponse = await invoke("handle_frontend_request", {
        frontendRequest: {
          action: "get_daemon_operational_situation",
          payload: {},
        },
      });
      setDaemonStatus(response.data?.daemon_operational_situation);
    } catch (error) {
      console.error("Error fetching daemon status:", error);
    }
  };
*/
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-900 text-white p-4">
      <h1 className="text-3xl font-bold mb-4">Tauri React App</h1>
      <input
        className="border p-2 rounded text-black"
        type="text"
        placeholder="Enter your name"
        value={name}
        onChange={(e) => setName(e.target.value)}
      />
{/*
      <button
        className="mt-2 bg-blue-500 px-4 py-2 rounded hover:bg-blue-600"
        onClick={greet}
      >
        Greet
      </button> 
*/}
      <button
        className="mt-2 bg-green-500 px-4 py-2 rounded hover:bg-green-600"
        onClick={calculate}
      >
        Calculate 5 + 10
      </button>
{/*
      <button
        className="mt-2 bg-yellow-500 px-4 py-2 rounded hover:bg-yellow-600"
        onClick={getDaemonStatus}
      >
        Get Daemon Status
      </button>
*/}
      
      {daemonStatus && <p className="mt-4 text-lg">Daemon Status: {daemonStatus}</p>}



{result && <p className="mt-4 text-lg">{result}</p>}
    </div>
  );
}


/*
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
//
interface BackendResponse {
  status: "success" | "error";  // Either "success" or "error"
  description: string;          // Description of the response
  data?: { [key: string]: any }; // Optional data (can be any JSON)
}/

export default function DaemonOperationalSituationDisplay() {
  const [daemon_operational_situation, setDaemonOperationalSituation] = useState<string>(
    localStorage.getItem("saved_daemon_operational_situation") || "Fetching..."
  );

  useEffect(() => {
    // Function to fetch daemon_operational_situation from Rust backend
    //
    const fetchDaemonOperationalSituation = () => {
      invoke("handle_frontend_request", {
        frontendRequest: { action: "get_daemon_operational_situation_string", payload: {} },
      })
        .then(() => console.log("DaemonOperationalSituation request sent"))
        .catch((error) => console.error("Invoke error:", error));
    };
    

    //
    const fetchDaemonOperationalSituation = async () => {
      try {
        console.log("📡 Requesting daemon_operational_situation from backend...");
        const response = await invoke<BackendResponse>("handle_frontend_request", {
          frontendRequest: { action: "get_daemon_operational_situation_string", payload: {} },
        });
        
        console.log("🎯 Received response:", response);
        if (response.status === "success" && response.data?.daemon_operational_situation) {
          setDaemonOperationalSituation(response.data.daemon_operational_situation);
          localStorage.setItem("saved_daemon_operational_situation", response.data.daemon_operational_situation);
        }
      } catch (error) {
        console.error("❌ Invoke error:", error);
      }
    };
    //

    // 🔹 Immediately request daemon_operational_situation when component loads
    fetchDaemonOperationalSituation();

    // 🔹 Set up event listener for backend response
    const unlistenPromise = listen("backend_response", (event: any) => {
      console.log("Received event:", event.payload);
      if (event.payload.status === "success" && event.payload.data?.daemon_operational_situation) {
        const newDaemonOperationalSituation = event.payload.data.daemon_operational_situation;
        setDaemonOperationalSituation(newDaemonOperationalSituation);
        localStorage.setItem("saved_daemon_operational_situation", newDaemonOperationalSituation); // ✅ Persist daemon_operational_situation
      }
    });

    // 🔹 Fetch daemon_operational_situation every 5 seconds
    const interval = setInterval(fetchDaemonOperationalSituation, 5000);

    return () => {
      clearInterval(interval);
      unlistenPromise.then((unlisten) => unlisten()); // Cleanup listener
    };
  }, []);

  return (
    <div>
      <p className="mt-2 text-lg font-mono">Current DaemonOperationalSituation: {daemon_operational_situation}</p>
    </div>
  );
}
*/
/*
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

export default function GetDaemonOperationalSituationString() {
  const [greeting, setGreeting] = useState<string>("");
  const [result, setResult] = useState<number | null>(null);
  const [daemon_operational_situation, setDaemonOperationalSituation] = useState<string>("");

  useEffect(() => {
    const unlisten = listen("backend_response", (event: any) => {
      console.log("Received event:", event.payload);
      if (event.payload.status === "success") {
        if (event.payload.description) {
          setGreeting(event.payload.description);
        }
        if (event.payload.data?.result) {
          setResult(event.payload.data.result);
        }
        if (event.payload.data?.daemon_operational_situation) {
          setDaemonOperationalSituation(event.payload.data.daemon_operational_situation);
        }
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    const interval = setInterval(() => {
      invoke("handle_frontend_request", {
        frontendRequest: { action: "get_daemon_operational_situation_string", payload: {} },
      });
    }, 5000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-100">
      <h1 className="text-2xl font-bold mb-4">Tauri + React + Rust</h1>

      <button
        onClick={() =>
          invoke("handle_frontend_request", {
            frontendRequest: { action: "greet", payload: { name: "Alice" } },
          })
        }
        className="px-4 py-2 bg-blue-500 text-white rounded-lg mb-2"
      >
        Send Greeting
      </button>

      <button
        onClick={() =>
          invoke("handle_frontend_request", {
            frontendRequest: { action: "calculate", payload: { a: 5, b: 10 } },
          })
        }
        className="px-4 py-2 bg-green-500 text-white rounded-lg mb-2"
      >
        Calculate Sum (5 + 10)
      </button>

      {greeting && <p className="mt-4 text-lg">{greeting}</p>}
      {result !== null && <p className="mt-2 text-lg">Result: {result}</p>}
      {daemon_operational_situation && <p className="mt-2 text-lg font-mono">Current DaemonOperationalSituation: {daemon_operational_situation}</p>}
    </div>
  );
}
*/

/*
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

export default function Home() {
  const [greeting, setGreeting] = useState<string>("");
  const [result, setResult] = useState<number | null>(null);

  useEffect(() => {
    const unlisten = listen("backend_response", (event: any) => {
      console.log("Received event:", event.payload);
      if (event.payload.status === "success") {
        if (event.payload.description) {
          setGreeting(event.payload.description);
        }
        if (event.payload.data?.result) {
          setResult(event.payload.data.result);
        }
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const sendGreeting = async () => {
    await invoke("handle_frontend_request", {
      frontendRequest: { action: "greet", payload: { name: "Alice" } },
    });
  };

  const calculateSum = async () => {
    await invoke("handle_frontend_request", {
      frontendRequest: { action: "calculate", payload: { a: 5, b: 10 } },
    });
  };

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-100">
      <h1 className="text-2xl font-bold mb-4">Tauri + React + Rust</h1>

      <button
        onClick={sendGreeting}
        className="px-4 py-2 bg-blue-500 text-white rounded-lg mb-2"
      >
        Send Greeting
      </button>

      <button
        onClick={calculateSum}
        className="px-4 py-2 bg-green-500 text-white rounded-lg mb-2"
      >
        Calculate Sum (5 + 10)
      </button>

      {greeting && <p className="mt-4 text-lg">{greeting}</p>}
      {result !== null && <p className="mt-2 text-lg">Result: {result}</p>}
    </div>
  );
}
*/