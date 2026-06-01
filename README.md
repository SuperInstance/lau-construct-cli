# lau-construct-cli

> CLI toolkit for managing a PLATO construct — status, inspect, deploy, debug.

## What This Does

CLI toolkit for managing a PLATO construct — status, inspect, deploy, debug.. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-construct-cli
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_construct_cli::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub enum DeployError 
pub trait Render 
pub enum EnsignState 
pub struct RoomStatus 
pub struct ConservationStatus 
pub struct ProviderStatus 
pub struct PortStatus 
pub struct ConstructStatus 
    pub fn new() -> Self 
pub struct RoomConfig 
pub struct Tile 
pub struct DeadbandStatus 
pub struct ProvenanceEntry 
pub enum CircuitState 
pub struct CircuitStatus 
pub struct PenroseCorrelation 
pub struct RoomInspection 
pub struct ConstructInspector 
    pub fn new() -> Self 
    pub fn register_room(&mut self, inspection: RoomInspection) 
    pub fn inspect_room(&self, room_id: &str) -> RoomInspection 
pub struct EnsignProfile 
pub struct RoomManifest 
pub struct Deployment 
pub struct ConstructDeployer 
    pub fn new() -> Self 
    pub fn deploy_room(&mut self, config: RoomManifest) -> Result<String, DeployError> 
    pub fn deploy_ensign(
    pub fn remove_room(&mut self, room_id: &str) -> Result<(), DeployError> 
    pub fn list_deployments(&self) -> Vec<Deployment> 
pub struct TraceStep 
pub struct ConservationReport 
pub struct ConstructDebug;
    pub fn new() -> Self 
    pub fn trace_message(&self, msg_id: &str) -> Vec<TraceStep> 
    pub fn verify_conservation(&self) -> ConservationReport 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**71 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
