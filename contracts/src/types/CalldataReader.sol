// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {OrderVariantMap} from "./OrderVariantMap.sol";
import {console} from "forge-std/console.sol";

/// @dev Represents a calldata offset.
type CalldataReader is uint256;

using CalldataReaderLib for CalldataReader global;
using {neq as !=, eq as ==, gt as >, lt as <, ge as >=, le as <=} for CalldataReader global;

function neq(CalldataReader a, CalldataReader b) pure returns (bool) {
    return CalldataReader.unwrap(a) != CalldataReader.unwrap(b);
}

function eq(CalldataReader a, CalldataReader b) pure returns (bool) {
    return CalldataReader.unwrap(a) == CalldataReader.unwrap(b);
}

function gt(CalldataReader a, CalldataReader b) pure returns (bool) {
    return CalldataReader.unwrap(a) > CalldataReader.unwrap(b);
}

function lt(CalldataReader a, CalldataReader b) pure returns (bool) {
    return CalldataReader.unwrap(a) < CalldataReader.unwrap(b);
}

function ge(CalldataReader a, CalldataReader b) pure returns (bool) {
    return CalldataReader.unwrap(a) >= CalldataReader.unwrap(b);
}

function le(CalldataReader a, CalldataReader b) pure returns (bool) {
    return CalldataReader.unwrap(a) <= CalldataReader.unwrap(b);
}

/// @author philogy <https://github.com/philogy>
library CalldataReaderLib {
    error ReaderNotAtEnd();

    function from(bytes calldata data) internal pure returns (CalldataReader reader) {
        assembly {
            reader := data.offset
        }
    }

    function requireAtEndOf(CalldataReader self, bytes calldata data) internal pure {
        assembly {
            let end := add(data.offset, data.length)
            if iszero(eq(self, end)) {
                mstore(0x00, 0x01842f8c /* ReaderNotAtEnd() */ )
                revert(0x1c, 0x04)
            }
        }
    }

    function requireAtEndOf(CalldataReader self, CalldataReader end) internal pure {
        if (self != end) revert ReaderNotAtEnd();
    }

    function offset(CalldataReader self) internal pure returns (uint256) {
        return CalldataReader.unwrap(self);
    }

    function readU8(CalldataReader self) internal pure returns (CalldataReader, uint8 value) {
        assembly {
            value := byte(0, calldataload(self))
            self := add(self, 1)
        }
        return (self, value);
    }

    function readU16(CalldataReader self) internal pure returns (CalldataReader, uint16 value) {
        assembly {
            value := shr(240, calldataload(self))
            self := add(self, 2)
        }
        return (self, value);
    }

    function readU24(CalldataReader self) internal pure returns (CalldataReader, uint24 value) {
        assembly {
            value := shr(232, calldataload(self))
            self := add(self, 3)
        }
        return (self, value);
    }

    function readI24(CalldataReader self) internal pure returns (CalldataReader, int24 value) {
        assembly {
            value := sar(232, calldataload(self))
            self := add(self, 3)
        }
        return (self, value);
    }

    function readU40(CalldataReader self) internal pure returns (CalldataReader, uint40 value) {
        assembly {
            value := shr(216, calldataload(self))
            self := add(self, 5)
        }
        return (self, value);
    }

    function readU64(CalldataReader self) internal pure returns (CalldataReader, uint64 value) {
        assembly {
            value := shr(192, calldataload(self))
            self := add(self, 8)
        }
        return (self, value);
    }

    function readU128(CalldataReader self) internal pure returns (CalldataReader, uint128 value) {
        assembly {
            value := shr(128, calldataload(self))
            self := add(self, 16)
        }
        return (self, value);
    }

    function readAddr(CalldataReader self) internal pure returns (CalldataReader, address addr) {
        assembly {
            addr := shr(96, calldataload(self))
            self := add(self, 20)
        }
        return (self, addr);
    }

    function readU256(CalldataReader self) internal pure returns (CalldataReader, uint256 value) {
        assembly {
            value := calldataload(self)
            self := add(self, 32)
        }
        return (self, value);
    }

    function readVariant(CalldataReader self) internal pure returns (CalldataReader, OrderVariantMap variant) {
        assembly {
            variant := shr(248, calldataload(self))
            self := add(self, 1)
        }
        return (self, variant);
    }

    function readU16End(CalldataReader self) internal pure returns (CalldataReader, CalldataReader end) {
        uint256 length;
        (self, length) = self.readU16();
        end = CalldataReader.wrap(self.offset() + length);
        return (self, end);
    }

    function readU24End(CalldataReader self) internal pure returns (CalldataReader, CalldataReader end) {
        uint256 length;
        (self, length) = self.readU24();
        end = CalldataReader.wrap(self.offset() + length);
        return (self, end);
    }

    function readBytes(CalldataReader self) internal pure returns (CalldataReader, bytes calldata slice) {
        assembly ("memory-safe") {
            slice.length := shr(232, calldataload(self))
            self := add(self, 3)
            slice.offset := self
            self := add(self, slice.length)
        }
        return (self, slice);
    }

    function logNext(CalldataReader self, uint256 n) internal pure {
        bytes memory b;
        assembly ("memory-safe") {
            b := mload(0x40)
            mstore(b, n)
            let dataOffset := add(b, 0x20)
            mstore(0x40, add(dataOffset, n))
            calldatacopy(dataOffset, self, n)
        }
        console.logBytes(b);
    }
}
