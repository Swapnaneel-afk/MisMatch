import React, { useState, useEffect } from 'react';
import { 
  IconButton, 
  Typography, 
  Button, 
  Box, 
  TextField, 
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  List,
  ListItem,
  ListItemText,
  ListItemIcon,
  ListItemSecondaryAction,
  Tooltip,
  Divider
} from '@mui/material';
import {
  Close as CloseIcon,
  Add as AddIcon,
  People as PeopleIcon,
  Delete as DeleteIcon,
  Edit as EditIcon,
  ArrowForward as JoinIcon
} from '@mui/icons-material';
import { 
  SidebarContainer, 
  SidebarHeader, 
  RoomList, 
  RoomItem, 
  ModalContent,
  RoomsList,
  RoomName,
  ActionButtons
} from './StyledComponents';
import CreateRoomButton from './CreateRoomButton';

const RoomSidebar = ({ 
  isOpen, 
  onClose, 
  rooms = [], 
  selectedRoom, 
  onSelectRoom, 
  onDeleteRoom,
  onEditRoom,
  onCreateRoom,
  onJoinRoom
}) => {
  const [localRooms, setLocalRooms] = useState(rooms);
  const [isCreateDialogOpen, setCreateDialogOpen] = useState(false);
  const [isEditDialogOpen, setEditDialogOpen] = useState(false);
  const [newRoomName, setNewRoomName] = useState('');
  const [editingRoom, setEditingRoom] = useState(null);
  const [error, setError] = useState('');

  useEffect(() => {
    setLocalRooms(rooms);
  }, [rooms]);

  const handleRoomClick = (roomId) => {
    if (onSelectRoom) {
      onSelectRoom(roomId);
    }
  };

  const handleDeleteClick = (e, roomId) => {
    e.stopPropagation();
    if (onDeleteRoom) {
      onDeleteRoom(roomId);
    }
  };

  const handleEditClick = (e, roomId, currentName) => {
    e.stopPropagation();
    if (onEditRoom) {
      onEditRoom(roomId, currentName);
    }
  };

  const handleCreateRoom = () => {
    if (!newRoomName.trim()) {
      setError('Room name cannot be empty');
      return;
    }

    if (newRoomName.length > 30) {
      setError('Room name must be less than 30 characters');
      return;
    }

    onCreateRoom(newRoomName.trim());
    setNewRoomName('');
    setCreateDialogOpen(false);
    setError('');
  };

  const handleOpenEditDialog = (room) => {
    setEditingRoom(room);
    setNewRoomName(room.name);
    setEditDialogOpen(true);
  };

  const handleEditRoom = () => {
    if (newRoomName.trim() && editingRoom) {
      onEditRoom(editingRoom.id, newRoomName.trim());
      setNewRoomName('');
      setEditDialogOpen(false);
      setEditingRoom(null);
    }
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter' && newRoomName.trim()) {
      handleCreateRoom();
    }
  };

  return (
    <>
      <SidebarContainer isOpen={isOpen}>
        <SidebarHeader>
          <Typography variant="h6">Chat Rooms</Typography>
          <IconButton onClick={onClose} size="small" edge="end">
            <CloseIcon />
          </IconButton>
        </SidebarHeader>
        
        <Box p={2}>
          <Button 
            variant="contained" 
            fullWidth 
            startIcon={<AddIcon />}
            onClick={() => {
              setNewRoomName('');
              setError('');
              setCreateDialogOpen(true);
            }}
          >
            CREATE NEW ROOM
          </Button>
        </Box>

        <RoomsList>
          {localRooms.length === 0 ? (
            <Typography variant="body2" sx={{ p: 2, color: 'text.secondary', textAlign: 'center' }}>
              No rooms available. Create one to get started!
            </Typography>
          ) : (
            localRooms.map((room) => (
              <RoomItem 
                key={room.id} 
                isActive={selectedRoom === room.id}
                onClick={() => handleRoomClick(room.id)}
              >
                <RoomName>
                  {room.name}
                </RoomName>
                <ActionButtons>
                  <IconButton 
                    size="small" 
                    onClick={(e) => handleEditClick(e, room.id, room.name)}
                  >
                    <EditIcon fontSize="small" />
                  </IconButton>
                  <IconButton 
                    size="small" 
                    onClick={(e) => handleDeleteClick(e, room.id)}
                  >
                    <DeleteIcon fontSize="small" />
                  </IconButton>
                  <IconButton
                    size="small"
                    color="primary"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleRoomClick(room.id);
                    }}
                  >
                    <JoinIcon fontSize="small" />
                  </IconButton>
                </ActionButtons>
              </RoomItem>
            ))
          )}
        </RoomsList>
      </SidebarContainer>

      {/* Create Room Dialog */}
      <Dialog 
        open={isCreateDialogOpen} 
        onClose={() => setCreateDialogOpen(false)}
        fullWidth
        maxWidth="xs"
      >
        <DialogTitle>Create New Room</DialogTitle>
        <DialogContent sx={{ pt: 2 }}>
          <TextField
            autoFocus
            margin="dense"
            label="Room Name"
            fullWidth
            value={newRoomName}
            onChange={(e) => setNewRoomName(e.target.value)}
            onKeyDown={handleKeyDown}
            error={!!error}
            helperText={error}
            inputProps={{ maxLength: 30 }}
          />
        </DialogContent>
        <DialogActions>
          <Button 
            onClick={() => setCreateDialogOpen(false)}
            sx={{ color: '#999' }}
          >
            CANCEL
          </Button>
          <Button 
            onClick={handleCreateRoom} 
            color="primary" 
            variant="contained"
            disabled={!newRoomName.trim()}
          >
            CREATE
          </Button>
        </DialogActions>
      </Dialog>

      {/* Edit Room Dialog */}
      <Dialog 
        open={isEditDialogOpen} 
        onClose={() => setEditDialogOpen(false)}
        PaperComponent={ModalContent}
        fullWidth
      >
        <DialogTitle>Edit Room</DialogTitle>
        <DialogContent>
          <TextField
            autoFocus
            margin="dense"
            label="Room Name"
            fullWidth
            value={newRoomName}
            onChange={(e) => setNewRoomName(e.target.value)}
            variant="outlined"
          />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancel</Button>
          <Button 
            onClick={handleEditRoom} 
            color="primary" 
            variant="contained"
            disabled={!newRoomName.trim()}
          >
            Save
          </Button>
        </DialogActions>
      </Dialog>

      <CreateRoomButton onRoomCreated={(roomName) => {
        if (onCreateRoom) {
          onCreateRoom(roomName);
        }
      }} />
    </>
  );
};

export default RoomSidebar; 