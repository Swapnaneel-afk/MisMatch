import React, { useEffect, useState } from 'react';
import { Box, Typography, Paper, Button, TextField } from '@mui/material';

function Debug() {
  const [backendStatus, setBackendStatus] = useState('Checking...');
  const [wsStatus, setWsStatus] = useState('Not connected');
  const [error, setError] = useState(null);
  const [logs, setLogs] = useState([]);
  const [manualWsUrl, setManualWsUrl] = useState('wss://new-carpenter-production.up.railway.app/ws');

  const addLog = (message) => {
    setLogs(prev => [...prev, `${new Date().toLocaleTimeString()}: ${message}`]);
  };

  const checkBackendHealth = async () => {
    try {
      addLog('Checking backend health...');
      
      // Determine the health check URL based on the configuration
      const backendUrl = process.env.REACT_APP_WS_URL 
        ? process.env.REACT_APP_WS_URL.replace('ws', 'http').replace('/ws', '/health') 
        : 'https://new-carpenter-production.up.railway.app/health';
      
      addLog(`Sending request to: ${backendUrl}`);
      
      // Try using CORS proxy to bypass CORS issues for testing
      const proxyUrl = 'https://corsproxy.io/?';
      const fetchUrl = `${proxyUrl}${encodeURIComponent(backendUrl)}`;
      addLog(`Using CORS proxy: ${fetchUrl}`);
      
      const response = await fetch(fetchUrl, {
        method: 'GET',
        headers: {
          'Accept': 'application/json'
        }
      });
      
      addLog(`Response status: ${response.status} ${response.statusText}`);
      
      if (response.ok) {
        const data = await response.json();
        setBackendStatus('Connected: ' + JSON.stringify(data));
        addLog(`Backend responded: ${JSON.stringify(data)}`);
      } else {
        setBackendStatus(`Error: ${response.status} ${response.statusText}`);
        addLog(`Backend error: ${response.status} ${response.statusText}`);
      }
    } catch (error) {
      setBackendStatus(`Connection failed: ${error.message}`);
      setError(error.message);
      addLog(`Backend connection error: ${error.message}`);
    }
  };

  const testWebSocket = (url = null) => {
    try {
      addLog('Testing WebSocket connection...');
      
      // Get WebSocket URL from provided parameter, manual input, environment or infer it
      let wsUrl = url || manualWsUrl;
      
      if (!wsUrl) {
        if (window.location.hostname === 'localhost' || window.location.hostname === '127.0.0.1') {
          wsUrl = `ws://${window.location.hostname}:8080/ws`;
        } else {
          // Try direct connection to Railway URL
          wsUrl = 'wss://new-carpenter-production.up.railway.app/ws';
        }
      }
      
      addLog(`Connecting to: ${wsUrl}?username=debugger`);
      addLog(`Browser supports WebSocket: ${typeof WebSocket !== 'undefined'}`);
      
      setWsStatus('Connecting...');
      const ws = new WebSocket(`${wsUrl}?username=debugger`);
      
      ws.onopen = () => {
        setWsStatus('Connected');
        addLog('WebSocket connected successfully');
        
        // Send a test message
        const testMsg = {
          message_type: 'chat',
          user: 'debugger',
          text: 'Connection test',
          timestamp: new Date().toISOString()
        };
        ws.send(JSON.stringify(testMsg));
        addLog(`Sent test message: ${JSON.stringify(testMsg)}`);
      };
      
      ws.onmessage = (event) => {
        addLog(`Received message: ${event.data}`);
      };
      
      ws.onclose = (event) => {
        let closeReason = '';
        switch(event.code) {
          case 1000: closeReason = 'Normal closure'; break;
          case 1001: closeReason = 'Going away'; break;
          case 1002: closeReason = 'Protocol error'; break;
          case 1003: closeReason = 'Unsupported data'; break;
          case 1005: closeReason = 'No status received'; break;
          case 1006: closeReason = 'Abnormal closure'; break;
          case 1007: closeReason = 'Invalid frame payload data'; break;
          case 1008: closeReason = 'Policy violation'; break;
          case 1009: closeReason = 'Message too big'; break;
          case 1010: closeReason = 'Mandatory extension'; break;
          case 1011: closeReason = 'Internal server error'; break;
          case 1012: closeReason = 'Service restart'; break;
          case 1013: closeReason = 'Try again later'; break;
          case 1014: closeReason = 'Bad gateway'; break;
          case 1015: closeReason = 'TLS handshake'; break;
          default: closeReason = 'Unknown'; break;
        }
        setWsStatus(`Closed: Code ${event.code} (${closeReason})`);
        addLog(`WebSocket closed: Code ${event.code} (${closeReason}) ${event.reason || ''}`);
        
        // Add recommendation for code 1006
        if (event.code === 1006) {
          addLog('Code 1006 usually indicates a network issue or CORS problem. Check that:');
          addLog('1. The backend server is reachable and accepting WebSocket connections');
          addLog('2. CORS is properly configured on the backend to allow connections from this origin');
          addLog('3. The WebSocket URL is correct (wss:// for HTTPS sites, ws:// for HTTP)');
        }
      };
      
      ws.onerror = (error) => {
        setWsStatus('Error');
        setError(`WebSocket error - check browser console for details`);
        addLog(`WebSocket error occurred. This is often due to network issues, CORS, or an unavailable server.`);
        console.error('WebSocket error:', error);
      };
      
      // Close connection after 10 seconds
      setTimeout(() => {
        if (ws.readyState === WebSocket.OPEN) {
          ws.close();
          addLog('Closed WebSocket connection after 10s');
        }
      }, 10000);
    } catch (error) {
      setWsStatus(`Error: ${error.message}`);
      setError(error.message);
      addLog(`Error creating WebSocket: ${error.message}`);
    }
  };

  // Add a function to check CORS info
  const checkCorsInfo = () => {
    const origin = window.location.origin;
    addLog(`Current origin: ${origin}`);
    addLog(`This origin needs to be allowed in the backend CORS configuration`);
    
    // Try an OPTIONS request
    addLog(`Testing preflight OPTIONS request with CORS proxy...`);
    
    const backendUrl = process.env.REACT_APP_WS_URL 
      ? process.env.REACT_APP_WS_URL.replace('ws', 'http').replace('/ws', '/health') 
      : 'https://new-carpenter-production.up.railway.app/health';
    
    const proxyUrl = 'https://corsproxy.io/?';
    const fetchUrl = `${proxyUrl}${encodeURIComponent(backendUrl)}`;
    
    fetch(fetchUrl, {
      method: 'OPTIONS',
      headers: {
        'Origin': origin,
        'Access-Control-Request-Method': 'GET'
      }
    }).then(response => {
      addLog(`OPTIONS response status: ${response.status}`);
      
      // List all response headers
      const headers = {};
      response.headers.forEach((value, key) => {
        headers[key] = value;
      });
      addLog(`Response headers: ${JSON.stringify(headers, null, 2)}`);
      
    }).catch(error => {
      addLog(`OPTIONS request error: ${error.message}`);
    });
  };

  useEffect(() => {
    // Log environment information
    addLog(`Environment: ${process.env.NODE_ENV}`);
    addLog(`Current URL: ${window.location.href}`);
    addLog(`WebSocket URL from env: ${process.env.REACT_APP_WS_URL || 'not set'}`);
    
    // Auto-run the health check
    checkBackendHealth();
  }, []);

  return (
    <Box sx={{ p: 3, maxWidth: 800, mx: 'auto' }}>
      <Typography variant="h4" gutterBottom>
        Connection Debugger
      </Typography>
      
      <Paper sx={{ p: 2, mb: 3 }}>
        <Typography variant="h6">Backend Status</Typography>
        <Typography color={backendStatus.includes('Connected') ? 'success.main' : 'error.main'}>
          {backendStatus}
        </Typography>
        <Box sx={{ display: 'flex', gap: 1, mt: 1 }}>
          <Button 
            variant="contained" 
            onClick={checkBackendHealth}
          >
            Check Backend
          </Button>
          <Button 
            variant="outlined" 
            onClick={checkCorsInfo}
          >
            Check CORS
          </Button>
        </Box>
      </Paper>
      
      <Paper sx={{ p: 2, mb: 3 }}>
        <Typography variant="h6">WebSocket Status</Typography>
        <Typography color={wsStatus === 'Connected' ? 'success.main' : 'info.main'}>
          {wsStatus}
        </Typography>
        
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1, mt: 1 }}>
          <TextField 
            label="WebSocket URL" 
            variant="outlined"
            fullWidth
            value={manualWsUrl}
            onChange={(e) => setManualWsUrl(e.target.value)}
            placeholder="e.g., wss://new-carpenter-production.up.railway.app/ws"
            size="small"
          />
          
          <Box sx={{ display: 'flex', gap: 1 }}>
            <Button 
              variant="contained" 
              onClick={() => testWebSocket()}
            >
              Test WebSocket
            </Button>
            <Button 
              variant="outlined" 
              onClick={() => {
                // Try the most likely working URL for WebSockets
                testWebSocket('wss://new-carpenter-production.up.railway.app/ws');
              }}
            >
              Try Direct Connection
            </Button>
          </Box>
        </Box>
      </Paper>
      
      {error && (
        <Paper sx={{ p: 2, mb: 3, bgcolor: 'error.light' }}>
          <Typography variant="h6">Error</Typography>
          <Typography>{error}</Typography>
        </Paper>
      )}
      
      <Paper sx={{ p: 2, maxHeight: 300, overflow: 'auto' }}>
        <Typography variant="h6">Debug Logs</Typography>
        {logs.map((log, index) => (
          <Typography key={index} variant="body2" fontFamily="monospace" sx={{ mb: 0.5 }}>
            {log}
          </Typography>
        ))}
      </Paper>
    </Box>
  );
}

export default Debug; 