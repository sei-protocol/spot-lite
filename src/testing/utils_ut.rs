use cosmwasm_std::Decimal;

use crate::utils::decimal_to_u128;

#[test]
pub fn test_decimal_to_u128() {
    let one = Decimal::one();
    assert_eq!(1, decimal_to_u128(one));

    let zero = Decimal::zero();
    assert_eq!(0, decimal_to_u128(zero));

    let ten = Decimal::from_atomics(10u128, 0).unwrap();
    assert_eq!(10, decimal_to_u128(ten));

    let fractional = Decimal::from_atomics(15u128, 1).unwrap();
    assert_eq!(1, decimal_to_u128(fractional));
}
