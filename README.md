# RustLink 🦀

**RustLink** is a high-performance, lightweight, and modular audio server implementation written in **Rust**. It is designed to be a compatible alternative to Lavalink (and NodeLink), providing a robust API for Discord bots to play music from various sources.

> ⚠️ **Status: Experimental / Work In Progress**
> This project is currently in active development. While the core API structure and architecture are in place, some audio processing features are still being implemented.

## 🚀 Features

*   **Lavalink v4 API Compatibility:** Implements the standard endpoints for player management, stats, and track decoding/encoding.
*   **High Performance:** Built on **Axum** and **Tokio** for asynchronous, non-blocking operations and low footprint.
*   **Modular Architecture:**
    *   **Audio Engine:** Decoupled worker logic for handling player states and playback commands.
    *   **Source Managers:** Extensible support for sources like YouTube, SoundCloud, and Spotify.
    *   **Filter System:** Foundation for real-time audio filters (Equalizer, Tremolo, Vibrato, etc.).
*   **Memory Safe:** Leverages Rust's ownership model to ensure safety and stability.

## 🛠️ Tech Stack

*   **Language:** Rust (Edition 2024)
*   **Web Framework:** Axum
*   **Async Runtime:** Tokio
*   **HTTP Client:** Reqwest
*   **Serialization:** Serde

## 📦 Installation & Usage

### Prerequisites

*   [Rust Toolchain](https://www.rust-lang.org/tools/install) (Latest Stable)

### Running the Server

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/yourusername/rustlink.git
    cd rustlink
    ```

2.  **Build and Run:**
    ```bash
    # Run in development mode
    cargo run

    # Run in release mode (recommended for production)
    cargo run --release
    ```

The server will start on `0.0.0.0:8080` (default).

## 📂 Project Structure

*   `src/api`: HTTP API route handlers (Lavalink v4 implementation).
*   `src/managers`: Core logic for sessions, sources, connections, and stats.
*   `src/playback`: Audio engine, player logic, and stream processing.
*   `src/sources`: Implementations for specific audio sources (YouTube, HTTP, etc.).
*   `src/utils`: Helper functions for decoding/encoding tracks and other utilities.

## 🤝 Contributing

Contributions are welcome! If you're interested in helping port features from NodeLink or optimizing the Rust implementation, please feel free to:

1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feature/amazing-feature`).
3.  Commit your changes (`git commit -m 'Add some amazing feature'`).
4.  Push to the branch (`git push origin feature/amazing-feature`).
5.  Open a Pull Request.

## 📄 License

[MIT](LICENSE) (or your preferred license)
