import React, { useState, useMemo, useEffect } from "react";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { Box, Typography, Button, TextField } from "@mui/material";
import Chat from "./components/Chat";
import RoomList from "./components/RoomList";
import { lightTheme, darkTheme } from "./theme";

function App() {
  const [mode, setMode] = useState(
    () => localStorage.getItem("theme") || "light"
  );
  const [username, setUsername] = useState(localStorage.getItem("username") || "");
  const [userId, setUserId] = useState(parseInt(localStorage.getItem("userId") || "0", 10) || null);
  const [selectedRoom, setSelectedRoom] = useState(null);
  const [registrationUsername, setRegistrationUsername] = useState("");
  const [error, setError] = useState("");
  const [isRegistering, setIsRegistering] = useState(false);

  const theme = useMemo(
    () => (mode === "light" ? lightTheme : darkTheme),
    [mode]
  );

  const toggleTheme = () => {
    const newMode = mode === "light" ? "dark" : "light";
    setMode(newMode);
    localStorage.setItem("theme", newMode);
  };

  const handleRegister = async (e) => {
    e.preventDefault();
    
    if (!registrationUsername.trim()) {
      setError("Username cannot be empty");
      return;
    }
    
    setIsRegistering(true);
    setError("");
    
    try {
      const response = await fetch(
        process.env.NODE_ENV === "production" 
          ? "https://mismatch-production.up.railway.app/api/users"
          : "http://localhost:8080/api/users", 
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            username: registrationUsername.trim(),
          }),
        }
      );
      
      const data = await response.json();
      
      if (data.success) {
        // Save user information to local storage
        localStorage.setItem("username", registrationUsername.trim());
        localStorage.setItem("userId", data.data.id.toString());
        
        setUsername(registrationUsername.trim());
        setUserId(data.data.id);
        setRegistrationUsername("");
      } else {
        setError(data.message || "Failed to register");
      }
    } catch (err) {
      setError("Network error when registering");
      console.error(err);
    } finally {
      setIsRegistering(false);
    }
  };

  const handleLogout = () => {
    localStorage.removeItem("username");
    localStorage.removeItem("userId");
    setUsername("");
    setUserId(null);
    setSelectedRoom(null);
  };

  const handleRoomSelect = (room) => {
    setSelectedRoom(room);
  };

  const handleBackToRooms = () => {
    setSelectedRoom(null);
  };

  // If user not registered, show registration form
  if (!username || !userId) {
    return (
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Box 
          sx={{ 
            height: "100vh", 
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            justifyContent: "center",
            p: 3,
            background:
              theme.palette.mode === "dark"
                ? "linear-gradient(145deg, #000000 0%, #1a1a1a 100%)"
                : "linear-gradient(145deg, #f6f6f6 0%, #ffffff 100%)",
          }}
        >
          <Box 
            sx={{ 
              width: "100%",
              maxWidth: 400,
              p: 4,
              borderRadius: 4,
              backdropFilter: "blur(10px)",
              backgroundColor:
                theme.palette.mode === "dark"
                  ? "rgba(0, 0, 0, 0.8)"
                  : "rgba(255, 255, 255, 0.8)",
              boxShadow: "0 8px 32px 0 rgba(31, 38, 135, 0.37)",
            }}
          >
            <Typography variant="h4" gutterBottom align="center">
              Register for MisMatch
            </Typography>
            
            {error && (
              <Typography color="error" align="center" sx={{ mb: 2 }}>
                {error}
              </Typography>
            )}
            
            <form onSubmit={handleRegister}>
              <TextField
                label="Username"
                variant="outlined"
                fullWidth
                margin="normal"
                value={registrationUsername}
                onChange={(e) => setRegistrationUsername(e.target.value)}
                disabled={isRegistering}
              />
              
              <Button
                type="submit"
                variant="contained"
                fullWidth
                size="large"
                disabled={isRegistering}
                sx={{ mt: 2, mb: 2 }}
              >
                {isRegistering ? "Registering..." : "Register"}
              </Button>
            </form>
            
            <Button
              variant="text"
              onClick={toggleTheme}
              fullWidth
            >
              Switch to {theme.palette.mode === "dark" ? "Light" : "Dark"} Mode
            </Button>
          </Box>
        </Box>
      </ThemeProvider>
    );
  }

  // If room not selected, show room list
  if (!selectedRoom) {
    return (
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Box 
          sx={{ 
            height: "100vh", 
            display: "flex",
            flexDirection: "column",
            p: 3,
            background:
              theme.palette.mode === "dark"
                ? "linear-gradient(145deg, #000000 0%, #1a1a1a 100%)"
                : "linear-gradient(145deg, #f6f6f6 0%, #ffffff 100%)",
          }}
        >
          <Box sx={{ display: "flex", justifyContent: "space-between", alignItems: "center", mb: 4 }}>
            <Typography variant="h4">MisMatch</Typography>
            <Box>
              <Button variant="text" onClick={toggleTheme} sx={{ mr: 1 }}>
                {theme.palette.mode === "dark" ? "Light Mode" : "Dark Mode"}
              </Button>
              <Button variant="outlined" color="error" onClick={handleLogout}>
                Logout
              </Button>
            </Box>
          </Box>
          
          <Box sx={{ display: "flex", alignItems: "center", mb: 2 }}>
            <Typography variant="h6">Welcome, {username}</Typography>
          </Box>
          
          <RoomList 
            username={username}
            userId={userId}
            onSelectRoom={handleRoomSelect}
          />
        </Box>
      </ThemeProvider>
    );
  }

  // Room selected, show chat
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Chat 
        toggleTheme={toggleTheme} 
        username={username}
        roomId={selectedRoom.id}
        roomName={selectedRoom.name}
        onBackToRooms={handleBackToRooms}
      />
    </ThemeProvider>
  );
}

export default App;
