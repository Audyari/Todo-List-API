// MongoDB initialization script
db = db.getSiblingDB('todo_db');

// Create a user for the todo application
db.createUser({
  user: 'todo_user',
  pwd: 'todo_password',
  roles: [
    {
      role: 'readWrite',
      db: 'todo_db'
    }
  ]
});

// Create the todos collection with some initial data (optional)
db.todos.insertMany([
  {
    title: 'Sample Todo 1',
    description: 'This is a sample todo item',
    completed: false,
    createdAt: new Date(),
    updatedAt: new Date()
  },
  {
    title: 'Sample Todo 2',
    description: 'Another sample todo item',
    completed: true,
    createdAt: new Date(),
    updatedAt: new Date()
  }
]);

print('MongoDB initialization completed');