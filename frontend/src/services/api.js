// API Service for communicating with Rust backend
const API_BASE_URL = 'http://localhost:3000/api';

// Create a base API request function
const apiRequest = async (endpoint, options = {}) => {
  const url = `${API_BASE_URL}${endpoint}`;
  
  // Get token from localStorage if available
  const token = localStorage.getItem('authToken');
  
  const config = {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    ...options,
  };

  // Add authorization header if token exists
  if (token) {
    config.headers['Authorization'] = `Bearer ${token}`;
  }

  try {
    const response = await fetch(url, config);

    // Handle different response status codes
    if (!response.ok) {
      let errorMessage = `HTTP error! status: ${response.status}`;

      try {
        const errorData = await response.json();
        if (errorData.message) {
          errorMessage = errorData.message;
        } else {
          // If no message in response body, use status text
          errorMessage = response.statusText || errorMessage;
        }
      } catch (jsonError) {
        // If response is not JSON, use status text
        errorMessage = response.statusText || errorMessage;
      }

      throw new Error(errorMessage);
    }

    const result = await response.json();
    console.log('API response:', result);

    // Check if the response indicates an error even with 200 status
    if (result && typeof result === 'object' && result.success === false) {
      throw new Error(result.message || 'API request failed');
    }

    return result;
  } catch (error) {
    console.error('API request error:', error);
    throw error;
  }
};

// User API functions
export const userAPI = {
  // Register a new user
  register: async (userData) => {
    return apiRequest('/users/register', {
      method: 'POST',
      body: JSON.stringify(userData),
    });
  },

  // Login user
  login: async (credentials) => {
    return apiRequest('/users/login', {
      method: 'POST',
      body: JSON.stringify(credentials),
    });
  },

  // Get all users (requires authentication)
  getUsers: async () => {
    return apiRequest('/users', {
      method: 'GET',
    });
  },

  // Get user by ID (requires authentication)
  getUserById: async (id) => {
    return apiRequest(`/users/${id}`, {
      method: 'GET',
    });
  },

  // Create a new user (requires authentication)
  createUser: async (userData) => {
    return apiRequest('/users', {
      method: 'POST',
      body: JSON.stringify(userData),
    });
  },

  // Update user by ID (requires authentication)
  updateUser: async (id, userData) => {
    return apiRequest(`/users/${id}`, {
      method: 'PUT',
      body: JSON.stringify(userData),
    });
  },

  // Delete user by ID (requires authentication)
  deleteUser: async (id) => {
    return apiRequest(`/users/${id}`, {
      method: 'DELETE',
    });
  },
};

import jwtUtils from '../utils/jwt';

// Task API functions
export const taskAPI = {
  // Get all tasks (requires authentication)
  getTasks: async () => {
    console.log('Fetching all tasks...');
    const result = await apiRequest('/tasks', {
      method: 'GET',
    });
    console.log('Tasks fetched successfully:', result);
    return result;
  },

  // Get task by ID (requires authentication)
  getTaskById: async (id) => {
    // Ensure id is a string, not an object
    // Handle different ID formats that might come from MongoDB
    let taskId = id;
    if (typeof id === 'object') {
      if (id._id) {
        // If it's an object with _id property
        taskId = id._id;
      } else if (id.$oid) {
        // If it's a MongoDB ObjectId format { $oid: '...' }
        taskId = id.$oid;
      } else {
        // If it's a string representation of ObjectId
        taskId = id.toString();
      }
    }

    return apiRequest(`/tasks/${taskId}`, {
      method: 'GET',
    });
  },

  // Create a new task (requires authentication)
  createTask: async (taskData) => {
    // Extract user ID from the stored token
    const token = localStorage.getItem('authToken');
    if (!token) {
      throw new Error('Authentication token not found. Please log in.');
    }

    const userId = jwtUtils.getUserIdFromToken(token);
    if (!userId) {
      throw new Error('User ID not found in token. Please log in again.');
    }

    // Add user ID to the task data - backend expects userId as string representation of ObjectId
    const taskDataWithUserId = {
      ...taskData,
      userId: userId  // This should be the hex string representation of ObjectId
    };

    console.log('Creating task with data:', taskDataWithUserId);

    return apiRequest('/tasks', {
      method: 'POST',
      body: JSON.stringify(taskDataWithUserId),
    });
  },

  // Update task by ID (requires authentication)
  updateTask: async (id, taskData) => {
    // Ensure id is a string, not an object
    // Handle different ID formats that might come from MongoDB
    let taskId = id;
    if (typeof id === 'object') {
      if (id._id) {
        // If it's an object with _id property
        taskId = id._id;
      } else if (id.$oid) {
        // If it's a MongoDB ObjectId format { $oid: '...' }
        taskId = id.$oid;
      } else {
        // If it's a string representation of ObjectId
        taskId = id.toString();
      }
    }

    // Create a copy of taskData to avoid modifying the original
    const updateData = { ...taskData };

    // Don't include userId in the update payload - backend should identify user from JWT
    // The backend validates authorization based on the token, not the userId in the payload
    if (updateData.userId) {
      delete updateData.userId;
    }

    // Ensure we're sending a valid request
    console.log('Updating task with ID:', taskId, 'and data:', updateData);

    return apiRequest(`/tasks/${taskId}`, {
      method: 'PUT',
      body: JSON.stringify(updateData),
    });
  },

  // Delete task by ID (requires authentication)
  deleteTask: async (id) => {
    // Ensure id is a string, not an object
    // Handle different ID formats that might come from MongoDB
    let taskId = id;
    if (typeof id === 'object') {
      if (id._id) {
        // If it's an object with _id property
        taskId = id._id;
      } else if (id.$oid) {
        // If it's a MongoDB ObjectId format { $oid: '...' }
        taskId = id.$oid;
      } else {
        // If it's a string representation of ObjectId
        taskId = id.toString();
      }
    }

    return apiRequest(`/tasks/${taskId}`, {
      method: 'DELETE',
    });
  },
};

export default {
  userAPI,
  taskAPI,
};