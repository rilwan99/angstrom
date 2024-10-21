pub mod mock_rewards_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockRewardsManager,
        "/Users/josephnoorchashm/Desktop/SorellaLabs/GitHub/angstrom/contracts/out/MockRewardsManager.sol/MockRewardsManager.json"
    );
}

pub mod pool_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolManager,
        "/Users/josephnoorchashm/Desktop/SorellaLabs/GitHub/angstrom/contracts/out/PoolManager.sol/PoolManager.json"
    );
}

pub mod pool_gate {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolGate,
        "/Users/josephnoorchashm/Desktop/SorellaLabs/GitHub/angstrom/contracts/out/PoolGate.sol/PoolGate.json"
    );
}

pub mod angstrom {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        Angstrom,
        "/Users/josephnoorchashm/Desktop/SorellaLabs/GitHub/angstrom/contracts/out/Angstrom.sol/Angstrom.json"
    );
}

