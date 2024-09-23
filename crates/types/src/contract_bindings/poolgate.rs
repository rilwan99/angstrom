///Module containing a contract's types and functions.
/**

```solidity
library IPoolManager {
    struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
}
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod IPoolManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ModifyLiquidityParams {
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        pub liquidityDelta: alloy::sol_types::private::primitives::aliases::I256,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I256,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ModifyLiquidityParams> for UnderlyingRustTuple<'_> {
            fn from(value: ModifyLiquidityParams) -> Self {
                (value.tickLower, value.tickUpper, value.liquidityDelta, value.salt)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ModifyLiquidityParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tickLower: tuple.0,
                    tickUpper: tuple.1,
                    liquidityDelta: tuple.2,
                    salt: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ModifyLiquidityParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ModifyLiquidityParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityDelta),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ModifyLiquidityParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ModifyLiquidityParams {
            const NAME: &'static str = "ModifyLiquidityParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ModifyLiquidityParams(int24 tickLower,int24 tickUpper,int256 liquidityDelta,bytes32 salt)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickLower)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickUpper)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidityDelta,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ModifyLiquidityParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickLower,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickUpper,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityDelta,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickLower,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickUpper,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityDelta,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IPoolManager`](self) contract instance.

See the [wrapper's documentation](`IPoolManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IPoolManagerInstance<T, P, N> {
        IPoolManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IPoolManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IPoolManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPoolManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IPoolManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPoolManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IPoolManager`](self) contract instance.

See the [wrapper's documentation](`IPoolManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IPoolManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPoolManagerInstance<T, P, N> {
            IPoolManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IPoolManager {
    struct ModifyLiquidityParams {
        int24 tickLower;
        int24 tickUpper;
        int256 liquidityDelta;
        bytes32 salt;
    }
}

interface PoolGate {
    type BalanceDelta is int256;

    error Overflow();

    constructor(address uniV4);

    function __addLiquidity(address asset0, address asset1, address sender, IPoolManager.ModifyLiquidityParams memory params) external returns (BalanceDelta callerDelta);
    function __initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96) external returns (int24 tick);
    function __mint(address to, address asset, uint256 amount) external;
    function __removeLiquidity(address asset0, address asset1, address sender, IPoolManager.ModifyLiquidityParams memory params) external returns (BalanceDelta delta);
    function __swap(address assetIn, address assetOut, int256 amountSpecified, uint160 sqrtPriceLimitX96) external returns (BalanceDelta swapDelta);
    function addLiquidity(address asset0, address asset1, int24 tickLower, int24 tickUpper, uint256 liquidity, bytes32 salt) external returns (uint256 amount0, uint256 amount1);
    function hook() external view returns (address);
    function initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96) external returns (int24 tick);
    function isInitialized(address asset0, address asset1) external view returns (bool);
    function mint(address asset, uint256 amount) external;
    function mint(address to, address asset, uint256 amount) external;
    function removeLiquidity(address asset0, address asset1, int24 tickLower, int24 tickUpper, uint256 liquidity, bytes32 salt) external returns (uint256 amount0, uint256 amount1);
    function setHook(address hook_) external;
    function swap(address assetIn, address assetOut, int256 amountSpecified, uint160 sqrtPriceLimitX96) external returns (BalanceDelta delta);
    function tickSpacing(int24 spacing) external;
    function unlockCallback(bytes memory data) external returns (bytes memory);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "uniV4",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__addLiquidity",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.ModifyLiquidityParams",
        "components": [
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "liquidityDelta",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "callerDelta",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__initializePool",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialSqrtPriceX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [
      {
        "name": "tick",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__mint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__removeLiquidity",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.ModifyLiquidityParams",
        "components": [
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "liquidityDelta",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "delta",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "__swap",
    "inputs": [
      {
        "name": "assetIn",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "assetOut",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountSpecified",
        "type": "int256",
        "internalType": "int256"
      },
      {
        "name": "sqrtPriceLimitX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [
      {
        "name": "swapDelta",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addLiquidity",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tickLower",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "hook",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initializePool",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "initialSqrtPriceX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [
      {
        "name": "tick",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isInitialized",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "mint",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeLiquidity",
    "inputs": [
      {
        "name": "asset0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "asset1",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "tickLower",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "liquidity",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setHook",
    "inputs": [
      {
        "name": "hook_",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "swap",
    "inputs": [
      {
        "name": "assetIn",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "assetOut",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountSpecified",
        "type": "int256",
        "internalType": "int256"
      },
      {
        "name": "sqrtPriceLimitX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [
      {
        "name": "delta",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "tickSpacing",
    "inputs": [
      {
        "name": "spacing",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unlockCallback",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "error",
    "name": "Overflow",
    "inputs": []
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod PoolGate {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040526008805462ffffff60a01b1916600f60a21b1790553480156023575f80fd5b506040516129ba3803806129ba8339810160408190526040916050565b6001600160a01b0316608052607b565b5f60208284031215605f575f80fd5b81516001600160a01b03811681146074575f80fd5b9392505050565b6080516128b76101035f395f81816104a30152818161080901528181610a5b01528181610bab01528181610c5d01528181610ede01528181610fe7015281816111200152818161128b015281816114d6015281816116f4015281816117e40152818161199401528181611d8801528181611e9601528181611efe0152611f9701526128b75ff3fe608060405234801561000f575f80fd5b50600436106100fb575f3560e01c80636e1f5b99116100935780639f5e1a73116100635780639f5e1a73146102fa578063c6c3bbe61461030d578063cf618a5514610320578063e4cb970b14610333575f80fd5b80636e1f5b99146102265780637f5a7c7b146102395780638a4c6af61461027e57806391dd7346146102da575f80fd5b80633dfd3873116100ce5780633dfd38731461018357806340c10f19146101da57806347c7c5a9146101ed57806347ddb81f14610213575f80fd5b806312b4f4e6146100ff5780632974c8a4146101255780632bdfdbd11461014d57806330315f6214610160575b5f80fd5b61011261010d36600461213a565b610346565b6040519081526020015b60405180910390f35b6101386101333660046121c6565b6106f4565b6040805192835260208301919091520161011c565b61011261015b36600461213a565b610901565b61017361016e366004612230565b610e35565b604051901515815260200161011c565b6101d8610191366004612267565b600880547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b005b6101d86101e8366004612289565b610f25565b6102006101fb3660046122b3565b610f34565b60405160029190910b815260200161011c565b6102006102213660046122b3565b611067565b6101126102343660046122fb565b6111cb565b6008546102599073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161011c565b6101d861028c36600461234b565b6008805462ffffff90921674010000000000000000000000000000000000000000027fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff909216919091179055565b6102ed6102e8366004612366565b611340565b60405161011c9190612420565b6101386103083660046121c6565b6113c3565b6101d861031b366004612432565b611643565b6101d861032e366004612432565b61178f565b6101126103413660046122fb565b611849565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b9483019490945292909316908301529081906040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff86166004820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b15801561044f575f80fd5b505af1158015610461573d5f803e3d5ffd5b50506040517f5a6bcfda0000000000000000000000000000000000000000000000000000000081525f925073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169150635a6bcfda906104db9085908890600401612470565b60408051808303815f875af11580156104f6573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061051a9190612549565b90935090506105298160801d90565b600f0b158015610543575061053e81600f0b90565b600f0b155b6105ae576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600d60248201527f47657474696e6720666565733f0000000000000000000000000000000000000060448201526064015b60405180910390fd5b5f6105b98460801d90565b600f0b131580156105d657505f6105d084600f0b90565b600f0b13155b610662576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f67657474696e6720746f6b656e7320666f7220616464696e67206c697175696460448201527f697479000000000000000000000000000000000000000000000000000000000060648201526084016105a5565b61066d878785611a93565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156106d4575f80fd5b505af11580156106e6573d5f803e3d5ffd5b505050505050949350505050565b5f805f60405180608001604052808860020b81526020018760020b815260200161071d87611ab4565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f2bdfdbd10000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293505f927f0000000000000000000000000000000000000000000000000000000000000000909216916348c894919161083e91600401612420565b5f604051808303815f875af1158015610859573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261089e9190810190612598565b90505f818060200190518101906108b59190612688565b90506108c18160801d90565b6fffffffffffffffffffffffffffffffff1694506108df81600f0b90565b6fffffffffffffffffffffffffffffffff169350505050965096945050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b9483019490945292909316908301529081906040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff86166004820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b158015610a0a575f80fd5b505af1158015610a1c573d5f803e3d5ffd5b50506040517f5a6bcfda00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169250635a6bcfda9150610a949084908790600401612470565b60408051808303815f875af1158015610aaf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad39190612549565b506040805173ffffffffffffffffffffffffffffffffffffffff87811660208084018290528b8316848601528451808503860181526060850190955284519401939093206080830193909352881660a0820152919350905f9060c001604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529082905280516020909101207ff135baaa0000000000000000000000000000000000000000000000000000000082526004820184905291505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610bf0573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c149190612688565b6040517ff135baaa000000000000000000000000000000000000000000000000000000008152600481018490529091505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610ca2573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cc69190612688565b9050610cea866fffffffffffffffffffffffffffffffff8316608085901b17611b15565b95505f610cf78760801d90565b600f0b12158015610d1457505f610d0e87600f0b90565b600f0b12155b610da0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f6c6f73696e67206d6f6e657920666f722072656d6f76696e67206c697175696460448201527f697479000000000000000000000000000000000000000000000000000000000060648201526084016105a5565b610dab8a8a88611a93565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610e12575f80fd5b505af1158015610e24573d5f803e3d5ffd5b505050505050505050949350505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a811682528981169382019390935295860183905274010000000000000000000000000000000000000000870460020b908601529094169183019190915281208290610f049073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690611b5b565b73ffffffffffffffffffffffffffffffffffffffff16151595945050505050565b610f30338383611643565b5050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8a811685528981169285019290925283860183905274010000000000000000000000000000000000000000870460020b908401529485169282019290925291517f695c5bf500000000000000000000000000000000000000000000000000000000815290927f0000000000000000000000000000000000000000000000000000000000000000169063695c5bf59061101e908490879060040161269f565b6020604051808303815f875af115801561103a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061105e9190612753565b95945050505050565b6040805173ffffffffffffffffffffffffffffffffffffffff8581166024830152848116604483015283811660648084019190915283518084039091018152608490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f47c7c5a90000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081525f9283927f0000000000000000000000000000000000000000000000000000000000000000909116916348c894919161115591600401612420565b5f604051808303815f875af1158015611170573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526111b59190810190612598565b90508080602001905181019061105e9190612753565b6040805173ffffffffffffffffffffffffffffffffffffffff86811660248301528581166044830152606482018590528381166084808401919091528351808403909101815260a490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe4cb970b0000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081525f9283927f0000000000000000000000000000000000000000000000000000000000000000909116916348c89491916112c091600401612420565b5f604051808303815f875af11580156112db573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526113209190810190612598565b9050808060200190518101906113369190612688565b9695505050505050565b60605f803073ffffffffffffffffffffffffffffffffffffffff16858560405161136b92919061276e565b5f604051808303815f865af19150503d805f81146113a4576040519150601f19603f3d011682016040523d82523d5f602084013e6113a9565b606091505b5091509150816113bb57805160208201fd5b949350505050565b5f805f60405180608001604052808860020b81526020018760020b81526020016113ec87611bfa565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f12b4f4e60000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293507f0000000000000000000000000000000000000000000000000000000000000000909116916348c894919161150b91600401612420565b5f604051808303815f875af192505050801561156657506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526115639190810190612598565b60015b6115e6573d808015611593576040519150601f19603f3d011682016040523d82523d5f602084013e611598565b606091505b506115d76040518060400160405280600f81526020017f7374756666206661696c65643f3f3f0000000000000000000000000000000000815250611c59565b6115e081611ceb565b50611637565b5f818060200190518101906115fb9190612688565b90506116078160801d90565b6116109061277d565b6fffffffffffffffffffffffffffffffff16945061162e81600f0b90565b6108df9061277d565b50965096945050505050565b6040805173ffffffffffffffffffffffffffffffffffffffff85811660248301528481166044830152606480830185905283518084039091018152608490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fcf618a550000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081527f0000000000000000000000000000000000000000000000000000000000000000909216916348c894919161172991600401612420565b5f604051808303815f875af1158015611744573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526117899190810190612598565b50505050565b6040517f156e29f600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8481166004830152602482018490526044820183905283917f00000000000000000000000000000000000000000000000000000000000000009091169063156e29f6906064015f604051808303815f87803b158015611827575f80fd5b505af1158015611839573d5f803e3d5ffd5b5050505061178983836001611d7a565b5f73ffffffffffffffffffffffffffffffffffffffff808516908616108181611900576008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8d811685528e8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b938201939093529190921691810191909152611990565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8e811685528d8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b9382019390935291909216918101919091525b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c82604051806060016040528086151581526020018981526020018873ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b8152600401611a1f9291906127de565b6020604051808303815f875af1158015611a3b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a5f9190612688565b9250611a77815f0151611a728560801d90565b611f8a565b611a898160200151611a7285600f0b90565b5050949350505050565b611aa183611a728360801d90565b611aaf82611a7283600f0b90565b505050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611b0f576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b5f608082811d9084901d01600f83810b9085900b0161105e611b36836120ae565b611b3f836120ae565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa158015611bd6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113bb9190612688565b5f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821115611c55576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5090565b611ce881604051602401611c6d9190612420565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac000000000000000000000000000000000000000000000000000000001790526120e8565b50565b611ce881604051602401611cff9190612420565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0be77f56000000000000000000000000000000000000000000000000000000001790526120e8565b8115611aaf578015611e59577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611ddf8573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611e42575f80fd5b505af1158015611e54573d5f803e3d5ffd5b505050505b6040517f40c10f1900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081166004830152602482018490528416906340c10f19906044015f604051808303815f87803b158015611ee6575f80fd5b505af1158015611ef8573d5f803e3d5ffd5b505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611f66573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117899190612688565b5f81600f0b1315612083577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166380f0b44c611fee8473ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526fffffffffffffffffffffffffffffffff841660248201526044015f604051808303815f87803b158015612069575f80fd5b505af115801561207b573d5f803e3d5ffd5b505050505050565b5f81600f0b1215610f3057610f3082825f036fffffffffffffffffffffffffffffffff166001611d7a565b80600f81900b81146120e3576120e37f93dafdf1000000000000000000000000000000000000000000000000000000006120f1565b919050565b611ce8816120f9565b805f5260045ffd5b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b73ffffffffffffffffffffffffffffffffffffffff81168114611ce8575f80fd5b5f805f8084860360e081121561214e575f80fd5b853561215981612119565b9450602086013561216981612119565b9350604086013561217981612119565b925060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0820112156121aa575f80fd5b509295919450926060019150565b8060020b8114611ce8575f80fd5b5f805f805f8060c087890312156121db575f80fd5b86356121e681612119565b955060208701356121f681612119565b94506040870135612206816121b8565b93506060870135612216816121b8565b9598949750929560808101359460a0909101359350915050565b5f8060408385031215612241575f80fd5b823561224c81612119565b9150602083013561225c81612119565b809150509250929050565b5f60208284031215612277575f80fd5b813561228281612119565b9392505050565b5f806040838503121561229a575f80fd5b82356122a581612119565b946020939093013593505050565b5f805f606084860312156122c5575f80fd5b83356122d081612119565b925060208401356122e081612119565b915060408401356122f081612119565b809150509250925092565b5f805f806080858703121561230e575f80fd5b843561231981612119565b9350602085013561232981612119565b925060408501359150606085013561234081612119565b939692955090935050565b5f6020828403121561235b575f80fd5b8135612282816121b8565b5f8060208385031215612377575f80fd5b823567ffffffffffffffff81111561238d575f80fd5b8301601f8101851361239d575f80fd5b803567ffffffffffffffff8111156123b3575f80fd5b8560208284010111156123c4575f80fd5b6020919091019590945092505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f61228260208301846123d4565b5f805f60608486031215612444575f80fd5b833561244f81612119565b9250602084013561245f81612119565b929592945050506040919091013590565b6124ef818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b5f82356124fb816121b8565b60020b60a08301526020830135612511816121b8565b60020b60c083015250604082013560e082015260609091013561010082015261014061012082018190525f9082015261016001919050565b5f806040838503121561255a575f80fd5b505080516020909101519092909150565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f602082840312156125a8575f80fd5b815167ffffffffffffffff8111156125be575f80fd5b8201601f810184136125ce575f80fd5b805167ffffffffffffffff8111156125e8576125e861256b565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156126545761265461256b565b60405281815282820160200186101561266b575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215612698575f80fd5b5051919050565b61271e818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff9190911660a082015260e060c082018190525f9082015261010001919050565b5f60208284031215612763575f80fd5b8151612282816121b8565b818382375f9101908152919050565b5f81600f0b7fffffffffffffffffffffffffffffffff8000000000000000000000000000000081036127d6577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f0392915050565b61285d818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8151151560a0820152602082015160c082015260409091015173ffffffffffffffffffffffffffffffffffffffff1660e082015261012061010082018190525f908201526101400191905056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R`\x08\x80Tb\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x0F`\xA2\x1B\x17\x90U4\x80\x15`#W_\x80\xFD[P`@Qa)\xBA8\x03\x80a)\xBA\x839\x81\x01`@\x81\x90R`@\x91`PV[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`{V[_` \x82\x84\x03\x12\x15`_W_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`tW_\x80\xFD[\x93\x92PPPV[`\x80Qa(\xB7a\x01\x03_9_\x81\x81a\x04\xA3\x01R\x81\x81a\x08\t\x01R\x81\x81a\n[\x01R\x81\x81a\x0B\xAB\x01R\x81\x81a\x0C]\x01R\x81\x81a\x0E\xDE\x01R\x81\x81a\x0F\xE7\x01R\x81\x81a\x11 \x01R\x81\x81a\x12\x8B\x01R\x81\x81a\x14\xD6\x01R\x81\x81a\x16\xF4\x01R\x81\x81a\x17\xE4\x01R\x81\x81a\x19\x94\x01R\x81\x81a\x1D\x88\x01R\x81\x81a\x1E\x96\x01R\x81\x81a\x1E\xFE\x01Ra\x1F\x97\x01Ra(\xB7_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cn\x1F[\x99\x11a\0\x93W\x80c\x9F^\x1As\x11a\0cW\x80c\x9F^\x1As\x14a\x02\xFAW\x80c\xC6\xC3\xBB\xE6\x14a\x03\rW\x80c\xCFa\x8AU\x14a\x03 W\x80c\xE4\xCB\x97\x0B\x14a\x033W_\x80\xFD[\x80cn\x1F[\x99\x14a\x02&W\x80c\x7FZ|{\x14a\x029W\x80c\x8ALj\xF6\x14a\x02~W\x80c\x91\xDDsF\x14a\x02\xDAW_\x80\xFD[\x80c=\xFD8s\x11a\0\xCEW\x80c=\xFD8s\x14a\x01\x83W\x80c@\xC1\x0F\x19\x14a\x01\xDAW\x80cG\xC7\xC5\xA9\x14a\x01\xEDW\x80cG\xDD\xB8\x1F\x14a\x02\x13W_\x80\xFD[\x80c\x12\xB4\xF4\xE6\x14a\0\xFFW\x80c)t\xC8\xA4\x14a\x01%W\x80c+\xDF\xDB\xD1\x14a\x01MW\x80c01_b\x14a\x01`W[_\x80\xFD[a\x01\x12a\x01\r6`\x04a!:V[a\x03FV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x0136`\x04a!\xC6V[a\x06\xF4V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1CV[a\x01\x12a\x01[6`\x04a!:V[a\t\x01V[a\x01sa\x01n6`\x04a\"0V[a\x0E5V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1CV[a\x01\xD8a\x01\x916`\x04a\"gV[`\x08\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[a\x01\xD8a\x01\xE86`\x04a\"\x89V[a\x0F%V[a\x02\0a\x01\xFB6`\x04a\"\xB3V[a\x0F4V[`@Q`\x02\x91\x90\x91\x0B\x81R` \x01a\x01\x1CV[a\x02\0a\x02!6`\x04a\"\xB3V[a\x10gV[a\x01\x12a\x0246`\x04a\"\xFBV[a\x11\xCBV[`\x08Ta\x02Y\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x01\xD8a\x02\x8C6`\x04a#KV[`\x08\x80Tb\xFF\xFF\xFF\x90\x92\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xEDa\x02\xE86`\x04a#fV[a\x13@V[`@Qa\x01\x1C\x91\x90a$ V[a\x018a\x03\x086`\x04a!\xC6V[a\x13\xC3V[a\x01\xD8a\x03\x1B6`\x04a$2V[a\x16CV[a\x01\xD8a\x03.6`\x04a$2V[a\x17\x8FV[a\x01\x12a\x03A6`\x04a\"\xFBV[a\x18IV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90\x81\x90`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04OW_\x80\xFD[PZ\xF1\x15\x80\x15a\x04aW=_\x80>=_\xFD[PP`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91PcZk\xCF\xDA\x90a\x04\xDB\x90\x85\x90\x88\x90`\x04\x01a$pV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1A\x91\x90a%IV[\x90\x93P\x90Pa\x05)\x81`\x80\x1D\x90V[`\x0F\x0B\x15\x80\x15a\x05CWPa\x05>\x81`\x0F\x0B\x90V[`\x0F\x0B\x15[a\x05\xAEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FGetting fees?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x05\xB9\x84`\x80\x1D\x90V[`\x0F\x0B\x13\x15\x80\x15a\x05\xD6WP_a\x05\xD0\x84`\x0F\x0B\x90V[`\x0F\x0B\x13\x15[a\x06bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fgetting tokens for adding liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xA5V[a\x06m\x87\x87\x85a\x1A\x93V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xD4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE6W=_\x80>=_\xFD[PPPPPP\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x07\x1D\x87a\x1A\xB4V[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F+\xDF\xDB\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x08>\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08YW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\x9E\x91\x90\x81\x01\x90a%\x98V[\x90P_\x81\x80` \x01\x90Q\x81\x01\x90a\x08\xB5\x91\x90a&\x88V[\x90Pa\x08\xC1\x81`\x80\x1D\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\x08\xDF\x81`\x0F\x0B\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93PPPP\x96P\x96\x94PPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90\x81\x90`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\nW_\x80\xFD[PZ\xF1\x15\x80\x15a\n\x1CW=_\x80>=_\xFD[PP`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92PcZk\xCF\xDA\x91Pa\n\x94\x90\x84\x90\x87\x90`\x04\x01a$pV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD3\x91\x90a%IV[P`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16` \x80\x84\x01\x82\x90R\x8B\x83\x16\x84\x86\x01R\x84Q\x80\x85\x03\x86\x01\x81R``\x85\x01\x90\x95R\x84Q\x94\x01\x93\x90\x93 `\x80\x83\x01\x93\x90\x93R\x88\x16`\xA0\x82\x01R\x91\x93P\x90_\x90`\xC0\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01\x84\x90R\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x14\x91\x90a&\x88V[`@Q\x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC6\x91\x90a&\x88V[\x90Pa\x0C\xEA\x86o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x85\x90\x1B\x17a\x1B\x15V[\x95P_a\x0C\xF7\x87`\x80\x1D\x90V[`\x0F\x0B\x12\x15\x80\x15a\r\x14WP_a\r\x0E\x87`\x0F\x0B\x90V[`\x0F\x0B\x12\x15[a\r\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Flosing money for removing liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xA5V[a\r\xAB\x8A\x8A\x88a\x1A\x93V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x12W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0E$W=_\x80>=_\xFD[PPPPPPPPP\x94\x93PPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x81\x16\x93\x82\x01\x93\x90\x93R\x95\x86\x01\x83\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x04`\x02\x0B\x90\x86\x01R\x90\x94\x16\x91\x83\x01\x91\x90\x91R\x81 \x82\x90a\x0F\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x1B[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x95\x94PPPPPV[a\x0F03\x83\x83a\x16CV[PPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x85R\x89\x81\x16\x92\x85\x01\x92\x90\x92R\x83\x86\x01\x83\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x04`\x02\x0B\x90\x84\x01R\x94\x85\x16\x92\x82\x01\x92\x90\x92R\x91Q\x7Fi\\[\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\\[\xF5\x90a\x10\x1E\x90\x84\x90\x87\x90`\x04\x01a&\x9FV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10:W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10^\x91\x90a'SV[\x95\x94PPPPPV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R\x83\x81\x16`d\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FG\xC7\xC5\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cH\xC8\x94\x91\x91a\x11U\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11pW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x11\xB5\x91\x90\x81\x01\x90a%\x98V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x10^\x91\x90a'SV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R`d\x82\x01\x85\x90R\x83\x81\x16`\x84\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA4\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE4\xCB\x97\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cH\xC8\x94\x91\x91a\x12\xC0\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xDBW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x13 \x91\x90\x81\x01\x90a%\x98V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x136\x91\x90a&\x88V[\x96\x95PPPPPPV[``_\x800s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85`@Qa\x13k\x92\x91\x90a'nV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x13\xA4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13\xA9V[``\x91P[P\x91P\x91P\x81a\x13\xBBW\x80Q` \x82\x01\xFD[\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x13\xEC\x87a\x1B\xFAV[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x12\xB4\xF4\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cH\xC8\x94\x91\x91a\x15\x0B\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x15fWP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x15c\x91\x90\x81\x01\x90a%\x98V[`\x01[a\x15\xE6W=\x80\x80\x15a\x15\x93W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x15\x98V[``\x91P[Pa\x15\xD7`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7Fstuff failed???\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1CYV[a\x15\xE0\x81a\x1C\xEBV[Pa\x167V[_\x81\x80` \x01\x90Q\x81\x01\x90a\x15\xFB\x91\x90a&\x88V[\x90Pa\x16\x07\x81`\x80\x1D\x90V[a\x16\x10\x90a'}V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\x16.\x81`\x0F\x0B\x90V[a\x08\xDF\x90a'}V[P\x96P\x96\x94PPPPPV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCFa\x8AU\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x17)\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17DW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x17\x89\x91\x90\x81\x01\x90a%\x98V[PPPPV[`@Q\x7F\x15n)\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`D\x82\x01\x83\x90R\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x15n)\xF6\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18'W_\x80\xFD[PZ\xF1\x15\x80\x15a\x189W=_\x80>=_\xFD[PPPPa\x17\x89\x83\x83`\x01a\x1DzV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x90\x86\x16\x10\x81\x81a\x19\0W`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16\x85R\x8E\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91Ra\x19\x90V[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8E\x81\x16\x85R\x8D\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91R[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x82`@Q\x80``\x01`@R\x80\x86\x15\x15\x81R` \x01\x89\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x1F\x92\x91\x90a'\xDEV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A;W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A_\x91\x90a&\x88V[\x92Pa\x1Aw\x81_\x01Qa\x1Ar\x85`\x80\x1D\x90V[a\x1F\x8AV[a\x1A\x89\x81` \x01Qa\x1Ar\x85`\x0F\x0B\x90V[PP\x94\x93PPPPV[a\x1A\xA1\x83a\x1Ar\x83`\x80\x1D\x90V[a\x1A\xAF\x82a\x1Ar\x83`\x0F\x0B\x90V[PPPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x1B\x0FW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a\x10^a\x1B6\x83a \xAEV[a\x1B?\x83a \xAEV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xD6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBB\x91\x90a&\x88V[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1CUW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[a\x1C\xE8\x81`@Q`$\x01a\x1Cm\x91\x90a$ V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra \xE8V[PV[a\x1C\xE8\x81`@Q`$\x01a\x1C\xFF\x91\x90a$ V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0B\xE7\x7FV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra \xE8V[\x81\x15a\x1A\xAFW\x80\x15a\x1EYW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1D\xDF\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1EBW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1ETW=_\x80>=_\xFD[PPPP[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x84\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\xE6W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\xF8W=_\x80>=_\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FfW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x89\x91\x90a&\x88V[_\x81`\x0F\x0B\x13\x15a \x83W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x80\xF0\xB4La\x1F\xEE\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a iW_\x80\xFD[PZ\xF1\x15\x80\x15a {W=_\x80>=_\xFD[PPPPPPV[_\x81`\x0F\x0B\x12\x15a\x0F0Wa\x0F0\x82\x82_\x03o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01a\x1DzV[\x80`\x0F\x81\x90\x0B\x81\x14a \xE3Wa \xE3\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a \xF1V[\x91\x90PV[a\x1C\xE8\x81a \xF9V[\x80_R`\x04_\xFD[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xE8W_\x80\xFD[_\x80_\x80\x84\x86\x03`\xE0\x81\x12\x15a!NW_\x80\xFD[\x855a!Y\x81a!\x19V[\x94P` \x86\x015a!i\x81a!\x19V[\x93P`@\x86\x015a!y\x81a!\x19V[\x92P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x12\x15a!\xAAW_\x80\xFD[P\x92\x95\x91\x94P\x92``\x01\x91PV[\x80`\x02\x0B\x81\x14a\x1C\xE8W_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a!\xDBW_\x80\xFD[\x865a!\xE6\x81a!\x19V[\x95P` \x87\x015a!\xF6\x81a!\x19V[\x94P`@\x87\x015a\"\x06\x81a!\xB8V[\x93P``\x87\x015a\"\x16\x81a!\xB8V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\"AW_\x80\xFD[\x825a\"L\x81a!\x19V[\x91P` \x83\x015a\"\\\x81a!\x19V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\"wW_\x80\xFD[\x815a\"\x82\x81a!\x19V[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\"\x9AW_\x80\xFD[\x825a\"\xA5\x81a!\x19V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_``\x84\x86\x03\x12\x15a\"\xC5W_\x80\xFD[\x835a\"\xD0\x81a!\x19V[\x92P` \x84\x015a\"\xE0\x81a!\x19V[\x91P`@\x84\x015a\"\xF0\x81a!\x19V[\x80\x91PP\x92P\x92P\x92V[_\x80_\x80`\x80\x85\x87\x03\x12\x15a#\x0EW_\x80\xFD[\x845a#\x19\x81a!\x19V[\x93P` \x85\x015a#)\x81a!\x19V[\x92P`@\x85\x015\x91P``\x85\x015a#@\x81a!\x19V[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a#[W_\x80\xFD[\x815a\"\x82\x81a!\xB8V[_\x80` \x83\x85\x03\x12\x15a#wW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\x8DW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a#\x9DW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xB3W_\x80\xFD[\x85` \x82\x84\x01\x01\x11\x15a#\xC4W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\"\x82` \x83\x01\x84a#\xD4V[_\x80_``\x84\x86\x03\x12\x15a$DW_\x80\xFD[\x835a$O\x81a!\x19V[\x92P` \x84\x015a$_\x81a!\x19V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[a$\xEF\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[_\x825a$\xFB\x81a!\xB8V[`\x02\x0B`\xA0\x83\x01R` \x83\x015a%\x11\x81a!\xB8V[`\x02\x0B`\xC0\x83\x01RP`@\x82\x015`\xE0\x82\x01R``\x90\x91\x015a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R_\x90\x82\x01Ra\x01`\x01\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a%ZW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a%\xA8W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xBEW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a%\xCEW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xE8Wa%\xE8a%kV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a&TWa&Ta%kV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a&kW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a&\x98W_\x80\xFD[PQ\x91\x90PV[a'\x1E\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\xA0\x82\x01R`\xE0`\xC0\x82\x01\x81\x90R_\x90\x82\x01Ra\x01\0\x01\x91\x90PV[_` \x82\x84\x03\x12\x15a'cW_\x80\xFD[\x81Qa\"\x82\x81a!\xB8V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_\x81`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x03a'\xD6W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x03\x92\x91PPV[a(]\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x81Q\x15\x15`\xA0\x82\x01R` \x82\x01Q`\xC0\x82\x01R`@\x90\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01\x81\x90R_\x90\x82\x01Ra\x01@\x01\x91\x90PV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100fb575f3560e01c80636e1f5b99116100935780639f5e1a73116100635780639f5e1a73146102fa578063c6c3bbe61461030d578063cf618a5514610320578063e4cb970b14610333575f80fd5b80636e1f5b99146102265780637f5a7c7b146102395780638a4c6af61461027e57806391dd7346146102da575f80fd5b80633dfd3873116100ce5780633dfd38731461018357806340c10f19146101da57806347c7c5a9146101ed57806347ddb81f14610213575f80fd5b806312b4f4e6146100ff5780632974c8a4146101255780632bdfdbd11461014d57806330315f6214610160575b5f80fd5b61011261010d36600461213a565b610346565b6040519081526020015b60405180910390f35b6101386101333660046121c6565b6106f4565b6040805192835260208301919091520161011c565b61011261015b36600461213a565b610901565b61017361016e366004612230565b610e35565b604051901515815260200161011c565b6101d8610191366004612267565b600880547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b005b6101d86101e8366004612289565b610f25565b6102006101fb3660046122b3565b610f34565b60405160029190910b815260200161011c565b6102006102213660046122b3565b611067565b6101126102343660046122fb565b6111cb565b6008546102599073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161011c565b6101d861028c36600461234b565b6008805462ffffff90921674010000000000000000000000000000000000000000027fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff909216919091179055565b6102ed6102e8366004612366565b611340565b60405161011c9190612420565b6101386103083660046121c6565b6113c3565b6101d861031b366004612432565b611643565b6101d861032e366004612432565b61178f565b6101126103413660046122fb565b611849565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b9483019490945292909316908301529081906040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff86166004820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b15801561044f575f80fd5b505af1158015610461573d5f803e3d5ffd5b50506040517f5a6bcfda0000000000000000000000000000000000000000000000000000000081525f925073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169150635a6bcfda906104db9085908890600401612470565b60408051808303815f875af11580156104f6573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061051a9190612549565b90935090506105298160801d90565b600f0b158015610543575061053e81600f0b90565b600f0b155b6105ae576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600d60248201527f47657474696e6720666565733f0000000000000000000000000000000000000060448201526064015b60405180910390fd5b5f6105b98460801d90565b600f0b131580156105d657505f6105d084600f0b90565b600f0b13155b610662576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f67657474696e6720746f6b656e7320666f7220616464696e67206c697175696460448201527f697479000000000000000000000000000000000000000000000000000000000060648201526084016105a5565b61066d878785611a93565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b1580156106d4575f80fd5b505af11580156106e6573d5f803e3d5ffd5b505050505050949350505050565b5f805f60405180608001604052808860020b81526020018760020b815260200161071d87611ab4565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f2bdfdbd10000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293505f927f0000000000000000000000000000000000000000000000000000000000000000909216916348c894919161083e91600401612420565b5f604051808303815f875af1158015610859573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261089e9190810190612598565b90505f818060200190518101906108b59190612688565b90506108c18160801d90565b6fffffffffffffffffffffffffffffffff1694506108df81600f0b90565b6fffffffffffffffffffffffffffffffff169350505050965096945050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b9483019490945292909316908301529081906040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff86166004820152909150737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b158015610a0a575f80fd5b505af1158015610a1c573d5f803e3d5ffd5b50506040517f5a6bcfda00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169250635a6bcfda9150610a949084908790600401612470565b60408051808303815f875af1158015610aaf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ad39190612549565b506040805173ffffffffffffffffffffffffffffffffffffffff87811660208084018290528b8316848601528451808503860181526060850190955284519401939093206080830193909352881660a0820152919350905f9060c001604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529082905280516020909101207ff135baaa0000000000000000000000000000000000000000000000000000000082526004820184905291505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610bf0573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c149190612688565b6040517ff135baaa000000000000000000000000000000000000000000000000000000008152600481018490529091505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610ca2573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cc69190612688565b9050610cea866fffffffffffffffffffffffffffffffff8316608085901b17611b15565b95505f610cf78760801d90565b600f0b12158015610d1457505f610d0e87600f0b90565b600f0b12155b610da0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f6c6f73696e67206d6f6e657920666f722072656d6f76696e67206c697175696460448201527f697479000000000000000000000000000000000000000000000000000000000060648201526084016105a5565b610dab8a8a88611a93565b7f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610e12575f80fd5b505af1158015610e24573d5f803e3d5ffd5b505050505050505050949350505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a811682528981169382019390935295860183905274010000000000000000000000000000000000000000870460020b908601529094169183019190915281208290610f049073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690611b5b565b73ffffffffffffffffffffffffffffffffffffffff16151595945050505050565b610f30338383611643565b5050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8a811685528981169285019290925283860183905274010000000000000000000000000000000000000000870460020b908401529485169282019290925291517f695c5bf500000000000000000000000000000000000000000000000000000000815290927f0000000000000000000000000000000000000000000000000000000000000000169063695c5bf59061101e908490879060040161269f565b6020604051808303815f875af115801561103a573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061105e9190612753565b95945050505050565b6040805173ffffffffffffffffffffffffffffffffffffffff8581166024830152848116604483015283811660648084019190915283518084039091018152608490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f47c7c5a90000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081525f9283927f0000000000000000000000000000000000000000000000000000000000000000909116916348c894919161115591600401612420565b5f604051808303815f875af1158015611170573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526111b59190810190612598565b90508080602001905181019061105e9190612753565b6040805173ffffffffffffffffffffffffffffffffffffffff86811660248301528581166044830152606482018590528381166084808401919091528351808403909101815260a490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe4cb970b0000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081525f9283927f0000000000000000000000000000000000000000000000000000000000000000909116916348c89491916112c091600401612420565b5f604051808303815f875af11580156112db573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526113209190810190612598565b9050808060200190518101906113369190612688565b9695505050505050565b60605f803073ffffffffffffffffffffffffffffffffffffffff16858560405161136b92919061276e565b5f604051808303815f865af19150503d805f81146113a4576040519150601f19603f3d011682016040523d82523d5f602084013e6113a9565b606091505b5091509150816113bb57805160208201fd5b949350505050565b5f805f60405180608001604052808860020b81526020018760020b81526020016113ec87611bfa565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f12b4f4e60000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293507f0000000000000000000000000000000000000000000000000000000000000000909116916348c894919161150b91600401612420565b5f604051808303815f875af192505050801561156657506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526115639190810190612598565b60015b6115e6573d808015611593576040519150601f19603f3d011682016040523d82523d5f602084013e611598565b606091505b506115d76040518060400160405280600f81526020017f7374756666206661696c65643f3f3f0000000000000000000000000000000000815250611c59565b6115e081611ceb565b50611637565b5f818060200190518101906115fb9190612688565b90506116078160801d90565b6116109061277d565b6fffffffffffffffffffffffffffffffff16945061162e81600f0b90565b6108df9061277d565b50965096945050505050565b6040805173ffffffffffffffffffffffffffffffffffffffff85811660248301528481166044830152606480830185905283518084039091018152608490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fcf618a550000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081527f0000000000000000000000000000000000000000000000000000000000000000909216916348c894919161172991600401612420565b5f604051808303815f875af1158015611744573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526117899190810190612598565b50505050565b6040517f156e29f600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8481166004830152602482018490526044820183905283917f00000000000000000000000000000000000000000000000000000000000000009091169063156e29f6906064015f604051808303815f87803b158015611827575f80fd5b505af1158015611839573d5f803e3d5ffd5b5050505061178983836001611d7a565b5f73ffffffffffffffffffffffffffffffffffffffff808516908616108181611900576008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8d811685528e8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b938201939093529190921691810191909152611990565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8e811685528d8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b9382019390935291909216918101919091525b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c82604051806060016040528086151581526020018981526020018873ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b8152600401611a1f9291906127de565b6020604051808303815f875af1158015611a3b573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a5f9190612688565b9250611a77815f0151611a728560801d90565b611f8a565b611a898160200151611a7285600f0b90565b5050949350505050565b611aa183611a728360801d90565b611aaf82611a7283600f0b90565b505050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611b0f576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b5f608082811d9084901d01600f83810b9085900b0161105e611b36836120ae565b611b3f836120ae565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa158015611bd6573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113bb9190612688565b5f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821115611c55576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5090565b611ce881604051602401611c6d9190612420565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac000000000000000000000000000000000000000000000000000000001790526120e8565b50565b611ce881604051602401611cff9190612420565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f0be77f56000000000000000000000000000000000000000000000000000000001790526120e8565b8115611aaf578015611e59577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611ddf8573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611e42575f80fd5b505af1158015611e54573d5f803e3d5ffd5b505050505b6040517f40c10f1900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081166004830152602482018490528416906340c10f19906044015f604051808303815f87803b158015611ee6575f80fd5b505af1158015611ef8573d5f803e3d5ffd5b505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611f66573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906117899190612688565b5f81600f0b1315612083577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166380f0b44c611fee8473ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526fffffffffffffffffffffffffffffffff841660248201526044015f604051808303815f87803b158015612069575f80fd5b505af115801561207b573d5f803e3d5ffd5b505050505050565b5f81600f0b1215610f3057610f3082825f036fffffffffffffffffffffffffffffffff166001611d7a565b80600f81900b81146120e3576120e37f93dafdf1000000000000000000000000000000000000000000000000000000006120f1565b919050565b611ce8816120f9565b805f5260045ffd5b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b73ffffffffffffffffffffffffffffffffffffffff81168114611ce8575f80fd5b5f805f8084860360e081121561214e575f80fd5b853561215981612119565b9450602086013561216981612119565b9350604086013561217981612119565b925060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0820112156121aa575f80fd5b509295919450926060019150565b8060020b8114611ce8575f80fd5b5f805f805f8060c087890312156121db575f80fd5b86356121e681612119565b955060208701356121f681612119565b94506040870135612206816121b8565b93506060870135612216816121b8565b9598949750929560808101359460a0909101359350915050565b5f8060408385031215612241575f80fd5b823561224c81612119565b9150602083013561225c81612119565b809150509250929050565b5f60208284031215612277575f80fd5b813561228281612119565b9392505050565b5f806040838503121561229a575f80fd5b82356122a581612119565b946020939093013593505050565b5f805f606084860312156122c5575f80fd5b83356122d081612119565b925060208401356122e081612119565b915060408401356122f081612119565b809150509250925092565b5f805f806080858703121561230e575f80fd5b843561231981612119565b9350602085013561232981612119565b925060408501359150606085013561234081612119565b939692955090935050565b5f6020828403121561235b575f80fd5b8135612282816121b8565b5f8060208385031215612377575f80fd5b823567ffffffffffffffff81111561238d575f80fd5b8301601f8101851361239d575f80fd5b803567ffffffffffffffff8111156123b3575f80fd5b8560208284010111156123c4575f80fd5b6020919091019590945092505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f61228260208301846123d4565b5f805f60608486031215612444575f80fd5b833561244f81612119565b9250602084013561245f81612119565b929592945050506040919091013590565b6124ef818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b5f82356124fb816121b8565b60020b60a08301526020830135612511816121b8565b60020b60c083015250604082013560e082015260609091013561010082015261014061012082018190525f9082015261016001919050565b5f806040838503121561255a575f80fd5b505080516020909101519092909150565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f602082840312156125a8575f80fd5b815167ffffffffffffffff8111156125be575f80fd5b8201601f810184136125ce575f80fd5b805167ffffffffffffffff8111156125e8576125e861256b565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156126545761265461256b565b60405281815282820160200186101561266b575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215612698575f80fd5b5051919050565b61271e818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff9190911660a082015260e060c082018190525f9082015261010001919050565b5f60208284031215612763575f80fd5b8151612282816121b8565b818382375f9101908152919050565b5f81600f0b7fffffffffffffffffffffffffffffffff8000000000000000000000000000000081036127d6577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f0392915050565b61285d818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8151151560a0820152602082015160c082015260409091015173ffffffffffffffffffffffffffffffffffffffff1660e082015261012061010082018190525f908201526101400191905056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cn\x1F[\x99\x11a\0\x93W\x80c\x9F^\x1As\x11a\0cW\x80c\x9F^\x1As\x14a\x02\xFAW\x80c\xC6\xC3\xBB\xE6\x14a\x03\rW\x80c\xCFa\x8AU\x14a\x03 W\x80c\xE4\xCB\x97\x0B\x14a\x033W_\x80\xFD[\x80cn\x1F[\x99\x14a\x02&W\x80c\x7FZ|{\x14a\x029W\x80c\x8ALj\xF6\x14a\x02~W\x80c\x91\xDDsF\x14a\x02\xDAW_\x80\xFD[\x80c=\xFD8s\x11a\0\xCEW\x80c=\xFD8s\x14a\x01\x83W\x80c@\xC1\x0F\x19\x14a\x01\xDAW\x80cG\xC7\xC5\xA9\x14a\x01\xEDW\x80cG\xDD\xB8\x1F\x14a\x02\x13W_\x80\xFD[\x80c\x12\xB4\xF4\xE6\x14a\0\xFFW\x80c)t\xC8\xA4\x14a\x01%W\x80c+\xDF\xDB\xD1\x14a\x01MW\x80c01_b\x14a\x01`W[_\x80\xFD[a\x01\x12a\x01\r6`\x04a!:V[a\x03FV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x0136`\x04a!\xC6V[a\x06\xF4V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1CV[a\x01\x12a\x01[6`\x04a!:V[a\t\x01V[a\x01sa\x01n6`\x04a\"0V[a\x0E5V[`@Q\x90\x15\x15\x81R` \x01a\x01\x1CV[a\x01\xD8a\x01\x916`\x04a\"gV[`\x08\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[a\x01\xD8a\x01\xE86`\x04a\"\x89V[a\x0F%V[a\x02\0a\x01\xFB6`\x04a\"\xB3V[a\x0F4V[`@Q`\x02\x91\x90\x91\x0B\x81R` \x01a\x01\x1CV[a\x02\0a\x02!6`\x04a\"\xB3V[a\x10gV[a\x01\x12a\x0246`\x04a\"\xFBV[a\x11\xCBV[`\x08Ta\x02Y\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x01\xD8a\x02\x8C6`\x04a#KV[`\x08\x80Tb\xFF\xFF\xFF\x90\x92\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xEDa\x02\xE86`\x04a#fV[a\x13@V[`@Qa\x01\x1C\x91\x90a$ V[a\x018a\x03\x086`\x04a!\xC6V[a\x13\xC3V[a\x01\xD8a\x03\x1B6`\x04a$2V[a\x16CV[a\x01\xD8a\x03.6`\x04a$2V[a\x17\x8FV[a\x01\x12a\x03A6`\x04a\"\xFBV[a\x18IV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90\x81\x90`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x04OW_\x80\xFD[PZ\xF1\x15\x80\x15a\x04aW=_\x80>=_\xFD[PP`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91PcZk\xCF\xDA\x90a\x04\xDB\x90\x85\x90\x88\x90`\x04\x01a$pV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04\xF6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x1A\x91\x90a%IV[\x90\x93P\x90Pa\x05)\x81`\x80\x1D\x90V[`\x0F\x0B\x15\x80\x15a\x05CWPa\x05>\x81`\x0F\x0B\x90V[`\x0F\x0B\x15[a\x05\xAEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FGetting fees?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x05\xB9\x84`\x80\x1D\x90V[`\x0F\x0B\x13\x15\x80\x15a\x05\xD6WP_a\x05\xD0\x84`\x0F\x0B\x90V[`\x0F\x0B\x13\x15[a\x06bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fgetting tokens for adding liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xA5V[a\x06m\x87\x87\x85a\x1A\x93V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06\xD4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\xE6W=_\x80>=_\xFD[PPPPPP\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x07\x1D\x87a\x1A\xB4V[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F+\xDF\xDB\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x08>\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x08YW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x08\x9E\x91\x90\x81\x01\x90a%\x98V[\x90P_\x81\x80` \x01\x90Q\x81\x01\x90a\x08\xB5\x91\x90a&\x88V[\x90Pa\x08\xC1\x81`\x80\x1D\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\x08\xDF\x81`\x0F\x0B\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93PPPP\x96P\x96\x94PPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90\x81\x90`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x04\x82\x01R\x90\x91Psq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\nW_\x80\xFD[PZ\xF1\x15\x80\x15a\n\x1CW=_\x80>=_\xFD[PP`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92PcZk\xCF\xDA\x91Pa\n\x94\x90\x84\x90\x87\x90`\x04\x01a$pV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\n\xAFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xD3\x91\x90a%IV[P`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16` \x80\x84\x01\x82\x90R\x8B\x83\x16\x84\x86\x01R\x84Q\x80\x85\x03\x86\x01\x81R``\x85\x01\x90\x95R\x84Q\x94\x01\x93\x90\x93 `\x80\x83\x01\x93\x90\x93R\x88\x16`\xA0\x82\x01R\x91\x93P\x90_\x90`\xC0\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01\x84\x90R\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xF0W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x14\x91\x90a&\x88V[`@Q\x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xA2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xC6\x91\x90a&\x88V[\x90Pa\x0C\xEA\x86o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x85\x90\x1B\x17a\x1B\x15V[\x95P_a\x0C\xF7\x87`\x80\x1D\x90V[`\x0F\x0B\x12\x15\x80\x15a\r\x14WP_a\r\x0E\x87`\x0F\x0B\x90V[`\x0F\x0B\x12\x15[a\r\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Flosing money for removing liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xA5V[a\r\xAB\x8A\x8A\x88a\x1A\x93V[\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0E\x12W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0E$W=_\x80>=_\xFD[PPPPPPPPP\x94\x93PPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x81\x16\x93\x82\x01\x93\x90\x93R\x95\x86\x01\x83\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x04`\x02\x0B\x90\x86\x01R\x90\x94\x16\x91\x83\x01\x91\x90\x91R\x81 \x82\x90a\x0F\x04\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x1B[V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x95\x94PPPPPV[a\x0F03\x83\x83a\x16CV[PPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x85R\x89\x81\x16\x92\x85\x01\x92\x90\x92R\x83\x86\x01\x83\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x04`\x02\x0B\x90\x84\x01R\x94\x85\x16\x92\x82\x01\x92\x90\x92R\x91Q\x7Fi\\[\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x90\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90ci\\[\xF5\x90a\x10\x1E\x90\x84\x90\x87\x90`\x04\x01a&\x9FV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10:W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10^\x91\x90a'SV[\x95\x94PPPPPV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R\x83\x81\x16`d\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FG\xC7\xC5\xA9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cH\xC8\x94\x91\x91a\x11U\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x11pW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x11\xB5\x91\x90\x81\x01\x90a%\x98V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x10^\x91\x90a'SV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16`$\x83\x01R\x85\x81\x16`D\x83\x01R`d\x82\x01\x85\x90R\x83\x81\x16`\x84\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\xA4\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE4\xCB\x97\x0B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cH\xC8\x94\x91\x91a\x12\xC0\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12\xDBW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x13 \x91\x90\x81\x01\x90a%\x98V[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x136\x91\x90a&\x88V[\x96\x95PPPPPPV[``_\x800s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85`@Qa\x13k\x92\x91\x90a'nV[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x13\xA4W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13\xA9V[``\x91P[P\x91P\x91P\x81a\x13\xBBW\x80Q` \x82\x01\xFD[\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x13\xEC\x87a\x1B\xFAV[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x12\xB4\xF4\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x91cH\xC8\x94\x91\x91a\x15\x0B\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x92PPP\x80\x15a\x15fWP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x15c\x91\x90\x81\x01\x90a%\x98V[`\x01[a\x15\xE6W=\x80\x80\x15a\x15\x93W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x15\x98V[``\x91P[Pa\x15\xD7`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7Fstuff failed???\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x1CYV[a\x15\xE0\x81a\x1C\xEBV[Pa\x167V[_\x81\x80` \x01\x90Q\x81\x01\x90a\x15\xFB\x91\x90a&\x88V[\x90Pa\x16\x07\x81`\x80\x1D\x90V[a\x16\x10\x90a'}V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\x16.\x81`\x0F\x0B\x90V[a\x08\xDF\x90a'}V[P\x96P\x96\x94PPPPPV[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCFa\x8AU\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x17)\x91`\x04\x01a$ V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x17DW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x17\x89\x91\x90\x81\x01\x90a%\x98V[PPPPV[`@Q\x7F\x15n)\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`D\x82\x01\x83\x90R\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x15n)\xF6\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x18'W_\x80\xFD[PZ\xF1\x15\x80\x15a\x189W=_\x80>=_\xFD[PPPPa\x17\x89\x83\x83`\x01a\x1DzV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x90\x86\x16\x10\x81\x81a\x19\0W`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16\x85R\x8E\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91Ra\x19\x90V[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8E\x81\x16\x85R\x8D\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91R[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x82`@Q\x80``\x01`@R\x80\x86\x15\x15\x81R` \x01\x89\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1A\x1F\x92\x91\x90a'\xDEV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1A;W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A_\x91\x90a&\x88V[\x92Pa\x1Aw\x81_\x01Qa\x1Ar\x85`\x80\x1D\x90V[a\x1F\x8AV[a\x1A\x89\x81` \x01Qa\x1Ar\x85`\x0F\x0B\x90V[PP\x94\x93PPPPV[a\x1A\xA1\x83a\x1Ar\x83`\x80\x1D\x90V[a\x1A\xAF\x82a\x1Ar\x83`\x0F\x0B\x90V[PPPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x1B\x0FW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a\x10^a\x1B6\x83a \xAEV[a\x1B?\x83a \xAEV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xD6W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xBB\x91\x90a&\x88V[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1CUW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[a\x1C\xE8\x81`@Q`$\x01a\x1Cm\x91\x90a$ V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra \xE8V[PV[a\x1C\xE8\x81`@Q`$\x01a\x1C\xFF\x91\x90a$ V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x0B\xE7\x7FV\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra \xE8V[\x81\x15a\x1A\xAFW\x80\x15a\x1EYW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1D\xDF\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1EBW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1ETW=_\x80>=_\xFD[PPPP[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x84\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\xE6W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\xF8W=_\x80>=_\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1FfW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x89\x91\x90a&\x88V[_\x81`\x0F\x0B\x13\x15a \x83W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x80\xF0\xB4La\x1F\xEE\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a iW_\x80\xFD[PZ\xF1\x15\x80\x15a {W=_\x80>=_\xFD[PPPPPPV[_\x81`\x0F\x0B\x12\x15a\x0F0Wa\x0F0\x82\x82_\x03o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01a\x1DzV[\x80`\x0F\x81\x90\x0B\x81\x14a \xE3Wa \xE3\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a \xF1V[\x91\x90PV[a\x1C\xE8\x81a \xF9V[\x80_R`\x04_\xFD[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xE8W_\x80\xFD[_\x80_\x80\x84\x86\x03`\xE0\x81\x12\x15a!NW_\x80\xFD[\x855a!Y\x81a!\x19V[\x94P` \x86\x015a!i\x81a!\x19V[\x93P`@\x86\x015a!y\x81a!\x19V[\x92P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x12\x15a!\xAAW_\x80\xFD[P\x92\x95\x91\x94P\x92``\x01\x91PV[\x80`\x02\x0B\x81\x14a\x1C\xE8W_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a!\xDBW_\x80\xFD[\x865a!\xE6\x81a!\x19V[\x95P` \x87\x015a!\xF6\x81a!\x19V[\x94P`@\x87\x015a\"\x06\x81a!\xB8V[\x93P``\x87\x015a\"\x16\x81a!\xB8V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_\x80`@\x83\x85\x03\x12\x15a\"AW_\x80\xFD[\x825a\"L\x81a!\x19V[\x91P` \x83\x015a\"\\\x81a!\x19V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a\"wW_\x80\xFD[\x815a\"\x82\x81a!\x19V[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\"\x9AW_\x80\xFD[\x825a\"\xA5\x81a!\x19V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_``\x84\x86\x03\x12\x15a\"\xC5W_\x80\xFD[\x835a\"\xD0\x81a!\x19V[\x92P` \x84\x015a\"\xE0\x81a!\x19V[\x91P`@\x84\x015a\"\xF0\x81a!\x19V[\x80\x91PP\x92P\x92P\x92V[_\x80_\x80`\x80\x85\x87\x03\x12\x15a#\x0EW_\x80\xFD[\x845a#\x19\x81a!\x19V[\x93P` \x85\x015a#)\x81a!\x19V[\x92P`@\x85\x015\x91P``\x85\x015a#@\x81a!\x19V[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15a#[W_\x80\xFD[\x815a\"\x82\x81a!\xB8V[_\x80` \x83\x85\x03\x12\x15a#wW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\x8DW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a#\x9DW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\xB3W_\x80\xFD[\x85` \x82\x84\x01\x01\x11\x15a#\xC4W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\"\x82` \x83\x01\x84a#\xD4V[_\x80_``\x84\x86\x03\x12\x15a$DW_\x80\xFD[\x835a$O\x81a!\x19V[\x92P` \x84\x015a$_\x81a!\x19V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[a$\xEF\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[_\x825a$\xFB\x81a!\xB8V[`\x02\x0B`\xA0\x83\x01R` \x83\x015a%\x11\x81a!\xB8V[`\x02\x0B`\xC0\x83\x01RP`@\x82\x015`\xE0\x82\x01R``\x90\x91\x015a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R_\x90\x82\x01Ra\x01`\x01\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a%ZW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a%\xA8W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xBEW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a%\xCEW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a%\xE8Wa%\xE8a%kV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a&TWa&Ta%kV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a&kW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a&\x98W_\x80\xFD[PQ\x91\x90PV[a'\x1E\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16`\xA0\x82\x01R`\xE0`\xC0\x82\x01\x81\x90R_\x90\x82\x01Ra\x01\0\x01\x91\x90PV[_` \x82\x84\x03\x12\x15a'cW_\x80\xFD[\x81Qa\"\x82\x81a!\xB8V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_\x81`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x03a'\xD6W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x03\x92\x91PPV[a(]\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x81Q\x15\x15`\xA0\x82\x01R` \x82\x01Q`\xC0\x82\x01R`@\x90\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01\x81\x90R_\x90\x82\x01Ra\x01@\x01\x91\x90PV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct BalanceDelta(alloy::sol_types::private::primitives::aliases::I256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BalanceDelta>
        for alloy::sol_types::private::primitives::aliases::I256 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Int<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl BalanceDelta {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(
                value: alloy::sol_types::private::primitives::aliases::I256,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::I256 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BalanceDelta {
            type RustType = alloy::sol_types::private::primitives::aliases::I256;
            type Token<'a> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BalanceDelta {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**Custom error with signature `Overflow()` and selector `0x35278d12`.
```solidity
error Overflow();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Overflow {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Overflow> for UnderlyingRustTuple<'_> {
            fn from(value: Overflow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Overflow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Overflow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Overflow()";
            const SELECTOR: [u8; 4] = [53u8, 39u8, 141u8, 18u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address uniV4);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub uniV4: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.uniV4,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { uniV4: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.uniV4,
                    ),
                )
            }
        }
    };
    /**Function with signature `__addLiquidity(address,address,address,(int24,int24,int256,bytes32))` and selector `0x12b4f4e6`.
```solidity
function __addLiquidity(address asset0, address asset1, address sender, IPoolManager.ModifyLiquidityParams memory params) external returns (BalanceDelta callerDelta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __addLiquidityCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub sender: alloy::sol_types::private::Address,
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`__addLiquidity(address,address,address,(int24,int24,int256,bytes32))`](__addLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __addLiquidityReturn {
        pub callerDelta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IPoolManager::ModifyLiquidityParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__addLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: __addLiquidityCall) -> Self {
                    (value.asset0, value.asset1, value.sender, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __addLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        sender: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__addLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: __addLiquidityReturn) -> Self {
                    (value.callerDelta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __addLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { callerDelta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __addLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IPoolManager::ModifyLiquidityParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __addLiquidityReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__addLiquidity(address,address,address,(int24,int24,int256,bytes32))";
            const SELECTOR: [u8; 4] = [18u8, 180u8, 244u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `__initializePool(address,address,uint160)` and selector `0x47c7c5a9`.
```solidity
function __initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96) external returns (int24 tick);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __initializePoolCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub initialSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`__initializePool(address,address,uint160)`](__initializePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __initializePoolReturn {
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U160,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__initializePoolCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: __initializePoolCall) -> Self {
                    (value.asset0, value.asset1, value.initialSqrtPriceX96)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __initializePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        initialSqrtPriceX96: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I24,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__initializePoolReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: __initializePoolReturn) -> Self {
                    (value.tick,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __initializePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tick: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __initializePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __initializePoolReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__initializePool(address,address,uint160)";
            const SELECTOR: [u8; 4] = [71u8, 199u8, 197u8, 169u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialSqrtPriceX96),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `__mint(address,address,uint256)` and selector `0xcf618a55`.
```solidity
function __mint(address to, address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __mintCall {
        pub to: alloy::sol_types::private::Address,
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`__mint(address,address,uint256)`](__mintCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __mintReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__mintCall> for UnderlyingRustTuple<'_> {
                fn from(value: __mintCall) -> Self {
                    (value.to, value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __mintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        asset: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__mintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __mintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __mintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __mintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __mintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__mint(address,address,uint256)";
            const SELECTOR: [u8; 4] = [207u8, 97u8, 138u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `__removeLiquidity(address,address,address,(int24,int24,int256,bytes32))` and selector `0x2bdfdbd1`.
```solidity
function __removeLiquidity(address asset0, address asset1, address sender, IPoolManager.ModifyLiquidityParams memory params) external returns (BalanceDelta delta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __removeLiquidityCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub sender: alloy::sol_types::private::Address,
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`__removeLiquidity(address,address,address,(int24,int24,int256,bytes32))`](__removeLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __removeLiquidityReturn {
        pub delta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IPoolManager::ModifyLiquidityParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__removeLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: __removeLiquidityCall) -> Self {
                    (value.asset0, value.asset1, value.sender, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __removeLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        sender: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__removeLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: __removeLiquidityReturn) -> Self {
                    (value.delta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __removeLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __removeLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                IPoolManager::ModifyLiquidityParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __removeLiquidityReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__removeLiquidity(address,address,address,(int24,int24,int256,bytes32))";
            const SELECTOR: [u8; 4] = [43u8, 223u8, 219u8, 209u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `__swap(address,address,int256,uint160)` and selector `0xe4cb970b`.
```solidity
function __swap(address assetIn, address assetOut, int256 amountSpecified, uint160 sqrtPriceLimitX96) external returns (BalanceDelta swapDelta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __swapCall {
        pub assetIn: alloy::sol_types::private::Address,
        pub assetOut: alloy::sol_types::private::Address,
        pub amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
        pub sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`__swap(address,address,int256,uint160)`](__swapCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __swapReturn {
        pub swapDelta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::I256,
                alloy::sol_types::private::primitives::aliases::U160,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__swapCall> for UnderlyingRustTuple<'_> {
                fn from(value: __swapCall) -> Self {
                    (
                        value.assetIn,
                        value.assetOut,
                        value.amountSpecified,
                        value.sqrtPriceLimitX96,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __swapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assetIn: tuple.0,
                        assetOut: tuple.1,
                        amountSpecified: tuple.2,
                        sqrtPriceLimitX96: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<__swapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: __swapReturn) -> Self {
                    (value.swapDelta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for __swapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { swapDelta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __swapCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __swapReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__swap(address,address,int256,uint160)";
            const SELECTOR: [u8; 4] = [228u8, 203u8, 151u8, 11u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetIn,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetOut,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountSpecified),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceLimitX96),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addLiquidity(address,address,int24,int24,uint256,bytes32)` and selector `0x9f5e1a73`.
```solidity
function addLiquidity(address asset0, address asset1, int24 tickLower, int24 tickUpper, uint256 liquidity, bytes32 salt) external returns (uint256 amount0, uint256 amount1);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct addLiquidityCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`addLiquidity(address,address,int24,int24,uint256,bytes32)`](addLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct addLiquidityReturn {
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityCall) -> Self {
                    (
                        value.asset0,
                        value.asset1,
                        value.tickLower,
                        value.tickUpper,
                        value.liquidity,
                        value.salt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        tickLower: tuple.2,
                        tickUpper: tuple.3,
                        liquidity: tuple.4,
                        salt: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addLiquidityReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addLiquidityReturn) -> Self {
                    (value.amount0, value.amount1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amount0: tuple.0,
                        amount1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addLiquidity(address,address,int24,int24,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [159u8, 94u8, 26u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `hook()` and selector `0x7f5a7c7b`.
```solidity
function hook() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct hookCall {}
    ///Container type for the return parameters of the [`hook()`](hookCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct hookReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hookCall> for UnderlyingRustTuple<'_> {
                fn from(value: hookCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hookCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hookReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hookReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hookReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hookCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hookReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hook()";
            const SELECTOR: [u8; 4] = [127u8, 90u8, 124u8, 123u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initializePool(address,address,uint160)` and selector `0x47ddb81f`.
```solidity
function initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96) external returns (int24 tick);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializePoolCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub initialSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`initializePool(address,address,uint160)`](initializePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializePoolReturn {
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U160,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializePoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializePoolCall) -> Self {
                    (value.asset0, value.asset1, value.initialSqrtPriceX96)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        initialSqrtPriceX96: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I24,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializePoolReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializePoolReturn) -> Self {
                    (value.tick,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tick: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializePoolReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializePool(address,address,uint160)";
            const SELECTOR: [u8; 4] = [71u8, 221u8, 184u8, 31u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.initialSqrtPriceX96),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isInitialized(address,address)` and selector `0x30315f62`.
```solidity
function isInitialized(address asset0, address asset1) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isInitializedCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isInitialized(address,address)`](isInitializedCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isInitializedReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isInitializedCall> for UnderlyingRustTuple<'_> {
                fn from(value: isInitializedCall) -> Self {
                    (value.asset0, value.asset1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isInitializedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isInitializedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isInitializedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isInitializedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isInitializedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isInitializedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isInitialized(address,address)";
            const SELECTOR: [u8; 4] = [48u8, 49u8, 95u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mint(address,uint256)` and selector `0x40c10f19`.
```solidity
function mint(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mint_0Call {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,uint256)`](mint_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mint_0Return {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mint_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: mint_0Call) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mint_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: mint_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mint_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mint_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,uint256)";
            const SELECTOR: [u8; 4] = [64u8, 193u8, 15u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mint(address,address,uint256)` and selector `0xc6c3bbe6`.
```solidity
function mint(address to, address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mint_1Call {
        pub to: alloy::sol_types::private::Address,
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,address,uint256)`](mint_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mint_1Return {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mint_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: mint_1Call) -> Self {
                    (value.to, value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        asset: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mint_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: mint_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mint_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mint_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mint_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,address,uint256)";
            const SELECTOR: [u8; 4] = [198u8, 195u8, 187u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeLiquidity(address,address,int24,int24,uint256,bytes32)` and selector `0x2974c8a4`.
```solidity
function removeLiquidity(address asset0, address asset1, int24 tickLower, int24 tickUpper, uint256 liquidity, bytes32 salt) external returns (uint256 amount0, uint256 amount1);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct removeLiquidityCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        pub liquidity: alloy::sol_types::private::primitives::aliases::U256,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`removeLiquidity(address,address,int24,int24,uint256,bytes32)`](removeLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct removeLiquidityReturn {
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::FixedBytes<32>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityCall) -> Self {
                    (
                        value.asset0,
                        value.asset1,
                        value.tickLower,
                        value.tickUpper,
                        value.liquidity,
                        value.salt,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        tickLower: tuple.2,
                        tickUpper: tuple.3,
                        liquidity: tuple.4,
                        salt: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeLiquidityReturn) -> Self {
                    (value.amount0, value.amount1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        amount0: tuple.0,
                        amount1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeLiquidity(address,address,int24,int24,uint256,bytes32)";
            const SELECTOR: [u8; 4] = [41u8, 116u8, 200u8, 164u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.asset1,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setHook(address)` and selector `0x3dfd3873`.
```solidity
function setHook(address hook_) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setHookCall {
        pub hook_: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setHook(address)`](setHookCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setHookReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setHookCall> for UnderlyingRustTuple<'_> {
                fn from(value: setHookCall) -> Self {
                    (value.hook_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setHookCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { hook_: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setHookReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setHookReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setHookReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setHookCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setHookReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setHook(address)";
            const SELECTOR: [u8; 4] = [61u8, 253u8, 56u8, 115u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hook_,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swap(address,address,int256,uint160)` and selector `0x6e1f5b99`.
```solidity
function swap(address assetIn, address assetOut, int256 amountSpecified, uint160 sqrtPriceLimitX96) external returns (BalanceDelta delta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapCall {
        pub assetIn: alloy::sol_types::private::Address,
        pub assetOut: alloy::sol_types::private::Address,
        pub amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
        pub sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`swap(address,address,int256,uint160)`](swapCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapReturn {
        pub delta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::I256,
                alloy::sol_types::private::primitives::aliases::U160,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapCall> for UnderlyingRustTuple<'_> {
                fn from(value: swapCall) -> Self {
                    (
                        value.assetIn,
                        value.assetOut,
                        value.amountSpecified,
                        value.sqrtPriceLimitX96,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assetIn: tuple.0,
                        assetOut: tuple.1,
                        amountSpecified: tuple.2,
                        sqrtPriceLimitX96: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: swapReturn) -> Self {
                    (value.delta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swap(address,address,int256,uint160)";
            const SELECTOR: [u8; 4] = [110u8, 31u8, 91u8, 153u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetIn,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetOut,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountSpecified),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceLimitX96),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `tickSpacing(int24)` and selector `0x8a4c6af6`.
```solidity
function tickSpacing(int24 spacing) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct tickSpacingCall {
        pub spacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`tickSpacing(int24)`](tickSpacingCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct tickSpacingReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I24,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tickSpacingCall> for UnderlyingRustTuple<'_> {
                fn from(value: tickSpacingCall) -> Self {
                    (value.spacing,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tickSpacingCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { spacing: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<tickSpacingReturn> for UnderlyingRustTuple<'_> {
                fn from(value: tickSpacingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for tickSpacingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for tickSpacingCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Int<24>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = tickSpacingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "tickSpacing(int24)";
            const SELECTOR: [u8; 4] = [138u8, 76u8, 106u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.spacing),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `unlockCallback(bytes)` and selector `0x91dd7346`.
```solidity
function unlockCallback(bytes memory data) external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct unlockCallbackCall {
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`unlockCallback(bytes)`](unlockCallbackCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct unlockCallbackReturn {
        pub _0: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockCallbackCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockCallbackCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockCallbackCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockCallbackReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: unlockCallbackReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for unlockCallbackReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockCallbackCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockCallbackReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlockCallback(bytes)";
            const SELECTOR: [u8; 4] = [145u8, 221u8, 115u8, 70u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`PoolGate`](self) function calls.
    pub enum PoolGateCalls {
        __addLiquidity(__addLiquidityCall),
        __initializePool(__initializePoolCall),
        __mint(__mintCall),
        __removeLiquidity(__removeLiquidityCall),
        __swap(__swapCall),
        addLiquidity(addLiquidityCall),
        hook(hookCall),
        initializePool(initializePoolCall),
        isInitialized(isInitializedCall),
        mint_0(mint_0Call),
        mint_1(mint_1Call),
        removeLiquidity(removeLiquidityCall),
        setHook(setHookCall),
        swap(swapCall),
        tickSpacing(tickSpacingCall),
        unlockCallback(unlockCallbackCall),
    }
    #[automatically_derived]
    impl PoolGateCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [18u8, 180u8, 244u8, 230u8],
            [41u8, 116u8, 200u8, 164u8],
            [43u8, 223u8, 219u8, 209u8],
            [48u8, 49u8, 95u8, 98u8],
            [61u8, 253u8, 56u8, 115u8],
            [64u8, 193u8, 15u8, 25u8],
            [71u8, 199u8, 197u8, 169u8],
            [71u8, 221u8, 184u8, 31u8],
            [110u8, 31u8, 91u8, 153u8],
            [127u8, 90u8, 124u8, 123u8],
            [138u8, 76u8, 106u8, 246u8],
            [145u8, 221u8, 115u8, 70u8],
            [159u8, 94u8, 26u8, 115u8],
            [198u8, 195u8, 187u8, 230u8],
            [207u8, 97u8, 138u8, 85u8],
            [228u8, 203u8, 151u8, 11u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolGateCalls {
        const NAME: &'static str = "PoolGateCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 16usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::__addLiquidity(_) => {
                    <__addLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__initializePool(_) => {
                    <__initializePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__mint(_) => <__mintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::__removeLiquidity(_) => {
                    <__removeLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::__swap(_) => <__swapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::addLiquidity(_) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hook(_) => <hookCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initializePool(_) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isInitialized(_) => {
                    <isInitializedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mint_0(_) => <mint_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::mint_1(_) => <mint_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::removeLiquidity(_) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setHook(_) => <setHookCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::swap(_) => <swapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::tickSpacing(_) => {
                    <tickSpacingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockCallback(_) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<PoolGateCalls>] = &[
                {
                    fn __addLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <__addLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::__addLiquidity)
                    }
                    __addLiquidity
                },
                {
                    fn removeLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <removeLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::removeLiquidity)
                    }
                    removeLiquidity
                },
                {
                    fn __removeLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <__removeLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::__removeLiquidity)
                    }
                    __removeLiquidity
                },
                {
                    fn isInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <isInitializedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::isInitialized)
                    }
                    isInitialized
                },
                {
                    fn setHook(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <setHookCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::setHook)
                    }
                    setHook
                },
                {
                    fn mint_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <mint_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::mint_0)
                    }
                    mint_0
                },
                {
                    fn __initializePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <__initializePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::__initializePool)
                    }
                    __initializePool
                },
                {
                    fn initializePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <initializePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::initializePool)
                    }
                    initializePool
                },
                {
                    fn swap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <swapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::swap)
                    }
                    swap
                },
                {
                    fn hook(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <hookCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::hook)
                    }
                    hook
                },
                {
                    fn tickSpacing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <tickSpacingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::tickSpacing)
                    }
                    tickSpacing
                },
                {
                    fn unlockCallback(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <unlockCallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::unlockCallback)
                    }
                    unlockCallback
                },
                {
                    fn addLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <addLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::addLiquidity)
                    }
                    addLiquidity
                },
                {
                    fn mint_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <mint_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::mint_1)
                    }
                    mint_1
                },
                {
                    fn __mint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <__mintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::__mint)
                    }
                    __mint
                },
                {
                    fn __swap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateCalls> {
                        <__swapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateCalls::__swap)
                    }
                    __swap
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::__addLiquidity(inner) => {
                    <__addLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::__initializePool(inner) => {
                    <__initializePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::__mint(inner) => {
                    <__mintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::__removeLiquidity(inner) => {
                    <__removeLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::__swap(inner) => {
                    <__swapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::addLiquidity(inner) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hook(inner) => {
                    <hookCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isInitialized(inner) => {
                    <isInitializedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::mint_0(inner) => {
                    <mint_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mint_1(inner) => {
                    <mint_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::removeLiquidity(inner) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setHook(inner) => {
                    <setHookCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::swap(inner) => {
                    <swapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::tickSpacing(inner) => {
                    <tickSpacingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlockCallback(inner) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::__addLiquidity(inner) => {
                    <__addLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__initializePool(inner) => {
                    <__initializePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__mint(inner) => {
                    <__mintCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::__removeLiquidity(inner) => {
                    <__removeLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::__swap(inner) => {
                    <__swapCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::addLiquidity(inner) => {
                    <addLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hook(inner) => {
                    <hookCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isInitialized(inner) => {
                    <isInitializedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mint_0(inner) => {
                    <mint_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::mint_1(inner) => {
                    <mint_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::removeLiquidity(inner) => {
                    <removeLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setHook(inner) => {
                    <setHookCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::swap(inner) => {
                    <swapCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::tickSpacing(inner) => {
                    <tickSpacingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlockCallback(inner) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PoolGate`](self) custom errors.
    pub enum PoolGateErrors {
        Overflow(Overflow),
    }
    #[automatically_derived]
    impl PoolGateErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[53u8, 39u8, 141u8, 18u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolGateErrors {
        const NAME: &'static str = "PoolGateErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::Overflow(_) => <Overflow as alloy_sol_types::SolError>::SELECTOR,
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<PoolGateErrors>] = &[
                {
                    fn Overflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolGateErrors> {
                        <Overflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolGateErrors::Overflow)
                    }
                    Overflow
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PoolGate`](self) contract instance.

See the [wrapper's documentation](`PoolGateInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolGateInstance<T, P, N> {
        PoolGateInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        uniV4: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PoolGateInstance<T, P, N>>,
    > {
        PoolGateInstance::<T, P, N>::deploy(provider, uniV4)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        uniV4: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        PoolGateInstance::<T, P, N>::deploy_builder(provider, uniV4)
    }
    /**A [`PoolGate`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolGate`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolGateInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolGateInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolGateInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolGateInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolGate`](self) contract instance.

See the [wrapper's documentation](`PoolGateInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            uniV4: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<PoolGateInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, uniV4);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            uniV4: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall { uniV4 },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> PoolGateInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolGateInstance<T, P, N> {
            PoolGateInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolGateInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`__addLiquidity`] function.
        pub fn __addLiquidity(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            sender: alloy::sol_types::private::Address,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, __addLiquidityCall, N> {
            self.call_builder(
                &__addLiquidityCall {
                    asset0,
                    asset1,
                    sender,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`__initializePool`] function.
        pub fn __initializePool(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            initialSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, __initializePoolCall, N> {
            self.call_builder(
                &__initializePoolCall {
                    asset0,
                    asset1,
                    initialSqrtPriceX96,
                },
            )
        }
        ///Creates a new call builder for the [`__mint`] function.
        pub fn __mint(
            &self,
            to: alloy::sol_types::private::Address,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, __mintCall, N> {
            self.call_builder(&__mintCall { to, asset, amount })
        }
        ///Creates a new call builder for the [`__removeLiquidity`] function.
        pub fn __removeLiquidity(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            sender: alloy::sol_types::private::Address,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, __removeLiquidityCall, N> {
            self.call_builder(
                &__removeLiquidityCall {
                    asset0,
                    asset1,
                    sender,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`__swap`] function.
        pub fn __swap(
            &self,
            assetIn: alloy::sol_types::private::Address,
            assetOut: alloy::sol_types::private::Address,
            amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
            sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, __swapCall, N> {
            self.call_builder(
                &__swapCall {
                    assetIn,
                    assetOut,
                    amountSpecified,
                    sqrtPriceLimitX96,
                },
            )
        }
        ///Creates a new call builder for the [`addLiquidity`] function.
        pub fn addLiquidity(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, addLiquidityCall, N> {
            self.call_builder(
                &addLiquidityCall {
                    asset0,
                    asset1,
                    tickLower,
                    tickUpper,
                    liquidity,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`hook`] function.
        pub fn hook(&self) -> alloy_contract::SolCallBuilder<T, &P, hookCall, N> {
            self.call_builder(&hookCall {})
        }
        ///Creates a new call builder for the [`initializePool`] function.
        pub fn initializePool(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            initialSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializePoolCall, N> {
            self.call_builder(
                &initializePoolCall {
                    asset0,
                    asset1,
                    initialSqrtPriceX96,
                },
            )
        }
        ///Creates a new call builder for the [`isInitialized`] function.
        pub fn isInitialized(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isInitializedCall, N> {
            self.call_builder(
                &isInitializedCall {
                    asset0,
                    asset1,
                },
            )
        }
        ///Creates a new call builder for the [`mint_0`] function.
        pub fn mint_0(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mint_0Call, N> {
            self.call_builder(&mint_0Call { asset, amount })
        }
        ///Creates a new call builder for the [`mint_1`] function.
        pub fn mint_1(
            &self,
            to: alloy::sol_types::private::Address,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mint_1Call, N> {
            self.call_builder(&mint_1Call { to, asset, amount })
        }
        ///Creates a new call builder for the [`removeLiquidity`] function.
        pub fn removeLiquidity(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            tickLower: alloy::sol_types::private::primitives::aliases::I24,
            tickUpper: alloy::sol_types::private::primitives::aliases::I24,
            liquidity: alloy::sol_types::private::primitives::aliases::U256,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeLiquidityCall, N> {
            self.call_builder(
                &removeLiquidityCall {
                    asset0,
                    asset1,
                    tickLower,
                    tickUpper,
                    liquidity,
                    salt,
                },
            )
        }
        ///Creates a new call builder for the [`setHook`] function.
        pub fn setHook(
            &self,
            hook_: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setHookCall, N> {
            self.call_builder(&setHookCall { hook_ })
        }
        ///Creates a new call builder for the [`swap`] function.
        pub fn swap(
            &self,
            assetIn: alloy::sol_types::private::Address,
            assetOut: alloy::sol_types::private::Address,
            amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
            sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapCall, N> {
            self.call_builder(
                &swapCall {
                    assetIn,
                    assetOut,
                    amountSpecified,
                    sqrtPriceLimitX96,
                },
            )
        }
        ///Creates a new call builder for the [`tickSpacing`] function.
        pub fn tickSpacing(
            &self,
            spacing: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::SolCallBuilder<T, &P, tickSpacingCall, N> {
            self.call_builder(&tickSpacingCall { spacing })
        }
        ///Creates a new call builder for the [`unlockCallback`] function.
        pub fn unlockCallback(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockCallbackCall, N> {
            self.call_builder(&unlockCallbackCall { data })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolGateInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
