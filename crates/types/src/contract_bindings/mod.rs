pub mod mintable_mock_erc_20 {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MintableMockERC20,
        "/Users/dbresnick/github/angstrom/contracts/out/MintableMockERC20.sol/MintableMockERC20.json"
    );
}

pub mod mock_rewards_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockRewardsManager,
        "/Users/dbresnick/github/angstrom/contracts/out/MockRewardsManager.sol/MockRewardsManager.json"
    );
}

pub mod pool_manager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolManager,
        "/Users/dbresnick/github/angstrom/contracts/out/PoolManager.sol/PoolManager.json"
    );
}

pub mod pool_gate {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolGate,
        "/Users/dbresnick/github/angstrom/contracts/out/PoolGate.sol/PoolGate.json"
    );
}

pub mod angstrom {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        Angstrom,
        "/Users/dbresnick/github/angstrom/contracts/out/Angstrom.sol/Angstrom.json"
    );
}

