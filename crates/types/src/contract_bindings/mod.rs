pub mod PoolManager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolManager,
        "../../contracts/out/PoolManager.sol/PoolManager.json"
    );
}
pub mod MockRewardsManager {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        MockRewardsManager,
        "../../contracts/out/MockRewardsManager.sol/MockRewardsManager.json"
    );
}
pub mod Angstrom {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        Angstrom,
        "../../contracts/out/Angstrom.sol/Angstrom.json"
    );
}
pub mod PoolGate {
    alloy::sol!(
        #[allow(missing_docs)]
        #[sol(rpc)]
        PoolGate,
        "../../contracts/out/PoolGate.sol/PoolGate.json"
    );
}
