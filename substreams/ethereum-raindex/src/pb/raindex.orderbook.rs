// @generated
/// Describes a vault owned by an individual user holding specific ERC20 tokens.
/// Vaults fund orders and this Protobuf representation is needed to associate
/// liquidity with orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    /// The owner of the vault.
    #[prost(bytes="vec", tag="1")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    /// The token held in the vault.
    #[prost(bytes="vec", tag="2")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    /// The vault ID of the vault.
    #[prost(bytes="vec", tag="3")]
    pub vault_id: ::prost::alloc::vec::Vec<u8>,
    /// The vault balance.
    #[prost(bytes="vec", tag="4")]
    pub balance: ::prost::alloc::vec::Vec<u8>,
}
/// Defines a fully deployed order ready to evaluate by Orderbook. Identical to
/// `Order` except for the newer `EvaluableV2`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderV3 {
    /// The owner of the order is the `msg.sender` that added the order.
    #[prost(bytes="vec", tag="1")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    /// Standard `EvaluableV2` with entrypoints for both "calculate order" and
    /// "handle IO". The latter MAY be empty bytes, in which case it will be 
    /// skipped at runtime to save gas.
    #[prost(message, optional, tag="2")]
    pub evaluable: ::core::option::Option<EvaluableV3>,
    /// A list of input tokens that are economically equivalent for the purpose
    /// of processing this order. Inputs are relative to the order so these
    /// tokens will be sent to the owners vault.
    #[prost(message, repeated, tag="3")]
    pub valid_inputs: ::prost::alloc::vec::Vec<Io>,
    /// A list of output tokens that are economically equivalent for the purpose
    /// of processing this order. Outputs are relative to the order so these
    /// tokens will be sent from the owners vault.
    #[prost(message, repeated, tag="4")]
    pub valid_outputs: ::prost::alloc::vec::Vec<Io>,
}
/// Struct over the return of `IParserV2.parse2` which MAY be more convenient to
/// work with than raw addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvaluableV3 {
    /// Address of the interpreter that will evaluate the expression.
    #[prost(bytes="vec", tag="1")]
    pub interpreter: ::prost::alloc::vec::Vec<u8>,
    /// Address of the store that will store state changes due to evaluation of the expression.
    #[prost(bytes="vec", tag="2")]
    pub store: ::prost::alloc::vec::Vec<u8>,
}
/// Configuration for a single input or output on an `Order`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Io {
    /// The token to either send from the owner as an output or receive from the
    /// counterparty to the owner as an input. The tokens are not moved during an
    /// order, only internal vault balances are updated, until a separate withdraw
    /// step.
    #[prost(bytes="vec", tag="1")]
    pub token: ::prost::alloc::vec::Vec<u8>,
    // // The decimals to use for internal scaling calculations for `token`. This is
    // // provided directly in IO to save gas on external lookups and to respect the
    // // ERC20 spec that mandates NOT assuming or using the `decimals` method for
    // // onchain calculations. Ostensibly the decimals exists so that all
    // // calculate order entrypoints can treat amounts and ratios as 18 decimal
    // // fixed point values. Order max amounts MUST be rounded down and IO ratios
    // // rounded up to compensate for any loss of precision during decimal
    // // rescaling.
    // uint32 decimals = 2;

    /// The vault ID that tokens will move into if this is an input or move out
    /// from if this is an output.
    /// NOTE: Make it a big endian encoded int to be consistent with Tycho.
    #[prost(bytes="vec", tag="3")]
    pub vault_id: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub orderbook_events: ::prost::alloc::vec::Vec<events::OrderbookEvent>,
}
/// Nested message and enum types in `Events`.
pub mod events {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrderbookEvent {
        #[prost(uint64, tag="100")]
        pub log_ordinal: u64,
        #[prost(oneof="orderbook_event::Event", tags="1, 2, 3")]
        pub event: ::core::option::Option<orderbook_event::Event>,
    }
    /// Nested message and enum types in `OrderbookEvent`.
    pub mod orderbook_event {
        /// Some tokens have been deposited to a vault.
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Deposit {
            /// `msg.sender` depositing tokens. Delegated deposits are NOT supported.
            #[prost(bytes="vec", tag="1")]
            pub sender: ::prost::alloc::vec::Vec<u8>,
            /// The token being deposited.
            #[prost(bytes="vec", tag="2")]
            pub token: ::prost::alloc::vec::Vec<u8>,
            /// The vault ID the tokens are being deposited under.
            /// NOTE: Make it a big endian encoded int to be consistent with Tycho.
            #[prost(bytes="vec", tag="3")]
            pub vault_id: ::prost::alloc::vec::Vec<u8>,
            /// The amount of tokens deposited.
            /// NOTE: Make it a big endian encoded int to be consistent with Tycho.
            #[prost(bytes="vec", tag="4")]
            pub amount: ::prost::alloc::vec::Vec<u8>,
        }
        /// Some tokens have been withdrawn from a vault.
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Withdraw {
            /// `msg.sender` withdrawing tokens. Delegated withdrawals are NOT supported.
            #[prost(bytes="vec", tag="1")]
            pub sender: ::prost::alloc::vec::Vec<u8>,
            /// The token being withdrawn.
            #[prost(bytes="vec", tag="2")]
            pub token: ::prost::alloc::vec::Vec<u8>,
            /// The vault ID the tokens are being withdrawn from.
            #[prost(bytes="vec", tag="3")]
            pub vault_id: ::prost::alloc::vec::Vec<u8>,
            // // The amount of tokens requested to withdraw.
            // bytes target_amount = 4;

            /// The amount of tokens withdrawn, can be less than the
            /// target amount if the vault does not have the funds available to cover
            /// the target amount. For example an active order might move tokens before
            /// the withdraw completes.
            #[prost(bytes="vec", tag="5")]
            pub amount: ::prost::alloc::vec::Vec<u8>,
        }
        /// An order has been added to the orderbook. The order is permanently and
        /// always active according to its expression until/unless it is removed.
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AddOrderV2 {
            /// `msg.sender` adding the order and is owner of the order.
            #[prost(bytes="vec", tag="1")]
            pub sender: ::prost::alloc::vec::Vec<u8>,
            /// The hash of the order as it is recorded onchain. Only
            /// the hash is stored in Orderbook storage to avoid paying gas to store the
            /// entire order.
            #[prost(bytes="vec", tag="2")]
            pub order_hash: ::prost::alloc::vec::Vec<u8>,
            /// The newly added order. MUST be handed back as-is when
            /// clearing orders and contains derived information in addition to the order
            /// config that was provided by the order owner.
            #[prost(message, optional, tag="3")]
            pub order: ::core::option::Option<super::super::OrderV3>,
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Event {
            #[prost(message, tag="1")]
            Deposit(Deposit),
            #[prost(message, tag="2")]
            Withdraw(Withdraw),
            /// RemoveOrderV2
            /// TakeOrderV2
            /// OrderNotFound
            /// OrderZeroAmount
            /// OrderExceedsMaxRatio
            /// ClearV2
            /// AfterClear
            #[prost(message, tag="3")]
            AddOrder(AddOrderV2),
        }
    }
}
// @@protoc_insertion_point(module)
