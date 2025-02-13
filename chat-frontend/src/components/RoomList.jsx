import React from "react";
import {
  Box,
  List,
  ListItem,
  ListItemText,
  Button,
  Typography,
  Paper,
} from "@mui/material";
import LockIcon from "@mui/icons-material/Lock";
import PublicIcon from "@mui/icons-material/Public";
import VpnLockIcon from "@mui/icons-material/VpnLock";

function RoomList({ rooms, selectedRoom, onRoomSelect, onCreateRoom }) {
  return (
    <Paper
      sx={{
        width: 240,
        borderRight: 1,
        borderColor: "divider",
        display: "flex",
        flexDirection: "column",
      }}
    >
      <Box sx={{ p: 2 }}>
        <Button variant="contained" fullWidth onClick={onCreateRoom}>
          Create Room
        </Button>
      </Box>
      <List sx={{ flex: 1, overflow: "auto" }}>
        {rooms.map((room) => (
          <ListItem
            button
            key={room.id}
            selected={selectedRoom?.id === room.id}
            onClick={() => onRoomSelect(room)}
          >
            <ListItemText
              primary={
                <Box sx={{ display: "flex", alignItems: "center", gap: 1 }}>
                  {room.type === "public" && <PublicIcon fontSize="small" />}
                  {room.type === "private" && <LockIcon fontSize="small" />}
                  {room.type === "protected" && (
                    <VpnLockIcon fontSize="small" />
                  )}
                  <Typography variant="body1">{room.name}</Typography>
                </Box>
              }
              secondary={room.type}
            />
          </ListItem>
        ))}
      </List>
    </Paper>
  );
}

export default RoomList;
