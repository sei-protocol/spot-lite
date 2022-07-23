use cosmwasm_std::testing::{MockApi, MockQuerier, MockQuerierCustomHandlerResult, MockStorage};
use cosmwasm_std::{to_binary, Coin, ContractResult, Decimal, OwnedDeps, SystemResult, Uint64};
use sei_cosmwasm::{
    DenomOracleExchangeRatePair, DexPair, DexTwap, DexTwapsResponse, ExchangeRatesResponse,
    OracleExchangeRate, OracleTwap, OracleTwapsResponse, SeiQuery, SeiQueryWrapper, SeiRoute,
};
use std::marker::PhantomData;

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<SeiQueryWrapper>, SeiQueryWrapper> {
    let mock_querier = MockQuerier::new(&[("addr0001", contract_balance)])
        .with_custom_handler(mock_custom_handler);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: mock_querier,
        custom_query_type: PhantomData,
    }
}

fn mock_custom_handler(custom_query: &SeiQueryWrapper) -> MockQuerierCustomHandlerResult {
    match custom_query.route {
        SeiRoute::Oracle => match custom_query.query_data {
            SeiQuery::ExchangeRates {} => SystemResult::Ok(ContractResult::Ok(
                to_binary(&mock_exchange_rates_response()).unwrap(),
            )),
            SeiQuery::OracleTwaps { lookback_seconds } => SystemResult::Ok(ContractResult::Ok(
                to_binary(&mock_oracle_twap_response(lookback_seconds)).unwrap(),
            )),
            SeiQuery::DexTwaps { .. } => panic!("Oracle route has no DexTypes"),
            SeiQuery::Epoch {} => panic!("Oracle route has no Epoch"),
            _ => panic!("Invalid route"),
        },
        SeiRoute::Dex => match custom_query.query_data {
            SeiQuery::ExchangeRates {} => panic!("Dex route has no ExchangeRates"),
            SeiQuery::OracleTwaps { .. } => panic!("Dex route has no OracleTwaps"),
            SeiQuery::DexTwaps {
                lookback_seconds, ..
            } => SystemResult::Ok(ContractResult::Ok(
                to_binary(&mock_dex_twap_response(lookback_seconds)).unwrap(),
            )),
            SeiQuery::Epoch {} => panic!("Dex route has no Epoch"),
            _ => panic!("Invalid route"),
        },
        SeiRoute::Epoch => panic!("Epoch route not implemented"),
    }
}

fn mock_exchange_rates_response() -> ExchangeRatesResponse {
    ExchangeRatesResponse {
        denom_oracle_exchange_rate_pairs: vec![
            DenomOracleExchangeRatePair {
                denom: "uusdc".to_owned(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::one(), // 1
                    last_update: Uint64::zero(),
                },
            },
            DenomOracleExchangeRatePair {
                denom: "usei".to_owned(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::from_atomics(1u128, 1).unwrap(), // 0.1
                    last_update: Uint64::zero(),
                },
            },
            DenomOracleExchangeRatePair {
                denom: "uatom".to_owned(),
                oracle_exchange_rate: OracleExchangeRate {
                    exchange_rate: Decimal::from_atomics(10u128, 0).unwrap(), // 10
                    last_update: Uint64::zero(),
                },
            },
        ],
    }
}

fn mock_oracle_twap_response(lookback_seconds: i64) -> OracleTwapsResponse {
    OracleTwapsResponse {
        oracle_twaps: vec![
            OracleTwap {
                denom: "uusdc".to_owned(),
                twap: Decimal::one(),
                lookback_seconds: lookback_seconds,
            },
            OracleTwap {
                denom: "usei".to_owned(),
                twap: Decimal::from_atomics(1u128, 1).unwrap(),
                lookback_seconds: lookback_seconds,
            },
            OracleTwap {
                denom: "uatom".to_owned(),
                twap: Decimal::from_atomics(10u128, 0).unwrap(),
                lookback_seconds: lookback_seconds,
            },
        ],
    }
}

fn mock_dex_twap_response(lookback_seconds: u64) -> DexTwapsResponse {
    DexTwapsResponse {
        twaps: vec![
            DexTwap {
                pair: DexPair {
                    price_denom: "USDC".to_owned(),
                    asset_denom: "SEI".to_owned(),
                    tick_size: Decimal::from_atomics(1u128, 2).unwrap(),
                },
                twap: Decimal::from_atomics(2u128, 1).unwrap(),
                lookback_seconds: lookback_seconds,
            },
            DexTwap {
                pair: DexPair {
                    price_denom: "USDC".to_owned(),
                    asset_denom: "ATOM".to_owned(),
                    tick_size: Decimal::from_atomics(1u128, 2).unwrap(),
                },
                twap: Decimal::from_atomics(10u128, 0).unwrap(),
                lookback_seconds: lookback_seconds,
            },
        ],
    }
}
