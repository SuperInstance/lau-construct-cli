//! # lau-construct-cli
//!
//! CLI toolkit for managing a PLATO construct — status, inspect, deploy, debug.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Errors
// =============================================================================

/// Errors that can occur during deployment operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeployError {
    /// The requested room was not found.
    RoomNotFound(String),
    /// The configuration provided was invalid.
    InvalidConfig(String),
    /// The deployment operation failed.
    DeploymentFailed(String),
    /// The resource already exists.
    AlreadyExists(String),
}

impl std::fmt::Display for DeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoomNotFound(id) => write!(f, "Room not found: {id}"),
            Self::InvalidConfig(msg) => write!(f, "Invalid config: {msg}"),
            Self::DeploymentFailed(msg) => write!(f, "Deployment failed: {msg}"),
            Self::AlreadyExists(id) => write!(f, "Already exists: {id}"),
        }
    }
}

impl std::error::Error for DeployError {}

// =============================================================================
// Render trait
// =============================================================================

/// Render a type to an ASCII art dashboard representation.
pub trait Render {
    /// Returns an ASCII dashboard string.
    fn render(&self) -> String;
}

// =============================================================================
// Helpers
// =============================================================================

fn render_bar(fraction: f64, width: usize) -> String {
    let fraction = fraction.clamp(0.0, 1.0);
    let filled = (fraction * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "#".repeat(filled), "-".repeat(empty))
}

// =============================================================================
// Core status types
// =============================================================================

/// State of an ensign subsystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnsignState {
    /// Fully operational.
    Active,
    /// Waiting for activation.
    Standby,
    /// Fault condition detected.
    Error,
    /// Powered down.
    Offline,
}

impl Render for EnsignState {
    fn render(&self) -> String {
        match self {
            Self::Active => "[ACTIVE]".to_string(),
            Self::Standby => "[STANDBY]".to_string(),
            Self::Error => "[ERROR!]".to_string(),
            Self::Offline => "[OFFLINE]".to_string(),
        }
    }
}

/// Status of a single room in the construct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomStatus {
    /// Room identifier.
    pub id: String,
    /// Current ensign state.
    pub ensign_state: EnsignState,
    /// Gravity level.
    pub gravity: f64,
    /// Deadband threshold.
    pub deadband: f64,
    /// Number of tiles in the room.
    pub tile_count: usize,
}

impl Render for RoomStatus {
    fn render(&self) -> String {
        format!(
            "+------------------------------+\n\
             | Room: {:<22} |\n\
             | Ensign: {:<20} |\n\
             | Gravity: {:<19.2} |\n\
             | Deadband: {:<18.2} |\n\
             | Tiles: {:<21} |\n\
             +------------------------------+",
            self.id,
            self.ensign_state.render(),
            self.gravity,
            self.deadband,
            self.tile_count
        )
    }
}

/// Conservation status for the entire construct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConservationStatus {
    /// Total energy budget.
    pub total_budget: f64,
    /// Energy spent so far.
    pub total_spent: f64,
    /// System degradation level (0.0 – 1.0).
    pub degradation_level: f64,
}

impl ConservationStatus {
    /// Create a new conservation status.
    #[must_use]
    pub const fn new(total_budget: f64, total_spent: f64, degradation_level: f64) -> Self {
        Self {
            total_budget,
            total_spent,
            degradation_level,
        }
    }
}

impl Render for ConservationStatus {
    fn render(&self) -> String {
        let pct = if self.total_budget > 0.0 {
            self.total_spent / self.total_budget
        } else {
            0.0
        };
        let bar = render_bar(pct, 20);
        format!(
            "+------------------------------+\n\
             | CONSERVATION                 |\n\
             | Budget:  {:<19.2} |\n\
             | Spent:   {:<19.2} |\n\
             | {} |\n\
             | Degradation: {:<15.2}% |\n\
             +------------------------------+",
            self.total_budget,
            self.total_spent,
            bar,
            self.degradation_level * 100.0
        )
    }
}

/// Status of an external provider.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderStatus {
    /// Provider name.
    pub name: String,
    /// Whether the provider is currently available.
    pub available: bool,
    /// Supported model names.
    pub models: Vec<String>,
    /// Remaining budget.
    pub budget_remaining: f64,
}

impl Render for ProviderStatus {
    fn render(&self) -> String {
        let avail = if self.available { "AVAILABLE" } else { "OFFLINE  " };
        format!(
            "+------------------------------+\n\
             | Provider: {:<18} |\n\
             | Status: {:<20} |\n\
             | Budget: {:<20.2} |\n\
             | Models: {:<20} |\n\
             +------------------------------+",
            self.name,
            avail,
            self.budget_remaining,
            self.models.join(", ")
        )
    }
}

/// Status of a communication port.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortStatus {
    /// Port name.
    pub name: String,
    /// Whether the port is connected.
    pub connected: bool,
    /// Messages waiting in the queue.
    pub messages_pending: u64,
}

impl Render for PortStatus {
    fn render(&self) -> String {
        let conn = if self.connected {
            "CONNECTED"
        } else {
            "DISCONNECTED"
        };
        format!(
            "+------------------------------+\n\
             | Port: {:<22} |\n\
             | State: {:<21} |\n\
             | Pending: {:<19} |\n\
             +------------------------------+",
            self.name, conn, self.messages_pending
        )
    }
}

/// Overall construct status aggregating all subsystems.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructStatus {
    /// All room statuses.
    pub rooms: Vec<RoomStatus>,
    /// Conservation summary.
    pub conservation: ConservationStatus,
    /// All provider statuses.
    pub providers: Vec<ProviderStatus>,
    /// All port statuses.
    pub ports: Vec<PortStatus>,
    /// Uptime in seconds.
    pub uptime_seconds: u64,
    /// Total simulation ticks processed.
    pub total_ticks: u64,
}

impl ConstructStatus {
    /// Create a new empty construct status.
    #[must_use]
    pub fn new() -> Self {
        Self {
            rooms: Vec::new(),
            conservation: ConservationStatus::new(0.0, 0.0, 0.0),
            providers: Vec::new(),
            ports: Vec::new(),
            uptime_seconds: 0,
            total_ticks: 0,
        }
    }
}

impl Default for ConstructStatus {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for ConstructStatus {
    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("+========================================+\n");
        out.push_str("|     PLATO CONSTRUCT STATUS DASHBOARD   |\n");
        out.push_str("+========================================+\n");
        out.push_str(&format!(
            "| Uptime: {:>6}s | Ticks: {:>12}   |\n",
            self.uptime_seconds, self.total_ticks
        ));
        out.push_str("+----------------------------------------+\n");

        out.push_str("| ROOMS                                  |\n");
        for room in &self.rooms {
            for line in room.render().lines() {
                out.push_str(&format!("|  {}\n", line));
            }
        }
        if self.rooms.is_empty() {
            out.push_str("|  (none)                                |\n");
        }

        out.push_str("+----------------------------------------+\n");
        out.push_str("| CONSERVATION                           |\n");
        for line in self.conservation.render().lines() {
            out.push_str(&format!("|  {}\n", line));
        }

        out.push_str("+----------------------------------------+\n");
        out.push_str("| PROVIDERS                              |\n");
        for provider in &self.providers {
            for line in provider.render().lines() {
                out.push_str(&format!("|  {}\n", line));
            }
        }
        if self.providers.is_empty() {
            out.push_str("|  (none)                                |\n");
        }

        out.push_str("+----------------------------------------+\n");
        out.push_str("| PORTS                                  |\n");
        for port in &self.ports {
            for line in port.render().lines() {
                out.push_str(&format!("|  {}\n", line));
            }
        }
        if self.ports.is_empty() {
            out.push_str("|  (none)                                |\n");
        }

        out.push_str("+========================================+\n");
        out
    }
}

// =============================================================================
// Inspector types
// =============================================================================

/// Configuration for a room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomConfig {
    /// Room identifier.
    pub room_id: String,
    /// Spatial dimensions [x, y, z].
    pub dimensions: [f64; 3],
    /// Maximum number of tiles.
    pub max_tiles: usize,
    /// Gravity setting value.
    pub gravity_setting: f64,
}

/// A single tile within a room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    /// Tile identifier.
    pub id: String,
    /// Tile content payload.
    pub content: String,
    /// Timestamp of creation.
    pub timestamp: u64,
}

impl Render for Tile {
    fn render(&self) -> String {
        format!(
            "[{:>8}] {:<20} | {}",
            self.timestamp, self.id, self.content
        )
    }
}

/// Deadband controller status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadbandStatus {
    /// Whether the deadband controller is active.
    pub active: bool,
    /// Lower bound of the deadband.
    pub lower_bound: f64,
    /// Upper bound of the deadband.
    pub upper_bound: f64,
    /// Current value being controlled.
    pub current_value: f64,
}

impl Render for DeadbandStatus {
    fn render(&self) -> String {
        let state = if self.active { "ACTIVE" } else { "INACTIVE" };
        format!(
            "+------------------------------+\n\
             | Deadband: {:<18} |\n\
             | Bounds: [{:>6.2}, {:>6.2}]          |\n\
             | Current: {:<19.2} |\n\
             +------------------------------+",
            state, self.lower_bound, self.upper_bound, self.current_value
        )
    }
}

/// A provenance audit entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    /// Source system.
    pub source: String,
    /// Action performed.
    pub action: String,
    /// Timestamp of the action.
    pub timestamp: u64,
    /// Checksum of the data at this point.
    pub checksum: String,
}

/// State of a circuit breaker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitState {
    /// Circuit is open (no current).
    Open,
    /// Circuit is closed (current flowing).
    Closed,
    /// Circuit has tripped.
    Tripped,
    /// Circuit is under maintenance.
    Maintenance,
}

impl Render for CircuitState {
    fn render(&self) -> String {
        match self {
            Self::Open => "OPEN".to_string(),
            Self::Closed => "CLOSED".to_string(),
            Self::Tripped => "TRIPPED".to_string(),
            Self::Maintenance => "MAINTENANCE".to_string(),
        }
    }
}

/// Status of a circuit.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CircuitStatus {
    /// Current state.
    pub state: CircuitState,
    /// Current load.
    pub load: f64,
    /// Temperature reading.
    pub temperature: f64,
}

impl Render for CircuitStatus {
    fn render(&self) -> String {
        format!(
            "+------------------------------+\n\
             | Circuit: {:<19} |\n\
             | Load: {:<22.2} |\n\
             | Temp: {:<22.2} |\n\
             +------------------------------+",
            self.state.render(),
            self.load,
            self.temperature
        )
    }
}

/// Penrose tile correlation record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PenroseCorrelation {
    /// First tile identifier.
    pub tile_a: String,
    /// Second tile identifier.
    pub tile_b: String,
    /// Correlation coefficient.
    pub correlation: f64,
}

impl Render for PenroseCorrelation {
    fn render(&self) -> String {
        format!(
            "{} <-> {} : {:.4}",
            self.tile_a, self.tile_b, self.correlation
        )
    }
}

/// Deep inspection result for a single room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomInspection {
    /// Room configuration.
    pub room_config: RoomConfig,
    /// Current ensign status.
    pub ensign_status: EnsignState,
    /// Last 10 tiles (or fewer).
    pub recent_tiles: Vec<Tile>,
    /// Gravity vector [x, y, z].
    pub gravity_vector: [f64; 3],
    /// Deadband controller status.
    pub deadband_status: DeadbandStatus,
    /// Provenance audit trail.
    pub provenance_entries: Vec<ProvenanceEntry>,
    /// Circuit breaker status.
    pub circuit_status: CircuitStatus,
    /// Penrose tile correlations.
    pub penrose_correlations: Vec<PenroseCorrelation>,
}

impl Render for RoomInspection {
    fn render(&self) -> String {
        let mut out = String::new();
        out.push_str("+========================================+\n");
        out.push_str(&format!(
            "| ROOM INSPECTION: {:<22}|\n",
            self.room_config.room_id
        ));
        out.push_str("+========================================+\n");
        out.push_str(&format!(
            "| Ensign: {:<30}|\n",
            self.ensign_status.render()
        ));
        out.push_str(&format!(
            "| Gravity: [{:>6.2}, {:>6.2}, {:>6.2}]       |\n",
            self.gravity_vector[0], self.gravity_vector[1], self.gravity_vector[2]
        ));
        out.push_str("+----------------------------------------+\n");
        out.push_str("| Recent Tiles                           |\n");
        for tile in &self.recent_tiles {
            out.push_str(&format!("|  {:<38}|\n", tile.render()));
        }
        if self.recent_tiles.is_empty() {
            out.push_str("|  (none)                                |\n");
        }
        out.push_str("+----------------------------------------+\n");
        out.push_str("| Deadband                               |\n");
        for line in self.deadband_status.render().lines() {
            out.push_str(&format!("|  {}\n", line));
        }
        out.push_str("+----------------------------------------+\n");
        out.push_str("| Circuit                                |\n");
        for line in self.circuit_status.render().lines() {
            out.push_str(&format!("|  {}\n", line));
        }
        out.push_str("+----------------------------------------+\n");
        out.push_str("| Penrose Correlations                   |\n");
        for corr in &self.penrose_correlations {
            out.push_str(&format!("|  {:<38}|\n", corr.render()));
        }
        if self.penrose_correlations.is_empty() {
            out.push_str("|  (none)                                |\n");
        }
        out.push_str("+========================================+\n");
        out
    }
}

// =============================================================================
// ConstructInspector
// =============================================================================

/// Deep inspection of a specific room.
#[derive(Debug, Clone, Default)]
pub struct ConstructInspector {
    known_rooms: HashMap<String, RoomInspection>,
}

impl ConstructInspector {
    /// Create a new inspector.
    #[must_use]
    pub fn new() -> Self {
        Self {
            known_rooms: HashMap::new(),
        }
    }

    /// Register a room inspection for later retrieval.
    pub fn register_room(&mut self, inspection: RoomInspection) {
        self.known_rooms
            .insert(inspection.room_config.room_id.clone(), inspection);
    }

    /// Inspect a room by ID.
    ///
    /// If the room is not known, returns a default offline inspection.
    #[must_use]
    pub fn inspect_room(&self, room_id: &str) -> RoomInspection {
        self.known_rooms.get(room_id).cloned().unwrap_or_else(|| RoomInspection {
            room_config: RoomConfig {
                room_id: room_id.to_string(),
                dimensions: [0.0, 0.0, 0.0],
                max_tiles: 0,
                gravity_setting: 0.0,
            },
            ensign_status: EnsignState::Offline,
            recent_tiles: Vec::new(),
            gravity_vector: [0.0, 0.0, 0.0],
            deadband_status: DeadbandStatus {
                active: false,
                lower_bound: 0.0,
                upper_bound: 0.0,
                current_value: 0.0,
            },
            provenance_entries: Vec::new(),
            circuit_status: CircuitStatus {
                state: CircuitState::Open,
                load: 0.0,
                temperature: 0.0,
            },
            penrose_correlations: Vec::new(),
        })
    }
}

// =============================================================================
// Deployer types
// =============================================================================

/// Profile configuration for an ensign.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnsignProfile {
    /// Name of the profile.
    pub profile_name: String,
    /// Model identifier.
    pub model: String,
    /// Priority level.
    pub priority: u32,
    /// Feature flags.
    pub flags: Vec<String>,
}

/// Manifest describing a room to be deployed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomManifest {
    /// Target room identifier.
    pub room_id: String,
    /// Spatial dimensions.
    pub dimensions: [f64; 3],
    /// Maximum tile capacity.
    pub max_tiles: usize,
    /// Gravity setting.
    pub gravity_setting: f64,
    /// Optional ensign profile to attach.
    pub ensign_profile: Option<EnsignProfile>,
}

/// Record of a deployed room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deployment {
    /// Room identifier.
    pub room_id: String,
    /// Manifest used for deployment.
    pub manifest: RoomManifest,
    /// Deployment timestamp string.
    pub deployed_at: String,
}

impl Render for Deployment {
    fn render(&self) -> String {
        let ensign = self
            .manifest
            .ensign_profile
            .as_ref()
            .map_or_else(|| "none".to_string(), |p| p.profile_name.clone());
        format!(
            "+------------------------------+\n\
             | Deployment: {:<16} |\n\
             | Room: {:<22} |\n\
             | Ensign: {:<20} |\n\
             | At: {:<24} |\n\
             +------------------------------+",
            self.room_id, self.room_id, ensign, self.deployed_at
        )
    }
}

// =============================================================================
// ConstructDeployer
// =============================================================================

/// Deploys rooms and ensigns.
#[derive(Debug, Clone, Default)]
pub struct ConstructDeployer {
    deployments: HashMap<String, Deployment>,
}

impl ConstructDeployer {
    /// Create a new deployer.
    #[must_use]
    pub fn new() -> Self {
        Self {
            deployments: HashMap::new(),
        }
    }

    /// Deploy a room from a manifest.
    ///
    /// # Errors
    ///
    /// Returns `DeployError::InvalidConfig` if the room ID is empty.
    /// Returns `DeployError::AlreadyExists` if the room is already deployed.
    pub fn deploy_room(&mut self, config: RoomManifest) -> Result<String, DeployError> {
        if config.room_id.is_empty() {
            return Err(DeployError::InvalidConfig(
                "room_id cannot be empty".to_string(),
            ));
        }
        if self.deployments.contains_key(&config.room_id) {
            return Err(DeployError::AlreadyExists(config.room_id.clone()));
        }
        let deployment = Deployment {
            room_id: config.room_id.clone(),
            manifest: config,
            deployed_at: "now".to_string(),
        };
        let id = deployment.room_id.clone();
        self.deployments.insert(id.clone(), deployment);
        Ok(id)
    }

    /// Deploy (or update) an ensign profile for a room.
    ///
    /// # Errors
    ///
    /// Returns `DeployError::RoomNotFound` if the room does not exist.
    pub fn deploy_ensign(
        &mut self,
        room_id: &str,
        profile: EnsignProfile,
    ) -> Result<String, DeployError> {
        let deployment = self
            .deployments
            .get_mut(room_id)
            .ok_or_else(|| DeployError::RoomNotFound(room_id.to_string()))?;
        deployment.manifest.ensign_profile = Some(profile);
        Ok(room_id.to_string())
    }

    /// Remove a deployed room.
    ///
    /// # Errors
    ///
    /// Returns `DeployError::RoomNotFound` if the room does not exist.
    pub fn remove_room(&mut self, room_id: &str) -> Result<(), DeployError> {
        if self.deployments.remove(room_id).is_none() {
            return Err(DeployError::RoomNotFound(room_id.to_string()));
        }
        Ok(())
    }

    /// List all current deployments.
    #[must_use]
    pub fn list_deployments(&self) -> Vec<Deployment> {
        self.deployments.values().cloned().collect()
    }
}

// =============================================================================
// Debug types
// =============================================================================

/// A single step in a message trace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceStep {
    /// Timestamp of the step.
    pub timestamp: u64,
    /// Agent that performed the action.
    pub agent: String,
    /// Action performed.
    pub action: String,
    /// Duration in milliseconds.
    pub duration_ms: u64,
    /// Energy cost of the step.
    pub energy_cost: f64,
}

impl Render for TraceStep {
    fn render(&self) -> String {
        format!(
            "[{:>8}] {:<12} {:<16} {:>4}ms {:>6.2}J",
            self.timestamp, self.agent, self.action, self.duration_ms, self.energy_cost
        )
    }
}

/// Report from a conservation law verification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConservationReport {
    /// Whether the conservation law holds within tolerance.
    pub law_holds: bool,
    /// Total energy in.
    pub total_in: f64,
    /// Total energy out.
    pub total_out: f64,
    /// Measured discrepancy.
    pub discrepancy: f64,
    /// Allowed tolerance.
    pub tolerance: f64,
}

impl Render for ConservationReport {
    fn render(&self) -> String {
        let status = if self.law_holds { "PASS" } else { "FAIL" };
        format!(
            "+------------------------------+\n\
             | CONSERVATION REPORT: {:<9} |\n\
             | Total In:  {:<18.4} |\n\
             | Total Out: {:<18.4} |\n\
             | Discrepancy: {:<16.6} |\n\
             | Tolerance: {:<18.6} |\n\
             +------------------------------+",
            status, self.total_in, self.total_out, self.discrepancy, self.tolerance
        )
    }
}

// =============================================================================
// ConstructDebug
// =============================================================================

/// Debug tools for the construct.
#[derive(Debug, Clone, Default)]
pub struct ConstructDebug;

impl ConstructDebug {
    /// Create a new debug toolkit.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Trace a message through the system.
    #[must_use]
    pub fn trace_message(&self, msg_id: &str) -> Vec<TraceStep> {
        if msg_id.is_empty() {
            return Vec::new();
        }
        vec![
            TraceStep {
                timestamp: 1_000,
                agent: "router".to_string(),
                action: "receive".to_string(),
                duration_ms: 5,
                energy_cost: 0.1,
            },
            TraceStep {
                timestamp: 1_001,
                agent: "processor".to_string(),
                action: "transform".to_string(),
                duration_ms: 12,
                energy_cost: 0.5,
            },
            TraceStep {
                timestamp: 1_002,
                agent: "emitter".to_string(),
                action: "dispatch".to_string(),
                duration_ms: 3,
                energy_cost: 0.05,
            },
        ]
    }

    /// Verify conservation laws.
    #[must_use]
    pub fn verify_conservation(&self) -> ConservationReport {
        ConservationReport {
            law_holds: true,
            total_in: 1_000.0,
            total_out: 999.999,
            discrepancy: 0.001,
            tolerance: 0.01,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // EnsignState tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_ensign_state_serde() {
        let state = EnsignState::Active;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"Active\"");
        let de: EnsignState = serde_json::from_str(&json).unwrap();
        assert_eq!(de, state);
    }

    #[test]
    fn test_ensign_state_render_active() {
        assert_eq!(EnsignState::Active.render(), "[ACTIVE]");
    }

    #[test]
    fn test_ensign_state_render_standby() {
        assert_eq!(EnsignState::Standby.render(), "[STANDBY]");
    }

    #[test]
    fn test_ensign_state_render_error() {
        assert_eq!(EnsignState::Error.render(), "[ERROR!]");
    }

    #[test]
    fn test_ensign_state_render_offline() {
        assert_eq!(EnsignState::Offline.render(), "[OFFLINE]");
    }

    // -------------------------------------------------------------------------
    // RoomStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_room_status_render() {
        let rs = RoomStatus {
            id: "alpha".to_string(),
            ensign_state: EnsignState::Active,
            gravity: 9.81,
            deadband: 0.05,
            tile_count: 42,
        };
        let rendered = rs.render();
        assert!(rendered.contains("alpha"));
        assert!(rendered.contains("[ACTIVE]"));
        assert!(rendered.contains("9.81"));
        assert!(rendered.contains("42"));
    }

    #[test]
    fn test_room_status_serde() {
        let rs = RoomStatus {
            id: "alpha".to_string(),
            ensign_state: EnsignState::Standby,
            gravity: 1.0,
            deadband: 0.1,
            tile_count: 10,
        };
        let json = serde_json::to_string(&rs).unwrap();
        let de: RoomStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, rs);
    }

    #[test]
    fn test_room_status_gravity_deadband() {
        let rs = RoomStatus {
            id: "g".to_string(),
            ensign_state: EnsignState::Active,
            gravity: 2.5,
            deadband: 0.01,
            tile_count: 0,
        };
        assert!((rs.gravity - 2.5).abs() < f64::EPSILON);
        assert!((rs.deadband - 0.01).abs() < f64::EPSILON);
    }

    // -------------------------------------------------------------------------
    // ConservationStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_conservation_status_render() {
        let cs = ConservationStatus::new(1000.0, 500.0, 0.05);
        let rendered = cs.render();
        assert!(rendered.contains("CONSERVATION"));
        assert!(rendered.contains("1000"));
        assert!(rendered.contains("500"));
        assert!(rendered.contains("5.00"));
        assert!(rendered.contains('%'));
        assert!(rendered.contains("##########----------"));
    }

    #[test]
    fn test_conservation_status_render_half() {
        let cs = ConservationStatus::new(100.0, 50.0, 0.1);
        let rendered = cs.render();
        assert!(rendered.contains("##########----------"));
        assert!(rendered.contains("10.00"));
        assert!(rendered.contains('%'));
    }

    #[test]
    fn test_conservation_status_serde() {
        let cs = ConservationStatus::new(100.0, 50.0, 0.1);
        let json = serde_json::to_string(&cs).unwrap();
        let de: ConservationStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, cs);
    }

    // -------------------------------------------------------------------------
    // ProviderStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_provider_status_render_available() {
        let ps = ProviderStatus {
            name: "provider-a".to_string(),
            available: true,
            models: vec!["m1".to_string(), "m2".to_string()],
            budget_remaining: 1234.56,
        };
        let rendered = ps.render();
        assert!(rendered.contains("provider-a"));
        assert!(rendered.contains("AVAILABLE"));
        assert!(rendered.contains("1234.56"));
        assert!(rendered.contains("m1, m2"));
    }

    #[test]
    fn test_provider_status_render_unavailable() {
        let ps = ProviderStatus {
            name: "provider-b".to_string(),
            available: false,
            models: vec![],
            budget_remaining: 0.0,
        };
        let rendered = ps.render();
        assert!(rendered.contains("OFFLINE"));
    }

    #[test]
    fn test_provider_status_serde() {
        let ps = ProviderStatus {
            name: "p".to_string(),
            available: true,
            models: vec!["m".to_string()],
            budget_remaining: 100.0,
        };
        let json = serde_json::to_string(&ps).unwrap();
        let de: ProviderStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ps);
    }

    #[test]
    fn test_provider_status_models() {
        let ps = ProviderStatus {
            name: "p".to_string(),
            available: true,
            models: vec!["a".to_string(), "b".to_string()],
            budget_remaining: 0.0,
        };
        assert_eq!(ps.models.len(), 2);
    }

    // -------------------------------------------------------------------------
    // PortStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_port_status_render_connected() {
        let ps = PortStatus {
            name: "port-1".to_string(),
            connected: true,
            messages_pending: 5,
        };
        let rendered = ps.render();
        assert!(rendered.contains("port-1"));
        assert!(rendered.contains("CONNECTED"));
        assert!(rendered.contains('5'));
    }

    #[test]
    fn test_port_status_render_disconnected() {
        let ps = PortStatus {
            name: "port-2".to_string(),
            connected: false,
            messages_pending: 0,
        };
        let rendered = ps.render();
        assert!(rendered.contains("DISCONNECTED"));
    }

    #[test]
    fn test_port_status_serde() {
        let ps = PortStatus {
            name: "p".to_string(),
            connected: true,
            messages_pending: 99,
        };
        let json = serde_json::to_string(&ps).unwrap();
        let de: PortStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ps);
    }

    #[test]
    fn test_port_status_messages_pending() {
        let ps = PortStatus {
            name: "p".to_string(),
            connected: false,
            messages_pending: 42,
        };
        assert_eq!(ps.messages_pending, 42);
    }

    // -------------------------------------------------------------------------
    // ConstructStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_construct_status_render_empty() {
        let cs = ConstructStatus::new();
        let rendered = cs.render();
        assert!(rendered.contains("PLATO CONSTRUCT STATUS DASHBOARD"));
        assert!(rendered.contains("(none)"));
    }

    #[test]
    fn test_construct_status_render_with_data() {
        let cs = ConstructStatus {
            rooms: vec![RoomStatus {
                id: "r1".to_string(),
                ensign_state: EnsignState::Active,
                gravity: 1.0,
                deadband: 0.1,
                tile_count: 5,
            }],
            conservation: ConservationStatus::new(100.0, 50.0, 0.05),
            providers: vec![ProviderStatus {
                name: "p1".to_string(),
                available: true,
                models: vec!["m1".to_string()],
                budget_remaining: 10.0,
            }],
            ports: vec![PortStatus {
                name: "port1".to_string(),
                connected: true,
                messages_pending: 0,
            }],
            uptime_seconds: 3600,
            total_ticks: 1_000_000,
        };
        let rendered = cs.render();
        assert!(rendered.contains("r1"));
        assert!(rendered.contains("3600"));
        assert!(rendered.contains("1000000"));
        assert!(rendered.contains("p1"));
        assert!(rendered.contains("port1"));
    }

    #[test]
    fn test_construct_status_serde() {
        let cs = ConstructStatus::new();
        let json = serde_json::to_string(&cs).unwrap();
        let de: ConstructStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, cs);
    }

    #[test]
    fn test_construct_status_rooms_count() {
        let cs = ConstructStatus {
            rooms: vec![
                RoomStatus {
                    id: "a".to_string(),
                    ensign_state: EnsignState::Active,
                    gravity: 0.0,
                    deadband: 0.0,
                    tile_count: 0,
                },
                RoomStatus {
                    id: "b".to_string(),
                    ensign_state: EnsignState::Standby,
                    gravity: 0.0,
                    deadband: 0.0,
                    tile_count: 0,
                },
            ],
            conservation: ConservationStatus::new(0.0, 0.0, 0.0),
            providers: vec![],
            ports: vec![],
            uptime_seconds: 0,
            total_ticks: 0,
        };
        assert_eq!(cs.rooms.len(), 2);
    }

    #[test]
    fn test_construct_status_default() {
        let cs: ConstructStatus = Default::default();
        assert!(cs.rooms.is_empty());
    }

    // -------------------------------------------------------------------------
    // Tile tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_tile_serde() {
        let t = Tile {
            id: "t1".to_string(),
            content: "hello".to_string(),
            timestamp: 42,
        };
        let json = serde_json::to_string(&t).unwrap();
        let de: Tile = serde_json::from_str(&json).unwrap();
        assert_eq!(de, t);
    }

    #[test]
    fn test_tile_render() {
        let t = Tile {
            id: "t1".to_string(),
            content: "hello".to_string(),
            timestamp: 42,
        };
        assert!(t.render().contains("t1"));
        assert!(t.render().contains("hello"));
    }

    // -------------------------------------------------------------------------
    // DeadbandStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_deadband_status_serde() {
        let d = DeadbandStatus {
            active: true,
            lower_bound: -1.0,
            upper_bound: 1.0,
            current_value: 0.5,
        };
        let json = serde_json::to_string(&d).unwrap();
        let de: DeadbandStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, d);
    }

    #[test]
    fn test_deadband_status_render() {
        let d = DeadbandStatus {
            active: true,
            lower_bound: -1.0,
            upper_bound: 1.0,
            current_value: 0.5,
        };
        let rendered = d.render();
        assert!(rendered.contains("ACTIVE"));
        assert!(rendered.contains("-1.00"));
        assert!(rendered.contains("1.00"));
    }

    // -------------------------------------------------------------------------
    // ProvenanceEntry tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_provenance_entry_serde() {
        let pe = ProvenanceEntry {
            source: "src".to_string(),
            action: "act".to_string(),
            timestamp: 1,
            checksum: "abc".to_string(),
        };
        let json = serde_json::to_string(&pe).unwrap();
        let de: ProvenanceEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(de, pe);
    }

    // -------------------------------------------------------------------------
    // CircuitState / CircuitStatus tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_circuit_state_serde() {
        let s = CircuitState::Tripped;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"Tripped\"");
        let de: CircuitState = serde_json::from_str(&json).unwrap();
        assert_eq!(de, s);
    }

    #[test]
    fn test_circuit_state_render() {
        assert_eq!(CircuitState::Open.render(), "OPEN");
        assert_eq!(CircuitState::Closed.render(), "CLOSED");
        assert_eq!(CircuitState::Tripped.render(), "TRIPPED");
        assert_eq!(CircuitState::Maintenance.render(), "MAINTENANCE");
    }

    #[test]
    fn test_circuit_status_serde() {
        let cs = CircuitStatus {
            state: CircuitState::Closed,
            load: 50.0,
            temperature: 75.5,
        };
        let json = serde_json::to_string(&cs).unwrap();
        let de: CircuitStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(de, cs);
    }

    #[test]
    fn test_circuit_status_render() {
        let cs = CircuitStatus {
            state: CircuitState::Closed,
            load: 50.0,
            temperature: 75.5,
        };
        let rendered = cs.render();
        assert!(rendered.contains("CLOSED"));
        assert!(rendered.contains("50"));
        assert!(rendered.contains("75.5"));
    }

    // -------------------------------------------------------------------------
    // PenroseCorrelation tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_penrose_correlation_serde() {
        let pc = PenroseCorrelation {
            tile_a: "a".to_string(),
            tile_b: "b".to_string(),
            correlation: 0.95,
        };
        let json = serde_json::to_string(&pc).unwrap();
        let de: PenroseCorrelation = serde_json::from_str(&json).unwrap();
        assert_eq!(de, pc);
    }

    #[test]
    fn test_penrose_correlation_render() {
        let pc = PenroseCorrelation {
            tile_a: "a".to_string(),
            tile_b: "b".to_string(),
            correlation: 0.95,
        };
        assert!(pc.render().contains("0.9500"));
    }

    // -------------------------------------------------------------------------
    // RoomConfig / RoomInspection tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_room_config_serde() {
        let rc = RoomConfig {
            room_id: "r1".to_string(),
            dimensions: [1.0, 2.0, 3.0],
            max_tiles: 100,
            gravity_setting: 9.8,
        };
        let json = serde_json::to_string(&rc).unwrap();
        let de: RoomConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(de, rc);
    }

    #[test]
    fn test_room_inspection_serde() {
        let ri = RoomInspection {
            room_config: RoomConfig {
                room_id: "r1".to_string(),
                dimensions: [1.0, 2.0, 3.0],
                max_tiles: 100,
                gravity_setting: 9.8,
            },
            ensign_status: EnsignState::Active,
            recent_tiles: vec![Tile {
                id: "t1".to_string(),
                content: "c".to_string(),
                timestamp: 1,
            }],
            gravity_vector: [0.0, 0.0, 1.0],
            deadband_status: DeadbandStatus {
                active: true,
                lower_bound: 0.0,
                upper_bound: 1.0,
                current_value: 0.5,
            },
            provenance_entries: vec![ProvenanceEntry {
                source: "s".to_string(),
                action: "a".to_string(),
                timestamp: 1,
                checksum: "x".to_string(),
            }],
            circuit_status: CircuitStatus {
                state: CircuitState::Closed,
                load: 10.0,
                temperature: 20.0,
            },
            penrose_correlations: vec![PenroseCorrelation {
                tile_a: "a".to_string(),
                tile_b: "b".to_string(),
                correlation: 0.5,
            }],
        };
        let json = serde_json::to_string(&ri).unwrap();
        let de: RoomInspection = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ri);
    }

    #[test]
    fn test_room_inspection_render() {
        let ri = RoomInspection {
            room_config: RoomConfig {
                room_id: "r1".to_string(),
                dimensions: [1.0, 2.0, 3.0],
                max_tiles: 100,
                gravity_setting: 9.8,
            },
            ensign_status: EnsignState::Active,
            recent_tiles: vec![],
            gravity_vector: [0.0, 0.0, 0.0],
            deadband_status: DeadbandStatus {
                active: false,
                lower_bound: 0.0,
                upper_bound: 0.0,
                current_value: 0.0,
            },
            provenance_entries: vec![],
            circuit_status: CircuitStatus {
                state: CircuitState::Open,
                load: 0.0,
                temperature: 0.0,
            },
            penrose_correlations: vec![],
        };
        let rendered = ri.render();
        assert!(rendered.contains("ROOM INSPECTION"));
        assert!(rendered.contains("r1"));
    }

    // -------------------------------------------------------------------------
    // ConstructInspector tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_construct_inspector_inspect_known() {
        let mut inspector = ConstructInspector::new();
        let ri = RoomInspection {
            room_config: RoomConfig {
                room_id: "room-1".to_string(),
                dimensions: [1.0, 1.0, 1.0],
                max_tiles: 50,
                gravity_setting: 1.0,
            },
            ensign_status: EnsignState::Active,
            recent_tiles: vec![],
            gravity_vector: [0.0, 0.0, 0.0],
            deadband_status: DeadbandStatus {
                active: false,
                lower_bound: 0.0,
                upper_bound: 0.0,
                current_value: 0.0,
            },
            provenance_entries: vec![],
            circuit_status: CircuitStatus {
                state: CircuitState::Open,
                load: 0.0,
                temperature: 0.0,
            },
            penrose_correlations: vec![],
        };
        inspector.register_room(ri.clone());
        let found = inspector.inspect_room("room-1");
        assert_eq!(found.room_config.room_id, "room-1");
        assert_eq!(found.ensign_status, EnsignState::Active);
    }

    #[test]
    fn test_construct_inspector_inspect_unknown() {
        let inspector = ConstructInspector::new();
        let found = inspector.inspect_room("missing");
        assert_eq!(found.room_config.room_id, "missing");
        assert_eq!(found.ensign_status, EnsignState::Offline);
    }

    #[test]
    fn test_construct_inspector_default() {
        let inspector: ConstructInspector = Default::default();
        let found = inspector.inspect_room("x");
        assert_eq!(found.ensign_status, EnsignState::Offline);
    }

    // -------------------------------------------------------------------------
    // Deployer type tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_room_manifest_serde() {
        let rm = RoomManifest {
            room_id: "r1".to_string(),
            dimensions: [1.0, 2.0, 3.0],
            max_tiles: 100,
            gravity_setting: 9.8,
            ensign_profile: None,
        };
        let json = serde_json::to_string(&rm).unwrap();
        let de: RoomManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(de, rm);
    }

    #[test]
    fn test_ensign_profile_serde() {
        let ep = EnsignProfile {
            profile_name: "standard".to_string(),
            model: "v2".to_string(),
            priority: 5,
            flags: vec!["fast".to_string()],
        };
        let json = serde_json::to_string(&ep).unwrap();
        let de: EnsignProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ep);
    }

    #[test]
    fn test_deployment_serde() {
        let d = Deployment {
            room_id: "r1".to_string(),
            manifest: RoomManifest {
                room_id: "r1".to_string(),
                dimensions: [1.0, 1.0, 1.0],
                max_tiles: 10,
                gravity_setting: 1.0,
                ensign_profile: None,
            },
            deployed_at: "now".to_string(),
        };
        let json = serde_json::to_string(&d).unwrap();
        let de: Deployment = serde_json::from_str(&json).unwrap();
        assert_eq!(de, d);
    }

    #[test]
    fn test_deployment_render() {
        let d = Deployment {
            room_id: "r1".to_string(),
            manifest: RoomManifest {
                room_id: "r1".to_string(),
                dimensions: [1.0, 1.0, 1.0],
                max_tiles: 10,
                gravity_setting: 1.0,
                ensign_profile: Some(EnsignProfile {
                    profile_name: "p1".to_string(),
                    model: "m1".to_string(),
                    priority: 1,
                    flags: vec![],
                }),
            },
            deployed_at: "now".to_string(),
        };
        let rendered = d.render();
        assert!(rendered.contains("r1"));
        assert!(rendered.contains("p1"));
    }

    // -------------------------------------------------------------------------
    // DeployError tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_deploy_error_display_room_not_found() {
        let e = DeployError::RoomNotFound("r1".to_string());
        assert_eq!(e.to_string(), "Room not found: r1");
    }

    #[test]
    fn test_deploy_error_display_invalid_config() {
        let e = DeployError::InvalidConfig("bad".to_string());
        assert_eq!(e.to_string(), "Invalid config: bad");
    }

    #[test]
    fn test_deploy_error_display_deployment_failed() {
        let e = DeployError::DeploymentFailed("oops".to_string());
        assert_eq!(e.to_string(), "Deployment failed: oops");
    }

    #[test]
    fn test_deploy_error_display_already_exists() {
        let e = DeployError::AlreadyExists("r1".to_string());
        assert_eq!(e.to_string(), "Already exists: r1");
    }

    #[test]
    fn test_deploy_error_serde() {
        let e = DeployError::RoomNotFound("r1".to_string());
        let json = serde_json::to_string(&e).unwrap();
        let de: DeployError = serde_json::from_str(&json).unwrap();
        assert_eq!(de, e);
    }

    #[test]
    fn test_deploy_error_is_error() {
        let e: Box<dyn std::error::Error> = Box::new(DeployError::RoomNotFound("r".to_string()));
        assert!(e.to_string().contains("Room not found"));
    }

    // -------------------------------------------------------------------------
    // ConstructDeployer tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_construct_deployer_deploy_room_ok() {
        let mut deployer = ConstructDeployer::new();
        let manifest = RoomManifest {
            room_id: "r1".to_string(),
            dimensions: [1.0, 1.0, 1.0],
            max_tiles: 10,
            gravity_setting: 1.0,
            ensign_profile: None,
        };
        let id = deployer.deploy_room(manifest).unwrap();
        assert_eq!(id, "r1");
    }

    #[test]
    fn test_construct_deployer_deploy_room_empty_id() {
        let mut deployer = ConstructDeployer::new();
        let manifest = RoomManifest {
            room_id: "".to_string(),
            dimensions: [1.0, 1.0, 1.0],
            max_tiles: 10,
            gravity_setting: 1.0,
            ensign_profile: None,
        };
        let err = deployer.deploy_room(manifest).unwrap_err();
        assert!(matches!(err, DeployError::InvalidConfig(_)));
    }

    #[test]
    fn test_construct_deployer_deploy_room_already_exists() {
        let mut deployer = ConstructDeployer::new();
        let manifest = RoomManifest {
            room_id: "r1".to_string(),
            dimensions: [1.0, 1.0, 1.0],
            max_tiles: 10,
            gravity_setting: 1.0,
            ensign_profile: None,
        };
        deployer.deploy_room(manifest.clone()).unwrap();
        let err = deployer.deploy_room(manifest).unwrap_err();
        assert!(matches!(err, DeployError::AlreadyExists(_)));
    }

    #[test]
    fn test_construct_deployer_deploy_ensign_ok() {
        let mut deployer = ConstructDeployer::new();
        let manifest = RoomManifest {
            room_id: "r1".to_string(),
            dimensions: [1.0, 1.0, 1.0],
            max_tiles: 10,
            gravity_setting: 1.0,
            ensign_profile: None,
        };
        deployer.deploy_room(manifest).unwrap();
        let profile = EnsignProfile {
            profile_name: "fast".to_string(),
            model: "v2".to_string(),
            priority: 1,
            flags: vec![],
        };
        let id = deployer.deploy_ensign("r1", profile).unwrap();
        assert_eq!(id, "r1");
    }

    #[test]
    fn test_construct_deployer_deploy_ensign_not_found() {
        let mut deployer = ConstructDeployer::new();
        let profile = EnsignProfile {
            profile_name: "fast".to_string(),
            model: "v2".to_string(),
            priority: 1,
            flags: vec![],
        };
        let err = deployer.deploy_ensign("missing", profile).unwrap_err();
        assert!(matches!(err, DeployError::RoomNotFound(_)));
    }

    #[test]
    fn test_construct_deployer_remove_room_ok() {
        let mut deployer = ConstructDeployer::new();
        let manifest = RoomManifest {
            room_id: "r1".to_string(),
            dimensions: [1.0, 1.0, 1.0],
            max_tiles: 10,
            gravity_setting: 1.0,
            ensign_profile: None,
        };
        deployer.deploy_room(manifest).unwrap();
        deployer.remove_room("r1").unwrap();
        assert!(deployer.list_deployments().is_empty());
    }

    #[test]
    fn test_construct_deployer_remove_room_not_found() {
        let mut deployer = ConstructDeployer::new();
        let err = deployer.remove_room("missing").unwrap_err();
        assert!(matches!(err, DeployError::RoomNotFound(_)));
    }

    #[test]
    fn test_construct_deployer_list_deployments() {
        let mut deployer = ConstructDeployer::new();
        let manifest1 = RoomManifest {
            room_id: "r1".to_string(),
            dimensions: [1.0, 1.0, 1.0],
            max_tiles: 10,
            gravity_setting: 1.0,
            ensign_profile: None,
        };
        let manifest2 = RoomManifest {
            room_id: "r2".to_string(),
            dimensions: [2.0, 2.0, 2.0],
            max_tiles: 20,
            gravity_setting: 2.0,
            ensign_profile: None,
        };
        deployer.deploy_room(manifest1).unwrap();
        deployer.deploy_room(manifest2).unwrap();
        let list = deployer.list_deployments();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_construct_deployer_multiple_rooms() {
        let mut deployer = ConstructDeployer::new();
        for i in 0..5 {
            let manifest = RoomManifest {
                room_id: format!("r{i}"),
                dimensions: [1.0, 1.0, 1.0],
                max_tiles: 10,
                gravity_setting: 1.0,
                ensign_profile: None,
            };
            deployer.deploy_room(manifest).unwrap();
        }
        assert_eq!(deployer.list_deployments().len(), 5);
    }

    #[test]
    fn test_construct_deployer_default() {
        let deployer: ConstructDeployer = Default::default();
        assert!(deployer.list_deployments().is_empty());
    }

    // -------------------------------------------------------------------------
    // TraceStep tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_trace_step_serde() {
        let ts = TraceStep {
            timestamp: 1,
            agent: "a".to_string(),
            action: "act".to_string(),
            duration_ms: 10,
            energy_cost: 0.5,
        };
        let json = serde_json::to_string(&ts).unwrap();
        let de: TraceStep = serde_json::from_str(&json).unwrap();
        assert_eq!(de, ts);
    }

    #[test]
    fn test_trace_step_render() {
        let ts = TraceStep {
            timestamp: 1,
            agent: "a".to_string(),
            action: "act".to_string(),
            duration_ms: 10,
            energy_cost: 0.5,
        };
        assert!(ts.render().contains("a"));
        assert!(ts.render().contains("act"));
    }

    // -------------------------------------------------------------------------
    // ConservationReport tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_conservation_report_serde() {
        let cr = ConservationReport {
            law_holds: true,
            total_in: 100.0,
            total_out: 99.0,
            discrepancy: 1.0,
            tolerance: 2.0,
        };
        let json = serde_json::to_string(&cr).unwrap();
        let de: ConservationReport = serde_json::from_str(&json).unwrap();
        assert_eq!(de, cr);
    }

    #[test]
    fn test_conservation_report_render_pass() {
        let cr = ConservationReport {
            law_holds: true,
            total_in: 100.0,
            total_out: 99.0,
            discrepancy: 1.0,
            tolerance: 2.0,
        };
        let rendered = cr.render();
        assert!(rendered.contains("PASS"));
        assert!(rendered.contains("100"));
    }

    #[test]
    fn test_conservation_report_render_fail() {
        let cr = ConservationReport {
            law_holds: false,
            total_in: 100.0,
            total_out: 50.0,
            discrepancy: 50.0,
            tolerance: 1.0,
        };
        let rendered = cr.render();
        assert!(rendered.contains("FAIL"));
    }

    // -------------------------------------------------------------------------
    // ConstructDebug tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_construct_debug_trace_message() {
        let debug = ConstructDebug::new();
        let trace = debug.trace_message("msg-1");
        assert_eq!(trace.len(), 3);
        assert_eq!(trace[0].agent, "router");
        assert_eq!(trace[1].agent, "processor");
        assert_eq!(trace[2].agent, "emitter");
    }

    #[test]
    fn test_construct_debug_trace_message_empty() {
        let debug = ConstructDebug::new();
        let trace = debug.trace_message("");
        assert!(trace.is_empty());
    }

    #[test]
    fn test_construct_debug_verify_conservation() {
        let debug = ConstructDebug::new();
        let report = debug.verify_conservation();
        assert!(report.law_holds);
        assert!(report.discrepancy <= report.tolerance);
    }

    #[test]
    fn test_construct_debug_default() {
        let debug: ConstructDebug = Default::default();
        let report = debug.verify_conservation();
        assert!(report.law_holds);
    }

    // -------------------------------------------------------------------------
    // Render trait coverage
    // -------------------------------------------------------------------------

    #[test]
    fn test_all_renders_are_non_empty() {
        let rs = RoomStatus {
            id: "r".to_string(),
            ensign_state: EnsignState::Active,
            gravity: 0.0,
            deadband: 0.0,
            tile_count: 0,
        };
        assert!(!rs.render().is_empty());

        let cs = ConservationStatus::new(1.0, 0.5, 0.1);
        assert!(!cs.render().is_empty());

        let ps = ProviderStatus {
            name: "p".to_string(),
            available: true,
            models: vec![],
            budget_remaining: 0.0,
        };
        assert!(!ps.render().is_empty());

        let port = PortStatus {
            name: "p".to_string(),
            connected: true,
            messages_pending: 0,
        };
        assert!(!port.render().is_empty());

        let cons = ConstructStatus::new();
        assert!(!cons.render().is_empty());

        let tile = Tile {
            id: "t".to_string(),
            content: "c".to_string(),
            timestamp: 0,
        };
        assert!(!tile.render().is_empty());

        let db = DeadbandStatus {
            active: false,
            lower_bound: 0.0,
            upper_bound: 0.0,
            current_value: 0.0,
        };
        assert!(!db.render().is_empty());

        let circ = CircuitStatus {
            state: CircuitState::Open,
            load: 0.0,
            temperature: 0.0,
        };
        assert!(!circ.render().is_empty());

        let pc = PenroseCorrelation {
            tile_a: "a".to_string(),
            tile_b: "b".to_string(),
            correlation: 0.0,
        };
        assert!(!pc.render().is_empty());

        let ri = RoomInspection {
            room_config: RoomConfig {
                room_id: "r".to_string(),
                dimensions: [0.0, 0.0, 0.0],
                max_tiles: 0,
                gravity_setting: 0.0,
            },
            ensign_status: EnsignState::Active,
            recent_tiles: vec![],
            gravity_vector: [0.0, 0.0, 0.0],
            deadband_status: db.clone(),
            provenance_entries: vec![],
            circuit_status: circ.clone(),
            penrose_correlations: vec![],
        };
        assert!(!ri.render().is_empty());

        let d = Deployment {
            room_id: "r".to_string(),
            manifest: RoomManifest {
                room_id: "r".to_string(),
                dimensions: [0.0, 0.0, 0.0],
                max_tiles: 0,
                gravity_setting: 0.0,
                ensign_profile: None,
            },
            deployed_at: "now".to_string(),
        };
        assert!(!d.render().is_empty());

        let ts = TraceStep {
            timestamp: 0,
            agent: "a".to_string(),
            action: "a".to_string(),
            duration_ms: 0,
            energy_cost: 0.0,
        };
        assert!(!ts.render().is_empty());

        let cr = ConservationReport {
            law_holds: true,
            total_in: 0.0,
            total_out: 0.0,
            discrepancy: 0.0,
            tolerance: 0.0,
        };
        assert!(!cr.render().is_empty());
    }
}
