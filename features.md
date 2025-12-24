# Feature Ideas for Factory Tank Miner

## High Priority

### Game Balance & Core Mechanics

- [ ] **Wave System**: Replace continuous spawning with timed waves that get progressively harder
- [ ] **Resource Depletion**: Make ore patches deplete over time, forcing expansion
- [ ] **Truck Upgrades**: Allow upgrading trucks with better mining speed, cargo capacity, or armor
- [ ] **Beacon Can Be Destroyed**: Add game over condition when beacon health reaches 0
- [ ] **Enemy Drops**: Enemies drop scrap/materials when killed that trucks can collect

### Buildings & Production

- [ ] **Turrets**: Stationary defensive structures that auto-fire at enemies
- [ ] **Walls**: Build defensive barriers to slow enemy advance
- [ ] **Smelter**: Convert raw ore into refined materials for advanced production
- [ ] **Repair Station**: Slowly repair damaged buildings and armed trucks
- [ ] **Storage Silos**: Store excess resources (currently unlimited in beacon)

### Combat & Defense

- [ ] **Multiple Weapon Types**: Different guns with varying damage, range, and fire rate
- [ ] **Area Damage**: Some weapons hit multiple enemies
- [ ] **Manual Targeting**: Right-click enemies to focus fire from selected armed trucks
- [ ] **Retreat Command**: Armed trucks flee when low on ammo or health
- [ ] **Truck Armor**: Armed trucks can take a few hits before being destroyed

## Medium Priority

### UI/UX Improvements

- [ ] **Minimap**: Small overview map showing entire play area
- [ ] **Resource Graph**: Track resource collection over time
- [ ] **Hotkeys**: Number keys to select truck groups, keyboard shortcuts for buildings
- [ ] **Alert System**: Notifications when buildings under attack or production complete
- [ ] **Time Display**: Show game time and time until next wave
- [ ] **Pause Menu**: Pause game and see stats/help

### Quality of Life

- [ ] **Truck Queuing**: Queue up multiple mining locations for trucks
- [ ] **Auto-Resupply**: Armed trucks automatically return to factory for ammo
- [ ] **Building Queue**: Queue multiple items in production buildings
- [ ] **Shift-Click Placement**: Place multiple buildings of same type
- [ ] **Delete Buildings**: Ability to demolish buildings for partial resource refund
- [ ] **Save/Load Game**: Persist game state between sessions

### Visual & Audio

- [ ] **Particle Effects**: Explosions, mining sparks, muzzle flashes
- [ ] **Sound Effects**: Gunfire, mining, building construction, enemy death
- [ ] **Background Music**: Ambient soundtrack with intensity based on enemy proximity
- [ ] **Health Bars Always Visible**: Option to always show health bars
- [ ] **Truck Trails**: Visual trail showing recent truck movement

## Low Priority / Polish

### Advanced Features

- [ ] **Enemy Variety**: Flying enemies, fast scouts, armored tanks
- [ ] **Research Tree**: Unlock new technologies and upgrades
- [ ] **Multiple Maps**: Different starting layouts and challenges
- [ ] **Difficulty Modes**: Easy, Normal, Hard with different enemy spawn rates
- [ ] **Achievements**: Track milestones and special accomplishments
- [ ] **Leaderboards**: High scores based on survival time or efficiency

### Expanded Gameplay

- [ ] **Oil Resource**: Third resource for advanced production
- [ ] **Power System**: Buildings require power from generators
- [ ] **Conveyor Belts**: Automated resource transport system
- [ ] **Drones**: Flying units for scouting or light combat
- [ ] **Allied Structures**: Automated friendly units that help defend
- [ ] **Boss Enemies**: Special powerful enemies at certain intervals

### Multiplayer (Future)

- [ ] **Co-op Mode**: Multiple players defend shared beacon
- [ ] **Competitive Mode**: Race to survive longest or gather most resources
- [ ] **Shared Resources**: Team resource pool

## Bugs to Fix

- [ ] Zoom affects click targeting (need to adjust mouse position calculations)
- [ ] Enemies sometimes stack on same position
- [ ] Production queue doesn't show what's being built in UI
- [ ] Trucks can get stuck when selecting new ore patch while mining

## Performance Optimizations

- [ ] Spatial partitioning for collision detection
- [ ] Cull off-screen entities from rendering
- [ ] Optimize pathfinding for large truck counts
- [ ] Batch rendering for similar entities
