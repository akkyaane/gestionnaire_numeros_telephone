# Phone Number Trie

A small Rust project that loads phone numbers from a JSON file, stores them in a
trie (prefix tree), and exports the structure as a PlantUML MindMap.
Numbers with the same prefix share the same nodes.

## Requirements

- Rust and Cargo (Rust 1.85 or newer, for edition 2024).
- A PlantUML renderer to view the diagram, such as the PlantUML VS Code extension
	or the Docker server described below. It is not needed to run the Rust program.

## Run

Run commands from the project root. With no argument, the program uses
`data/annuaire1.json`:

```sh
cargo run
```

To choose an input file:

```sh
cargo run -- data/annuaire2.json
cargo run -- data/annuaire3.json
```


## Input Files

| File | Contents |
| --- | --- |
| `data/annuaire1.json` | Original directory with 30 contacts |
| `data/annuaire2.json` | Five contacts from the assignment's example diagram |
| `data/annuaire3.json` | Simple example: Police (17) and Pompiers (18) |

Each JSON file contains an array of contacts:

```json
[
		{ "phone": "15", "name": "SAMU" },
		{ "phone": "17", "name": "Police" }
]
```

Phone numbers must be strings to preserve leading zeros. Use digits only,
without international prefixes such as `+33`.

## Output

The program prints the trie in the terminal and generates
`exemple/annuaire.puml`. The folder is created automatically, and each run
overwrites the previous diagram.

The MindMap contains one node per digit and a separate node for each contact's
name. The Rust program does not launch PlantUML.

To view the diagram in VS Code, install the PlantUML extension by jebbs, open
the generated file, and press `Alt + D`. Local rendering requires Java.

Alternatively, start a PlantUML server with Docker.


## Tests

```sh
cargo test
```