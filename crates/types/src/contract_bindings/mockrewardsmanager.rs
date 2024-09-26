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

interface MockRewardsManager {
    type Currency is address;
    type PoolId is bytes32;
    struct PoolKey {
        Currency currency0;
        Currency currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    error AssetAccessOutOfBounds(uint256 index, uint256 length);
    error AssetsOutOfOrderOrNotUnique();
    error AssetsUnsorted();
    error BundleChangeNetNegative(address asset);
    error FailedToDeployNewStore();
    error FeeAboveMax();
    error InvalidPoolKey();
    error InvalidTickSpacing();
    error MissingHookPermissions(uint160);
    error NoEntry();
    error NotController();
    error NotNode();
    error NotUniswap();
    error OnlyOncePerBlock();
    error OutOfOrderOrDuplicatePairs();
    error Overflow();
    error PairAccessOutOfBounds(uint256 index, uint256 length);
    error ReaderNotAtEnd();
    error Underflow();
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4PoolManager, address controller);

    function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeInitialize(address, PoolKey memory poolKey, uint160, bytes memory storeIndexBytes) external view returns (bytes4);
    function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 feeInE6) external;
    function consts() external pure returns (uint24 poolFee);
    function getGrowthInsideRange(PoolId id, int24 lowerTick, int24 upperTick) external view returns (uint256);
    function getGrowthInsideTick(PoolId id, int24 tick, int24 tickSpacing) external view returns (uint256);
    function toggleNodes(address[] memory nodes) external;
    function update(bytes memory encoded) external;
    function updateAfterTickMove(PoolId id, int24 lastTick, int24 newTick, int24 tickSpacing) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "uniV4PoolManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "controller",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "beforeAddLiquidity",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
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
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "beforeInitialize",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "poolKey",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "",
        "type": "uint160",
        "internalType": "uint160"
      },
      {
        "name": "storeIndexBytes",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "beforeRemoveLiquidity",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
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
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "configurePool",
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
        "name": "tickSpacing",
        "type": "uint16",
        "internalType": "uint16"
      },
      {
        "name": "feeInE6",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "consts",
    "inputs": [],
    "outputs": [
      {
        "name": "poolFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "getGrowthInsideRange",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "PoolId"
      },
      {
        "name": "lowerTick",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "upperTick",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getGrowthInsideTick",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "PoolId"
      },
      {
        "name": "tick",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "toggleNodes",
    "inputs": [
      {
        "name": "nodes",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "update",
    "inputs": [
      {
        "name": "encoded",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateAfterTickMove",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "internalType": "PoolId"
      },
      {
        "name": "lastTick",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "newTick",
        "type": "int24",
        "internalType": "int24"
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "error",
    "name": "AssetAccessOutOfBounds",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "AssetsOutOfOrderOrNotUnique",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AssetsUnsorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BundleChangeNetNegative",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "FailedToDeployNewStore",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FeeAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPoolKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTickSpacing",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MissingHookPermissions",
    "inputs": [
      {
        "name": "",
        "type": "uint160",
        "internalType": "uint160"
      }
    ]
  },
  {
    "type": "error",
    "name": "NoEntry",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotController",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotNode",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotUniswap",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyOncePerBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfOrderOrDuplicatePairs",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Overflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PairAccessOutOfBounds",
    "inputs": [
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "ReaderNotAtEnd",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Underflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WrongEndLiquidity",
    "inputs": [
      {
        "name": "endLiquidity",
        "type": "uint128",
        "internalType": "uint128"
      },
      {
        "name": "actualCurrentLiquidity",
        "type": "uint128",
        "internalType": "uint128"
      }
    ]
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod MockRewardsManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c060405234801561000f575f80fd5b50604051613c5c380380613c5c83398101604081905261002e91610161565b6001600160a01b03808316608052811660a05261004c610a00610080565b610057612080610080565b610079604051806060016040528060338152602001613c2960339139826100c6565b50506101d9565b806001600160a01b03168130166001600160a01b0316146100c357604051630ea7064560e31b81526001600160a01b038216600482015260240160405180910390fd5b50565b61010f82826040516024016100dc929190610192565b60408051601f198184030181529190526020810180516001600160e01b0390811663319af33360e01b1790915261011316565b5050565b6100c38161012660201b610f541760201c565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b80516001600160a01b038116811461015c575f80fd5b919050565b5f8060408385031215610172575f80fd5b61017b83610146565b915061018960208401610146565b90509250929050565b604081525f83518060408401528060208601606085015e5f60608285018101919091526001600160a01b03949094166020840152601f01601f191690910190910192915050565b60805160a0516139806102a95f395f81816103340152610e6c01525f818161023b015281816102ce0152818161041c01528181610532015281816105ae0152818161069d0152818161077e0152818161090501528181610bdd01528181610c3401528181611021015281816115ab0152818161161f0152818161168901528181611d3f01528181611d7b01528181611dbc01528181611e0001528181611e4c015281816124a80152818161272301528181612c4001528181612d7001528181612ef40152612f4f01526139805ff3fe608060405234801561000f575f80fd5b50600436106100b9575f3560e01c806362889dd611610072578063c43ed2c811610058578063c43ed2c814610188578063d6cffd1e1461019b578063d86d744e146101ae575f80fd5b806362889dd61461016257806371cca81b14610175575f80fd5b806321d0ee70116100a257806321d0ee70146100f8578063259982e51461013c5780633440d8201461014f575f80fd5b80630b3dd76e146100bd5780631090641d146100e3575b5f80fd5b6100d06100cb366004613192565b6101bc565b6040519081526020015b60405180910390f35b6100f66100f13660046131fe565b61031c565b005b61010b6101063660046132b6565b610403565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020016100da565b61010b61014a3660046132b6565b610684565b61010b61015d366004613355565b6108ec565b6100d0610170366004613192565b610b3c565b6100f66101833660046133af565b610c20565b6100f66101963660046133ee565b610c61565b6100f66101a936600461342d565b610e54565b6040515f81526020016100da565b5f6101de6040518060600160405280602381526020016138f860239139610f74565b61021e84846040518060400160405280600481526020017f7469636b00000000000000000000000000000000000000000000000000000000815250611006565b5f805b61026373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168787876110d5565b90925090508115610221576102ae86826040518060400160405280600a81526020017f6e6578745469636b557000000000000000000000000000000000000000000000815250611006565b6103126102fd6102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001689611147565b60a01c60020b90565b5f88815260066020526040902090878461117e565b9695505050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461038b576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6004546103bd9068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff168585858561121e565b600460086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610473576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6104818560400135611425565b90505f61048d87611466565b90505f8061050f838b6104a360208c018c61349e565b6104b360408d0160208e0161349e565b604080516006929092526003929092525f92835260608d810135602652603a600c2094845260208590526005835290922091905260081c7f10000000000000000000000000000000000000000000000000000000000000001791565b90925090505f6105586102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611147565b90505f6105918261056c60208d018d61349e565b61057c60408e0160208f0161349e565b5f89815260066020526040902092919061117e565b90505f6105d573ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687866114d4565b85549091505f906105f8846fffffffffffffffffffffffffffffffff851661152f565b61060291906134e4565b90506106208e8e5f01602081019061061a91906134f7565b8361155a565b61064f61062c896116f3565b6106369084613512565b84906fffffffffffffffffffffffffffffffff1661152f565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106f4576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408401355f61070387611466565b90505f610713602088018861349e565b90505f6107266040890160208a0161349e565b90505f60065f8581526020019081526020015f2090505f61075d858d86868e60600135600561147a9095949392919063ffffffff16565b5090505f6107a46102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611147565b90505f8362ffffff8716630100000081106107c1576107c161353a565b015490505f8462ffffff8716630100000081106107e0576107e061353a565b015490505f8760020b8460020b121561083357818310156108225781925082865f018962ffffff166301000000811061081b5761081b61353a565b0155610891565b61082c82846134e4565b9050610891565b8360020b8760020b13610870578282101561086657829150818662ffffff89166301000000811061081b5761081b61353a565b61082c83836134e4565b8183876301000000015461088491906134e4565b61088e91906134e4565b90505b5f61089c828c61152f565b905080865f015f8282546108b09190613567565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461095c576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61098261096d60208801886134f7565b61097d6040890160208a016134f7565b611718565b90505f6109c282610993868861357a565b60045468010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16919060f01c611770565b509050610a036040518060400160405280600c81526020017f476f7420706f6f6c206b65790000000000000000000000000000000000000000815250610f74565b610a416040518060400160405280601581526020017f436f6d706172696e67206b65792073706163696e670000000000000000000000815250610f74565b610a5c610a546080890160608a0161349e565b60020b6117f3565b610a688160020b6117f3565b610a9f6040518060600160405280602781526020016138d160279139610a9460608a0160408b016135e0565b62ffffff165f611884565b600281900b610ab46080890160608a0161349e565b60020b141580610ad857505f610ad06060890160408a016135e0565b62ffffff1614155b15610b0f576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d82000000000000000000000000000000000000000000000000000000000979650505050505050565b5f610b7d84846040518060400160405280600981526020017f6c6f7765725469636b0000000000000000000000000000000000000000000000815250611006565b610bbd84836040518060400160405280600981526020017f75707065725469636b0000000000000000000000000000000000000000000000815250611006565b610c18610c036102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687611147565b5f86815260066020526040902090858561117e565b949350505050565b5f848152600660205260409020610c5b90857f0000000000000000000000000000000000000000000000000000000000000000868686611917565b50505050565b610c9f6040518060400160405280600d81526020017f5570646174652063616c6c656400000000000000000000000000000000000000815250610f74565b815f610caa82611998565b60408051808201909152600e81527f417373657473206465636f64656400000000000000000000000000000000000060208201529193509150610cec90610f74565b5f610d1a8383600460089054906101000a900473ffffffffffffffffffffffffffffffffffffffff16611a69565b60408051808201909152600d81527f5061697273206465636f6465640000000000000000000000000000000000000060208201529194509150610d5c90610f74565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e08101829052610100810182905261014081019190915263f3cd914c81523060a082015261012080820152610dc38482600185611c04565b9350610e036040518060400160405280600c81526020017f506f6f6c20757064617465640000000000000000000000000000000000000000815250610f74565b610e0e848787611ec2565b610e4c6040518060400160405280601a81526020017f5475726e73206f75742077652772652061742074686520656e64000000000000815250610f74565b505050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610ec3576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b81811015610f4f575f838383818110610ee057610ee061353a565b9050602002016020810190610ef591906134f7565b73ffffffffffffffffffffffffffffffffffffffff165f90815260036020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610ec5565b505050565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b61100381604051602401610f889190613645565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052611ed9565b50565b5f61104873ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168585611ee2565b506fffffffffffffffffffffffffffffffff1690505f81118261106d8560020b611f47565b60405160200161107e92919061366e565b604051602081830303815290604052906110ce576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016110c59190613645565b60405180910390fd5b5050505050565b5f8080806110fa6110ef86880783138789050360016136d8565b600281900b60081d91565b909250905061112a8161112473ffffffffffffffffffffffffffffffffffffffff8b168a86611f52565b90611f96565b909450905061113a828287612058565b9250505094509492505050565b5f81815260066020526040812061117473ffffffffffffffffffffffffffffffffffffffff851682612082565b9150505b92915050565b5f808562ffffff85166301000000811061119a5761119a61353a565b015490505f8662ffffff8516630100000081106111b9576111b961353a565b015490508460020b8660020b12156111de576111d581836134e4565b92505050610c18565b8560020b8460020b136111f5576111d582826134e4565b8082886301000000015461120991906134e4565b61121391906134e4565b979650505050505050565b5f8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1611611284576040517f9f38afb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f036112c1576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff83161115611303576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6113238773ffffffffffffffffffffffffffffffffffffffff166120b2565b90505f6113308787611718565b90506040516020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156113a45783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008116870361139857828552506113a4565b5060208401935061135c565b9081526b600b380380600b5f395ff3008452821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff8516915061141a9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b5f80821315611460576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b604080516006939093526003939093525f938452602652603a600c209383526020849052938152606090912092905260089190911c7f10000000000000000000000000000000000000000000000000000000000000001791565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6115135763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f1904831182021561154a5763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b805f0361156657505050565b6040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a5841194906024015f604051808303815f87803b1580156115ec575f80fd5b505af11580156115fe573d5f803e3d5ffd5b506116449250505073ffffffffffffffffffffffffffffffffffffffff83167f0000000000000000000000000000000000000000000000000000000000000000836120d5565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af11580156116cf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c5b9190613719565b5f70010000000000000000000000000000000082106117145761171461211e565b5090565b5f6028838360405160200161175092919073ffffffffffffffffffffffffffffffffffffffff92831681529116602082015260400190565b60405160208183030381529060405280519060200120901b905092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008116851402806117df576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6110038160405160240161180991815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f4e0c1d1d00000000000000000000000000000000000000000000000000000000179052611ed9565b610f4f83838360405160240161189c93929190613730565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f969cdd0300000000000000000000000000000000000000000000000000000000179052611ed9565b8260020b8260020b131561195b578260020b61193f828460020b61212b90919063ffffffff16565b60020b13156119565761195686858786868661213c565b610e4c565b8260020b8260020b1215610e4c575f600284900b828107919091129082900503810260020b8260020b1215610e4c57610e4c8685878686866121cf565b6003818101915f918291803560e81c01018160446119b686846134e4565b6119c09190613754565b905080602086901b1792505f805b82811015611a5d575f6119ec602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611a53576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b91506001016119ce565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026611a8b8a846134e4565b611a959190613754565b905060405193508060c0028401925082604052808460201b179450505f5b82841015611bf75760048901983560e081901c905f90611adb906119e5908c9060f01c61226e565b90505f611aef6119e58c61ffff861661226e565b90508363ffffffff168363ffffffff16111580611b3857508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611b6f576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80611bae73ffffffffffffffffffffffffffffffffffffffff8c168585611770565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611ab3565b5093505050935093915050565b6001848101945f919035821a90611c20908690831615156122cf565b60408051808201909152600d81527f70616972496e6465783a20257300000000000000000000000000000000000000602082015260028701963560f01c90611c689082612320565b611c8a611c798561ffff84166123b5565b805160208201516040909201519092565b60020b608089015273ffffffffffffffffffffffffffffffffffffffff9081166040890152166020870190815260a090205f60108901893560801c60408051808201909152600c81527f616d6f756e74496e3a20257300000000000000000000000000000000000000006020820152919a506fffffffffffffffffffffffffffffffff169150611d1a9082612320565b5f8115611e2f575f611d656102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611147565b9050611d7083612415565b60e08b0152611d9f8a7f0000000000000000000000000000000000000000000000000000000000000000612470565b611de26102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611147565b60808b01515f868152600660205260409020919350611e29919086907f00000000000000000000000000000000000000000000000000000000000000009085908790611917565b50611e75565b611e726102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611147565b90505b5f611e9c6002871615155f86815260066020526040902060808d01518e919088908761248e565b60208c0151919c509150611eb2908a908361289c565b50999a9950505050505050505050565b808201808414610c5b576301842f8c5f526004601cfd5b61100381610f54565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa611f225763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b6060611178826128df565b5f8281526006602090815260408083208484526005019091528120611f8d73ffffffffffffffffffffffffffffffffffffffff861682612082565b95945050505050565b5f805f6120318460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826120475760ff61204e565b8360ff1681015b9150509250929050565b5f8160ff841661206e600187900b61010061378c565b61207891906136d8565b610c18919061378c565b5f81602052631e2eaeaf5f5260205f6024601c865afa6120a95763535cf94b5f526004601cfd5b50505f51919050565b5f611178602073ffffffffffffffffffffffffffffffffffffffff84163b613754565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716612115576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b5f8183071291819005919091030290565b5f61215f73ffffffffffffffffffffffffffffffffffffffff87168686856110d5565b94509050600284810b9084900b12156121785750610e4c565b80156121c9578662ffffff8516630100000081106121985761219861353a565b015487630100000001546121ac91906134e4565b8762ffffff8616630100000081106121c6576121c661353a565b01555b5061213c565b5f6121f273ffffffffffffffffffffffffffffffffffffffff8716868685612934565b94509050600283810b9085900b1361220a5750610e4c565b801561225b578662ffffff85166301000000811061222a5761222a61353a565b0154876301000000015461223e91906134e4565b8762ffffff8616630100000081106122585761225861353a565b01555b83612265816137ab565b945050506121cf565b5f8163ffffffff8416116122bd576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016110c5565b602083901c60448302015b9392505050565b80151560c0830152806122f65773fffd8963efd1fc6a506488495d951d5263988d256122fd565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b6123b18282604051602401612336929190613807565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9710a9d000000000000000000000000000000000000000000000000000000000179052611ed9565b5050565b5f8163ffffffff841611612404576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016110c5565b5060c08102602083901c0192915050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611460576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af180610f4f576040513d5f823e503d5ff35b5f8087156125305760108701963560801c6124fa816124e37f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1689612979565b6fffffffffffffffffffffffffffffffff166129aa565b876301000000015f82825461250f9190613567565b90915550889350506fffffffffffffffffffffffffffffffff169050612891565b60408051808201909152600d81527f73746172745469636b3a20257300000000000000000000000000000000000000602082015260038801975f9182913560e81d9061258090600283900b6129c5565b60408051808201909152600d81527f6c69717569646974793a20257300000000000000000000000000000000000000602082015260108b019a3560801c906125c89082612320565b5f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b131561261f5761261a8388878985612a56565b61262c565b61262c8388878985612e37565b60408051606081019091526035808252929d50909b5092975090935061265a919061393f6020830139610f74565b60408051808201909152601381527f646f6e617465546f43757272656e743a20257300000000000000000000000000602082015260108601953560801c906126a29082612320565b6126d0816fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff166129aa565b6126da908b613567565b99506126f86fffffffffffffffffffffffffffffffff821684613567565b92506127048686612fa4565b81515f906127499073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690612979565b905061279c6040518060400160405280601481526020017f63757272656e744c69717569646974793a202573000000000000000000000000815250826fffffffffffffffffffffffffffffffff16612320565b6127ed6040518060400160405280601081526020017f656e644c69717569646974793a202573000000000000000000000000000000008152508b6fffffffffffffffffffffffffffffffff16612320565b806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1614612866576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c166004830152821660248201526044016110c5565b8a856301000000015f82825461287c9190613567565b90915550969c50929a50505050505050505050505b965096945050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f9081526020849052604081206128d76128ce825c85612fdd565b92508183612ff5565b509392505050565b60605f82126128f15761117882612ffc565b6128fe8219600101612ffc565b8051602d82526001017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90910190815292915050565b5f8080806129498587078213868805036110ef565b909250905061112a8161297373ffffffffffffffffffffffffffffffffffffffff8b168a86611f52565b9061305c565b5f8181526006602052604081205f611f8d73ffffffffffffffffffffffffffffffffffffffff861660038401612082565b5f6122c8826129c1670de0b6b3a764000086613828565b0490565b6123b182826040516024016129db929190613807565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167faf7faa3800000000000000000000000000000000000000000000000000000000179052611ed9565b5f805f805f600190505f80612a9f6040518060400160405280600f81526020017f696e205f72657761726441626f76650000000000000000000000000000000000815250610f74565b612af06040518060400160405280601381526020017f7374617274206c69717569646974793a202573000000000000000000000000008152508a6fffffffffffffffffffffffffffffffff16612320565b612b326040518060400160405280601981526020017f547279696e6720746f20726577617264207469636b3a202573000000000000008152508c60020b6129c5565b8215612d1057612b766040518060400160405280601381526020017f5469636b20697320696e697469616c697a656400000000000000000000000000815250610f74565b60408051808201909152601581527f476f742072657761726420616d6f756e743a2025730000000000000000000000602082015260108b019a3560801c90612bbe9082612320565b612bc88184613567565b9250612be6818b6fffffffffffffffffffffffffffffffff166129aa565b612bf09083613567565b9150818d8d62ffffff1663010000008110612c0d57612c0d61353a565b015f828254612c1c9190613567565b909155505088515f90612c679073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611ee2565b915050612cac6040518060400160405280601081526020017f6e65744c69717569646974793a2025730000000000000000000000000000000081525082600f0b6129c5565b612cb68b82613124565b9a50612d096040518060400160405280601181526020017f6e6577206c69717569646974793a2025730000000000000000000000000000008152508c6fffffffffffffffffffffffffffffffff16612320565b5050612d4e565b612d4e6040518060400160405280601781526020017f5469636b206973206e6f7420696e697469616c697a6564000000000000000000815250610f74565b87516020890151612d989173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e9061313e565b809c508194505050876040015160020b8b60020b13612af057612df06040518060400160405280601a81526020017f446f6e6520726577617264696e672e2020546f74616c3a20257300000000000081525083612320565b612e2460405180606001604052806024815260200161391b602491398a6fffffffffffffffffffffffffffffffff16612320565b989b909a50979850959695505050505050565b5f805f805f600190505f80612e636040518060600160405280602381526020016138ae60239139610f74565b8215612f2d5760108a01993560801c612e7c8184613567565b9250612e9a818b6fffffffffffffffffffffffffffffffff166129aa565b612ea49083613567565b9150818d8d62ffffff1663010000008110612ec157612ec161353a565b015f828254612ed09190613567565b909155505088515f90612f1b9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611ee2565b915050612f288b82613158565b9a5050505b87516020890151612f779173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e906110d5565b809c508194505050876040015160020b8b60020b1315612e6357989b909a50979850959695505050505050565b8082146123b1576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808203828113156111785763c9654ed45f526004601cfd5b80825d5050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a9004806130155750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b5f805f8360ff0390505f6130fd8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083613112575f613119565b8160ff1681035b925050509250929050565b808203608081901c156111785763c9654ed45f526004601cfd5b5f8080806129496110ef60018789078413888a050361383f565b818101608081901c156111785763c9654ed45f526004601cfd5b61317a613880565b565b8035600281900b811461318d575f80fd5b919050565b5f805f606084860312156131a4575f80fd5b833592506131b46020850161317c565b91506131c26040850161317c565b90509250925092565b73ffffffffffffffffffffffffffffffffffffffff81168114611003575f80fd5b803562ffffff8116811461318d575f80fd5b5f805f8060808587031215613211575f80fd5b843561321c816131cb565b9350602085013561322c816131cb565b9250604085013561ffff81168114613242575f80fd5b9150613250606086016131ec565b905092959194509250565b5f60a0828403121561326b575f80fd5b50919050565b5f8083601f840112613281575f80fd5b50813567ffffffffffffffff811115613298575f80fd5b6020830191508360208285010111156132af575f80fd5b9250929050565b5f805f805f8587036101608112156132cc575f80fd5b86356132d7816131cb565b95506132e6886020890161325b565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215613317575f80fd5b5060c08601925061014086013567ffffffffffffffff811115613338575f80fd5b61334488828901613271565b969995985093965092949392505050565b5f805f805f610100868803121561336a575f80fd5b8535613375816131cb565b9450613384876020880161325b565b935060c0860135613394816131cb565b925060e086013567ffffffffffffffff811115613338575f80fd5b5f805f80608085870312156133c2575f80fd5b843593506133d26020860161317c565b92506133e06040860161317c565b91506132506060860161317c565b5f80602083850312156133ff575f80fd5b823567ffffffffffffffff811115613415575f80fd5b61342185828601613271565b90969095509350505050565b5f806020838503121561343e575f80fd5b823567ffffffffffffffff811115613454575f80fd5b8301601f81018513613464575f80fd5b803567ffffffffffffffff81111561347a575f80fd5b8560208260051b840101111561348e575f80fd5b6020919091019590945092505050565b5f602082840312156134ae575f80fd5b6122c88261317c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115611178576111786134b7565b5f60208284031215613507575f80fd5b81356122c8816131cb565b6fffffffffffffffffffffffffffffffff8281168282160390811115611178576111786134b7565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115611178576111786134b7565b80357fffff00000000000000000000000000000000000000000000000000000000000081169060028410156135d9577fffff000000000000000000000000000000000000000000000000000000000000808560020360031b1b82161691505b5092915050565b5f602082840312156135f0575f80fd5b6122c8826131ec565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6122c860208301846135f9565b5f81518060208401855e5f93019283525090919050565b5f6136798285613657565b7f205b00000000000000000000000000000000000000000000000000000000000081526136a96002820185613657565b7f5d206e6f7420696e697469616c697a6564000000000000000000000000000000815260110195945050505050565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715611178576111786134b7565b5f60208284031215613729575f80fd5b5051919050565b606081525f61374260608301866135f9565b60208301949094525060400152919050565b5f82613787577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f8260020b8260020b028060020b91508082146135d9576135d96134b7565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000081036137df576137df6134b7565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b604081525f61381960408301856135f9565b90508260208301529392505050565b8082028115828204841417611178576111786134b7565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715611178576111786134b7565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52605160045260245ffdfe696e205f72657761726442656c6f77206275742077652073686f756c646e2774206265436f6d706172696e67206b65792066656520257320746f2065787065637465642066656520257367657447726f777468496e736964655469636b20686173206265656e2063616c6c6564446f6e6520726577617264696e672e202046696e616c206c69717569646974793a202573446f6e6520726577617264696e672c20747279696e6720746f206765742072657761726420666f722063757272656e74207469636ba164736f6c634300081a000a72657761726473206d616e61676572206465706c6f796564207769746820636f6e74726f6c6c65722061646472657373202573
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa<\\8\x03\x80a<\\\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01aV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R\x81\x16`\xA0Ra\0La\n\0a\0\x80V[a\0Wa \x80a\0\x80V[a\0y`@Q\x80``\x01`@R\x80`3\x81R` \x01a<)`3\x919\x82a\0\xC6V[PPa\x01\xD9V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x810\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\0\xC3W`@Qc\x0E\xA7\x06E`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PV[a\x01\x0F\x82\x82`@Q`$\x01a\0\xDC\x92\x91\x90a\x01\x92V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16c1\x9A\xF33`\xE0\x1B\x17\x90\x91Ra\x01\x13\x16V[PPV[a\0\xC3\x81a\x01&` \x1Ba\x0FT\x17` \x1CV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\\W_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x01rW_\x80\xFD[a\x01{\x83a\x01FV[\x91Pa\x01\x89` \x84\x01a\x01FV[\x90P\x92P\x92\x90PV[`@\x81R_\x83Q\x80`@\x84\x01R\x80` \x86\x01``\x85\x01^_``\x82\x85\x01\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16` \x84\x01R`\x1F\x01`\x1F\x19\x16\x90\x91\x01\x90\x91\x01\x92\x91PPV[`\x80Q`\xA0Qa9\x80a\x02\xA9_9_\x81\x81a\x034\x01Ra\x0El\x01R_\x81\x81a\x02;\x01R\x81\x81a\x02\xCE\x01R\x81\x81a\x04\x1C\x01R\x81\x81a\x052\x01R\x81\x81a\x05\xAE\x01R\x81\x81a\x06\x9D\x01R\x81\x81a\x07~\x01R\x81\x81a\t\x05\x01R\x81\x81a\x0B\xDD\x01R\x81\x81a\x0C4\x01R\x81\x81a\x10!\x01R\x81\x81a\x15\xAB\x01R\x81\x81a\x16\x1F\x01R\x81\x81a\x16\x89\x01R\x81\x81a\x1D?\x01R\x81\x81a\x1D{\x01R\x81\x81a\x1D\xBC\x01R\x81\x81a\x1E\0\x01R\x81\x81a\x1EL\x01R\x81\x81a$\xA8\x01R\x81\x81a'#\x01R\x81\x81a,@\x01R\x81\x81a-p\x01R\x81\x81a.\xF4\x01Ra/O\x01Ra9\x80_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xB9W_5`\xE0\x1C\x80cb\x88\x9D\xD6\x11a\0rW\x80c\xC4>\xD2\xC8\x11a\0XW\x80c\xC4>\xD2\xC8\x14a\x01\x88W\x80c\xD6\xCF\xFD\x1E\x14a\x01\x9BW\x80c\xD8mtN\x14a\x01\xAEW_\x80\xFD[\x80cb\x88\x9D\xD6\x14a\x01bW\x80cq\xCC\xA8\x1B\x14a\x01uW_\x80\xFD[\x80c!\xD0\xEEp\x11a\0\xA2W\x80c!\xD0\xEEp\x14a\0\xF8W\x80c%\x99\x82\xE5\x14a\x01<W\x80c4@\xD8 \x14a\x01OW_\x80\xFD[\x80c\x0B=\xD7n\x14a\0\xBDW\x80c\x10\x90d\x1D\x14a\0\xE3W[_\x80\xFD[a\0\xD0a\0\xCB6`\x04a1\x92V[a\x01\xBCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF6a\0\xF16`\x04a1\xFEV[a\x03\x1CV[\0[a\x01\x0Ba\x01\x066`\x04a2\xB6V[a\x04\x03V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\0\xDAV[a\x01\x0Ba\x01J6`\x04a2\xB6V[a\x06\x84V[a\x01\x0Ba\x01]6`\x04a3UV[a\x08\xECV[a\0\xD0a\x01p6`\x04a1\x92V[a\x0B<V[a\0\xF6a\x01\x836`\x04a3\xAFV[a\x0C V[a\0\xF6a\x01\x966`\x04a3\xEEV[a\x0CaV[a\0\xF6a\x01\xA96`\x04a4-V[a\x0ETV[`@Q_\x81R` \x01a\0\xDAV[_a\x01\xDE`@Q\x80``\x01`@R\x80`#\x81R` \x01a8\xF8`#\x919a\x0FtV[a\x02\x1E\x84\x84`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[_\x80[a\x02cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x87\x87a\x10\xD5V[\x90\x92P\x90P\x81\x15a\x02!Wa\x02\xAE\x86\x82`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FnextTickUp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[a\x03\x12a\x02\xFDa\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x89a\x11GV[`\xA0\x1C`\x02\x0B\x90V[_\x88\x81R`\x06` R`@\x90 \x90\x87\x84a\x11~V[\x96\x95PPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\x8BW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04Ta\x03\xBD\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x12\x1EV[`\x04`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04sW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04\x81\x85`@\x015a\x14%V[\x90P_a\x04\x8D\x87a\x14fV[\x90P_\x80a\x05\x0F\x83\x8Ba\x04\xA3` \x8C\x01\x8Ca4\x9EV[a\x04\xB3`@\x8D\x01` \x8E\x01a4\x9EV[`@\x80Q`\x06\x92\x90\x92R`\x03\x92\x90\x92R_\x92\x83R``\x8D\x81\x015`&R`:`\x0C \x94\x84R` \x85\x90R`\x05\x83R\x90\x92 \x91\x90R`\x08\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[\x90\x92P\x90P_a\x05Xa\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11GV[\x90P_a\x05\x91\x82a\x05l` \x8D\x01\x8Da4\x9EV[a\x05|`@\x8E\x01` \x8F\x01a4\x9EV[_\x89\x81R`\x06` R`@\x90 \x92\x91\x90a\x11~V[\x90P_a\x05\xD5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x14\xD4V[\x85T\x90\x91P_\x90a\x05\xF8\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x15/V[a\x06\x02\x91\x90a4\xE4V[\x90Pa\x06 \x8E\x8E_\x01` \x81\x01\x90a\x06\x1A\x91\x90a4\xF7V[\x83a\x15ZV[a\x06Oa\x06,\x89a\x16\xF3V[a\x066\x90\x84a5\x12V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15/V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xF4W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x015_a\x07\x03\x87a\x14fV[\x90P_a\x07\x13` \x88\x01\x88a4\x9EV[\x90P_a\x07&`@\x89\x01` \x8A\x01a4\x9EV[\x90P_`\x06_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x07]\x85\x8D\x86\x86\x8E``\x015`\x05a\x14z\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x07\xA4a\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x11GV[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xC1Wa\x07\xC1a5:V[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xE0Wa\x07\xE0a5:V[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\x083W\x81\x83\x10\x15a\x08\"W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x08\x1BWa\x08\x1Ba5:V[\x01Ua\x08\x91V[a\x08,\x82\x84a4\xE4V[\x90Pa\x08\x91V[\x83`\x02\x0B\x87`\x02\x0B\x13a\x08pW\x82\x82\x10\x15a\x08fW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\x08\x1BWa\x08\x1Ba5:V[a\x08,\x83\x83a4\xE4V[\x81\x83\x87c\x01\0\0\0\x01Ta\x08\x84\x91\x90a4\xE4V[a\x08\x8E\x91\x90a4\xE4V[\x90P[_a\x08\x9C\x82\x8Ca\x15/V[\x90P\x80\x86_\x01_\x82\x82Ta\x08\xB0\x91\x90a5gV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\\W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\x82a\tm` \x88\x01\x88a4\xF7V[a\t}`@\x89\x01` \x8A\x01a4\xF7V[a\x17\x18V[\x90P_a\t\xC2\x82a\t\x93\x86\x88a5zV[`\x04Th\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90`\xF0\x1Ca\x17pV[P\x90Pa\n\x03`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FGot pool key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a\nA`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7FComparing key spacing\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a\n\\a\nT`\x80\x89\x01``\x8A\x01a4\x9EV[`\x02\x0Ba\x17\xF3V[a\nh\x81`\x02\x0Ba\x17\xF3V[a\n\x9F`@Q\x80``\x01`@R\x80`'\x81R` \x01a8\xD1`'\x919a\n\x94``\x8A\x01`@\x8B\x01a5\xE0V[b\xFF\xFF\xFF\x16_a\x18\x84V[`\x02\x81\x90\x0Ba\n\xB4`\x80\x89\x01``\x8A\x01a4\x9EV[`\x02\x0B\x14\x15\x80a\n\xD8WP_a\n\xD0``\x89\x01`@\x8A\x01a5\xE0V[b\xFF\xFF\xFF\x16\x14\x15[\x15a\x0B\x0FW`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x96PPPPPPPV[_a\x0B}\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FlowerTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[a\x0B\xBD\x84\x83`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FupperTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[a\x0C\x18a\x0C\x03a\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87a\x11GV[_\x86\x81R`\x06` R`@\x90 \x90\x85\x85a\x11~V[\x94\x93PPPPV[_\x84\x81R`\x06` R`@\x90 a\x0C[\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x86\x86a\x19\x17V[PPPPV[a\x0C\x9F`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FUpdate called\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[\x81_a\x0C\xAA\x82a\x19\x98V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81R\x7FAssets decoded\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x93P\x91Pa\x0C\xEC\x90a\x0FtV[_a\r\x1A\x83\x83`\x04`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1AiV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FPairs decoded\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x94P\x91Pa\r\\\x90a\x0FtV[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01Ra\r\xC3\x84\x82`\x01\x85a\x1C\x04V[\x93Pa\x0E\x03`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FPool updated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a\x0E\x0E\x84\x87\x87a\x1E\xC2V[a\x0EL`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FTurns out we're at the end\0\0\0\0\0\0\x81RPa\x0FtV[PPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0E\xC3W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x0FOW_\x83\x83\x83\x81\x81\x10a\x0E\xE0Wa\x0E\xE0a5:V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xF5\x91\x90a4\xF7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x03` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x0E\xC5V[PPPV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[a\x10\x03\x81`@Q`$\x01a\x0F\x88\x91\x90a6EV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[PV[_a\x10Hs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\x1E\xE2V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x11\x82a\x10m\x85`\x02\x0Ba\x1FGV[`@Q` \x01a\x10~\x92\x91\x90a6nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x10\xCEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xC5\x91\x90a6EV[`@Q\x80\x91\x03\x90\xFD[PPPPPV[_\x80\x80\x80a\x10\xFAa\x10\xEF\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a6\xD8V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa\x11*\x81a\x11$s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1FRV[\x90a\x1F\x96V[\x90\x94P\x90Pa\x11:\x82\x82\x87a XV[\x92PPP\x94P\x94\x92PPPV[_\x81\x81R`\x06` R`@\x81 a\x11ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a \x82V[\x91PP[\x92\x91PPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\x9AWa\x11\x9Aa5:V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xB9Wa\x11\xB9a5:V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x11\xDEWa\x11\xD5\x81\x83a4\xE4V[\x92PPPa\x0C\x18V[\x85`\x02\x0B\x84`\x02\x0B\x13a\x11\xF5Wa\x11\xD5\x82\x82a4\xE4V[\x80\x82\x88c\x01\0\0\0\x01Ta\x12\t\x91\x90a4\xE4V[a\x12\x13\x91\x90a4\xE4V[\x97\x96PPPPPPPV[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x12\x84W`@Q\x7F\x9F8\xAF\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x12\xC1W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x13\x03W`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13#\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a \xB2V[\x90P_a\x130\x87\x87a\x17\x18V[\x90P`@Q` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x13\xA4W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x13\x98W\x82\x85RPa\x13\xA4V[P` \x84\x01\x93Pa\x13\\V[\x90\x81Rk`\x0B8\x03\x80`\x0B_9_\xF3\0\x84R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x14\x1A\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[_\x80\x82\x13\x15a\x14`W`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`@\x80Q`\x06\x93\x90\x93R`\x03\x93\x90\x93R_\x93\x84R`&R`:`\x0C \x93\x83R` \x84\x90R\x93\x81R``\x90\x91 \x92\x90R`\x08\x91\x90\x91\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x15\x13WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x15JWc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x80_\x03a\x15fWPPPV[`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\xECW_\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xFEW=_\x80>=_\xFD[Pa\x16D\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a \xD5V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\xCFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C[\x91\x90a7\x19V[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x17\x14Wa\x17\x14a!\x1EV[P\x90V[_`(\x83\x83`@Q` \x01a\x17P\x92\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x1B\x90P\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x17\xDFW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[a\x10\x03\x81`@Q`$\x01a\x18\t\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FN\x0C\x1D\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[a\x0FO\x83\x83\x83`@Q`$\x01a\x18\x9C\x93\x92\x91\x90a70V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x96\x9C\xDD\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a\x19[W\x82`\x02\x0Ba\x19?\x82\x84`\x02\x0Ba!+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a\x19VWa\x19V\x86\x85\x87\x86\x86\x86a!<V[a\x0ELV[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a\x0ELW_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a\x0ELWa\x0EL\x86\x85\x87\x86\x86\x86a!\xCFV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x19\xB6\x86\x84a4\xE4V[a\x19\xC0\x91\x90a7TV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x1A]W_a\x19\xEC` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1ASW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x19\xCEV[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x1A\x8B\x8A\x84a4\xE4V[a\x1A\x95\x91\x90a7TV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x1B\xF7W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x1A\xDB\x90a\x19\xE5\x90\x8C\x90`\xF0\x1Ca\"nV[\x90P_a\x1A\xEFa\x19\xE5\x8Ca\xFF\xFF\x86\x16a\"nV[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x1B8WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1BoW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x1B\xAEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x17pV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x1A\xB3V[P\x93PPP\x93P\x93\x91PPV[`\x01\x84\x81\x01\x94_\x91\x905\x82\x1A\x90a\x1C \x90\x86\x90\x83\x16\x15\x15a\"\xCFV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FpairIndex: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x02\x87\x01\x965`\xF0\x1C\x90a\x1Ch\x90\x82a# V[a\x1C\x8Aa\x1Cy\x85a\xFF\xFF\x84\x16a#\xB5V[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x89\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x89\x01R\x16` \x87\x01\x90\x81R`\xA0\x90 _`\x10\x89\x01\x895`\x80\x1C`@\x80Q\x80\x82\x01\x90\x91R`\x0C\x81R\x7FamountIn: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x9APo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91Pa\x1D\x1A\x90\x82a# V[_\x81\x15a\x1E/W_a\x1Dea\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11GV[\x90Pa\x1Dp\x83a$\x15V[`\xE0\x8B\x01Ra\x1D\x9F\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$pV[a\x1D\xE2a\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11GV[`\x80\x8B\x01Q_\x86\x81R`\x06` R`@\x90 \x91\x93Pa\x1E)\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a\x19\x17V[Pa\x1EuV[a\x1Era\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x11GV[\x90P[_a\x1E\x9C`\x02\x87\x16\x15\x15_\x86\x81R`\x06` R`@\x90 `\x80\x8D\x01Q\x8E\x91\x90\x88\x90\x87a$\x8EV[` \x8C\x01Q\x91\x9CP\x91Pa\x1E\xB2\x90\x8A\x90\x83a(\x9CV[P\x99\x9A\x99PPPPPPPPPPV[\x80\x82\x01\x80\x84\x14a\x0C[Wc\x01\x84/\x8C_R`\x04`\x1C\xFD[a\x10\x03\x81a\x0FTV[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa\x1F\"WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[``a\x11x\x82a(\xDFV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a\x1F\x8Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a \x82V[\x95\x94PPPPPV[_\x80_a 1\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a GW`\xFFa NV[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a n`\x01\x87\x90\x0Ba\x01\0a7\x8CV[a x\x91\x90a6\xD8V[a\x0C\x18\x91\x90a7\x8CV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa \xA9WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[_a\x11x` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;a7TV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a!\x15Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a!_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a\x10\xD5V[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a!xWPa\x0ELV[\x80\x15a!\xC9W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a!\x98Wa!\x98a5:V[\x01T\x87c\x01\0\0\0\x01Ta!\xAC\x91\x90a4\xE4V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a!\xC6Wa!\xC6a5:V[\x01U[Pa!<V[_a!\xF2s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a)4V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a\"\nWPa\x0ELV[\x80\x15a\"[W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\"*Wa\"*a5:V[\x01T\x87c\x01\0\0\0\x01Ta\">\x91\x90a4\xE4V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\"XWa\"Xa5:V[\x01U[\x83a\"e\x81a7\xABV[\x94PPPa!\xCFV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a\"\xBDW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x10\xC5V[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[\x80\x15\x15`\xC0\x83\x01R\x80a\"\xF6Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\"\xFDV[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[a#\xB1\x82\x82`@Q`$\x01a#6\x92\x91\x90a8\x07V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97\x10\xA9\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[PPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a$\x04W`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x10\xC5V[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x14`W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x0FOW`@Q=_\x82>P=_\xF3[_\x80\x87\x15a%0W`\x10\x87\x01\x965`\x80\x1Ca$\xFA\x81a$\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a)yV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[\x87c\x01\0\0\0\x01_\x82\x82Ta%\x0F\x91\x90a5gV[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa(\x91V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FstartTick: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x03\x88\x01\x97_\x91\x82\x915`\xE8\x1D\x90a%\x80\x90`\x02\x83\x90\x0Ba)\xC5V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7Fliquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a%\xC8\x90\x82a# V[_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a&\x1FWa&\x1A\x83\x88\x87\x89\x85a*VV[a&,V[a&,\x83\x88\x87\x89\x85a.7V[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x92\x9DP\x90\x9BP\x92\x97P\x90\x93Pa&Z\x91\x90a9?` \x83\x019a\x0FtV[`@\x80Q\x80\x82\x01\x90\x91R`\x13\x81R\x7FdonateToCurrent: %s\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x86\x01\x955`\x80\x1C\x90a&\xA2\x90\x82a# V[a&\xD0\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[a&\xDA\x90\x8Ba5gV[\x99Pa&\xF8o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84a5gV[\x92Pa'\x04\x86\x86a/\xA4V[\x81Q_\x90a'I\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a)yV[\x90Pa'\x9C`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7FcurrentLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[a'\xED`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FendLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(fW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x10\xC5V[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta(|\x91\x90a5gV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a(\xD7a(\xCE\x82\\\x85a/\xDDV[\x92P\x81\x83a/\xF5V[P\x93\x92PPPV[``_\x82\x12a(\xF1Wa\x11x\x82a/\xFCV[a(\xFE\x82\x19`\x01\x01a/\xFCV[\x80Q`-\x82R`\x01\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x01\x90\x81R\x92\x91PPV[_\x80\x80\x80a)I\x85\x87\x07\x82\x13\x86\x88\x05\x03a\x10\xEFV[\x90\x92P\x90Pa\x11*\x81a)ss\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1FRV[\x90a0\\V[_\x81\x81R`\x06` R`@\x81 _a\x1F\x8Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a \x82V[_a\"\xC8\x82a)\xC1g\r\xE0\xB6\xB3\xA7d\0\0\x86a8(V[\x04\x90V[a#\xB1\x82\x82`@Q`$\x01a)\xDB\x92\x91\x90a8\x07V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xAF\x7F\xAA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[_\x80_\x80_`\x01\x90P_\x80a*\x9F`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7Fin _rewardAbove\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a*\xF0`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7Fstart liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[a+2`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FTrying to reward tick: %s\0\0\0\0\0\0\0\x81RP\x8C`\x02\x0Ba)\xC5V[\x82\x15a-\x10Wa+v`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7FTick is initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[`@\x80Q\x80\x82\x01\x90\x91R`\x15\x81R\x7FGot reward amount: %s\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a+\xBE\x90\x82a# V[a+\xC8\x81\x84a5gV[\x92Pa+\xE6\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[a+\xF0\x90\x83a5gV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a,\rWa,\ra5:V[\x01_\x82\x82Ta,\x1C\x91\x90a5gV[\x90\x91UPP\x88Q_\x90a,g\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1E\xE2V[\x91PPa,\xAC`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FnetLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82`\x0F\x0Ba)\xC5V[a,\xB6\x8B\x82a1$V[\x9APa-\t`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7Fnew liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Co\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[PPa-NV[a-N`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FTick is not initialized\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[\x87Q` \x89\x01Qa-\x98\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a1>V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a*\xF0Wa-\xF0`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FDone rewarding.  Total: %s\0\0\0\0\0\0\x81RP\x83a# V[a.$`@Q\x80``\x01`@R\x80`$\x81R` \x01a9\x1B`$\x919\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80_\x80_`\x01\x90P_\x80a.c`@Q\x80``\x01`@R\x80`#\x81R` \x01a8\xAE`#\x919a\x0FtV[\x82\x15a/-W`\x10\x8A\x01\x995`\x80\x1Ca.|\x81\x84a5gV[\x92Pa.\x9A\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[a.\xA4\x90\x83a5gV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a.\xC1Wa.\xC1a5:V[\x01_\x82\x82Ta.\xD0\x91\x90a5gV[\x90\x91UPP\x88Q_\x90a/\x1B\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1E\xE2V[\x91PPa/(\x8B\x82a1XV[\x9APPP[\x87Q` \x89\x01Qa/w\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a\x10\xD5V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a.cW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a#\xB1W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x11xWc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a0\x15WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[_\x80_\x83`\xFF\x03\x90P_a0\xFD\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a1\x12W_a1\x19V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x11xWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a)Ia\x10\xEF`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03a8?V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x11xWc\xC9eN\xD4_R`\x04`\x1C\xFD[a1za8\x80V[V[\x805`\x02\x81\x90\x0B\x81\x14a1\x8DW_\x80\xFD[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a1\xA4W_\x80\xFD[\x835\x92Pa1\xB4` \x85\x01a1|V[\x91Pa1\xC2`@\x85\x01a1|V[\x90P\x92P\x92P\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10\x03W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a1\x8DW_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a2\x11W_\x80\xFD[\x845a2\x1C\x81a1\xCBV[\x93P` \x85\x015a2,\x81a1\xCBV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14a2BW_\x80\xFD[\x91Pa2P``\x86\x01a1\xECV[\x90P\x92\x95\x91\x94P\x92PV[_`\xA0\x82\x84\x03\x12\x15a2kW_\x80\xFD[P\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a2\x81W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x98W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\xAFW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15a2\xCCW_\x80\xFD[\x865a2\xD7\x81a1\xCBV[\x95Pa2\xE6\x88` \x89\x01a2[V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a3\x17W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a38W_\x80\xFD[a3D\x88\x82\x89\x01a2qV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a3jW_\x80\xFD[\x855a3u\x81a1\xCBV[\x94Pa3\x84\x87` \x88\x01a2[V[\x93P`\xC0\x86\x015a3\x94\x81a1\xCBV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a38W_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a3\xC2W_\x80\xFD[\x845\x93Pa3\xD2` \x86\x01a1|V[\x92Pa3\xE0`@\x86\x01a1|V[\x91Pa2P``\x86\x01a1|V[_\x80` \x83\x85\x03\x12\x15a3\xFFW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\x15W_\x80\xFD[a4!\x85\x82\x86\x01a2qV[\x90\x96\x90\x95P\x93PPPPV[_\x80` \x83\x85\x03\x12\x15a4>W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4TW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a4dW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4zW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a4\x8EW_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a4\xAEW_\x80\xFD[a\"\xC8\x82a1|V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x11xWa\x11xa4\xB7V[_` \x82\x84\x03\x12\x15a5\x07W_\x80\xFD[\x815a\"\xC8\x81a1\xCBV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x11xWa\x11xa4\xB7V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x11xWa\x11xa4\xB7V[\x805\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x02\x84\x10\x15a5\xD9W\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x02\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_` \x82\x84\x03\x12\x15a5\xF0W_\x80\xFD[a\"\xC8\x82a1\xECV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\"\xC8` \x83\x01\x84a5\xF9V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a6y\x82\x85a6WV[\x7F [\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra6\xA9`\x02\x82\x01\x85a6WV[\x7F] not initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11\x01\x95\x94PPPPPV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x11xWa\x11xa4\xB7V[_` \x82\x84\x03\x12\x15a7)W_\x80\xFD[PQ\x91\x90PV[``\x81R_a7B``\x83\x01\x86a5\xF9V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82a7\x87W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a5\xD9Wa5\xD9a4\xB7V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a7\xDFWa7\xDFa4\xB7V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[`@\x81R_a8\x19`@\x83\x01\x85a5\xF9V[\x90P\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11xWa\x11xa4\xB7V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x11xWa\x11xa4\xB7V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`Q`\x04R`$_\xFD\xFEin _rewardBelow but we shouldn't beComparing key fee %s to expected fee %sgetGrowthInsideTick has been calledDone rewarding.  Final liquidity: %sDone rewarding, trying to get reward for current tick\xA1dsolcC\0\x08\x1A\0\nrewards manager deployed with controller address %s",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100b9575f3560e01c806362889dd611610072578063c43ed2c811610058578063c43ed2c814610188578063d6cffd1e1461019b578063d86d744e146101ae575f80fd5b806362889dd61461016257806371cca81b14610175575f80fd5b806321d0ee70116100a257806321d0ee70146100f8578063259982e51461013c5780633440d8201461014f575f80fd5b80630b3dd76e146100bd5780631090641d146100e3575b5f80fd5b6100d06100cb366004613192565b6101bc565b6040519081526020015b60405180910390f35b6100f66100f13660046131fe565b61031c565b005b61010b6101063660046132b6565b610403565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020016100da565b61010b61014a3660046132b6565b610684565b61010b61015d366004613355565b6108ec565b6100d0610170366004613192565b610b3c565b6100f66101833660046133af565b610c20565b6100f66101963660046133ee565b610c61565b6100f66101a936600461342d565b610e54565b6040515f81526020016100da565b5f6101de6040518060600160405280602381526020016138f860239139610f74565b61021e84846040518060400160405280600481526020017f7469636b00000000000000000000000000000000000000000000000000000000815250611006565b5f805b61026373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168787876110d5565b90925090508115610221576102ae86826040518060400160405280600a81526020017f6e6578745469636b557000000000000000000000000000000000000000000000815250611006565b6103126102fd6102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001689611147565b60a01c60020b90565b5f88815260066020526040902090878461117e565b9695505050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461038b576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6004546103bd9068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff168585858561121e565b600460086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610473576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6104818560400135611425565b90505f61048d87611466565b90505f8061050f838b6104a360208c018c61349e565b6104b360408d0160208e0161349e565b604080516006929092526003929092525f92835260608d810135602652603a600c2094845260208590526005835290922091905260081c7f10000000000000000000000000000000000000000000000000000000000000001791565b90925090505f6105586102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611147565b90505f6105918261056c60208d018d61349e565b61057c60408e0160208f0161349e565b5f89815260066020526040902092919061117e565b90505f6105d573ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687866114d4565b85549091505f906105f8846fffffffffffffffffffffffffffffffff851661152f565b61060291906134e4565b90506106208e8e5f01602081019061061a91906134f7565b8361155a565b61064f61062c896116f3565b6106369084613512565b84906fffffffffffffffffffffffffffffffff1661152f565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106f4576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408401355f61070387611466565b90505f610713602088018861349e565b90505f6107266040890160208a0161349e565b90505f60065f8581526020019081526020015f2090505f61075d858d86868e60600135600561147a9095949392919063ffffffff16565b5090505f6107a46102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611147565b90505f8362ffffff8716630100000081106107c1576107c161353a565b015490505f8462ffffff8716630100000081106107e0576107e061353a565b015490505f8760020b8460020b121561083357818310156108225781925082865f018962ffffff166301000000811061081b5761081b61353a565b0155610891565b61082c82846134e4565b9050610891565b8360020b8760020b13610870578282101561086657829150818662ffffff89166301000000811061081b5761081b61353a565b61082c83836134e4565b8183876301000000015461088491906134e4565b61088e91906134e4565b90505b5f61089c828c61152f565b905080865f015f8282546108b09190613567565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461095c576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61098261096d60208801886134f7565b61097d6040890160208a016134f7565b611718565b90505f6109c282610993868861357a565b60045468010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16919060f01c611770565b509050610a036040518060400160405280600c81526020017f476f7420706f6f6c206b65790000000000000000000000000000000000000000815250610f74565b610a416040518060400160405280601581526020017f436f6d706172696e67206b65792073706163696e670000000000000000000000815250610f74565b610a5c610a546080890160608a0161349e565b60020b6117f3565b610a688160020b6117f3565b610a9f6040518060600160405280602781526020016138d160279139610a9460608a0160408b016135e0565b62ffffff165f611884565b600281900b610ab46080890160608a0161349e565b60020b141580610ad857505f610ad06060890160408a016135e0565b62ffffff1614155b15610b0f576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d82000000000000000000000000000000000000000000000000000000000979650505050505050565b5f610b7d84846040518060400160405280600981526020017f6c6f7765725469636b0000000000000000000000000000000000000000000000815250611006565b610bbd84836040518060400160405280600981526020017f75707065725469636b0000000000000000000000000000000000000000000000815250611006565b610c18610c036102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687611147565b5f86815260066020526040902090858561117e565b949350505050565b5f848152600660205260409020610c5b90857f0000000000000000000000000000000000000000000000000000000000000000868686611917565b50505050565b610c9f6040518060400160405280600d81526020017f5570646174652063616c6c656400000000000000000000000000000000000000815250610f74565b815f610caa82611998565b60408051808201909152600e81527f417373657473206465636f64656400000000000000000000000000000000000060208201529193509150610cec90610f74565b5f610d1a8383600460089054906101000a900473ffffffffffffffffffffffffffffffffffffffff16611a69565b60408051808201909152600d81527f5061697273206465636f6465640000000000000000000000000000000000000060208201529194509150610d5c90610f74565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e08101829052610100810182905261014081019190915263f3cd914c81523060a082015261012080820152610dc38482600185611c04565b9350610e036040518060400160405280600c81526020017f506f6f6c20757064617465640000000000000000000000000000000000000000815250610f74565b610e0e848787611ec2565b610e4c6040518060400160405280601a81526020017f5475726e73206f75742077652772652061742074686520656e64000000000000815250610f74565b505050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610ec3576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b81811015610f4f575f838383818110610ee057610ee061353a565b9050602002016020810190610ef591906134f7565b73ffffffffffffffffffffffffffffffffffffffff165f90815260036020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610ec5565b505050565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b61100381604051602401610f889190613645565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052611ed9565b50565b5f61104873ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168585611ee2565b506fffffffffffffffffffffffffffffffff1690505f81118261106d8560020b611f47565b60405160200161107e92919061366e565b604051602081830303815290604052906110ce576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016110c59190613645565b60405180910390fd5b5050505050565b5f8080806110fa6110ef86880783138789050360016136d8565b600281900b60081d91565b909250905061112a8161112473ffffffffffffffffffffffffffffffffffffffff8b168a86611f52565b90611f96565b909450905061113a828287612058565b9250505094509492505050565b5f81815260066020526040812061117473ffffffffffffffffffffffffffffffffffffffff851682612082565b9150505b92915050565b5f808562ffffff85166301000000811061119a5761119a61353a565b015490505f8662ffffff8516630100000081106111b9576111b961353a565b015490508460020b8660020b12156111de576111d581836134e4565b92505050610c18565b8560020b8460020b136111f5576111d582826134e4565b8082886301000000015461120991906134e4565b61121391906134e4565b979650505050505050565b5f8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1611611284576040517f9f38afb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f036112c1576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff83161115611303576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6113238773ffffffffffffffffffffffffffffffffffffffff166120b2565b90505f6113308787611718565b90506040516020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156113a45783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008116870361139857828552506113a4565b5060208401935061135c565b9081526b600b380380600b5f395ff3008452821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff8516915061141a9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b5f80821315611460576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b604080516006939093526003939093525f938452602652603a600c209383526020849052938152606090912092905260089190911c7f10000000000000000000000000000000000000000000000000000000000000001791565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6115135763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f1904831182021561154a5763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b805f0361156657505050565b6040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a5841194906024015f604051808303815f87803b1580156115ec575f80fd5b505af11580156115fe573d5f803e3d5ffd5b506116449250505073ffffffffffffffffffffffffffffffffffffffff83167f0000000000000000000000000000000000000000000000000000000000000000836120d5565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af11580156116cf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610c5b9190613719565b5f70010000000000000000000000000000000082106117145761171461211e565b5090565b5f6028838360405160200161175092919073ffffffffffffffffffffffffffffffffffffffff92831681529116602082015260400190565b60405160208183030381529060405280519060200120901b905092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008116851402806117df576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6110038160405160240161180991815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f4e0c1d1d00000000000000000000000000000000000000000000000000000000179052611ed9565b610f4f83838360405160240161189c93929190613730565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f969cdd0300000000000000000000000000000000000000000000000000000000179052611ed9565b8260020b8260020b131561195b578260020b61193f828460020b61212b90919063ffffffff16565b60020b13156119565761195686858786868661213c565b610e4c565b8260020b8260020b1215610e4c575f600284900b828107919091129082900503810260020b8260020b1215610e4c57610e4c8685878686866121cf565b6003818101915f918291803560e81c01018160446119b686846134e4565b6119c09190613754565b905080602086901b1792505f805b82811015611a5d575f6119ec602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611a53576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b91506001016119ce565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026611a8b8a846134e4565b611a959190613754565b905060405193508060c0028401925082604052808460201b179450505f5b82841015611bf75760048901983560e081901c905f90611adb906119e5908c9060f01c61226e565b90505f611aef6119e58c61ffff861661226e565b90508363ffffffff168363ffffffff16111580611b3857508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611b6f576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80611bae73ffffffffffffffffffffffffffffffffffffffff8c168585611770565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611ab3565b5093505050935093915050565b6001848101945f919035821a90611c20908690831615156122cf565b60408051808201909152600d81527f70616972496e6465783a20257300000000000000000000000000000000000000602082015260028701963560f01c90611c689082612320565b611c8a611c798561ffff84166123b5565b805160208201516040909201519092565b60020b608089015273ffffffffffffffffffffffffffffffffffffffff9081166040890152166020870190815260a090205f60108901893560801c60408051808201909152600c81527f616d6f756e74496e3a20257300000000000000000000000000000000000000006020820152919a506fffffffffffffffffffffffffffffffff169150611d1a9082612320565b5f8115611e2f575f611d656102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611147565b9050611d7083612415565b60e08b0152611d9f8a7f0000000000000000000000000000000000000000000000000000000000000000612470565b611de26102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611147565b60808b01515f868152600660205260409020919350611e29919086907f00000000000000000000000000000000000000000000000000000000000000009085908790611917565b50611e75565b611e726102f473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611147565b90505b5f611e9c6002871615155f86815260066020526040902060808d01518e919088908761248e565b60208c0151919c509150611eb2908a908361289c565b50999a9950505050505050505050565b808201808414610c5b576301842f8c5f526004601cfd5b61100381610f54565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa611f225763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b6060611178826128df565b5f8281526006602090815260408083208484526005019091528120611f8d73ffffffffffffffffffffffffffffffffffffffff861682612082565b95945050505050565b5f805f6120318460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826120475760ff61204e565b8360ff1681015b9150509250929050565b5f8160ff841661206e600187900b61010061378c565b61207891906136d8565b610c18919061378c565b5f81602052631e2eaeaf5f5260205f6024601c865afa6120a95763535cf94b5f526004601cfd5b50505f51919050565b5f611178602073ffffffffffffffffffffffffffffffffffffffff84163b613754565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716612115576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b5f8183071291819005919091030290565b5f61215f73ffffffffffffffffffffffffffffffffffffffff87168686856110d5565b94509050600284810b9084900b12156121785750610e4c565b80156121c9578662ffffff8516630100000081106121985761219861353a565b015487630100000001546121ac91906134e4565b8762ffffff8616630100000081106121c6576121c661353a565b01555b5061213c565b5f6121f273ffffffffffffffffffffffffffffffffffffffff8716868685612934565b94509050600283810b9085900b1361220a5750610e4c565b801561225b578662ffffff85166301000000811061222a5761222a61353a565b0154876301000000015461223e91906134e4565b8762ffffff8616630100000081106122585761225861353a565b01555b83612265816137ab565b945050506121cf565b5f8163ffffffff8416116122bd576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016110c5565b602083901c60448302015b9392505050565b80151560c0830152806122f65773fffd8963efd1fc6a506488495d951d5263988d256122fd565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b6123b18282604051602401612336929190613807565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9710a9d000000000000000000000000000000000000000000000000000000000179052611ed9565b5050565b5f8163ffffffff841611612404576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016110c5565b5060c08102602083901c0192915050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611460576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af180610f4f576040513d5f823e503d5ff35b5f8087156125305760108701963560801c6124fa816124e37f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1689612979565b6fffffffffffffffffffffffffffffffff166129aa565b876301000000015f82825461250f9190613567565b90915550889350506fffffffffffffffffffffffffffffffff169050612891565b60408051808201909152600d81527f73746172745469636b3a20257300000000000000000000000000000000000000602082015260038801975f9182913560e81d9061258090600283900b6129c5565b60408051808201909152600d81527f6c69717569646974793a20257300000000000000000000000000000000000000602082015260108b019a3560801c906125c89082612320565b5f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b131561261f5761261a8388878985612a56565b61262c565b61262c8388878985612e37565b60408051606081019091526035808252929d50909b5092975090935061265a919061393f6020830139610f74565b60408051808201909152601381527f646f6e617465546f43757272656e743a20257300000000000000000000000000602082015260108601953560801c906126a29082612320565b6126d0816fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff166129aa565b6126da908b613567565b99506126f86fffffffffffffffffffffffffffffffff821684613567565b92506127048686612fa4565b81515f906127499073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690612979565b905061279c6040518060400160405280601481526020017f63757272656e744c69717569646974793a202573000000000000000000000000815250826fffffffffffffffffffffffffffffffff16612320565b6127ed6040518060400160405280601081526020017f656e644c69717569646974793a202573000000000000000000000000000000008152508b6fffffffffffffffffffffffffffffffff16612320565b806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1614612866576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c166004830152821660248201526044016110c5565b8a856301000000015f82825461287c9190613567565b90915550969c50929a50505050505050505050505b965096945050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f9081526020849052604081206128d76128ce825c85612fdd565b92508183612ff5565b509392505050565b60605f82126128f15761117882612ffc565b6128fe8219600101612ffc565b8051602d82526001017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90910190815292915050565b5f8080806129498587078213868805036110ef565b909250905061112a8161297373ffffffffffffffffffffffffffffffffffffffff8b168a86611f52565b9061305c565b5f8181526006602052604081205f611f8d73ffffffffffffffffffffffffffffffffffffffff861660038401612082565b5f6122c8826129c1670de0b6b3a764000086613828565b0490565b6123b182826040516024016129db929190613807565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167faf7faa3800000000000000000000000000000000000000000000000000000000179052611ed9565b5f805f805f600190505f80612a9f6040518060400160405280600f81526020017f696e205f72657761726441626f76650000000000000000000000000000000000815250610f74565b612af06040518060400160405280601381526020017f7374617274206c69717569646974793a202573000000000000000000000000008152508a6fffffffffffffffffffffffffffffffff16612320565b612b326040518060400160405280601981526020017f547279696e6720746f20726577617264207469636b3a202573000000000000008152508c60020b6129c5565b8215612d1057612b766040518060400160405280601381526020017f5469636b20697320696e697469616c697a656400000000000000000000000000815250610f74565b60408051808201909152601581527f476f742072657761726420616d6f756e743a2025730000000000000000000000602082015260108b019a3560801c90612bbe9082612320565b612bc88184613567565b9250612be6818b6fffffffffffffffffffffffffffffffff166129aa565b612bf09083613567565b9150818d8d62ffffff1663010000008110612c0d57612c0d61353a565b015f828254612c1c9190613567565b909155505088515f90612c679073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611ee2565b915050612cac6040518060400160405280601081526020017f6e65744c69717569646974793a2025730000000000000000000000000000000081525082600f0b6129c5565b612cb68b82613124565b9a50612d096040518060400160405280601181526020017f6e6577206c69717569646974793a2025730000000000000000000000000000008152508c6fffffffffffffffffffffffffffffffff16612320565b5050612d4e565b612d4e6040518060400160405280601781526020017f5469636b206973206e6f7420696e697469616c697a6564000000000000000000815250610f74565b87516020890151612d989173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e9061313e565b809c508194505050876040015160020b8b60020b13612af057612df06040518060400160405280601a81526020017f446f6e6520726577617264696e672e2020546f74616c3a20257300000000000081525083612320565b612e2460405180606001604052806024815260200161391b602491398a6fffffffffffffffffffffffffffffffff16612320565b989b909a50979850959695505050505050565b5f805f805f600190505f80612e636040518060600160405280602381526020016138ae60239139610f74565b8215612f2d5760108a01993560801c612e7c8184613567565b9250612e9a818b6fffffffffffffffffffffffffffffffff166129aa565b612ea49083613567565b9150818d8d62ffffff1663010000008110612ec157612ec161353a565b015f828254612ed09190613567565b909155505088515f90612f1b9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611ee2565b915050612f288b82613158565b9a5050505b87516020890151612f779173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e906110d5565b809c508194505050876040015160020b8b60020b1315612e6357989b909a50979850959695505050505050565b8082146123b1576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808203828113156111785763c9654ed45f526004601cfd5b80825d5050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a9004806130155750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b5f805f8360ff0390505f6130fd8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083613112575f613119565b8160ff1681035b925050509250929050565b808203608081901c156111785763c9654ed45f526004601cfd5b5f8080806129496110ef60018789078413888a050361383f565b818101608081901c156111785763c9654ed45f526004601cfd5b61317a613880565b565b8035600281900b811461318d575f80fd5b919050565b5f805f606084860312156131a4575f80fd5b833592506131b46020850161317c565b91506131c26040850161317c565b90509250925092565b73ffffffffffffffffffffffffffffffffffffffff81168114611003575f80fd5b803562ffffff8116811461318d575f80fd5b5f805f8060808587031215613211575f80fd5b843561321c816131cb565b9350602085013561322c816131cb565b9250604085013561ffff81168114613242575f80fd5b9150613250606086016131ec565b905092959194509250565b5f60a0828403121561326b575f80fd5b50919050565b5f8083601f840112613281575f80fd5b50813567ffffffffffffffff811115613298575f80fd5b6020830191508360208285010111156132af575f80fd5b9250929050565b5f805f805f8587036101608112156132cc575f80fd5b86356132d7816131cb565b95506132e6886020890161325b565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215613317575f80fd5b5060c08601925061014086013567ffffffffffffffff811115613338575f80fd5b61334488828901613271565b969995985093965092949392505050565b5f805f805f610100868803121561336a575f80fd5b8535613375816131cb565b9450613384876020880161325b565b935060c0860135613394816131cb565b925060e086013567ffffffffffffffff811115613338575f80fd5b5f805f80608085870312156133c2575f80fd5b843593506133d26020860161317c565b92506133e06040860161317c565b91506132506060860161317c565b5f80602083850312156133ff575f80fd5b823567ffffffffffffffff811115613415575f80fd5b61342185828601613271565b90969095509350505050565b5f806020838503121561343e575f80fd5b823567ffffffffffffffff811115613454575f80fd5b8301601f81018513613464575f80fd5b803567ffffffffffffffff81111561347a575f80fd5b8560208260051b840101111561348e575f80fd5b6020919091019590945092505050565b5f602082840312156134ae575f80fd5b6122c88261317c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115611178576111786134b7565b5f60208284031215613507575f80fd5b81356122c8816131cb565b6fffffffffffffffffffffffffffffffff8281168282160390811115611178576111786134b7565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115611178576111786134b7565b80357fffff00000000000000000000000000000000000000000000000000000000000081169060028410156135d9577fffff000000000000000000000000000000000000000000000000000000000000808560020360031b1b82161691505b5092915050565b5f602082840312156135f0575f80fd5b6122c8826131ec565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f6122c860208301846135f9565b5f81518060208401855e5f93019283525090919050565b5f6136798285613657565b7f205b00000000000000000000000000000000000000000000000000000000000081526136a96002820185613657565b7f5d206e6f7420696e697469616c697a6564000000000000000000000000000000815260110195945050505050565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715611178576111786134b7565b5f60208284031215613729575f80fd5b5051919050565b606081525f61374260608301866135f9565b60208301949094525060400152919050565b5f82613787577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f8260020b8260020b028060020b91508082146135d9576135d96134b7565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000081036137df576137df6134b7565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b604081525f61381960408301856135f9565b90508260208301529392505050565b8082028115828204841417611178576111786134b7565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715611178576111786134b7565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52605160045260245ffdfe696e205f72657761726442656c6f77206275742077652073686f756c646e2774206265436f6d706172696e67206b65792066656520257320746f2065787065637465642066656520257367657447726f777468496e736964655469636b20686173206265656e2063616c6c6564446f6e6520726577617264696e672e202046696e616c206c69717569646974793a202573446f6e6520726577617264696e672c20747279696e6720746f206765742072657761726420666f722063757272656e74207469636ba164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xB9W_5`\xE0\x1C\x80cb\x88\x9D\xD6\x11a\0rW\x80c\xC4>\xD2\xC8\x11a\0XW\x80c\xC4>\xD2\xC8\x14a\x01\x88W\x80c\xD6\xCF\xFD\x1E\x14a\x01\x9BW\x80c\xD8mtN\x14a\x01\xAEW_\x80\xFD[\x80cb\x88\x9D\xD6\x14a\x01bW\x80cq\xCC\xA8\x1B\x14a\x01uW_\x80\xFD[\x80c!\xD0\xEEp\x11a\0\xA2W\x80c!\xD0\xEEp\x14a\0\xF8W\x80c%\x99\x82\xE5\x14a\x01<W\x80c4@\xD8 \x14a\x01OW_\x80\xFD[\x80c\x0B=\xD7n\x14a\0\xBDW\x80c\x10\x90d\x1D\x14a\0\xE3W[_\x80\xFD[a\0\xD0a\0\xCB6`\x04a1\x92V[a\x01\xBCV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF6a\0\xF16`\x04a1\xFEV[a\x03\x1CV[\0[a\x01\x0Ba\x01\x066`\x04a2\xB6V[a\x04\x03V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\0\xDAV[a\x01\x0Ba\x01J6`\x04a2\xB6V[a\x06\x84V[a\x01\x0Ba\x01]6`\x04a3UV[a\x08\xECV[a\0\xD0a\x01p6`\x04a1\x92V[a\x0B<V[a\0\xF6a\x01\x836`\x04a3\xAFV[a\x0C V[a\0\xF6a\x01\x966`\x04a3\xEEV[a\x0CaV[a\0\xF6a\x01\xA96`\x04a4-V[a\x0ETV[`@Q_\x81R` \x01a\0\xDAV[_a\x01\xDE`@Q\x80``\x01`@R\x80`#\x81R` \x01a8\xF8`#\x919a\x0FtV[a\x02\x1E\x84\x84`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[_\x80[a\x02cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x87\x87a\x10\xD5V[\x90\x92P\x90P\x81\x15a\x02!Wa\x02\xAE\x86\x82`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FnextTickUp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[a\x03\x12a\x02\xFDa\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x89a\x11GV[`\xA0\x1C`\x02\x0B\x90V[_\x88\x81R`\x06` R`@\x90 \x90\x87\x84a\x11~V[\x96\x95PPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\x8BW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x04Ta\x03\xBD\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x12\x1EV[`\x04`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04sW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04\x81\x85`@\x015a\x14%V[\x90P_a\x04\x8D\x87a\x14fV[\x90P_\x80a\x05\x0F\x83\x8Ba\x04\xA3` \x8C\x01\x8Ca4\x9EV[a\x04\xB3`@\x8D\x01` \x8E\x01a4\x9EV[`@\x80Q`\x06\x92\x90\x92R`\x03\x92\x90\x92R_\x92\x83R``\x8D\x81\x015`&R`:`\x0C \x94\x84R` \x85\x90R`\x05\x83R\x90\x92 \x91\x90R`\x08\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[\x90\x92P\x90P_a\x05Xa\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11GV[\x90P_a\x05\x91\x82a\x05l` \x8D\x01\x8Da4\x9EV[a\x05|`@\x8E\x01` \x8F\x01a4\x9EV[_\x89\x81R`\x06` R`@\x90 \x92\x91\x90a\x11~V[\x90P_a\x05\xD5s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x14\xD4V[\x85T\x90\x91P_\x90a\x05\xF8\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x15/V[a\x06\x02\x91\x90a4\xE4V[\x90Pa\x06 \x8E\x8E_\x01` \x81\x01\x90a\x06\x1A\x91\x90a4\xF7V[\x83a\x15ZV[a\x06Oa\x06,\x89a\x16\xF3V[a\x066\x90\x84a5\x12V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x15/V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xF4W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x015_a\x07\x03\x87a\x14fV[\x90P_a\x07\x13` \x88\x01\x88a4\x9EV[\x90P_a\x07&`@\x89\x01` \x8A\x01a4\x9EV[\x90P_`\x06_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x07]\x85\x8D\x86\x86\x8E``\x015`\x05a\x14z\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x07\xA4a\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x11GV[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xC1Wa\x07\xC1a5:V[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xE0Wa\x07\xE0a5:V[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\x083W\x81\x83\x10\x15a\x08\"W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x08\x1BWa\x08\x1Ba5:V[\x01Ua\x08\x91V[a\x08,\x82\x84a4\xE4V[\x90Pa\x08\x91V[\x83`\x02\x0B\x87`\x02\x0B\x13a\x08pW\x82\x82\x10\x15a\x08fW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\x08\x1BWa\x08\x1Ba5:V[a\x08,\x83\x83a4\xE4V[\x81\x83\x87c\x01\0\0\0\x01Ta\x08\x84\x91\x90a4\xE4V[a\x08\x8E\x91\x90a4\xE4V[\x90P[_a\x08\x9C\x82\x8Ca\x15/V[\x90P\x80\x86_\x01_\x82\x82Ta\x08\xB0\x91\x90a5gV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\\W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t\x82a\tm` \x88\x01\x88a4\xF7V[a\t}`@\x89\x01` \x8A\x01a4\xF7V[a\x17\x18V[\x90P_a\t\xC2\x82a\t\x93\x86\x88a5zV[`\x04Th\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90`\xF0\x1Ca\x17pV[P\x90Pa\n\x03`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FGot pool key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a\nA`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7FComparing key spacing\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a\n\\a\nT`\x80\x89\x01``\x8A\x01a4\x9EV[`\x02\x0Ba\x17\xF3V[a\nh\x81`\x02\x0Ba\x17\xF3V[a\n\x9F`@Q\x80``\x01`@R\x80`'\x81R` \x01a8\xD1`'\x919a\n\x94``\x8A\x01`@\x8B\x01a5\xE0V[b\xFF\xFF\xFF\x16_a\x18\x84V[`\x02\x81\x90\x0Ba\n\xB4`\x80\x89\x01``\x8A\x01a4\x9EV[`\x02\x0B\x14\x15\x80a\n\xD8WP_a\n\xD0``\x89\x01`@\x8A\x01a5\xE0V[b\xFF\xFF\xFF\x16\x14\x15[\x15a\x0B\x0FW`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x96PPPPPPPV[_a\x0B}\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FlowerTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[a\x0B\xBD\x84\x83`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FupperTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x10\x06V[a\x0C\x18a\x0C\x03a\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87a\x11GV[_\x86\x81R`\x06` R`@\x90 \x90\x85\x85a\x11~V[\x94\x93PPPPV[_\x84\x81R`\x06` R`@\x90 a\x0C[\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x86\x86a\x19\x17V[PPPPV[a\x0C\x9F`@Q\x80`@\x01`@R\x80`\r\x81R` \x01\x7FUpdate called\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[\x81_a\x0C\xAA\x82a\x19\x98V[`@\x80Q\x80\x82\x01\x90\x91R`\x0E\x81R\x7FAssets decoded\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x93P\x91Pa\x0C\xEC\x90a\x0FtV[_a\r\x1A\x83\x83`\x04`\x08\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1AiV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FPairs decoded\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x94P\x91Pa\r\\\x90a\x0FtV[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01Ra\r\xC3\x84\x82`\x01\x85a\x1C\x04V[\x93Pa\x0E\x03`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FPool updated\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a\x0E\x0E\x84\x87\x87a\x1E\xC2V[a\x0EL`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FTurns out we're at the end\0\0\0\0\0\0\x81RPa\x0FtV[PPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0E\xC3W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x0FOW_\x83\x83\x83\x81\x81\x10a\x0E\xE0Wa\x0E\xE0a5:V[\x90P` \x02\x01` \x81\x01\x90a\x0E\xF5\x91\x90a4\xF7V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x03` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x0E\xC5V[PPPV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[a\x10\x03\x81`@Q`$\x01a\x0F\x88\x91\x90a6EV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[PV[_a\x10Hs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\x1E\xE2V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x11\x82a\x10m\x85`\x02\x0Ba\x1FGV[`@Q` \x01a\x10~\x92\x91\x90a6nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x10\xCEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x10\xC5\x91\x90a6EV[`@Q\x80\x91\x03\x90\xFD[PPPPPV[_\x80\x80\x80a\x10\xFAa\x10\xEF\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a6\xD8V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa\x11*\x81a\x11$s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1FRV[\x90a\x1F\x96V[\x90\x94P\x90Pa\x11:\x82\x82\x87a XV[\x92PPP\x94P\x94\x92PPPV[_\x81\x81R`\x06` R`@\x81 a\x11ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a \x82V[\x91PP[\x92\x91PPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\x9AWa\x11\x9Aa5:V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xB9Wa\x11\xB9a5:V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x11\xDEWa\x11\xD5\x81\x83a4\xE4V[\x92PPPa\x0C\x18V[\x85`\x02\x0B\x84`\x02\x0B\x13a\x11\xF5Wa\x11\xD5\x82\x82a4\xE4V[\x80\x82\x88c\x01\0\0\0\x01Ta\x12\t\x91\x90a4\xE4V[a\x12\x13\x91\x90a4\xE4V[\x97\x96PPPPPPPV[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x12\x84W`@Q\x7F\x9F8\xAF\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x12\xC1W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x13\x03W`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x13#\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a \xB2V[\x90P_a\x130\x87\x87a\x17\x18V[\x90P`@Q` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x13\xA4W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x13\x98W\x82\x85RPa\x13\xA4V[P` \x84\x01\x93Pa\x13\\V[\x90\x81Rk`\x0B8\x03\x80`\x0B_9_\xF3\0\x84R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x14\x1A\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[_\x80\x82\x13\x15a\x14`W`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`@\x80Q`\x06\x93\x90\x93R`\x03\x93\x90\x93R_\x93\x84R`&R`:`\x0C \x93\x83R` \x84\x90R\x93\x81R``\x90\x91 \x92\x90R`\x08\x91\x90\x91\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x15\x13WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x15JWc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x80_\x03a\x15fWPPPV[`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x15\xECW_\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xFEW=_\x80>=_\xFD[Pa\x16D\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a \xD5V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x16\xCFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C[\x91\x90a7\x19V[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x17\x14Wa\x17\x14a!\x1EV[P\x90V[_`(\x83\x83`@Q` \x01a\x17P\x92\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x1B\x90P\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x17\xDFW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[a\x10\x03\x81`@Q`$\x01a\x18\t\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FN\x0C\x1D\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[a\x0FO\x83\x83\x83`@Q`$\x01a\x18\x9C\x93\x92\x91\x90a70V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x96\x9C\xDD\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a\x19[W\x82`\x02\x0Ba\x19?\x82\x84`\x02\x0Ba!+\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a\x19VWa\x19V\x86\x85\x87\x86\x86\x86a!<V[a\x0ELV[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a\x0ELW_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a\x0ELWa\x0EL\x86\x85\x87\x86\x86\x86a!\xCFV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x19\xB6\x86\x84a4\xE4V[a\x19\xC0\x91\x90a7TV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x1A]W_a\x19\xEC` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1ASW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x19\xCEV[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x1A\x8B\x8A\x84a4\xE4V[a\x1A\x95\x91\x90a7TV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x1B\xF7W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x1A\xDB\x90a\x19\xE5\x90\x8C\x90`\xF0\x1Ca\"nV[\x90P_a\x1A\xEFa\x19\xE5\x8Ca\xFF\xFF\x86\x16a\"nV[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x1B8WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1BoW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x1B\xAEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x17pV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x1A\xB3V[P\x93PPP\x93P\x93\x91PPV[`\x01\x84\x81\x01\x94_\x91\x905\x82\x1A\x90a\x1C \x90\x86\x90\x83\x16\x15\x15a\"\xCFV[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FpairIndex: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x02\x87\x01\x965`\xF0\x1C\x90a\x1Ch\x90\x82a# V[a\x1C\x8Aa\x1Cy\x85a\xFF\xFF\x84\x16a#\xB5V[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x89\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x89\x01R\x16` \x87\x01\x90\x81R`\xA0\x90 _`\x10\x89\x01\x895`\x80\x1C`@\x80Q\x80\x82\x01\x90\x91R`\x0C\x81R\x7FamountIn: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x9APo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91Pa\x1D\x1A\x90\x82a# V[_\x81\x15a\x1E/W_a\x1Dea\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11GV[\x90Pa\x1Dp\x83a$\x15V[`\xE0\x8B\x01Ra\x1D\x9F\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$pV[a\x1D\xE2a\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11GV[`\x80\x8B\x01Q_\x86\x81R`\x06` R`@\x90 \x91\x93Pa\x1E)\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a\x19\x17V[Pa\x1EuV[a\x1Era\x02\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x11GV[\x90P[_a\x1E\x9C`\x02\x87\x16\x15\x15_\x86\x81R`\x06` R`@\x90 `\x80\x8D\x01Q\x8E\x91\x90\x88\x90\x87a$\x8EV[` \x8C\x01Q\x91\x9CP\x91Pa\x1E\xB2\x90\x8A\x90\x83a(\x9CV[P\x99\x9A\x99PPPPPPPPPPV[\x80\x82\x01\x80\x84\x14a\x0C[Wc\x01\x84/\x8C_R`\x04`\x1C\xFD[a\x10\x03\x81a\x0FTV[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa\x1F\"WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[``a\x11x\x82a(\xDFV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a\x1F\x8Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a \x82V[\x95\x94PPPPPV[_\x80_a 1\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a GW`\xFFa NV[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a n`\x01\x87\x90\x0Ba\x01\0a7\x8CV[a x\x91\x90a6\xD8V[a\x0C\x18\x91\x90a7\x8CV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa \xA9WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[_a\x11x` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;a7TV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a!\x15Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a!_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a\x10\xD5V[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a!xWPa\x0ELV[\x80\x15a!\xC9W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a!\x98Wa!\x98a5:V[\x01T\x87c\x01\0\0\0\x01Ta!\xAC\x91\x90a4\xE4V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a!\xC6Wa!\xC6a5:V[\x01U[Pa!<V[_a!\xF2s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a)4V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a\"\nWPa\x0ELV[\x80\x15a\"[W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\"*Wa\"*a5:V[\x01T\x87c\x01\0\0\0\x01Ta\">\x91\x90a4\xE4V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\"XWa\"Xa5:V[\x01U[\x83a\"e\x81a7\xABV[\x94PPPa!\xCFV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a\"\xBDW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x10\xC5V[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[\x80\x15\x15`\xC0\x83\x01R\x80a\"\xF6Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\"\xFDV[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[a#\xB1\x82\x82`@Q`$\x01a#6\x92\x91\x90a8\x07V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97\x10\xA9\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[PPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a$\x04W`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x10\xC5V[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x14`W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x0FOW`@Q=_\x82>P=_\xF3[_\x80\x87\x15a%0W`\x10\x87\x01\x965`\x80\x1Ca$\xFA\x81a$\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a)yV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[\x87c\x01\0\0\0\x01_\x82\x82Ta%\x0F\x91\x90a5gV[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa(\x91V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FstartTick: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x03\x88\x01\x97_\x91\x82\x915`\xE8\x1D\x90a%\x80\x90`\x02\x83\x90\x0Ba)\xC5V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7Fliquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a%\xC8\x90\x82a# V[_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a&\x1FWa&\x1A\x83\x88\x87\x89\x85a*VV[a&,V[a&,\x83\x88\x87\x89\x85a.7V[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x92\x9DP\x90\x9BP\x92\x97P\x90\x93Pa&Z\x91\x90a9?` \x83\x019a\x0FtV[`@\x80Q\x80\x82\x01\x90\x91R`\x13\x81R\x7FdonateToCurrent: %s\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x86\x01\x955`\x80\x1C\x90a&\xA2\x90\x82a# V[a&\xD0\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[a&\xDA\x90\x8Ba5gV[\x99Pa&\xF8o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84a5gV[\x92Pa'\x04\x86\x86a/\xA4V[\x81Q_\x90a'I\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a)yV[\x90Pa'\x9C`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7FcurrentLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[a'\xED`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FendLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a(fW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x10\xC5V[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta(|\x91\x90a5gV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a(\xD7a(\xCE\x82\\\x85a/\xDDV[\x92P\x81\x83a/\xF5V[P\x93\x92PPPV[``_\x82\x12a(\xF1Wa\x11x\x82a/\xFCV[a(\xFE\x82\x19`\x01\x01a/\xFCV[\x80Q`-\x82R`\x01\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x01\x90\x81R\x92\x91PPV[_\x80\x80\x80a)I\x85\x87\x07\x82\x13\x86\x88\x05\x03a\x10\xEFV[\x90\x92P\x90Pa\x11*\x81a)ss\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1FRV[\x90a0\\V[_\x81\x81R`\x06` R`@\x81 _a\x1F\x8Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a \x82V[_a\"\xC8\x82a)\xC1g\r\xE0\xB6\xB3\xA7d\0\0\x86a8(V[\x04\x90V[a#\xB1\x82\x82`@Q`$\x01a)\xDB\x92\x91\x90a8\x07V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xAF\x7F\xAA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1E\xD9V[_\x80_\x80_`\x01\x90P_\x80a*\x9F`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7Fin _rewardAbove\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[a*\xF0`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7Fstart liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[a+2`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FTrying to reward tick: %s\0\0\0\0\0\0\0\x81RP\x8C`\x02\x0Ba)\xC5V[\x82\x15a-\x10Wa+v`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7FTick is initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[`@\x80Q\x80\x82\x01\x90\x91R`\x15\x81R\x7FGot reward amount: %s\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a+\xBE\x90\x82a# V[a+\xC8\x81\x84a5gV[\x92Pa+\xE6\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[a+\xF0\x90\x83a5gV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a,\rWa,\ra5:V[\x01_\x82\x82Ta,\x1C\x91\x90a5gV[\x90\x91UPP\x88Q_\x90a,g\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1E\xE2V[\x91PPa,\xAC`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FnetLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82`\x0F\x0Ba)\xC5V[a,\xB6\x8B\x82a1$V[\x9APa-\t`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7Fnew liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Co\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[PPa-NV[a-N`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FTick is not initialized\0\0\0\0\0\0\0\0\0\x81RPa\x0FtV[\x87Q` \x89\x01Qa-\x98\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a1>V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a*\xF0Wa-\xF0`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FDone rewarding.  Total: %s\0\0\0\0\0\0\x81RP\x83a# V[a.$`@Q\x80``\x01`@R\x80`$\x81R` \x01a9\x1B`$\x919\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a# V[\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80_\x80_`\x01\x90P_\x80a.c`@Q\x80``\x01`@R\x80`#\x81R` \x01a8\xAE`#\x919a\x0FtV[\x82\x15a/-W`\x10\x8A\x01\x995`\x80\x1Ca.|\x81\x84a5gV[\x92Pa.\x9A\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a)\xAAV[a.\xA4\x90\x83a5gV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a.\xC1Wa.\xC1a5:V[\x01_\x82\x82Ta.\xD0\x91\x90a5gV[\x90\x91UPP\x88Q_\x90a/\x1B\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1E\xE2V[\x91PPa/(\x8B\x82a1XV[\x9APPP[\x87Q` \x89\x01Qa/w\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a\x10\xD5V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a.cW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a#\xB1W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x11xWc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a0\x15WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[_\x80_\x83`\xFF\x03\x90P_a0\xFD\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a1\x12W_a1\x19V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x11xWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a)Ia\x10\xEF`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03a8?V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x11xWc\xC9eN\xD4_R`\x04`\x1C\xFD[a1za8\x80V[V[\x805`\x02\x81\x90\x0B\x81\x14a1\x8DW_\x80\xFD[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a1\xA4W_\x80\xFD[\x835\x92Pa1\xB4` \x85\x01a1|V[\x91Pa1\xC2`@\x85\x01a1|V[\x90P\x92P\x92P\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x10\x03W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a1\x8DW_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a2\x11W_\x80\xFD[\x845a2\x1C\x81a1\xCBV[\x93P` \x85\x015a2,\x81a1\xCBV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14a2BW_\x80\xFD[\x91Pa2P``\x86\x01a1\xECV[\x90P\x92\x95\x91\x94P\x92PV[_`\xA0\x82\x84\x03\x12\x15a2kW_\x80\xFD[P\x91\x90PV[_\x80\x83`\x1F\x84\x01\x12a2\x81W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a2\x98W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\xAFW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15a2\xCCW_\x80\xFD[\x865a2\xD7\x81a1\xCBV[\x95Pa2\xE6\x88` \x89\x01a2[V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a3\x17W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a38W_\x80\xFD[a3D\x88\x82\x89\x01a2qV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a3jW_\x80\xFD[\x855a3u\x81a1\xCBV[\x94Pa3\x84\x87` \x88\x01a2[V[\x93P`\xC0\x86\x015a3\x94\x81a1\xCBV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a38W_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a3\xC2W_\x80\xFD[\x845\x93Pa3\xD2` \x86\x01a1|V[\x92Pa3\xE0`@\x86\x01a1|V[\x91Pa2P``\x86\x01a1|V[_\x80` \x83\x85\x03\x12\x15a3\xFFW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\x15W_\x80\xFD[a4!\x85\x82\x86\x01a2qV[\x90\x96\x90\x95P\x93PPPPV[_\x80` \x83\x85\x03\x12\x15a4>W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4TW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a4dW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4zW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a4\x8EW_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a4\xAEW_\x80\xFD[a\"\xC8\x82a1|V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x11xWa\x11xa4\xB7V[_` \x82\x84\x03\x12\x15a5\x07W_\x80\xFD[\x815a\"\xC8\x81a1\xCBV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x11xWa\x11xa4\xB7V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x11xWa\x11xa4\xB7V[\x805\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x02\x84\x10\x15a5\xD9W\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x02\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_` \x82\x84\x03\x12\x15a5\xF0W_\x80\xFD[a\"\xC8\x82a1\xECV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\"\xC8` \x83\x01\x84a5\xF9V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a6y\x82\x85a6WV[\x7F [\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra6\xA9`\x02\x82\x01\x85a6WV[\x7F] not initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11\x01\x95\x94PPPPPV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x11xWa\x11xa4\xB7V[_` \x82\x84\x03\x12\x15a7)W_\x80\xFD[PQ\x91\x90PV[``\x81R_a7B``\x83\x01\x86a5\xF9V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82a7\x87W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a5\xD9Wa5\xD9a4\xB7V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a7\xDFWa7\xDFa4\xB7V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[`@\x81R_a8\x19`@\x83\x01\x85a5\xF9V[\x90P\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x11xWa\x11xa4\xB7V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x11xWa\x11xa4\xB7V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`Q`\x04R`$_\xFD\xFEin _rewardBelow but we shouldn't beComparing key fee %s to expected fee %sgetGrowthInsideTick has been calledDone rewarding.  Final liquidity: %sDone rewarding, trying to get reward for current tick\xA1dsolcC\0\x08\x1A\0\n",
    );
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Currency(alloy::sol_types::private::Address);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Currency>
        for alloy::sol_types::private::Address {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                '_,
            > {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Address,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        self,
                    )
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encode_packed_to(
                    self,
                    out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl Currency {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::Address) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::Address {
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
        impl alloy_sol_types::SolType for Currency {
            type RustType = alloy::sol_types::private::Address;
            type Token<'a> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                'a,
            >;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Currency {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    rust,
                )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    out,
                )
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
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
    /**```solidity
struct PoolKey { Currency currency0; Currency currency1; uint24 fee; int24 tickSpacing; address hooks; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoolKey {
        pub currency0: <Currency as alloy::sol_types::SolType>::RustType,
        pub currency1: <Currency as alloy::sol_types::SolType>::RustType,
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        pub hooks: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            Currency,
            Currency,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Currency as alloy::sol_types::SolType>::RustType,
            <Currency as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U24,
            alloy::sol_types::private::primitives::aliases::I24,
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
        impl ::core::convert::From<PoolKey> for UnderlyingRustTuple<'_> {
            fn from(value: PoolKey) -> Self {
                (
                    value.currency0,
                    value.currency1,
                    value.fee,
                    value.tickSpacing,
                    value.hooks,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currency0: tuple.0,
                    currency1: tuple.1,
                    fee: tuple.2,
                    tickSpacing: tuple.3,
                    hooks: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PoolKey {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PoolKey {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency0),
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency1),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hooks,
                    ),
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
        impl alloy_sol_types::SolType for PoolKey {
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
        impl alloy_sol_types::SolStruct for PoolKey {
            const NAME: &'static str = "PoolKey";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PoolKey(address currency0,address currency1,uint24 fee,int24 tickSpacing,address hooks)",
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
                    <Currency as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currency0,
                        )
                        .0,
                    <Currency as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currency1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fee)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickSpacing)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hooks,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolKey {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Currency as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currency0,
                    )
                    + <Currency as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currency1,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.fee)
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickSpacing,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hooks,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <Currency as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currency0,
                    out,
                );
                <Currency as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currency1,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.fee, out);
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickSpacing,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hooks,
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
    /**Custom error with signature `AssetAccessOutOfBounds(uint256,uint256)` and selector `0xffc31e71`.
```solidity
error AssetAccessOutOfBounds(uint256 index, uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AssetAccessOutOfBounds {
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<AssetAccessOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: AssetAccessOutOfBounds) -> Self {
                (value.index, value.length)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssetAccessOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    index: tuple.0,
                    length: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AssetAccessOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AssetAccessOutOfBounds(uint256,uint256)";
            const SELECTOR: [u8; 4] = [255u8, 195u8, 30u8, 113u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    /**Custom error with signature `AssetsOutOfOrderOrNotUnique()` and selector `0x80f11acf`.
```solidity
error AssetsOutOfOrderOrNotUnique();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AssetsOutOfOrderOrNotUnique {}
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
        impl ::core::convert::From<AssetsOutOfOrderOrNotUnique>
        for UnderlyingRustTuple<'_> {
            fn from(value: AssetsOutOfOrderOrNotUnique) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AssetsOutOfOrderOrNotUnique {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AssetsOutOfOrderOrNotUnique {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AssetsOutOfOrderOrNotUnique()";
            const SELECTOR: [u8; 4] = [128u8, 241u8, 26u8, 207u8];
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
    /**Custom error with signature `AssetsUnsorted()` and selector `0x9f38afb8`.
```solidity
error AssetsUnsorted();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AssetsUnsorted {}
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
        impl ::core::convert::From<AssetsUnsorted> for UnderlyingRustTuple<'_> {
            fn from(value: AssetsUnsorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AssetsUnsorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AssetsUnsorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AssetsUnsorted()";
            const SELECTOR: [u8; 4] = [159u8, 56u8, 175u8, 184u8];
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
    /**Custom error with signature `BundleChangeNetNegative(address)` and selector `0xd49b70f5`.
```solidity
error BundleChangeNetNegative(address asset);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct BundleChangeNetNegative {
        pub asset: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<BundleChangeNetNegative> for UnderlyingRustTuple<'_> {
            fn from(value: BundleChangeNetNegative) -> Self {
                (value.asset,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BundleChangeNetNegative {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { asset: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BundleChangeNetNegative {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BundleChangeNetNegative(address)";
            const SELECTOR: [u8; 4] = [212u8, 155u8, 112u8, 245u8];
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
                )
            }
        }
    };
    /**Custom error with signature `FailedToDeployNewStore()` and selector `0x56702587`.
```solidity
error FailedToDeployNewStore();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FailedToDeployNewStore {}
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
        impl ::core::convert::From<FailedToDeployNewStore> for UnderlyingRustTuple<'_> {
            fn from(value: FailedToDeployNewStore) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FailedToDeployNewStore {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FailedToDeployNewStore {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FailedToDeployNewStore()";
            const SELECTOR: [u8; 4] = [86u8, 112u8, 37u8, 135u8];
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
    /**Custom error with signature `FeeAboveMax()` and selector `0x76a3f95d`.
```solidity
error FeeAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FeeAboveMax {}
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
        impl ::core::convert::From<FeeAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: FeeAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FeeAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FeeAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FeeAboveMax()";
            const SELECTOR: [u8; 4] = [118u8, 163u8, 249u8, 93u8];
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
    /**Custom error with signature `InvalidPoolKey()` and selector `0xc256622b`.
```solidity
error InvalidPoolKey();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidPoolKey {}
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
        impl ::core::convert::From<InvalidPoolKey> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPoolKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPoolKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPoolKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPoolKey()";
            const SELECTOR: [u8; 4] = [194u8, 86u8, 98u8, 43u8];
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
    /**Custom error with signature `InvalidTickSpacing()` and selector `0x270815a0`.
```solidity
error InvalidTickSpacing();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidTickSpacing {}
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
        impl ::core::convert::From<InvalidTickSpacing> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTickSpacing) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTickSpacing {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTickSpacing {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTickSpacing()";
            const SELECTOR: [u8; 4] = [39u8, 8u8, 21u8, 160u8];
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
    /**Custom error with signature `MissingHookPermissions(uint160)` and selector `0x75383228`.
```solidity
error MissingHookPermissions(uint160);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MissingHookPermissions {
        pub _0: alloy::sol_types::private::primitives::aliases::U160,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<160>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
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
        impl ::core::convert::From<MissingHookPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: MissingHookPermissions) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MissingHookPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MissingHookPermissions {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MissingHookPermissions(uint160)";
            const SELECTOR: [u8; 4] = [117u8, 56u8, 50u8, 40u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
            }
        }
    };
    /**Custom error with signature `NoEntry()` and selector `0x2f659e44`.
```solidity
error NoEntry();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NoEntry {}
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
        impl ::core::convert::From<NoEntry> for UnderlyingRustTuple<'_> {
            fn from(value: NoEntry) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoEntry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoEntry {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoEntry()";
            const SELECTOR: [u8; 4] = [47u8, 101u8, 158u8, 68u8];
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
    /**Custom error with signature `NotController()` and selector `0x23019e67`.
```solidity
error NotController();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NotController {}
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
        impl ::core::convert::From<NotController> for UnderlyingRustTuple<'_> {
            fn from(value: NotController) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotController {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotController {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotController()";
            const SELECTOR: [u8; 4] = [35u8, 1u8, 158u8, 103u8];
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
    /**Custom error with signature `NotNode()` and selector `0x5cd26b68`.
```solidity
error NotNode();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NotNode {}
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
        impl ::core::convert::From<NotNode> for UnderlyingRustTuple<'_> {
            fn from(value: NotNode) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotNode {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotNode {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotNode()";
            const SELECTOR: [u8; 4] = [92u8, 210u8, 107u8, 104u8];
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
    /**Custom error with signature `NotUniswap()` and selector `0xf8328614`.
```solidity
error NotUniswap();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NotUniswap {}
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
        impl ::core::convert::From<NotUniswap> for UnderlyingRustTuple<'_> {
            fn from(value: NotUniswap) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotUniswap {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotUniswap {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotUniswap()";
            const SELECTOR: [u8; 4] = [248u8, 50u8, 134u8, 20u8];
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
    /**Custom error with signature `OnlyOncePerBlock()` and selector `0xd8a6b89b`.
```solidity
error OnlyOncePerBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OnlyOncePerBlock {}
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
        impl ::core::convert::From<OnlyOncePerBlock> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyOncePerBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyOncePerBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyOncePerBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyOncePerBlock()";
            const SELECTOR: [u8; 4] = [216u8, 166u8, 184u8, 155u8];
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
    /**Custom error with signature `OutOfOrderOrDuplicatePairs()` and selector `0xf35f9399`.
```solidity
error OutOfOrderOrDuplicatePairs();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OutOfOrderOrDuplicatePairs {}
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
        impl ::core::convert::From<OutOfOrderOrDuplicatePairs>
        for UnderlyingRustTuple<'_> {
            fn from(value: OutOfOrderOrDuplicatePairs) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OutOfOrderOrDuplicatePairs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfOrderOrDuplicatePairs {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfOrderOrDuplicatePairs()";
            const SELECTOR: [u8; 4] = [243u8, 95u8, 147u8, 153u8];
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
    /**Custom error with signature `PairAccessOutOfBounds(uint256,uint256)` and selector `0xf6601b50`.
```solidity
error PairAccessOutOfBounds(uint256 index, uint256 length);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PairAccessOutOfBounds {
        pub index: alloy::sol_types::private::primitives::aliases::U256,
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<PairAccessOutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: PairAccessOutOfBounds) -> Self {
                (value.index, value.length)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PairAccessOutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    index: tuple.0,
                    length: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PairAccessOutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PairAccessOutOfBounds(uint256,uint256)";
            const SELECTOR: [u8; 4] = [246u8, 96u8, 27u8, 80u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
                )
            }
        }
    };
    /**Custom error with signature `ReaderNotAtEnd()` and selector `0x01842f8c`.
```solidity
error ReaderNotAtEnd();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ReaderNotAtEnd {}
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
        impl ::core::convert::From<ReaderNotAtEnd> for UnderlyingRustTuple<'_> {
            fn from(value: ReaderNotAtEnd) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ReaderNotAtEnd {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ReaderNotAtEnd {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ReaderNotAtEnd()";
            const SELECTOR: [u8; 4] = [1u8, 132u8, 47u8, 140u8];
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
    /**Custom error with signature `Underflow()` and selector `0xcaccb6d9`.
```solidity
error Underflow();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Underflow {}
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
        impl ::core::convert::From<Underflow> for UnderlyingRustTuple<'_> {
            fn from(value: Underflow) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Underflow {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Underflow {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Underflow()";
            const SELECTOR: [u8; 4] = [202u8, 204u8, 182u8, 217u8];
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
    /**Custom error with signature `WrongEndLiquidity(uint128,uint128)` and selector `0x6429cfd2`.
```solidity
error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct WrongEndLiquidity {
        pub endLiquidity: u128,
        pub actualCurrentLiquidity: u128,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Uint<128>,
            alloy::sol_types::sol_data::Uint<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u128, u128);
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
        impl ::core::convert::From<WrongEndLiquidity> for UnderlyingRustTuple<'_> {
            fn from(value: WrongEndLiquidity) -> Self {
                (value.endLiquidity, value.actualCurrentLiquidity)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WrongEndLiquidity {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    endLiquidity: tuple.0,
                    actualCurrentLiquidity: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WrongEndLiquidity {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WrongEndLiquidity(uint128,uint128)";
            const SELECTOR: [u8; 4] = [100u8, 41u8, 207u8, 210u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.endLiquidity),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.actualCurrentLiquidity,
                    ),
                )
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address uniV4PoolManager, address controller);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub uniV4PoolManager: alloy::sol_types::private::Address,
        pub controller: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value.uniV4PoolManager, value.controller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        uniV4PoolManager: tuple.0,
                        controller: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
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
                        &self.uniV4PoolManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.controller,
                    ),
                )
            }
        }
    };
    /**Function with signature `beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x259982e5`.
```solidity
function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeAddLiquidityCall {
        pub sender: alloy::sol_types::private::Address,
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        pub _3: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)`](beforeAddLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeAddLiquidityReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<beforeAddLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeAddLiquidityCall) -> Self {
                    (value.sender, value.key, value.params, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeAddLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        key: tuple.1,
                        params: tuple.2,
                        _3: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<beforeAddLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeAddLiquidityReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeAddLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beforeAddLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beforeAddLiquidityReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [37u8, 153u8, 130u8, 229u8];
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
                        &self.sender,
                    ),
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._3,
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
    /**Function with signature `beforeInitialize(address,(address,address,uint24,int24,address),uint160,bytes)` and selector `0x3440d820`.
```solidity
function beforeInitialize(address, PoolKey memory poolKey, uint160, bytes memory storeIndexBytes) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeInitializeCall {
        pub _0: alloy::sol_types::private::Address,
        pub poolKey: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub _2: alloy::sol_types::private::primitives::aliases::U160,
        pub storeIndexBytes: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`beforeInitialize(address,(address,address,uint24,int24,address),uint160,bytes)`](beforeInitializeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeInitializeReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U160,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<beforeInitializeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeInitializeCall) -> Self {
                    (value._0, value.poolKey, value._2, value.storeIndexBytes)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeInitializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        poolKey: tuple.1,
                        _2: tuple.2,
                        storeIndexBytes: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<beforeInitializeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeInitializeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeInitializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beforeInitializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beforeInitializeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beforeInitialize(address,(address,address,uint24,int24,address),uint160,bytes)";
            const SELECTOR: [u8; 4] = [52u8, 64u8, 216u8, 32u8];
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
                        &self._0,
                    ),
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.poolKey),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.storeIndexBytes,
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
    /**Function with signature `beforeRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x21d0ee70`.
```solidity
function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeRemoveLiquidityCall {
        pub sender: alloy::sol_types::private::Address,
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        pub _3: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`beforeRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)`](beforeRemoveLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeRemoveLiquidityReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<beforeRemoveLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeRemoveLiquidityCall) -> Self {
                    (value.sender, value.key, value.params, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeRemoveLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        key: tuple.1,
                        params: tuple.2,
                        _3: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<beforeRemoveLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: beforeRemoveLiquidityReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeRemoveLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for beforeRemoveLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = beforeRemoveLiquidityReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "beforeRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [33u8, 208u8, 238u8, 112u8];
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
                        &self.sender,
                    ),
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._3,
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
    /**Function with signature `configurePool(address,address,uint16,uint24)` and selector `0x1090641d`.
```solidity
function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 feeInE6) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct configurePoolCall {
        pub asset0: alloy::sol_types::private::Address,
        pub asset1: alloy::sol_types::private::Address,
        pub tickSpacing: u16,
        pub feeInE6: alloy::sol_types::private::primitives::aliases::U24,
    }
    ///Container type for the return parameters of the [`configurePool(address,address,uint16,uint24)`](configurePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct configurePoolReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u16,
                alloy::sol_types::private::primitives::aliases::U24,
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
            impl ::core::convert::From<configurePoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: configurePoolCall) -> Self {
                    (value.asset0, value.asset1, value.tickSpacing, value.feeInE6)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configurePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset0: tuple.0,
                        asset1: tuple.1,
                        tickSpacing: tuple.2,
                        feeInE6: tuple.3,
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
            impl ::core::convert::From<configurePoolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: configurePoolReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configurePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for configurePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = configurePoolReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "configurePool(address,address,uint16,uint24)";
            const SELECTOR: [u8; 4] = [16u8, 144u8, 100u8, 29u8];
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
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.feeInE6),
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
    /**Function with signature `consts()` and selector `0xd86d744e`.
```solidity
function consts() external pure returns (uint24 poolFee);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constsCall {}
    ///Container type for the return parameters of the [`consts()`](constsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constsReturn {
        pub poolFee: alloy::sol_types::private::primitives::aliases::U24,
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
            impl ::core::convert::From<constsCall> for UnderlyingRustTuple<'_> {
                fn from(value: constsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<24>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U24,
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
            impl ::core::convert::From<constsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: constsReturn) -> Self {
                    (value.poolFee,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { poolFee: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for constsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = constsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "consts()";
            const SELECTOR: [u8; 4] = [216u8, 109u8, 116u8, 78u8];
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
    /**Function with signature `getGrowthInsideRange(bytes32,int24,int24)` and selector `0x62889dd6`.
```solidity
function getGrowthInsideRange(PoolId id, int24 lowerTick, int24 upperTick) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getGrowthInsideRangeCall {
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        pub lowerTick: alloy::sol_types::private::primitives::aliases::I24,
        pub upperTick: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`getGrowthInsideRange(bytes32,int24,int24)`](getGrowthInsideRangeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getGrowthInsideRangeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolId,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<getGrowthInsideRangeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGrowthInsideRangeCall) -> Self {
                    (value.id, value.lowerTick, value.upperTick)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGrowthInsideRangeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        lowerTick: tuple.1,
                        upperTick: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getGrowthInsideRangeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGrowthInsideRangeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGrowthInsideRangeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGrowthInsideRangeCall {
            type Parameters<'a> = (
                PoolId,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getGrowthInsideRangeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGrowthInsideRange(bytes32,int24,int24)";
            const SELECTOR: [u8; 4] = [98u8, 136u8, 157u8, 214u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolId as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.lowerTick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.upperTick),
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
    /**Function with signature `getGrowthInsideTick(bytes32,int24,int24)` and selector `0x0b3dd76e`.
```solidity
function getGrowthInsideTick(PoolId id, int24 tick, int24 tickSpacing) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getGrowthInsideTickCall {
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`getGrowthInsideTick(bytes32,int24,int24)`](getGrowthInsideTickCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getGrowthInsideTickReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolId,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<getGrowthInsideTickCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGrowthInsideTickCall) -> Self {
                    (value.id, value.tick, value.tickSpacing)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGrowthInsideTickCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        tick: tuple.1,
                        tickSpacing: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getGrowthInsideTickReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getGrowthInsideTickReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGrowthInsideTickReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getGrowthInsideTickCall {
            type Parameters<'a> = (
                PoolId,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getGrowthInsideTickReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGrowthInsideTick(bytes32,int24,int24)";
            const SELECTOR: [u8; 4] = [11u8, 61u8, 215u8, 110u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolId as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
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
    /**Function with signature `toggleNodes(address[])` and selector `0xd6cffd1e`.
```solidity
function toggleNodes(address[] memory nodes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct toggleNodesCall {
        pub nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`toggleNodes(address[])`](toggleNodesCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct toggleNodesReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<toggleNodesCall> for UnderlyingRustTuple<'_> {
                fn from(value: toggleNodesCall) -> Self {
                    (value.nodes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for toggleNodesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nodes: tuple.0 }
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
            impl ::core::convert::From<toggleNodesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: toggleNodesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for toggleNodesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for toggleNodesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = toggleNodesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "toggleNodes(address[])";
            const SELECTOR: [u8; 4] = [214u8, 207u8, 253u8, 30u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.nodes),
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
    /**Function with signature `update(bytes)` and selector `0xc43ed2c8`.
```solidity
function update(bytes memory encoded) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateCall {
        pub encoded: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`update(bytes)`](updateCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateReturn {}
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
            impl ::core::convert::From<updateCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateCall) -> Self {
                    (value.encoded,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { encoded: tuple.0 }
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
            impl ::core::convert::From<updateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "update(bytes)";
            const SELECTOR: [u8; 4] = [196u8, 62u8, 210u8, 200u8];
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
                        &self.encoded,
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
    /**Function with signature `updateAfterTickMove(bytes32,int24,int24,int24)` and selector `0x71cca81b`.
```solidity
function updateAfterTickMove(PoolId id, int24 lastTick, int24 newTick, int24 tickSpacing) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateAfterTickMoveCall {
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        pub lastTick: alloy::sol_types::private::primitives::aliases::I24,
        pub newTick: alloy::sol_types::private::primitives::aliases::I24,
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`updateAfterTickMove(bytes32,int24,int24,int24)`](updateAfterTickMoveCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateAfterTickMoveReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolId,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::I24,
                alloy::sol_types::private::primitives::aliases::I24,
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
            impl ::core::convert::From<updateAfterTickMoveCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAfterTickMoveCall) -> Self {
                    (value.id, value.lastTick, value.newTick, value.tickSpacing)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAfterTickMoveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        id: tuple.0,
                        lastTick: tuple.1,
                        newTick: tuple.2,
                        tickSpacing: tuple.3,
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
            impl ::core::convert::From<updateAfterTickMoveReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAfterTickMoveReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAfterTickMoveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAfterTickMoveCall {
            type Parameters<'a> = (
                PoolId,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAfterTickMoveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAfterTickMove(bytes32,int24,int24,int24)";
            const SELECTOR: [u8; 4] = [113u8, 204u8, 168u8, 27u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolId as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.lastTick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTick),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
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
    ///Container for all the [`MockRewardsManager`](self) function calls.
    pub enum MockRewardsManagerCalls {
        beforeAddLiquidity(beforeAddLiquidityCall),
        beforeInitialize(beforeInitializeCall),
        beforeRemoveLiquidity(beforeRemoveLiquidityCall),
        configurePool(configurePoolCall),
        consts(constsCall),
        getGrowthInsideRange(getGrowthInsideRangeCall),
        getGrowthInsideTick(getGrowthInsideTickCall),
        toggleNodes(toggleNodesCall),
        update(updateCall),
        updateAfterTickMove(updateAfterTickMoveCall),
    }
    #[automatically_derived]
    impl MockRewardsManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 61u8, 215u8, 110u8],
            [16u8, 144u8, 100u8, 29u8],
            [33u8, 208u8, 238u8, 112u8],
            [37u8, 153u8, 130u8, 229u8],
            [52u8, 64u8, 216u8, 32u8],
            [98u8, 136u8, 157u8, 214u8],
            [113u8, 204u8, 168u8, 27u8],
            [196u8, 62u8, 210u8, 200u8],
            [214u8, 207u8, 253u8, 30u8],
            [216u8, 109u8, 116u8, 78u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockRewardsManagerCalls {
        const NAME: &'static str = "MockRewardsManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 10usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::beforeAddLiquidity(_) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeInitialize(_) => {
                    <beforeInitializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeRemoveLiquidity(_) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configurePool(_) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::consts(_) => <constsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getGrowthInsideRange(_) => {
                    <getGrowthInsideRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGrowthInsideTick(_) => {
                    <getGrowthInsideTickCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::toggleNodes(_) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::update(_) => <updateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateAfterTickMove(_) => {
                    <updateAfterTickMoveCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockRewardsManagerCalls>] = &[
                {
                    fn getGrowthInsideTick(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <getGrowthInsideTickCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::getGrowthInsideTick)
                    }
                    getGrowthInsideTick
                },
                {
                    fn configurePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <configurePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::configurePool)
                    }
                    configurePool
                },
                {
                    fn beforeRemoveLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::beforeRemoveLiquidity)
                    }
                    beforeRemoveLiquidity
                },
                {
                    fn beforeAddLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::beforeAddLiquidity)
                    }
                    beforeAddLiquidity
                },
                {
                    fn beforeInitialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <beforeInitializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::beforeInitialize)
                    }
                    beforeInitialize
                },
                {
                    fn getGrowthInsideRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <getGrowthInsideRangeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::getGrowthInsideRange)
                    }
                    getGrowthInsideRange
                },
                {
                    fn updateAfterTickMove(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <updateAfterTickMoveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::updateAfterTickMove)
                    }
                    updateAfterTickMove
                },
                {
                    fn update(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <updateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::update)
                    }
                    update
                },
                {
                    fn toggleNodes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <toggleNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::toggleNodes)
                    }
                    toggleNodes
                },
                {
                    fn consts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <constsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::consts)
                    }
                    consts
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
                Self::beforeAddLiquidity(inner) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beforeInitialize(inner) => {
                    <beforeInitializeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beforeRemoveLiquidity(inner) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::configurePool(inner) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::consts(inner) => {
                    <constsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getGrowthInsideRange(inner) => {
                    <getGrowthInsideRangeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getGrowthInsideTick(inner) => {
                    <getGrowthInsideTickCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::toggleNodes(inner) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::update(inner) => {
                    <updateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateAfterTickMove(inner) => {
                    <updateAfterTickMoveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::beforeAddLiquidity(inner) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beforeInitialize(inner) => {
                    <beforeInitializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beforeRemoveLiquidity(inner) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::configurePool(inner) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::consts(inner) => {
                    <constsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getGrowthInsideRange(inner) => {
                    <getGrowthInsideRangeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getGrowthInsideTick(inner) => {
                    <getGrowthInsideTickCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::toggleNodes(inner) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::update(inner) => {
                    <updateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateAfterTickMove(inner) => {
                    <updateAfterTickMoveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`MockRewardsManager`](self) custom errors.
    pub enum MockRewardsManagerErrors {
        AssetAccessOutOfBounds(AssetAccessOutOfBounds),
        AssetsOutOfOrderOrNotUnique(AssetsOutOfOrderOrNotUnique),
        AssetsUnsorted(AssetsUnsorted),
        BundleChangeNetNegative(BundleChangeNetNegative),
        FailedToDeployNewStore(FailedToDeployNewStore),
        FeeAboveMax(FeeAboveMax),
        InvalidPoolKey(InvalidPoolKey),
        InvalidTickSpacing(InvalidTickSpacing),
        MissingHookPermissions(MissingHookPermissions),
        NoEntry(NoEntry),
        NotController(NotController),
        NotNode(NotNode),
        NotUniswap(NotUniswap),
        OnlyOncePerBlock(OnlyOncePerBlock),
        OutOfOrderOrDuplicatePairs(OutOfOrderOrDuplicatePairs),
        Overflow(Overflow),
        PairAccessOutOfBounds(PairAccessOutOfBounds),
        ReaderNotAtEnd(ReaderNotAtEnd),
        Underflow(Underflow),
        WrongEndLiquidity(WrongEndLiquidity),
    }
    #[automatically_derived]
    impl MockRewardsManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 132u8, 47u8, 140u8],
            [35u8, 1u8, 158u8, 103u8],
            [39u8, 8u8, 21u8, 160u8],
            [47u8, 101u8, 158u8, 68u8],
            [53u8, 39u8, 141u8, 18u8],
            [86u8, 112u8, 37u8, 135u8],
            [92u8, 210u8, 107u8, 104u8],
            [100u8, 41u8, 207u8, 210u8],
            [117u8, 56u8, 50u8, 40u8],
            [118u8, 163u8, 249u8, 93u8],
            [128u8, 241u8, 26u8, 207u8],
            [159u8, 56u8, 175u8, 184u8],
            [194u8, 86u8, 98u8, 43u8],
            [202u8, 204u8, 182u8, 217u8],
            [212u8, 155u8, 112u8, 245u8],
            [216u8, 166u8, 184u8, 155u8],
            [243u8, 95u8, 147u8, 153u8],
            [246u8, 96u8, 27u8, 80u8],
            [248u8, 50u8, 134u8, 20u8],
            [255u8, 195u8, 30u8, 113u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockRewardsManagerErrors {
        const NAME: &'static str = "MockRewardsManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AssetAccessOutOfBounds(_) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AssetsOutOfOrderOrNotUnique(_) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AssetsUnsorted(_) => {
                    <AssetsUnsorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BundleChangeNetNegative(_) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedToDeployNewStore(_) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeAboveMax(_) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPoolKey(_) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTickSpacing(_) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MissingHookPermissions(_) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoEntry(_) => <NoEntry as alloy_sol_types::SolError>::SELECTOR,
                Self::NotController(_) => {
                    <NotController as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotNode(_) => <NotNode as alloy_sol_types::SolError>::SELECTOR,
                Self::NotUniswap(_) => {
                    <NotUniswap as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyOncePerBlock(_) => {
                    <OnlyOncePerBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfOrderOrDuplicatePairs(_) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Overflow(_) => <Overflow as alloy_sol_types::SolError>::SELECTOR,
                Self::PairAccessOutOfBounds(_) => {
                    <PairAccessOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ReaderNotAtEnd(_) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Underflow(_) => <Underflow as alloy_sol_types::SolError>::SELECTOR,
                Self::WrongEndLiquidity(_) => {
                    <WrongEndLiquidity as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<MockRewardsManagerErrors>] = &[
                {
                    fn ReaderNotAtEnd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::ReaderNotAtEnd)
                    }
                    ReaderNotAtEnd
                },
                {
                    fn NotController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <NotController as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::NotController)
                    }
                    NotController
                },
                {
                    fn InvalidTickSpacing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <InvalidTickSpacing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::InvalidTickSpacing)
                    }
                    InvalidTickSpacing
                },
                {
                    fn NoEntry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <NoEntry as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::NoEntry)
                    }
                    NoEntry
                },
                {
                    fn Overflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <Overflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::Overflow)
                    }
                    Overflow
                },
                {
                    fn FailedToDeployNewStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::FailedToDeployNewStore)
                    }
                    FailedToDeployNewStore
                },
                {
                    fn NotNode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <NotNode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::NotNode)
                    }
                    NotNode
                },
                {
                    fn WrongEndLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <WrongEndLiquidity as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::WrongEndLiquidity)
                    }
                    WrongEndLiquidity
                },
                {
                    fn MissingHookPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <MissingHookPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::MissingHookPermissions)
                    }
                    MissingHookPermissions
                },
                {
                    fn FeeAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <FeeAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::FeeAboveMax)
                    }
                    FeeAboveMax
                },
                {
                    fn AssetsOutOfOrderOrNotUnique(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::AssetsOutOfOrderOrNotUnique)
                    }
                    AssetsOutOfOrderOrNotUnique
                },
                {
                    fn AssetsUnsorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <AssetsUnsorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::AssetsUnsorted)
                    }
                    AssetsUnsorted
                },
                {
                    fn InvalidPoolKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <InvalidPoolKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::InvalidPoolKey)
                    }
                    InvalidPoolKey
                },
                {
                    fn Underflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <Underflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::Underflow)
                    }
                    Underflow
                },
                {
                    fn BundleChangeNetNegative(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <BundleChangeNetNegative as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::BundleChangeNetNegative)
                    }
                    BundleChangeNetNegative
                },
                {
                    fn OnlyOncePerBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::OnlyOncePerBlock)
                    }
                    OnlyOncePerBlock
                },
                {
                    fn OutOfOrderOrDuplicatePairs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::OutOfOrderOrDuplicatePairs)
                    }
                    OutOfOrderOrDuplicatePairs
                },
                {
                    fn PairAccessOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::PairAccessOutOfBounds)
                    }
                    PairAccessOutOfBounds
                },
                {
                    fn NotUniswap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <NotUniswap as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::NotUniswap)
                    }
                    NotUniswap
                },
                {
                    fn AssetAccessOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::AssetAccessOutOfBounds)
                    }
                    AssetAccessOutOfBounds
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
                Self::AssetAccessOutOfBounds(inner) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AssetsOutOfOrderOrNotUnique(inner) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AssetsUnsorted(inner) => {
                    <AssetsUnsorted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BundleChangeNetNegative(inner) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FailedToDeployNewStore(inner) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeAboveMax(inner) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidPoolKey(inner) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTickSpacing(inner) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MissingHookPermissions(inner) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoEntry(inner) => {
                    <NoEntry as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotController(inner) => {
                    <NotController as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotNode(inner) => {
                    <NotNode as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotUniswap(inner) => {
                    <NotUniswap as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyOncePerBlock(inner) => {
                    <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfOrderOrDuplicatePairs(inner) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::PairAccessOutOfBounds(inner) => {
                    <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ReaderNotAtEnd(inner) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Underflow(inner) => {
                    <Underflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::WrongEndLiquidity(inner) => {
                    <WrongEndLiquidity as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AssetAccessOutOfBounds(inner) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AssetsOutOfOrderOrNotUnique(inner) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AssetsUnsorted(inner) => {
                    <AssetsUnsorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BundleChangeNetNegative(inner) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FailedToDeployNewStore(inner) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FeeAboveMax(inner) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidPoolKey(inner) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTickSpacing(inner) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MissingHookPermissions(inner) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoEntry(inner) => {
                    <NoEntry as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotController(inner) => {
                    <NotController as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotNode(inner) => {
                    <NotNode as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::NotUniswap(inner) => {
                    <NotUniswap as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyOncePerBlock(inner) => {
                    <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfOrderOrDuplicatePairs(inner) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::PairAccessOutOfBounds(inner) => {
                    <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ReaderNotAtEnd(inner) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Underflow(inner) => {
                    <Underflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::WrongEndLiquidity(inner) => {
                    <WrongEndLiquidity as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockRewardsManager`](self) contract instance.

See the [wrapper's documentation](`MockRewardsManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> MockRewardsManagerInstance<T, P, N> {
        MockRewardsManagerInstance::<T, P, N>::new(address, provider)
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
        uniV4PoolManager: alloy::sol_types::private::Address,
        controller: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockRewardsManagerInstance<T, P, N>>,
    > {
        MockRewardsManagerInstance::<
            T,
            P,
            N,
        >::deploy(provider, uniV4PoolManager, controller)
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
        uniV4PoolManager: alloy::sol_types::private::Address,
        controller: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockRewardsManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, uniV4PoolManager, controller)
    }
    /**A [`MockRewardsManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`MockRewardsManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockRewardsManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MockRewardsManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockRewardsManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockRewardsManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`MockRewardsManager`](self) contract instance.

See the [wrapper's documentation](`MockRewardsManagerInstance`) for more details.*/
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
            uniV4PoolManager: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MockRewardsManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                uniV4PoolManager,
                controller,
            );
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
            uniV4PoolManager: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            uniV4PoolManager,
                            controller,
                        },
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
    impl<T, P: ::core::clone::Clone, N> MockRewardsManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockRewardsManagerInstance<T, P, N> {
            MockRewardsManagerInstance {
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
    > MockRewardsManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`beforeAddLiquidity`] function.
        pub fn beforeAddLiquidity(
            &self,
            sender: alloy::sol_types::private::Address,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeAddLiquidityCall, N> {
            self.call_builder(
                &beforeAddLiquidityCall {
                    sender,
                    key,
                    params,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`beforeInitialize`] function.
        pub fn beforeInitialize(
            &self,
            _0: alloy::sol_types::private::Address,
            poolKey: <PoolKey as alloy::sol_types::SolType>::RustType,
            _2: alloy::sol_types::private::primitives::aliases::U160,
            storeIndexBytes: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeInitializeCall, N> {
            self.call_builder(
                &beforeInitializeCall {
                    _0,
                    poolKey,
                    _2,
                    storeIndexBytes,
                },
            )
        }
        ///Creates a new call builder for the [`beforeRemoveLiquidity`] function.
        pub fn beforeRemoveLiquidity(
            &self,
            sender: alloy::sol_types::private::Address,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeRemoveLiquidityCall, N> {
            self.call_builder(
                &beforeRemoveLiquidityCall {
                    sender,
                    key,
                    params,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`configurePool`] function.
        pub fn configurePool(
            &self,
            asset0: alloy::sol_types::private::Address,
            asset1: alloy::sol_types::private::Address,
            tickSpacing: u16,
            feeInE6: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, configurePoolCall, N> {
            self.call_builder(
                &configurePoolCall {
                    asset0,
                    asset1,
                    tickSpacing,
                    feeInE6,
                },
            )
        }
        ///Creates a new call builder for the [`consts`] function.
        pub fn consts(&self) -> alloy_contract::SolCallBuilder<T, &P, constsCall, N> {
            self.call_builder(&constsCall {})
        }
        ///Creates a new call builder for the [`getGrowthInsideRange`] function.
        pub fn getGrowthInsideRange(
            &self,
            id: <PoolId as alloy::sol_types::SolType>::RustType,
            lowerTick: alloy::sol_types::private::primitives::aliases::I24,
            upperTick: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::SolCallBuilder<T, &P, getGrowthInsideRangeCall, N> {
            self.call_builder(
                &getGrowthInsideRangeCall {
                    id,
                    lowerTick,
                    upperTick,
                },
            )
        }
        ///Creates a new call builder for the [`getGrowthInsideTick`] function.
        pub fn getGrowthInsideTick(
            &self,
            id: <PoolId as alloy::sol_types::SolType>::RustType,
            tick: alloy::sol_types::private::primitives::aliases::I24,
            tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::SolCallBuilder<T, &P, getGrowthInsideTickCall, N> {
            self.call_builder(
                &getGrowthInsideTickCall {
                    id,
                    tick,
                    tickSpacing,
                },
            )
        }
        ///Creates a new call builder for the [`toggleNodes`] function.
        pub fn toggleNodes(
            &self,
            nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, toggleNodesCall, N> {
            self.call_builder(&toggleNodesCall { nodes })
        }
        ///Creates a new call builder for the [`update`] function.
        pub fn update(
            &self,
            encoded: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateCall, N> {
            self.call_builder(&updateCall { encoded })
        }
        ///Creates a new call builder for the [`updateAfterTickMove`] function.
        pub fn updateAfterTickMove(
            &self,
            id: <PoolId as alloy::sol_types::SolType>::RustType,
            lastTick: alloy::sol_types::private::primitives::aliases::I24,
            newTick: alloy::sol_types::private::primitives::aliases::I24,
            tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAfterTickMoveCall, N> {
            self.call_builder(
                &updateAfterTickMoveCall {
                    id,
                    lastTick,
                    newTick,
                    tickSpacing,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > MockRewardsManagerInstance<T, P, N> {
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
