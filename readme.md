# Kernel for the Genau Environment  

`genauai-kernel` is a Rust library for building generative AI tools based on human feedback . It provides a set of APIs for managing conversations, saving and loading plans, and more.

# Features

* Conversations: start, stop, and manage conversations with users
* Plans: create, save, and load plans for your chatbot
* Messages: save and retrieve messages for analysis and reporting
* Database: provides an SQLite database interface for easy data management

# Installation

Add `genauai-kernel` to your project's Cargo.toml file:

```toml
[dependencies]
genauai-kernel = "0.0.1"
```

# Getting Started

```rust

use genauai_kernel::{get_db, get_plan, reset_database, save_message, save_plan};
use rusqlite::Connection;

fn main() {
    let conn = get_db();
    let plan = get_plan(&conn).unwrap();

    let message = "Hello, world!";
    save_message(&conn, 1, "user", &message).unwrap();

    let new_plan = Plan::default();
    save_plan(&conn, 1, &new_plan).unwrap();

    reset_database(&conn).unwrap();
}
```

# Contributing

Contributions are welcome! If you find a bug or want to suggest a new feature, please open an issue or submit a pull request.

# License

`genauai-kernel` is licensed under the MIT License. See LICENSE for details.