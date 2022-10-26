#[derive(Clone)]
pub struct CountStateHandler {
    pub include_self: bool,
    pub range_offset: isize,
    pub valid_counts: Vec<usize>,
    pub valid_state: bool,
}

impl Default for CountStateHandler {
    fn default() -> Self {
        CountStateHandler {
            include_self: true,
            range_offset: 2,
            valid_counts: vec![2, 4],
            valid_state: true,
        }
    }
}
