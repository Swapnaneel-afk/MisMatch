# Deployment Troubleshooting Guide

## WebSocket Connection Issues

### Problem
When attempting to connect to the WebSocket endpoint at `wss://new-carpenter-production.up.railway.app/ws?username=tester`, we encountered consistent connection failures with:
- Error: Unknown error
- Disconnected: Code 1006 (Abnormal closure)

### Diagnosis
1. **WebSocket Code 1006 (Abnormal closure)** indicates that the connection was closed unexpectedly without a proper close frame being received.
2. The Railway deployment was not responding to basic HTTP requests, suggesting the application wasn't properly deployed or was offline.

### Solution
1. **Redeployed the application**: Used `railway up` command to redeploy the application to Railway.
2. The redeployment restored the service at the domain `new-carpenter-production.up.railway.app`.
3. After redeployment, the WebSocket connection was successfully established.

## Possible Causes of WebSocket Connection Issues
- Service downtime or failed deployment
- Railway platform maintenance or temporary outage
- Application crash due to resource constraints
- Network connectivity issues between client and Railway servers

## Prevention Tips
- Set up health checks to monitor application uptime
- Implement proper error handling in WebSocket connections
- Add reconnection logic to WebSocket clients
- Monitor Railway deployment logs for potential errors
- Use proper error handling on both client and server sides

## Commands Used for Troubleshooting
- `railway status` - Check project status
- `railway domain` - Verify current domain
- `railway up` - Redeploy the application 

## System Architecture

### Real-time Chat System Overview
This application implements a real-time chat system using:
- **Rust Backend**: Built with Actix Web framework for high performance
- **WebSockets**: For real-time bidirectional communication
- **PostgreSQL Database**: For persistent message storage and user management
- **Railway Deployment**: Cloud platform for hosting both the application and database

### Key Components
1. **WebSocket Server**: Handles real-time message broadcasting between connected clients
2. **HTTP API**: Provides endpoints for user operations and message history
3. **Database Layer**: Persists chat messages and user information
4. **Client Application**: Connects to the WebSocket server and renders the chat interface

### Why PostgreSQL?
PostgreSQL is essential to our chat application for several reasons:

1. **Message Persistence**: Stores all chat messages, allowing users to see conversation history when they join or reconnect
2. **User Management**: Maintains user profiles, authentication details, and session information
3. **Relationship Handling**: Efficiently manages complex relationships between users, chats, and groups
4. **Transactional Integrity**: Ensures that operations like sending messages or updating user statuses are atomic and consistent
5. **Scalability**: Can handle growing numbers of users and messages with proper indexing and optimizations
6. **Advanced Queries**: Supports complex queries for features like message search, filtering, and analytics
7. **JSONB Support**: Enables flexible schema for storing message metadata and user preferences

PostgreSQL provides the reliability and feature set needed for a production-ready chat application where data integrity and consistency are critical. 

## Detailed Message Flow

### Complete End-to-End Process

#### 1. User Authentication and Connection
1. **User Login**: 
   - User enters credentials on the login page
   - Frontend sends authentication request to the backend via HTTPS
   - Backend validates credentials against PostgreSQL user table
   - JWT token is generated and returned to the client
   
2. **WebSocket Connection Establishment**:
   - Client initiates WebSocket connection to `wss://new-carpenter-production.up.railway.app/ws?username={username}`
   - Server verifies the JWT token and username
   - Connection is established, creating a new WebSocket session
   - User's online status is updated in PostgreSQL
   - Server broadcasts "user joined" event to other connected clients

#### 2. Message Sending Process
1. **User Types and Sends Message**:
   - User types text in the input field and clicks send button (or presses Enter)
   - Client-side validation checks for empty messages or other constraints
   - Frontend creates a message object with metadata (timestamp, sender, message type)

2. **Client to Server Transmission**:
   - Message is serialized to JSON format
   - Message is sent through the established WebSocket connection
   - Client may display a "sending" indicator or temporary message preview

3. **Server Processing**:
   - WebSocket server receives the JSON message
   - Message is deserialized into the appropriate Rust struct
   - Server validates message format and content
   - Server determines message recipients (group chat vs direct message)
   - Message is enriched with server-side metadata (server timestamp, message ID)

4. **Database Persistence**:
   - Server creates a database transaction
   - Message is written to the `messages` table in PostgreSQL
   - Related tables may be updated (e.g., conversation last activity timestamp)
   - Transaction is committed

5. **Message Broadcasting**:
   - Server identifies all WebSocket connections that should receive the message
   - Message is serialized to JSON
   - Message is broadcast to all recipient connections
   - Delivery receipts may be tracked and stored

#### 3. Message Reception Process
1. **Client Receives Message**:
   - Recipient's WebSocket connection receives the JSON message
   - Client deserializes the message into appropriate data structure
   - Client validates the message format

2. **UI Updates**:
   - Message is added to the appropriate conversation in the UI
   - Chat window scrolls to show new message if appropriate
   - Notification sounds may play
   - Unread message counters are updated
   - Read receipts may be sent back to the server

3. **Offline Handling**:
   - For users currently offline, messages remain in PostgreSQL
   - When users reconnect, their client requests recent message history
   - Server retrieves unread messages from PostgreSQL and sends them to the newly connected client

#### 4. Additional Features
1. **Message Status Tracking**:
   - Sent: Message has left the sender's device
   - Delivered: Message has been received by the server and stored in PostgreSQL
   - Read: Recipient has viewed the message
   - Each status change is communicated via WebSocket and persisted to PostgreSQL

2. **Media Handling**:
   - Images/files are uploaded via separate HTTP endpoints
   - Server stores media in object storage
   - Only media references/URLs are sent through WebSockets
   - Media is loaded asynchronously by recipient clients

3. **Real-time Features**:
   - Typing indicators: WebSocket events signal when users are typing
   - Presence updates: Online/offline status changes are broadcast
   - Read receipts: Notifications when messages are viewed

### Error Handling and Edge Cases
1. **Connection Issues**:
   - Client implements reconnection strategy with exponential backoff
   - Messages typed during disconnection are queued locally
   - Upon reconnection, queued messages are sent and recent history is fetched

2. **Message Delivery Failures**:
   - Server tracks failed deliveries in PostgreSQL
   - Retry mechanisms handle transient failures
   - Client shows delivery status indicators to users

3. **Database Failures**:
   - Write operations use transactions to ensure consistency
   - Read fallbacks may provide degraded service if database is overloaded
   - Caching layer may reduce database load for frequently accessed data

4. **Load Balancing and Scaling**:
   - Multiple WebSocket server instances handle connection distribution
   - Pub/Sub system (like Redis) coordinates message distribution across server instances
   - Database connection pooling optimizes PostgreSQL resource usage 