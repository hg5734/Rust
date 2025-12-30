## 🦀 Rust Backend Learning & Projects

This repository documents my journey to mastering **high-performance backend development in Rust**, with a focus on:

- **Safety & Concurrency**
- **Asynchronous I/O**
- **Scalable and maintainable design**

---

## 🧱 Core Rust Fundamentals

- **Ownership, Borrowing & Lifetimes**
- **Structs, Enums & Pattern Matching**
- **Traits & Trait Bounds**
- **Generics and Associated Types**
- **Modules & Project Structure**
- **Error Handling**
  - `Result`, `Option`, and the `?` operator
  - Custom error types (e.g. using the `thiserror` crate)
- **Iterators & Closures**
- **Smart Pointers**
  - `Box`, `Rc`, `Arc`, `RefCell`

---

## ⚙️ Concurrency & Async Programming

- **Concurrency Primitives**
  - Threads and channels
  - `Mutex` / `RwLock`
- **Async & Futures**
  - `async` / `await` and `Future` fundamentals
  - Using the **Tokio runtime** for concurrency
- **Task Management**
  - Spawning and joining tasks (`JoinHandle`, `join!`)
- **Practical Patterns**
  - Retry logic with backoff using async tasks
  - Concurrent HTTP requests with `reqwest`
  - Collecting results into `Vec<Result<T, E>>`

---

## 🌐 Web & Networking

- **Web Frameworks**
  - Built REST APIs using **Actix Web**
- **HTTP Clients**
  - Async HTTP client with **Reqwest**
- **Data Handling in Web Context**
  - JSON serialization/deserialization with `serde`
- **Project Example**
  - Actix Web server making concurrent API calls using Reqwest
  - Custom error propagation and response handling

---

## 💾 File & Data Handling

- **Async File I/O**
  - Reading and writing files asynchronously
- **Data Formats**
  - JSON, CSV, YAML parsing using `serde`, `csv`, and related crates
- **In-Memory Storage**
  - Basic in-memory KV store using `HashMap` and `RwLock`

---

## 🧠 Advanced Rust Topics

- **Trait Objects & Dynamic Dispatch**
- **Complex Generics & Lifetimes**
- **Async Traits**
  - Implemented via the `async-trait` crate
- **Futures Internals**
  - `Pin` / `Unpin` concepts
- **Testing & Benchmarking**
  - Async tests with `#[tokio::test]`
  - Mocking async functions
  - Benchmarks using the `criterion` crate

---

## 🔒 Blockchain & System-Level Concepts

- **Cryptographic Primitives**
  - Hashing and signing basics
- **Smart Contract-Oriented Design**
  - Ownership and safety considerations in smart contract development
- **Integration**
  - Practiced integration with EVM and Web3-related libraries

---

## 📁 Example Mini Projects

- **Async Downloader**
  - Concurrent URL fetching
- **Retryable Network Task**
  - Exponential backoff and error handling
- **Multi-Task Executor**
  - Aggregating `Vec<Result<T, E>>` from multiple async tasks
- **Actix Web API**
  - REST API with external API integration
- **In-Memory KV Store**
  - Async locks for safe concurrent access

---

## 🧰 Tooling & Ecosystem

- **Core Tools**
  - `cargo` – project & dependency management
  - `clippy` – linting
  - `rustfmt` – formatting
  - `cargo fix`, `cargo check` – automated fixes & static checks
- **Debugging & Profiling**
  - `RUST_BACKTRACE` – debugging panics
  - `tokio-console` – inspect async tasks
  - `bacon` – continuous compile/check
  - `hyperfine` – benchmarking CLI commands
  - `flamegraph` – CPU profiling
  - `dhat` – heap profiling
  - `oha` – HTTP load testing
- **Parallelism & Data Structures**
  - `rayon` – data-parallelism
  - `dash-map` – concurrent hash map

---

## 🧑‍💻 Tech Stack

- `Rust`
- `Tokio`
- `Actix Web`
- `Reqwest`
- `Serde`
- `thiserror`
- `async-trait`
- `chrono`
- `sqlx`

---

## 🔗 Useful References

- **Official Rust Book**
  - [Generics](https://doc.rust-lang.org/book/ch10-00-generics.html)
  - [Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)
  - [Final Project: Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
- **Futures**
  - [`Future` trait](https://doc.rust-lang.org/std/future/trait.Future.html)
- **Key Libraries**
  - [Actix Web](https://actix.rs/) – extremely fast HTTP server
  - [Serde](https://serde.rs/) – data serialization/deserialization
  - [Tokio](https://tokio.rs/) – async runtime
  - [`reqwest` docs](https://docs.rs/reqwest/latest/reqwest/) – HTTP client
  - [`sqlx` docs](https://docs.rs/sqlx/latest/sqlx/) – async SQL toolkit
- **Learning Resources**
  - Rust Bootcamp PDF: <https://cdn.100xdevs.com/youtube/rust-part-2.pdf>
  - Rust Bootcamp Track: <https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-23>

---

## 📝 Error Handling & Runtime Practices

- **Error Handling Techniques**
  - Error propagation with the `?` operator
  - Custom error types for cleaner APIs
- **Runtime Practices**
  - Using `cargo fix`, `cargo fmt`, and `cargo check` as part of the regular workflow
