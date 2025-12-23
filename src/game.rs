use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};
use crate::truck::Truck;

pub struct GameApp {
    pub trucks: Vec<Truck>,
    pub next_truck_id: usize,
    dragging: bool,
    drag_start: Option<Pos2>,
    drag_end: Option<Pos2>,
    camera_offset: Vec2,
    panning: bool,
    pan_start: Option<Pos2>,
}

impl Default for GameApp {
    fn default() -> Self {
        let mut trucks = Vec::new();
        trucks.push(Truck::new(0, Pos2::new(200.0, 200.0)));
        trucks.push(Truck::new(1, Pos2::new(300.0, 250.0)));
        trucks.push(Truck::new(2, Pos2::new(250.0, 350.0)));
        
        Self {
            trucks,
            next_truck_id: 3,
            dragging: false,
            drag_start: None,
            drag_end: None,
            camera_offset: Vec2::ZERO,
            panning: false,
            pan_start: None,
        }
    }
}

impl eframe::App for GameApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repainting for smooth animation
        ctx.request_repaint();
        
        let delta_time = ctx.input(|i| i.stable_dt);
        
        // Update all trucks
        for truck in &mut self.trucks {
            truck.update(delta_time);
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Factory Tank Miner");
            
            ui.horizontal(|ui| {
                ui.label(format!("Trucks: {}", self.trucks.len()));
                ui.separator();
                let selected_count = self.trucks.iter().filter(|t| t.selected).count();
                ui.label(format!("Selected: {}", selected_count));
                ui.separator();
                if ui.button("Add Truck").clicked() {
                    let pos = Pos2::new(400.0, 300.0);
                    self.trucks.push(Truck::new(self.next_truck_id, pos));
                    self.next_truck_id += 1;
                }
            });
            
            ui.separator();
            ui.label("Controls:");
            ui.label("- Left click: Select single truck");
            ui.label("- Ctrl + Left click: Add/remove from selection");
            ui.label("- Right click: Move selected trucks");
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
            
            // Handle right click to move
            if response.secondary_clicked() && !self.panning {
                if let Some(target_pos) = pointer_pos {
                    let world_target = Pos2::new(target_pos.x - self.camera_offset.x, target_pos.y - self.camera_offset.y);
                    for truck in &mut self.trucks {
                        if truck.selected {
                            truck.target = Some(world_target);
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
                let color = if truck.selected {
                    Color32::from_rgb(100, 255, 100)
                } else {
                    Color32::from_rgb(100, 150, 255)
                };
                
                // Draw truck body
                let bounds = Rect::from_center_size(screen_pos, Vec2::splat(truck.size));
                painter.rect_filled(bounds, 2.0, color);
                painter.rect_stroke(bounds, 2.0, (2.0, Color32::BLACK));
                
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
        });
    }
}
