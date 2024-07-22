// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Vm} from "forge-std/Vm.sol";
import {UintVec, VecLib} from "super-sol/collections/Vec.sol";
import {UserOrder, OrderMeta} from "../../../src/reference/UserOrder.sol";
import {TypedDataHasher} from "src/types/TypedDataHasher.sol";

struct Trader {
    uint256 key;
    address addr;
    uint256 nextNonce;
}

using TraderLib for Trader global;

/// @author philogy <https://github.com/philogy>
library TraderLib {
    using VecLib for UintVec;

    Vm constant vm = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    /// @dev `keccak256("Trader.nonceCache.slot") - 1`
    uint256 internal constant _NEXT_NONCE_CACHE_SLOT =
        0x4ef60bdb50c7c20e080f0dbe0c461b46fed86729a8fde2574c4a51e10dac62ad;

    function sign(Trader memory self, bytes32 hash) internal pure returns (uint8 v, bytes32 r, bytes32 s) {
        (v, r, s) = vm.sign(self.key, hash);
    }

    function sign(Trader memory self, UserOrder order, TypedDataHasher typedHasher) internal pure {
        bytes32 hash = order.hash712(typedHasher);
        (uint8 v, bytes32 r, bytes32 s) = self.sign(hash);
        order.setMeta(OrderMeta({from: self.addr, signature: abi.encodePacked(v, r, s)}));
    }

    function getFreshNonce(Trader memory self) internal pure returns (uint256 nonce) {
        nonce = self.nextNonce++;
    }
}
