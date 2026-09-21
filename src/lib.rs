//! Cockatiel protocol types — the single source of truth for the wire format.
//!
//! This crate compiles [`cockatiel_protobuf.proto`] and exposes the generated
//! protobuf types. Every component that speaks the Cockatiel protocol — the
//! engine, the client SDKs, and every module — depends on this crate so the
//! generated Rust types always match.
//!
//! [`cockatiel_protobuf.proto`]: https://github.com/vulbyte/cockatiel_proto/blob/main/cockatiel_protobuf.proto

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/cockatiel_protobuf.v1.rs"));
}

use proto::Prompt;

/// The kind of answer a `Prompt` expects. The UI picks its input widget from
/// this; [`Prompt::kind`] derives it from `prompt_type` with a legacy fallback
/// (empty `input_label` -> boolean, non-empty -> free-text string).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    Boolean,
    String,
    Credential,
}

impl Prompt {
    /// What kind of answer this prompt wants.
    pub fn kind(&self) -> PromptKind {
        // `prompt_type` is an i32 (prost). 1=boolean, 2=string, 3=credential.
        match self.prompt_type {
            1 => PromptKind::Boolean,
            2 => PromptKind::String,
            3 => PromptKind::Credential,
            // Legacy / unspecified: infer from the input label.
            _ => {
                if self.input_label.is_empty() {
                    PromptKind::Boolean
                } else {
                    PromptKind::String
                }
            }
        }
    }
}