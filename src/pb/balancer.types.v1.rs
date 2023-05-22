// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pools {
    #[prost(message, repeated, tag="1")]
    pub pools: ::prost::alloc::vec::Vec<Pool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolTokens {
    #[prost(message, repeated, tag="1")]
    pub pool_tokens: ::prost::alloc::vec::Vec<PoolToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolToken {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub balance: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolTokenBalanceChanges {
    #[prost(message, repeated, tag="1")]
    pub pool_token_balance_changes: ::prost::alloc::vec::Vec<PoolTokenBalanceChange>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolTokenBalanceChange {
    #[prost(string, tag="1")]
    pub pool_token_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub delta_balance: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub log_ordinal: u64,
}
// @@protoc_insertion_point(module)
