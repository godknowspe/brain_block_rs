# Brain Block Rust Rewrite - Design Document

## 專案概述 (Project Overview)
本專案是將原本用 Python (dlx + matplotlib) 撰寫的 Brain Block 益智遊戲解題器，改寫為效能更好、可移植性更高的 Rust 版本。

## 階段規劃 (Phases)

### Phase 1: 核心演算法與 CLI (Core Algorithm & CLI)
* **目標**: 實作精確覆蓋問題 (Exact Cover Problem) 的解題核心 (Dancing Links / DLX)。
* **組件**:
  * `shape.rs`: 拼圖形狀定義、旋轉 (Rotate)、翻轉 (Mirror)、平移 (Translate) 的幾何操作。
  * `dlx.rs` 或使用開源 crate (如 `dancing_links`)：實作 DLX 演算法。
  * `board.rs`: 棋盤狀態管理與座標系統。
  * `cli.rs`: 終端機介面，讀取設定並輸出解答 (ASCII 呈現)。

### Phase 2: 互動式圖形介面 (Interactive GUI)
* **目標**: 取代原先的 `matplotlib`，提供流暢的拖曳與點擊操作體驗。
* **技術選擇**: 使用 `eframe` (`egui`)，因為其為 Immediate Mode GUI，開發最為快速直覺且原生效能極佳。
* **功能**:
  * 繪製拼圖與棋盤。
  * 支援滑鼠點擊拖曳 (Drag & Drop) 預置拼圖。
  * 即時呼叫背景解題器並視覺化呈現結果。

### Phase 3: 效能最佳化與進階功能
* **目標**: 針對桌面版原生環境進行多執行緒最佳化、UX 提升。
* **細節**: 不需考慮 Web 部署，專注於原生渲染效能與高效率的操作回饋。
