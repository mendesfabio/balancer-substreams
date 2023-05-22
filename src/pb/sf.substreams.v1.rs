// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub number: u64,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Modules {
    #[prost(message, repeated, tag="1")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    #[prost(message, repeated, tag="2")]
    pub binaries: ::prost::alloc::vec::Vec<Binary>,
}
/// Binary represents some code compiled to its binary form.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Binary {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub content: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub binary_index: u32,
    #[prost(string, tag="5")]
    pub binary_entrypoint: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="6")]
    pub inputs: ::prost::alloc::vec::Vec<module::Input>,
    #[prost(message, optional, tag="7")]
    pub output: ::core::option::Option<module::Output>,
    #[prost(uint64, tag="8")]
    pub initial_block: u64,
    #[prost(oneof="module::Kind", tags="2, 3")]
    pub kind: ::core::option::Option<module::Kind>,
}
/// Nested message and enum types in `Module`.
pub mod module {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KindMap {
        #[prost(string, tag="1")]
        pub output_type: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KindStore {
        /// The `update_policy` determines the functions available to mutate the store
        /// (like `set()`, `set_if_not_exists()` or `sum()`, etc..) in
        /// order to ensure that parallel operations are possible and deterministic
        ///
        /// Say a store cumulates keys from block 0 to 1M, and a second store
        /// cumulates keys from block 1M to 2M. When we want to use this
        /// store as a dependency for a downstream module, we will merge the
        /// two stores according to this policy.
        #[prost(enumeration="kind_store::UpdatePolicy", tag="1")]
        pub update_policy: i32,
        #[prost(string, tag="2")]
        pub value_type: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `KindStore`.
    pub mod kind_store {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum UpdatePolicy {
            Unset = 0,
            /// Provides a store where you can `set()` keys, and the latest key wins
            Set = 1,
            /// Provides a store where you can `set_if_not_exists()` keys, and the first key wins
            SetIfNotExists = 2,
            /// Provides a store where you can `add_*()` keys, where two stores merge by summing its values.
            Add = 3,
            /// Provides a store where you can `min_*()` keys, where two stores merge by leaving the minimum value.
            Min = 4,
            /// Provides a store where you can `max_*()` keys, where two stores merge by leaving the maximum value.
            Max = 5,
            /// Provides a store where you can `append()` keys, where two stores merge by concatenating the bytes in order.
            Append = 6,
        }
        impl UpdatePolicy {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    UpdatePolicy::Unset => "UPDATE_POLICY_UNSET",
                    UpdatePolicy::Set => "UPDATE_POLICY_SET",
                    UpdatePolicy::SetIfNotExists => "UPDATE_POLICY_SET_IF_NOT_EXISTS",
                    UpdatePolicy::Add => "UPDATE_POLICY_ADD",
                    UpdatePolicy::Min => "UPDATE_POLICY_MIN",
                    UpdatePolicy::Max => "UPDATE_POLICY_MAX",
                    UpdatePolicy::Append => "UPDATE_POLICY_APPEND",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UPDATE_POLICY_UNSET" => Some(Self::Unset),
                    "UPDATE_POLICY_SET" => Some(Self::Set),
                    "UPDATE_POLICY_SET_IF_NOT_EXISTS" => Some(Self::SetIfNotExists),
                    "UPDATE_POLICY_ADD" => Some(Self::Add),
                    "UPDATE_POLICY_MIN" => Some(Self::Min),
                    "UPDATE_POLICY_MAX" => Some(Self::Max),
                    "UPDATE_POLICY_APPEND" => Some(Self::Append),
                    _ => None,
                }
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(oneof="input::Input", tags="1, 2, 3")]
        pub input: ::core::option::Option<input::Input>,
    }
    /// Nested message and enum types in `Input`.
    pub mod input {
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Source {
            /// ex: "sf.ethereum.type.v1.Block"
            #[prost(string, tag="1")]
            pub r#type: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Map {
            /// ex: "block_to_pairs"
            #[prost(string, tag="1")]
            pub module_name: ::prost::alloc::string::String,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Store {
            #[prost(string, tag="1")]
            pub module_name: ::prost::alloc::string::String,
            #[prost(enumeration="store::Mode", tag="2")]
            pub mode: i32,
        }
        /// Nested message and enum types in `Store`.
        pub mod store {
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
            #[repr(i32)]
            pub enum Mode {
                Unset = 0,
                Get = 1,
                Deltas = 2,
            }
            impl Mode {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Mode::Unset => "UNSET",
                        Mode::Get => "GET",
                        Mode::Deltas => "DELTAS",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "UNSET" => Some(Self::Unset),
                        "GET" => Some(Self::Get),
                        "DELTAS" => Some(Self::Deltas),
                        _ => None,
                    }
                }
            }
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Input {
            #[prost(message, tag="1")]
            Source(Source),
            #[prost(message, tag="2")]
            Map(Map),
            #[prost(message, tag="3")]
            Store(Store),
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Output {
        #[prost(string, tag="1")]
        pub r#type: ::prost::alloc::string::String,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        #[prost(message, tag="2")]
        KindMap(KindMap),
        #[prost(message, tag="3")]
        KindStore(KindStore),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// Needs to be one so this file can be used _directly_ as a
    /// buf `Image` andor a ProtoSet for grpcurl and other tools
    #[prost(message, repeated, tag="1")]
    pub proto_files: ::prost::alloc::vec::Vec<::prost_types::FileDescriptorProto>,
    #[prost(uint64, tag="5")]
    pub version: u64,
    #[prost(message, optional, tag="6")]
    pub modules: ::core::option::Option<Modules>,
    #[prost(message, repeated, tag="7")]
    pub module_meta: ::prost::alloc::vec::Vec<ModuleMetadata>,
    #[prost(message, repeated, tag="8")]
    pub package_meta: ::prost::alloc::vec::Vec<PackageMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageMetadata {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub doc: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleMetadata {
    /// Corresponds to the index in `Package.metadata.package_meta`
    #[prost(uint64, tag="1")]
    pub package_index: u64,
    #[prost(string, tag="2")]
    pub doc: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(int64, tag="1")]
    pub start_block_num: i64,
    #[prost(string, tag="2")]
    pub start_cursor: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub stop_block_num: u64,
    #[prost(enumeration="ForkStep", repeated, tag="4")]
    pub fork_steps: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, tag="5")]
    pub irreversibility_condition: ::prost::alloc::string::String,
    /// By default, the engine runs in developer mode, with richer and deeper output,
    /// * support for multiple `output_modules`, of `store` and `map` kinds
    /// * support for `initial_store_snapshot_for_modules`
    /// * log outputs for output modules
    ///
    /// With `production_mode`, however, you trade off functionality for high speed, where it:
    /// * restricts the possible requested `output_modules` to a single mapper module,
    /// * turns off support for `initial_store_snapshot_for_modules`,
    /// * still streams output linearly, with a cursor, but at higher speeds
    /// * and purges log outputs from responses.
    #[prost(bool, tag="9")]
    pub production_mode: bool,
    #[prost(message, optional, tag="6")]
    pub modules: ::core::option::Option<Modules>,
    #[prost(string, repeated, tag="7")]
    pub output_modules: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Available only in developer mode
    #[prost(string, repeated, tag="8")]
    pub debug_initial_store_snapshot_for_modules: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="10")]
    pub output_module: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(oneof="response::Message", tags="5, 1, 2, 3, 4")]
    pub message: ::core::option::Option<response::Message>,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// Always sent first
        #[prost(message, tag="5")]
        Session(super::SessionInit),
        /// Progress of data preparation, before sending in the stream of `data` events.
        #[prost(message, tag="1")]
        Progress(super::ModulesProgress),
        /// Available only in developer mode, and only if `debug_initial_store_snapshot_for_modules` is set.
        #[prost(message, tag="2")]
        DebugSnapshotData(super::InitialSnapshotData),
        /// Available only in developer mode, and only if `debug_initial_store_snapshot_for_modules` is set.
        #[prost(message, tag="3")]
        DebugSnapshotComplete(super::InitialSnapshotComplete),
        #[prost(message, tag="4")]
        Data(super::BlockScopedData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionInit {
    #[prost(string, tag="1")]
    pub trace_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialSnapshotComplete {
    #[prost(string, tag="1")]
    pub cursor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialSnapshotData {
    #[prost(string, tag="1")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub deltas: ::core::option::Option<StoreDeltas>,
    #[prost(uint64, tag="4")]
    pub sent_keys: u64,
    #[prost(uint64, tag="3")]
    pub total_keys: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockScopedData {
    #[prost(message, repeated, tag="1")]
    pub outputs: ::prost::alloc::vec::Vec<ModuleOutput>,
    #[prost(message, optional, tag="3")]
    pub clock: ::core::option::Option<Clock>,
    #[prost(enumeration="ForkStep", tag="6")]
    pub step: i32,
    #[prost(string, tag="10")]
    pub cursor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleOutput {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub debug_logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// LogsTruncated is a flag that tells you if you received all the logs or if they
    /// were truncated because you logged too much (fixed limit currently is set to 128 KiB).
    #[prost(bool, tag="5")]
    pub debug_logs_truncated: bool,
    #[prost(bool, tag="6")]
    pub cached: bool,
    #[prost(oneof="module_output::Data", tags="2, 3")]
    pub data: ::core::option::Option<module_output::Data>,
}
/// Nested message and enum types in `ModuleOutput`.
pub mod module_output {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="2")]
        MapOutput(::prost_types::Any),
        /// StoreDeltas are produced for store modules in development mode.
        /// It is not possible to retrieve store models in production, with parallelization
        /// enabled. If you need the deltas directly, write a pass through mapper module
        /// that will get them down to you.
        #[prost(message, tag="3")]
        DebugStoreDeltas(super::StoreDeltas),
    }
}
// think about:
// message ModuleOutput { ...
//    ModuleOutputDebug debug_info = 6;
// ...}
// message ModuleOutputDebug {
//   StoreDeltas store_deltas = 3;
//   repeated string logs = 4;
//   // LogsTruncated is a flag that tells you if you received all the logs or if they
//   // were truncated because you logged too much (fixed limit currently is set to 128 KiB).
//   bool logs_truncated = 5;
// }

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModulesProgress {
    #[prost(message, repeated, tag="1")]
    pub modules: ::prost::alloc::vec::Vec<ModuleProgress>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleProgress {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="module_progress::Type", tags="2, 3, 4, 5")]
    pub r#type: ::core::option::Option<module_progress::Type>,
}
/// Nested message and enum types in `ModuleProgress`.
pub mod module_progress {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessedRange {
        #[prost(message, repeated, tag="1")]
        pub processed_ranges: ::prost::alloc::vec::Vec<super::BlockRange>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InitialState {
        #[prost(uint64, tag="2")]
        pub available_up_to_block: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessedBytes {
        #[prost(uint64, tag="1")]
        pub total_bytes_read: u64,
        #[prost(uint64, tag="2")]
        pub total_bytes_written: u64,
        #[prost(uint64, tag="3")]
        pub bytes_read_delta: u64,
        #[prost(uint64, tag="4")]
        pub bytes_written_delta: u64,
        #[prost(uint64, tag="5")]
        pub nano_seconds_delta: u64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failed {
        #[prost(string, tag="1")]
        pub reason: ::prost::alloc::string::String,
        #[prost(string, repeated, tag="2")]
        pub logs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// FailureLogsTruncated is a flag that tells you if you received all the logs or if they
        /// were truncated because you logged too much (fixed limit currently is set to 128 KiB).
        #[prost(bool, tag="3")]
        pub logs_truncated: bool,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="2")]
        ProcessedRanges(ProcessedRange),
        #[prost(message, tag="3")]
        InitialState(InitialState),
        #[prost(message, tag="4")]
        ProcessedBytes(ProcessedBytes),
        #[prost(message, tag="5")]
        Failed(Failed),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRange {
    #[prost(uint64, tag="2")]
    pub start_block: u64,
    #[prost(uint64, tag="3")]
    pub end_block: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreDeltas {
    #[prost(message, repeated, tag="1")]
    pub deltas: ::prost::alloc::vec::Vec<StoreDelta>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreDelta {
    #[prost(enumeration="store_delta::Operation", tag="1")]
    pub operation: i32,
    #[prost(uint64, tag="2")]
    pub ordinal: u64,
    #[prost(string, tag="3")]
    pub key: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub old_value: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub new_value: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `StoreDelta`.
pub mod store_delta {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operation {
        Unset = 0,
        Create = 1,
        Update = 2,
        Delete = 3,
    }
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unset => "UNSET",
                Operation::Create => "CREATE",
                Operation::Update => "UPDATE",
                Operation::Delete => "DELETE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "CREATE" => Some(Self::Create),
                "UPDATE" => Some(Self::Update),
                "DELETE" => Some(Self::Delete),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(string, tag="2")]
    pub block_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub value: ::core::option::Option<::prost_types::Any>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ForkStep {
    StepUnknown = 0,
    /// Block is new head block of the chain, that is linear with the previous block
    StepNew = 1,
    /// Block is now forked and should be undone, it's not the head block of the chain anymore
    StepUndo = 2,
    /// Block is now irreversible and can be committed to (finality is chain specific, see chain documentation for more details)
    StepIrreversible = 4,
}
impl ForkStep {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ForkStep::StepUnknown => "STEP_UNKNOWN",
            ForkStep::StepNew => "STEP_NEW",
            ForkStep::StepUndo => "STEP_UNDO",
            ForkStep::StepIrreversible => "STEP_IRREVERSIBLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STEP_UNKNOWN" => Some(Self::StepUnknown),
            "STEP_NEW" => Some(Self::StepNew),
            "STEP_UNDO" => Some(Self::StepUndo),
            "STEP_IRREVERSIBLE" => Some(Self::StepIrreversible),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
