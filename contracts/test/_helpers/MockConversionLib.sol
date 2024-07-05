// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {MockERC20} from "../_mocks/MockERC20.sol";

/// @author philogy <https://github.com/philogy>
library MockConversionLib {
    function into(MockERC20 erc20) internal pure returns (address) {
        return address(erc20);
    }
}
