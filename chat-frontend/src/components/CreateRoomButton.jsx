import React, { useState } from 'react';
import { Dialog, DialogTitle, DialogContent, DialogActions, Button, TextField, Fab } from '@mui/material';
import AddIcon from '@mui/icons-material/Add';
import { ModalContent } from './StyledComponents';

const CreateRoomButton = ({ onRoomCreated }) => {
  const [isDialogOpen, setDialogOpen] = useState(false);
  const [roomName, setRoomName] = useState('');
  const [error, setError] = useState('');

  const handleOpenDialog = () => {
    setDialogOpen(true);
    setRoomName('');
    setError('');
  };

  const handleCloseDialog = () => {
    setDialogOpen(false);
  };

  const handleCreateRoom = () => {
    if (!roomName.trim()) {
      setError('Room name cannot be empty');
      return;
    }

    if (roomName.length > 30) {
      setError('Room name must be less than 30 characters');
      return;
    }

    onRoomCreated(roomName.trim());
    setDialogOpen(false);
    setRoomName('');
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      handleCreateRoom();
    }
  };

  return (
    <>
      <Fab 
        color="primary" 
        aria-label="add room"
        onClick={handleOpenDialog}
        sx={{
          position: 'fixed',
          bottom: 24,
          right: 24,
          zIndex: 1100
        }}
      >
        <AddIcon />
      </Fab>

      <Dialog open={isDialogOpen} onClose={handleCloseDialog} fullWidth maxWidth="xs">
        <DialogTitle>Create New Room</DialogTitle>
        <DialogContent>
          <ModalContent>
            <TextField
              autoFocus
              margin="dense"
              label="Room Name"
              type="text"
              fullWidth
              value={roomName}
              onChange={(e) => setRoomName(e.target.value)}
              onKeyDown={handleKeyDown}
              error={!!error}
              helperText={error}
              inputProps={{ maxLength: 30 }}
            />
          </ModalContent>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleCloseDialog}>Cancel</Button>
          <Button onClick={handleCreateRoom} color="primary" variant="contained">
            Create
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
};

export default CreateRoomButton; 