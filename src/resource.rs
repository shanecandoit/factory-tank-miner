use egui::Pos2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    Iron,
    Coal,
}

#[derive(Debug, Clone)]
pub struct OrePatch {
    pub position: Pos2,
    pub size: f32,
    pub resource_type: ResourceType,
    pub amount: u32, // Infinite for now, but could be depleted later
}

impl OrePatch {
    pub fn new(position: Pos2, resource_type: ResourceType) -> Self {
        Self {
            position,
            size: 40.0,
            resource_type,
            amount: 999999, // Effectively infinite
        }
    }
    
    pub fn contains_point(&self, point: Pos2) -> bool {
        let dx = point.x - self.position.x;
        let dy = point.y - self.position.y;
        (dx * dx + dy * dy) < (self.size * self.size)
    }
}
