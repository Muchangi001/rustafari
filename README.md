# 🦀 Rustafari: A Vanilla Rust Community Platform

Rustafari is a vanilla Rust community platform that connects Rust developers based on their interests, mentorship needs, and collaboration opportunities.

## Features

- **User Profiles**: Create profiles with interests and bio information
- **Connection Types**: Connect as mentors, collaborators, followers, or project buddies
- **Interest-Based Discovery**: Find other Rust developers with similar interests
- **Smart Recommendations**: Get recommended connections based on shared interests
- **Completely Vanilla Rust**: Built with pure Rust and minimal dependencies

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/users` | POST | Add a new user |
| `/users/:username` | GET | Get a specific user's profile |
| `/connections` | POST | Create a connection between users |
| `/users/:username/recommendations` | GET | Get connection recommendations for a user |
| `/interests/:interest/users` | GET | Find users with a specific interest |

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

1. Clone the repository
```bash
git clone https://github.com/your-username/rustafari.git
cd rustafari
```

2. Build the project
```bash
cargo build --release
```

3. Run the server
```bash
cargo run --release
```

The server will start at http://127.0.0.1:3000

## Example API Usage

### Creating a User

```bash
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"username": "ferris", "bio": "Rust enthusiast and crab lover", "interests": ["async", "embedded", "wasm"]}'
```

### Connecting Users

```bash
curl -X POST http://localhost:3000/connections \
  -H "Content-Type: application/json" \
  -d '{"from": "ferris", "to": "rustacean", "kind": "Mentor", "tags": ["async"], "since": "2023-05-07"}'
```

### Getting Recommendations

```bash
curl http://localhost:3000/users/ferris/recommendations
```

## Project Structure

- `src/main.rs`: Server initialization
- `src/graph.rs`: Community graph data structure and operations
- `src/routes.rs`: API endpoints and handlers
- `src/errors.rs`: Error handling infrastructure

## Contributing

We welcome contributions! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.