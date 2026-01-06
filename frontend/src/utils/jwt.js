// JWT utility functions
export const jwtUtils = {
  // Decode JWT token to extract payload
  decodeToken: (token) => {
    try {
      // Split the token to get the payload part
      const parts = token.split('.');
      if (parts.length !== 3) {
        throw new Error('Invalid token format');
      }
      
      // Decode the payload (second part)
      const payload = parts[1];
      // Add padding if needed
      const paddedPayload = payload + '='.repeat((4 - payload.length % 4) % 4);
      // Base64 decode
      const decodedPayload = atob(paddedPayload);
      // Parse as JSON
      return JSON.parse(decodedPayload);
    } catch (error) {
      console.error('Error decoding token:', error);
      throw new Error('Failed to decode token');
    }
  },
  
  // Get user ID from token
  getUserIdFromToken: (token) => {
    try {
      const payload = jwtUtils.decodeToken(token);
      return payload.userId || payload.sub || payload.id;
    } catch (error) {
      console.error('Error getting user ID from token:', error);
      return null;
    }
  },
  
  // Check if token is expired
  isTokenExpired: (token) => {
    try {
      const payload = jwtUtils.decodeToken(token);
      const currentTime = Math.floor(Date.now() / 1000); // Current time in seconds
      return payload.exp < currentTime;
    } catch (error) {
      console.error('Error checking token expiration:', error);
      return true; // Assume expired if we can't verify
    }
  }
};

export default jwtUtils;