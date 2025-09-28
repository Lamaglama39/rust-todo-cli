# todo-cli

A simple command-line todo application written in Rust using SQLite for persistent storage.

## Features

- **Add tasks** - Create new todo items
- **Complete tasks** - Mark tasks as completed
- **List tasks** - View all tasks with completion status
- **Delete tasks** - Remove tasks from the list
- **Persistent storage** - Uses SQLite database for data persistence
- **Comprehensive tests** - Full test coverage with integration tests

## Installation

### Prerequisites
- Rust 1.90.0 or later
- Cargo (comes with Rust)

### Build from source
```bash
git clone <repository-url>
cd todo-cli
cargo build --release
```

The binary will be available at `target/release/todo-cli`.

## Usage

### Help
```bash
cargo run -- --help
cargo run -- add --help    # Help for specific command
```

### Add a task
```bash
cargo run -- add "Buy groceries"
cargo run -- add "Study Rust programming"
```

### List all tasks
```bash
cargo run -- list
```

Output example:
```
Todo List:
[ ] Buy groceries
[x] Study Rust programming
[ ] Write documentation
```

- `[ ]` - Incomplete task
- `[x]` - Completed task

### Complete a task
```bash
cargo run -- complete "Buy groceries"
```

### Delete a task
```bash
cargo run -- delete "Buy groceries"
```

### Version
```bash
cargo run -- --version
```

## Testing

This project includes comprehensive integration tests that cover all functionality.

### Run all tests
```bash
cargo test
```

## Technical Details

### Architecture
- **Language**: Rust 2024 edition
- **Database**: SQLite with rusqlite crate
- **Structure**: Library + Binary architecture for testability

### Database Schema
```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
```

### Project Structure
```
todo-cli/
├── src/
│   ├── lib.rs          # Library with Todo struct and methods
│   └── main.rs         # CLI application entry point
├── tests/
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Project configuration
└── README.md           # This file
```

### Dependencies
- `rusqlite` (v0.36.0) - SQLite interface with bundled SQLite
- `clap` (v4.5) - Command line argument parser with derive feature

## Data Storage

- **Database file**: `todo.db` (created automatically in current directory)
- **Backup**: Simply copy the `todo.db` file to backup your data
- **Reset**: Delete `todo.db` to start fresh

## License

This project is open source and available under the [MIT License](LICENSE).
