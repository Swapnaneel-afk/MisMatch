import React, { useState, useMemo } from "react";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import Chat from "./components/Chat";
import { lightTheme, darkTheme } from "./theme";

function App() {
  const [mode, setMode] = useState(
    () => localStorage.getItem("theme") || "light"
  );

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
      <Chat toggleTheme={toggleTheme} />
    </ThemeProvider>
  );
}

export default App;
