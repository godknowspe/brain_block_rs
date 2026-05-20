# Brain Block Puzzle Solver (Rust) 🧩🦀

A highly optimized, blazingly fast Exact Cover solver for the classic "Brain Block" puzzle. This project is entirely rewritten in Rust and features a lightning-fast Dancing Links (DLX) backend paired with a buttery-smooth `egui` native graphical interface.

![Screenshot](docs/screenshot.png) *(Preview placeholder)*

## ✨ Features

- **Blazing Fast DLX Engine**: Leverages Knuth's Algorithm X (Dancing Links) via the `dlx-rs` crate to solve complex puzzle configurations in milliseconds.
- **Multithreaded Architecture**: The UI never freezes. Solving operations are safely offloaded to background threads.
- **Multiple Solutions**: Instantly calculates up to 10 valid permutations at once.
- **Interactive GUI**:
  - Drag and drop (click to pick up, click to place).
  - Pre-place pieces onto the board to constrain the solver dynamically.
  - Rotate (`R`) and Mirror (`M`) pieces in mid-air.
  - Interactive "Inventory" tracking unused pieces.
- **Cycle Through Answers**: Use the built-in `<` `>` pagination controls to view different layout solutions.
- **Multiple Puzzle Sets**: Supports the classic `8x5` layout, the massive `10x6` set, and a simple `3x2` set for testing.

## 🚀 Getting Started

### Prerequisites

You need the Rust toolchain installed. If you don't have it, install it from [rustup.rs](https://rustup.rs/):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build & Run

Clone the repository and run it using Cargo:

```bash
git clone https://github.com/godknowspe/brain_block_rs.git
cd brain_block_rs
cargo run --release
```
*(We recommend running in `--release` mode for maximum solver performance!)*

## 🎮 Controls

* **Left Click (Inventory)**: Pick up a piece.
* **Left Click (Board)**: Place the held piece down onto the grid.
* **Right Click (Board)**: Remove a placed piece, sending it back to your inventory.
* **`R` Key**: Rotate the held piece 90° clockwise.
* **`M` Key**: Mirror (flip) the held piece horizontally.
* **`Escape` Key**: Drop the currently held piece.

## 🏗️ Technical Architecture

This project maps the 2D spatial puzzle into an **Exact Cover** constraint matrix:
1. **Piece Constraint**: Each shape can only be used exactly once.
2. **Cell Constraint**: Each physical coordinate on the grid must be covered exactly once.

The UI is built with `egui` via `eframe`, using an Immediate Mode paradigm that keeps UI state management remarkably lightweight.
