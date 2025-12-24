use egui::{Pos2, Vec2};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemySize {
    Small,  // 10 radius
    Medium, // 50 radius
    Large,  // 100 radius
}

impl EnemySize {
    pub fn radius(&self) -> f32 {
        match self {
            EnemySize::Small => 10.0,
            EnemySize::Medium => 25.0,
            EnemySize::Large => 50.0,
        }
    }
    
    pub fn health(&self) -> u32 {
        match self {
            EnemySize::Small => 10,
            EnemySize::Medium => 40,
            EnemySize::Large => 100,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnemyBehavior {
    Wandering,
    Fleeing,
    Attacking,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub id: usize,
    pub position: Pos2,
    pub size: EnemySize,
    pub health: u32,
    pub max_health: u32,
    pub target: Option<Pos2>,
    pub wander_timer: f32,
    pub behavior: EnemyBehavior,
    pub being_shot_at: bool,
}

impl Enemy {
    pub fn new(id: usize, position: Pos2, size: EnemySize) -> Self {
        let max_health = size.health();
        Self {
            id,
            position,
            size,
            health: max_health,
            max_health,
            target: None,
            wander_timer: 0.0,
            behavior: EnemyBehavior::Wandering,
            being_shot_at: false,
        }
    }
    
    pub fn update(&mut self, delta_time: f32, beacon_pos: Pos2) {
        // Always head towards the beacon (origin)
        self.target = Some(beacon_pos);
        
        // Move towards target
        if let Some(target) = self.target {
            let direction = target - self.position;
            let distance = direction.length();
            
            if distance > 2.0 {
                let speed = match self.size {
                    EnemySize::Small => 20.0,
                    EnemySize::Medium => 15.0,
                    EnemySize::Large => 10.0,
                };
                
                let movement = direction.normalized() * speed * delta_time;
                if movement.length() < distance {
                    self.position += movement;
                } else {
                    self.position = target;
                }
            }
        }
    }
    
    pub fn radius(&self) -> f32 {
        self.size.radius()
    }
}
