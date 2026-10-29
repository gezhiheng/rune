# 进度

## 已完成

阶段 0，2026-10-29。

- `cargo run -- --smoke` 打印 `rune ready`。
- 伪终端运行 `target/debug/rune`。输出里有 `Rune` 和 `press q to quit`。写入 `q` 后退出码 0，并离开备用屏幕。
- `cargo test --bin rune q_and_ctrl_c_quit` 通过。`should_quit` 改成恒 `false` 后该测试变红，已改回。

## 当前

阶段 1：插入一个字符。用户写 `src/buffer.rs` 的字段和函数体。

## 待解决

`cargo test --test buffer_insert` 失败。两个测试都在 `src/buffer.rs:23` panic：`not yet implemented: 阶段 1`。

## 下一步

用户完成阶段 1。验收通过前不进入阶段 2。

## 阶段 1 验收

手算：

- 空文档，光标 0。插入 `h`，得到 `h`，光标 1。再插入 `i`，得到 `hi`，光标 2。
- `from_text("ac", 1)` 后插入 `b`，得到 `abc`，光标 2。
- `from_text("aé", 1)` 后插入 `你`，得到 `a你é`，光标 2。

命令：`cargo test --test buffer_insert` 两个测试都通过。
