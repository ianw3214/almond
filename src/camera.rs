pub struct Camera {
    pub x : f32,
    pub y : f32,
    pub zoom : f32
}

impl Camera {
    #[allow(dead_code)]
    pub fn screen_to_world_x(&self, x : i32) -> f32 {
        x as f32 + self.x
    }

    #[allow(dead_code)]
    pub fn screen_to_world_y(&self, y : i32) -> f32 {
        y as f32 + self.y
    }

    pub fn screen_to_world_x_i32(&self, x : i32) -> i32 {
        x + self.x as i32
    }

    pub fn screen_to_world_y_i32(&self, y : i32) -> i32 {
        y + self.y as i32
    }

    pub fn world_to_screen_x(&self, x : f32) -> i32 {
        (x - self.x) as i32
    }

    pub fn world_to_screen_y(&self, y : f32) -> i32 {
        (y - self.y) as i32
    }
}