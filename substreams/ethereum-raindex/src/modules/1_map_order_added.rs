use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams::scalar::BigInt;
use substreams_ethereum::pb::eth::v2::{self as eth};

use substreams_helper::{event_handler::EventHandler, hex::Hexable};

use crate::abi::raindex_orderbook::events::AddOrderV2;

use tycho_substreams::prelude::*;

#[substreams::handlers::map]
pub fn map_order_added(
    params: String,
    block: eth::Block,
) -> Result<BlockEntityChanges, substreams::errors::Error> {
    println!("Running map_order_added");

    let mut new_orders: Vec<TransactionEntityChanges> = vec![];
    let orderbook_address = params.as_str();

    get_new_orders(&block, &mut new_orders, orderbook_address);

    Ok(BlockEntityChanges { block: None, changes: new_orders })
}

// Extract new orders from AddOrderV2 events
fn get_new_orders(
    block: &eth::Block,
    new_orders: &mut Vec<TransactionEntityChanges>,
    factory_address: &str,
) {
    // Extract new orders from AddOrderV2 events
    let mut on_order_added = |event: AddOrderV2, _tx: &eth::TransactionTrace, _log: &eth::Log| {
        let tycho_tx: Transaction = _tx.into();

        let (owner, _evaluable, valid_inputs, valid_outputs, _nonce) = event.order.clone();

        let input_tokens = valid_inputs
            .clone()
            .into_iter()
            .map(|(token, _decimals, _vault_id)| token)
            .collect::<Vec<_>>();

        let output_tokens = valid_outputs
            .clone()
            .into_iter()
            .map(|(token, _decimals, _vault_id)| token)
            .collect::<Vec<_>>();

        let tokens = input_tokens
            .into_iter()
            .chain(output_tokens)
            .collect::<std::collections::HashSet<_>>();

        let tokens: Vec<_> = tokens.into_iter().collect();

        let balance_changes = valid_inputs
            .into_iter()
            .chain(valid_outputs)
            .map(|(token, _decimals, _vault_id)| BalanceChange {
                token,
                // NOTE: This might be wrong as the vault can already exist and have a balance
                balance: BigInt::from(0).to_signed_bytes_be(),
                component_id: event.order_hash.clone().to_vec(),
            })
            .collect::<Vec<_>>();

        let component_id = format!(
            "{owner}:{order}",
            owner = owner.to_vec().to_hex(),
            order = event.order_hash.to_vec().to_hex()
        );

        new_orders.push(TransactionEntityChanges {
            tx: Some(tycho_tx.clone()),
            entity_changes: vec![EntityChanges {
                component_id: component_id.clone(),

                attributes: vec![
                    // Attribute {
                    //     name: "liquidity".to_string(),
                    //     value: BigInt::from(0).to_signed_bytes_be(),
                    //     change: ChangeType::Creation.into(),
                    // },
                    // Attribute {
                    //     name: "tick".to_string(),
                    //     value: BigInt::from(0).to_signed_bytes_be(),
                    //     change: ChangeType::Creation.into(),
                    // },
                    // Attribute {
                    //     name: "sqrt_price_x96".to_string(),
                    //     value: BigInt::from(0).to_signed_bytes_be(),
                    //     change: ChangeType::Creation.into(),
                    // },
                ],
            }],

            component_changes: vec![ProtocolComponent {
                id: component_id,
                tokens,
                contracts: vec![],
                static_att: vec![
                    // Attribute {
                    //     name: "fee".to_string(),
                    //     value: event.fee.to_signed_bytes_be(),
                    //     change: ChangeType::Creation.into(),
                    // },
                    // Attribute {
                    //     name: "tick_spacing".to_string(),
                    //     value: event.tick_spacing.to_signed_bytes_be(),
                    //     change: ChangeType::Creation.into(),
                    // },
                    // Attribute {
                    //     name: "pool_address".to_string(),
                    //     value: event.pool.clone(),
                    //     change: ChangeType::Creation.into(),
                    // },
                ],
                change: i32::from(ChangeType::Creation),
                protocol_type: Option::from(ProtocolType {
                    name: "raindex_order".to_string(),
                    financial_type: FinancialType::Swap.into(),
                    attribute_schema: vec![],
                    implementation_type: ImplementationType::Custom.into(),
                }),
                tx: Some(tycho_tx),
            }],
            balance_changes,
            // balance_changes: vec![ ],
        })
    };

    let mut eh = EventHandler::new(block);

    eh.filter_by_address(vec![Address::from_str(factory_address).unwrap()]);

    eh.on::<AddOrderV2, _>(&mut on_order_added);
    eh.handle_events();
}
