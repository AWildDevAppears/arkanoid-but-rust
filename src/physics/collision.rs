#[derive(Debug, Clone, Copy, Default)]
pub enum CollisionGroup {
    #[default]
    None,
    HeightBoundary,
    WidthBoundary,
    Pill,
    Ball,
    Paddle,
}

struct CollisionBounds {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CollisionBox {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub group: CollisionGroup,
}

impl CollisionBox {
    pub fn get_bounds(&self) -> CollisionBounds {
        return CollisionBounds { x1: self.x, y1: self.y, x2: self.x + self.width, y2: self.y + self.height };
    }

}

pub struct CollisionState {
    pub side_boundaries: [CollisionBox; 4],
    pub ball_collider: CollisionBox,
}

impl Default for CollisionState {
    fn default() -> Self {
        Self {
            side_boundaries: [CollisionBox::default(); 4],
            ball_collider: CollisionBox::default()
        }
    }
}

impl CollisionState {
    pub fn attach_boundaries(&mut self, width: i32, height: i32) {
        self.side_boundaries = [
            CollisionBox {
                x: 0.0,
                y: 0.0,
                height: height as f32,
                width: 1.0,
                group: CollisionGroup::WidthBoundary,
            },
            CollisionBox {
                x: width as f32,
                y: 0.0,
                height: height as f32,
                width: 1.0,
                group: CollisionGroup::WidthBoundary,
            },

            CollisionBox {
                x: 0.0,
                y: 0.0,
                height: 1.0,
                width: width as f32,
                group: CollisionGroup::HeightBoundary,
            },
            CollisionBox {
                x: 0.0,
                y: height as f32,
                height: 1.0,
                width: width as f32,
                group: CollisionGroup::HeightBoundary,
            },
        ];
    }

    pub fn update_ball_collider(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.ball_collider = CollisionBox { x, y, height, width, group: CollisionGroup::Ball }
    }

    fn is_colliding(&self, collider1: CollisionBox, collider2: CollisionBox) -> bool {
        let bounds1 = collider1.get_bounds();
        let bounds2 = collider2.get_bounds();

        return bounds1.x1 < bounds2.x2 &&
            bounds1.x2 > bounds2.x1 &&
            bounds1.y1 < bounds2.y2 &&
            bounds1.y2 > bounds2.y1;
    }
}


