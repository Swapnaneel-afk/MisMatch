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

function Chat({ toggleTheme }) {
  const [messages, setMessages] = useState([]);
  const [messageInput, setMessageInput] = useState("");
  const [connected, setConnected] = useState(false);
  const [username, setUsername] = useState("");
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
    if (!username) return;

    wsRef.current = new WebSocket(
      `ws://127.0.0.1:8080/ws?username=${encodeURIComponent(username)}`
    );

    wsRef.current.onopen = () => {
      console.log("Connected to WebSocket");
      setConnected(true);
    };

    wsRef.current.onmessage = (event) => {
      const message = JSON.parse(event.data);
      console.log("Received message:", message);

      switch (message.message_type) {
        case "typing":
          setTypingUsers((prev) => new Set([...prev, message.user]));
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
        default:
          setMessages((prev) => [...prev, message]);
      }

      if (messageAreaRef.current) {
        messageAreaRef.current.scrollTop = messageAreaRef.current.scrollHeight;
      }
    };

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [username]);

  const handleTyping = () => {
    if (wsRef.current) {
      const typingMessage = {
        message_type: "typing",
        user: username,
        text: "",
        timestamp: new Date().toISOString(),
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
    };

    wsRef.current.send(JSON.stringify(message));
    setMessageInput("");
  };

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
  }

  return (
    <ChatContainer>
      <Header>
        <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
          <Typography variant="h5" sx={{ fontWeight: 600 }}>
            Chat Room {connected ? "(Connected)" : "(Disconnected)"}
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
                    <Avatar src={msg.avatar} sx={{ width: 24, height: 24 }} />
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
