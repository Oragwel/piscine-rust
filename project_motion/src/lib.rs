#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        // If already on the ground, always return None
        if self.actual_position.y <= 0.0 && self.time != 0.0 {
            return None;
        }

        // Increase time by 1 second
        self.time += 1.0;

        let t = self.time;
        let gravity = -9.8_f32;

        // Calculate new position
        let new_x = self.init_position.x + self.init_velocity.x * t;
        let new_y = self.init_position.y + self.init_velocity.y * t + 0.5 * gravity * t * t;

        // Calculate new velocity
        let new_vx = self.init_velocity.x;
        let new_vy = self.init_velocity.y + gravity * t;

        // Update positions
        self.actual_position.x = new_x;
        self.actual_position.y = new_y;
        self.actual_velocity.x = new_vx;
        self.actual_velocity.y = new_vy;

        // If after updating the y-position it is now below the floor (y <= 0), cap it to zero
        if self.actual_position.y <= 0.0 {
            self.actual_position.y = 0.0;
            self.actual_velocity.y = 0.0;
            return None;
        }

        Some(self.clone())
    }
}
