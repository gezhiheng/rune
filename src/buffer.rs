//! 阶段 1 的核心。字段和函数体由你写。

/// 一段文本，加上一个光标。
///
/// 光标是字符偏移，不是字节偏移。
/// `0` 在第一个字符前面。`n` 在第 `n` 个字符后面。
pub struct Buffer {
    // 你来决定字段。
}

impl Buffer {
    /// 空文本，光标在 `0`。
    pub fn new() -> Self {
        todo!("阶段 1")
    }

    /// 用已有文本和光标构造。
    ///
    /// `cursor` 按字符计数。
    /// `cursor` 大于字符个数时 panic。
    pub fn from_text(text: &str, cursor: usize) -> Self {
        let _ = (text, cursor);
        todo!("阶段 1")
    }

    /// 整个文档。
    pub fn text(&self) -> String {
        todo!("阶段 1")
    }

    /// 当前光标，字符偏移。
    pub fn cursor(&self) -> usize {
        todo!("阶段 1")
    }

    /// 在光标处插入 `ch`，然后把光标向前移一个字符。
    pub fn insert_char(&mut self, ch: char) {
        let _ = ch;
        todo!("阶段 1")
    }
}
