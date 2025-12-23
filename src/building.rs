use egui::Pos2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildingType {
    Beacon,  // The starting base
    Garage,  // Builds trucks
    Factory, // Makes guns and bullets
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProductionType {
    Truck,
    Gun,
    Bullets,
}

impl ProductionType {
    pub fn cost(&self) -> (u32, u32) {
        // Returns (iron, coal) cost
        match self {
            ProductionType::Truck => (20, 10),
            ProductionType::Gun => (30, 5),
            ProductionType::Bullets => (5, 10),
        }
    }
    
    pub fn time(&self) -> f32 {
        match self {
            ProductionType::Truck => 5.0,
            ProductionType::Gun => 8.0,
            ProductionType::Bullets => 3.0,
        }
    }
    
    pub fn name(&self) -> &str {
        match self {
            ProductionType::Truck => "Truck",
            ProductionType::Gun => "Gun",
            ProductionType::Bullets => "Bullets",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Building {
    pub position: Pos2,
    pub building_type: BuildingType,
    pub size: f32,
    pub production_queue: Vec<ProductionType>,
    pub production_progress: f32,
    pub stored_guns: u32,
    pub stored_bullet_boxes: u32,
}

impl Building {
    pub fn new(position: Pos2, building_type: BuildingType) -> Self {
        let size = match building_type {
            BuildingType::Beacon => 30.0,
            BuildingType::Garage => 40.0,
            BuildingType::Factory => 50.0,
        };
        
        Self {
            position,
            building_type,
            size,
            production_queue: Vec::new(),
            production_progress: 0.0,
            stored_guns: 0,
            stored_bullet_boxes: 0,
        }
    }
    
    pub fn contains_point(&self, point: Pos2) -> bool {
        let dx = (point.x - self.position.x).abs();
        let dy = (point.y - self.position.y).abs();
        dx < self.size && dy < self.size
    }
    
    pub fn cost(&self) -> (u32, u32) {
        // Returns (iron, coal) cost
        match self.building_type {
            BuildingType::Beacon => (0, 0),
            BuildingType::Garage => (50, 30),
            BuildingType::Factory => (100, 50),
        }
    }
    
    pub fn can_produce(&self, production_type: ProductionType) -> bool {
        match (self.building_type, production_type) {
            (BuildingType::Garage, ProductionType::Truck) => true,
            (BuildingType::Factory, ProductionType::Gun) => true,
            (BuildingType::Factory, ProductionType::Bullets) => true,
            _ => false,
        }
    }
}
