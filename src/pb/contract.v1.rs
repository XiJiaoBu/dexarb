// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub factory_fee_amount_enableds: ::prost::alloc::vec::Vec<FactoryFeeAmountEnabled>,
    #[prost(message, repeated, tag="2")]
    pub factory_owner_changeds: ::prost::alloc::vec::Vec<FactoryOwnerChanged>,
    #[prost(message, repeated, tag="3")]
    pub factory_pool_createds: ::prost::alloc::vec::Vec<FactoryPoolCreated>,
    #[prost(message, repeated, tag="4")]
    pub sushifactory_pair_createds: ::prost::alloc::vec::Vec<SushifactoryPairCreated>,
    #[prost(message, repeated, tag="5")]
    pub oneinchfactory_error_1s: ::prost::alloc::vec::Vec<OneinchfactoryError1>,
    #[prost(message, repeated, tag="6")]
    pub oneinchfactory_error_2s: ::prost::alloc::vec::Vec<OneinchfactoryError2>,
    #[prost(message, repeated, tag="7")]
    pub oneinchfactory_ownership_transferred_1s: ::prost::alloc::vec::Vec<OneinchfactoryOwnershipTransferred1>,
    #[prost(message, repeated, tag="8")]
    pub oneinchfactory_ownership_transferred_2s: ::prost::alloc::vec::Vec<OneinchfactoryOwnershipTransferred2>,
    #[prost(message, repeated, tag="9")]
    pub oneinchfactory_swapped_1s: ::prost::alloc::vec::Vec<OneinchfactorySwapped1>,
    #[prost(message, repeated, tag="10")]
    pub oneinchfactory_swapped_2s: ::prost::alloc::vec::Vec<OneinchfactorySwapped2>,
    #[prost(message, repeated, tag="11")]
    pub pool_burns: ::prost::alloc::vec::Vec<PoolBurn>,
    #[prost(message, repeated, tag="12")]
    pub pool_collects: ::prost::alloc::vec::Vec<PoolCollect>,
    #[prost(message, repeated, tag="13")]
    pub pool_collect_protocols: ::prost::alloc::vec::Vec<PoolCollectProtocol>,
    #[prost(message, repeated, tag="14")]
    pub pool_flashes: ::prost::alloc::vec::Vec<PoolFlash>,
    #[prost(message, repeated, tag="15")]
    pub pool_increase_observation_cardinality_nexts: ::prost::alloc::vec::Vec<PoolIncreaseObservationCardinalityNext>,
    #[prost(message, repeated, tag="16")]
    pub pool_initializes: ::prost::alloc::vec::Vec<PoolInitialize>,
    #[prost(message, repeated, tag="17")]
    pub pool_mints: ::prost::alloc::vec::Vec<PoolMint>,
    #[prost(message, repeated, tag="18")]
    pub pool_set_fee_protocols: ::prost::alloc::vec::Vec<PoolSetFeeProtocol>,
    #[prost(message, repeated, tag="19")]
    pub pool_swaps: ::prost::alloc::vec::Vec<PoolSwap>,
    #[prost(message, repeated, tag="20")]
    pub sushipool_approvals: ::prost::alloc::vec::Vec<SushipoolApproval>,
    #[prost(message, repeated, tag="21")]
    pub sushipool_burns: ::prost::alloc::vec::Vec<SushipoolBurn>,
    #[prost(message, repeated, tag="22")]
    pub sushipool_mints: ::prost::alloc::vec::Vec<SushipoolMint>,
    #[prost(message, repeated, tag="23")]
    pub sushipool_swaps: ::prost::alloc::vec::Vec<SushipoolSwap>,
    #[prost(message, repeated, tag="24")]
    pub sushipool_syncs: ::prost::alloc::vec::Vec<SushipoolSync>,
    #[prost(message, repeated, tag="25")]
    pub sushipool_transfers: ::prost::alloc::vec::Vec<SushipoolTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Calls {
    #[prost(message, repeated, tag="1")]
    pub oneinchfactory_call_destroy_1s: ::prost::alloc::vec::Vec<OneinchfactoryDestroy1call>,
    #[prost(message, repeated, tag="2")]
    pub oneinchfactory_call_destroy_2s: ::prost::alloc::vec::Vec<OneinchfactoryDestroy2call>,
    #[prost(message, repeated, tag="3")]
    pub oneinchfactory_call_discounted_swap_1s: ::prost::alloc::vec::Vec<OneinchfactoryDiscountedSwap1call>,
    #[prost(message, repeated, tag="4")]
    pub oneinchfactory_call_discounted_swap_2s: ::prost::alloc::vec::Vec<OneinchfactoryDiscountedSwap2call>,
    #[prost(message, repeated, tag="5")]
    pub oneinchfactory_call_renounce_ownership_1s: ::prost::alloc::vec::Vec<OneinchfactoryRenounceOwnership1call>,
    #[prost(message, repeated, tag="6")]
    pub oneinchfactory_call_renounce_ownership_2s: ::prost::alloc::vec::Vec<OneinchfactoryRenounceOwnership2call>,
    #[prost(message, repeated, tag="7")]
    pub oneinchfactory_call_rescue_funds_1s: ::prost::alloc::vec::Vec<OneinchfactoryRescueFunds1call>,
    #[prost(message, repeated, tag="8")]
    pub oneinchfactory_call_rescue_funds_2s: ::prost::alloc::vec::Vec<OneinchfactoryRescueFunds2call>,
    #[prost(message, repeated, tag="9")]
    pub oneinchfactory_call_swap_1s: ::prost::alloc::vec::Vec<OneinchfactorySwap1call>,
    #[prost(message, repeated, tag="10")]
    pub oneinchfactory_call_swap_2s: ::prost::alloc::vec::Vec<OneinchfactorySwap2call>,
    #[prost(message, repeated, tag="11")]
    pub oneinchfactory_call_transfer_ownership_1s: ::prost::alloc::vec::Vec<OneinchfactoryTransferOwnership1call>,
    #[prost(message, repeated, tag="12")]
    pub oneinchfactory_call_transfer_ownership_2s: ::prost::alloc::vec::Vec<OneinchfactoryTransferOwnership2call>,
    #[prost(message, repeated, tag="13")]
    pub oneinchfactory_call_unoswap_1s: ::prost::alloc::vec::Vec<OneinchfactoryUnoswap1call>,
    #[prost(message, repeated, tag="14")]
    pub oneinchfactory_call_unoswap_2s: ::prost::alloc::vec::Vec<OneinchfactoryUnoswap2call>,
    #[prost(message, repeated, tag="15")]
    pub oneinchfactory_call_unoswap_with_permit_1s: ::prost::alloc::vec::Vec<OneinchfactoryUnoswapWithPermit1call>,
    #[prost(message, repeated, tag="16")]
    pub oneinchfactory_call_unoswap_with_permit_2s: ::prost::alloc::vec::Vec<OneinchfactoryUnoswapWithPermit2call>,
    #[prost(message, repeated, tag="17")]
    pub pool_call_burns: ::prost::alloc::vec::Vec<PoolBurnCall>,
    #[prost(message, repeated, tag="18")]
    pub pool_call_collects: ::prost::alloc::vec::Vec<PoolCollectCall>,
    #[prost(message, repeated, tag="19")]
    pub pool_call_collect_protocols: ::prost::alloc::vec::Vec<PoolCollectProtocolCall>,
    #[prost(message, repeated, tag="20")]
    pub pool_call_flashes: ::prost::alloc::vec::Vec<PoolFlashCall>,
    #[prost(message, repeated, tag="21")]
    pub pool_call_increase_observation_cardinality_nexts: ::prost::alloc::vec::Vec<PoolIncreaseObservationCardinalityNextCall>,
    #[prost(message, repeated, tag="22")]
    pub pool_call_initializes: ::prost::alloc::vec::Vec<PoolInitializeCall>,
    #[prost(message, repeated, tag="23")]
    pub pool_call_mints: ::prost::alloc::vec::Vec<PoolMintCall>,
    #[prost(message, repeated, tag="24")]
    pub pool_call_set_fee_protocols: ::prost::alloc::vec::Vec<PoolSetFeeProtocolCall>,
    #[prost(message, repeated, tag="25")]
    pub pool_call_swaps: ::prost::alloc::vec::Vec<PoolSwapCall>,
    #[prost(message, repeated, tag="26")]
    pub sushipool_call_approves: ::prost::alloc::vec::Vec<SushipoolApproveCall>,
    #[prost(message, repeated, tag="27")]
    pub sushipool_call_burns: ::prost::alloc::vec::Vec<SushipoolBurnCall>,
    #[prost(message, repeated, tag="28")]
    pub sushipool_call_initializes: ::prost::alloc::vec::Vec<SushipoolInitializeCall>,
    #[prost(message, repeated, tag="29")]
    pub sushipool_call_mints: ::prost::alloc::vec::Vec<SushipoolMintCall>,
    #[prost(message, repeated, tag="30")]
    pub sushipool_call_permits: ::prost::alloc::vec::Vec<SushipoolPermitCall>,
    #[prost(message, repeated, tag="31")]
    pub sushipool_call_skims: ::prost::alloc::vec::Vec<SushipoolSkimCall>,
    #[prost(message, repeated, tag="32")]
    pub sushipool_call_swaps: ::prost::alloc::vec::Vec<SushipoolSwapCall>,
    #[prost(message, repeated, tag="33")]
    pub sushipool_call_syncs: ::prost::alloc::vec::Vec<SushipoolSyncCall>,
    #[prost(message, repeated, tag="34")]
    pub sushipool_call_transfers: ::prost::alloc::vec::Vec<SushipoolTransferCall>,
    #[prost(message, repeated, tag="35")]
    pub sushipool_call_transfer_froms: ::prost::alloc::vec::Vec<SushipoolTransferFromCall>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventsCalls {
    #[prost(message, optional, tag="1")]
    pub events: ::core::option::Option<Events>,
    #[prost(message, optional, tag="2")]
    pub calls: ::core::option::Option<Calls>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryFeeAmountEnabled {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub fee: u64,
    #[prost(int64, tag="6")]
    pub tick_spacing: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryOwnerChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub old_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FactoryPoolCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub fee: u64,
    #[prost(int64, tag="8")]
    pub tick_spacing: i64,
    #[prost(bytes="vec", tag="9")]
    pub pool: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushifactoryPairCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub token1: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub pair: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub param3: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryError1 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reason: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryError2 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub reason: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryOwnershipTransferred1 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryOwnershipTransferred2 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactorySwapped1 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub src_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub dst_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub dst_receiver: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub spent_amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub return_amount: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub price_src_token_per_dst_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactorySwapped2 {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub src_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub dst_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub dst_receiver: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub spent_amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub return_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryDestroy1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryDestroy2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryDiscountedSwap1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub output_return_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub output_gas_left: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub output_chi_spent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryDiscountedSwap2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub output_return_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub output_gas_left: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub output_chi_spent: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryRenounceOwnership1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryRenounceOwnership2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryRescueFunds1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryRescueFunds2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactorySwap1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub output_return_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub output_gas_left: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactorySwap2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub caller: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub output_return_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub output_gas_left: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryTransferOwnership1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryTransferOwnership2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryUnoswap1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub src_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub min_return: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="9")]
    pub param3: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag="10")]
    pub output_return_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryUnoswap2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub src_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub min_return: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="9")]
    pub param3: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag="10")]
    pub output_return_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryUnoswapWithPermit1call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub src_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub min_return: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="9")]
    pub pools: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="10")]
    pub permit: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="11")]
    pub output_return_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneinchfactoryUnoswapWithPermit2call {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(bytes="vec", tag="6")]
    pub src_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub min_return: ::prost::alloc::string::String,
    #[prost(bytes="vec", repeated, tag="9")]
    pub pools: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="10")]
    pub permit: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="11")]
    pub output_return_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolBurn {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="7")]
    pub tick_lower: i64,
    #[prost(int64, tag="8")]
    pub tick_upper: i64,
    #[prost(string, tag="9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollect {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub tick_lower: i64,
    #[prost(int64, tag="9")]
    pub tick_upper: i64,
    #[prost(string, tag="10")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollectProtocol {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolFlash {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub paid0: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub paid1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolIncreaseObservationCardinalityNext {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub observation_cardinality_next_old: u64,
    #[prost(uint64, tag="7")]
    pub observation_cardinality_next_new: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitialize {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub tick: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolMint {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub tick_lower: i64,
    #[prost(int64, tag="9")]
    pub tick_upper: i64,
    #[prost(string, tag="10")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSetFeeProtocol {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub fee_protocol0_old: u64,
    #[prost(uint64, tag="7")]
    pub fee_protocol1_old: u64,
    #[prost(uint64, tag="8")]
    pub fee_protocol0_new: u64,
    #[prost(uint64, tag="9")]
    pub fee_protocol1_new: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSwap {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub liquidity: ::prost::alloc::string::String,
    #[prost(int64, tag="12")]
    pub tick: i64,
    #[prost(int64, tag="13")]
    pub executed_price: i64,
    #[prost(int64, tag="14")]
    pub expected_price: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolBurnCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(int64, tag="7")]
    pub tick_lower: i64,
    #[prost(int64, tag="8")]
    pub tick_upper: i64,
    #[prost(string, tag="9")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollectCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub tick_lower: i64,
    #[prost(int64, tag="9")]
    pub tick_upper: i64,
    #[prost(string, tag="10")]
    pub amount0_requested: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub amount1_requested: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolCollectProtocolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount0_requested: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount1_requested: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolFlashCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolIncreaseObservationCardinalityNextCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub observation_cardinality_next: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolInitializeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub sqrt_price_x96: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolMintCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(int64, tag="8")]
    pub tick_lower: i64,
    #[prost(int64, tag="9")]
    pub tick_upper: i64,
    #[prost(string, tag="10")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="12")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSetFeeProtocolCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub fee_protocol0: u64,
    #[prost(uint64, tag="8")]
    pub fee_protocol1: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolSwapCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="8")]
    pub zero_for_one: bool,
    #[prost(string, tag="9")]
    pub amount_specified: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub sqrt_price_limit_x96: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="12")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub output_amount1: ::prost::alloc::string::String,
    #[prost(uint64, tag="14")]
    pub gas_price: u64,
    #[prost(uint64, tag="15")]
    pub gas_used: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolApproval {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolBurn {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolMint {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolSwap {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub amount0_in: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1_in: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub amount0_out: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub amount1_out: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string,tag="12")]
    pub price_token0_per_token1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolSync {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub reserve0: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub reserve1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub evt_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolApproveCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolBurnCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub output_amount0: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub output_amount1: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolInitializeCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub u_token0: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub u_token1: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolMintCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub output_liquidity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolPermitCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub spender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub deadline: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub v: u64,
    #[prost(bytes="vec", tag="12")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="13")]
    pub s: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolSkimCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolSwapCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub amount0_out: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub amount1_out: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="10")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolSyncCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolTransferCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="9")]
    pub output_param0: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SushipoolTransferFromCall {
    #[prost(string, tag="1")]
    pub call_tx_hash: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub call_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="3")]
    pub call_block_number: u64,
    #[prost(uint64, tag="4")]
    pub call_ordinal: u64,
    #[prost(bool, tag="5")]
    pub call_success: bool,
    #[prost(string, tag="6")]
    pub call_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub value: ::prost::alloc::string::String,
    #[prost(bool, tag="10")]
    pub output_param0: bool,
}
// @@protoc_insertion_point(module)
