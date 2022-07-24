use crate::state::{
    Balance, DepositInfo, LiquidationRequest, Order, OrderPlacement, SettlementEntry,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    Settlement {
        epoch: i64,
        entries: Vec<SettlementEntry>,
    },

    BulkOrderPlacements {
        orders: Vec<OrderPlacement>,
        deposits: Vec<DepositInfo>,
    },

    BulkOrderCancellations {
        ids: Vec<u64>,
    },

    Liquidation {
        requests: Vec<LiquidationRequest>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BulkOrderPlacementsResponse {
    pub unsuccessful_order_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance { account: String, denom: String },
    GetOrder { id: u64 },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct GetBalanceResponse {
    pub balance: Balance,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct GetOrderResponse {
    pub order: Order,
}
