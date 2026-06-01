# lau-construct-cli

**CLI toolkit for managing a PLATO construct — status dashboards, room inspection, deployment, and debug tracing, all rendered as ASCII art.**

A Rust library providing the data types and rendering primitives you'd need to build a CLI for the LAU Construct system. Every major type implements a `Render` trait that produces ASCII dashboard panels, making it trivial to build terminal-based status views, inspection tools, and deployment managers.

---

## What This Does

`lau-construct-cli` provides four subsystems:

1. **Status Dashboard** (`ConstructStatus`) — aggregate view of rooms, conservation, providers, and ports
2. **Room Inspector** (`ConstructInspector`) — deep inspection of individual rooms (tiles, gravity, deadband, provenance, circuits, correlations)
3. **Deployer** (`ConstructDeployer`) — deploy rooms and ensign profiles with manifest-based configuration
4. **Debugger** (`ConstructDebug`) — trace messages through the system and verify conservation laws

Everything renders as ASCII box-drawing panels suitable for terminal output.

---

## Key Idea

Every displayable type implements the `Render` trait:

```rust
pub trait Render {
    fn render(&self) -> String;  // Returns ASCII dashboard string
}
```

This means you can compose complex dashboards by nesting rendered panels. A `ConstructStatus` renders a full dashboard by embedding the rendered output of each `RoomStatus`, `ConservationStatus`, `ProviderStatus`, and `PortStatus`.

---

## Install

```toml
[dependencies]
lau-construct-cli = "0.1"
```

Or:

```sh
cargo add lau-construct-cli
```

### Requirements

- Rust 2021 edition or later
- `serde` (transitive)

---

## Quick Start

```rust
use lau_construct_cli::{
    ConstructStatus, ConservationStatus, RoomStatus,
    EnsignState, PortStatus, ProviderStatus,
    ConstructInspector, RoomInspection, RoomConfig,
    ConstructDeployer, RoomManifest, EnsignProfile,
    ConstructDebug,
};

fn main() {
    // 1. Build a status dashboard
    let status = ConstructStatus {
        rooms: vec![RoomStatus {
            id: "engineering-bay".to_string(),
            ensign_state: EnsignState::Active,
            gravity: 9.81,
            deadband: 0.05,
            tile_count: 42,
        }],
        conservation: ConservationStatus::new(1000.0, 450.0, 0.05),
        providers: vec![ProviderStatus {
            name: "openai".to_string(),
            available: true,
            models: vec!["gpt-4".to_string()],
            budget_remaining: 550.0,
        }],
        ports: vec![PortStatus {
            name: "telegram".to_string(),
            connected: true,
            messages_pending: 3,
        }],
        uptime_seconds: 3600,
        total_ticks: 1_000_000,
    };

    // 2. Render the dashboard
    println!("{}", status.render());

    // 3. Inspect a room
    let inspector = ConstructInspector::new();
    let room = inspector.inspect_room("engineering-bay");
    println!("{}", room.render());

    // 4. Deploy a room
    let mut deployer = ConstructDeployer::new();
    let manifest = RoomManifest {
        room_id: "bridge-1".to_string(),
        dimensions: [10.0, 10.0, 3.0],
        max_tiles: 100,
        gravity_setting: 1.0,
        ensign_profile: None,
    };
    let id = deployer.deploy_room(manifest).unwrap();

    // 5. Attach an ensign
    let profile = EnsignProfile {
        profile_name: "standard".to_string(),
        model: "v2".to_string(),
        priority: 5,
        flags: vec!["fast".to_string()],
    };
    deployer.deploy_ensign(&id, profile).unwrap();

    // 6. Debug: trace a message
    let debug = ConstructDebug::new();
    let trace = debug.trace_message("msg-42");
    for step in &trace {
        println!("{}", step.render());
    }

    // 7. Verify conservation
    let report = debug.verify_conservation();
    println!("{}", report.render());
}
```

---

## API Reference

### Status Types

#### `ConstructStatus`

The top-level dashboard.

```rust
pub struct ConstructStatus {
    pub rooms: Vec<RoomStatus>,
    pub conservation: ConservationStatus,
    pub providers: Vec<ProviderStatus>,
    pub ports: Vec<PortStatus>,
    pub uptime_seconds: u64,
    pub total_ticks: u64,
}
```

Implements `Render` → full ASCII dashboard with sections for rooms, conservation, providers, and ports.

#### `RoomStatus`

```rust
pub struct RoomStatus {
    pub id: String,
    pub ensign_state: EnsignState,
    pub gravity: f64,
    pub deadband: f64,
    pub tile_count: usize,
}
```

#### `EnsignState`

| Variant | Render |
|---------|--------|
| `Active` | `[ACTIVE]` |
| `Standby` | `[STANDBY]` |
| `Error` | `[ERROR!]` |
| `Offline` | `[OFFLINE]` |

#### `ConservationStatus`

```rust
pub struct ConservationStatus {
    pub total_budget: f64,
    pub total_spent: f64,
    pub degradation_level: f64,  // 0.0–1.0
}
```

Renders with a progress bar (`[##########----------]`).

#### `ProviderStatus`

```rust
pub struct ProviderStatus {
    pub name: String,
    pub available: bool,
    pub models: Vec<String>,
    pub budget_remaining: f64,
}
```

#### `PortStatus`

```rust
pub struct PortStatus {
    pub name: String,
    pub connected: bool,
    pub messages_pending: u64,
}
```

### Inspection Types

#### `ConstructInspector`

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `Self` | Create an empty inspector |
| `register_room(inspection)` | `()` | Register a room inspection |
| `inspect_room(room_id)` | `RoomInspection` | Get inspection (offline default if unknown) |

#### `RoomInspection`

Deep inspection result for a single room.

```rust
pub struct RoomInspection {
    pub room_config: RoomConfig,
    pub ensign_status: EnsignState,
    pub recent_tiles: Vec<Tile>,
    pub gravity_vector: [f64; 3],
    pub deadband_status: DeadbandStatus,
    pub provenance_entries: Vec<ProvenanceEntry>,
    pub circuit_status: CircuitStatus,
    pub penrose_correlations: Vec<PenroseCorrelation>,
}
```

Implements `Render` → full inspection panel with tiles, deadband, circuit, and correlations.

#### `RoomConfig`

```rust
pub struct RoomConfig {
    pub room_id: String,
    pub dimensions: [f64; 3],
    pub max_tiles: usize,
    pub gravity_setting: f64,
}
```

#### `Tile`

```rust
pub struct Tile {
    pub id: String,
    pub content: String,
    pub timestamp: u64,
}
```

Renders as `[timestamp] id | content`.

#### `DeadbandStatus`

```rust
pub struct DeadbandStatus {
    pub active: bool,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub current_value: f64,
}
```

#### `CircuitState` / `CircuitStatus`

```rust
pub enum CircuitState { Open, Closed, Tripped, Maintenance }
pub struct CircuitStatus {
    pub state: CircuitState,
    pub load: f64,
    pub temperature: f64,
}
```

#### `PenroseCorrelation`

```rust
pub struct PenroseCorrelation {
    pub tile_a: String,
    pub tile_b: String,
    pub correlation: f64,
}
```

Renders as `tile_a <-> tile_b : 0.9500`.

#### `ProvenanceEntry`

```rust
pub struct ProvenanceEntry {
    pub source: String,
    pub action: String,
    pub timestamp: u64,
    pub checksum: String,
}
```

### Deployer Types

#### `ConstructDeployer`

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `Self` | Create empty deployer |
| `deploy_room(config)` | `Result<String, DeployError>` | Deploy a room from manifest |
| `deploy_ensign(room_id, profile)` | `Result<String, DeployError>` | Attach ensign to room |
| `remove_room(room_id)` | `Result<(), DeployError>` | Remove a deployment |
| `list_deployments()` | `Vec<Deployment>` | All current deployments |

#### `RoomManifest`

```rust
pub struct RoomManifest {
    pub room_id: String,
    pub dimensions: [f64; 3],
    pub max_tiles: usize,
    pub gravity_setting: f64,
    pub ensign_profile: Option<EnsignProfile>,
}
```

#### `EnsignProfile`

```rust
pub struct EnsignProfile {
    pub profile_name: String,
    pub model: String,
    pub priority: u32,
    pub flags: Vec<String>,
}
```

#### `Deployment`

```rust
pub struct Deployment {
    pub room_id: String,
    pub manifest: RoomManifest,
    pub deployed_at: String,
}
```

Implements `Render`.

#### `DeployError`

| Variant | Meaning |
|---------|---------|
| `RoomNotFound(String)` | Room doesn't exist |
| `InvalidConfig(String)` | Bad configuration |
| `DeploymentFailed(String)` | Deployment failed |
| `AlreadyExists(String)` | Room already deployed |

Implements `Display` and `std::error::Error`.

### Debug Types

#### `ConstructDebug`

| Method | Returns | Description |
|--------|---------|-------------|
| `new()` | `Self` | Create debug toolkit |
| `trace_message(msg_id)` | `Vec<TraceStep>` | Trace a message (3-step: router → processor → emitter) |
| `verify_conservation()` | `ConservationReport` | Check conservation law |

#### `TraceStep`

```rust
pub struct TraceStep {
    pub timestamp: u64,
    pub agent: String,
    pub action: String,
    pub duration_ms: u64,
    pub energy_cost: f64,
}
```

Renders as `[timestamp] agent action Nms X.XXJ`.

#### `ConservationReport`

```rust
pub struct ConservationReport {
    pub law_holds: bool,
    pub total_in: f64,
    pub total_out: f64,
    pub discrepancy: f64,
    pub tolerance: f64,
}
```

Renders with PASS/FAIL status.

---

## How It Works

### The Render Trait

Every displayable type implements `Render`, which returns a pre-formatted ASCII string. The rendering follows a consistent pattern:

```
+------------------------------+
| Label: value                 |
| Label: value                 |
+------------------------------+
```

Composite types (like `ConstructStatus`) embed the rendered output of their sub-types, creating nested dashboard panels.

### Progress Bars

`ConservationStatus` uses a helper `render_bar(fraction, width)` that converts a 0.0–1.0 fraction into a visual bar:

```
[##########----------]  // 50% used, width=20
```

The bar is clamped to [0.0, 1.0] and rounded to the nearest whole cell.

### Inspector Pattern

`ConstructInspector` stores `RoomInspection` objects keyed by room ID. When you call `inspect_room()` for an unknown room, it returns a default "offline" inspection with zero values and `EnsignState::Offline`. This ensures the CLI always has something to display.

### Deployer State Machine

The deployer maintains a `HashMap<String, Deployment>`:

1. `deploy_room()` — validates room_id is non-empty and doesn't already exist, then inserts
2. `deploy_ensign()` — validates the room exists, then attaches the profile
3. `remove_room()` — validates the room exists, then removes
4. `list_deployments()` — returns all values

### Debug Tracing

`trace_message()` produces a fixed 3-step trace (router → processor → emitter) for non-empty message IDs. This is a simulation scaffold — in a real system, this would hook into the actual message pipeline.

---

## The Math

### Conservation Verification

The debug report checks:

```
|total_in - total_out| ≤ tolerance
```

If the discrepancy is within tolerance, `law_holds = true`. The default verification reports:

- `total_in = 1000.0`
- `total_out = 999.999`
- `discrepancy = 0.001`
- `tolerance = 0.01`

This demonstrates the pattern: near-perfect conservation with a small tolerance window.

### Deadband Control

The deadband controller status reports:

```
lower_bound ≤ current_value ≤ upper_bound
```

The "deadband" is the range where no corrective action is taken. Values within `[lower, upper]` are considered stable.

### Degradation Level

`ConservationStatus.degradation_level` is a 0.0–1.0 fraction:

```
degradation_level = how_degraded_the_system_is
```

Rendered as a percentage in the dashboard. This feeds into the broader LAU conservation system's graceful degradation strategy.

### Penrose Correlations

Tile correlations are simple Pearson-like coefficients:

```
correlation ∈ [-1.0, 1.0]
```

Where 1.0 means tiles are perfectly correlated, -1.0 means anti-correlated, and 0.0 means no relationship. In the LAU system, these represent the non-periodic tiling relationships that underlie the Penrose-style spatial organization.

---

## Testing

The library includes **71 unit tests** covering:

- All `EnsignState` and `CircuitState` variants and their render output
- `RoomStatus`, `ConservationStatus`, `ProviderStatus`, `PortStatus` rendering
- `ConservationStatus` progress bar at different fractions
- `ConstructStatus` empty and populated dashboard rendering
- `Tile`, `DeadbandStatus`, `ProvenanceEntry`, `PenroseCorrelation` serde
- `CircuitStatus` rendering with different states
- `RoomConfig`, `RoomInspection` serde and rendering
- `ConstructInspector` known-room lookup and unknown-room fallback
- `ConstructDeployer` full lifecycle: deploy, deploy ensign, list, remove
- Deployer error cases: empty ID, duplicate, not found
- `DeployError` display and `std::error::Error` implementation
- `ConstructDebug` trace (normal and empty message), conservation verification
- `TraceStep` and `ConservationReport` rendering
- Comprehensive "all renders are non-empty" coverage test
- Serde roundtrip for every serializable type

Run them:

```sh
cargo test
```

---

## License

MIT
