# cockatiel_proto

The Cockatiel **protobuf protocol** — the single source of truth for the wire
types shared by the engine, the client SDKs, and every module.

## Contents

- `cockatiel_protobuf.proto` — the Cockatiel protocol (Container, Payload,
  MessagePre/In/PostProcess, Prompt, DatabaseQuery, ModuleControl, …). This is
  the file every component speaks.
- `youtube_stream_list.proto` — the YouTube gRPC live-chat API definition used
  by the youtube-adapter module (not part of the Cockatiel protocol).
- `src/lib.rs` — generated Rust types (`cockatiel_proto::proto`) + the shared
  `PromptKind` / `Prompt::kind()` helper.

## For Rust consumers

Depend on this crate from Cargo, pinned to a specific commit so the types a
module is built against never shift:

```toml
[dependencies]
cockatiel-proto = { git = "https://github.com/vulbyte/cockatiel_proto", rev = "<commit-sha>" }
prost = "0.14"
```

Then:

```rust
use cockatiel_proto::proto::{Container, container::Payload};
use cockatiel_proto::PromptKind;
```

A prebuilt binary or plain `.proto` consumer doesn't need the crate — the
`.proto` file itself is the contract.

## Changing the protocol

1. Edit `cockatiel_protobuf.proto`.
2. Bump consumers deliberately: every crate that pins this repo by `rev`
   decides when it picks up the change (no silent drift).

## License

GPL-2.0