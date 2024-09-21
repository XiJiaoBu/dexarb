mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1::PoolSwapCall;
use pb::contract::v1::SushipoolSwap;
use pb::contract::v1 as contract;
use prost_types::Timestamp;
use substreams::prelude::*;
use substreams::store;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use crate::contract::OneinchfactorySwapped1;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const FACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("1f98431c8ad98523631ae4a59f267346ea31f984");
const SUSHIFACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("c0aee478e3658e2610c5f7a4a2e1777ce9e4f2ac");
const ONEINCHFACTORY_TRACKED_CONTRACT: [u8; 20] = hex!("11111112542d85b3ef69ae05771c2dccff4faa26");

fn map_factory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.factory_fee_amount_enableds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::FeeAmountEnabled::match_and_decode(log) {
                        return Some(contract::FactoryFeeAmountEnabled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            fee: event.fee.to_u64(),
                            tick_spacing: Into::<num_bigint::BigInt>::into(event.tick_spacing).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.factory_owner_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::OwnerChanged::match_and_decode(log) {
                        return Some(contract::FactoryOwnerChanged {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            old_owner: event.old_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.factory_pool_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::factory_contract::events::PoolCreated::match_and_decode(log) {
                        return Some(contract::FactoryPoolCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            fee: event.fee.to_u64(),
                            pool: event.pool,
                            tick_spacing: Into::<num_bigint::BigInt>::into(event.tick_spacing).to_i64().unwrap(),
                            token0: event.token0,
                            token1: event.token1,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_sushifactory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.sushifactory_pair_createds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == SUSHIFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::sushifactory_contract::events::PairCreated::match_and_decode(log) {
                        return Some(contract::SushifactoryPairCreated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            pair: event.pair,
                            param3: event.param3.to_string(),
                            token0: event.token0,
                            token1: event.token1,
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_oneinchfactory_events(blk: &eth::Block, events: &mut contract::Events) {
    events.oneinchfactory_error_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ONEINCHFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::oneinchfactory_contract::events::Error1::match_and_decode(log) {
                        return Some(contract::OneinchfactoryError1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            reason: event.reason,
                        });
                    }

                    None
                })
        })
        .collect());
    events.oneinchfactory_error_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ONEINCHFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::oneinchfactory_contract::events::Error2::match_and_decode(log) {
                        return Some(contract::OneinchfactoryError2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            reason: event.reason,
                        });
                    }

                    None
                })
        })
        .collect());
    events.oneinchfactory_ownership_transferred_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ONEINCHFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::oneinchfactory_contract::events::OwnershipTransferred1::match_and_decode(log) {
                        return Some(contract::OneinchfactoryOwnershipTransferred1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.oneinchfactory_ownership_transferred_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ONEINCHFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::oneinchfactory_contract::events::OwnershipTransferred2::match_and_decode(log) {
                        return Some(contract::OneinchfactoryOwnershipTransferred2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.oneinchfactory_swapped_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ONEINCHFACTORY_TRACKED_CONTRACT)
                .filter_map(move |log| {
                    if let Some(event) = abi::oneinchfactory_contract::events::Swapped1::match_and_decode(log) {
    let mut oneinch_swapped_event = contract::OneinchfactorySwapped1 {
        evt_tx_hash: Hex(&view.transaction.hash).to_string(),
        evt_index: log.block_index,
        evt_block_time: Some(blk.timestamp().to_owned()),
        evt_block_number: blk.number,
        dst_receiver: event.dst_receiver,
        dst_token: event.dst_token,
        return_amount: event.return_amount.to_string(),
        sender: event.sender,
        spent_amount: event.spent_amount.to_string(),
        src_token: event.src_token,
        price_src_token_per_dst_token: "he".to_string() // TODO: FIX
    };
    
    // Calculate token price
    handle_oneinchfactory_swapped(&mut oneinch_swapped_event); // Call the handler to calculate price
    
    events.oneinchfactory_swapped_1s.push(oneinch_swapped_event);
}

    
                    None
                })
        })
        .collect());
    events.oneinchfactory_swapped_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == ONEINCHFACTORY_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::oneinchfactory_contract::events::Swapped2::match_and_decode(log) {
                        // Parse amounts for price calculation
                        let spent_amount: f64 = event.spent_amount.to_string().parse().unwrap_or(0.0);
                        let return_amount: f64 = event.return_amount.to_string().parse().unwrap_or(0.0);
    
                        // Calculate token price
                        let price = calculate_token_price(spent_amount, return_amount);
    
                        return Some(contract::OneinchfactorySwapped2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            dst_receiver: event.dst_receiver,
                            dst_token: event.dst_token,
                            return_amount: event.return_amount.to_string(),
                            sender: event.sender,
                            spent_amount: event.spent_amount.to_string(),
                            src_token: event.src_token,
                        });
                    }
    
                    None
                })
        })
        .collect());
}
fn map_oneinchfactory_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.oneinchfactory_call_destroy_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::Destroy1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::Destroy1::decode(call) {
                        Ok(_decoded_call) => {
                            Some(contract::OneinchfactoryDestroy1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_destroy_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::Destroy2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::Destroy2::decode(call) {
                        Ok(_decoded_call) => {
                            Some(contract::OneinchfactoryDestroy2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_discounted_swap_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::DiscountedSwap1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::DiscountedSwap1::decode(call) {
                        Ok(decoded_call) => {
                            let (output_return_amount, output_gas_left, output_chi_spent) = match abi::oneinchfactory_contract::functions::DiscountedSwap1::output(&call.return_data) {
                                Ok((output_return_amount, output_gas_left, output_chi_spent)) => {(output_return_amount, output_gas_left, output_chi_spent)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactoryDiscountedSwap1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                caller: decoded_call.caller,
                                data: decoded_call.data,
                                output_chi_spent: output_chi_spent.to_string(),
                                output_gas_left: output_gas_left.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_discounted_swap_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::DiscountedSwap2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::DiscountedSwap2::decode(call) {
                        Ok(decoded_call) => {
                            let (output_return_amount, output_gas_left, output_chi_spent) = match abi::oneinchfactory_contract::functions::DiscountedSwap2::output(&call.return_data) {
                                Ok((output_return_amount, output_gas_left, output_chi_spent)) => {(output_return_amount, output_gas_left, output_chi_spent)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactoryDiscountedSwap2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                caller: decoded_call.caller,
                                data: decoded_call.data,
                                output_chi_spent: output_chi_spent.to_string(),
                                output_gas_left: output_gas_left.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_renounce_ownership_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::RenounceOwnership1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::RenounceOwnership1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::OneinchfactoryRenounceOwnership1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_renounce_ownership_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::RenounceOwnership2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::RenounceOwnership2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::OneinchfactoryRenounceOwnership2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_rescue_funds_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::RescueFunds1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::RescueFunds1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::OneinchfactoryRescueFunds1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                token: decoded_call.token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_rescue_funds_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::RescueFunds2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::RescueFunds2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::OneinchfactoryRescueFunds2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                token: decoded_call.token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_swap_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::Swap1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::Swap1::decode(call) {
                        Ok(decoded_call) => {
                            let (output_return_amount, output_gas_left) = match abi::oneinchfactory_contract::functions::Swap1::output(&call.return_data) {
                                Ok((output_return_amount, output_gas_left)) => {(output_return_amount, output_gas_left)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactorySwap1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                caller: decoded_call.caller,
                                data: decoded_call.data,
                                output_gas_left: output_gas_left.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_swap_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::Swap2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::Swap2::decode(call) {
                        Ok(decoded_call) => {
                            let (output_return_amount, output_gas_left) = match abi::oneinchfactory_contract::functions::Swap2::output(&call.return_data) {
                                Ok((output_return_amount, output_gas_left)) => {(output_return_amount, output_gas_left)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactorySwap2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                caller: decoded_call.caller,
                                data: decoded_call.data,
                                output_gas_left: output_gas_left.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_transfer_ownership_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::TransferOwnership1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::TransferOwnership1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::OneinchfactoryTransferOwnership1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_transfer_ownership_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::TransferOwnership2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::TransferOwnership2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::OneinchfactoryTransferOwnership2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_owner: decoded_call.new_owner,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_unoswap_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::Unoswap1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::Unoswap1::decode(call) {
                        Ok(decoded_call) => {
                            let output_return_amount = match abi::oneinchfactory_contract::functions::Unoswap1::output(&call.return_data) {
                                Ok(output_return_amount) => {output_return_amount}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactoryUnoswap1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                min_return: decoded_call.min_return.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                                param3: decoded_call.param3.into_iter().map(|x| Vec::from(x)).collect::<Vec<_>>(),
                                src_token: decoded_call.src_token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_unoswap_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::Unoswap2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::Unoswap2::decode(call) {
                        Ok(decoded_call) => {
                            let output_return_amount = match abi::oneinchfactory_contract::functions::Unoswap2::output(&call.return_data) {
                                Ok(output_return_amount) => {output_return_amount}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactoryUnoswap2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                min_return: decoded_call.min_return.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                                param3: decoded_call.param3.into_iter().map(|x| Vec::from(x)).collect::<Vec<_>>(),
                                src_token: decoded_call.src_token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_unoswap_with_permit_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::UnoswapWithPermit1::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::UnoswapWithPermit1::decode(call) {
                        Ok(decoded_call) => {
                            let output_return_amount = match abi::oneinchfactory_contract::functions::UnoswapWithPermit1::output(&call.return_data) {
                                Ok(output_return_amount) => {output_return_amount}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactoryUnoswapWithPermit1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                min_return: decoded_call.min_return.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                                permit: decoded_call.permit,
                                pools: decoded_call.pools.into_iter().map(|x| Vec::from(x)).collect::<Vec<_>>(),
                                src_token: decoded_call.src_token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.oneinchfactory_call_unoswap_with_permit_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == ONEINCHFACTORY_TRACKED_CONTRACT && abi::oneinchfactory_contract::functions::UnoswapWithPermit2::match_call(call))
                .filter_map(|call| {
                    match abi::oneinchfactory_contract::functions::UnoswapWithPermit2::decode(call) {
                        Ok(decoded_call) => {
                            let output_return_amount = match abi::oneinchfactory_contract::functions::UnoswapWithPermit2::output(&call.return_data) {
                                Ok(output_return_amount) => {output_return_amount}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::OneinchfactoryUnoswapWithPermit2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                min_return: decoded_call.min_return.to_string(),
                                output_return_amount: output_return_amount.to_string(),
                                permit: decoded_call.permit,
                                pools: decoded_call.pools.into_iter().map(|x| Vec::from(x)).collect::<Vec<_>>(),
                                src_token: decoded_call.src_token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}

#[derive(Default, Debug, Clone)]
pub struct PoolSwap {
    pub evt_tx_hash: String,
    pub evt_index: u64,
    pub evt_block_time: Option<Timestamp>,
    pub evt_block_number: u64,
    pub evt_address: String,
    pub amount0: String,
    pub amount1: String,
    pub liquidity: String,
    pub recipient: String,
    pub sender: String,
    pub sqrt_price_x96: String,
    pub tick: i64,
    pub price_token0_per_token1: String,
    pub expected_price: String,
    pub executed_price: String
}


fn is_declared_dds_address(addr: &Vec<u8>, ordinal: u64, dds_store: &store::StoreGetInt64) -> bool {
    //    substreams::log::info!("Checking if address {} is declared dds address", Hex(addr).to_string());
    if dds_store.get_at(ordinal, Hex(addr).to_string()).is_some() {
        return true;
    }
    return false;
}
fn map_pool_events(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    events: &mut contract::Events,
) {

    events.pool_burns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Burn::match_and_decode(log) {
                        return Some(contract::PoolBurn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount: event.amount.to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            owner: event.owner,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_collects.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Collect::match_and_decode(log) {
                        return Some(contract::PoolCollect {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            owner: event.owner,
                            recipient: event.recipient,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_collect_protocols.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::CollectProtocol::match_and_decode(log) {
                        return Some(contract::PoolCollectProtocol {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            recipient: event.recipient,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_flashes.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Flash::match_and_decode(log) {
                        return Some(contract::PoolFlash {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            paid0: event.paid0.to_string(),
                            paid1: event.paid1.to_string(),
                            recipient: event.recipient,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_increase_observation_cardinality_nexts.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::IncreaseObservationCardinalityNext::match_and_decode(log) {
                        return Some(contract::PoolIncreaseObservationCardinalityNext {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            observation_cardinality_next_new: event.observation_cardinality_next_new.to_u64(),
                            observation_cardinality_next_old: event.observation_cardinality_next_old.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_initializes.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Initialize::match_and_decode(log) {
                        return Some(contract::PoolInitialize {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            sqrt_price_x96: event.sqrt_price_x96.to_string(),
                            tick: Into::<num_bigint::BigInt>::into(event.tick).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_mints.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::Mint::match_and_decode(log) {
                        return Some(contract::PoolMint {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount: event.amount.to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            owner: event.owner,
                            sender: event.sender,
                            tick_lower: Into::<num_bigint::BigInt>::into(event.tick_lower).to_i64().unwrap(),
                            tick_upper: Into::<num_bigint::BigInt>::into(event.tick_upper).to_i64().unwrap(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.pool_set_fee_protocols.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::pool_contract::events::SetFeeProtocol::match_and_decode(log) {
                        return Some(contract::PoolSetFeeProtocol {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            fee_protocol0_new: event.fee_protocol0_new.to_u64(),
                            fee_protocol0_old: event.fee_protocol0_old.to_u64(),
                            fee_protocol1_new: event.fee_protocol1_new.to_u64(),
                            fee_protocol1_old: event.fee_protocol1_old.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());

        events.pool_swaps.append(&mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                    .filter_map(|log| {
                        if let Some(event) = abi::pool_contract::events::Swap::match_and_decode(log) {
                            return Some(contract::PoolSwap {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                evt_address: Hex(&log.address).to_string(),
                                amount0: event.amount0.to_string(),
                                amount1: event.amount1.to_string(),
                                liquidity: event.liquidity.to_string(),
                                recipient: event.recipient,
                                sender: event.sender,
                                sqrt_price_x96: event.sqrt_price_x96.to_string(),
                                tick: Into::<num_bigint::BigInt>::into(event.tick).to_i64().unwrap(),
                                expected_price: 0, // TODO: FIX
                                executed_price: 0 // TODO: FIX
                            });
                        }
    
                        None
                    })
        })
        .collect());
}
fn map_pool_calls(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    calls: &mut contract::Calls,
) {
    calls.pool_call_burns.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::Burn::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::Burn::decode(call) {
                            Ok(decoded_call) => {
                            let (output_amount0, output_amount1) = match abi::pool_contract::functions::Burn::output(&call.return_data) {
                                Ok((output_amount0, output_amount1)) => {(output_amount0, output_amount1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolBurnCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount: decoded_call.amount.to_string(),
                                output_amount0: output_amount0.to_string(),
                                output_amount1: output_amount1.to_string(),
                                tick_lower: Into::<num_bigint::BigInt>::into(decoded_call.tick_lower).to_i64().unwrap(),
                                tick_upper: Into::<num_bigint::BigInt>::into(decoded_call.tick_upper).to_i64().unwrap(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_collects.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::Collect::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::Collect::decode(call) {
                            Ok(decoded_call) => {
                            let (output_amount0, output_amount1) = match abi::pool_contract::functions::Collect::output(&call.return_data) {
                                Ok((output_amount0, output_amount1)) => {(output_amount0, output_amount1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolCollectCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount0_requested: decoded_call.amount0_requested.to_string(),
                                amount1_requested: decoded_call.amount1_requested.to_string(),
                                output_amount0: output_amount0.to_string(),
                                output_amount1: output_amount1.to_string(),
                                recipient: decoded_call.recipient,
                                tick_lower: Into::<num_bigint::BigInt>::into(decoded_call.tick_lower).to_i64().unwrap(),
                                tick_upper: Into::<num_bigint::BigInt>::into(decoded_call.tick_upper).to_i64().unwrap(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_collect_protocols.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::CollectProtocol::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::CollectProtocol::decode(call) {
                            Ok(decoded_call) => {
                            let (output_amount0, output_amount1) = match abi::pool_contract::functions::CollectProtocol::output(&call.return_data) {
                                Ok((output_amount0, output_amount1)) => {(output_amount0, output_amount1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolCollectProtocolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount0_requested: decoded_call.amount0_requested.to_string(),
                                amount1_requested: decoded_call.amount1_requested.to_string(),
                                output_amount0: output_amount0.to_string(),
                                output_amount1: output_amount1.to_string(),
                                recipient: decoded_call.recipient,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_flashes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::Flash::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::Flash::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolFlashCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount0: decoded_call.amount0.to_string(),
                                amount1: decoded_call.amount1.to_string(),
                                data: decoded_call.data,
                                recipient: decoded_call.recipient,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_increase_observation_cardinality_nexts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::IncreaseObservationCardinalityNext::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::IncreaseObservationCardinalityNext::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolIncreaseObservationCardinalityNextCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                observation_cardinality_next: decoded_call.observation_cardinality_next.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::Initialize::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                sqrt_price_x96: decoded_call.sqrt_price_x96.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_mints.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::Mint::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::Mint::decode(call) {
                            Ok(decoded_call) => {
                            let (output_amount0, output_amount1) = match abi::pool_contract::functions::Mint::output(&call.return_data) {
                                Ok((output_amount0, output_amount1)) => {(output_amount0, output_amount1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolMintCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount: decoded_call.amount.to_string(),
                                data: decoded_call.data,
                                output_amount0: output_amount0.to_string(),
                                output_amount1: output_amount1.to_string(),
                                recipient: decoded_call.recipient,
                                tick_lower: Into::<num_bigint::BigInt>::into(decoded_call.tick_lower).to_i64().unwrap(),
                                tick_upper: Into::<num_bigint::BigInt>::into(decoded_call.tick_upper).to_i64().unwrap(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_set_fee_protocols.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::SetFeeProtocol::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::SetFeeProtocol::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::PoolSetFeeProtocolCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                fee_protocol0: decoded_call.fee_protocol0.to_u64(),
                                fee_protocol1: decoded_call.fee_protocol1.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.pool_call_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::pool_contract::functions::Swap::match_call(call))
                .filter_map(|call| {
                    match abi::pool_contract::functions::Swap::decode(call) {
                            Ok(decoded_call) => {
                            let (output_amount0, output_amount1) = match abi::pool_contract::functions::Swap::output(&call.return_data) {
                                Ok((output_amount0, output_amount1)) => {(output_amount0, output_amount1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::PoolSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount_specified: decoded_call.amount_specified.to_string(),
                                data: decoded_call.data,
                                output_amount0: output_amount0.to_string(),
                                output_amount1: output_amount1.to_string(),
                                recipient: decoded_call.recipient,
                                sqrt_price_limit_x96: decoded_call.sqrt_price_limit_x96.to_string(),
                                zero_for_one: decoded_call.zero_for_one,
                                gas_price: 0, // TODO: FIX
                                gas_used: 0 // TODO: FIX
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}


fn map_sushipool_events(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    events: &mut contract::Events,
) {

    events.sushipool_approvals.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::sushipool_contract::events::Approval::match_and_decode(log) {
                        return Some(contract::SushipoolApproval {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            owner: event.owner,
                            spender: event.spender,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.sushipool_burns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::sushipool_contract::events::Burn::match_and_decode(log) {
                        return Some(contract::SushipoolBurn {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            sender: event.sender,
                            to: event.to,
                        });
                    }

                    None
                })
        })
        .collect());

    events.sushipool_mints.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::sushipool_contract::events::Mint::match_and_decode(log) {
                        return Some(contract::SushipoolMint {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            amount0: event.amount0.to_string(),
                            amount1: event.amount1.to_string(),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());

        events.sushipool_swaps.append(&mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt.logs.iter()
                    .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                    .filter_map(|log| {
                        if let Some(event) = abi::sushipool_contract::events::Swap::match_and_decode(log) {
                            return Some(contract::SushipoolSwap {
                                evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                evt_address: Hex(&log.address).to_string(),
                                amount0_in: event.amount0_in.to_string(),
                                amount0_out: event.amount0_out.to_string(),
                                amount1_in: event.amount1_in.to_string(),
                                amount1_out: event.amount1_out.to_string(),
                                sender: event.sender,
                                to: event.to,
                                price_token0_per_token1: event.amount0_in.to_string() // TODO: fix this
                            });
                        }
    
                        None
                    })
        })
        .collect());

    events.sushipool_syncs.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::sushipool_contract::events::Sync::match_and_decode(log) {
                        return Some(contract::SushipoolSync {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            reserve0: event.reserve0.to_string(),
                            reserve1: event.reserve1.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());

    events.sushipool_transfers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| is_declared_dds_address(&log.address, log.ordinal, dds_store))
                .filter_map(|log| {
                    if let Some(event) = abi::sushipool_contract::events::Transfer::match_and_decode(log) {
                        return Some(contract::SushipoolTransfer {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            evt_address: Hex(&log.address).to_string(),
                            from: event.from,
                            to: event.to,
                            value: event.value.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
}
fn map_sushipool_calls(
    blk: &eth::Block,
    dds_store: &store::StoreGetInt64,
    calls: &mut contract::Calls,
) {
    calls.sushipool_call_approves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Approve::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Approve::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::sushipool_contract::functions::Approve::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SushipoolApproveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                output_param0: output_param0,
                                spender: decoded_call.spender,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_burns.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Burn::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Burn::decode(call) {
                            Ok(decoded_call) => {
                            let (output_amount0, output_amount1) = match abi::sushipool_contract::functions::Burn::output(&call.return_data) {
                                Ok((output_amount0, output_amount1)) => {(output_amount0, output_amount1)}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SushipoolBurnCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                output_amount0: output_amount0.to_string(),
                                output_amount1: output_amount1.to_string(),
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_initializes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Initialize::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Initialize::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::SushipoolInitializeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                u_token0: decoded_call.u_token0,
                                u_token1: decoded_call.u_token1,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_mints.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Mint::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Mint::decode(call) {
                            Ok(decoded_call) => {
                            let output_liquidity = match abi::sushipool_contract::functions::Mint::output(&call.return_data) {
                                Ok(output_liquidity) => {output_liquidity}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SushipoolMintCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                output_liquidity: output_liquidity.to_string(),
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_permits.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Permit::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Permit::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::SushipoolPermitCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                deadline: decoded_call.deadline.to_string(),
                                owner: decoded_call.owner,
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                                spender: decoded_call.spender,
                                v: decoded_call.v.to_u64(),
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_skims.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Skim::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Skim::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::SushipoolSkimCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_swaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Swap::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Swap::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::SushipoolSwapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                amount0_out: decoded_call.amount0_out.to_string(),
                                amount1_out: decoded_call.amount1_out.to_string(),
                                data: decoded_call.data,
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_syncs.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Sync::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Sync::decode(call) {
                            Ok(decoded_call) => {
                            Some(contract::SushipoolSyncCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::Transfer::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::Transfer::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::sushipool_contract::functions::Transfer::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SushipoolTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                output_param0: output_param0,
                                to: decoded_call.to,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.sushipool_call_transfer_froms.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| is_declared_dds_address(&call.address, call.begin_ordinal, dds_store) && abi::sushipool_contract::functions::TransferFrom::match_call(call))
                .filter_map(|call| {
                    match abi::sushipool_contract::functions::TransferFrom::decode(call) {
                            Ok(decoded_call) => {
                            let output_param0 = match abi::sushipool_contract::functions::TransferFrom::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::SushipoolTransferFromCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                call_address: Hex(&call.address).to_string(),
                                from: decoded_call.from,
                                output_param0: output_param0,
                                to: decoded_call.to,
                                value: decoded_call.value.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}


#[substreams::handlers::store]
fn store_pool_created(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt
            .receipt
            .logs
            .iter()
            .filter(|log| log.address == FACTORY_TRACKED_CONTRACT)
        {
            if let Some(event) = abi::factory_contract::events::PoolCreated::match_and_decode(log) {
                store.set(log.ordinal, Hex(event.pool).to_string(), &1);
            }
        }
    }
}
#[substreams::handlers::store]
fn store_sushipool_created(blk: eth::Block, store: StoreSetInt64) {
    for rcpt in blk.receipts() {
        for log in rcpt
            .receipt
            .logs
            .iter()
            .filter(|log| log.address == SUSHIFACTORY_TRACKED_CONTRACT)
        {
            if let Some(event) = abi::sushifactory_contract::events::PairCreated::match_and_decode(log) {
                store.set(log.ordinal, Hex(event.pair).to_string(), &1);
            }
        }
    }
}
#[substreams::handlers::map]
fn map_events(
    blk: eth::Block,
    store_pool: StoreGetInt64,
    store_sushipool: StoreGetInt64,
) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_factory_events(&blk, &mut events);
    map_sushifactory_events(&blk, &mut events);
    map_oneinchfactory_events(&blk, &mut events);
    map_pool_events(&blk, &store_pool, &mut events);
    map_sushipool_events(&blk, &store_sushipool, &mut events);
    substreams::skip_empty_output();
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(
    blk: eth::Block,
    store_pool: StoreGetInt64,
    store_sushipool: StoreGetInt64,
) -> Result<contract::Calls, substreams::errors::Error> {
    let mut calls = contract::Calls::default();
    map_pool_calls(&blk, &store_pool, &mut calls);
    map_sushipool_calls(&blk, &store_sushipool, &mut calls);
    map_oneinchfactory_calls(&blk, &mut calls);
    substreams::skip_empty_output();
    Ok(calls)
}

fn calculate_token_price(amount0: f64, amount1: f64) -> f64 {
    if amount1 > 0.0 {
        return amount0 / amount1;
    } else {
        return 0.0; // Avoid division by zero
    }
}
fn handle_pool_swap(swap_event: &mut PoolSwap) {
    let amount0: f64 = swap_event.amount0.parse().unwrap_or(0.0);
    let amount1: f64 = swap_event.amount1.parse().unwrap_or(0.0);
    let price = calculate_token_price(amount0, amount1);
    swap_event.price_token0_per_token1 = price.to_string();
}
fn handle_sushipool_swap(swap_event: &mut SushipoolSwap) {
    let amount0_in = swap_event.amount0_in.parse::<f64>().unwrap();
    let amount1_out = swap_event.amount1_out.parse::<f64>().unwrap();
    
    // Calculate token0 per token1 price
    let price_token0_per_token1 = if amount1_out > 0.0 {
        amount0_in / amount1_out
    } else {
        0.0 // Avoid division by zero
    };
    
    // Update the event with the derived price
    swap_event.price_token0_per_token1 = price_token0_per_token1.to_string();
}

fn handle_oneinchfactory_swapped(swapped_event: &mut OneinchfactorySwapped1) {
    let spent_amount = swapped_event.spent_amount.parse::<f64>().unwrap();
    let return_amount = swapped_event.return_amount.parse::<f64>().unwrap();
    
    // Calculate the price of the source token relative to the destination token
    let price_src_token_per_dst_token = if return_amount > 0.0 {
        spent_amount / return_amount
    } else {
        0.0
    };
    
    // Update the event with the derived price
    swapped_event.price_src_token_per_dst_token = price_src_token_per_dst_token.to_string();
}
fn handle_swap_call(swap_call: PoolSwapCall) {
    // Add logic to track gas price and gas used
    let gas_cost = swap_call.gas_price * swap_call.gas_used;
    println!("Gas cost for the swap: {}", gas_cost);
}
fn detect_arbitrage(price_dex_1: f64, price_dex_2: f64, threshold: f64) -> bool {
    let price_diff = (price_dex_1 - price_dex_2).abs();
    return price_diff > threshold;
}
fn handle_arbitrage_event(pool_swap: PoolSwap, sushi_swap: SushipoolSwap) {
    let price_dex_1 = pool_swap.price_token0_per_token1.parse::<f64>().unwrap();
    let price_dex_2 = sushi_swap.price_token0_per_token1.parse::<f64>().unwrap();

    if detect_arbitrage(price_dex_1, price_dex_2, 0.02) {
        println!("Arbitrage opportunity detected!");
        // Trigger trade execution or alert
    }
}
fn handle_slippage(swap_event: PoolSwap) {
    let expected_price = swap_event.expected_price.parse::<f64>().unwrap();
    let executed_price = swap_event.executed_price.parse::<f64>().unwrap();
    let slippage = ((executed_price - expected_price) / expected_price).abs();

    if slippage > 0.005 { // 0.5% slippage tolerance
        println!("Warning: High slippage detected!");
        // Optionally abort trade execution
    }
}
