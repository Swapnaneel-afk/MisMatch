import React, { useState, useEffect } from 'react';
import {
  Box,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  ListItemAvatar,
  Avatar,
  Typography,
  Button,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Chip
} from '@mui/material';
import GroupIcon from '@mui/icons-material/Group';
import LockIcon from '@mui/icons-material/Lock';
import AddIcon from '@mui/icons-material/Add';

const API_URL = process.env.NODE_ENV === 'production' 
  ? 'https://mismatch-production.up.railway.app/api'
  : 'http://localhost:8080/api';

function RoomList({ username, userId, onSelectRoom }) {
  const [rooms, setRooms] = useState([]);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [joinDialogOpen, setJoinDialogOpen] = useState(false);
  const [selectedRoom, setSelectedRoom] = useState(null);
  const [newRoomName, setNewRoomName] = useState('');
  const [newRoomType, setNewRoomType] = useState('public');
  const [newRoomPassword, setNewRoomPassword] = useState('');
  const [joinPassword, setJoinPassword] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    fetchRooms();
  }, []);

  const fetchRooms = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`${API_URL}/rooms`);
      const data = await response.json();
      
      if (data.success) {
        setRooms(data.data || []);
      } else {
        setError(data.message || 'Failed to fetch rooms');
      }
    } catch (err) {
      setError('Network error when fetching rooms');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCreateRoom = async () => {
    if (!newRoomName.trim()) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`${API_URL}/rooms/create/${userId}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          name: newRoomName.trim(),
          room_type: newRoomType,
          password: newRoomType === 'protected' ? newRoomPassword : null
        })
      });
      
      const data = await response.json();
      
      if (data.success) {
        setCreateDialogOpen(false);
        setNewRoomName('');
        setNewRoomType('public');
        setNewRoomPassword('');
        fetchRooms();
        // Select the newly created room
        if (data.data && data.data.id) {
          onSelectRoom(data.data);
        }
      } else {
        setError(data.message || 'Failed to create room');
      }
    } catch (err) {
      setError('Network error when creating room');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const handleJoinRoom = async () => {
    if (!selectedRoom) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`${API_URL}/rooms/join`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          user_id: userId,
          room_id: selectedRoom.id,
          password: selectedRoom.room_type === 'protected' ? joinPassword : null
        })
      });
      
      const data = await response.json();
      
      if (data.success) {
        setJoinDialogOpen(false);
        setJoinPassword('');
        onSelectRoom(selectedRoom);
      } else {
        setError(data.message || 'Failed to join room');
      }
    } catch (err) {
      setError('Network error when joining room');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  const openJoinDialog = (room) => {
    setSelectedRoom(room);
    if (room.room_type === 'protected') {
      setJoinDialogOpen(true);
    } else {
      // For public rooms, join directly
      handleRoomSelect(room);
    }
  };

  const handleRoomSelect = async (room) => {
    // First try to join the room
    setIsLoading(true);
    setError(null);
    
    try {
      const response = await fetch(`${API_URL}/rooms/join`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          user_id: userId,
          room_id: room.id
        })
      });
      
      const data = await response.json();
      
      if (data.success) {
        onSelectRoom(room);
      } else {
        setError(data.message || 'Failed to join room');
      }
    } catch (err) {
      setError('Network error when joining room');
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Box sx={{ width: '100%', maxWidth: 360, bgcolor: 'background.paper' }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', p: 2 }}>
        <Typography variant="h6">Chat Rooms</Typography>
        <Button 
          variant="contained" 
          size="small" 
          startIcon={<AddIcon />}
          onClick={() => setCreateDialogOpen(true)}
        >
          New
        </Button>
      </Box>
      
      {error && (
        <Box sx={{ p: 2, color: 'error.main' }}>
          <Typography variant="body2">{error}</Typography>
        </Box>
      )}
      
      <List sx={{ maxHeight: 300, overflow: 'auto' }}>
        {rooms.map((room) => (
          <ListItem key={room.id} disablePadding>
            <ListItemButton onClick={() => openJoinDialog(room)}>
              <ListItemAvatar>
                <Avatar>
                  {room.room_type === 'protected' ? <LockIcon /> : <GroupIcon />}
                </Avatar>
              </ListItemAvatar>
              <ListItemText 
                primary={room.name} 
                secondary={
                  <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
                    <Chip 
                      label={room.room_type} 
                      size="small" 
                      color={room.room_type === 'public' ? 'success' : 'warning'} 
                      variant="outlined"
                    />
                  </Box>
                }
              />
            </ListItemButton>
          </ListItem>
        ))}
        {rooms.length === 0 && !isLoading && (
          <Box sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="body2" color="text.secondary">
              No rooms available. Create one!
            </Typography>
          </Box>
        )}
        {isLoading && (
          <Box sx={{ p: 2, textAlign: 'center' }}>
            <Typography variant="body2" color="text.secondary">
              Loading rooms...
            </Typography>
          </Box>
        )}
      </List>
      
      {/* Create Room Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)}>
        <DialogTitle>Create New Room</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Room Name"
            type="text"
            fullWidth
            variant="outlined"
            value={newRoomName}
            onChange={(e) => setNewRoomName(e.target.value)}
          />
          <FormControl fullWidth sx={{ mt: 2 }}>
            <InputLabel>Room Type</InputLabel>
            <Select
              value={newRoomType}
              label="Room Type"
              onChange={(e) => setNewRoomType(e.target.value)}
            >
              <MenuItem value="public">Public</MenuItem>
              <MenuItem value="protected">Protected (Password)</MenuItem>
            </Select>
          </FormControl>
          
          {newRoomType === 'protected' && (
            <TextField
              margin="dense"
              label="Password"
              type="password"
              fullWidth
              variant="outlined"
              value={newRoomPassword}
              onChange={(e) => setNewRoomPassword(e.target.value)}
            />
          )}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleCreateRoom} 
            variant="contained"
            disabled={!newRoomName.trim() || (newRoomType === 'protected' && !newRoomPassword)}
          >
            Create
          </Button>
        </DialogActions>
      </Dialog>
      
      {/* Join Protected Room Dialog */}
      <Dialog open={joinDialogOpen} onClose={() => setJoinDialogOpen(false)}>
        <DialogTitle>Enter Room Password</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Password"
            type="password"
            fullWidth
            variant="outlined"
            value={joinPassword}
            onChange={(e) => setJoinPassword(e.target.value)}
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setJoinDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleJoinRoom} 
            variant="contained"
            disabled={!joinPassword}
          >
            Join
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}

export default RoomList; 