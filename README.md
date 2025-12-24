# Factory Tank Miner

A real-time strategy defense game where you manage mining trucks, build factories, and defend your beacon from waves of enemies.

## Overview

Factory Tank Miner is a resource management and tower defense hybrid. You start with three trucks and a beacon. Send trucks to mine iron and coal, build factories to produce weapons, and arm your trucks to defend against increasingly dangerous enemies that slowly approach your base.

## How to Play

### Objective

Survive as long as possible by mining resources, producing weapons, and defending your beacon from enemy attacks.

### Starting Resources

- **3 Trucks**: One armed with a gun and 200 bullets, two unarmed miners
- **1 Beacon**: Your base and resource drop-off point (indestructible for now)
- **2 Ore Patches**: Iron (left) and Coal (right)
- **5 Minutes**: Grace period before first enemies spawn

### Controls

**Mouse:**

- **Left Click**: Select individual truck or building
- **Left Drag**: Box select multiple trucks
- **Right Click**: Move selected trucks / Attack-move (for armed trucks)
- **Right Drag**: Pan the camera

**UI Buttons:**

- **Garage**: Build more trucks (20 Iron + 10 Coal, 5s)
- **Factory**: Produce guns and ammunition (100 Iron + 50 Coal to build)

**Zoom Controls** (unlocks after 200 seconds):

- **🔍+**: Zoom in
- **🔍-**: Zoom out

### Game Mechanics

#### Mining

1. Select trucks (click or drag to select)
2. Right-click on ore patches to send trucks mining
3. Trucks automatically mine until their cargo is full (64 units)
4. Full trucks automatically return to the beacon and unload
5. After unloading, trucks return to their last mining location

#### Production

1. Build a **Garage** to produce more trucks
2. Build a **Factory** to produce guns and bullet boxes
3. Select a building and queue production items
4. Trucks near factories automatically equip guns and load bullets (up to 400 bullets)

#### Combat

- Armed trucks (orange colored) automatically attack enemies within range (150 pixels)
- Each bullet deals 1 damage
- Enemies slowly move toward your beacon
- Enemies attack buildings when in range:
  - Small enemies: 1 damage/sec, 10 HP
  - Medium enemies: 2 damage/sec, 40 HP
  - Large enemies: 5 damage/sec, 100 HP
- Destroyed buildings are removed (except the beacon)

#### Resource Costs

| Item | Iron | Coal | Time |
|------|------|------|------|
| Truck | 20 | 10 | 5s |
| Gun | 30 | 5 | 8s |
| Bullets (100 rounds) | 5 | 10 | 3s |
| Garage | 50 | 30 | - |
| Factory | 100 | 50 | - |

### Visual Indicators

- **Orange trucks**: Armed with guns (shows "G200" for gun + ammo count)
- **Blue trucks**: Unarmed miners
- **Brown/Gray trucks**: Carrying iron/coal
- **Yellow bullet tracers**: When trucks fire
- **Red health bars**: Damaged buildings and enemies
- **Green progress bars**: Production progress on buildings
- **Yellow selection box**: Selected trucks

## Building & Running

### Prerequisites

- Rust toolchain (1.70+)
- On Windows: MSVC toolchain recommended

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Tips & Strategy

1. **Get Mining Early**: Send both unarmed trucks to mine immediately
2. **Build a Factory First**: You need guns and bullets before enemies arrive
3. **Arm Your Fleet**: Equip multiple trucks with guns for better defense
4. **Keep Production Running**: Queue multiple guns and bullet boxes
5. **Expand Carefully**: Balance between building trucks and producing weapons
6. **Protect Your Buildings**: Armed trucks can defend factories and garages
7. **Watch Your Ammo**: Trucks without bullets can't fight - keep production going

## Current Features

- ✅ Real-time resource mining and management
- ✅ Multiple truck control with selection and movement
- ✅ Building placement system (Garages and Factories)
- ✅ Production queue system
- ✅ Automatic weapon equipping and ammunition loading
- ✅ Enemy spawning with multiple size variants
- ✅ Automatic combat system for armed trucks
- ✅ Building health and destruction
- ✅ Camera panning and zoom
- ✅ Resource tracking and UI

## Technical Details

Built with:

- **Rust** - Systems programming language
- **egui** - Immediate mode GUI library
- **eframe** - Application framework for egui
- **rand** - Random number generation

## License

AGPL-3.0 License.
This project is open source. Feel free to fork, modify, and expand upon it!

**Enjoy the game! Defend your beacon and see how long you can survive!**
