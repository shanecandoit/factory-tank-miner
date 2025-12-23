use egui::{Pos2, Rect, Vec2};

#[derive(Debug, Clone)]
pub struct Truck {
    pub id: usize,
    pub position: Pos2,
    pub target: Option<Pos2>,
    pub selected: bool,
    pub size: f32,
}

impl Truck {
    pub fn new(id: usize, position: Pos2) -> Self {
        Self {
            id,
            position,
            target: None,
            selected: false,
            size: 20.0,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if let Some(target) = self.target {
            let direction = target - self.position;
            let distance = direction.length();
            
            if distance > 2.0 {
                let speed = 100.0; // pixels per second
                let movement = direction.normalized() * speed * delta_time;
                
                if movement.length() < distance {
                    self.position += movement;
                } else {
                    self.position = target;
                    self.target = None;
                }
            } else {
                self.target = None;
            }
        }
    }
    
    pub fn bounds(&self) -> Rect {
        Rect::from_center_size(self.position, Vec2::splat(self.size))
    }
    
    pub fn contains_point(&self, point: Pos2) -> bool {
        self.bounds().contains(point)
    }
}
