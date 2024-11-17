**Note:**
This project, **Syncly**, is currently under active development. Features, functionality, and documentation may change as the project evolves. Contributions and feedback are welcome!
---
# **Syncly - File Synchronizer CLI**

Effortlessly synchronize files and directories across devices.
**Syncly** simplifies file management, ensuring your data stays consistent and up-to-date across locations.

---

## **Features**
- **Initialize projects** with Syncly for seamless synchronization.
- **File synchronization** between source and target directories.
- **Status tracking** for pending changes.
- **Configurable options** to exclude files, apply sync rules, and customize behavior.
- **Synchronization history** to track past changes.
- **Reset functionality** to clear saved states and start fresh.
- **Dry-run support** to preview changes before applying them.
- **Recursive sync** for nested directories.
- **Verbose logging** for detailed insights.

---

## **Installation**

### **Using Cargo**
```bash
cargo install syncly
```

### **From Source**
1. Clone the repository:
    ```bash
    git clone https://github.com/ArshilHapani/syncly.git
    cd syncly
    ```
2. Build and install:
    ```bash
    cargo build --release
    ```
3. Add the binary to your PATH:
    ```bash
    export PATH=$PATH:$(pwd)/target/release
    ```

---

## **Usage**
Here’s the above CLI structure in a tabular format for better readability:

---

### **Commands**

| Command   | Description                                                             |
|-----------|-------------------------------------------------------------------------|
| `init`    | Initialize Syncly in the current directory.                             |
| `sync`    | Synchronize files between the source and target directories.            |
| `status`  | Show the synchronization status and pending changes.                    |
| `config`  | Configure Syncly settings (e.g., sync rules, excluded files).           |
| `history` | View the synchronization history.                                       |
| `reset`   | Reset the synchronization state.                                        |
| `help`    | Display this help menu.                                                 |

---

### **Options**

| Option             | Description                                                                 |
|---------------------|-----------------------------------------------------------------------------|
| `-s`, `--source <path>`  | Specify the source directory for synchronization.                     |
| `-t`, `--target <path>`  | Specify the target directory for synchronization.                     |
| `-e`, `--exclude <list>` | Exclude specific files or directories (comma-separated).              |
| `-r`, `--recursive`      | Synchronize directories recursively.                                 |
| `-d`, `--dry-run`        | Simulate synchronization without making changes.                    |
| `-v`, `--verbose`        | Enable verbose output for detailed logs.                             |
| `-h`, `--help`           | Display this help menu.                                              |

---


## **Examples**

### **Initialize Syncly**
```bash
syncly init
```

### **Synchronize Files**
```bash
syncly sync -s /path/to/source -t /path/to/target
```

### **View Sync Status**
```bash
syncly status
```

### **Configure Exclusions**
```bash
syncly config --exclude "node_modules,temp,.git"
```

### **Reset Synchronization State**
```bash
syncly reset
```

---

## **Contributing**

We welcome contributions to make **Syncly** even better! Follow these steps:

1. Fork the repository.
2. Create a new branch:
    ```bash
    git checkout -b feature/your-feature
    ```
3. Commit your changes:
    ```bash
    git commit -m "Add your feature"
    ```
4. Push your branch:
    ```bash
    git push origin feature/your-feature
    ```
5. Open a Pull Request.

---

## **Support**

- **Documentation**: [Syncly Wiki](https://github.com/ArshilHapani/syncly/wiki)
- **Issues**: [Report an Issue](https://github.com/ArshilHapani/syncly/issues)

---

## **License**

Syncly is licensed under the [MIT License](https://opensource.org/licenses/MIT).
Feel free to use, modify, and distribute it as per the terms of the license.

---

**Happy Synchronizing! 🚀**
Keep your files consistent with Syncly!
