import { createTheme } from "@mui/material/styles";

const commonStyles = {
  typography: {
    fontFamily:
      '-apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif',
    h1: {
      fontSize: "2.5rem",
      fontWeight: 600,
      letterSpacing: "-0.02em",
    },
    h2: {
      fontSize: "2rem",
      fontWeight: 600,
      letterSpacing: "-0.02em",
    },
    body1: {
      fontSize: "1rem",
      letterSpacing: "-0.01em",
    },
  },
  shape: {
    borderRadius: 12,
  },
  transitions: {
    easing: {
      easeInOut: "cubic-bezier(0.4, 0, 0.2, 1)",
    },
  },
};

export const lightTheme = createTheme({
  ...commonStyles,
  palette: {
    mode: "light",
    primary: {
      main: "#000000",
      light: "#1a1a1a",
      dark: "#000000",
      contrastText: "#ffffff",
    },
    secondary: {
      main: "#86868b",
    },
    background: {
      default: "#ffffff",
      paper: "rgba(255, 255, 255, 0.8)",
    },
    text: {
      primary: "#1d1d1f",
      secondary: "#86868b",
    },
  },
});

export const darkTheme = createTheme({
  ...commonStyles,
  palette: {
    mode: "dark",
    primary: {
      main: "#ffffff",
      light: "#f5f5f7",
      dark: "#e5e5e5",
      contrastText: "#000000",
    },
    secondary: {
      main: "#86868b",
    },
    background: {
      default: "#000000",
      paper: "rgba(0, 0, 0, 0.8)",
    },
    text: {
      primary: "#f5f5f7",
      secondary: "#86868b",
    },
  },
});
