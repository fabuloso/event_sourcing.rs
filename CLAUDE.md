# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**esrs** (Event Sourcing.rs) is an opinionated Rust library for implementing CQRS/Event Sourcing patterns. It provides core traits and a PostgreSQL-backed persistence layer.

## Build and Test Commands

Development requires Docker for the full test environment:

```bash
# Start development environment (provides PostgreSQL, Kafka, RabbitMQ)
docker compose run --service-ports web bash

# Run tests (requires all services running)
cargo make test

# Run linting (includes format check)
cargo make clippy

# Run a specific test
cargo test --all-features test_name

# Run an example
cargo run --example readme --features=postgres

# Generate docs
cargo make docs
```

Feature-specific checks:
```bash
cargo check --features=postgres
cargo check --features=kafka
cargo check --features=rabbit
cargo check --all-features
```

## Architecture

### Core Traits

The library is built around four main traits in `src/`:

1. **`Aggregate`** (`aggregate.rs`) - The central abstraction defining:
   - `NAME`: Unique identifier linking aggregate instances to their events (changing breaks historical links)
   - `State`, `Command`, `Event`, `Error`, `Services`: Associated types
   - `handle_command()`: Async command validation and event emission. Receives `&Self::Services` for external collaborators
   - `apply_event()`: Synchronous state reconstruction from events (must remain pure, no side effects)

2. **`EventStore`** (`store/mod.rs`) - Persistence abstraction with `lock()`, `by_aggregate_id()`, `persist()`, `delete()` methods

3. **`EventHandler`** (`handler.rs`) - Eventually consistent event processor for read models and side effects (infallible)

4. **`TransactionalEventHandler`** (`handler.rs`) - Strongly consistent event processor running within the persistence transaction (avoid using on other aggregates - causes deadlocks)

### Key Components

- **`AggregateManager`** (`manager.rs`) - Command bus coupling aggregates with stores. Holds the aggregate's `Services` and passes them to `handle_command` internally. Two constructors: `new(store)` when `Services: Default` (including `()`), `with_services(store, services)` for custom collaborators. Returns nested `Result`: outer for technical errors, inner for domain errors
- **`PgStore`** (`store/postgres/`) - PostgreSQL EventStore implementation with automatic migrations. Each store is globally unique per Aggregate type
- **`StoreEvent`** (`store/mod.rs`) - Event wrapper with id, aggregate_id, payload, occurred_on, sequence_number, version

### Features

```toml
postgres   # PostgreSQL store + migrations (most examples need this)
kafka      # Kafka event bus
rabbit     # RabbitMQ event bus
rebuilder  # Projection rebuilding via ReplayableEventHandler
upcasting  # Event versioning utilities
```

### Schema Abstraction

`PgStore<A, Schema>` decouples domain events from persistence format via `Persistable` trait. See `examples/schema/` for event deprecation and upcasting patterns.

## Important Patterns

- **Double Result**: `handle_command()` returns `Result<Result<State, DomainError>, StoreError>` - technical errors outer, domain errors inner
- **Services (Collaborators)**: Aggregates declare `type Services = ();` when no external dependencies are needed. For aggregates that need external services (e.g. payment gateways), define a custom `Services` type and use `AggregateManager::with_services()`
- **Async handle_command, Sync apply_event**: `handle_command()` is async and can use services for side effects. `apply_event()` must remain synchronous and pure - it is used for event replay
- **ReplayableEventHandler**: Only idempotent handlers should implement this for rebuilding projections
- **Dynamic Stores**: Call `without_running_migrations()` on `PgStoreBuilder` when creating stores at runtime

## Code Style

- MSRV: 1.85.0
- Line length: 120 characters
- Warnings treated as errors in CI (`-D warnings`)
