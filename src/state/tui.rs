use super::InputMode;

#[derive(Default)]
pub struct TuiState {
    pub input_mode: InputMode,
    pub cursor: Option<(u16, u16)>,
}
