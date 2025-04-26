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
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
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

function Chat({ toggleTheme, username, roomId, roomName, onBackToRooms }) {
  const [messages, setMessages] = useState([]);
  const [messageInput, setMessageInput] = useState("");
  const [connected, setConnected] = useState(false);
  const [onlineUsers, setOnlineUsers] = useState(new Set());
  const [typingUsers, setTypingUsers] = useState(new Set());
  const wsRef = useRef(null);
  const messageAreaRef = useRef(null);
  const typingTimeoutRef = useRef({});
  const theme = useTheme();

  const formatMessageTime = (timestamp) => {
    return format(new Date(timestamp), "HH:mm");
  };

  useEffect(() => {
    if (!username || !roomId) return;

    const WS_URL =
      process.env.NODE_ENV === "production"
        ? "wss://mismatch-production.up.railway.app"
        : "ws://127.0.0.1:8080";

    // Connect to WebSocket with username and roomId
    wsRef.current = new WebSocket(
      `${WS_URL}/ws?username=${encodeURIComponent(username)}&roomId=${roomId}`
    );

    wsRef.current.onopen = () => {
      console.log("Connected to WebSocket");
      setConnected(true);
    };

    wsRef.current.onmessage = (event) => {
      const message = JSON.parse(event.data);
      console.log("Received message:", message);

      switch (message.message_type) {
        case "user_list": // Handle the new UserList message type
          if (message.users) {
            setOnlineUsers(new Set(message.users));
          }
          break;
        case "typing":
          // Only add typing indicator if it's not the current user
          if (message.user !== username) {
            setTypingUsers((prev) => new Set([...prev, message.user]));
          }
          break;
        case "stop_typing":
          setTypingUsers((prev) => {
            const newSet = new Set(prev);
            newSet.delete(message.user);
            return newSet;
          });
          break;
        case "join":
          setOnlineUsers((prev) => new Set([...prev, message.user]));
          setMessages((prev) => [...prev, message]);
          break;
        case "leave":
          setOnlineUsers((prev) => {
            const newSet = new Set(prev);
            newSet.delete(message.user);
            return newSet;
          });
          setMessages((prev) => [...prev, message]);
          break;
        case "chat":
          setMessages((prev) => [...prev, message]);
          break;
        default:
          console.log("Unknown message type:", message.message_type);
      }

      if (messageAreaRef.current) {
        messageAreaRef.current.scrollTop = messageAreaRef.current.scrollHeight;
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

    // Fetch previous messages for this room
    const fetchMessages = async () => {
      try {
        const API_URL = process.env.NODE_ENV === "production"
          ? "https://mismatch-production.up.railway.app/api"
          : "http://localhost:8080/api";
        
        const response = await fetch(`${API_URL}/rooms/${roomId}/messages`);
        const data = await response.json();
        
        if (data.success && data.data) {
          setMessages(data.data.map(msg => ({
            message_type: "chat",
            user: msg.sender_username || "Unknown",
            text: msg.content,
            timestamp: msg.created_at
          })));
        }
      } catch (err) {
        console.error("Error fetching messages:", err);
      }
    };
    
    fetchMessages();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [username, roomId]);

  const handleTyping = () => {
    if (wsRef.current) {
      const typingMessage = {
        message_type: "typing",
        user: username,
        text: "",
        timestamp: new Date().toISOString(),
        room_id: roomId
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
            room_id: roomId
          };
          wsRef.current.send(JSON.stringify(stopTypingMessage));
        }
      }, 1000);
    }
  };

  const sendMessage = (e) => {
    e.preventDefault();
    if (!messageInput.trim() || !wsRef.current) return;

    const message = {
      message_type: "chat",
      user: username,
      text: messageInput.trim(),
      timestamp: new Date().toISOString(),
      room_id: roomId
    };

    wsRef.current.send(JSON.stringify(message));
    setMessageInput("");
  };

  return (
    <ChatContainer>
      <Header>
        <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
          <IconButton onClick={onBackToRooms} size="small">
            <ArrowBackIcon />
          </IconButton>
          <Typography variant="h5" sx={{ fontWeight: 600 }}>
            {roomName} {connected ? "(Connected)" : "(Disconnected)"}
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
      </Header>

      <ChatArea ref={messageAreaRef}>
        <AnimatePresence>
          {messages.map((msg, index) => (
            <motion.div
              key={index}
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
                      src={`https://ui-avatars.com/api/?name=${encodeURIComponent(
                        msg.user
                      )}&background=random`} 
                      sx={{ width: 24, height: 24 }} 
                    />
                    <Typography variant="body2" sx={{ fontWeight: 600 }}>
                      {msg.user}
                    </Typography>
                    <Typography variant="caption" color="text.secondary">
                      {formatMessageTime(msg.timestamp)}
                    </Typography>
                  </Box>
                  <Typography variant="body1">{msg.text}</Typography>
                </MessageBubble>
              </MessageContainer>
            </motion.div>
          ))}
        </AnimatePresence>
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
            placeholder="Type a message..."
            disabled={!connected}
            InputProps={{
              endAdornment: (
                <IconButton
                  type="submit"
                  disabled={!connected || !messageInput.trim()}
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
    </ChatContainer>
  );
}

export default Chat;
