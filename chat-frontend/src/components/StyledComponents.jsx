import { styled } from "@mui/material/styles";
import { Paper, Box, TextField } from "@mui/material";

export const ChatContainer = styled(Box)({
  height: "100vh",
  display: "flex",
  flexDirection: "column",
});

export const Header = styled(Box)(({ theme }) => ({
  padding: theme.spacing(2),
  borderBottom: `1px solid ${theme.palette.divider}`,
}));

export const ChatArea = styled(Box)(({ theme }) => ({
  flex: 1,
  overflow: "auto",
  padding: theme.spacing(2),
}));

export const InputArea = styled(Box)(({ theme }) => ({
  padding: theme.spacing(2),
  borderTop: `1px solid ${theme.palette.divider}`,
}));

export const MessageContainer = styled(Box, {
  shouldForwardProp: (prop) => prop !== "isOwn",
})(({ isOwn }) => ({
  display: "flex",
  justifyContent: isOwn ? "flex-end" : "flex-start",
  marginBottom: "8px",
}));

export const MessageBubble = styled(Paper, {
  shouldForwardProp: (prop) => prop !== "isOwn",
})(({ theme, isOwn }) => ({
  padding: theme.spacing(1, 2),
  maxWidth: "70%",
  backgroundColor: isOwn
    ? theme.palette.primary.main
    : theme.palette.background.paper,
  color: isOwn
    ? theme.palette.primary.contrastText
    : theme.palette.text.primary,
}));

export const StyledTextField = styled(TextField)({
  width: "100%",
});

export const UserChip = styled(Paper, {
  shouldForwardProp: (prop) => prop !== "isOnline",
})(({ theme, isOnline }) => ({
  display: "inline-flex",
  alignItems: "center",
  padding: theme.spacing(0.5, 1),
  borderRadius: theme.shape.borderRadius * 2,
  gap: theme.spacing(1),
  backgroundColor: isOnline
    ? theme.palette.success.light
    : theme.palette.background.paper,
}));
