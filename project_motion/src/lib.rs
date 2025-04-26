// Structure representing a point or a velocity in 2D space (x, y coordinates)
#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

// Structure representing a thrown object with its motion data
#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,    // Initial position when the throw starts
    pub init_velocity: Object,    // Initial velocity at the start
    pub actual_position: Object,  // Current position after elapsed time
    pub actual_velocity: Object,  // Current velocity after elapsed time
    pub time: f32,                // Elapsed time in seconds
}

impl ThrowObject {
    // Creates a new ThrowObject given an initial position and initial velocity
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position.clone(),
            actual_velocity: init_velocity.clone(),
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    // Advances the object's state by 1 second each time .next() is called
    fn next(&mut self) -> Option<ThrowObject> {
        // Increase time by 1 second
        self.time += 1.0;

        // Calculate new position using kinematic equations:
        // p = p0 + v0 * t + (1/2) * a * t² (gravity acts downward)
        let actual_distance = Object {
            x: round_two(self.init_position.x + self.init_velocity.x * self.time),
            y: round_two(
                self.init_position.y + self.init_velocity.y * self.time
                    - 9.8 * self.time * self.time / 2.0, // gravity effect
            ),
        };

        // Calculate new velocity:
        // v = v0 + a * t
        let actual_velocity = Object {
            x: round_two(self.init_velocity.x),
            y: round_two(self.init_velocity.y - 9.8 * self.time),
        };

        // If the object has fallen below the ground (y < 0), stop the iteration
        if actual_distance.y < 0.0 {
            return None;
        } else {
            // Otherwise, update the object's state and continue
            return Some(ThrowObject {
                init_position: self.init_position.clone(),
                init_velocity: self.init_velocity.clone(),
                actual_position: actual_distance,
                actual_velocity: actual_velocity,
                time: self.time,
            });
        }
    }
}

// Helper function to round a floating point number to two decimal places
fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}
