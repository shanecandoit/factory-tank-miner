use egui::{Pos2, Rect, Vec2};
use crate::resource::ResourceType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TruckState {
    Idle,
    Moving,
    Mining,
    ReturningToBase,
}

#[derive(Debug, Clone)]
pub struct Truck {
    pub id: usize,
    pub position: Pos2,
    pub target: Option<Pos2>,
    pub selected: bool,
    pub size: f32,
    pub state: TruckState,
    pub cargo: Option<ResourceType>,
    pub cargo_amount: u32,
    pub mining_progress: f32,
    pub last_mining_position: Option<Pos2>,
}

impl Truck {
    pub fn new(id: usize, position: Pos2) -> Self {
        Self {
            id,
            position,
            target: None,
            selected: false,
            size: 20.0,
            state: TruckState::Idle,
            cargo: None,
            cargo_amount: 0,
            mining_progress: 0.0,
            last_mining_position: None,
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        match self.state {
            TruckState::Mining => {
                // Mining progress
                self.mining_progress += delta_time;
                if self.mining_progress >= 1.0 {
                    // Mine one unit per second
                    self.mining_progress = 0.0;
                    self.cargo_amount += 1;
                    
                    // Check if full
                    if self.cargo_amount >= 64 {
                        self.state = TruckState::ReturningToBase;
                        self.target = Some(Pos2::new(0.0, 0.0)); // Head to beacon
                    }
                }
            }
            TruckState::Moving | TruckState::ReturningToBase => {
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
                            
                            // If returning to base, unload
                            if self.state == TruckState::ReturningToBase {
                                self.cargo = None;
                                self.cargo_amount = 0;
                                self.state = TruckState::Idle;
                            } else {
                                self.state = TruckState::Idle;
                            }
                        }
                    } else {
                        self.position = target;
                        self.target = None;
                        
                        // If returning to base, unload
                        if self.state == TruckState::ReturningToBase {
                            self.cargo = None;
                            self.cargo_amount = 0;
                            self.state = TruckState::Idle;
                        } else {
                            self.state = TruckState::Idle;
                        }
                    }
                }
            }
            TruckState::Idle => {
                // Just sitting idle
            }
        }
    }
    
    pub fn start_moving(&mut self, target: Pos2) {
        if self.state != TruckState::ReturningToBase {
            self.target = Some(target);
            self.state = TruckState::Moving;
            self.mining_progress = 0.0;
        }
    }
    
    pub fn start_mining(&mut self, resource_type: ResourceType) {
        if self.state != TruckState::ReturningToBase {
            self.target = None;
            self.state = TruckState::Mining;
            self.cargo = Some(resource_type);
            self.mining_progress = 0.0;
            self.last_mining_position = Some(self.position);
        }
    }
    
    pub fn bounds(&self) -> Rect {
        Rect::from_center_size(self.position, Vec2::splat(self.size))
    }
    
    pub fn contains_point(&self, point: Pos2) -> bool {
        self.bounds().contains(point)
    }
}
