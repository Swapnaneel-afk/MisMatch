import { styled, keyframes, alpha } from "@mui/material/styles";
import { Paper, Box, TextField, Fab } from "@mui/material";

const fadeIn = keyframes`
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
`;

export const GlassPaper = styled(Paper)(({ theme }) => ({
  padding: theme.spacing(3),
  backdropFilter: "blur(10px)",
  borderRadius: theme.shape.borderRadius * 2,
  transition: "all 0.3s ease-in-out",
  animation: `${fadeIn} 0.5s ease-out`,
  "&:hover": {
    transform: "translateY(-2px)",
  },
}));

export const MessageContainer = styled(Box)(({ theme, isOwn }) => ({
  display: "flex",
  justifyContent: isOwn ? "flex-end" : "flex-start",
  marginBottom: theme.spacing(2),
  animation: `${fadeIn} 0.3s ease-out`,
}));

export const MessageBubble = styled(Paper)(({ theme, isOwn }) => ({
  padding: theme.spacing(1.5, 2),
  maxWidth: "70%",
  borderRadius: theme.shape.borderRadius * 2,
  backgroundColor: isOwn
    ? theme.palette.primary.main
    : theme.palette.background.paper,
  color: isOwn
    ? theme.palette.primary.contrastText
    : theme.palette.text.primary,
  boxShadow: "none",
  position: "relative",
}));

export const StyledTextField = styled(TextField)(({ theme }) => ({
  "& .MuiOutlinedInput-root": {
    borderRadius: theme.shape.borderRadius * 1.5,
    transition: "all 0.2s ease-in-out",
    backgroundColor: theme.palette.background.paper,
    "&:hover": {
      transform: "translateY(-1px)",
    },
    "&.Mui-focused": {
      transform: "translateY(-1px)",
    },
  },
}));

export const UserChip = styled(Paper)(({ theme, isOnline }) => ({
  padding: theme.spacing(0.5, 1.5),
  borderRadius: theme.shape.borderRadius * 3,
  display: "inline-flex",
  alignItems: "center",
  gap: theme.spacing(1),
  backgroundColor: isOnline
    ? alpha(theme.palette.success.main, 0.1)
    : theme.palette.background.paper,
  border: `1px solid ${
    isOnline ? theme.palette.success.main : theme.palette.divider
  }`,
  transition: "all 0.2s ease-in-out",
  "&:hover": {
    transform: "translateY(-1px)",
  },
}));

export const ChatContainer = styled(Box)(({ theme }) => ({
  height: "100vh",
  display: "flex",
  flexDirection: "column",
  background:
    theme.palette.mode === "dark"
      ? "linear-gradient(145deg, #000000 0%, #1a1a1a 100%)"
      : "linear-gradient(145deg, #f6f6f6 0%, #ffffff 100%)",
}));

export const Header = styled(Box)(({ theme }) => ({
  padding: theme.spacing(2),
  backdropFilter: "blur(10px)",
  borderBottom: `1px solid ${theme.palette.divider}`,
  display: "flex",
  alignItems: "center",
  justifyContent: "space-between",
  backgroundColor:
    theme.palette.mode === "dark"
      ? "rgba(0, 0, 0, 0.5)"
      : "rgba(255, 255, 255, 0.5)",
}));

export const ChatArea = styled(Box)(({ theme }) => ({
  flex: 1,
  overflow: "auto",
  padding: theme.spacing(2),
  "&::-webkit-scrollbar": {
    width: "8px",
  },
  "&::-webkit-scrollbar-track": {
    background: "transparent",
  },
  "&::-webkit-scrollbar-thumb": {
    background:
      theme.palette.mode === "dark"
        ? "rgba(255,255,255,0.2)"
        : "rgba(0,0,0,0.2)",
    borderRadius: "4px",
  },
}));

export const InputArea = styled(Box)(({ theme }) => ({
  padding: theme.spacing(2),
  backdropFilter: "blur(10px)",
  borderTop: `1px solid ${theme.palette.divider}`,
  backgroundColor:
    theme.palette.mode === "dark"
      ? "rgba(0, 0, 0, 0.5)"
      : "rgba(255, 255, 255, 0.5)",
}));

// New components for room management
export const SidebarContainer = styled(Box)(({ theme, isOpen }) => ({
  position: 'fixed',
  top: 0,
  left: isOpen ? 0 : '-280px',
  width: 280,
  height: '100vh',
  backgroundColor: theme.palette.mode === "dark" ? "#111" : "#f5f5f5",
  borderRight: `1px solid ${theme.palette.divider}`,
  zIndex: 1200,
  display: 'flex',
  flexDirection: 'column',
  transition: 'left 0.3s ease-in-out',
  boxShadow: isOpen ? theme.shadows[8] : 'none',
}));

export const SidebarHeader = styled(Box)(({ theme }) => ({
  padding: theme.spacing(2),
  borderBottom: `1px solid ${theme.palette.divider}`,
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  backgroundColor: theme.palette.mode === "dark" ? "#000" : "#fff",
}));

export const RoomsList = styled(Box)(({ theme }) => ({
  flexGrow: 1,
  overflow: 'auto',
  padding: theme.spacing(1),
}));

export const RoomItem = styled(Box)(({ theme, isActive }) => ({
  padding: theme.spacing(1.5),
  borderRadius: theme.shape.borderRadius,
  marginBottom: theme.spacing(0.5),
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  cursor: 'pointer',
  backgroundColor: isActive ? 
    (theme.palette.mode === "dark" ? "rgba(255,255,255,0.1)" : "rgba(0,0,0,0.1)") 
    : 'transparent',
  '&:hover': {
    backgroundColor: theme.palette.mode === "dark" ? "rgba(255,255,255,0.05)" : "rgba(0,0,0,0.05)",
  },
}));

export const RoomName = styled(Box)({
  flexGrow: 1,
  overflow: 'hidden',
  textOverflow: 'ellipsis',
  whiteSpace: 'nowrap',
});

export const ActionButtons = styled(Box)(({ theme }) => ({
  display: 'flex',
  gap: theme.spacing(0.5),
}));

// Floating action button
export const FloatingButton = styled(Fab)(({ theme }) => ({
  position: 'absolute',
  bottom: theme.spacing(2),
  right: theme.spacing(2),
}));

// Dialog components
export const ModalContent = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.background.paper,
  borderRadius: theme.shape.borderRadius,
  padding: theme.spacing(2),
}));
