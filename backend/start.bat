@echo off
REM Script to run the Todo List API with MongoDB on Windows

echo Starting Todo List API with MongoDB...

REM Check if docker-compose is available
docker-compose --version >nul 2>&1
if errorlevel 1 (
    echo Error: docker-compose is not installed or not in PATH.
    exit /b 1
)

REM Start the services
docker-compose up -d

echo Services started successfully!
echo Backend API: http://localhost:3000
echo MongoDB: mongodb://admin:password@localhost:27017
echo.
echo To view logs: docker-compose logs -f
echo To stop services: docker-compose down