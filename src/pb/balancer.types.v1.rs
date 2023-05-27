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
    #[prost(string, tag="3")]
    pub vault: ::prost::alloc::string::String,
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
    #[prost(message, optional, tag="2")]
    pub pool: ::core::option::Option<Pool>,
    #[prost(message, optional, tag="3")]
    pub token: ::core::option::Option<Token>,
    #[prost(string, tag="4")]
    pub balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tokens {
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub decimals: u64,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
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
    #[prost(message, optional, tag="1")]
    pub pool_token: ::core::option::Option<PoolToken>,
    #[prost(string, tag="2")]
    pub delta_balance: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
