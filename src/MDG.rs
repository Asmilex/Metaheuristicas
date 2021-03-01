pub struct MDG_info {
    n: u8,
    m: u8,
}

impl MDG_info {
    pub fn lineas_archivo(&self) -> u64 {
        self.n * (self.n - 1) / 2
    }
}