//! Экспериментальный парсер Markdown

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Header,
    Code,
    Text(TextType),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TextType {
    Monospace,
    Bold,
    Italic,
    Default,
    NewLine,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub kind: TokenKind,
}
