# Zenoh Topic Explorer

**Zenoh Topic Explorer** is a real-time graphical tool to explore and monitor topics and messages in a [Zenoh](https://zenoh.io/) network. Inspired by MQTT Explorer, it provides a tree-based view of all published topics and displays message histories in an intuitive interface.

---

## What It Does

- Subscribes to **all topics** in a Zenoh network (`**` wildcard).
- Builds a **live topic tree** that updates as new topics are discovered.
- Displays the **history of messages** received per topic (with timestamps).
- Offers a **collapsible UI** to navigate deeply nested topic structures.
- Can be configured using a `JSON5` Zenoh configuration file.

---

## Installation

### Clone and Build

```bash
git clone https://github.com/zisraw/zenoh-explorer.git
cd zenoh-explorer
cargo build --release
```

## Run

### Default session
```bash
cargo run --release
```

### With configuration file
```bash
cargo run -- path/to/config.json5 --release
```
