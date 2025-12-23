use eframe::egui;
use egui::{Color32, Pos2, Rect, Vec2};
use crate::truck::Truck;

pub struct GameApp {
    pub trucks: Vec<Truck>,
    pub next_truck_id: usize,
    dragging: bool,
    drag_start: Option<Pos2>,
    drag_end: Option<Pos2>,
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
            
            // Vertical lines
            let mut x = (canvas_rect.min.x / grid_size).floor() * grid_size;
            while x < canvas_rect.max.x {
                if x >= canvas_rect.min.x {
                    painter.line_segment(
                        [Pos2::new(x, canvas_rect.min.y), Pos2::new(x, canvas_rect.max.y)],
                        (1.0, grid_color)
                    );
                }
                x += grid_size;
            }
            
            // Horizontal lines
            let mut y = (canvas_rect.min.y / grid_size).floor() * grid_size;
            while y < canvas_rect.max.y {
                if y >= canvas_rect.min.y {
                    painter.line_segment(
                        [Pos2::new(canvas_rect.min.x, y), Pos2::new(canvas_rect.max.x, y)],
                        (1.0, grid_color)
                    );
                }
                y += grid_size;
            }
            
            // Draw origin beacon
            let origin = Pos2::new(100.0, 100.0);
            if canvas_rect.contains(origin) {
                // Cross at origin
                let beacon_color = Color32::from_rgba_premultiplied(255, 255, 255, 40);
                let size = 16.0;
                painter.line_segment(
                    [Pos2::new(origin.x - size, origin.y), Pos2::new(origin.x + size, origin.y)],
                    (2.0, beacon_color)
                );
                painter.line_segment(
                    [Pos2::new(origin.x, origin.y - size), Pos2::new(origin.x, origin.y + size)],
                    (2.0, beacon_color)
                );
                // Small circle at center
                painter.circle_filled(origin, 3.0, Color32::from_rgba_premultiplied(255, 255, 255, 60));
            }
            
            // Handle input
            let pointer_pos = response.hover_pos();
            let ctrl_held = ui.input(|i| i.modifiers.ctrl);
            
            // Handle drag selection
            if response.drag_started() && !ctrl_held {
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
                    let min_x = start.x.min(end.x);
                    let max_x = start.x.max(end.x);
                    let min_y = start.y.min(end.y);
                    let max_y = start.y.max(end.y);
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
            if response.clicked() && !self.dragging {
                if let Some(pos) = pointer_pos {
                    let mut clicked_truck = None;
                    for (i, truck) in self.trucks.iter().enumerate() {
                        if truck.contains_point(pos) {
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
            if response.secondary_clicked() {
                if let Some(target_pos) = pointer_pos {
                    for truck in &mut self.trucks {
                        if truck.selected {
                            truck.target = Some(target_pos);
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
                let color = if truck.selected {
                    Color32::from_rgb(100, 255, 100)
                } else {
                    Color32::from_rgb(100, 150, 255)
                };
                
                // Draw truck body
                painter.rect_filled(truck.bounds(), 2.0, color);
                painter.rect_stroke(truck.bounds(), 2.0, (2.0, Color32::BLACK));
                
                // Draw selection indicator
                if truck.selected {
                    let selection_rect = Rect::from_center_size(
                        truck.position,
                        Vec2::splat(truck.size + 6.0)
                    );
                    painter.rect_stroke(selection_rect, 0.0, (2.0, Color32::YELLOW));
                }
                
                // Draw target indicator
                if let Some(target) = truck.target {
                    painter.circle_stroke(target, 5.0, (2.0, Color32::from_rgb(255, 255, 100)));
                    painter.line_segment(
                        [truck.position, target],
                        (1.0, Color32::from_rgb(150, 150, 50))
                    );
                }
            }
        });
    }
}
