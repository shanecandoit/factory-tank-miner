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
            EnemySize::Small => 50,
            EnemySize::Medium => 200,
            EnemySize::Large => 500,
        }
    }
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
        }
    }
    
    pub fn update(&mut self, delta_time: f32) {
        self.wander_timer -= delta_time;
        
        // Pick a new random target occasionally
        if self.wander_timer <= 0.0 {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(50.0..200.0);
            
            self.target = Some(Pos2::new(
                self.position.x + angle.cos() * distance,
                self.position.y + angle.sin() * distance,
            ));
            
            self.wander_timer = rng.gen_range(2.0..5.0);
        }
        
        // Move towards target
        if let Some(target) = self.target {
            let direction = target - self.position;
            let distance = direction.length();
            
            if distance > 2.0 {
                let speed = match self.size {
                    EnemySize::Small => 40.0,
                    EnemySize::Medium => 25.0,
                    EnemySize::Large => 15.0,
                };
                
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
    
    pub fn radius(&self) -> f32 {
        self.size.radius()
    }
}
