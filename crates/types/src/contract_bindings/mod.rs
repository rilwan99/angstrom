pub mod pool_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolManager,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/PoolManager.sol/PoolManager.\
         json"
    );
}

pub mod mock_rewards_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockRewardsManager,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/MockRewardsManager.sol/\
         MockRewardsManager.json"
    );
}

pub mod angstrom {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        Angstrom,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/Angstrom.sol/Angstrom.json"
    );
}

pub mod pool_gate {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolGate,
        "/home/will/ghq/github.com/SorellaLabs/angstrom/contracts/out/PoolGate.sol/PoolGate.json"
    );
}
