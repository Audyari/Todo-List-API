# Todo List API with MongoDB

This is a Rust-based Todo List API using Axum framework and MongoDB as the database.

## Prerequisites

- Docker
- Docker Compose

## Running the Application

### Using Docker Compose (Recommended)

1. Make sure you're in the backend directory:
   ```bash
   cd backend
   ```

2. Start the application and MongoDB:
   ```bash
   docker-compose up -d
   ```

3. The application will be available at: `http://localhost:3000`
4. MongoDB will be available at: `mongodb://admin:password@localhost:27017`

### Building and Running Manually

1. Build the application:
   ```bash
   docker build -t todo-backend .
   ```

2. Run MongoDB separately:
   ```bash
   docker run -d -p 27017:27017 --name todo-mongodb -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD=password -v mongodb_data:/data/db mongo:7.0
   ```

3. Run the application:
   ```bash
   docker run -p 3000:3000 --name todo-backend --link todo-mongodb -e MONGODB_URI=mongodb://admin:password@todo-mongodb:27017/todo_db?authSource=admin todo-backend
   ```

## API Endpoints

- `GET /` - Hello World endpoint
- `GET /health` - Health check endpoint
- `GET /api/todos` - Get all todos
- `POST /api/todos` - Create a new todo
- `GET /api/todos/{id}` - Get a specific todo by ID
- `PUT /api/todos/{id}` - Update a specific todo
- `DELETE /api/todos/{id}` - Delete a specific todo

## Environment Variables

- `MONGODB_URI` - MongoDB connection string
- `DATABASE_NAME` - Name of the database to use

## Stopping the Application

To stop the application:
```bash
docker-compose down
```

To stop and remove volumes (this will delete all data):
```bash
docker-compose down -v
```

## Development

For development, you can modify the docker-compose.yml file to mount the source code and use a development runner instead of the compiled binary.