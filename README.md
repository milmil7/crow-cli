# 🪶 crow-cli

**crow-cli** is a modern, interactive, terminal-based HTTP client written in Rust using [ratatui](https://github.com/ratatui-org/ratatui). Think of it as a fast, keyboard-driven alternative to Postman or Insomnia, right in your terminal.

Made with ❤️ by [@milmil7](https://github.com/milmil7)

---

## ✨ Features

- 🌐 Send HTTP requests using all common methods: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`
- 🧭 Interactive tab-based navigation with keyboard shortcuts
- 🔐 Support for Authorization headers (`Bearer`, `Basic`)
- 🧾 Add custom headers and query parameters easily
- 🛠️ JSON body editor with syntax highlighting
- 🎨 Beautiful TUI layout powered by `ratatui`
- 📬 Real-time response preview with pretty-printed JSON
- 🧵 Async and fast thanks to `tokio` and `reqwest`

---

## 🚀 Getting Started

### 📦 Prerequisites

- Rust (1.74+ recommended)
- Cargo

### 🔧 Installation

```bash
git clone https://github.com/milmil7/crow-cli.git
cd crow-cli
cargo build --release
```

The binary will be located at: `target/release/crow-cli`

### 🦅 Launch

```bash
cargo run --release
```

---

## 🛠️ Controls

| Key         | Action                           |
|-------------|----------------------------------|
| `← / →`     | Switch between method & sections |
| `↑ / ↓`     | Move between sections            |
| `Tab`       | Switch between input fields      |
| `Enter`     | Send the request                 |
| `Esc`       | Exit                             |
| `Backspace` | Delete character in input        |

---

## 📚 Sections

- **Method**: Choose from GET, POST, PUT, etc.
- **URL**: Enter the full request URL
- **Params**: Add query parameters like `key1:val1,key2:val2`
- **Headers**: Add headers like `Content-Type:application/json`
- **Auth**: Choose between Bearer or Basic tokens
- **Body**: JSON payload (for POST/PUT/PATCH)
- **Response**: View the server's response

---

## 🔧 Dependencies

- [ratatui](https://github.com/ratatui-org/ratatui)
- [crossterm](https://github.com/crossterm-rs/crossterm)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [tokio](https://github.com/tokio-rs/tokio)
- [serde](https://github.com/serde-rs/serde)
- [jsonxf](https://github.com/LukeMathWalker/jsonxf)

---

## 🛤️ Roadmap

- [ ] Request history
- [ ] Save/load requests
- [ ] Export to `curl` or Postman collection
- [ ] Auto-completion for headers and common fields
- [ ] Response headers and status line view
- [ ] Environment/variables system like Postman

---

## 🤝 Contributing

Pull requests and ideas are welcome! Please open issues for bugs, improvements, or new feature requests.

---

## 📄 License

MIT License  
© 2025 [@milmil7](https://github.com/milmil7)

---

## 🌟 Acknowledgements

Thanks to the open-source community for amazing crates like `ratatui`, `crossterm`, and `reqwest` that power this app.