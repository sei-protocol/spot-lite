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

    let dec1 = Decimal::from_atomics(600u128, 0).unwrap();
    assert_eq!(600, decimal_to_u128(dec1));

    let dec1 = Decimal::from_atomics(600u128, 1).unwrap();
    assert_eq!(60, decimal_to_u128(dec1));

    let dec1 = Decimal::from_atomics(600u128, 3).unwrap();
    assert_eq!(0, decimal_to_u128(dec1));
}
