// SPDX-License-Identifier: MIT
pragma solidity ^0.8.4;

import {console2 as console} from "forge-std/console2.sol";

struct TickRewards {
    mapping(int24 tick => uint256 growthOutside) tickGrowthOutside;
    uint256 globalGrowth;
}

using TickRewardsLib for TickRewards global;

library TickRewardsLib {
    function getGrowthInside(TickRewards storage self, int24 current, int24 lower, int24 upper)
        internal
        view
        returns (uint256)
    {
        uint256 lowerGrowth = self.tickGrowthOutside[lower];
        uint256 upperGrowth = self.tickGrowthOutside[upper];
        if (current < lower) {
            return lowerGrowth - upperGrowth;
        } else if (current >= upper) {
            return upperGrowth - lowerGrowth;
        } else {
            return self.globalGrowth - lowerGrowth - upperGrowth;
        }
    }
}
