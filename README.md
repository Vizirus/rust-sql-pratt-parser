# Rust SQL Pratt Parser

A zero-dependency SQL parser implemented in **pure Rust** (`std` only, Edition 2021) using a custom lexical **tokenizer** and a **Pratt parser** (operator-precedence parsing) to transform SQL queries into structured AST statements. 

## Features

- **Custom Tokenizer (`tokenizer.rs`, `token.rs`):** Case-insensitive keyword recognition and lexical scanning of identifiers, literals, operators, and punctuation.
- **Pratt Expression Parser (`parser.rs`):** Operator-precedence parsing for arithmetic (`+`, `-`, `*`, `/`) and comparison (`<`, `>`, `<=`, `>=`) expressions.
- **SQL Statement Support (`statement.rs`, `ParsingStatments.rs`):**
  - `SELECT` statements with `FROM`, optional `WHERE`, and `ORDER BY` (`ASC` / `DESC`) clauses
  - `CREATE TABLE` statements with basic column definitions and `CHECK` constraints
- **Unit Tests (`Tests.rs`):** Automated test coverage for tokenization and statement parsing.

## Tech Stack

- **Language:** Rust (Edition 2021)
- **Dependencies:** None (pure Rust standard library)

## Getting Started

```bash
# Run interactive CLI parser
cargo run

# Run unit tests
cargo test
```
