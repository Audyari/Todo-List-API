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

// Create the tasks collection with some initial data (optional)
db.tasks.insertMany([
  {
    title: 'Sample Task 1',
    description: 'This is a sample task item',
    completed: false,
    userId: ObjectId(),
    createdAt: new Date(),
    updatedAt: new Date()
  },
  {
    title: 'Sample Task 2',
    description: 'Another sample task item',
    completed: true,
    userId: ObjectId(),
    createdAt: new Date(),
    updatedAt: new Date()
  }
]);

print('MongoDB initialization completed');