# Rust Backend - Todo List API

RESTful API Hello World basic dengan Rust menggunakan Axum framework.

## Tech Stack

- **Framework**: Axum (Rust web framework)
- **Runtime**: Tokio
- **Serialization**: Serde
- **Database**: TBD

## Endpoints

### Hello World

```
GET /
```

Returns a hello world message with timestamp.

### Health Check

```
GET /health
```

Returns service health status.

## Installation & Running

1. Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Run the server:

```bash
cd backend
cargo run
```

3. Test the API:

```bash
curl http://localhost:3000/
curl http://localhost:3000/health
```

## Project Structure

```
backend/
├── Cargo.toml          # Dependencies and project config
├── .gitignore          # Git ignore rules
├── README.md           # This file
└── src/
    └── main.rs         # Main application code
```

## Development

- Port: 3000
- Framework: Axum v0.7
- Rust Edition: 2021
