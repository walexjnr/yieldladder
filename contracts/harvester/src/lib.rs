#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

/// Permissionless yield harvester that claims AMM trading fees and compounds
/// them into [`StrategyVault`].
///
/// Any caller may invoke `harvest()` at any time; the contract trustlessly
/// distributes a 10 bps bounty on the harvested yield to the caller as an
/// incentive, with the remainder forwarded to `StrategyVault`.
#[contract]
pub struct Harvester;

#[contractimpl]
impl Harvester {
    // harvest() — permissionless; claims AMM trading fees and compounds into StrategyVault.
    // Caller receives 10 bps bounty on harvested yield.
    // Full implementation in subsequent commits.
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn contract_instantiates() {
        let env = Env::default();
        let _id = env.register_contract(None, Harvester);
    }
}
