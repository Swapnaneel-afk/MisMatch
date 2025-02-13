import React, { useState } from "react";
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Button,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
} from "@mui/material";

function RoomDialog({ open, onClose, onCreate }) {
  const [roomName, setRoomName] = useState("");
  const [roomType, setRoomType] = useState("public");
  const [password, setPassword] = useState("");

  const handleCreate = () => {
    if (!roomName.trim()) return;

    const roomData = {
      name: roomName.trim(),
      room_type: roomType, // Make sure this matches exactly
      password: roomType === "protected" ? password : null,
      created_by: localStorage.getItem("username") || "anonymous",
    };

    console.log("Creating room with data:", roomData); // Debug log
    onCreate(roomData);

    // Reset form
    setRoomName("");
    setRoomType("public");
    setPassword("");
  };

  return (
    <Dialog open={open} onClose={onClose} fullWidth>
      <DialogTitle>Create New Room</DialogTitle>
      <DialogContent>
        <TextField
          autoFocus
          margin="dense"
          label="Room Name"
          fullWidth
          value={roomName}
          onChange={(e) => setRoomName(e.target.value)}
        />
        <FormControl fullWidth margin="dense">
          <InputLabel>Room Type</InputLabel>
          <Select
            value={roomType}
            onChange={(e) => setRoomType(e.target.value)}
            label="Room Type"
          >
            <MenuItem value="public">Public</MenuItem>
            <MenuItem value="private">Private</MenuItem>
            <MenuItem value="protected">Protected</MenuItem>
          </Select>
        </FormControl>
        {roomType === "protected" && (
          <TextField
            margin="dense"
            label="Password"
            type="password"
            fullWidth
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        )}
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button
          onClick={handleCreate}
          disabled={!roomName.trim()}
          variant="contained"
        >
          Create
        </Button>
      </DialogActions>
    </Dialog>
  );
}

export default RoomDialog;
