# rs_mqtt

## 專案簡介
這是一個使用 Rust 開發的 MQTT 客戶端，實現了以下功能：
- 訂閱指定的 MQTT 主題。
- 每 5 秒發佈一次 CPU 使用率的訊息。

## 使用的技術與套件
- **paho-mqtt**: 用於 MQTT 通訊。
- **sysinfo**: 用於獲取系統資訊（如 CPU 使用率）。
- **serde_json**: 用於 JSON 序列化與反序列化。
- **ctrlc**: 用於處理終止信號。

## 開發過程中遇到的問題

### 1. Rust 環境設置
- **問題**: 初始環境中未安裝 Rust 工具鏈。
- **解決方法**: 使用 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 安裝 Rust。

### 2. 缺少必要的依賴項
- **問題**: 編譯時出現未解析的模組錯誤，例如 `paho-mqtt`、`sysinfo` 等。
- **解決方法**: 在 `Cargo.toml` 中新增以下依賴項：
  ```toml
  [dependencies]
  paho-mqtt = "0.11.0"
  serde_json = "1.0"
  sysinfo = "0.29.0"
  ctrlc = "3.3.3"
  ```

### 3. CMake 錯誤
- **問題**: 編譯 `paho-mqtt-sys` 時出現 `CMake` 錯誤，提示 `CMakeLists.txt` 的版本要求不符合。
- **解決方法**:
  1. 確保已安裝 `CMake`，並且版本高於 3.5。
  2. 使用 Homebrew 升級 CMake：`brew upgrade cmake`。

### 4. OpenSSL 路徑問題
- **問題**: `paho-mqtt-sys` 嘗試使用 OpenSSL，但無法正確找到路徑。
- **解決方法**: 設置環境變數 `OPENSSL_ROOT_DIR`：
  ```bash
  export OPENSSL_ROOT_DIR=$(brew --prefix openssl)
  ```

### 5. 子模組下載問題
- **問題**: `paho.mqtt.c` 的原始碼未正確下載，導致 `fatal: not a git repository` 錯誤。
- **解決方法**: 手動下載 `paho.mqtt.c` 原始碼：
  ```bash
  git clone https://github.com/eclipse/paho.mqtt.c.git
  cd paho.mqtt.c
  git checkout v1.3.9
  ```

### 6. 目前狀態
- **問題**: 雖然已嘗試多種方法，但仍無法成功編譯專案。
- **原因**: 可能與 `paho-mqtt-sys` 的建置腳本或環境配置有關，具體原因尚未確定。

## 待解決問題
- 繼續調查 `paho-mqtt-sys` 的建置過程，確保所有依賴項與環境變數正確配置。
- 嘗試使用其他 MQTT 客戶端庫作為替代方案。

## 如何執行
1. 確保已安裝 Rust 工具鏈。
2. 安裝必要的系統工具：
   ```bash
   brew install cmake openssl
   ```
3. 編譯專案：
   ```bash
   cargo build
   ```
4. 執行專案：
   ```bash
   cargo run
   ```