use enigo::*;

pub struct Bot {
    enigo: Enigo,
    screen_config: ScreenConfig
}

impl Bot {
    pub fn new(screen_config: ScreenConfig) -> Self {
        Bot {
            enigo: Enigo::new(),
            screen_config: screen_config
        }
    }

    pub fn place_ore(&mut self, x: usize , y: usize) {
        let coords = self.screen_config.to_screen([x, y]);
        self.enigo.mouse_move_to(coords[0], coords[1]);
        self.enigo.mouse_click(MouseButton::Left);
    }
}

pub struct ScreenConfig {
    cell_size: f32,
    zero_point: [f32; 2]
}

impl ScreenConfig {
    pub fn default() -> Self {
        ScreenConfig {
            cell_size: 75.0,
            zero_point: [0.0, 413.0]
        }
    }
    fn to_screen(&self, from: [usize; 2]) -> [i32; 2] {
        return [
            (self.zero_point[0] + (from[0] as f32 + 0.5) * self.cell_size) as i32,
            (self.zero_point[1] + (from[1] as f32 + 0.5) * self.cell_size) as i32
        ];
    }
}
