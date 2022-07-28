use crate::error::ContractError;
use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, Copy)]
pub struct Balance {
    pub amount: Decimal,
    pub withheld: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Order {
    pub id: u64,
    pub account: String,
    pub price_denom: String,
    pub asset_denom: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub remaining_quantity: Decimal,
    pub direction: PositionDirection,
    pub effect: PositionEffect,
    pub order_type: OrderType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SettlementEntry {
    pub account: String,
    pub price_denom: String,
    pub asset_denom: String,
    pub quantity: Decimal,
    pub execution_cost_or_proceed: Decimal,
    pub expected_cost_or_proceed: Decimal,
    pub position_direction: PositionDirection,
    pub order_type: OrderType,
    pub order_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderPlacement {
    pub id: u64,
    pub status: i32,
    pub account: String,
    pub contract_address: String,
    pub price_denom: String,
    pub asset_denom: String,
    pub price: Decimal,
    pub quantity: Decimal,
    pub order_type: i32,
    pub position_direction: i32,
    pub data: String,
    pub status_description: String,
}

impl OrderPlacement {
    pub fn to_order(&self) -> Result<Order, ContractError> {
        let order_data: OrderData = match serde_json_wasm::from_str(&self.data) {
            Ok(data) => data,
            Err(_) => return Result::Err(ContractError::InvalidOrderData {}),
        };
        let order = Order {
            id: self.id,
            account: self.account.to_owned(),
            price_denom: self.price_denom.to_owned(),
            asset_denom: self.asset_denom.to_owned(),
            price: self.price,
            quantity: self.quantity,
            remaining_quantity: self.quantity,
            direction: i32_to_direction(self.position_direction),
            order_type: i32_to_order_type(self.order_type),
            effect: order_data.position_effect,
        };
        Result::Ok(order)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OrderData {
    pub position_effect: PositionEffect,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DepositInfo {
    pub account: String,
    pub denom: String,
    pub amount: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct LiquidationRequest {
    pub requestor: String,
    pub account: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema, Eq, Hash)]
pub enum PositionDirection {
    Unknown,
    Long,
    Short,
}

pub fn i32_to_direction(i: i32) -> PositionDirection {
    match i {
        0i32 => PositionDirection::Long,
        1i32 => PositionDirection::Short,
        _ => PositionDirection::Unknown,
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema, Eq, Hash)]
pub enum OrderType {
    Unknown,
    Limit,
    Market,
    Liquidation,
}

pub fn i32_to_order_type(i: i32) -> OrderType {
    match i {
        0i32 => OrderType::Limit,
        1i32 => OrderType::Market,
        2i32 => OrderType::Liquidation,
        _ => OrderType::Unknown,
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, JsonSchema, Eq, Hash)]
pub enum PositionEffect {
    Unknown,
    Open,
    Close,
}

#[derive(Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Debug, JsonSchema)]
pub struct Pair {
    pub price_denom: String,
    pub asset_denom: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct LiquidationResponse {
    pub successful_accounts: Vec<String>,
    pub liquidation_orders: Vec<OrderPlacement>,
}
