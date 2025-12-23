use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};
use crate::truck::Truck;
use crate::resource::{OrePatch, ResourceType};
use crate::building::{Building, BuildingType, ProductionType};
use crate::enemy::{Enemy, EnemySize};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildMode {
    None,
    PlacingGarage,
    PlacingFactory,
}

pub struct GameApp {
    pub trucks: Vec<Truck>,
    pub ore_patches: Vec<OrePatch>,
    pub buildings: Vec<Building>,
    pub enemies: Vec<Enemy>,
    pub next_truck_id: usize,
    pub next_enemy_id: usize,
    pub iron: u32,
    pub coal: u32,
    pub guns: u32,
    pub bullets: u32,
    dragging: bool,
    drag_start: Option<Pos2>,
    drag_end: Option<Pos2>,
    camera_offset: Vec2,
    panning: bool,
    pan_start: Option<Pos2>,
    build_mode: BuildMode,
    selected_building: Option<usize>,
    enemy_spawn_timer: f32,
    camera_initialized: bool,
}

impl Default for GameApp {
    fn default() -> Self {
        let mut trucks = Vec::new();
        trucks.push(Truck::new(0, Pos2::new(50.0, 50.0)));
        trucks.push(Truck::new(1, Pos2::new(100.0, 50.0)));
        trucks.push(Truck::new(2, Pos2::new(75.0, 100.0)));
        
        let mut ore_patches = Vec::new();
        ore_patches.push(OrePatch::new(Pos2::new(-150.0, 100.0), ResourceType::Iron));
        ore_patches.push(OrePatch::new(Pos2::new(150.0, 100.0), ResourceType::Coal));
        
        let mut buildings = Vec::new();
        buildings.push(Building::new(Pos2::new(0.0, 0.0), BuildingType::Beacon));
        
        Self {
            trucks,
            ore_patches,
            buildings,
            enemies: Vec::new(),
            next_truck_id: 3,
            next_enemy_id: 0,
            iron: 0,
            coal: 0,
            guns: 0,
            bullets: 0,
            dragging: false,
            drag_start: None,
            drag_end: None,
            camera_offset: Vec2::ZERO,
            panning: false,
            pan_start: None,
            build_mode: BuildMode::None,
            selected_building: None,
            enemy_spawn_timer: 10.0,
            camera_initialized: false,
        }
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repainting for smooth animation
        ctx.request_repaint();
        
        let delta_time = ctx.input(|i| i.stable_dt);
        
        // Center camera on beacon on first frame
        if !self.camera_initialized {
            let screen_rect = ctx.screen_rect();
            self.camera_offset = Vec2::new(screen_rect.width() / 2.0, screen_rect.height() / 2.0);
            self.camera_initialized = true;
        }
        
        // Spawn enemies periodically far from beacon
        self.enemy_spawn_timer -= delta_time;
        if self.enemy_spawn_timer <= 0.0 {
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let distance = rng.gen_range(1000.0..1500.0);
            let pos = Pos2::new(angle.cos() * distance, angle.sin() * distance);
            
            let size = match rng.gen_range(0..10) {
                0..=6 => EnemySize::Small,
                7..=8 => EnemySize::Medium,
                _ => EnemySize::Large,
            };
            
            self.enemies.push(Enemy::new(self.next_enemy_id, pos, size));
            self.next_enemy_id += 1;
            self.enemy_spawn_timer = rng.gen_range(8.0..15.0);
        }
        
        // Update enemies
        for enemy in &mut self.enemies {
            enemy.update(delta_time);
            enemy.being_shot_at = false; // Reset each frame
        }
        
        // Update all trucks
        for truck in &mut self.trucks {
            truck.update(delta_time);
            
            // Check if truck is at beacon to unload (any state, any amount)
            let beacon_pos = Pos2::new(0.0, 0.0);
            let distance = (truck.position - beacon_pos).length();
            if distance < 35.0 && truck.cargo_amount > 0 {
                // Unload cargo
                match truck.cargo {
                    Some(ResourceType::Iron) => self.iron += truck.cargo_amount,
                    Some(ResourceType::Coal) => self.coal += truck.cargo_amount,
                    None => {}
                }
                truck.cargo = None;
                truck.cargo_amount = 0;
                // If they were returning to base, they can now be idle
                if truck.state == crate::truck::TruckState::ReturningToBase {
                    truck.state = crate::truck::TruckState::Idle;
                    truck.target = None;
                }
            }
            
            // Auto-return to last mining position if empty and idle at beacon
            if distance < 35.0 && truck.state == crate::truck::TruckState::Idle 
                && truck.cargo_amount == 0 && truck.last_mining_position.is_some() {
                if let Some(mining_pos) = truck.last_mining_position {
                    truck.start_moving(mining_pos);
                }
            }
            
            // Check if truck is at a factory to equip weapons
            for building in &mut self.buildings {
                if building.building_type == BuildingType::Factory {
                    let factory_dist = (truck.position - building.position).length();
                    // Allow equipping even while moving, just need to be close
                    if factory_dist < 70.0 {
                        // Equip gun if available and truck doesn't have one
                        if !truck.has_gun && building.stored_guns > 0 {
                            truck.has_gun = true;
                            building.stored_guns -= 1;
                        }
                        
                        // Load bullets if truck has gun and factory has bullets
                        if truck.has_gun && truck.bullets < 400 && building.stored_bullet_boxes > 0 {
                            let bullets_needed = 400 - truck.bullets;
                            let boxes_to_load = (bullets_needed / 100).min(building.stored_bullet_boxes);
                            if boxes_to_load > 0 {
                                truck.bullets += boxes_to_load * 100;
                                building.stored_bullet_boxes -= boxes_to_load;
                            }
                        }
                    }
                }
            }
            
            // Check if truck is on an ore patch and should start mining
            if truck.state == crate::truck::TruckState::Idle && truck.cargo_amount < 64 {
                for patch in &self.ore_patches {
                    if patch.contains_point(truck.position) {
                        truck.start_mining(patch.resource_type);
                        break;
                    }
                }
            }
        }
        
        // Armed trucks auto-fire at enemies in range
        
        // Update buildings production
        for building in &mut self.buildings {
            if !building.production_queue.is_empty() {
                let current = building.production_queue[0];
                let production_time = current.time();
                
                building.production_progress += delta_time;
                
                if building.production_progress >= production_time {
                    building.production_progress = 0.0;
                    building.production_queue.remove(0);
                    
                    // Produce the item
                    match current {
                        ProductionType::Truck => {
                            let offset_x = (self.next_truck_id as f32 % 3.0) * 30.0 - 30.0;
                            let new_truck = Truck::new(
                                self.next_truck_id,
                                Pos2::new(building.position.x + offset_x, building.position.y + 60.0)
                            );
                            self.trucks.push(new_truck);
                            self.next_truck_id += 1;
                        }
                        ProductionType::Gun => {
                            building.stored_guns += 1;
                        }
                        ProductionType::Bullets => {
                            building.stored_bullet_boxes += 1;
                        }
                    }
                }
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Factory Tank Miner");
            
            ui.horizontal(|ui| {
                ui.label(format!("Iron: {}", self.iron));
                ui.separator();
                ui.label(format!("Coal: {}", self.coal));
                ui.separator();
                
                // Calculate total factory inventory
                let total_guns: u32 = self.buildings.iter()
                    .filter(|b| b.building_type == BuildingType::Factory)
                    .map(|b| b.stored_guns)
                    .sum();
                let total_bullets: u32 = self.buildings.iter()
                    .filter(|b| b.building_type == BuildingType::Factory)
                    .map(|b| b.stored_bullet_boxes)
                    .sum();
                
                ui.label(format!("Guns: {} | Bullets: {} boxes", total_guns, total_bullets));
                ui.separator();
                ui.label(format!("Trucks: {}", self.trucks.len()));
                ui.separator();
                let selected_count = self.trucks.iter().filter(|t| t.selected).count();
                ui.label(format!("Selected: {}", selected_count));
                ui.separator();
                let mining_count = self.trucks.iter().filter(|t| t.state == crate::truck::TruckState::Mining).count();
                ui.label(format!("Mining: {}", mining_count));
                ui.separator();
                ui.label(format!("Enemies: {}", self.enemies.len()));
            });
            
            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Build:");
                
                let garage_cost = Building::new(Pos2::ZERO, BuildingType::Garage).cost();
                let can_afford_garage = self.iron >= garage_cost.0 && self.coal >= garage_cost.1;
                let garage_text = format!("Garage ({}Fe {}C)", garage_cost.0, garage_cost.1);
                
                if ui.add_enabled(can_afford_garage && self.build_mode == BuildMode::None, 
                    egui::Button::new(garage_text)).clicked() {
                    self.build_mode = BuildMode::PlacingGarage;
                }
                
                let factory_cost = Building::new(Pos2::ZERO, BuildingType::Factory).cost();
                let can_afford_factory = self.iron >= factory_cost.0 && self.coal >= factory_cost.1;
                let factory_text = format!("Factory ({}Fe {}C)", factory_cost.0, factory_cost.1);
                
                if ui.add_enabled(can_afford_factory && self.build_mode == BuildMode::None,
                    egui::Button::new(factory_text)).clicked() {
                    self.build_mode = BuildMode::PlacingFactory;
                }
                
                if self.build_mode != BuildMode::None {
                    if ui.button("Cancel").clicked() {
                        self.build_mode = BuildMode::None;
                    }
                }
            });
            
            ui.separator();
            
            // Building production UI
            if let Some(building_idx) = self.selected_building {
                let building_type = self.buildings.get(building_idx).map(|b| b.building_type);
                let queue_len = self.buildings.get(building_idx).map(|b| b.production_queue.len()).unwrap_or(0);
                
                if let Some(btype) = building_type {
                    ui.horizontal(|ui| {
                        ui.label(format!("Selected: {:?}", btype));
                        
                        match btype {
                            BuildingType::Garage => {
                                let cost = ProductionType::Truck.cost();
                                let can_afford = self.iron >= cost.0 && self.coal >= cost.1;
                                if ui.add_enabled(can_afford, egui::Button::new(format!("Build Truck ({}Fe {}C)", cost.0, cost.1))).clicked() {
                                    if let Some(b) = self.buildings.get_mut(building_idx) {
                                        self.iron -= cost.0;
                                        self.coal -= cost.1;
                                        b.production_queue.push(ProductionType::Truck);
                                    }
                                }
                            }
                            BuildingType::Factory => {
                                let gun_cost = ProductionType::Gun.cost();
                                let can_afford_gun = self.iron >= gun_cost.0 && self.coal >= gun_cost.1;
                                if ui.add_enabled(can_afford_gun, egui::Button::new(format!("Build Gun ({}Fe {}C)", gun_cost.0, gun_cost.1))).clicked() {
                                    if let Some(b) = self.buildings.get_mut(building_idx) {
                                        self.iron -= gun_cost.0;
                                        self.coal -= gun_cost.1;
                                        b.production_queue.push(ProductionType::Gun);
                                    }
                                }
                                
                                let bullet_cost = ProductionType::Bullets.cost();
                                let can_afford_bullets = self.iron >= bullet_cost.0 && self.coal >= bullet_cost.1;
                                if ui.add_enabled(can_afford_bullets, egui::Button::new(format!("Build Bullets ({}Fe {}C)", bullet_cost.0, bullet_cost.1))).clicked() {
                                    if let Some(b) = self.buildings.get_mut(building_idx) {
                                        self.iron -= bullet_cost.0;
                                        self.coal -= bullet_cost.1;
                                        b.production_queue.push(ProductionType::Bullets);
                                    }
                                }
                            }
                            BuildingType::Beacon => {}
                        }
                        
                        if queue_len > 0 {
                            ui.separator();
                            ui.label(format!("Queue: {}", queue_len));
                        }
                    });
                }
            }
            
            ui.separator();
            ui.label("Controls:");
            ui.label("- Left click: Select single truck");
            ui.label("- Ctrl + Left click: Add/remove from selection");
            ui.label("- Right click: Move selected trucks or place building");
            ui.label("- Drag: Box select trucks");
            ui.label("- Middle mouse drag: Pan camera");
            
            ui.separator();
            
            // Game canvas
            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click_and_drag(),
            );
            
            let canvas_rect = response.rect;
            painter.rect_filled(canvas_rect, 0.0, Color32::from_rgb(30, 30, 35));
            
            // Draw grid
            let grid_size = 64.0;
            let grid_color = Color32::from_rgb(25, 25, 30);
            
            // Calculate world space bounds visible in the canvas
            let world_min_x = canvas_rect.min.x - self.camera_offset.x;
            let world_max_x = canvas_rect.max.x - self.camera_offset.x;
            let world_min_y = canvas_rect.min.y - self.camera_offset.y;
            let world_max_y = canvas_rect.max.y - self.camera_offset.y;
            
            // Vertical lines
            let start_x = (world_min_x / grid_size).floor() * grid_size;
            let mut x = start_x;
            while x <= world_max_x {
                let screen_x = x + self.camera_offset.x;
                painter.line_segment(
                    [Pos2::new(screen_x, canvas_rect.min.y), Pos2::new(screen_x, canvas_rect.max.y)],
                    (1.0, grid_color)
                );
                x += grid_size;
            }
            
            // Horizontal lines
            let start_y = (world_min_y / grid_size).floor() * grid_size;
            let mut y = start_y;
            while y <= world_max_y {
                let screen_y = y + self.camera_offset.y;
                painter.line_segment(
                    [Pos2::new(canvas_rect.min.x, screen_y), Pos2::new(canvas_rect.max.x, screen_y)],
                    (1.0, grid_color)
                );
                y += grid_size;
            }
            
            // Draw origin beacon
            let origin_screen = Pos2::new(0.0 + self.camera_offset.x, 0.0 + self.camera_offset.y);
            if canvas_rect.contains(origin_screen) {
                // Cross at origin
                let beacon_color = Color32::from_rgba_premultiplied(255, 255, 255, 40);
                let size = 16.0;
                painter.line_segment(
                    [Pos2::new(origin_screen.x - size, origin_screen.y), Pos2::new(origin_screen.x + size, origin_screen.y)],
                    (2.0, beacon_color)
                );
                painter.line_segment(
                    [Pos2::new(origin_screen.x, origin_screen.y - size), Pos2::new(origin_screen.x, origin_screen.y + size)],
                    (2.0, beacon_color)
                );
                // Small circle at center
                painter.circle_filled(origin_screen, 3.0, Color32::from_rgba_premultiplied(255, 255, 255, 60));
            }
            
            // Draw ore patches
            for patch in &self.ore_patches {
                let screen_pos = Pos2::new(patch.position.x + self.camera_offset.x, patch.position.y + self.camera_offset.y);
                
                let color = match patch.resource_type {
                    ResourceType::Iron => Color32::from_rgb(180, 140, 120),
                    ResourceType::Coal => Color32::from_rgb(60, 60, 70),
                };
                
                painter.circle_filled(screen_pos, patch.size, color);
                painter.circle_stroke(screen_pos, patch.size, (2.0, Color32::BLACK));
                
                // Draw label
                let label = match patch.resource_type {
                    ResourceType::Iron => "IRON",
                    ResourceType::Coal => "COAL",
                };
                painter.text(
                    screen_pos,
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(12.0),
                    Color32::WHITE,
                );
            }
            
            // Draw enemies
            for enemy in &self.enemies {
                let screen_pos = Pos2::new(enemy.position.x + self.camera_offset.x, enemy.position.y + self.camera_offset.y);
                
                // Red color, darker for larger enemies
                let color = match enemy.size {
                    EnemySize::Small => Color32::from_rgb(255, 100, 100),
                    EnemySize::Medium => Color32::from_rgb(220, 60, 60),
                    EnemySize::Large => Color32::from_rgb(180, 20, 20),
                };
                
                painter.circle_filled(screen_pos, enemy.radius(), color);
                painter.circle_stroke(screen_pos, enemy.radius(), (2.0, Color32::from_rgb(100, 0, 0)));
                
                // Health bar
                if enemy.health < enemy.max_health {
                    let bar_width = enemy.radius() * 2.0;
                    let bar_height = 4.0;
                    let health_percent = enemy.health as f32 / enemy.max_health as f32;
                    
                    let bg_rect = Rect::from_min_size(
                        Pos2::new(screen_pos.x - bar_width / 2.0, screen_pos.y - enemy.radius() - 10.0),
                        Vec2::new(bar_width, bar_height)
                    );
                    painter.rect_filled(bg_rect, 0.0, Color32::from_rgb(50, 50, 50));
                    
                    let health_rect = Rect::from_min_size(
                        Pos2::new(screen_pos.x - bar_width / 2.0, screen_pos.y - enemy.radius() - 10.0),
                        Vec2::new(bar_width * health_percent, bar_height)
                    );
                    painter.rect_filled(health_rect, 0.0, Color32::from_rgb(255, 0, 0));
                }
            }
            
            // Draw buildings
            for (idx, building) in self.buildings.iter().enumerate() {
                let screen_pos = Pos2::new(building.position.x + self.camera_offset.x, building.position.y + self.camera_offset.y);
                
                let (color, mut label) = match building.building_type {
                    BuildingType::Beacon => (Color32::from_rgb(255, 215, 0), "BEACON".to_string()),
                    BuildingType::Garage => (Color32::from_rgb(120, 120, 140), "GARAGE".to_string()),
                    BuildingType::Factory => {
                        let label = format!("FACTORY\n({}G {}B)", building.stored_guns, building.stored_bullet_boxes);
                        (Color32::from_rgb(140, 100, 80), label)
                    },
                };
                
                let rect = Rect::from_center_size(screen_pos, Vec2::splat(building.size * 2.0));
                painter.rect_filled(rect, 2.0, color);
                
                // Highlight if selected
                let stroke_color = if Some(idx) == self.selected_building {
                    Color32::from_rgb(255, 255, 0)
                } else {
                    Color32::BLACK
                };
                painter.rect_stroke(rect, 2.0, (3.0, stroke_color));
                
                painter.text(
                    screen_pos,
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(9.0),
                    Color32::BLACK,
                );
                
                // Show production progress
                if !building.production_queue.is_empty() {
                    let current = building.production_queue[0];
                    let progress = building.production_progress / current.time();
                    let bar_width = building.size * 2.0;
                    let bar_height = 6.0;
                    
                    let bg_rect = Rect::from_min_size(
                        Pos2::new(screen_pos.x - bar_width / 2.0, screen_pos.y + building.size + 5.0),
                        Vec2::new(bar_width, bar_height)
                    );
                    painter.rect_filled(bg_rect, 0.0, Color32::from_rgb(50, 50, 50));
                    
                    let bar_rect = Rect::from_min_size(
                        Pos2::new(screen_pos.x - bar_width / 2.0, screen_pos.y + building.size + 5.0),
                        Vec2::new(bar_width * progress, bar_height)
                    );
                    painter.rect_filled(bar_rect, 0.0, Color32::from_rgb(100, 255, 100));
                }
            }
            
            // Handle input
            let pointer_pos = response.hover_pos();
            let ctrl_held = ui.input(|i| i.modifiers.ctrl);
            let middle_button = ui.input(|i| i.pointer.button_down(egui::PointerButton::Middle));
            
            // Handle camera panning with middle mouse button
            if middle_button && !self.panning {
                if let Some(pos) = pointer_pos {
                    self.panning = true;
                    self.pan_start = Some(pos);
                }
            }
            
            if self.panning {
                if middle_button {
                    if let (Some(start), Some(current)) = (self.pan_start, pointer_pos) {
                        let delta = current - start;
                        self.camera_offset += delta;
                        self.pan_start = Some(current);
                    }
                } else {
                    self.panning = false;
                    self.pan_start = None;
                }
            }
            
            // Handle drag selection
            if response.drag_started() && !ctrl_held && !self.panning {
                if let Some(pos) = pointer_pos {
                    self.dragging = true;
                    self.drag_start = Some(pos);
                    self.drag_end = Some(pos);
                }
            }
            
            if response.dragged() && self.dragging {
                self.drag_end = pointer_pos;
            }
            
            if response.drag_stopped() && self.dragging {
                if let (Some(start), Some(end)) = (self.drag_start, self.drag_end) {
                    // Convert screen space selection to world space
                    let world_start = Pos2::new(start.x - self.camera_offset.x, start.y - self.camera_offset.y);
                    let world_end = Pos2::new(end.x - self.camera_offset.x, end.y - self.camera_offset.y);
                    
                    let min_x = world_start.x.min(world_end.x);
                    let max_x = world_start.x.max(world_end.x);
                    let min_y = world_start.y.min(world_end.y);
                    let max_y = world_start.y.max(world_end.y);
                    let selection_rect = Rect::from_min_max(
                        Pos2::new(min_x, min_y),
                        Pos2::new(max_x, max_y)
                    );
                    
                    for truck in &mut self.trucks {
                        truck.selected = selection_rect.intersects(truck.bounds());
                    }
                }
                
                self.dragging = false;
                self.drag_start = None;
                self.drag_end = None;
            }
            
            // Handle single click selection
            if response.clicked() && !self.dragging && !self.panning {
                if let Some(pos) = pointer_pos {
                    let world_pos = Pos2::new(pos.x - self.camera_offset.x, pos.y - self.camera_offset.y);
                    
                    // Check if clicking on a building first
                    let mut clicked_building = None;
                    for (i, building) in self.buildings.iter().enumerate() {
                        if building.contains_point(world_pos) {
                            clicked_building = Some(i);
                            break;
                        }
                    }
                    
                    if let Some(i) = clicked_building {
                        self.selected_building = Some(i);
                    } else {
                        self.selected_building = None;
                        
                        // Try to select a truck
                        let mut clicked_truck = None;
                        for (i, truck) in self.trucks.iter().enumerate() {
                            if truck.contains_point(world_pos) {
                                clicked_truck = Some(i);
                                break;
                            }
                        }
                    
                    if let Some(i) = clicked_truck {
                        if ctrl_held {
                            self.trucks[i].selected = !self.trucks[i].selected;
                        } else {
                            for truck in &mut self.trucks {
                                truck.selected = false;
                            }
                            self.trucks[i].selected = true;
                        }
                    } else if !ctrl_held {
                        for truck in &mut self.trucks {
                            truck.selected = false;
                        }
                    }
                    }
                }
            }
            
            // Handle right click to move or place building
            if response.secondary_clicked() && !self.panning {
                if let Some(target_pos) = pointer_pos {
                    let world_target = Pos2::new(target_pos.x - self.camera_offset.x, target_pos.y - self.camera_offset.y);
                    
                    match self.build_mode {
                        BuildMode::PlacingGarage => {
                            let cost = Building::new(Pos2::ZERO, BuildingType::Garage).cost();
                            if self.iron >= cost.0 && self.coal >= cost.1 {
                                self.iron -= cost.0;
                                self.coal -= cost.1;
                                self.buildings.push(Building::new(world_target, BuildingType::Garage));
                                self.build_mode = BuildMode::None;
                            }
                        }
                        BuildMode::PlacingFactory => {
                            let cost = Building::new(Pos2::ZERO, BuildingType::Factory).cost();
                            if self.iron >= cost.0 && self.coal >= cost.1 {
                                self.iron -= cost.0;
                                self.coal -= cost.1;
                                self.buildings.push(Building::new(world_target, BuildingType::Factory));
                                self.build_mode = BuildMode::None;
                            }
                        }
                        BuildMode::None => {
                            // Normal truck movement
                            for truck in &mut self.trucks {
                                if truck.selected {
                                    truck.start_moving(world_target);
                                }
                            }
                        }
                    }
                }
            }
            
            // Draw selection box
            if self.dragging {
                if let (Some(start), Some(end)) = (self.drag_start, self.drag_end) {
                    let min_x = start.x.min(end.x);
                    let max_x = start.x.max(end.x);
                    let min_y = start.y.min(end.y);
                    let max_y = start.y.max(end.y);
                    let rect = Rect::from_min_max(
                        Pos2::new(min_x, min_y),
                        Pos2::new(max_x, max_y)
                    );
                    painter.rect_stroke(rect, 0.0, (1.0, Color32::WHITE));
                    painter.rect_filled(rect, 0.0, Color32::from_rgba_premultiplied(255, 255, 255, 20));
                }
            }
            
            // Draw trucks
            for truck in &self.trucks {
                let screen_pos = Pos2::new(truck.position.x + self.camera_offset.x, truck.position.y + self.camera_offset.y);
                
                // Choose color: armed=orange, with cargo=cargo color, else=blue, selected=green
                let color = if truck.has_gun {
                    Color32::from_rgb(255, 140, 0) // Orange for armed trucks
                } else if truck.selected {
                    Color32::from_rgb(100, 255, 100)
                } else {
                    match truck.cargo {
                        Some(ResourceType::Iron) => Color32::from_rgb(180, 140, 120),
                        Some(ResourceType::Coal) => Color32::from_rgb(80, 80, 90),
                        None => Color32::from_rgb(100, 150, 255),
                    }
                };
                
                // Draw truck body
                let bounds = Rect::from_center_size(screen_pos, Vec2::splat(truck.size));
                painter.rect_filled(bounds, 2.0, color);
                painter.rect_stroke(bounds, 2.0, (2.0, Color32::BLACK));
                
                // Draw gun/ammo indicator for armed trucks
                if truck.has_gun {
                    // Draw "G" with bullet count
                    painter.text(
                        screen_pos,
                        egui::Align2::CENTER_CENTER,
                        format!("G({})", truck.bullets),
                        egui::FontId::proportional(9.0),
                        Color32::WHITE,
                    );
                } else if truck.cargo_amount > 0 {
                    // Show cargo amount for mining trucks
                    painter.text(
                        screen_pos,
                        egui::Align2::CENTER_CENTER,
                        format!("{}", truck.cargo_amount),
                        egui::FontId::proportional(10.0),
                        Color32::WHITE,
                    );
                }
                
                // Draw selection indicator
                if truck.selected {
                    let selection_rect = Rect::from_center_size(
                        screen_pos,
                        Vec2::splat(truck.size + 6.0)
                    );
                    painter.rect_stroke(selection_rect, 0.0, (2.0, Color32::YELLOW));
                }
                
                // Draw target indicator
                if let Some(target) = truck.target {
                    let screen_target = Pos2::new(target.x + self.camera_offset.x, target.y + self.camera_offset.y);
                    painter.circle_stroke(screen_target, 5.0, (2.0, Color32::from_rgb(255, 255, 100)));
                    painter.line_segment(
                        [screen_pos, screen_target],
                        (1.0, Color32::from_rgb(150, 150, 50))
                    );
                }
            }
            
            // Draw building placement preview
            if self.build_mode != BuildMode::None {
                if let Some(pos) = pointer_pos {
                    let world_pos = Pos2::new(pos.x - self.camera_offset.x, pos.y - self.camera_offset.y);
                    let screen_pos = pos;
                    
                    let (building_type, size) = match self.build_mode {
                        BuildMode::PlacingGarage => (BuildingType::Garage, 40.0),
                        BuildMode::PlacingFactory => (BuildingType::Factory, 50.0),
                        BuildMode::None => unreachable!(),
                    };
                    
                    let rect = Rect::from_center_size(screen_pos, Vec2::splat(size * 2.0));
                    painter.rect_filled(rect, 2.0, Color32::from_rgba_premultiplied(100, 255, 100, 100));
                    painter.rect_stroke(rect, 2.0, (2.0, Color32::GREEN));
                }
            }
        });
    }
}
