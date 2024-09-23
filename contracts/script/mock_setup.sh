#!/bin/bash
forge script script/test.s.sol --rpc-url http://localhost:8545 --broadcast -vvv
export V4_POOL_MANAGER=ef11D1c2aA48826D4c41e54ab82D1Ff5Ad8A64Ca
forge script script/MockRewardsManager.s.sol --rpc-url http://localhost:8545 --broadcast -vvv