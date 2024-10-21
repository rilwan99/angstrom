pub mod pool_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolManager,
        "../../contracts/out/PoolManager.sol/PoolManager.json"
    );
}

pub mod mock_rewards_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockRewardsManager,
        "../../contracts/out/MockRewardsManager.sol/MockRewardsManager.json"
    );
}

pub mod angstrom {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        Angstrom,
        "../../contracts/out/Angstrom.sol/Angstrom.json"
    );
}

pub mod pool_gate {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolGate,
        "../../contracts/out/PoolGate.sol/PoolGate.json"
    );
}
