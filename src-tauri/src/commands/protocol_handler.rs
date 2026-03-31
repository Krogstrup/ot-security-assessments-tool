//! Protocol handler trait and `ProcessorCore`.
//!
//! # Design
//!
//! `PacketProcessor` holds a `ProcessorCore` (shared state that every packet
//! touches regardless of protocol) and a `Vec<Box<dyn ProtocolHandler>>`
//! (per-protocol accumulators).
//!
//! After all packets are processed:
//! - Each handler's `finalize()` populates the relevant field of
//!   `HashMap<String, DeepParseInfo>`.
//! - `ProcessorCore` drives connection tracking, topology, and asset building.
//!
//! # Adding a new protocol
//!
//! 1. Create `handlers/my_proto.rs` with a struct that implements `ProtocolHandler`.
//! 2. Register it in `PacketProcessor::new()`:
//!    `handlers.push(Box::new(MyProtoHandler::default()))`.
//! 3. Add a `DeepParseResult::MyProto` arm in `process_packet()` if it isn't
//!    already routed by `deep_parse()`.
//!
//! No changes to `PacketProcessor` or `ProcessorCore` are required.

use std::collections::HashMap;

use gm_capture::ParsedPacket;
use gm_parsers::DeepParseResult;

use super::DeepParseInfo;

/// Extension point for per-protocol packet accumulation.
///
/// Each implementor holds only the HashMap/HashSet fields relevant to its
/// protocol. The trait is object-safe; implementors are boxed and stored in
/// `PacketProcessor::handlers`.
pub trait ProtocolHandler: Send {
    /// Inspect a packet and update accumulators if it is relevant to this
    /// protocol. Called for every packet for which `deep_parse()` returned
    /// `Some(result)`.
    ///
    /// The full `ParsedPacket` is provided for IP/port context. `deep_result`
    /// is the parsed protocol-specific data from `gm-parsers`.
    fn process(&mut self, packet: &ParsedPacket, deep_result: &DeepParseResult);

    /// Called once after all packets have been processed.
    ///
    /// For each IP address that spoke this protocol, inserts or updates the
    /// corresponding field in `deep_parse` (e.g. `entry.modbus = Some(...)`,
    /// `entry.dnp3 = Some(...)`).
    fn finalize(&self, deep_parse: &mut HashMap<String, DeepParseInfo>);
}
