mod affinity;
mod contracts;
pub mod events;
mod global_state;
mod safe;
mod to_alloy;
mod tokens;
pub mod transactions;
pub mod ui_events;
pub use affinity::Affinity;
pub use alloy_primitives::{address, Address, B256, U256, U64};
pub use contracts::{Contract, ContractWithAbi};
pub use ethers::{abi::Abi, types::Bytes};
pub use events::Event;
pub use global_state::GlobalState;
pub use safe::{SafeContractData, SafeMultisigTxsData};
pub use to_alloy::{ToAlloy, ToEthers};
pub use tokens::{
    Erc1155Token, Erc1155TokenData, Erc721Collection, Erc721Token, Erc721TokenData,
    Erc721TokenDetails, TokenBalance, TokenMetadata,
};
pub use ui_events::UINotify;

pub type Json = serde_json::Value;

#[derive(Debug, Default)]
pub struct SyncUpdates {
    pub events: Option<Vec<Event>>,
    pub erc20_balances: Option<Vec<(Address, U256)>>,
    pub native_balance: Option<U256>,
    pub tip: Option<u64>,
}
