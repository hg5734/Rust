# 🦀 Rust Projects

A covering key fundamentals, async programming, web development, and advanced backend concepts.

---

## 🧱 Core Fundamentals
- Ownership, Borrowing & Lifetimes  
- Structs, Enums & Pattern Matching  
- Traits & Trait Bounds  
- Generics and Associated Types  
- Modules & Project Structure  
- Error Handling (`Result`, `Option`, `?` operator, `thiserror` crate)  
- Iterators & Closures  
- Smart Pointers (`Box`, `Rc`, `Arc`, `RefCell`)  

---

## ⚙️ Concurrency & Async Programming
- Threads, Channels, and `Mutex`/`RwLock` usage  
- Async/Await and `Future` fundamentals  
- Using **Tokio runtime** for concurrency  
- Spawning multiple async tasks and joining results (`JoinHandle`, `join!`)  
- Implemented retry logic with backoff using async tasks  
- Concurrent HTTP requests with `reqwest` and async collections  

---

## 🌐 Web & Networking
- Built REST APIs using **Actix Web**  
- Created async HTTP clients using **Reqwest**  
- Hands-on project:  
  - Actix Web server making concurrent API calls with Reqwest  
  - JSON serialization/deserialization with `serde`  
  - Error propagation and custom response handling  

---

## 💾 File & Data Handling
- Read/Write files asynchronously  
- JSON, CSV, and YAML parsing with `serde` and `csv` crates  
- Implemented basic in-memory data store using `HashMap` and `RwLock`  

---

## 🧠 Advanced Topics
- Trait objects and dynamic dispatch  
- Generic constraints and lifetimes in complex structs  
- Async traits (via `async-trait` crate)  
- Futures and Pin/Unpin concepts  
- Testing with `#[tokio::test]` and mocking async functions  
- Benchmarking with `criterion` crate  

---

## 🔒 Blockchain & System-Level
- Overview of cryptographic primitives (hashing, signing)  
- Understanding ownership and safety for smart contract development  
- Practiced integration with EVM and Web3 libraries  

---

## 🧰 Tooling & Ecosystem
- **Cargo** for project management  
- **Clippy** and **rustfmt** for linting and formatting  
- **Rust Analyzer** for IDE integration  
- Debugging async code using `RUST_BACKTRACE` and `tokio-console`  

---

## 📁 Example Mini Projects
- Async downloader (concurrent URL fetching)  
- Retryable network task (with exponential backoff)  
- Multi-task executor collecting results into a `Vec<Result<T, E>>`  
- Basic Actix Web API with external API integration  
- In-memory KV store using async locks  

---

## 🚀 Goal
To master **high-performance backend development in Rust**, focusing on:
- Safety + Concurrency
- Asynchronous I/O
- Scalable and maintainable design

---

## 🧑‍💻 Tech Stack
`Rust` • `Tokio` • `Actix Web` • `Reqwest` • `Serde` • `thiserror` • `async-trait`



## Some External library

chrono -> date time lib

https://actix.rs/
 – Extremely fast HTTP server

https://serde.rs/
 – Serializing and deserializing data in Rust

https://tokio.rs/
 – Asynchronous runtime in Rust

https://docs.rs/reqwest/latest/reqwest/
 – Send HTTP requests

https://docs.rs/sqlx/latest/sqlx/
 – Connect to SQL database


https://doc.rust-lang.org/book/ch10-00-generics.html
https://doc.rust-lang.org/book/ch19-00-advanced-features.html
https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
https://doc.rust-lang.org/std/future/trait.Future.html

https://cdn.100xdevs.com/youtube/rust-part-2.pdf

https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-23


Run Time Exp handling technique 
Error propagation — the ? operator
Custom error types