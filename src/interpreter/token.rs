#[derive(Debug, Clone)]
pub(crate) enum Token {
    Left,
    Right,
    Plus,
    Minus,
    Output,
    Input,
    EnterLoop,
    ExitLoop,
}
