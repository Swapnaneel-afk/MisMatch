import React, { useState, useEffect, useRef } from "react";
import {
  Container,
  Paper,
  TextField,
  Button,
  List,
  ListItem,
  ListItemText,
  Typography,
  Chip,
  Box,
  IconButton,
  Avatar,
  useTheme,
} from "@mui/material";
import { styled } from "@mui/material/styles";
import Brightness4Icon from "@mui/icons-material/Brightness4";
import Brightness7Icon from "@mui/icons-material/Brightness7";
import { format, isToday, isYesterday } from "date-fns";

const StyledPaper = styled(Paper)(({ theme }) => ({
  padding: "20px",
  backgroundColor: theme.palette.background.paper,
}));

const MessageArea = styled("div")({
  height: "400px",
  overflowY: "auto",
  marginBottom: "20px",
  padding: "10px",
});

const MessageItem = styled(ListItem)(({ theme, isSystem, isOwn }) => ({
  margin: "5px 0",
  padding: "10px",
  borderRadius: "8px",
  backgroundColor: isSystem
    ? theme.palette.mode === "dark"
      ? "#1a365d"
      : "#e3f2fd"
    : isOwn
    ? theme.palette.mode === "dark"
      ? "#2d4a3e"
      : "#e8f5e9"
    : theme.palette.mode === "dark"
    ? "#2d3748"
    : "#f5f5f5",
  width: "auto",
  maxWidth: "80%",
  marginLeft: isOwn ? "auto" : "0",
  marginRight: isOwn ? "0" : "auto",
}));

const StyledAvatar = styled(Avatar)({
  width: 40,
  height: 40,
  marginRight: 8,
});

const TypingIndicator = styled(Typography)(({ theme }) => ({
  padding: theme.spacing(1),
  color: theme.palette.text.secondary,
  fontStyle: "italic",
}));

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
    const date = new Date(timestamp);
    if (isToday(date)) {
      return format(date, "HH:mm");
    } else if (isYesterday(date)) {
      return "Yesterday " + format(date, "HH:mm");
    }
    return format(date, "MMM d, HH:mm");
  };

  const handleTyping = () => {
    if (wsRef.current) {
      wsRef.current.send(
        JSON.stringify({
          message_type: "typing",
          user: username,
          text: "",
          timestamp: new Date(),
        })
      );

      if (typingTimeoutRef.current[username]) {
        clearTimeout(typingTimeoutRef.current[username]);
      }

      typingTimeoutRef.current[username] = setTimeout(() => {
        if (wsRef.current) {
          wsRef.current.send(
            JSON.stringify({
              message_type: "stop_typing",
              user: username,
              text: "",
              timestamp: new Date(),
            })
          );
        }
      }, 2000);
    }
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

    wsRef.current.onclose = () => {
      console.log("Disconnected from WebSocket");
      setConnected(false);
      setOnlineUsers(new Set());
      setTypingUsers(new Set());
    };

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [username]);

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

  const renderMessage = (msg, index) => (
    <MessageItem
      key={index}
      isSystem={msg.message_type === "join" || msg.message_type === "leave"}
      isOwn={msg.user === username && msg.message_type === "chat"}
    >
      <Box sx={{ display: "flex", alignItems: "flex-start", width: "100%" }}>
        <StyledAvatar
          src={`https://ui-avatars.com/api/?name=${encodeURIComponent(
            msg.user
          )}&background=random`}
          alt={msg.user}
        >
          {msg.user[0].toUpperCase()}
        </StyledAvatar>
        <Box sx={{ flex: 1 }}>
          <ListItemText
            primary={
              <Box sx={{ display: "flex", justifyContent: "space-between" }}>
                <Typography
                  component="span"
                  color={msg.user === username ? "primary" : "textPrimary"}
                  variant="body2"
                  sx={{ fontWeight: "bold" }}
                >
                  {msg.user}
                </Typography>
                <Typography
                  variant="caption"
                  color="textSecondary"
                  sx={{ ml: 2 }}
                >
                  {formatMessageTime(msg.timestamp)}
                </Typography>
              </Box>
            }
            secondary={
              <Typography component="span" variant="body2" color="textPrimary">
                {msg.text}
              </Typography>
            }
          />
        </Box>
      </Box>
    </MessageItem>
  );

  const renderHeader = () => (
    <Box
      sx={{
        display: "flex",
        justifyContent: "space-between",
        alignItems: "center",
        mb: 2,
      }}
    >
      <Typography variant="h5">
        Chat Room {connected ? "(Connected)" : "(Disconnected)"}
      </Typography>
      <Box sx={{ display: "flex", alignItems: "center", gap: 2 }}>
        <IconButton onClick={toggleTheme} color="inherit">
          {theme.palette.mode === "dark" ? (
            <Brightness7Icon />
          ) : (
            <Brightness4Icon />
          )}
        </IconButton>
        <Typography variant="body2" color="textSecondary">
          Logged in as: {username}
        </Typography>
      </Box>
    </Box>
  );

  const renderTypingIndicator = () => {
    const typingUsersArray = Array.from(typingUsers).filter(
      (user) => user !== username
    );
    if (typingUsersArray.length === 0) return null;

    const typingText =
      typingUsersArray.length === 1
        ? `${typingUsersArray[0]} is typing...`
        : `${typingUsersArray.join(", ")} are typing...`;

    return <TypingIndicator>{typingText}</TypingIndicator>;
  };

  if (!username) {
    return (
      <Container maxWidth="sm" sx={{ mt: 5 }}>
        <StyledPaper>
          <Typography variant="h5" gutterBottom>
            Enter your username
          </Typography>
          <form
            onSubmit={(e) => {
              e.preventDefault();
              const input = e.target.username.value.trim();
              if (input) setUsername(input);
            }}
          >
            <TextField
              name="username"
              label="Username"
              variant="outlined"
              fullWidth
              margin="normal"
              autoFocus
            />
            <Button type="submit" variant="contained" color="primary" fullWidth>
              Join Chat
            </Button>
          </form>
        </StyledPaper>
      </Container>
    );
  }

  return (
    <Container maxWidth="md" sx={{ mt: 5 }}>
      <StyledPaper>
        {renderHeader()}

        <Paper sx={{ p: 1, mb: 2, display: "flex", flexWrap: "wrap", gap: 1 }}>
          {Array.from(onlineUsers).map((user) => (
            <Chip
              key={user}
              avatar={
                <Avatar
                  src={`https://ui-avatars.com/api/?name=${encodeURIComponent(
                    user
                  )}&background=random`}
                >
                  {user[0].toUpperCase()}
                </Avatar>
              }
              label={user}
              color={user === username ? "primary" : "default"}
              variant={user === username ? "filled" : "outlined"}
            />
          ))}
        </Paper>

        <MessageArea ref={messageAreaRef}>
          <List>{messages.map((msg, index) => renderMessage(msg, index))}</List>
        </MessageArea>

        {renderTypingIndicator()}

        <form onSubmit={sendMessage}>
          <TextField
            value={messageInput}
            onChange={(e) => {
              setMessageInput(e.target.value);
              handleTyping();
            }}
            label="Type a message"
            variant="outlined"
            fullWidth
            disabled={!connected}
            sx={{ mb: 2 }}
          />
          <Button
            type="submit"
            variant="contained"
            color="primary"
            fullWidth
            disabled={!connected || !messageInput.trim()}
          >
            Send
          </Button>
        </form>
      </StyledPaper>
    </Container>
  );
}

export default Chat;
