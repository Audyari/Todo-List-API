#!/bin/bash
# Script to run the Todo List API with MongoDB

echo "Starting Todo List API with MongoDB..."

# Check if docker-compose is available
if ! [ -x "$(command -v docker-compose)" ]; then
  echo 'Error: docker-compose is not installed.' >&2
  exit 1
fi

# Start the services
docker-compose up -d

echo "Services started successfully!"
echo "Backend API: http://localhost:3000"
echo "MongoDB: mongodb://admin:password@localhost:27017"
echo ""
echo "To view logs: docker-compose logs -f"
echo "To stop services: docker-compose down"