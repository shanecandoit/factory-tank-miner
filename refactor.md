# Refactoring Plan for Factory Tank Miner

## Immediate Priorities

### 1. Fix Compiler Warnings

Priority: **HIGH**

#### Dead Code

- Remove unused imports (`Vec2`, `rand::Rng` in enemy.rs)
- Remove or utilize `guns` and `bullets` fields in `GameApp`
- Remove unused `amount` field from `OrePatch` or implement depletion
- Remove unused enum variants `Fleeing` and `Attacking` from `EnemyBehavior`
- Remove or use `id`, `wander_timer`, `behavior` fields in `Enemy`
- Remove unused methods `name()` and `can_produce()`

#### Unnecessary Mutability

- Fix `mut label` in building rendering code (line 550)

#### Unused Variables

- Remove or use `grid_size`, `world_pos`, `building_type` variables

### 2. Extract Rendering Logic

Priority: **MEDIUM**

Current: All rendering logic is in `GameApp::update()` (800+ lines)

**Proposed Structure:**
```rust
// src/renderer.rs
pub struct GameRenderer {
    zoom: f32,
    camera_offset: Vec2,
}

impl GameRenderer {
    pub fn render_grid(&self, painter: &Painter, canvas_rect: Rect) { }
    pub fn render_ore_patches(&self, painter: &Painter, patches: &[OrePatch]) { }
    pub fn render_enemies(&self, painter: &Painter, enemies: &[Enemy]) { }
    pub fn render_buildings(&self, painter: &Painter, buildings: &[Building]) { }
    pub fn render_trucks(&self, painter: &Painter, trucks: &[Truck]) { }
    pub fn render_ui(&self, ui: &mut Ui, game_state: &GameState) { }
    
    fn world_to_screen(&self, pos: Pos2) -> Pos2 { }
    fn screen_to_world(&self, pos: Pos2) -> Pos2 { }
}
```

### 3. Separate Game State from UI

Priority: **MEDIUM**

**Create GameState struct:**
```rust
// src/game_state.rs
pub struct GameState {
    pub trucks: Vec<Truck>,
    pub ore_patches: Vec<OrePatch>,
    pub buildings: Vec<Building>,
    pub enemies: Vec<Enemy>,
    pub iron: u32,
    pub coal: u32,
    pub game_timer: f32,
    // ... other game state
}

impl GameState {
    pub fn update(&mut self, delta_time: f32) { }
    pub fn spawn_enemies(&mut self) { }
    pub fn update_production(&mut self) { }
    pub fn handle_combat(&mut self) { }
}
```

### 4. Input Handling Module

Priority: **LOW**

**Extract input handling:**

```rust
// src/input.rs
pub struct InputHandler {
    dragging: bool,
    drag_start: Option<Pos2>,
    drag_end: Option<Pos2>,
    panning: bool,
    pan_start: Option<Pos2>,
}

impl InputHandler {
    pub fn handle_mouse_input(&mut self, response: &Response) -> InputAction { }
    pub fn handle_keyboard_input(&mut self, ctx: &Context) -> Vec<Command> { }
}

pub enum InputAction {
    DragSelect(Rect),
    Click(Pos2),
    RightClick(Pos2),
    Pan(Vec2),
    None,
}
```

## Code Organization

### Current Structure Issues

- `game.rs` is 900+ lines - too large
- All game logic in single `update()` method
- Rendering mixed with game logic
- Hard to test individual components

### Proposed Structure

```
src/
├── main.rs                 # Entry point
├── game.rs                 # Main game coordinator (smaller)
├── game_state.rs           # Core game state and logic
├── renderer.rs             # All rendering code
├── input.rs                # Input handling
├── systems/
│   ├── mod.rs
│   ├── combat.rs           # Combat calculations
│   ├── production.rs       # Building production logic
│   ├── mining.rs           # Mining logic
│   └── enemy_spawning.rs   # Enemy spawn system
├── entities/
│   ├── mod.rs
│   ├── truck.rs            # Existing
│   ├── enemy.rs            # Existing
│   ├── building.rs         # Existing
│   └── resource.rs         # Existing
└── ui/
    ├── mod.rs
    ├── hud.rs              # Top bar UI
    ├── building_menu.rs    # Building placement UI
    └── production_ui.rs    # Production queue UI
```

## Specific Refactorings

### 1. Combat System

**Current:** Combat logic scattered in `update()` method

**Refactor to:**
```rust
// src/systems/combat.rs
pub fn process_truck_attacks(trucks: &mut [Truck], enemies: &mut [Enemy]) {
    // Armed trucks auto-fire logic
}

pub fn process_enemy_attacks(enemies: &[Enemy], buildings: &mut [Building]) {
    // Enemy building damage logic
}
```

### 2. Coordinate Transformation

**Current:** Manual zoom calculations everywhere

**Refactor to:**
```rust
// src/renderer.rs
pub struct Camera {
    offset: Vec2,
    zoom: f32,
}

impl Camera {
    pub fn world_to_screen(&self, world_pos: Pos2) -> Pos2 {
        Pos2::new(
            world_pos.x * self.zoom + self.offset.x,
            world_pos.y * self.zoom + self.offset.y
        )
    }
    
    pub fn screen_to_world(&self, screen_pos: Pos2) -> Pos2 {
        Pos2::new(
            (screen_pos.x - self.offset.x) / self.zoom,
            (screen_pos.y - self.offset.y) / self.zoom
        )
    }
    
    pub fn apply_zoom(&mut self, factor: f32, center: Pos2) {
        // Zoom toward mouse position
    }
}
```

### 3. Constants Module

**Create centralized constants:**
```rust
// src/constants.rs
pub mod truck {
    pub const SIZE: f32 = 20.0;
    pub const SPEED: f32 = 100.0;
    pub const WEAPON_RANGE: f32 = 150.0;
    pub const FIRE_RATE: f32 = 0.5;
    pub const MAX_CARGO: u32 = 64;
    pub const MAX_BULLETS: u32 = 400;
}

pub mod enemy {
    pub const SPAWN_INTERVAL_MIN: f32 = 8.0;
    pub const SPAWN_INTERVAL_MAX: f32 = 15.0;
    pub const SPAWN_DISTANCE_MIN: f32 = 1000.0;
    pub const SPAWN_DISTANCE_MAX: f32 = 1500.0;
}

pub mod game {
    pub const INITIAL_ENEMY_DELAY: f32 = 300.0; // 5 minutes
    pub const ZOOM_UNLOCK_TIME: f32 = 200.0;
    pub const MIN_ZOOM: f32 = 0.5;
    pub const MAX_ZOOM: f32 = 3.0;
}
```

### 4. UI Component Extraction

```rust
// src/ui/hud.rs
pub fn render_resource_bar(ui: &mut Ui, state: &GameState) {
    ui.horizontal(|ui| {
        ui.label(format!("Iron: {}", state.iron));
        // ...
    });
}

pub fn render_zoom_controls(ui: &mut Ui, zoom: &mut f32, game_timer: f32) {
    let enabled = game_timer >= ZOOM_UNLOCK_TIME;
    // ...
}
```

## Testing Strategy

### Unit Tests to Add

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_truck_mining_progression() { }
    
    #[test]
    fn test_enemy_damage_calculation() { }
    
    #[test]
    fn test_production_queue() { }
    
    #[test]
    fn test_camera_transformations() { }
    
    #[test]
    fn test_collision_detection() { }
}
```

## Performance Improvements

### 1. Spatial Partitioning

```rust
// src/spatial.rs
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<EntityId>>,
}

impl SpatialGrid {
    pub fn query_radius(&self, pos: Pos2, radius: f32) -> Vec<EntityId> { }
}
```

### 2. Entity Component System (Future)

Consider migrating to ECS pattern using `specs` or `hecs` crate for better performance with many entities.

## Migration Plan

### Phase 1: Quick Wins (1-2 hours)

1. Fix all compiler warnings
2. Extract constants to constants.rs
3. Add helper methods for coordinate transformation

### Phase 2: Structural (3-4 hours)

1. Create `GameState` struct
2. Extract renderer module
3. Separate input handling
4. Add basic tests

### Phase 3: Systems (4-6 hours)

1. Create systems modules
2. Extract combat logic
3. Extract production logic
4. Extract enemy spawning

### Phase 4: Polish (2-3 hours)

1. Extract UI components
2. Add comprehensive tests
3. Documentation and comments
4. Performance profiling

## Notes

- Maintain backward compatibility during refactoring
- Test after each major change
- Keep commits small and focused
- Consider feature flags for experimental features
