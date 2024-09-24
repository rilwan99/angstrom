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
    type PoolId is bytes32;

    error Overflow();

    constructor(address uniV4);

    function __addLiquidity(address asset0, address asset1, address sender, IPoolManager.ModifyLiquidityParams memory params) external returns (BalanceDelta callerDelta);
    function __initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96, uint16 storeIndex) external returns (PoolId);
    function __mint(address to, address asset, uint256 amount) external;
    function __removeLiquidity(address asset0, address asset1, address sender, IPoolManager.ModifyLiquidityParams memory params) external returns (BalanceDelta delta);
    function __swap(address assetIn, address assetOut, int256 amountSpecified, uint160 sqrtPriceLimitX96) external returns (BalanceDelta swapDelta);
    function addLiquidity(address asset0, address asset1, int24 tickLower, int24 tickUpper, uint256 liquidity, bytes32 salt) external returns (uint256 amount0, uint256 amount1);
    function hook() external view returns (address);
    function initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96, uint16 storeIndex) external returns (PoolId);
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
      },
      {
        "name": "storeIndex",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "PoolId"
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
      },
      {
        "name": "storeIndex",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "PoolId"
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
    ///0x60a06040526008805462ffffff60a01b1916600f60a21b1790553480156023575f80fd5b506040516127733803806127738339810160408190526040916050565b6001600160a01b0316608052607b565b5f60208284031215605f575f80fd5b81516001600160a01b03811681146074575f80fd5b9392505050565b6080516126706101035f395f81816103730152818161062e015281816109af01528181610c1a01528181610d6801528181610e1a015281816110b70152818161118c015281816112c7015281816114960152818161162f0152818161171f015281816118cf01528181611b2f01528181611c3d01528181611ca50152611d3e01526126705ff3fe608060405234801561000f575f80fd5b50600436106100fb575f3560e01c80636e1f5b99116100935780639f5e1a73116100635780639f5e1a73146102e7578063c6c3bbe6146102fa578063cf618a551461030d578063e4cb970b14610320575f80fd5b80636e1f5b99146102135780637f5a7c7b146102265780638a4c6af61461026b57806391dd7346146102c7575f80fd5b806330315f62116100ce57806330315f62146101735780633c4eb2e7146101965780633dfd3873146101a957806340c10f1914610200575f80fd5b8063034432c7146100ff57806312b4f4e6146101255780632974c8a4146101385780632bdfdbd114610160575b5f80fd5b61011261010d366004611eeb565b610333565b6040519081526020015b60405180910390f35b610112610133366004611f4a565b6104b9565b61014b610146366004611fd6565b61089a565b6040805192835260208301919091520161011c565b61011261016e366004611f4a565b610aa7565b610186610181366004612040565b61100e565b604051901515815260200161011c565b6101126101a4366004611eeb565b6110fe565b6101fe6101b7366004612077565b600880547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b005b6101fe61020e366004612099565b61127b565b6101126102213660046120c3565b61128a565b6008546102469073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161011c565b6101fe610279366004612108565b6008805462ffffff90921674010000000000000000000000000000000000000000027fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff909216919091179055565b6102da6102d5366004612123565b6112fe565b60405161011c91906121dd565b61014b6102f5366004611fd6565b611381565b6101fe6103083660046121ef565b61157e565b6101fe61031b3660046121ef565b6116ca565b61011261032e3660046120c3565b611784565b60405173ffffffffffffffffffffffffffffffffffffffff85811660248301528481166044830152838116606483015261ffff831660848301525f9182917f000000000000000000000000000000000000000000000000000000000000000016906348c89491903090633c4eb2e79060a4015b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815291815260208201805160e094851b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff909116179052519184901b7fffffffff000000000000000000000000000000000000000000000000000000001682526104399250906004016121dd565b5f604051808303815f875af1158015610454573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610499919081019061225a565b9050808060200190518101906104af919061234a565b9695505050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b94830194909452929093169083015290737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105ef576040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b1580156105d8575f80fd5b505af11580156105ea573d5f803e3d5ffd5b505050505b6040517f5a6bcfda0000000000000000000000000000000000000000000000000000000081525f9073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690635a6bcfda906106659085908890600401612361565b60408051808303815f875af1158015610680573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106a4919061243a565b90935090506106b38160801d90565b600f0b1580156106cd57506106c881600f0b90565b600f0b155b610738576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600d60248201527f47657474696e6720666565733f0000000000000000000000000000000000000060448201526064015b60405180910390fd5b5f6107438460801d90565b600f0b1315801561076057505f61075a84600f0b90565b600f0b13155b6107ec576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f67657474696e6720746f6b656e7320666f7220616464696e67206c697175696460448201527f6974790000000000000000000000000000000000000000000000000000000000606482015260840161072f565b6107f78787856119c4565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610890577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610879575f80fd5b505af115801561088b573d5f803e3d5ffd5b505050505b5050949350505050565b5f805f60405180608001604052808860020b81526020018760020b81526020016108c3876119e5565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f2bdfdbd10000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293505f927f0000000000000000000000000000000000000000000000000000000000000000909216916348c89491916109e4916004016121dd565b5f604051808303815f875af11580156109ff573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610a44919081019061225a565b90505f81806020019051810190610a5b919061234a565b9050610a678160801d90565b6fffffffffffffffffffffffffffffffff169450610a8581600f0b90565b6fffffffffffffffffffffffffffffffff169350505050965096945050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b94830194909452929093169083015290737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610bdd576040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b158015610bc6575f80fd5b505af1158015610bd8573d5f803e3d5ffd5b505050505b6040517f5a6bcfda00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690635a6bcfda90610c519084908790600401612361565b60408051808303815f875af1158015610c6c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c90919061243a565b506040805173ffffffffffffffffffffffffffffffffffffffff87811660208084018290528b8316848601528451808503860181526060850190955284519401939093206080830193909352881660a0820152919350905f9060c001604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529082905280516020909101207ff135baaa0000000000000000000000000000000000000000000000000000000082526004820184905291505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610dad573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd1919061234a565b6040517ff135baaa000000000000000000000000000000000000000000000000000000008152600481018490529091505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610e5f573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e83919061234a565b9050610ea7866fffffffffffffffffffffffffffffffff8316608085901b17611a46565b95505f610eb48760801d90565b600f0b12158015610ed157505f610ecb87600f0b90565b600f0b12155b610f5d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f6c6f73696e67206d6f6e657920666f722072656d6f76696e67206c697175696460448201527f6974790000000000000000000000000000000000000000000000000000000000606482015260840161072f565b610f688a8a886119c4565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611001577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610fea575f80fd5b505af1158015610ffc573d5f803e3d5ffd5b505050505b5050505050949350505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a811682528981169382019390935295860183905274010000000000000000000000000000000000000000870460020b9086015290941691830191909152812082906110dd9073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690611a95565b73ffffffffffffffffffffffffffffffffffffffff16151595945050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b948301949094529290931690830152907f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663695c5bf582868660f01b60405160200161120491907fffff00000000000000000000000000000000000000000000000000000000000091909116815260020190565b6040516020818303038152906040526040518463ffffffff1660e01b81526004016112319392919061245c565b6020604051808303815f875af115801561124d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611271919061250c565b5060a081206104af565b61128633838361157e565b5050565b60405173ffffffffffffffffffffffffffffffffffffffff858116602483015284811660448301526064820184905282811660848301525f9182917f000000000000000000000000000000000000000000000000000000000000000016906348c8949190309063e4cb970b9060a4016103a6565b60605f803073ffffffffffffffffffffffffffffffffffffffff168585604051611329929190612527565b5f604051808303815f865af19150503d805f8114611362576040519150601f19603f3d011682016040523d82523d5f602084013e611367565b606091505b50915091508161137957805160208201fd5b949350505050565b5f805f60405180608001604052808860020b81526020018760020b81526020016113aa87611ac2565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f12b4f4e60000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293505f927f0000000000000000000000000000000000000000000000000000000000000000909216916348c89491916114cb916004016121dd565b5f604051808303815f875af11580156114e6573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261152b919081019061225a565b90505f81806020019051810190611542919061234a565b905061154e8160801d90565b61155790612536565b6fffffffffffffffffffffffffffffffff16945061157581600f0b90565b610a8590612536565b6040805173ffffffffffffffffffffffffffffffffffffffff85811660248301528481166044830152606480830185905283518084039091018152608490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fcf618a550000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081527f0000000000000000000000000000000000000000000000000000000000000000909216916348c8949191611664916004016121dd565b5f604051808303815f875af115801561167f573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526116c4919081019061225a565b50505050565b6040517f156e29f600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8481166004830152602482018490526044820183905283917f00000000000000000000000000000000000000000000000000000000000000009091169063156e29f6906064015f604051808303815f87803b158015611762575f80fd5b505af1158015611774573d5f803e3d5ffd5b505050506116c483836001611b21565b5f73ffffffffffffffffffffffffffffffffffffffff80851690861610818161183b576008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8d811685528e8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b9382019390935291909216918101919091526118cb565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8e811685528d8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b9382019390935291909216918101919091525b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c82604051806060016040528086151581526020018981526020018873ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b815260040161195a929190612597565b6020604051808303815f875af1158015611976573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061199a919061234a565b92506119b2815f01516119ad8560801d90565b611d31565b61089081602001516119ad85600f0b90565b6119d2836119ad8360801d90565b6119e0826119ad83600f0b90565b505050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611a40576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b5f608082811d9084901d01600f83810b9085900b01611a8c611a6783611e55565b611a7083611e55565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b95945050505050565b5f81815260066020526040812061137973ffffffffffffffffffffffffffffffffffffffff851682611e8f565b5f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821115611b1d576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5090565b81156119e0578015611c00577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611b868573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611be9575f80fd5b505af1158015611bfb573d5f803e3d5ffd5b505050505b6040517f40c10f1900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081166004830152602482018490528416906340c10f19906044015f604051808303815f87803b158015611c8d575f80fd5b505af1158015611c9f573d5f803e3d5ffd5b505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611d0d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116c4919061234a565b5f81600f0b1315611e2a577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166380f0b44c611d958473ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526fffffffffffffffffffffffffffffffff841660248201526044015f604051808303815f87803b158015611e10575f80fd5b505af1158015611e22573d5f803e3d5ffd5b505050505050565b5f81600f0b12156112865761128682825f036fffffffffffffffffffffffffffffffff166001611b21565b80600f81900b8114611e8a57611e8a7f93dafdf100000000000000000000000000000000000000000000000000000000611ebf565b919050565b5f81602052631e2eaeaf5f5260205f6024601c865afa611eb65763535cf94b5f526004601cfd5b50505f51919050565b805f5260045ffd5b73ffffffffffffffffffffffffffffffffffffffff81168114611ee8575f80fd5b50565b5f805f8060808587031215611efe575f80fd5b8435611f0981611ec7565b93506020850135611f1981611ec7565b92506040850135611f2981611ec7565b9150606085013561ffff81168114611f3f575f80fd5b939692955090935050565b5f805f8084860360e0811215611f5e575f80fd5b8535611f6981611ec7565b94506020860135611f7981611ec7565b93506040860135611f8981611ec7565b925060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa082011215611fba575f80fd5b509295919450926060019150565b8060020b8114611ee8575f80fd5b5f805f805f8060c08789031215611feb575f80fd5b8635611ff681611ec7565b9550602087013561200681611ec7565b9450604087013561201681611fc8565b9350606087013561202681611fc8565b9598949750929560808101359460a0909101359350915050565b5f8060408385031215612051575f80fd5b823561205c81611ec7565b9150602083013561206c81611ec7565b809150509250929050565b5f60208284031215612087575f80fd5b813561209281611ec7565b9392505050565b5f80604083850312156120aa575f80fd5b82356120b581611ec7565b946020939093013593505050565b5f805f80608085870312156120d6575f80fd5b84356120e181611ec7565b935060208501356120f181611ec7565b9250604085013591506060850135611f3f81611ec7565b5f60208284031215612118575f80fd5b813561209281611fc8565b5f8060208385031215612134575f80fd5b823567ffffffffffffffff81111561214a575f80fd5b8301601f8101851361215a575f80fd5b803567ffffffffffffffff811115612170575f80fd5b856020828401011115612181575f80fd5b6020919091019590945092505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6120926020830184612191565b5f805f60608486031215612201575f80fd5b833561220c81611ec7565b9250602084013561221c81611ec7565b929592945050506040919091013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561226a575f80fd5b815167ffffffffffffffff811115612280575f80fd5b8201601f81018413612290575f80fd5b805167ffffffffffffffff8111156122aa576122aa61222d565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156123165761231661222d565b60405281815282820160200186101561232d575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f6020828403121561235a575f80fd5b5051919050565b6123e0818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b5f82356123ec81611fc8565b60020b60a0830152602083013561240281611fc8565b60020b60c083015250604082013560e082015260609091013561010082015261014061012082018190525f9082015261016001919050565b5f806040838503121561244b575f80fd5b505080516020909101519092909150565b6124db818573ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff831660a082015260e060c08201525f611a8c60e0830184612191565b5f6020828403121561251c575f80fd5b815161209281611fc8565b818382375f9101908152919050565b5f81600f0b7fffffffffffffffffffffffffffffffff80000000000000000000000000000000810361258f577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f0392915050565b612616818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8151151560a0820152602082015160c082015260409091015173ffffffffffffffffffffffffffffffffffffffff1660e082015261012061010082018190525f908201526101400191905056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R`\x08\x80Tb\xFF\xFF\xFF`\xA0\x1B\x19\x16`\x0F`\xA2\x1B\x17\x90U4\x80\x15`#W_\x80\xFD[P`@Qa's8\x03\x80a's\x839\x81\x01`@\x81\x90R`@\x91`PV[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`{V[_` \x82\x84\x03\x12\x15`_W_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`tW_\x80\xFD[\x93\x92PPPV[`\x80Qa&pa\x01\x03_9_\x81\x81a\x03s\x01R\x81\x81a\x06.\x01R\x81\x81a\t\xAF\x01R\x81\x81a\x0C\x1A\x01R\x81\x81a\rh\x01R\x81\x81a\x0E\x1A\x01R\x81\x81a\x10\xB7\x01R\x81\x81a\x11\x8C\x01R\x81\x81a\x12\xC7\x01R\x81\x81a\x14\x96\x01R\x81\x81a\x16/\x01R\x81\x81a\x17\x1F\x01R\x81\x81a\x18\xCF\x01R\x81\x81a\x1B/\x01R\x81\x81a\x1C=\x01R\x81\x81a\x1C\xA5\x01Ra\x1D>\x01Ra&p_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cn\x1F[\x99\x11a\0\x93W\x80c\x9F^\x1As\x11a\0cW\x80c\x9F^\x1As\x14a\x02\xE7W\x80c\xC6\xC3\xBB\xE6\x14a\x02\xFAW\x80c\xCFa\x8AU\x14a\x03\rW\x80c\xE4\xCB\x97\x0B\x14a\x03 W_\x80\xFD[\x80cn\x1F[\x99\x14a\x02\x13W\x80c\x7FZ|{\x14a\x02&W\x80c\x8ALj\xF6\x14a\x02kW\x80c\x91\xDDsF\x14a\x02\xC7W_\x80\xFD[\x80c01_b\x11a\0\xCEW\x80c01_b\x14a\x01sW\x80c<N\xB2\xE7\x14a\x01\x96W\x80c=\xFD8s\x14a\x01\xA9W\x80c@\xC1\x0F\x19\x14a\x02\0W_\x80\xFD[\x80c\x03D2\xC7\x14a\0\xFFW\x80c\x12\xB4\xF4\xE6\x14a\x01%W\x80c)t\xC8\xA4\x14a\x018W\x80c+\xDF\xDB\xD1\x14a\x01`W[_\x80\xFD[a\x01\x12a\x01\r6`\x04a\x1E\xEBV[a\x033V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x12a\x0136`\x04a\x1FJV[a\x04\xB9V[a\x01Ka\x01F6`\x04a\x1F\xD6V[a\x08\x9AV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1CV[a\x01\x12a\x01n6`\x04a\x1FJV[a\n\xA7V[a\x01\x86a\x01\x816`\x04a @V[a\x10\x0EV[`@Q\x90\x15\x15\x81R` \x01a\x01\x1CV[a\x01\x12a\x01\xA46`\x04a\x1E\xEBV[a\x10\xFEV[a\x01\xFEa\x01\xB76`\x04a wV[`\x08\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[a\x01\xFEa\x02\x0E6`\x04a \x99V[a\x12{V[a\x01\x12a\x02!6`\x04a \xC3V[a\x12\x8AV[`\x08Ta\x02F\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x01\xFEa\x02y6`\x04a!\x08V[`\x08\x80Tb\xFF\xFF\xFF\x90\x92\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xDAa\x02\xD56`\x04a!#V[a\x12\xFEV[`@Qa\x01\x1C\x91\x90a!\xDDV[a\x01Ka\x02\xF56`\x04a\x1F\xD6V[a\x13\x81V[a\x01\xFEa\x03\x086`\x04a!\xEFV[a\x15~V[a\x01\xFEa\x03\x1B6`\x04a!\xEFV[a\x16\xCAV[a\x01\x12a\x03.6`\x04a \xC3V[a\x17\x84V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R\x83\x81\x16`d\x83\x01Ra\xFF\xFF\x83\x16`\x84\x83\x01R_\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x900\x90c<N\xB2\xE7\x90`\xA4\x01[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x91\x81R` \x82\x01\x80Q`\xE0\x94\x85\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90RQ\x91\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82Ra\x049\x92P\x90`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04TW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04\x99\x91\x90\x81\x01\x90a\"ZV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x04\xAF\x91\x90a#JV[\x96\x95PPPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xEFW`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xD8W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xEAW=_\x80>=_\xFD[PPPP[`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZk\xCF\xDA\x90a\x06e\x90\x85\x90\x88\x90`\x04\x01a#aV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\x80W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA4\x91\x90a$:V[\x90\x93P\x90Pa\x06\xB3\x81`\x80\x1D\x90V[`\x0F\x0B\x15\x80\x15a\x06\xCDWPa\x06\xC8\x81`\x0F\x0B\x90V[`\x0F\x0B\x15[a\x078W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FGetting fees?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x07C\x84`\x80\x1D\x90V[`\x0F\x0B\x13\x15\x80\x15a\x07`WP_a\x07Z\x84`\x0F\x0B\x90V[`\x0F\x0B\x13\x15[a\x07\xECW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fgetting tokens for adding liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07/V[a\x07\xF7\x87\x87\x85a\x19\xC4V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\x90W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08yW_\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x8BW=_\x80>=_\xFD[PPPP[PP\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x08\xC3\x87a\x19\xE5V[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F+\xDF\xDB\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\t\xE4\x91`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xFFW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\nD\x91\x90\x81\x01\x90a\"ZV[\x90P_\x81\x80` \x01\x90Q\x81\x01\x90a\n[\x91\x90a#JV[\x90Pa\ng\x81`\x80\x1D\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\n\x85\x81`\x0F\x0B\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93PPPP\x96P\x96\x94PPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0B\xDDW`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xC6W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xD8W=_\x80>=_\xFD[PPPP[`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZk\xCF\xDA\x90a\x0CQ\x90\x84\x90\x87\x90`\x04\x01a#aV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0ClW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x90\x91\x90a$:V[P`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16` \x80\x84\x01\x82\x90R\x8B\x83\x16\x84\x86\x01R\x84Q\x80\x85\x03\x86\x01\x81R``\x85\x01\x90\x95R\x84Q\x94\x01\x93\x90\x93 `\x80\x83\x01\x93\x90\x93R\x88\x16`\xA0\x82\x01R\x91\x93P\x90_\x90`\xC0\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01\x84\x90R\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD1\x91\x90a#JV[`@Q\x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E_W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x83\x91\x90a#JV[\x90Pa\x0E\xA7\x86o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x85\x90\x1B\x17a\x1AFV[\x95P_a\x0E\xB4\x87`\x80\x1D\x90V[`\x0F\x0B\x12\x15\x80\x15a\x0E\xD1WP_a\x0E\xCB\x87`\x0F\x0B\x90V[`\x0F\x0B\x12\x15[a\x0F]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Flosing money for removing liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07/V[a\x0Fh\x8A\x8A\x88a\x19\xC4V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x10\x01W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xEAW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFCW=_\x80>=_\xFD[PPPP[PPPPP\x94\x93PPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x81\x16\x93\x82\x01\x93\x90\x93R\x95\x86\x01\x83\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x04`\x02\x0B\x90\x86\x01R\x90\x94\x16\x91\x83\x01\x91\x90\x91R\x81 \x82\x90a\x10\xDD\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x1A\x95V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x95\x94PPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ci\\[\xF5\x82\x86\x86`\xF0\x1B`@Q` \x01a\x12\x04\x91\x90\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90\x91\x16\x81R`\x02\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x121\x93\x92\x91\x90a$\\V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12MW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12q\x91\x90a%\x0CV[P`\xA0\x81 a\x04\xAFV[a\x12\x863\x83\x83a\x15~V[PPV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x82\x01\x84\x90R\x82\x81\x16`\x84\x83\x01R_\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x900\x90c\xE4\xCB\x97\x0B\x90`\xA4\x01a\x03\xA6V[``_\x800s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85`@Qa\x13)\x92\x91\x90a%'V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x13bW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13gV[``\x91P[P\x91P\x91P\x81a\x13yW\x80Q` \x82\x01\xFD[\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x13\xAA\x87a\x1A\xC2V[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x12\xB4\xF4\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x14\xCB\x91`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\xE6W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x15+\x91\x90\x81\x01\x90a\"ZV[\x90P_\x81\x80` \x01\x90Q\x81\x01\x90a\x15B\x91\x90a#JV[\x90Pa\x15N\x81`\x80\x1D\x90V[a\x15W\x90a%6V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\x15u\x81`\x0F\x0B\x90V[a\n\x85\x90a%6V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCFa\x8AU\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x16d\x91`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\x7FW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x16\xC4\x91\x90\x81\x01\x90a\"ZV[PPPPV[`@Q\x7F\x15n)\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`D\x82\x01\x83\x90R\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x15n)\xF6\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17bW_\x80\xFD[PZ\xF1\x15\x80\x15a\x17tW=_\x80>=_\xFD[PPPPa\x16\xC4\x83\x83`\x01a\x1B!V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x90\x86\x16\x10\x81\x81a\x18;W`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16\x85R\x8E\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91Ra\x18\xCBV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8E\x81\x16\x85R\x8D\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91R[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x82`@Q\x80``\x01`@R\x80\x86\x15\x15\x81R` \x01\x89\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19Z\x92\x91\x90a%\x97V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19vW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x9A\x91\x90a#JV[\x92Pa\x19\xB2\x81_\x01Qa\x19\xAD\x85`\x80\x1D\x90V[a\x1D1V[a\x08\x90\x81` \x01Qa\x19\xAD\x85`\x0F\x0B\x90V[a\x19\xD2\x83a\x19\xAD\x83`\x80\x1D\x90V[a\x19\xE0\x82a\x19\xAD\x83`\x0F\x0B\x90V[PPPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x1A@W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a\x1A\x8Ca\x1Ag\x83a\x1EUV[a\x1Ap\x83a\x1EUV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x95\x94PPPPPV[_\x81\x81R`\x06` R`@\x81 a\x13ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a\x1E\x8FV[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1B\x1DW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[\x81\x15a\x19\xE0W\x80\x15a\x1C\0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1B\x86\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xE9W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xFBW=_\x80>=_\xFD[PPPP[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x84\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\x8DW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\x9FW=_\x80>=_\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\rW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC4\x91\x90a#JV[_\x81`\x0F\x0B\x13\x15a\x1E*W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x80\xF0\xB4La\x1D\x95\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\x10W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\"W=_\x80>=_\xFD[PPPPPPV[_\x81`\x0F\x0B\x12\x15a\x12\x86Wa\x12\x86\x82\x82_\x03o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01a\x1B!V[\x80`\x0F\x81\x90\x0B\x81\x14a\x1E\x8AWa\x1E\x8A\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1E\xBFV[\x91\x90PV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa\x1E\xB6WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[\x80_R`\x04_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E\xE8W_\x80\xFD[PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x1E\xFEW_\x80\xFD[\x845a\x1F\t\x81a\x1E\xC7V[\x93P` \x85\x015a\x1F\x19\x81a\x1E\xC7V[\x92P`@\x85\x015a\x1F)\x81a\x1E\xC7V[\x91P``\x85\x015a\xFF\xFF\x81\x16\x81\x14a\x1F?W_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_\x80_\x80\x84\x86\x03`\xE0\x81\x12\x15a\x1F^W_\x80\xFD[\x855a\x1Fi\x81a\x1E\xC7V[\x94P` \x86\x015a\x1Fy\x81a\x1E\xC7V[\x93P`@\x86\x015a\x1F\x89\x81a\x1E\xC7V[\x92P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x12\x15a\x1F\xBAW_\x80\xFD[P\x92\x95\x91\x94P\x92``\x01\x91PV[\x80`\x02\x0B\x81\x14a\x1E\xE8W_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xEBW_\x80\xFD[\x865a\x1F\xF6\x81a\x1E\xC7V[\x95P` \x87\x015a \x06\x81a\x1E\xC7V[\x94P`@\x87\x015a \x16\x81a\x1F\xC8V[\x93P``\x87\x015a &\x81a\x1F\xC8V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_\x80`@\x83\x85\x03\x12\x15a QW_\x80\xFD[\x825a \\\x81a\x1E\xC7V[\x91P` \x83\x015a l\x81a\x1E\xC7V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a \x87W_\x80\xFD[\x815a \x92\x81a\x1E\xC7V[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a \xAAW_\x80\xFD[\x825a \xB5\x81a\x1E\xC7V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a \xD6W_\x80\xFD[\x845a \xE1\x81a\x1E\xC7V[\x93P` \x85\x015a \xF1\x81a\x1E\xC7V[\x92P`@\x85\x015\x91P``\x85\x015a\x1F?\x81a\x1E\xC7V[_` \x82\x84\x03\x12\x15a!\x18W_\x80\xFD[\x815a \x92\x81a\x1F\xC8V[_\x80` \x83\x85\x03\x12\x15a!4W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!JW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a!ZW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!pW_\x80\xFD[\x85` \x82\x84\x01\x01\x11\x15a!\x81W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a \x92` \x83\x01\x84a!\x91V[_\x80_``\x84\x86\x03\x12\x15a\"\x01W_\x80\xFD[\x835a\"\x0C\x81a\x1E\xC7V[\x92P` \x84\x015a\"\x1C\x81a\x1E\xC7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\"jW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x80W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\"\x90W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xAAWa\"\xAAa\"-V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a#\x16Wa#\x16a\"-V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a#-W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a#ZW_\x80\xFD[PQ\x91\x90PV[a#\xE0\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[_\x825a#\xEC\x81a\x1F\xC8V[`\x02\x0B`\xA0\x83\x01R` \x83\x015a$\x02\x81a\x1F\xC8V[`\x02\x0B`\xC0\x83\x01RP`@\x82\x015`\xE0\x82\x01R``\x90\x91\x015a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R_\x90\x82\x01Ra\x01`\x01\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a$KW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[a$\xDB\x81\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\xA0\x82\x01R`\xE0`\xC0\x82\x01R_a\x1A\x8C`\xE0\x83\x01\x84a!\x91V[_` \x82\x84\x03\x12\x15a%\x1CW_\x80\xFD[\x81Qa \x92\x81a\x1F\xC8V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_\x81`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x03a%\x8FW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x03\x92\x91PPV[a&\x16\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x81Q\x15\x15`\xA0\x82\x01R` \x82\x01Q`\xC0\x82\x01R`@\x90\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01\x81\x90R_\x90\x82\x01Ra\x01@\x01\x91\x90PV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100fb575f3560e01c80636e1f5b99116100935780639f5e1a73116100635780639f5e1a73146102e7578063c6c3bbe6146102fa578063cf618a551461030d578063e4cb970b14610320575f80fd5b80636e1f5b99146102135780637f5a7c7b146102265780638a4c6af61461026b57806391dd7346146102c7575f80fd5b806330315f62116100ce57806330315f62146101735780633c4eb2e7146101965780633dfd3873146101a957806340c10f1914610200575f80fd5b8063034432c7146100ff57806312b4f4e6146101255780632974c8a4146101385780632bdfdbd114610160575b5f80fd5b61011261010d366004611eeb565b610333565b6040519081526020015b60405180910390f35b610112610133366004611f4a565b6104b9565b61014b610146366004611fd6565b61089a565b6040805192835260208301919091520161011c565b61011261016e366004611f4a565b610aa7565b610186610181366004612040565b61100e565b604051901515815260200161011c565b6101126101a4366004611eeb565b6110fe565b6101fe6101b7366004612077565b600880547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b005b6101fe61020e366004612099565b61127b565b6101126102213660046120c3565b61128a565b6008546102469073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161011c565b6101fe610279366004612108565b6008805462ffffff90921674010000000000000000000000000000000000000000027fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff909216919091179055565b6102da6102d5366004612123565b6112fe565b60405161011c91906121dd565b61014b6102f5366004611fd6565b611381565b6101fe6103083660046121ef565b61157e565b6101fe61031b3660046121ef565b6116ca565b61011261032e3660046120c3565b611784565b60405173ffffffffffffffffffffffffffffffffffffffff85811660248301528481166044830152838116606483015261ffff831660848301525f9182917f000000000000000000000000000000000000000000000000000000000000000016906348c89491903090633c4eb2e79060a4015b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815291815260208201805160e094851b7bffffffffffffffffffffffffffffffffffffffffffffffffffffffff909116179052519184901b7fffffffff000000000000000000000000000000000000000000000000000000001682526104399250906004016121dd565b5f604051808303815f875af1158015610454573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610499919081019061225a565b9050808060200190518101906104af919061234a565b9695505050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b94830194909452929093169083015290737109709ecfa91a80626ff3989d68f67f5b1dd12d3b156105ef576040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b1580156105d8575f80fd5b505af11580156105ea573d5f803e3d5ffd5b505050505b6040517f5a6bcfda0000000000000000000000000000000000000000000000000000000081525f9073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690635a6bcfda906106659085908890600401612361565b60408051808303815f875af1158015610680573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106a4919061243a565b90935090506106b38160801d90565b600f0b1580156106cd57506106c881600f0b90565b600f0b155b610738576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600d60248201527f47657474696e6720666565733f0000000000000000000000000000000000000060448201526064015b60405180910390fd5b5f6107438460801d90565b600f0b1315801561076057505f61075a84600f0b90565b600f0b13155b6107ec576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f67657474696e6720746f6b656e7320666f7220616464696e67206c697175696460448201527f6974790000000000000000000000000000000000000000000000000000000000606482015260840161072f565b6107f78787856119c4565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610890577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610879575f80fd5b505af115801561088b573d5f803e3d5ffd5b505050505b5050949350505050565b5f805f60405180608001604052808860020b81526020018760020b81526020016108c3876119e5565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f2bdfdbd10000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293505f927f0000000000000000000000000000000000000000000000000000000000000000909216916348c89491916109e4916004016121dd565b5f604051808303815f875af11580156109ff573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610a44919081019061225a565b90505f81806020019051810190610a5b919061234a565b9050610a678160801d90565b6fffffffffffffffffffffffffffffffff169450610a8581600f0b90565b6fffffffffffffffffffffffffffffffff169350505050965096945050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b94830194909452929093169083015290737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15610bdd576040517f06447d5600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85166004820152737109709ecfa91a80626ff3989d68f67f5b1dd12d906306447d56906024015f604051808303815f87803b158015610bc6575f80fd5b505af1158015610bd8573d5f803e3d5ffd5b505050505b6040517f5a6bcfda00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690635a6bcfda90610c519084908790600401612361565b60408051808303815f875af1158015610c6c573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c90919061243a565b506040805173ffffffffffffffffffffffffffffffffffffffff87811660208084018290528b8316848601528451808503860181526060850190955284519401939093206080830193909352881660a0820152919350905f9060c001604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529082905280516020909101207ff135baaa0000000000000000000000000000000000000000000000000000000082526004820184905291505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610dad573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610dd1919061234a565b6040517ff135baaa000000000000000000000000000000000000000000000000000000008152600481018490529091505f9073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169063f135baaa90602401602060405180830381865afa158015610e5f573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e83919061234a565b9050610ea7866fffffffffffffffffffffffffffffffff8316608085901b17611a46565b95505f610eb48760801d90565b600f0b12158015610ed157505f610ecb87600f0b90565b600f0b12155b610f5d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602360248201527f6c6f73696e67206d6f6e657920666f722072656d6f76696e67206c697175696460448201527f6974790000000000000000000000000000000000000000000000000000000000606482015260840161072f565b610f688a8a886119c4565b737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611001577f885cb69240a935d632d79c317109709ecfa91a80626ff3989d68f67f5b1dd12d5f1c73ffffffffffffffffffffffffffffffffffffffff166390c5013b6040518163ffffffff1660e01b81526004015f604051808303815f87803b158015610fea575f80fd5b505af1158015610ffc573d5f803e3d5ffd5b505050505b5050505050949350505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a811682528981169382019390935295860183905274010000000000000000000000000000000000000000870460020b9086015290941691830191909152812082906110dd9073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690611a95565b73ffffffffffffffffffffffffffffffffffffffff16151595945050505050565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8b811685528a81169285019290925294830182905274010000000000000000000000000000000000000000860460020b948301949094529290931690830152907f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663695c5bf582868660f01b60405160200161120491907fffff00000000000000000000000000000000000000000000000000000000000091909116815260020190565b6040516020818303038152906040526040518463ffffffff1660e01b81526004016112319392919061245c565b6020604051808303815f875af115801561124d573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611271919061250c565b5060a081206104af565b61128633838361157e565b5050565b60405173ffffffffffffffffffffffffffffffffffffffff858116602483015284811660448301526064820184905282811660848301525f9182917f000000000000000000000000000000000000000000000000000000000000000016906348c8949190309063e4cb970b9060a4016103a6565b60605f803073ffffffffffffffffffffffffffffffffffffffff168585604051611329929190612527565b5f604051808303815f865af19150503d805f8114611362576040519150601f19603f3d011682016040523d82523d5f602084013e611367565b606091505b50915091508161137957805160208201fd5b949350505050565b5f805f60405180608001604052808860020b81526020018760020b81526020016113aa87611ac2565b815260209081018690526040805173ffffffffffffffffffffffffffffffffffffffff8d811660248301528c811660448301523360648301528451600290810b608484015285850151900b60a48301528483015160c4830152606085015160e48084019190915283518084039091018152610104909201835292810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f12b4f4e60000000000000000000000000000000000000000000000000000000017905290517f48c894910000000000000000000000000000000000000000000000000000000081529293505f927f0000000000000000000000000000000000000000000000000000000000000000909216916348c89491916114cb916004016121dd565b5f604051808303815f875af11580156114e6573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261152b919081019061225a565b90505f81806020019051810190611542919061234a565b905061154e8160801d90565b61155790612536565b6fffffffffffffffffffffffffffffffff16945061157581600f0b90565b610a8590612536565b6040805173ffffffffffffffffffffffffffffffffffffffff85811660248301528481166044830152606480830185905283518084039091018152608490920183526020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fcf618a550000000000000000000000000000000000000000000000000000000017905291517f48c894910000000000000000000000000000000000000000000000000000000081527f0000000000000000000000000000000000000000000000000000000000000000909216916348c8949191611664916004016121dd565b5f604051808303815f875af115801561167f573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526116c4919081019061225a565b50505050565b6040517f156e29f600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8481166004830152602482018490526044820183905283917f00000000000000000000000000000000000000000000000000000000000000009091169063156e29f6906064015f604051808303815f87803b158015611762575f80fd5b505af1158015611774573d5f803e3d5ffd5b505050506116c483836001611b21565b5f73ffffffffffffffffffffffffffffffffffffffff80851690861610818161183b576008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8d811685528e8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b9382019390935291909216918101919091526118cb565b6008546040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551938401865273ffffffffffffffffffffffffffffffffffffffff8e811685528d8116928501929092529483019190915274010000000000000000000000000000000000000000850460020b9382019390935291909216918101919091525b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c82604051806060016040528086151581526020018981526020018873ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b815260040161195a929190612597565b6020604051808303815f875af1158015611976573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061199a919061234a565b92506119b2815f01516119ad8560801d90565b611d31565b61089081602001516119ad85600f0b90565b6119d2836119ad8360801d90565b6119e0826119ad83600f0b90565b505050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611a40576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b5f608082811d9084901d01600f83810b9085900b01611a8c611a6783611e55565b611a7083611e55565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b95945050505050565b5f81815260066020526040812061137973ffffffffffffffffffffffffffffffffffffffff851682611e8f565b5f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff821115611b1d576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5090565b81156119e0578015611c00577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611b868573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611be9575f80fd5b505af1158015611bfb573d5f803e3d5ffd5b505050505b6040517f40c10f1900000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000081166004830152602482018490528416906340c10f19906044015f604051808303815f87803b158015611c8d575f80fd5b505af1158015611c9f573d5f803e3d5ffd5b505050507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611d0d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906116c4919061234a565b5f81600f0b1315611e2a577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166380f0b44c611d958473ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526fffffffffffffffffffffffffffffffff841660248201526044015f604051808303815f87803b158015611e10575f80fd5b505af1158015611e22573d5f803e3d5ffd5b505050505050565b5f81600f0b12156112865761128682825f036fffffffffffffffffffffffffffffffff166001611b21565b80600f81900b8114611e8a57611e8a7f93dafdf100000000000000000000000000000000000000000000000000000000611ebf565b919050565b5f81602052631e2eaeaf5f5260205f6024601c865afa611eb65763535cf94b5f526004601cfd5b50505f51919050565b805f5260045ffd5b73ffffffffffffffffffffffffffffffffffffffff81168114611ee8575f80fd5b50565b5f805f8060808587031215611efe575f80fd5b8435611f0981611ec7565b93506020850135611f1981611ec7565b92506040850135611f2981611ec7565b9150606085013561ffff81168114611f3f575f80fd5b939692955090935050565b5f805f8084860360e0811215611f5e575f80fd5b8535611f6981611ec7565b94506020860135611f7981611ec7565b93506040860135611f8981611ec7565b925060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa082011215611fba575f80fd5b509295919450926060019150565b8060020b8114611ee8575f80fd5b5f805f805f8060c08789031215611feb575f80fd5b8635611ff681611ec7565b9550602087013561200681611ec7565b9450604087013561201681611fc8565b9350606087013561202681611fc8565b9598949750929560808101359460a0909101359350915050565b5f8060408385031215612051575f80fd5b823561205c81611ec7565b9150602083013561206c81611ec7565b809150509250929050565b5f60208284031215612087575f80fd5b813561209281611ec7565b9392505050565b5f80604083850312156120aa575f80fd5b82356120b581611ec7565b946020939093013593505050565b5f805f80608085870312156120d6575f80fd5b84356120e181611ec7565b935060208501356120f181611ec7565b9250604085013591506060850135611f3f81611ec7565b5f60208284031215612118575f80fd5b813561209281611fc8565b5f8060208385031215612134575f80fd5b823567ffffffffffffffff81111561214a575f80fd5b8301601f8101851361215a575f80fd5b803567ffffffffffffffff811115612170575f80fd5b856020828401011115612181575f80fd5b6020919091019590945092505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6120926020830184612191565b5f805f60608486031215612201575f80fd5b833561220c81611ec7565b9250602084013561221c81611ec7565b929592945050506040919091013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561226a575f80fd5b815167ffffffffffffffff811115612280575f80fd5b8201601f81018413612290575f80fd5b805167ffffffffffffffff8111156122aa576122aa61222d565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156123165761231661222d565b60405281815282820160200186101561232d575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f6020828403121561235a575f80fd5b5051919050565b6123e0818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b5f82356123ec81611fc8565b60020b60a0830152602083013561240281611fc8565b60020b60c083015250604082013560e082015260609091013561010082015261014061012082018190525f9082015261016001919050565b5f806040838503121561244b575f80fd5b505080516020909101519092909150565b6124db818573ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff831660a082015260e060c08201525f611a8c60e0830184612191565b5f6020828403121561251c575f80fd5b815161209281611fc8565b818382375f9101908152919050565b5f81600f0b7fffffffffffffffffffffffffffffffff80000000000000000000000000000000810361258f577f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f0392915050565b612616818473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8151151560a0820152602082015160c082015260409091015173ffffffffffffffffffffffffffffffffffffffff1660e082015261012061010082018190525f908201526101400191905056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80cn\x1F[\x99\x11a\0\x93W\x80c\x9F^\x1As\x11a\0cW\x80c\x9F^\x1As\x14a\x02\xE7W\x80c\xC6\xC3\xBB\xE6\x14a\x02\xFAW\x80c\xCFa\x8AU\x14a\x03\rW\x80c\xE4\xCB\x97\x0B\x14a\x03 W_\x80\xFD[\x80cn\x1F[\x99\x14a\x02\x13W\x80c\x7FZ|{\x14a\x02&W\x80c\x8ALj\xF6\x14a\x02kW\x80c\x91\xDDsF\x14a\x02\xC7W_\x80\xFD[\x80c01_b\x11a\0\xCEW\x80c01_b\x14a\x01sW\x80c<N\xB2\xE7\x14a\x01\x96W\x80c=\xFD8s\x14a\x01\xA9W\x80c@\xC1\x0F\x19\x14a\x02\0W_\x80\xFD[\x80c\x03D2\xC7\x14a\0\xFFW\x80c\x12\xB4\xF4\xE6\x14a\x01%W\x80c)t\xC8\xA4\x14a\x018W\x80c+\xDF\xDB\xD1\x14a\x01`W[_\x80\xFD[a\x01\x12a\x01\r6`\x04a\x1E\xEBV[a\x033V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x12a\x0136`\x04a\x1FJV[a\x04\xB9V[a\x01Ka\x01F6`\x04a\x1F\xD6V[a\x08\x9AV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\x1CV[a\x01\x12a\x01n6`\x04a\x1FJV[a\n\xA7V[a\x01\x86a\x01\x816`\x04a @V[a\x10\x0EV[`@Q\x90\x15\x15\x81R` \x01a\x01\x1CV[a\x01\x12a\x01\xA46`\x04a\x1E\xEBV[a\x10\xFEV[a\x01\xFEa\x01\xB76`\x04a wV[`\x08\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[a\x01\xFEa\x02\x0E6`\x04a \x99V[a\x12{V[a\x01\x12a\x02!6`\x04a \xC3V[a\x12\x8AV[`\x08Ta\x02F\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x01\xFEa\x02y6`\x04a!\x08V[`\x08\x80Tb\xFF\xFF\xFF\x90\x92\x16t\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x02\xDAa\x02\xD56`\x04a!#V[a\x12\xFEV[`@Qa\x01\x1C\x91\x90a!\xDDV[a\x01Ka\x02\xF56`\x04a\x1F\xD6V[a\x13\x81V[a\x01\xFEa\x03\x086`\x04a!\xEFV[a\x15~V[a\x01\xFEa\x03\x1B6`\x04a!\xEFV[a\x16\xCAV[a\x01\x12a\x03.6`\x04a \xC3V[a\x17\x84V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R\x83\x81\x16`d\x83\x01Ra\xFF\xFF\x83\x16`\x84\x83\x01R_\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x900\x90c<N\xB2\xE7\x90`\xA4\x01[`@\x80Q\x80\x83\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x01\x81R\x91\x81R` \x82\x01\x80Q`\xE0\x94\x85\x1B{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90RQ\x91\x84\x90\x1B\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x82Ra\x049\x92P\x90`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x04TW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x04\x99\x91\x90\x81\x01\x90a\"ZV[\x90P\x80\x80` \x01\x90Q\x81\x01\x90a\x04\xAF\x91\x90a#JV[\x96\x95PPPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x05\xEFW`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x05\xD8W_\x80\xFD[PZ\xF1\x15\x80\x15a\x05\xEAW=_\x80>=_\xFD[PPPP[`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZk\xCF\xDA\x90a\x06e\x90\x85\x90\x88\x90`\x04\x01a#aV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x06\x80W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA4\x91\x90a$:V[\x90\x93P\x90Pa\x06\xB3\x81`\x80\x1D\x90V[`\x0F\x0B\x15\x80\x15a\x06\xCDWPa\x06\xC8\x81`\x0F\x0B\x90V[`\x0F\x0B\x15[a\x078W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\r`$\x82\x01R\x7FGetting fees?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[_a\x07C\x84`\x80\x1D\x90V[`\x0F\x0B\x13\x15\x80\x15a\x07`WP_a\x07Z\x84`\x0F\x0B\x90V[`\x0F\x0B\x13\x15[a\x07\xECW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Fgetting tokens for adding liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07/V[a\x07\xF7\x87\x87\x85a\x19\xC4V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x08\x90W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x08yW_\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x8BW=_\x80>=_\xFD[PPPP[PP\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x08\xC3\x87a\x19\xE5V[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F+\xDF\xDB\xD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\t\xE4\x91`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xFFW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\nD\x91\x90\x81\x01\x90a\"ZV[\x90P_\x81\x80` \x01\x90Q\x81\x01\x90a\n[\x91\x90a#JV[\x90Pa\ng\x81`\x80\x1D\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\n\x85\x81`\x0F\x0B\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x93PPPP\x96P\x96\x94PPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x0B\xDDW`@Q\x7F\x06D}V\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\x04\x82\x01Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\x06D}V\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xC6W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xD8W=_\x80>=_\xFD[PPPP[`@Q\x7FZk\xCF\xDA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cZk\xCF\xDA\x90a\x0CQ\x90\x84\x90\x87\x90`\x04\x01a#aV[`@\x80Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0ClW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x90\x91\x90a$:V[P`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x81\x16` \x80\x84\x01\x82\x90R\x8B\x83\x16\x84\x86\x01R\x84Q\x80\x85\x03\x86\x01\x81R``\x85\x01\x90\x95R\x84Q\x94\x01\x93\x90\x93 `\x80\x83\x01\x93\x90\x93R\x88\x16`\xA0\x82\x01R\x91\x93P\x90_\x90`\xC0\x01`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x90\x82\x90R\x80Q` \x90\x91\x01 \x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`\x04\x82\x01\x84\x90R\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xADW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xD1\x91\x90a#JV[`@Q\x7F\xF15\xBA\xAA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x84\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xF15\xBA\xAA\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E_W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\x83\x91\x90a#JV[\x90Pa\x0E\xA7\x86o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x85\x90\x1B\x17a\x1AFV[\x95P_a\x0E\xB4\x87`\x80\x1D\x90V[`\x0F\x0B\x12\x15\x80\x15a\x0E\xD1WP_a\x0E\xCB\x87`\x0F\x0B\x90V[`\x0F\x0B\x12\x15[a\x0F]W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7Flosing money for removing liquid`D\x82\x01R\x7Fity\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07/V[a\x0Fh\x8A\x8A\x88a\x19\xC4V[sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x10\x01W\x7F\x88\\\xB6\x92@\xA95\xD62\xD7\x9C1q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-_\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\xC5\x01;`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xEAW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFCW=_\x80>=_\xFD[PPPP[PPPPP\x94\x93PPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x81\x16\x93\x82\x01\x93\x90\x93R\x95\x86\x01\x83\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x04`\x02\x0B\x90\x86\x01R\x90\x94\x16\x91\x83\x01\x91\x90\x91R\x81 \x82\x90a\x10\xDD\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a\x1A\x95V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15\x15\x95\x94PPPPPV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x81\x16\x85R\x8A\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x82\x90Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x04`\x02\x0B\x94\x83\x01\x94\x90\x94R\x92\x90\x93\x16\x90\x83\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16ci\\[\xF5\x82\x86\x86`\xF0\x1B`@Q` \x01a\x12\x04\x91\x90\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90\x91\x16\x81R`\x02\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x121\x93\x92\x91\x90a$\\V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x12MW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12q\x91\x90a%\x0CV[P`\xA0\x81 a\x04\xAFV[a\x12\x863\x83\x83a\x15~V[PPV[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x82\x01\x84\x90R\x82\x81\x16`\x84\x83\x01R_\x91\x82\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x900\x90c\xE4\xCB\x97\x0B\x90`\xA4\x01a\x03\xA6V[``_\x800s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85`@Qa\x13)\x92\x91\x90a%'V[_`@Q\x80\x83\x03\x81_\x86Z\xF1\x91PP=\x80_\x81\x14a\x13bW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x13gV[``\x91P[P\x91P\x91P\x81a\x13yW\x80Q` \x82\x01\xFD[\x94\x93PPPPV[_\x80_`@Q\x80`\x80\x01`@R\x80\x88`\x02\x0B\x81R` \x01\x87`\x02\x0B\x81R` \x01a\x13\xAA\x87a\x1A\xC2V[\x81R` \x90\x81\x01\x86\x90R`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16`$\x83\x01R\x8C\x81\x16`D\x83\x01R3`d\x83\x01R\x84Q`\x02\x90\x81\x0B`\x84\x84\x01R\x85\x85\x01Q\x90\x0B`\xA4\x83\x01R\x84\x83\x01Q`\xC4\x83\x01R``\x85\x01Q`\xE4\x80\x84\x01\x91\x90\x91R\x83Q\x80\x84\x03\x90\x91\x01\x81Ra\x01\x04\x90\x92\x01\x83R\x92\x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x12\xB4\xF4\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x90Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x92\x93P_\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x14\xCB\x91`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\xE6W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x15+\x91\x90\x81\x01\x90a\"ZV[\x90P_\x81\x80` \x01\x90Q\x81\x01\x90a\x15B\x91\x90a#JV[\x90Pa\x15N\x81`\x80\x1D\x90V[a\x15W\x90a%6V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x94Pa\x15u\x81`\x0F\x0B\x90V[a\n\x85\x90a%6V[`@\x80Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x81\x16`$\x83\x01R\x84\x81\x16`D\x83\x01R`d\x80\x83\x01\x85\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`\x84\x90\x92\x01\x83R` \x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xCFa\x8AU\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R\x91Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x92\x16\x91cH\xC8\x94\x91\x91a\x16d\x91`\x04\x01a!\xDDV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\x7FW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x16\xC4\x91\x90\x81\x01\x90a\"ZV[PPPPV[`@Q\x7F\x15n)\xF6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`D\x82\x01\x83\x90R\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\x15n)\xF6\x90`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17bW_\x80\xFD[PZ\xF1\x15\x80\x15a\x17tW=_\x80>=_\xFD[PPPPa\x16\xC4\x83\x83`\x01a\x1B!V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16\x90\x86\x16\x10\x81\x81a\x18;W`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8D\x81\x16\x85R\x8E\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91Ra\x18\xCBV[`\x08T`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x93\x84\x01\x86Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8E\x81\x16\x85R\x8D\x81\x16\x92\x85\x01\x92\x90\x92R\x94\x83\x01\x91\x90\x91Rt\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x04`\x02\x0B\x93\x82\x01\x93\x90\x93R\x91\x90\x92\x16\x91\x81\x01\x91\x90\x91R[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x82`@Q\x80``\x01`@R\x80\x86\x15\x15\x81R` \x01\x89\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x19Z\x92\x91\x90a%\x97V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x19vW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x9A\x91\x90a#JV[\x92Pa\x19\xB2\x81_\x01Qa\x19\xAD\x85`\x80\x1D\x90V[a\x1D1V[a\x08\x90\x81` \x01Qa\x19\xAD\x85`\x0F\x0B\x90V[a\x19\xD2\x83a\x19\xAD\x83`\x80\x1D\x90V[a\x19\xE0\x82a\x19\xAD\x83`\x0F\x0B\x90V[PPPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x1A@W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a\x1A\x8Ca\x1Ag\x83a\x1EUV[a\x1Ap\x83a\x1EUV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x95\x94PPPPPV[_\x81\x81R`\x06` R`@\x81 a\x13ys\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a\x1E\x8FV[_\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x1B\x1DW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x90V[\x81\x15a\x19\xE0W\x80\x15a\x1C\0W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1B\x86\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1B\xE9W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xFBW=_\x80>=_\xFD[PPPP[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x84\x16\x90c@\xC1\x0F\x19\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1C\x8DW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1C\x9FW=_\x80>=_\xFD[PPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1D\rW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xC4\x91\x90a#JV[_\x81`\x0F\x0B\x13\x15a\x1E*W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x80\xF0\xB4La\x1D\x95\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\x10W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\"W=_\x80>=_\xFD[PPPPPPV[_\x81`\x0F\x0B\x12\x15a\x12\x86Wa\x12\x86\x82\x82_\x03o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01a\x1B!V[\x80`\x0F\x81\x90\x0B\x81\x14a\x1E\x8AWa\x1E\x8A\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1E\xBFV[\x91\x90PV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa\x1E\xB6WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[\x80_R`\x04_\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1E\xE8W_\x80\xFD[PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x1E\xFEW_\x80\xFD[\x845a\x1F\t\x81a\x1E\xC7V[\x93P` \x85\x015a\x1F\x19\x81a\x1E\xC7V[\x92P`@\x85\x015a\x1F)\x81a\x1E\xC7V[\x91P``\x85\x015a\xFF\xFF\x81\x16\x81\x14a\x1F?W_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_\x80_\x80\x84\x86\x03`\xE0\x81\x12\x15a\x1F^W_\x80\xFD[\x855a\x1Fi\x81a\x1E\xC7V[\x94P` \x86\x015a\x1Fy\x81a\x1E\xC7V[\x93P`@\x86\x015a\x1F\x89\x81a\x1E\xC7V[\x92P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x12\x15a\x1F\xBAW_\x80\xFD[P\x92\x95\x91\x94P\x92``\x01\x91PV[\x80`\x02\x0B\x81\x14a\x1E\xE8W_\x80\xFD[_\x80_\x80_\x80`\xC0\x87\x89\x03\x12\x15a\x1F\xEBW_\x80\xFD[\x865a\x1F\xF6\x81a\x1E\xC7V[\x95P` \x87\x015a \x06\x81a\x1E\xC7V[\x94P`@\x87\x015a \x16\x81a\x1F\xC8V[\x93P``\x87\x015a &\x81a\x1F\xC8V[\x95\x98\x94\x97P\x92\x95`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[_\x80`@\x83\x85\x03\x12\x15a QW_\x80\xFD[\x825a \\\x81a\x1E\xC7V[\x91P` \x83\x015a l\x81a\x1E\xC7V[\x80\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a \x87W_\x80\xFD[\x815a \x92\x81a\x1E\xC7V[\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a \xAAW_\x80\xFD[\x825a \xB5\x81a\x1E\xC7V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a \xD6W_\x80\xFD[\x845a \xE1\x81a\x1E\xC7V[\x93P` \x85\x015a \xF1\x81a\x1E\xC7V[\x92P`@\x85\x015\x91P``\x85\x015a\x1F?\x81a\x1E\xC7V[_` \x82\x84\x03\x12\x15a!\x18W_\x80\xFD[\x815a \x92\x81a\x1F\xC8V[_\x80` \x83\x85\x03\x12\x15a!4W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!JW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a!ZW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a!pW_\x80\xFD[\x85` \x82\x84\x01\x01\x11\x15a!\x81W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a \x92` \x83\x01\x84a!\x91V[_\x80_``\x84\x86\x03\x12\x15a\"\x01W_\x80\xFD[\x835a\"\x0C\x81a\x1E\xC7V[\x92P` \x84\x015a\"\x1C\x81a\x1E\xC7V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a\"jW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x80W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\"\x90W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\xAAWa\"\xAAa\"-V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a#\x16Wa#\x16a\"-V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a#-W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a#ZW_\x80\xFD[PQ\x91\x90PV[a#\xE0\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[_\x825a#\xEC\x81a\x1F\xC8V[`\x02\x0B`\xA0\x83\x01R` \x83\x015a$\x02\x81a\x1F\xC8V[`\x02\x0B`\xC0\x83\x01RP`@\x82\x015`\xE0\x82\x01R``\x90\x91\x015a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R_\x90\x82\x01Ra\x01`\x01\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a$KW_\x80\xFD[PP\x80Q` \x90\x91\x01Q\x90\x92\x90\x91PV[a$\xDB\x81\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\xA0\x82\x01R`\xE0`\xC0\x82\x01R_a\x1A\x8C`\xE0\x83\x01\x84a!\x91V[_` \x82\x84\x03\x12\x15a%\x1CW_\x80\xFD[\x81Qa \x92\x81a\x1F\xC8V[\x81\x83\x827_\x91\x01\x90\x81R\x91\x90PV[_\x81`\x0F\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x03a%\x8FW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_\x03\x92\x91PPV[a&\x16\x81\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x81Q\x15\x15`\xA0\x82\x01R` \x82\x01Q`\xC0\x82\x01R`@\x90\x91\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01\x81\x90R_\x90\x82\x01Ra\x01@\x01\x91\x90PV\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoolId(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PoolId>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl PoolId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::FixedBytes<32> {
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
        impl alloy_sol_types::SolType for PoolId {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
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
    /**Function with signature `__initializePool(address,address,uint160,uint16)` and selector `0x3c4eb2e7`.
```solidity
function __initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96, uint16 storeIndex) external returns (PoolId);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __initializePoolCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub initialSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        pub storeIndex: u16,
    }
    ///Container type for the return parameters of the [`__initializePool(address,address,uint160,uint16)`](__initializePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct __initializePoolReturn {
        pub _0: <PoolId as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U160,
                u16,
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
                    (
                        value.asset0,
                        value.asset1,
                        value.initialSqrtPriceX96,
                        value.storeIndex,
                    )
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
                        storeIndex: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (PoolId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for __initializePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for __initializePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = __initializePoolReturn;
            type ReturnTuple<'a> = (PoolId,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "__initializePool(address,address,uint160,uint16)";
            const SELECTOR: [u8; 4] = [60u8, 78u8, 178u8, 231u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.storeIndex),
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
    /**Function with signature `initializePool(address,address,uint160,uint16)` and selector `0x034432c7`.
```solidity
function initializePool(address asset0, address asset1, uint160 initialSqrtPriceX96, uint16 storeIndex) external returns (PoolId);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializePoolCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub initialSqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        pub storeIndex: u16,
    }
    ///Container type for the return parameters of the [`initializePool(address,address,uint160,uint16)`](initializePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializePoolReturn {
        pub _0: <PoolId as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U160,
                u16,
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
                    (
                        value.asset0,
                        value.asset1,
                        value.initialSqrtPriceX96,
                        value.storeIndex,
                    )
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
                        storeIndex: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (PoolId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializePoolReturn;
            type ReturnTuple<'a> = (PoolId,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializePool(address,address,uint160,uint16)";
            const SELECTOR: [u8; 4] = [3u8, 68u8, 50u8, 199u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.storeIndex),
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
            [3u8, 68u8, 50u8, 199u8],
            [18u8, 180u8, 244u8, 230u8],
            [41u8, 116u8, 200u8, 164u8],
            [43u8, 223u8, 219u8, 209u8],
            [48u8, 49u8, 95u8, 98u8],
            [60u8, 78u8, 178u8, 231u8],
            [61u8, 253u8, 56u8, 115u8],
            [64u8, 193u8, 15u8, 25u8],
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
            storeIndex: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, __initializePoolCall, N> {
            self.call_builder(
                &__initializePoolCall {
                    asset0,
                    asset1,
                    initialSqrtPriceX96,
                    storeIndex,
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
            storeIndex: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializePoolCall, N> {
            self.call_builder(
                &initializePoolCall {
                    asset0,
                    asset1,
                    initialSqrtPriceX96,
                    storeIndex,
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
