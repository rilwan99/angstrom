# Angstrom

**Corresponding Source file:** [`Angstrom.sol`](../../contracts/src/Angstrom.sol)

## Overview

The `Angstrom` contract is the core entry point for nodes submitting bundles of orders. The contract
is in charge of:
- validating orders (signatures, nonces, limits)
- settling orders
- enforcing key protocol invariants
    - uniform clearing (all orders for the same pair get the same price)
    - only active nodes are allowed to submit bundles
    - there may only be 1 bundle / block
- interacting with Uniswap V4
    - swapping against Angstrom pools
    - transferring tokens between Uniswap and itself
    - flash borrowing from Uniswap to ensure orders can be filled

Note that the protocol only enforces *some* protocol invariants and leaves others to the economic
security of the nodes to save on gas e.g. checking that prices are consistent across alternative
pairs.

The main entry point for submitting a bundle to the `Angstrom` contract is the `execute(bytes)`
method which takes the encoded bundle.

## Bundle specification & `execute(bytes)`

**Entry-point Control Flow**

When `execute` is called all it does is authenticate that the caller is a node before calling into Uniswap's
`unlock` function which in turn calls **back into** the `Angstrom` contract passing it the original
payload as `data` (this back and forth is required because it establishes a call context from which
we're allowed to actually interact with Uniswap at all).

### Bundle contents

#### Top-level Payload Contents

The `data` payload consists of an ♻️ ABI-encoded (encoding soon to change) tuple of structs:

```solidity
(
    Asset[] assets,
    Price[] initialPrices,
    TopOfBlockOrderEnvelope[] topOfBlockOrders,
    PoolSwap[] swaps,
    GenericOrder[] orders,
    PoolRewardsUpdate[] poolRewardsUpdates
);
```

#### `Asset[] assets`

```solidity
struct Asset {
    address addr;
    uint256 borrow;
    uint256 save;
    uint256 settle;
}
```

The array of assets specifies what assets are going to be traded or settled in the submitted bundle.
The elements **must be** sorted in ascending order according the value of `.addr`.

|Field|Description|Unit|
|-----|-----------|----|
|`address addr`|Contract address of ERC20 token of the asset, must be ordered relative to neighbors.|N/A|
|`uint256 borrow`|Amount of the asset to flash borrow.| `.addr` base unit|
|`uint256 save`|Amount of the asset to save as the network fee |`.addr` base unit|
|`uint256 settle`|Final amount that needs to be repayed to Uniswap post-bundle execution .|`.addr` base unit|


#### `Price[] initialPrices`

```solidity
struct Price {
    AssetIndex outIndex;
    AssetIndex inIndex;
    uint256 price;
}
```

This list defines the uniform clearing prices for the different pairs in the bundle, if an order
references a missing pair the bundle will revert.

|Field|Description|Unit|
|-----|-----------|----|
|`AssetIndex outIndex`|Index of price out asset|max. 16-bit index into the `asset` list|
|`AssetIndex inIndex`|Index of price in asset|max. 16-bit index into the `asset` list|
|`uint256 price`|Price of pair|Price of $\frac{\text{out}}{\text{in}}$ asset scaled by $10^{27}$ (aka "RAY")|

