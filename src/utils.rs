use cosmwasm_std::Decimal;

pub fn decimal_to_u128(decimal: Decimal) -> u128 {
    let atomics = decimal.atomics().u128();
    let denominator = 10u128.pow(decimal.decimal_places()) as u128;
    atomics / denominator
}
