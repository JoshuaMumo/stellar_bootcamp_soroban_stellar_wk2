# `Soroban Todo List Smart Contract`

This contract allows users to manage a todo list on the Stellar blockchain. Each todo item has a title, description and completion status. The contract supports full CRUD operations — create, read, update, complete and delete.

## `Project Structure`
 
```
.
├── src/
│   ├── lib.rs          # Crate root
|   |── test.rs         # Test cases
│   └── todo.rs         # Contract logic (TodoList, Todo struct)
├── Cargo.toml
└── README.md
```

## `Contract Functions`
 
### `create a todo list`

Creates a new todo item and saves it to the ledger.
 
- Auto-assigns the next available ID
- Sets `is_completed` to `false` by default
- Returns the newly created `Todo`
---
 
### `update a todo list`

Updates the title and description of an existing todo by ID.
 
- Finds the todo by `id` and overwrites `title` and `description`
- Does not alter `is_completed`
- Returns `true` if the todo was found and updated, `false` otherwise
---
 
### `todo list marked as completed `

Marks an existing todo as completed.
 
- Flips `is_completed` to `true`
- Does not alter `title` or `description`
- Returns `true` if the todo was found and updated, `false` otherwise
---
 
### `delete a todo list`

Permanently removes a todo from the ledger.
 
- Rebuilds the todo list excluding the matched item
- Returns `true` if the todo was found and deleted, `false` otherwise
---
 
## `Running Tests`
 
```bash
cargo test
```