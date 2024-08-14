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

The bundle is a struct encoded using the [PADE format](./Encoding.md). The string of bytes is the
value that the `execute` method is called with (not that the call itself is still ABI encoded, just
the contents of the single `bytes` parameter itself is PADE).

#### Top-level Payload Contents

The `data` payload is the PADE encoding of the following struct:

```rust
struct Bundle {
    assets: Sequence<2, Asset>,
    pairs: Sequence<2, Pair>,
    swaps: Sequence<2, PoolSwap>,
    toBOrders: Sequence<3, TopOfBlockOrder>,
    userOrders: Sequence<3, UserOrder>,
    poolRewardsUpdates: Sequence<3, PoolRewardsUpdate>
}
```

#### `Asset`

Solidity: [decoding implementation (`src/types/Asset.sol`)](../../contracts/src/types/Asset.sol) | [reference
encoding (`src/reference/Asset.sol`)](../../contracts/src/reference/Asset.sol)

```rust
struct Asset {
    addr: address,
    borrow: u128,
    save: u128,
    settle: u128
}
```

The array of assets specifies what assets are going to be traded or settled in the submitted bundle.
The elements **must be** sorted in ascending order according the value of `.addr`.

|Field|Description|
|-----|-----------|
|`addr: address`|Contract address of ERC20 token of the asset. |
|`borrow: uint128`|Amount of the asset to flash borrow. (`.addr` base unit)|
|`save: uint128`|Amount of the asset to save as the network fee (`.addr` base unit)|
|`settle: uint128`|Final amount to be repayed to Uniswap post-bundle execution. (`.addr` base unit)|

#### `Pair`

Solidity: [decoding implementation (`src/types/Pair.sol`)](../../contracts/src/types/Pair.sol) | [reference
encoding (`src/reference/Pair.sol`)](../../contracts/src/reference/Pair.sol)

```rust
struct Pair {
    asset_indices: AssetIndexPair,
    price_ray_assetAOverB: u256
}

```

This list of pairs defines the unique uniform clearing prices for the different pairs in the bundle.
The elements **must be** sorted in ascending order according the value of `.asset_indices`.

|Field|Description|
|-----|-----------|
|`assets: AssetIndexPair`|Pair's assets as indices into the asset array. `asset_indices.index_a() < asset_indices.index_b()` **must** hold.|
|`price_AOverB: u256`|Unform clearing price of pair in asset A **over** asset B base units in Ray e.g. `13.2e27` represents 13.2 base units of A for every base unit of A.|



#### `PoolSwap`

Solidity: [decoding implementation (`src/types/PoolSwap.sol`)](../../contracts/src/types/PoolSwap.sol) | [reference
encoding (`src/reference/PoolSwap.sol`)](../../contracts/src/reference/PoolSwap.sol)


```rust
struct PoolSwap {
    assets_in_out: AssetIndexPair,
    quantity_in: u128
}
```

The array of `PoolSwap`s represents the netted out swaps to execute against the underlying Uniswap
pool (where Angstrom is its hook). The contract does not enforce swap uniqueness, it is recommended
to net out multiple swaps against the same pool into one to save on gas.

|Field|Description|
|-----|-----------|
|`assets: AssetIndexPair`|Swap's input & output assets as indices into the asset array. Asset A is the input asset, asset B the output asset.|
|`quantity_in: u128`|The swap input quantity in the input asset's base units.|

#### 🚧 `TopOfBlockOrder`

Solidity: [decoding implementation (`src/Angstrom.sol:Angstrom._validateAndExecuteToB`)](../../contracts/src/Angstrom.sol) | [reference encoding (`src/reference/OrderTypes.sol`)](../../contracts/src/reference/OrderTypes.sol).

```rust
struct TopOfBlockOrder {
    use_internal: bool,
    quantity_in: u128,
    quantity_out: u128,
    assets_in_out: AssetIndexPair,
    recipient: Recipient,
    hookData: Option<Sequence<3, bytes1>>,
    signature: Signature
}
```

|Field|Description|
|-----|-----------|
|`use_internal: bool`|Whether to use angstrom internal balance (`true`) or actual ERC20 balance (`false`) to settle|
|`quantity_in: u128`|The order offered input quanity in the input asset's base units.|
|`quantity_out: u128`|The order expected output quantity in the output asset's base units.|
|`assets_in_out: AssetIndexPair`|Swap's input & output assets as indices into the asset array. Asset A is the input asset, asset B the output asset.|

#### `PoolRewardsUpdate`


```rust
struct PoolRewardsUpdate {
    assets: AssetIndexPair,
    update: RewardsUpdate
}
```

The `PoolRewardsUpdate` struct and its parameters instruct the contract how to account and
distribute rewards to different ticks. Similar to Uniswap pool rewards are accounted by tick by
tracking the "cumulative growth outside".

|Field|Description|
|-----|-----------|
|`assets: AssetIndexPair`|Asset 0 & 1 of the pool to reward as indices into the assets array.|
|`update: RewardsUpdate`|The reward update data for the select pool (more below).|

**Rewards Update**

```rust

struct RewardsUpdate {
    start_tick: i24,
    start_liquidity: u128,
    quantities: Sequence<2, u128>
}
```

To gain a better intuition over how parameters need to be set it's good to have an understanding how
the meaning of a tick's `reward_growth_outside` changes _relative to the current tick_:

![](./assets/fee-growth-outside-meaning.png)

For ticks that are **below or at** the current tick their `reward_growth_outside` value represents the sum
of all accrued rewards in the ticks below **excluding the tick's own rewards**.

Conversely for ticks that are above the current tick their `reward_growth_outside` value represents
the sum of all accrued rewards in the ticks above **including its own**.

Generally the logic for updating pool rewards looks as follows:

```python
def update_rewards(
    pool: Pool,
    start_tick: Tick,
    reward_to_current: int,
    quantities: list[int],
    liquidity: int,
    below: bool
):
    cumulative_reward_growth: float = 0

    end_tick: Tick = get_current_tick()
    ticks: list[Tick] = initialized_tick_range(start_tick, end_tick, include_end=below)
    # Missing quantities interpreted as zeros
    padded_quantities: list[int] = quantities + [0] * max(0, len(ticks) - len(quantities))

    for tick, quantity in zip(ticks, padded_quantities):
        cumulative_reward_growth += quantity / liquidity
        tick.reward_growth_outside += cumulative_reward_growth

        if below:
            liquidity += tick.net_liquidity
        else:
            liquidity -= tick.net_liquidity

    # Last quantity assumed to be reward for current tick.
    if len(quantities) > len(ticks):
        current_tick_reward: int = quantities[len(ticks)]
        cumulative_reward_growth += current_tick_reward / liquidity

    pool.global_reward_growth += sum(quantities)

    assert len(quantities) <= len(ticks) + 1, 'Unused quantities'
    assert liquidity == get_current_liquidity(), 'Invalid set start liquidity'
```

|Field|Description |
|-----|-----------|
|`start_tick: i24`| When rewarding below the current tick: the first tick **above** the first tick to donate to. <br> When rewarding above: just the first tick actually being donated to. |
|`start_liquidity: u128`|The current liquidity if `start_tick` were the current tick.|
|`quantities: Sequence<2, u128>`|The reward for each initialized tick. Zeros at the end of the array can be left out. To donate to the current tick append a quantity.|
