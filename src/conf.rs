pub const FIELD_SIZE: (usize, usize) = (6, 5);


#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub cell_size: f32,
    pub zero_point: [f32; 2]
}

impl Config {
    pub fn default() -> Self {
        Config {
            cell_size: 75.0,
            zero_point: [0.0, 413.0]
        }
    }

    /// Transforms field coordinates to screen absolute coordinates.
    pub fn to_screen(&self, from: [usize; 2]) -> [i32; 2] {
        return [
            (self.zero_point[0] + (from[0] as f32 + 0.5) * self.cell_size) as i32,
            (self.zero_point[1] + (from[1] as f32 + 0.5) * self.cell_size) as i32
        ];
    }
}
