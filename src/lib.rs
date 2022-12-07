#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
#[allow(deprecated)]
mod protos {
    include!("./protos/mod.rs");

    use raft_proto::eraftpb;
}

pub use protos::*;
