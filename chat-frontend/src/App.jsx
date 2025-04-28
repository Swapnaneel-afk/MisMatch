import React, { useState, useMemo, useEffect } from "react";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import Chat from "./components/Chat";
import Debug from "./components/Debug";
import { lightTheme, darkTheme } from "./theme";

function App() {
  const [mode, setMode] = useState(
    () => localStorage.getItem("theme") || "light"
  );
  const [page, setPage] = useState("chat");

  // Simple routing based on URL
  useEffect(() => {
    const path = window.location.pathname;
    if (path.includes("debug")) {
      setPage("debug");
    } else {
      setPage("chat");
    }

    // Log app initialization for debugging
    console.log("App initialized. Environment:", process.env.NODE_ENV);
    console.log("WebSocket URL:", process.env.REACT_APP_WS_URL || "not set");
  }, []);

  const theme = useMemo(
    () => (mode === "light" ? lightTheme : darkTheme),
    [mode]
  );

  const toggleTheme = () => {
    const newMode = mode === "light" ? "dark" : "light";
    setMode(newMode);
    localStorage.setItem("theme", newMode);
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      {page === "debug" ? (
        <Debug />
      ) : (
        <Chat toggleTheme={toggleTheme} />
      )}
    </ThemeProvider>
  );
}

export default App;
