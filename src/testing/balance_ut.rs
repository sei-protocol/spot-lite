use cosmwasm_std::Decimal;

use crate::balance::{get_balance, save_balance};
use crate::state::Balance;
use crate::testing::mock_querier::mock_dependencies;

const TEST_ACCOUNT: &str = "account";
const TEST_DENOM: &str = "denom";

#[test]
fn test_save_get_balance() {
    let mut deps = mock_dependencies(&vec![]);
    let balance = Balance {
        amount: Decimal::from_atomics(123u128, 0).unwrap(),
        withheld: Decimal::from_atomics(456u128, 0).unwrap(),
    };
    save_balance(
        deps.as_mut().storage,
        TEST_ACCOUNT.to_owned(),
        TEST_DENOM.to_owned(),
        &balance,
    );
    let stored_balance = get_balance(
        deps.as_ref().storage,
        TEST_ACCOUNT.to_owned(),
        TEST_DENOM.to_owned(),
    );
    assert_eq!(balance, stored_balance);
}
