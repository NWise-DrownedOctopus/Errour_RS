use macroquad::prelude::*;

// This is the base collider trait, all specific colliders will implement this
pub trait Collider {
    fn contains_point(&self, point:Vec2) -> bool;
    fn intersects(&self, other: &dyn Collider) -> bool;

    // Optional helper trait to support downcasting
    fn as_rect(&self) -> Option<&RectCollider> {
        None
    }
    fn as_circle(&self) -> Option<&CircleCollider> {
        None
    }
    fn debug_draw(&self) {}
}

// Our Rects are using the top left corner for x and y pos
pub struct RectCollider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Collider for RectCollider {
    
    fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.x &&
        point.x <= self.x + self.width &&
        point.y >= self.y &&
        point.y <= self.y + self.height
    }

    fn intersects(&self, other: &dyn Collider) -> bool {
        if let Some(rect) = other.as_rect() {
            return self.x < rect.x + rect.width &&
            self.x + self.width > rect.x &&
            self.y < rect.y + rect.height &&
            self.y + self.height > rect.y;
        }
        false
    }

    fn as_rect(&self) -> Option<&RectCollider> {
        Some(self)
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

impl Collider for CircleCollider {
    fn contains_point(&self, point: Vec2) -> bool {
        self.center.distance(point) <= self.radius
    }

    fn intersects(&self, other: &dyn Collider) -> bool {
        if let Some(circle) = other.as_circle() {
            let dist = self.center.distance(circle.center);
            return dist <= (self.radius + circle.radius);
        }
        false
    }

    fn as_circle(&self) -> Option<&CircleCollider> {
        Some(self)
    }

    fn debug_draw(&self) {
        let debug_color = Color::new(255.0, 0.0, 255.0, 0.5);
        draw_circle(self.center.x,self.center.y, self.radius, debug_color);
    }
}