# Locus

A fast, efficient, and keyboard-driven note-taking and todo management application that runs entirely in your terminal. Built with Rust 🦀 and `ratatui`.

## Features

* **Vim-Inspired Navigation**: Use `j` and `k` to navigate lists and `Esc` to exit modes.
* **Dual Functionality**: Manage both persistent notes and actionable todo items in separate sections.
* **Advanced Todo Management**: Todos support completion status, severity levels, and due dates to help you prioritize.
* **Data Persistence**: Your notes and todos are automatically saved to `~/.terminal_notes/data.json`.
* **Safe Quit**: The app warns you about unsaved changes before quitting.
* **Data Portability**: Easily back up your data or export it to Markdown and CSV formats.

## Installation & Setup

Before you begin, ensure you have the **Rust toolchain** installed on your system. If you don't, you can install it from the [official Rust website](https://rustup.rs/).

1. **Clone the repository**:
   ```bash
   git clone https://github.com/thesfb/Locus.git
   cd Locus
   ```

2. **Build the project**: Cargo will handle all the dependencies for you.
   ```bash
   cargo build --release
   ```

## How to Run 🚀

To run the application, navigate to the project's root directory and use the following command:

```bash
cargo run
```

This will compile and launch the application.

## Commands and Keybindings

The application operates in several modes, primarily **Normal Mode** (for navigation) and **Command Mode** (for executing commands).

### General Navigation

| Key | Action |
|-----|--------|
| `j` / `Down` | Move down in a list |
| `k` / `Up` | Move up in a list |
| `Enter` | Select an item or enter **Editing Mode** |
| `Esc` | Exit the current mode (e.g., Editing, Help) |
| `Spacebar` | (In Todos) Toggle an item's completion status |
| `Ctrl` + `Q` | Quit the application |

### Command Mode

Press `:` in **Normal Mode** to enter **Command Mode**.

| Command | Description |
|---------|-------------|
| `[n]nn` | Create `[n]` new notes (e.g., `2nn`) |
| `[n]ntodo` | Create `[n]` new todos (e.g., `3ntodo`) |
| `[n]del` | Delete the selected item(s) |
| `rnm` | Rename the currently selected note or todo |
| `mm` | Return to the Main Menu |
| `?` | Show the help screen |
| `save` or `w` | Save all changes to disk |
| `backup` | Create a timestamped backup of your data file |
| `export-md` | Export notes and todos to a Markdown file |
| `export-csv` | Export notes and todos to a CSV file |
| `q` or `quit` | Quit the application (will warn if unsaved) |
| `q!` | Force quit without saving |

## Data Storage

Your notes and todos are stored in `~/.terminal_notes/data.json`. This file is created automatically when you first run the application.

## Export Options

Terminal Notes supports multiple export formats:
- **Markdown**: Perfect for documentation and sharing
- **CSV**: Compatible with spreadsheet applications
- **Backup**: JSON format for complete data preservation

## Contributing

Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests.



---

Built with ❤️ using Rust and ratatui