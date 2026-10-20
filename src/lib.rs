//! Cockatiel protocol types — the single source of truth for the wire format.
//!
//! This crate compiles [`cockatiel_protobuf.proto`] and exposes the generated
//! protobuf types. Every component that speaks the Cockatiel protocol — the
//! engine, the client SDKs, and every module — depends on this crate so the
//! generated Rust types always match.
//!
//! [`cockatiel_protobuf.proto`]: https://github.com/vulbyte/cockatiel_proto/blob/main/cockatiel_protobuf.proto

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/cockatiel_protobuf.rs"));
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

#[cfg(test)]
mod proxy_roundtrip_tests {
    use super::proto::{ProxyIdentity, SendToPlatforms};
    use prost::Message;

    /// The Discord RP proxy fields round-trip: a `SendToPlatforms` carrying a
    /// `ProxyIdentity` (character key + name + avatar) and a trigger id to
    /// delete survives encode/decode with every field intact.
    #[test]
    fn proxy_identity_round_trips() {
        let send = SendToPlatforms {
            msg: "hello".into(),
            platform: "discord".into(),
            channel_id: "123".into(),
            actor_handle: "alice".into(),
            proxy: Some(ProxyIdentity {
                character_key: "char-1".into(),
                display_name: "Alice".into(),
                avatar: vec![1, 2, 3, 4],
            }),
            delete_message_id: "trig-9".into(),
            ..Default::default()
        };
        let mut buf = Vec::new();
        send.encode(&mut buf).unwrap();
        let back = SendToPlatforms::decode(buf.as_slice()).unwrap();
        assert_eq!(back.msg, "hello");
        let p = back.proxy.expect("proxy present");
        assert_eq!(p.character_key, "char-1");
        assert_eq!(p.display_name, "Alice");
        assert_eq!(p.avatar, vec![1, 2, 3, 4]);
        assert_eq!(back.delete_message_id, "trig-9");
        // A plain send carries no proxy and nothing to delete.
        let plain = SendToPlatforms::default();
        assert!(plain.proxy.is_none());
        assert!(plain.delete_message_id.is_empty());
    }
}