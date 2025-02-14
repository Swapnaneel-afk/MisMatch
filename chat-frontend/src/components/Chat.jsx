import React, { useState, useEffect, useRef } from "react";
import {
  Box,
  IconButton,
  Typography,
  Avatar,
  useTheme,
  Button,
} from "@mui/material";
import { motion, AnimatePresence } from "framer-motion";
import DarkModeIcon from "@mui/icons-material/DarkMode";
import LightModeIcon from "@mui/icons-material/LightMode";
import SendRoundedIcon from "@mui/icons-material/SendRounded";
import { CircularProgress } from "@mui/material";
import { format } from "date-fns";

import {
  ChatContainer,
  Header,
  ChatArea,
  InputArea,
  MessageContainer,
  MessageBubble,
  StyledTextField,
  UserChip,
} from "./StyledComponents";
import RoomDialog from "./RoomDialog";
import RoomList from "./RoomList";

function Chat({ toggleTheme }) {
  // State management
  const [messages, setMessages] = useState([]);
  const [messageInput, setMessageInput] = useState("");
  const [connected, setConnected] = useState(false);
  const [username, setUsername] = useState("");
  const [onlineUsers, setOnlineUsers] = useState(new Set());
  const [typingUsers, setTypingUsers] = useState(new Set());
  const [rooms, setRooms] = useState([]);
  const [selectedRoom, setSelectedRoom] = useState(null);
  const [openNewRoomDialog, setOpenNewRoomDialog] = useState(false);
  const [hasMoreMessages, setHasMoreMessages] = useState(true);
  const [page, setPage] = useState(1);

  ////
  const [messagesByRoom, setMessagesByRoom] = useState({});
  const [typingByRoom, setTypingByRoom] = useState({});
  const [isLoadingHistory, setIsLoadingHistory] = useState(false);

  // Refs
  const wsRef = useRef(null);
  const messageAreaRef = useRef(null);
  const typingTimeoutRef = useRef({});
  const theme = useTheme();

  // Utility functions
  const formatMessageTime = (timestamp) => {
    return format(new Date(timestamp), "HH:mm");
  };
  // Add after your existing utility functions
  const fetchRoomHistory = async (roomId) => {
    setIsLoadingHistory(true);
    try {
      const response = await fetch(
        `http://localhost:8080/api/rooms/${roomId}/messages`
      );
      if (response.ok) {
        const history = await response.json();
        setMessagesByRoom((prev) => ({
          ...prev,
          [roomId]: history,
        }));
      }
    } catch (error) {
      console.error("Failed to fetch room history:", error);
    } finally {
      setIsLoadingHistory(false);
    }
  };
  const loadMoreMessages = async (roomId) => {
    if (!roomId || isLoadingHistory) return;

    setIsLoadingHistory(true);
    try {
      const oldestMessage = messagesByRoom[roomId]?.[0];
      const response = await fetch(
        `http://localhost:8080/api/rooms/${roomId}/messages?before=${oldestMessage?.timestamp}&limit=20`
      );

      if (response.ok) {
        const history = await response.json();
        if (history.length < 20) {
          setHasMoreMessages(false);
        }

        setMessagesByRoom((prev) => ({
          ...prev,
          [roomId]: [...history, ...(prev[roomId] || [])],
        }));

        // Maintain scroll position
        if (messageAreaRef.current && history.length > 0) {
          const firstNewMessage = document.getElementById(
            `message-${history[0].id}`
          );
          if (firstNewMessage) {
            firstNewMessage.scrollIntoView();
          }
        }
      }
    } catch (error) {
      console.error("Failed to load more messages:", error);
    } finally {
      setIsLoadingHistory(false);
    }
  };
  // Replace the RoomList onRoomSelect prop
  const handleRoomSelect = async (room) => {
    setSelectedRoom(room);
    setHasMoreMessages(true); // Reset pagination when switching rooms
    setPage(1);

    try {
      // Join the room
      const response = await fetch(
        `http://localhost:8080/api/rooms/${room.id}/join`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ username }),
        }
      );

      if (response.ok) {
        // Fetch room history if we haven't already
        if (!messagesByRoom[room.id]) {
          await fetchRoomHistory(room.id);
        }

        // Send join message through WebSocket
        if (wsRef.current) {
          const joinMessage = {
            message_type: "join",
            user: username,
            text: `${username} joined ${room.name}`,
            timestamp: new Date().toISOString(),
            room_id: room.id,
          };
          wsRef.current.send(JSON.stringify(joinMessage));
        }
      }
    } catch (error) {
      console.error("Failed to join room:", error);
    }
  };

  // Room loading effect
  useEffect(() => {
    const loadRooms = async () => {
      try {
        const response = await fetch("http://localhost:8080/api/rooms");
        if (response.ok) {
          const roomsList = await response.json();
          setRooms(roomsList);
          console.log("Loaded rooms:", roomsList); // Debug log
        }
      } catch (error) {
        console.error("Failed to load rooms:", error);
      }
    };

    if (connected) {
      loadRooms();
    }
  }, [connected]);

  useEffect(() => {
    if (!connected) return;

    const refreshRooms = async () => {
      try {
        const response = await fetch("http://localhost:8080/api/rooms");
        if (response.ok) {
          const roomsList = await response.json();
          setRooms(roomsList);
        }
      } catch (error) {
        console.error("Failed to refresh rooms:", error);
      }
    };

    // Refresh rooms every 30 seconds
    const intervalId = setInterval(refreshRooms, 30000);

    return () => clearInterval(intervalId);
  }, [connected]);

  // WebSocket connection effect
  useEffect(() => {
    if (!username) return;

    const refreshRooms = async () => {
      try {
        const response = await fetch("http://localhost:8080/api/rooms");
        if (response.ok) {
          const roomsList = await response.json();
          setRooms(roomsList);
        }
      } catch (error) {
        console.error("Failed to refresh rooms:", error);
      }
    };

    const WS_URL =
      process.env.NODE_ENV === "production"
        ? "wss://mismatch-production.up.railway.app"
        : "ws://127.0.0.1:8080";

    wsRef.current = new WebSocket(
      `${WS_URL}/ws?username=${encodeURIComponent(username)}`
    );

    wsRef.current.onopen = () => {
      console.log("Connected to WebSocket");
      setConnected(true);
    };

    wsRef.current.onmessage = (event) => {
      const message = JSON.parse(event.data);
      console.log("Received message:", message);

      switch (message.message_type) {
        case "new_room":
          setRooms((prev) => [...prev, message.room]);
          break;

        case "room_list":
          setRooms(message.rooms);
          break;
        case "chat":
          if (message.room_id) {
            setMessagesByRoom((prev) => ({
              ...prev,
              [message.room_id]: [...(prev[message.room_id] || []), message],
            }));

            // Auto-scroll for new messages
            if (
              messageAreaRef.current &&
              message.room_id === selectedRoom?.id
            ) {
              setTimeout(() => {
                messageAreaRef.current.scrollTop =
                  messageAreaRef.current.scrollHeight;
              }, 100);
            }
          }
          break;

        case "typing":
          if (message.room_id && message.user !== username) {
            setTypingByRoom((prev) => ({
              ...prev,
              [message.room_id]: {
                ...prev[message.room_id],
                [message.user]: true,
              },
            }));
          }
          break;

        case "stop_typing":
          if (message.room_id && message.user !== username) {
            setTypingByRoom((prev) => ({
              ...prev,
              [message.room_id]: {
                ...prev[message.room_id],
                [message.user]: false,
              },
            }));
          }
          break;

        case "join":
          setOnlineUsers((prev) => new Set([...prev, message.user]));
          break;

        case "leave":
          setOnlineUsers((prev) => {
            const newSet = new Set(prev);
            newSet.delete(message.user);
            return newSet;
          });
          break;

        case "user_list":
          if (message.users) {
            setOnlineUsers(new Set(message.users));
          }
          break;
      }
    };

    wsRef.current.onclose = () => {
      console.log("Disconnected from WebSocket");
      setConnected(false);
      setOnlineUsers(new Set());
      setTypingUsers(new Set());
    };

    wsRef.current.onerror = (error) => {
      console.error("WebSocket error:", error);
    };

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [username]);
  // Handlers
  const handleCreateRoom = async (roomData) => {
    try {
      console.log("Creating room with data:", roomData);

      const response = await fetch("http://localhost:8080/api/rooms", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          name: roomData.name,
          room_type: roomData.room_type,
          password: roomData.password,
          created_by: username,
        }),
      });

      const responseText = await response.text();
      console.log("Server response:", responseText);

      if (!response.ok) {
        console.error("Failed to create room:", response.status, responseText);
        return;
      }

      const newRoom = JSON.parse(responseText);
      console.log("Room created:", newRoom);

      // Send new room notification through WebSocket
      if (wsRef.current) {
        const roomNotification = {
          message_type: "new_room",
          room: newRoom,
        };
        wsRef.current.send(JSON.stringify(roomNotification));
      }

      setRooms((prev) => [...prev, newRoom]);
      setSelectedRoom(newRoom);
      setOpenNewRoomDialog(false);
    } catch (error) {
      console.error("Failed to create room:", error);
    }
  };

  const handleTyping = () => {
    if (wsRef.current) {
      const typingMessage = {
        message_type: "typing",
        user: username,
        text: "",
        timestamp: new Date().toISOString(),
        room_id: selectedRoom?.id,
      };

      wsRef.current.send(JSON.stringify(typingMessage));

      if (typingTimeoutRef.current[username]) {
        clearTimeout(typingTimeoutRef.current[username]);
      }

      typingTimeoutRef.current[username] = setTimeout(() => {
        if (wsRef.current) {
          const stopTypingMessage = {
            message_type: "stop_typing",
            user: username,
            text: "",
            timestamp: new Date().toISOString(),
            room_id: selectedRoom?.id,
          };
          wsRef.current.send(JSON.stringify(stopTypingMessage));
        }
      }, 1000);
    }
  };

  const sendMessage = (e) => {
    e.preventDefault();
    if (!messageInput.trim() || !wsRef.current || !selectedRoom) return;

    const message = {
      message_type: "chat",
      user: username,
      text: messageInput.trim(),
      timestamp: new Date().toISOString(),
      room_id: selectedRoom.id,
    };

    wsRef.current.send(JSON.stringify(message));
    setMessageInput("");
  };

  // Login screen
  if (!username) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <Box
          sx={{
            height: "100vh",
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            background:
              theme.palette.mode === "dark"
                ? "linear-gradient(145deg, #000000 0%, #1a1a1a 100%)"
                : "linear-gradient(145deg, #f6f6f6 0%, #ffffff 100%)",
          }}
        >
          <Box
            component={motion.div}
            whileHover={{ scale: 1.02 }}
            sx={{
              p: 4,
              borderRadius: 4,
              width: "100%",
              maxWidth: 400,
              backdropFilter: "blur(10px)",
              backgroundColor:
                theme.palette.mode === "dark"
                  ? "rgba(0, 0, 0, 0.8)"
                  : "rgba(255, 255, 255, 0.8)",
              boxShadow: "0 8px 32px 0 rgba(31, 38, 135, 0.37)",
            }}
          >
            <Typography
              variant="h4"
              gutterBottom
              align="center"
              sx={{ fontWeight: 600 }}
            >
              Join Chat
            </Typography>
            <form
              onSubmit={(e) => {
                e.preventDefault();
                const input = e.target.username.value.trim();
                if (input) setUsername(input);
              }}
            >
              <StyledTextField
                name="username"
                label="Username"
                variant="outlined"
                fullWidth
                margin="normal"
                autoFocus
              />
              <Button
                type="submit"
                variant="contained"
                fullWidth
                size="large"
                sx={{
                  mt: 2,
                  height: 48,
                  textTransform: "none",
                  fontSize: "1.1rem",
                }}
              >
                Join
              </Button>
            </form>
          </Box>
        </Box>
      </motion.div>
    );
  } // Main chat interface
  return (
    <ChatContainer>
      <Box sx={{ display: "flex", height: "100vh" }}>
        {/* Sidebar */}
        <RoomList
          rooms={rooms}
          selectedRoom={selectedRoom}
          onRoomSelect={setSelectedRoom}
          onCreateRoom={() => setOpenNewRoomDialog(true)}
        />

        {/* Main Chat Area */}
        <Box sx={{ flex: 1, display: "flex", flexDirection: "column" }}>
          <Header>
            <Box
              sx={{
                display: "flex",
                alignItems: "center",
                justifyContent: "space-between",
                width: "100%",
              }}
            >
              <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
                <Typography variant="h5" sx={{ fontWeight: 600 }}>
                  {selectedRoom ? selectedRoom.name : "Select a Room"}{" "}
                  {connected ? "(Connected)" : "(Disconnected)"}
                </Typography>
                <Box sx={{ display: "flex", gap: 1 }}>
                  <AnimatePresence>
                    {Array.from(onlineUsers).map((user) => (
                      <motion.div
                        key={user}
                        initial={{ scale: 0.8, opacity: 0 }}
                        animate={{ scale: 1, opacity: 1 }}
                        exit={{ scale: 0.8, opacity: 0 }}
                        transition={{ type: "spring", stiffness: 300 }}
                      >
                        <UserChip isOnline>
                          <Avatar
                            src={`https://ui-avatars.com/api/?name=${encodeURIComponent(
                              user
                            )}&background=random`}
                            sx={{ width: 24, height: 24 }}
                          />
                          <Typography variant="body2">{user}</Typography>
                        </UserChip>
                      </motion.div>
                    ))}
                  </AnimatePresence>
                </Box>
              </Box>
              <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
                <Typography variant="body2" color="text.secondary">
                  {username}
                </Typography>
                <IconButton onClick={toggleTheme} size="small">
                  {theme.palette.mode === "dark" ? (
                    <LightModeIcon />
                  ) : (
                    <DarkModeIcon />
                  )}
                </IconButton>
              </Box>
            </Box>
          </Header>

          <ChatArea ref={messageAreaRef}>
            {selectedRoom && hasMoreMessages && (
              <Box sx={{ display: "flex", justifyContent: "center", p: 2 }}>
                <Button
                  onClick={() => loadMoreMessages(selectedRoom.id)}
                  disabled={isLoadingHistory}
                  variant="outlined"
                  size="small"
                  sx={{ mb: 2 }}
                >
                  {isLoadingHistory ? "Loading..." : "Load More Messages"}
                </Button>
              </Box>
            )}
            {isLoadingHistory ? (
              <Box sx={{ display: "flex", justifyContent: "center", p: 2 }}>
                <CircularProgress size={24} />
              </Box>
            ) : (
              <AnimatePresence>
                {selectedRoom &&
                  messagesByRoom[selectedRoom.id]?.map((msg, index) => (
                    <motion.div
                      key={`${msg.room_id}-${index}`}
                      id={`message-${msg.id}`}
                      initial={{ y: 20, opacity: 0 }}
                      animate={{ y: 0, opacity: 1 }}
                      exit={{ y: -20, opacity: 0 }}
                      transition={{ type: "spring", stiffness: 500 }}
                    >
                      <MessageContainer isOwn={msg.user === username}>
                        <MessageBubble isOwn={msg.user === username}>
                          <Box
                            sx={{
                              display: "flex",
                              alignItems: "center",
                              gap: 1,
                              mb: 0.5,
                            }}
                          >
                            <Avatar
                              src={msg.avatar}
                              sx={{ width: 24, height: 24 }}
                            />
                            <Typography
                              variant="body2"
                              sx={{ fontWeight: 600 }}
                            >
                              {msg.user}
                            </Typography>
                            <Typography
                              variant="caption"
                              color="text.secondary"
                            >
                              {formatMessageTime(msg.timestamp)}
                            </Typography>
                          </Box>
                          <Typography variant="body1">{msg.text}</Typography>
                        </MessageBubble>
                      </MessageContainer>
                    </motion.div>
                  ))}
              </AnimatePresence>
            )}

            {selectedRoom && typingByRoom[selectedRoom.id] && (
              <motion.div
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: 10 }}
              >
                <Typography
                  variant="caption"
                  color="text.secondary"
                  sx={{ mt: 1, display: "block", textAlign: "center" }}
                >
                  {Object.entries(typingByRoom[selectedRoom.id])
                    .filter(([_, isTyping]) => isTyping)
                    .map(([user]) => user)
                    .join(", ")}{" "}
                  typing...
                </Typography>
              </motion.div>
            )}
          </ChatArea>

          <InputArea>
            <Box
              component="form"
              onSubmit={sendMessage}
              sx={{ display: "flex", gap: 2 }}
            >
              <StyledTextField
                fullWidth
                value={messageInput}
                onChange={(e) => {
                  setMessageInput(e.target.value);
                  handleTyping();
                }}
                placeholder={
                  selectedRoom
                    ? "Type a message..."
                    : "Select a room to start chatting"
                }
                disabled={!connected || !selectedRoom}
                InputProps={{
                  endAdornment: (
                    <IconButton
                      type="submit"
                      disabled={
                        !connected || !messageInput.trim() || !selectedRoom
                      }
                      color="primary"
                    >
                      <SendRoundedIcon />
                    </IconButton>
                  ),
                }}
              />
            </Box>
            <AnimatePresence>
              {typingUsers.size > 0 && (
                <motion.div
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: 10 }}
                >
                  <Typography
                    variant="caption"
                    color="text.secondary"
                    sx={{ mt: 1, display: "block" }}
                  >
                    {Array.from(typingUsers).join(", ")} typing...
                  </Typography>
                </motion.div>
              )}
            </AnimatePresence>
          </InputArea>
        </Box>
      </Box>

      <RoomDialog
        open={openNewRoomDialog}
        onClose={() => setOpenNewRoomDialog(false)}
        onCreate={handleCreateRoom}
      />
    </ChatContainer>
  );
}

export default Chat;
