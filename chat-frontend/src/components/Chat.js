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
} from "@mui/material";
import { styled } from "@mui/material/styles";

const StyledPaper = styled(Paper)(({ theme }) => ({
  padding: "20px",
}));

const MessageArea = styled("div")({
  height: "400px",
  overflowY: "auto",
  marginBottom: "20px",
});

const MessageInput = styled(TextField)({
  width: "100%",
  marginBottom: "10px",
});

const Message = styled(ListItem)({
  margin: "10px 0",
  padding: "10px",
  borderRadius: "5px",
  backgroundColor: "#f5f5f5",
});

function Chat() {
  const [messages, setMessages] = useState([]);
  const [messageInput, setMessageInput] = useState("");
  const [connected, setConnected] = useState(false);
  const [username, setUsername] = useState("");
  const wsRef = useRef(null);
  const messageAreaRef = useRef(null);

  useEffect(() => {
    if (!username) return;

    wsRef.current = new WebSocket("ws://127.0.0.1:8080/ws");

    wsRef.current.onopen = () => {
      console.log("Connected to WebSocket");
      setConnected(true);
    };

    wsRef.current.onmessage = (event) => {
      const message = JSON.parse(event.data);
      setMessages((prev) => [...prev, message]);

      // Auto-scroll to bottom
      if (messageAreaRef.current) {
        messageAreaRef.current.scrollTop = messageAreaRef.current.scrollHeight;
      }
    };

    wsRef.current.onclose = () => {
      console.log("Disconnected from WebSocket");
      setConnected(false);
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
      user: username,
      text: messageInput.trim(),
    };

    wsRef.current.send(JSON.stringify(message));
    setMessageInput("");
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
        <Typography variant="h5" gutterBottom>
          Chat Room ({connected ? "Connected" : "Disconnected"})
        </Typography>
        <MessageArea ref={messageAreaRef}>
          <List>
            {messages.map((msg, index) => (
              <Message key={index}>
                <ListItemText primary={msg.user} secondary={msg.text} />
              </Message>
            ))}
          </List>
        </MessageArea>
        <form onSubmit={sendMessage}>
          <MessageInput
            value={messageInput}
            onChange={(e) => setMessageInput(e.target.value)}
            label="Type a message"
            variant="outlined"
            disabled={!connected}
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
