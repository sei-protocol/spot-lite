use crate::state::{DepositInfo, LiquidationRequest, OrderPlacement, SettlementEntry};
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
