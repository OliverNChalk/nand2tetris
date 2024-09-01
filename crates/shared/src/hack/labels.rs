#[derive(Default)]
pub struct Labels {
    count: u32,
}

impl Labels {
    pub fn next(&mut self) -> String {
        self.count += 1;

        format!("LOW_LEVEL_LABEL_{}", self.count)
    }
}
