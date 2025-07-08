use macroquad::prelude::*;

// Our Rects are using the top left corner for x and y pos
pub struct RectCollider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl RectCollider {
    
    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.x &&
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height
    }

    fn intersects(&self, other: &RectCollider) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y

    }

    fn debug_draw(&self) {
        let debug_color = Color::new(255.0, 0.0, 255.0, 0.5);
        draw_rectangle(self.x, self.y, self.width, self.height, debug_color);
    }
}

pub struct CircleCollider {
    pub center: Vec2,
    pub radius: f32,
}

impl CircleCollider {
    fn contains_point(&self, point: Vec2) -> bool {
        self.center.distance(point) <= self.radius
    }

    fn intersects(&self, other: &CircleCollider) -> bool {
        let dist = self.center.distance(other.center);
        dist <= (self.radius + other.radius)     
    }

    fn debug_draw(&self) {
        let debug_color = Color::new(255.0, 0.0, 255.0, 0.5);
        draw_circle(self.center.x,self.center.y, self.radius, debug_color);
    }
}