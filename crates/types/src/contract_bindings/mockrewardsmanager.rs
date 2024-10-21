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
    error BundleChangeNetNegative(address asset);
    error DuplicateAsset();
    error FailedToDeployNewStore();
    error FeeAboveMax();
    error IndexMayHaveChanged();
    error InvalidPoolKey();
    error InvalidStoreIndex();
    error InvalidTickSpacing();
    error MissingHookPermissions();
    error NoEntry();
    error NotController();
    error NotFeeMaster();
    error NotFromHook();
    error NotNode();
    error NotUniswap();
    error OnlyOncePerBlock();
    error OutOfOrderOrDuplicatePairs();
    error Overflow();
    error PairAccessOutOfBounds(uint256 index, uint256 length);
    error ReaderNotAtEnd();
    error Underflow();
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4, address controller);

    function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function configurePool(address assetA, address assetB, uint16 tickSpacing, uint24 feeInE6) external;
    function consts() external pure returns (uint24 poolFee);
    function deposit(address asset, uint256 amount) external;
    function deposit(address asset, address to, uint256 amount) external;
    function getGrowthInsideRange(PoolId id, int24 lowerTick, int24 upperTick) external view returns (uint256);
    function getGrowthInsideTick(PoolId id, int24 tick, int24 tickSpacing) external view returns (uint256);
    function initializePool(address assetA, address assetB, uint256 storeIndex, uint160 sqrtPriceX96) external;
    function pullFee(address asset, uint256 amount) external;
    function removePool(address expectedStore, uint256 storeIndex) external;
    function toggleNodes(address[] memory nodes) external;
    function update(bytes memory encoded) external;
    function updateAfterTickMove(PoolId id, int24 lastTick, int24 newTick, int24 tickSpacing) external;
    function withdraw(address asset, address to, uint256 amount) external;
    function withdraw(address asset, uint256 amount) external;
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
        "internalType": "contract IPoolManager"
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
        "name": "assetA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "assetB",
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
    "name": "deposit",
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
    "name": "deposit",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
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
    "name": "initializePool",
    "inputs": [
      {
        "name": "assetA",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "assetB",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "storeIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "internalType": "uint160"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "pullFee",
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
    "name": "removePool",
    "inputs": [
      {
        "name": "expectedStore",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "storeIndex",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "type": "function",
    "name": "withdraw",
    "inputs": [
      {
        "name": "asset",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "to",
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
    "name": "withdraw",
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
    "name": "DuplicateAsset",
    "inputs": []
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
    "name": "IndexMayHaveChanged",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPoolKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidStoreIndex",
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
    "inputs": []
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
    "name": "NotFeeMaster",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotFromHook",
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
    ///0x60e060405234801561000f575f80fd5b5060405161348b38038061348b83398101604081905261002e91610093565b6001600160a01b038083166080525f60a052811660c05261004d610054565b50506100cb565b30613fff16612a801461007a5760405163d7ab502760e01b815260040160405180910390fd5b565b6001600160a01b0381168114610090575f80fd5b50565b5f80604083850312156100a4575f80fd5b82516100af8161007c565b60208401519092506100c08161007c565b809150509250929050565b60805160a05160c0516132f56101965f395f61137001525f61105e01525f81816102ce01528181610361015281816104d20152818161054e015281816105ac015281816106760152818161070a0152818161086d01528181610ae601528181610b3d01528181610d2a0152818161115b0152818161162c01528181611dc001528181611dfc01528181611e3d01528181611e8101528181611ecd01528181612441015281816125cd01528181612867015281816128c2015281816129b00152612a0b01526132f55ff3fe608060405234801561000f575f80fd5b50600436106100fb575f3560e01c80638340f54911610093578063d86d744e11610063578063d86d744e14610229578063d9caed1214610237578063d9e17f981461024a578063f3fef3a31461025d575f80fd5b80638340f549146101dd5780638587f450146101f0578063c43ed2c814610203578063d6cffd1e14610216575f80fd5b806347e7ef24116100ce57806347e7ef241461019157806362889dd6146101a457806371cca81b146101b75780637d1f3226146101ca575f80fd5b80630b3dd76e146100ff5780631090641d1461012557806321d0ee701461013a578063259982e51461017e575b5f80fd5b61011261010d366004612c3f565b610270565b6040519081526020015b60405180910390f35b610138610133366004612c9f565b6103af565b005b61014d610148366004612d4a565b61042f565b6040517fffffffff00000000000000000000000000000000000000000000000000000000909116815260200161011c565b61014d61018c366004612d4a565b6107da565b61013861019f366004612e0f565b6109db565b6101126101b2366004612c3f565b610a45565b6101386101c5366004612e39565b610b29565b6101386101d8366004612e0f565b610b6a565b6101386101eb366004612e7e565b610c39565b6101386101fe366004612ebc565b610ca8565b610138610211366004612f01565b610e84565b610138610224366004612f40565b610f47565b6040515f815260200161011c565b610138610245366004612e7e565b610fe0565b610138610258366004612e0f565b611046565b61013861026b366004612e0f565b6110da565b5f6102b184846040518060400160405280600481526020017f7469636b00000000000000000000000000000000000000000000000000000000815250611140565b5f805b6102f673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687878761120f565b909250905081156102b45761034186826040518060400160405280600a81526020017f6e6578745469636b557000000000000000000000000000000000000000000000815250611140565b6103a561039061038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001689611281565b60a01c60020b90565b5f8881526005602052604090209087846112b8565b9695505050505050565b6103b7611358565b6003546103e99068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16858585856113c9565b600360086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b5f610438611614565b5f6104468560400135611683565b90505f610452876116c4565b90505f806104af838b61046860208c018c612fb1565b61047860408d0160208e01612fb1565b6006526003525f90815260608b01356026908152603a600c2090829052918152600460209081526040808320848452909152902091565b90925090505f6104f861038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611281565b90505f6105318261050c60208d018d612fb1565b61051c60408e0160208f01612fb1565b5f8981526005602052604090209291906112b8565b90505f61057573ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168786611710565b85549091505f90610598846fffffffffffffffffffffffffffffffff851661176b565b6105a29190612ff9565b90508015610776577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a58411948e5f0160208101906105f8919061300c565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b15801561065b575f80fd5b505af115801561066d573d5f803e3d5ffd5b505050506106c57f0000000000000000000000000000000000000000000000000000000000000000828f5f0160208101906106a8919061300c565b73ffffffffffffffffffffffffffffffffffffffff169190611796565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8f811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af1158015610750573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107749190613027565b505b6107a5610782896117df565b61078c908461303e565b84906fffffffffffffffffffffffffffffffff1661176b565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f6107e3611614565b60408401355f6107f2876116c4565b90505f6108026020880188612fb1565b90505f6108156040890160208a01612fb1565b90505f60055f8581526020019081526020015f2090505f61084c858d86868e6060013560046116d89095949392919063ffffffff16565b5090505f61089361038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611281565b90505f8362ffffff8716630100000081106108b0576108b0613066565b015490505f8462ffffff8716630100000081106108cf576108cf613066565b015490505f8760020b8460020b121561092257818310156109115781925082865f018962ffffff166301000000811061090a5761090a613066565b0155610980565b61091b8284612ff9565b9050610980565b8360020b8760020b1361095f578282101561095557829150818662ffffff89166301000000811061090a5761090a613066565b61091b8383612ff9565b818387630100000001546109739190612ff9565b61097d9190612ff9565b90505b5f61098b828c61176b565b905080865f015f82825461099f9190613093565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b6109fd73ffffffffffffffffffffffffffffffffffffffff8316333084611804565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260016020908152604080832033845290915281208054839290610a3c908490613093565b90915550505050565b5f610a8684846040518060400160405280600981526020017f6c6f7765725469636b0000000000000000000000000000000000000000000000815250611140565b610ac684836040518060400160405280600981526020017f75707065725469636b0000000000000000000000000000000000000000000000815250611140565b610b21610b0c61038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687611281565b5f8681526005602052604090209085856112b8565b949350505050565b5f848152600560205260409020610b6490857f000000000000000000000000000000000000000000000000000000000000000086868661185c565b50505050565b610b72611358565b60035473ffffffffffffffffffffffffffffffffffffffff6801000000000000000090910481169083168114610bd4576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610bf473ffffffffffffffffffffffffffffffffffffffff8216836118dd565b600360086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050565b610c5b73ffffffffffffffffffffffffffffffffffffffff8416333084611804565b73ffffffffffffffffffffffffffffffffffffffff8084165f90815260016020908152604080832093861683529290529081208054839290610c9e908490613093565b9091555050505050565b8273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff161115610ce0579192915b5f84815260208490526040812060281b6003549091505f90610d259068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff168386611a17565b5090507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16636276cbbe6040518060a00160405280610d768a90565b73ffffffffffffffffffffffffffffffffffffffff1681526020018873ffffffffffffffffffffffffffffffffffffffff90811682525f602080840191909152600287810b6040808601919091523060609586015280517fffffffff0000000000000000000000000000000000000000000000000000000060e089901b168152865185166004820152928601518416602484015285015162ffffff1660448301529284015190920b60648301526080909201518216608482015290861660a482015260c4016020604051808303815f875af1158015610e57573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e7b91906130a6565b50505050505050565b815f610e8f82611a9a565b60035491935091505f90610ec8908490849068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16611b6b565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e08101829052610100810182905261014081019190915263f3cd914c81523060a0820152610120808201529194509150610f32848284611d06565b9350610f3f848787611f42565b505050505050565b610f4f611358565b5f5b81811015610fdb575f838383818110610f6c57610f6c613066565b9050602002016020810190610f81919061300c565b73ffffffffffffffffffffffffffffffffffffffff165f90815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610f51565b505050565b73ffffffffffffffffffffffffffffffffffffffff83165f9081526001602090815260408083203384529091528120805483929061101f908490612ff9565b90915550610fdb905073ffffffffffffffffffffffffffffffffffffffff84168383611796565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146110b5576040517f2833655e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110d673ffffffffffffffffffffffffffffffffffffffff83163383611796565b5050565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260016020908152604080832033845290915281208054839290611119908490612ff9565b909155506110d6905073ffffffffffffffffffffffffffffffffffffffff83163383611796565b5f61118273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168585611f59565b506fffffffffffffffffffffffffffffffff1690505f8111826111a78560020b611fbe565b6040516020016111b89291906130d8565b60405160208183030381529060405290611208576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016111ff9190613142565b60405180910390fd5b5050505050565b5f8080806112346112298688078313878905036001613195565b600281900b60081d91565b90925090506112648161125e73ffffffffffffffffffffffffffffffffffffffff8b168a86611fc9565b9061200d565b90945090506112748282876120cf565b9250505094509492505050565b5f8181526006602052604081206112ae73ffffffffffffffffffffffffffffffffffffffff8516826120f9565b9150505b92915050565b5f808562ffffff8516630100000081106112d4576112d4613066565b015490505f8662ffffff8516630100000081106112f3576112f3613066565b015490508460020b8660020b12156113185761130f8183612ff9565b92505050610b21565b8560020b8460020b1361132f5761130f8282612ff9565b808288630100000001546113439190612ff9565b61134d9190612ff9565b979650505050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146113c7576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff161115611402579293925b8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1603611467576040517f587daa3000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f036114a4576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff831611156114e6576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6115068773ffffffffffffffffffffffffffffffffffffffff16612129565b90505f61151f87875f9182526020526040902060281b90565b90506040516b600b380380600b5f395ff30081526020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156115a25783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008116870361159657828552506115a2565b5060208401935061155a565b908152821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff851691506116099050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146113c7576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f808213156116be576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b6006919091526003919091525f9182526026908152603a600c2090829052918152602092835260408082208383529093529190912091565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa61174f5763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f190483118202156117865763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f511417166117d6576390b8ec185f526004601cfd5b5f603452505050565b5f70010000000000000000000000000000000082106118005761180061214c565b5090565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f5114171661184f57637939f4245f526004601cfd5b5f60605260405250505050565b8260020b8260020b13156118a0578260020b611884828460020b61215990919063ffffffff16565b60020b131561189b5761189b86858786868661216a565b610f3f565b8260020b8260020b1215610f3f575f600284900b828107919091129082900503810260020b8260020b1215610f3f57610f3f8685878686866121fd565b5f806118fe8473ffffffffffffffffffffffffffffffffffffffff16612129565b9050808310611939576040517fd2c6aae600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8060010361194757506112b2565b5f60016119548584612ff9565b61195e9190612ff9565b90506040516b600b380380600b5f395ff30081526020810183602002806001838a3c60208702820191506020840260208301835e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff85169150611a0f9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811685140280611a86576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6003818101915f918291803560e81c0101816044611ab88684612ff9565b611ac291906131d6565b905080602086901b1792505f805b82811015611b5f575f611aee602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611b55576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611ad0565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026611b8d8a84612ff9565b611b9791906131d6565b905060405193508060c0028401925082604052808460201b179450505f5b82841015611cf95760048901983560e081901c905f90611bdd90611ae7908c9060f01c61229c565b90505f611bf1611ae78c61ffff861661229c565b90508363ffffffff168363ffffffff16111580611c3a57508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611c71576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80611cb073ffffffffffffffffffffffffffffffffffffffff8c168585611a17565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611bb5565b5093505050935093915050565b6001838101935f919035821a90611d22908590831615156122fd565b60028501943560f01c611d49611d38858361234e565b805160208201516040909201519092565b60020b608088015273ffffffffffffffffffffffffffffffffffffffff9081166040880152166020860190815260a090205f60108801883560801c9098506fffffffffffffffffffffffffffffffff1690505f8115611eb0575f611de661038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611281565b9050611df1836123ae565b60e08a0152611e20897f0000000000000000000000000000000000000000000000000000000000000000612409565b611e6361038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611281565b60808a01515f868152600560205260409020919350611eaa919086907f0000000000000000000000000000000000000000000000000000000000000000908590879061185c565b50611ef6565b611ef361038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611281565b90505b5f611f1d6002871615155f86815260056020526040902060808c01518d9190889087612427565b60208b0151919b509150611f33905f90836126a4565b50989998505050505050505050565b808201808414610b64576301842f8c5f526004601cfd5b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa611f995763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b60606112b2826126e7565b5f828152600660209081526040808320848452600501909152812061200473ffffffffffffffffffffffffffffffffffffffff8616826120f9565b95945050505050565b5f805f6120a88460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826120be5760ff6120c5565b8360ff1681015b9150509250929050565b5f8160ff84166120e5600187900b61010061320e565b6120ef9190613195565b610b21919061320e565b5f81602052631e2eaeaf5f5260205f6024601c865afa6121205763535cf94b5f526004601cfd5b50505f51919050565b5f6112b2602073ffffffffffffffffffffffffffffffffffffffff84163b6131d6565b6335278d125f526004601cfd5b5f8183071291819005919091030290565b5f61218d73ffffffffffffffffffffffffffffffffffffffff871686868561120f565b94509050600284810b9084900b12156121a65750610f3f565b80156121f7578662ffffff8516630100000081106121c6576121c6613066565b015487630100000001546121da9190612ff9565b8762ffffff8616630100000081106121f4576121f4613066565b01555b5061216a565b5f61222073ffffffffffffffffffffffffffffffffffffffff871686868561273c565b94509050600283810b9085900b136122385750610f3f565b8015612289578662ffffff85166301000000811061225857612258613066565b0154876301000000015461226c9190612ff9565b8762ffffff86166301000000811061228657612286613066565b01555b8361229381613234565b945050506121fd565b5f8163ffffffff8416116122eb576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016111ff565b602083901c60448302015b9392505050565b80151560c0830152806123245773fffd8963efd1fc6a506488495d951d5263988d2561232b565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b5f8163ffffffff84161161239d576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016111ff565b5060c08102602083901c0192915050565b5f7f80000000000000000000000000000000000000000000000000000000000000008211156116be576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af180610fdb576040513d5f823e503d5ff35b5f8087156124c95760108701963560801c6124938161247c7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1689612781565b6fffffffffffffffffffffffffffffffff166127b2565b876301000000015f8282546124a89190613093565b90915550889350506fffffffffffffffffffffffffffffffff169050612699565b5f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b13156125405761253b83888789856127cd565b61254d565b61254d8388878985612916565b909b50995060108201965092503560801c61257a816fffffffffffffffffffffffffffffffff8b166127b2565b612584908b613093565b99506125a26fffffffffffffffffffffffffffffffff821684613093565b92506125ae8686612a60565b81515f906125f39073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690612781565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff161461266e576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c166004830152821660248201526044016111ff565b8a856301000000015f8282546126849190613093565b90915550969c50929a50505050505050505050505b965096945050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f9081526020849052604081206126df6126d6825c85612a99565b92508183612ab1565b509392505050565b60605f82126126f9576112b282612ab8565b6127068219600101612ab8565b8051602d82526001017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90910190815292915050565b5f808080612751858707821386880503611229565b90925090506112648161277b73ffffffffffffffffffffffffffffffffffffffff8b168a86611fc9565b90612b18565b5f8181526006602052604081205f61200473ffffffffffffffffffffffffffffffffffffffff8616600384016120f9565b5f6122f6826127c9670de0b6b3a764000086613290565b0490565b5f808080600181805b82156128a05760108a01993560801c6127ef8184613093565b925061280d818b6fffffffffffffffffffffffffffffffff166127b2565b6128179083613093565b9150818d8d62ffffff166301000000811061283457612834613066565b015f8282546128439190613093565b909155505088515f9061288e9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611f59565b91505061289b8b82612be0565b9a5050505b875160208901516128ea9173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90612bfa565b809c508194505050876040015160020b8b60020b136127d657989b909a50979850959695505050505050565b5f808080600181805b82156129e95760108a01993560801c6129388184613093565b9250612956818b6fffffffffffffffffffffffffffffffff166127b2565b6129609083613093565b9150818d8d62ffffff166301000000811061297d5761297d613066565b015f82825461298c9190613093565b909155505088515f906129d79073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611f59565b9150506129e48b82612c14565b9a5050505b87516020890151612a339173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e9061120f565b809c508194505050876040015160020b8b60020b131561291f57989b909a50979850959695505050505050565b8082146110d6576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808203828113156112b25763c9654ed45f526004601cfd5b80825d5050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a900480612ad15750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b5f805f8360ff0390505f612bb98260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083612bce575f612bd5565b8160ff1681035b925050509250929050565b808203608081901c156112b25763c9654ed45f526004601cfd5b5f80808061275161122960018789078413888a05036132a7565b818101608081901c156112b25763c9654ed45f526004601cfd5b8060020b8114612c3c575f80fd5b50565b5f805f60608486031215612c51575f80fd5b833592506020840135612c6381612c2e565b91506040840135612c7381612c2e565b809150509250925092565b73ffffffffffffffffffffffffffffffffffffffff81168114612c3c575f80fd5b5f805f8060808587031215612cb2575f80fd5b8435612cbd81612c7e565b93506020850135612ccd81612c7e565b9250604085013561ffff81168114612ce3575f80fd5b9150606085013562ffffff81168114612cfa575f80fd5b939692955090935050565b5f8083601f840112612d15575f80fd5b50813567ffffffffffffffff811115612d2c575f80fd5b602083019150836020828501011115612d43575f80fd5b9250929050565b5f805f805f858703610160811215612d60575f80fd5b8635612d6b81612c7e565b955060a07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe082011215612d9c575f80fd5b60208701945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215612dd1575f80fd5b5060c08601925061014086013567ffffffffffffffff811115612df2575f80fd5b612dfe88828901612d05565b969995985093965092949392505050565b5f8060408385031215612e20575f80fd5b8235612e2b81612c7e565b946020939093013593505050565b5f805f8060808587031215612e4c575f80fd5b843593506020850135612e5e81612c2e565b92506040850135612e6e81612c2e565b91506060850135612cfa81612c2e565b5f805f60608486031215612e90575f80fd5b8335612e9b81612c7e565b92506020840135612eab81612c7e565b929592945050506040919091013590565b5f805f8060808587031215612ecf575f80fd5b8435612eda81612c7e565b93506020850135612eea81612c7e565b9250604085013591506060850135612cfa81612c7e565b5f8060208385031215612f12575f80fd5b823567ffffffffffffffff811115612f28575f80fd5b612f3485828601612d05565b90969095509350505050565b5f8060208385031215612f51575f80fd5b823567ffffffffffffffff811115612f67575f80fd5b8301601f81018513612f77575f80fd5b803567ffffffffffffffff811115612f8d575f80fd5b8560208260051b8401011115612fa1575f80fd5b6020919091019590945092505050565b5f60208284031215612fc1575f80fd5b81356122f681612c2e565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b818103818111156112b2576112b2612fcc565b5f6020828403121561301c575f80fd5b81356122f681612c7e565b5f60208284031215613037575f80fd5b5051919050565b6fffffffffffffffffffffffffffffffff82811682821603908111156112b2576112b2612fcc565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b808201808211156112b2576112b2612fcc565b5f602082840312156130b6575f80fd5b81516122f681612c2e565b5f81518060208401855e5f93019283525090919050565b5f6130e382856130c1565b7f205b000000000000000000000000000000000000000000000000000000000000815261311360028201856130c1565b7f5d206e6f7420696e697469616c697a6564000000000000000000000000000000815260110195945050505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156112b2576112b2612fcc565b5f82613209577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f8260020b8260020b028060020b915080821461322d5761322d612fcc565b5092915050565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000810361326857613268612fcc565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b80820281158282048414176112b2576112b2612fcc565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156112b2576112b2612fcc56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa4\x8B8\x03\x80a4\x8B\x839\x81\x01`@\x81\x90Ra\0.\x91a\0\x93V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x80R_`\xA0R\x81\x16`\xC0Ra\0Ma\0TV[PPa\0\xCBV[0a?\xFF\x16a*\x80\x14a\0zW`@Qc\xD7\xABP'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x90W_\x80\xFD[PV[_\x80`@\x83\x85\x03\x12\x15a\0\xA4W_\x80\xFD[\x82Qa\0\xAF\x81a\0|V[` \x84\x01Q\x90\x92Pa\0\xC0\x81a\0|V[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Qa2\xF5a\x01\x96_9_a\x13p\x01R_a\x10^\x01R_\x81\x81a\x02\xCE\x01R\x81\x81a\x03a\x01R\x81\x81a\x04\xD2\x01R\x81\x81a\x05N\x01R\x81\x81a\x05\xAC\x01R\x81\x81a\x06v\x01R\x81\x81a\x07\n\x01R\x81\x81a\x08m\x01R\x81\x81a\n\xE6\x01R\x81\x81a\x0B=\x01R\x81\x81a\r*\x01R\x81\x81a\x11[\x01R\x81\x81a\x16,\x01R\x81\x81a\x1D\xC0\x01R\x81\x81a\x1D\xFC\x01R\x81\x81a\x1E=\x01R\x81\x81a\x1E\x81\x01R\x81\x81a\x1E\xCD\x01R\x81\x81a$A\x01R\x81\x81a%\xCD\x01R\x81\x81a(g\x01R\x81\x81a(\xC2\x01R\x81\x81a)\xB0\x01Ra*\x0B\x01Ra2\xF5_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80c\x83@\xF5I\x11a\0\x93W\x80c\xD8mtN\x11a\0cW\x80c\xD8mtN\x14a\x02)W\x80c\xD9\xCA\xED\x12\x14a\x027W\x80c\xD9\xE1\x7F\x98\x14a\x02JW\x80c\xF3\xFE\xF3\xA3\x14a\x02]W_\x80\xFD[\x80c\x83@\xF5I\x14a\x01\xDDW\x80c\x85\x87\xF4P\x14a\x01\xF0W\x80c\xC4>\xD2\xC8\x14a\x02\x03W\x80c\xD6\xCF\xFD\x1E\x14a\x02\x16W_\x80\xFD[\x80cG\xE7\xEF$\x11a\0\xCEW\x80cG\xE7\xEF$\x14a\x01\x91W\x80cb\x88\x9D\xD6\x14a\x01\xA4W\x80cq\xCC\xA8\x1B\x14a\x01\xB7W\x80c}\x1F2&\x14a\x01\xCAW_\x80\xFD[\x80c\x0B=\xD7n\x14a\0\xFFW\x80c\x10\x90d\x1D\x14a\x01%W\x80c!\xD0\xEEp\x14a\x01:W\x80c%\x99\x82\xE5\x14a\x01~W[_\x80\xFD[a\x01\x12a\x01\r6`\x04a,?V[a\x02pV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x0136`\x04a,\x9FV[a\x03\xAFV[\0[a\x01Ma\x01H6`\x04a-JV[a\x04/V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x01Ma\x01\x8C6`\x04a-JV[a\x07\xDAV[a\x018a\x01\x9F6`\x04a.\x0FV[a\t\xDBV[a\x01\x12a\x01\xB26`\x04a,?V[a\nEV[a\x018a\x01\xC56`\x04a.9V[a\x0B)V[a\x018a\x01\xD86`\x04a.\x0FV[a\x0BjV[a\x018a\x01\xEB6`\x04a.~V[a\x0C9V[a\x018a\x01\xFE6`\x04a.\xBCV[a\x0C\xA8V[a\x018a\x02\x116`\x04a/\x01V[a\x0E\x84V[a\x018a\x02$6`\x04a/@V[a\x0FGV[`@Q_\x81R` \x01a\x01\x1CV[a\x018a\x02E6`\x04a.~V[a\x0F\xE0V[a\x018a\x02X6`\x04a.\x0FV[a\x10FV[a\x018a\x02k6`\x04a.\x0FV[a\x10\xDAV[_a\x02\xB1\x84\x84`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[_\x80[a\x02\xF6s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x87\x87a\x12\x0FV[\x90\x92P\x90P\x81\x15a\x02\xB4Wa\x03A\x86\x82`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FnextTickUp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[a\x03\xA5a\x03\x90a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x89a\x12\x81V[`\xA0\x1C`\x02\x0B\x90V[_\x88\x81R`\x05` R`@\x90 \x90\x87\x84a\x12\xB8V[\x96\x95PPPPPPV[a\x03\xB7a\x13XV[`\x03Ta\x03\xE9\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x13\xC9V[`\x03`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[_a\x048a\x16\x14V[_a\x04F\x85`@\x015a\x16\x83V[\x90P_a\x04R\x87a\x16\xC4V[\x90P_\x80a\x04\xAF\x83\x8Ba\x04h` \x8C\x01\x8Ca/\xB1V[a\x04x`@\x8D\x01` \x8E\x01a/\xB1V[`\x06R`\x03R_\x90\x81R``\x8B\x015`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R`\x04` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\x04\xF8a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x12\x81V[\x90P_a\x051\x82a\x05\x0C` \x8D\x01\x8Da/\xB1V[a\x05\x1C`@\x8E\x01` \x8F\x01a/\xB1V[_\x89\x81R`\x05` R`@\x90 \x92\x91\x90a\x12\xB8V[\x90P_a\x05us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x17\x10V[\x85T\x90\x91P_\x90a\x05\x98\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x17kV[a\x05\xA2\x91\x90a/\xF9V[\x90P\x80\x15a\x07vW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94\x8E_\x01` \x81\x01\x90a\x05\xF8\x91\x90a0\x0CV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06[W_\x80\xFD[PZ\xF1\x15\x80\x15a\x06mW=_\x80>=_\xFD[PPPPa\x06\xC5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8F_\x01` \x81\x01\x90a\x06\xA8\x91\x90a0\x0CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90a\x17\x96V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07PW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07t\x91\x90a0'V[P[a\x07\xA5a\x07\x82\x89a\x17\xDFV[a\x07\x8C\x90\x84a0>V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17kV[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_a\x07\xE3a\x16\x14V[`@\x84\x015_a\x07\xF2\x87a\x16\xC4V[\x90P_a\x08\x02` \x88\x01\x88a/\xB1V[\x90P_a\x08\x15`@\x89\x01` \x8A\x01a/\xB1V[\x90P_`\x05_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x08L\x85\x8D\x86\x86\x8E``\x015`\x04a\x16\xD8\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x08\x93a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x12\x81V[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xB0Wa\x08\xB0a0fV[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xCFWa\x08\xCFa0fV[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\t\"W\x81\x83\x10\x15a\t\x11W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\t\nWa\t\na0fV[\x01Ua\t\x80V[a\t\x1B\x82\x84a/\xF9V[\x90Pa\t\x80V[\x83`\x02\x0B\x87`\x02\x0B\x13a\t_W\x82\x82\x10\x15a\tUW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\t\nWa\t\na0fV[a\t\x1B\x83\x83a/\xF9V[\x81\x83\x87c\x01\0\0\0\x01Ta\ts\x91\x90a/\xF9V[a\t}\x91\x90a/\xF9V[\x90P[_a\t\x8B\x82\x8Ca\x17kV[\x90P\x80\x86_\x01_\x82\x82Ta\t\x9F\x91\x90a0\x93V[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[a\t\xFDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x1630\x84a\x18\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\n<\x90\x84\x90a0\x93V[\x90\x91UPPPPV[_a\n\x86\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FlowerTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[a\n\xC6\x84\x83`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FupperTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[a\x0B!a\x0B\x0Ca\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87a\x12\x81V[_\x86\x81R`\x05` R`@\x90 \x90\x85\x85a\x12\xB8V[\x94\x93PPPPV[_\x84\x81R`\x05` R`@\x90 a\x0Bd\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x86\x86a\x18\\V[PPPPV[a\x0Bra\x13XV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x81\x14a\x0B\xD4W`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83a\x18\xDDV[`\x03`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[a\x0C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x1630\x84a\x18\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x0C\x9E\x90\x84\x90a0\x93V[\x90\x91UPPPPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xE0W\x91\x92\x91[_\x84\x81R` \x84\x90R`@\x81 `(\x1B`\x03T\x90\x91P_\x90a\r%\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x86a\x1A\x17V[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cbv\xCB\xBE`@Q\x80`\xA0\x01`@R\x80a\rv\x8A\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R_` \x80\x84\x01\x91\x90\x91R`\x02\x87\x81\x0B`@\x80\x86\x01\x91\x90\x91R0``\x95\x86\x01R\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x81R\x86Q\x85\x16`\x04\x82\x01R\x92\x86\x01Q\x84\x16`$\x84\x01R\x85\x01Qb\xFF\xFF\xFF\x16`D\x83\x01R\x92\x84\x01Q\x90\x92\x0B`d\x83\x01R`\x80\x90\x92\x01Q\x82\x16`\x84\x82\x01R\x90\x86\x16`\xA4\x82\x01R`\xC4\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0EWW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E{\x91\x90a0\xA6V[PPPPPPPV[\x81_a\x0E\x8F\x82a\x1A\x9AV[`\x03T\x91\x93P\x91P_\x90a\x0E\xC8\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1BkV[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R\x91\x94P\x91Pa\x0F2\x84\x82\x84a\x1D\x06V[\x93Pa\x0F?\x84\x87\x87a\x1FBV[PPPPPPV[a\x0FOa\x13XV[_[\x81\x81\x10\x15a\x0F\xDBW_\x83\x83\x83\x81\x81\x10a\x0FlWa\x0Fla0fV[\x90P` \x02\x01` \x81\x01\x90a\x0F\x81\x91\x90a0\x0CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x02` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x0FQV[PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x10\x1F\x90\x84\x90a/\xF9V[\x90\x91UPa\x0F\xDB\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x17\x96V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xB5W`@Q\x7F(3e^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xD6s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x17\x96V[PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x11\x19\x90\x84\x90a/\xF9V[\x90\x91UPa\x10\xD6\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x17\x96V[_a\x11\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\x1FYV[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x11\x82a\x11\xA7\x85`\x02\x0Ba\x1F\xBEV[`@Q` \x01a\x11\xB8\x92\x91\x90a0\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x12\x08W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xFF\x91\x90a1BV[`@Q\x80\x91\x03\x90\xFD[PPPPPV[_\x80\x80\x80a\x124a\x12)\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a1\x95V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa\x12d\x81a\x12^s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1F\xC9V[\x90a \rV[\x90\x94P\x90Pa\x12t\x82\x82\x87a \xCFV[\x92PPP\x94P\x94\x92PPPV[_\x81\x81R`\x06` R`@\x81 a\x12\xAEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a \xF9V[\x91PP[\x92\x91PPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x12\xD4Wa\x12\xD4a0fV[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x12\xF3Wa\x12\xF3a0fV[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x13\x18Wa\x13\x0F\x81\x83a/\xF9V[\x92PPPa\x0B!V[\x85`\x02\x0B\x84`\x02\x0B\x13a\x13/Wa\x13\x0F\x82\x82a/\xF9V[\x80\x82\x88c\x01\0\0\0\x01Ta\x13C\x91\x90a/\xF9V[a\x13M\x91\x90a/\xF9V[\x97\x96PPPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13\xC7W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x14\x02W\x92\x93\x92[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x14gW`@Q\x7FX}\xAA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x14\xA4W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x14\xE6W`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\x06\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!)V[\x90P_a\x15\x1F\x87\x87_\x91\x82R` R`@\x90 `(\x1B\x90V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x15\xA2W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x15\x96W\x82\x85RPa\x15\xA2V[P` \x84\x01\x93Pa\x15ZV[\x90\x81R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x16\t\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13\xC7W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x82\x13\x15a\x16\xBEW`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`\x06\x91\x90\x91R`\x03\x91\x90\x91R_\x91\x82R`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R` \x92\x83R`@\x80\x82 \x83\x83R\x90\x93R\x91\x90\x91 \x91V[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x17OWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x17\x86Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x17\xD6Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x18\0Wa\x18\0a!LV[P\x90V[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a\x18OWcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a\x18\xA0W\x82`\x02\x0Ba\x18\x84\x82\x84`\x02\x0Ba!Y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a\x18\x9BWa\x18\x9B\x86\x85\x87\x86\x86\x86a!jV[a\x0F?V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a\x0F?W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a\x0F?Wa\x0F?\x86\x85\x87\x86\x86\x86a!\xFDV[_\x80a\x18\xFE\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!)V[\x90P\x80\x83\x10a\x199W`@Q\x7F\xD2\xC6\xAA\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x03a\x19GWPa\x12\xB2V[_`\x01a\x19T\x85\x84a/\xF9V[a\x19^\x91\x90a/\xF9V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8A<` \x87\x02\x82\x01\x91P` \x84\x02` \x83\x01\x83^\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x1A\x0F\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x1A\x86W`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x1A\xB8\x86\x84a/\xF9V[a\x1A\xC2\x91\x90a1\xD6V[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x1B_W_a\x1A\xEE` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1BUW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x1A\xD0V[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x1B\x8D\x8A\x84a/\xF9V[a\x1B\x97\x91\x90a1\xD6V[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x1C\xF9W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x1B\xDD\x90a\x1A\xE7\x90\x8C\x90`\xF0\x1Ca\"\x9CV[\x90P_a\x1B\xF1a\x1A\xE7\x8Ca\xFF\xFF\x86\x16a\"\x9CV[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x1C:WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1CqW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x1C\xB0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x1A\x17V[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x1B\xB5V[P\x93PPP\x93P\x93\x91PPV[`\x01\x83\x81\x01\x93_\x91\x905\x82\x1A\x90a\x1D\"\x90\x85\x90\x83\x16\x15\x15a\"\xFDV[`\x02\x85\x01\x945`\xF0\x1Ca\x1DIa\x1D8\x85\x83a#NV[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x88\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x88\x01R\x16` \x86\x01\x90\x81R`\xA0\x90 _`\x10\x88\x01\x885`\x80\x1C\x90\x98Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a\x1E\xB0W_a\x1D\xE6a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x12\x81V[\x90Pa\x1D\xF1\x83a#\xAEV[`\xE0\x8A\x01Ra\x1E \x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$\tV[a\x1Eca\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x12\x81V[`\x80\x8A\x01Q_\x86\x81R`\x05` R`@\x90 \x91\x93Pa\x1E\xAA\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a\x18\\V[Pa\x1E\xF6V[a\x1E\xF3a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x12\x81V[\x90P[_a\x1F\x1D`\x02\x87\x16\x15\x15_\x86\x81R`\x05` R`@\x90 `\x80\x8C\x01Q\x8D\x91\x90\x88\x90\x87a$'V[` \x8B\x01Q\x91\x9BP\x91Pa\x1F3\x90_\x90\x83a&\xA4V[P\x98\x99\x98PPPPPPPPPV[\x80\x82\x01\x80\x84\x14a\x0BdWc\x01\x84/\x8C_R`\x04`\x1C\xFD[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa\x1F\x99WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[``a\x12\xB2\x82a&\xE7V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a \x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a \xF9V[\x95\x94PPPPPV[_\x80_a \xA8\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a \xBEW`\xFFa \xC5V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a \xE5`\x01\x87\x90\x0Ba\x01\0a2\x0EV[a \xEF\x91\x90a1\x95V[a\x0B!\x91\x90a2\x0EV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa! WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[_a\x12\xB2` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;a1\xD6V[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a!\x8Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a\x12\x0FV[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a!\xA6WPa\x0F?V[\x80\x15a!\xF7W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a!\xC6Wa!\xC6a0fV[\x01T\x87c\x01\0\0\0\x01Ta!\xDA\x91\x90a/\xF9V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a!\xF4Wa!\xF4a0fV[\x01U[Pa!jV[_a\" s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a'<V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a\"8WPa\x0F?V[\x80\x15a\"\x89W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\"XWa\"Xa0fV[\x01T\x87c\x01\0\0\0\x01Ta\"l\x91\x90a/\xF9V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\"\x86Wa\"\x86a0fV[\x01U[\x83a\"\x93\x81a24V[\x94PPPa!\xFDV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a\"\xEBW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x11\xFFV[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[\x80\x15\x15`\xC0\x83\x01R\x80a#$Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a#+V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a#\x9DW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x11\xFFV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x16\xBEW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x0F\xDBW`@Q=_\x82>P=_\xF3[_\x80\x87\x15a$\xC9W`\x10\x87\x01\x965`\x80\x1Ca$\x93\x81a$|\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a'\x81V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xB2V[\x87c\x01\0\0\0\x01_\x82\x82Ta$\xA8\x91\x90a0\x93V[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa&\x99V[_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a%@Wa%;\x83\x88\x87\x89\x85a'\xCDV[a%MV[a%M\x83\x88\x87\x89\x85a)\x16V[\x90\x9BP\x99P`\x10\x82\x01\x96P\x92P5`\x80\x1Ca%z\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16a'\xB2V[a%\x84\x90\x8Ba0\x93V[\x99Pa%\xA2o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84a0\x93V[\x92Pa%\xAE\x86\x86a*`V[\x81Q_\x90a%\xF3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a'\x81V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&nW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x11\xFFV[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta&\x84\x91\x90a0\x93V[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a&\xDFa&\xD6\x82\\\x85a*\x99V[\x92P\x81\x83a*\xB1V[P\x93\x92PPPV[``_\x82\x12a&\xF9Wa\x12\xB2\x82a*\xB8V[a'\x06\x82\x19`\x01\x01a*\xB8V[\x80Q`-\x82R`\x01\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x01\x90\x81R\x92\x91PPV[_\x80\x80\x80a'Q\x85\x87\x07\x82\x13\x86\x88\x05\x03a\x12)V[\x90\x92P\x90Pa\x12d\x81a'{s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1F\xC9V[\x90a+\x18V[_\x81\x81R`\x06` R`@\x81 _a \x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a \xF9V[_a\"\xF6\x82a'\xC9g\r\xE0\xB6\xB3\xA7d\0\0\x86a2\x90V[\x04\x90V[_\x80\x80\x80`\x01\x81\x80[\x82\x15a(\xA0W`\x10\x8A\x01\x995`\x80\x1Ca'\xEF\x81\x84a0\x93V[\x92Pa(\r\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xB2V[a(\x17\x90\x83a0\x93V[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a(4Wa(4a0fV[\x01_\x82\x82Ta(C\x91\x90a0\x93V[\x90\x91UPP\x88Q_\x90a(\x8E\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1FYV[\x91PPa(\x9B\x8B\x82a+\xE0V[\x9APPP[\x87Q` \x89\x01Qa(\xEA\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a+\xFAV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a'\xD6W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80\x80\x80`\x01\x81\x80[\x82\x15a)\xE9W`\x10\x8A\x01\x995`\x80\x1Ca)8\x81\x84a0\x93V[\x92Pa)V\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xB2V[a)`\x90\x83a0\x93V[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a)}Wa)}a0fV[\x01_\x82\x82Ta)\x8C\x91\x90a0\x93V[\x90\x91UPP\x88Q_\x90a)\xD7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1FYV[\x91PPa)\xE4\x8B\x82a,\x14V[\x9APPP[\x87Q` \x89\x01Qa*3\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a\x12\x0FV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a)\x1FW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a\x10\xD6W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x12\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a*\xD1WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[_\x80_\x83`\xFF\x03\x90P_a+\xB9\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a+\xCEW_a+\xD5V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x12\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a'Qa\x12)`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03a2\xA7V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x12\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80`\x02\x0B\x81\x14a,<W_\x80\xFD[PV[_\x80_``\x84\x86\x03\x12\x15a,QW_\x80\xFD[\x835\x92P` \x84\x015a,c\x81a,.V[\x91P`@\x84\x015a,s\x81a,.V[\x80\x91PP\x92P\x92P\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,<W_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a,\xB2W_\x80\xFD[\x845a,\xBD\x81a,~V[\x93P` \x85\x015a,\xCD\x81a,~V[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14a,\xE3W_\x80\xFD[\x91P``\x85\x015b\xFF\xFF\xFF\x81\x16\x81\x14a,\xFAW_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_\x80\x83`\x1F\x84\x01\x12a-\x15W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-,W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a-CW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15a-`W_\x80\xFD[\x865a-k\x81a,~V[\x95P`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x01\x12\x15a-\x9CW_\x80\xFD[` \x87\x01\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a-\xD1W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xF2W_\x80\xFD[a-\xFE\x88\x82\x89\x01a-\x05V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a. W_\x80\xFD[\x825a.+\x81a,~V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a.LW_\x80\xFD[\x845\x93P` \x85\x015a.^\x81a,.V[\x92P`@\x85\x015a.n\x81a,.V[\x91P``\x85\x015a,\xFA\x81a,.V[_\x80_``\x84\x86\x03\x12\x15a.\x90W_\x80\xFD[\x835a.\x9B\x81a,~V[\x92P` \x84\x015a.\xAB\x81a,~V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x80_\x80`\x80\x85\x87\x03\x12\x15a.\xCFW_\x80\xFD[\x845a.\xDA\x81a,~V[\x93P` \x85\x015a.\xEA\x81a,~V[\x92P`@\x85\x015\x91P``\x85\x015a,\xFA\x81a,~V[_\x80` \x83\x85\x03\x12\x15a/\x12W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/(W_\x80\xFD[a/4\x85\x82\x86\x01a-\x05V[\x90\x96\x90\x95P\x93PPPPV[_\x80` \x83\x85\x03\x12\x15a/QW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/gW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a/wW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x8DW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a/\xA1W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a/\xC1W_\x80\xFD[\x815a\"\xF6\x81a,.V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12\xB2Wa\x12\xB2a/\xCCV[_` \x82\x84\x03\x12\x15a0\x1CW_\x80\xFD[\x815a\"\xF6\x81a,~V[_` \x82\x84\x03\x12\x15a07W_\x80\xFD[PQ\x91\x90PV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x12\xB2Wa\x12\xB2a/\xCCV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12\xB2Wa\x12\xB2a/\xCCV[_` \x82\x84\x03\x12\x15a0\xB6W_\x80\xFD[\x81Qa\"\xF6\x81a,.V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a0\xE3\x82\x85a0\xC1V[\x7F [\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra1\x13`\x02\x82\x01\x85a0\xC1V[\x7F] not initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11\x01\x95\x94PPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x12\xB2Wa\x12\xB2a/\xCCV[_\x82a2\tW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a2-Wa2-a/\xCCV[P\x92\x91PPV[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a2hWa2ha/\xCCV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12\xB2Wa\x12\xB2a/\xCCV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x12\xB2Wa\x12\xB2a/\xCCV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100fb575f3560e01c80638340f54911610093578063d86d744e11610063578063d86d744e14610229578063d9caed1214610237578063d9e17f981461024a578063f3fef3a31461025d575f80fd5b80638340f549146101dd5780638587f450146101f0578063c43ed2c814610203578063d6cffd1e14610216575f80fd5b806347e7ef24116100ce57806347e7ef241461019157806362889dd6146101a457806371cca81b146101b75780637d1f3226146101ca575f80fd5b80630b3dd76e146100ff5780631090641d1461012557806321d0ee701461013a578063259982e51461017e575b5f80fd5b61011261010d366004612c3f565b610270565b6040519081526020015b60405180910390f35b610138610133366004612c9f565b6103af565b005b61014d610148366004612d4a565b61042f565b6040517fffffffff00000000000000000000000000000000000000000000000000000000909116815260200161011c565b61014d61018c366004612d4a565b6107da565b61013861019f366004612e0f565b6109db565b6101126101b2366004612c3f565b610a45565b6101386101c5366004612e39565b610b29565b6101386101d8366004612e0f565b610b6a565b6101386101eb366004612e7e565b610c39565b6101386101fe366004612ebc565b610ca8565b610138610211366004612f01565b610e84565b610138610224366004612f40565b610f47565b6040515f815260200161011c565b610138610245366004612e7e565b610fe0565b610138610258366004612e0f565b611046565b61013861026b366004612e0f565b6110da565b5f6102b184846040518060400160405280600481526020017f7469636b00000000000000000000000000000000000000000000000000000000815250611140565b5f805b6102f673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687878761120f565b909250905081156102b45761034186826040518060400160405280600a81526020017f6e6578745469636b557000000000000000000000000000000000000000000000815250611140565b6103a561039061038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001689611281565b60a01c60020b90565b5f8881526005602052604090209087846112b8565b9695505050505050565b6103b7611358565b6003546103e99068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16858585856113c9565b600360086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b5f610438611614565b5f6104468560400135611683565b90505f610452876116c4565b90505f806104af838b61046860208c018c612fb1565b61047860408d0160208e01612fb1565b6006526003525f90815260608b01356026908152603a600c2090829052918152600460209081526040808320848452909152902091565b90925090505f6104f861038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611281565b90505f6105318261050c60208d018d612fb1565b61051c60408e0160208f01612fb1565b5f8981526005602052604090209291906112b8565b90505f61057573ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168786611710565b85549091505f90610598846fffffffffffffffffffffffffffffffff851661176b565b6105a29190612ff9565b90508015610776577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a58411948e5f0160208101906105f8919061300c565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b15801561065b575f80fd5b505af115801561066d573d5f803e3d5ffd5b505050506106c57f0000000000000000000000000000000000000000000000000000000000000000828f5f0160208101906106a8919061300c565b73ffffffffffffffffffffffffffffffffffffffff169190611796565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8f811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af1158015610750573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107749190613027565b505b6107a5610782896117df565b61078c908461303e565b84906fffffffffffffffffffffffffffffffff1661176b565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f6107e3611614565b60408401355f6107f2876116c4565b90505f6108026020880188612fb1565b90505f6108156040890160208a01612fb1565b90505f60055f8581526020019081526020015f2090505f61084c858d86868e6060013560046116d89095949392919063ffffffff16565b5090505f61089361038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611281565b90505f8362ffffff8716630100000081106108b0576108b0613066565b015490505f8462ffffff8716630100000081106108cf576108cf613066565b015490505f8760020b8460020b121561092257818310156109115781925082865f018962ffffff166301000000811061090a5761090a613066565b0155610980565b61091b8284612ff9565b9050610980565b8360020b8760020b1361095f578282101561095557829150818662ffffff89166301000000811061090a5761090a613066565b61091b8383612ff9565b818387630100000001546109739190612ff9565b61097d9190612ff9565b90505b5f61098b828c61176b565b905080865f015f82825461099f9190613093565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b6109fd73ffffffffffffffffffffffffffffffffffffffff8316333084611804565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260016020908152604080832033845290915281208054839290610a3c908490613093565b90915550505050565b5f610a8684846040518060400160405280600981526020017f6c6f7765725469636b0000000000000000000000000000000000000000000000815250611140565b610ac684836040518060400160405280600981526020017f75707065725469636b0000000000000000000000000000000000000000000000815250611140565b610b21610b0c61038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001687611281565b5f8681526005602052604090209085856112b8565b949350505050565b5f848152600560205260409020610b6490857f000000000000000000000000000000000000000000000000000000000000000086868661185c565b50505050565b610b72611358565b60035473ffffffffffffffffffffffffffffffffffffffff6801000000000000000090910481169083168114610bd4576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610bf473ffffffffffffffffffffffffffffffffffffffff8216836118dd565b600360086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050565b610c5b73ffffffffffffffffffffffffffffffffffffffff8416333084611804565b73ffffffffffffffffffffffffffffffffffffffff8084165f90815260016020908152604080832093861683529290529081208054839290610c9e908490613093565b9091555050505050565b8273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff161115610ce0579192915b5f84815260208490526040812060281b6003549091505f90610d259068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff168386611a17565b5090507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16636276cbbe6040518060a00160405280610d768a90565b73ffffffffffffffffffffffffffffffffffffffff1681526020018873ffffffffffffffffffffffffffffffffffffffff90811682525f602080840191909152600287810b6040808601919091523060609586015280517fffffffff0000000000000000000000000000000000000000000000000000000060e089901b168152865185166004820152928601518416602484015285015162ffffff1660448301529284015190920b60648301526080909201518216608482015290861660a482015260c4016020604051808303815f875af1158015610e57573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610e7b91906130a6565b50505050505050565b815f610e8f82611a9a565b60035491935091505f90610ec8908490849068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16611b6b565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e08101829052610100810182905261014081019190915263f3cd914c81523060a0820152610120808201529194509150610f32848284611d06565b9350610f3f848787611f42565b505050505050565b610f4f611358565b5f5b81811015610fdb575f838383818110610f6c57610f6c613066565b9050602002016020810190610f81919061300c565b73ffffffffffffffffffffffffffffffffffffffff165f90815260026020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610f51565b505050565b73ffffffffffffffffffffffffffffffffffffffff83165f9081526001602090815260408083203384529091528120805483929061101f908490612ff9565b90915550610fdb905073ffffffffffffffffffffffffffffffffffffffff84168383611796565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146110b5576040517f2833655e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110d673ffffffffffffffffffffffffffffffffffffffff83163383611796565b5050565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260016020908152604080832033845290915281208054839290611119908490612ff9565b909155506110d6905073ffffffffffffffffffffffffffffffffffffffff83163383611796565b5f61118273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168585611f59565b506fffffffffffffffffffffffffffffffff1690505f8111826111a78560020b611fbe565b6040516020016111b89291906130d8565b60405160208183030381529060405290611208576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016111ff9190613142565b60405180910390fd5b5050505050565b5f8080806112346112298688078313878905036001613195565b600281900b60081d91565b90925090506112648161125e73ffffffffffffffffffffffffffffffffffffffff8b168a86611fc9565b9061200d565b90945090506112748282876120cf565b9250505094509492505050565b5f8181526006602052604081206112ae73ffffffffffffffffffffffffffffffffffffffff8516826120f9565b9150505b92915050565b5f808562ffffff8516630100000081106112d4576112d4613066565b015490505f8662ffffff8516630100000081106112f3576112f3613066565b015490508460020b8660020b12156113185761130f8183612ff9565b92505050610b21565b8560020b8460020b1361132f5761130f8282612ff9565b808288630100000001546113439190612ff9565b61134d9190612ff9565b979650505050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146113c7576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff161115611402579293925b8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1603611467576040517f587daa3000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f036114a4576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff831611156114e6576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6115068773ffffffffffffffffffffffffffffffffffffffff16612129565b90505f61151f87875f9182526020526040902060281b90565b90506040516b600b380380600b5f395ff30081526020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156115a25783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff00000000008116870361159657828552506115a2565b5060208401935061155a565b908152821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff851691506116099050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146113c7576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f808213156116be576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b6006919091526003919091525f9182526026908152603a600c2090829052918152602092835260408082208383529093529190912091565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa61174f5763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f190483118202156117865763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f511417166117d6576390b8ec185f526004601cfd5b5f603452505050565b5f70010000000000000000000000000000000082106118005761180061214c565b5090565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f5114171661184f57637939f4245f526004601cfd5b5f60605260405250505050565b8260020b8260020b13156118a0578260020b611884828460020b61215990919063ffffffff16565b60020b131561189b5761189b86858786868661216a565b610f3f565b8260020b8260020b1215610f3f575f600284900b828107919091129082900503810260020b8260020b1215610f3f57610f3f8685878686866121fd565b5f806118fe8473ffffffffffffffffffffffffffffffffffffffff16612129565b9050808310611939576040517fd2c6aae600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8060010361194757506112b2565b5f60016119548584612ff9565b61195e9190612ff9565b90506040516b600b380380600b5f395ff30081526020810183602002806001838a3c60208702820191506020840260208301835e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff85169150611a0f9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811685140280611a86576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6003818101915f918291803560e81c0101816044611ab88684612ff9565b611ac291906131d6565b905080602086901b1792505f805b82811015611b5f575f611aee602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611b55576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611ad0565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026611b8d8a84612ff9565b611b9791906131d6565b905060405193508060c0028401925082604052808460201b179450505f5b82841015611cf95760048901983560e081901c905f90611bdd90611ae7908c9060f01c61229c565b90505f611bf1611ae78c61ffff861661229c565b90508363ffffffff168363ffffffff16111580611c3a57508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611c71576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80611cb073ffffffffffffffffffffffffffffffffffffffff8c168585611a17565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611bb5565b5093505050935093915050565b6001838101935f919035821a90611d22908590831615156122fd565b60028501943560f01c611d49611d38858361234e565b805160208201516040909201519092565b60020b608088015273ffffffffffffffffffffffffffffffffffffffff9081166040880152166020860190815260a090205f60108801883560801c9098506fffffffffffffffffffffffffffffffff1690505f8115611eb0575f611de661038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611281565b9050611df1836123ae565b60e08a0152611e20897f0000000000000000000000000000000000000000000000000000000000000000612409565b611e6361038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611281565b60808a01515f868152600560205260409020919350611eaa919086907f0000000000000000000000000000000000000000000000000000000000000000908590879061185c565b50611ef6565b611ef361038773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611281565b90505b5f611f1d6002871615155f86815260056020526040902060808c01518d9190889087612427565b60208b0151919b509150611f33905f90836126a4565b50989998505050505050505050565b808201808414610b64576301842f8c5f526004601cfd5b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa611f995763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b60606112b2826126e7565b5f828152600660209081526040808320848452600501909152812061200473ffffffffffffffffffffffffffffffffffffffff8616826120f9565b95945050505050565b5f805f6120a88460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826120be5760ff6120c5565b8360ff1681015b9150509250929050565b5f8160ff84166120e5600187900b61010061320e565b6120ef9190613195565b610b21919061320e565b5f81602052631e2eaeaf5f5260205f6024601c865afa6121205763535cf94b5f526004601cfd5b50505f51919050565b5f6112b2602073ffffffffffffffffffffffffffffffffffffffff84163b6131d6565b6335278d125f526004601cfd5b5f8183071291819005919091030290565b5f61218d73ffffffffffffffffffffffffffffffffffffffff871686868561120f565b94509050600284810b9084900b12156121a65750610f3f565b80156121f7578662ffffff8516630100000081106121c6576121c6613066565b015487630100000001546121da9190612ff9565b8762ffffff8616630100000081106121f4576121f4613066565b01555b5061216a565b5f61222073ffffffffffffffffffffffffffffffffffffffff871686868561273c565b94509050600283810b9085900b136122385750610f3f565b8015612289578662ffffff85166301000000811061225857612258613066565b0154876301000000015461226c9190612ff9565b8762ffffff86166301000000811061228657612286613066565b01555b8361229381613234565b945050506121fd565b5f8163ffffffff8416116122eb576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016111ff565b602083901c60448302015b9392505050565b80151560c0830152806123245773fffd8963efd1fc6a506488495d951d5263988d2561232b565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b5f8163ffffffff84161161239d576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff841660248201526044016111ff565b5060c08102602083901c0192915050565b5f7f80000000000000000000000000000000000000000000000000000000000000008211156116be576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af180610fdb576040513d5f823e503d5ff35b5f8087156124c95760108701963560801c6124938161247c7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1689612781565b6fffffffffffffffffffffffffffffffff166127b2565b876301000000015f8282546124a89190613093565b90915550889350506fffffffffffffffffffffffffffffffff169050612699565b5f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b13156125405761253b83888789856127cd565b61254d565b61254d8388878985612916565b909b50995060108201965092503560801c61257a816fffffffffffffffffffffffffffffffff8b166127b2565b612584908b613093565b99506125a26fffffffffffffffffffffffffffffffff821684613093565b92506125ae8686612a60565b81515f906125f39073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690612781565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff161461266e576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c166004830152821660248201526044016111ff565b8a856301000000015f8282546126849190613093565b90915550969c50929a50505050505050505050505b965096945050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f9081526020849052604081206126df6126d6825c85612a99565b92508183612ab1565b509392505050565b60605f82126126f9576112b282612ab8565b6127068219600101612ab8565b8051602d82526001017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff90910190815292915050565b5f808080612751858707821386880503611229565b90925090506112648161277b73ffffffffffffffffffffffffffffffffffffffff8b168a86611fc9565b90612b18565b5f8181526006602052604081205f61200473ffffffffffffffffffffffffffffffffffffffff8616600384016120f9565b5f6122f6826127c9670de0b6b3a764000086613290565b0490565b5f808080600181805b82156128a05760108a01993560801c6127ef8184613093565b925061280d818b6fffffffffffffffffffffffffffffffff166127b2565b6128179083613093565b9150818d8d62ffffff166301000000811061283457612834613066565b015f8282546128439190613093565b909155505088515f9061288e9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611f59565b91505061289b8b82612be0565b9a5050505b875160208901516128ea9173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90612bfa565b809c508194505050876040015160020b8b60020b136127d657989b909a50979850959695505050505050565b5f808080600181805b82156129e95760108a01993560801c6129388184613093565b9250612956818b6fffffffffffffffffffffffffffffffff166127b2565b6129609083613093565b9150818d8d62ffffff166301000000811061297d5761297d613066565b015f82825461298c9190613093565b909155505088515f906129d79073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f611f59565b9150506129e48b82612c14565b9a5050505b87516020890151612a339173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e9061120f565b809c508194505050876040015160020b8b60020b131561291f57989b909a50979850959695505050505050565b8082146110d6576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808203828113156112b25763c9654ed45f526004601cfd5b80825d5050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a900480612ad15750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b5f805f8360ff0390505f612bb98260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083612bce575f612bd5565b8160ff1681035b925050509250929050565b808203608081901c156112b25763c9654ed45f526004601cfd5b5f80808061275161122960018789078413888a05036132a7565b818101608081901c156112b25763c9654ed45f526004601cfd5b8060020b8114612c3c575f80fd5b50565b5f805f60608486031215612c51575f80fd5b833592506020840135612c6381612c2e565b91506040840135612c7381612c2e565b809150509250925092565b73ffffffffffffffffffffffffffffffffffffffff81168114612c3c575f80fd5b5f805f8060808587031215612cb2575f80fd5b8435612cbd81612c7e565b93506020850135612ccd81612c7e565b9250604085013561ffff81168114612ce3575f80fd5b9150606085013562ffffff81168114612cfa575f80fd5b939692955090935050565b5f8083601f840112612d15575f80fd5b50813567ffffffffffffffff811115612d2c575f80fd5b602083019150836020828501011115612d43575f80fd5b9250929050565b5f805f805f858703610160811215612d60575f80fd5b8635612d6b81612c7e565b955060a07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe082011215612d9c575f80fd5b60208701945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215612dd1575f80fd5b5060c08601925061014086013567ffffffffffffffff811115612df2575f80fd5b612dfe88828901612d05565b969995985093965092949392505050565b5f8060408385031215612e20575f80fd5b8235612e2b81612c7e565b946020939093013593505050565b5f805f8060808587031215612e4c575f80fd5b843593506020850135612e5e81612c2e565b92506040850135612e6e81612c2e565b91506060850135612cfa81612c2e565b5f805f60608486031215612e90575f80fd5b8335612e9b81612c7e565b92506020840135612eab81612c7e565b929592945050506040919091013590565b5f805f8060808587031215612ecf575f80fd5b8435612eda81612c7e565b93506020850135612eea81612c7e565b9250604085013591506060850135612cfa81612c7e565b5f8060208385031215612f12575f80fd5b823567ffffffffffffffff811115612f28575f80fd5b612f3485828601612d05565b90969095509350505050565b5f8060208385031215612f51575f80fd5b823567ffffffffffffffff811115612f67575f80fd5b8301601f81018513612f77575f80fd5b803567ffffffffffffffff811115612f8d575f80fd5b8560208260051b8401011115612fa1575f80fd5b6020919091019590945092505050565b5f60208284031215612fc1575f80fd5b81356122f681612c2e565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b818103818111156112b2576112b2612fcc565b5f6020828403121561301c575f80fd5b81356122f681612c7e565b5f60208284031215613037575f80fd5b5051919050565b6fffffffffffffffffffffffffffffffff82811682821603908111156112b2576112b2612fcc565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b808201808211156112b2576112b2612fcc565b5f602082840312156130b6575f80fd5b81516122f681612c2e565b5f81518060208401855e5f93019283525090919050565b5f6130e382856130c1565b7f205b000000000000000000000000000000000000000000000000000000000000815261311360028201856130c1565b7f5d206e6f7420696e697469616c697a6564000000000000000000000000000000815260110195945050505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156112b2576112b2612fcc565b5f82613209577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f8260020b8260020b028060020b915080821461322d5761322d612fcc565b5092915050565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000810361326857613268612fcc565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b80820281158282048414176112b2576112b2612fcc565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156112b2576112b2612fcc56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xFBW_5`\xE0\x1C\x80c\x83@\xF5I\x11a\0\x93W\x80c\xD8mtN\x11a\0cW\x80c\xD8mtN\x14a\x02)W\x80c\xD9\xCA\xED\x12\x14a\x027W\x80c\xD9\xE1\x7F\x98\x14a\x02JW\x80c\xF3\xFE\xF3\xA3\x14a\x02]W_\x80\xFD[\x80c\x83@\xF5I\x14a\x01\xDDW\x80c\x85\x87\xF4P\x14a\x01\xF0W\x80c\xC4>\xD2\xC8\x14a\x02\x03W\x80c\xD6\xCF\xFD\x1E\x14a\x02\x16W_\x80\xFD[\x80cG\xE7\xEF$\x11a\0\xCEW\x80cG\xE7\xEF$\x14a\x01\x91W\x80cb\x88\x9D\xD6\x14a\x01\xA4W\x80cq\xCC\xA8\x1B\x14a\x01\xB7W\x80c}\x1F2&\x14a\x01\xCAW_\x80\xFD[\x80c\x0B=\xD7n\x14a\0\xFFW\x80c\x10\x90d\x1D\x14a\x01%W\x80c!\xD0\xEEp\x14a\x01:W\x80c%\x99\x82\xE5\x14a\x01~W[_\x80\xFD[a\x01\x12a\x01\r6`\x04a,?V[a\x02pV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x018a\x0136`\x04a,\x9FV[a\x03\xAFV[\0[a\x01Ma\x01H6`\x04a-JV[a\x04/V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x01\x1CV[a\x01Ma\x01\x8C6`\x04a-JV[a\x07\xDAV[a\x018a\x01\x9F6`\x04a.\x0FV[a\t\xDBV[a\x01\x12a\x01\xB26`\x04a,?V[a\nEV[a\x018a\x01\xC56`\x04a.9V[a\x0B)V[a\x018a\x01\xD86`\x04a.\x0FV[a\x0BjV[a\x018a\x01\xEB6`\x04a.~V[a\x0C9V[a\x018a\x01\xFE6`\x04a.\xBCV[a\x0C\xA8V[a\x018a\x02\x116`\x04a/\x01V[a\x0E\x84V[a\x018a\x02$6`\x04a/@V[a\x0FGV[`@Q_\x81R` \x01a\x01\x1CV[a\x018a\x02E6`\x04a.~V[a\x0F\xE0V[a\x018a\x02X6`\x04a.\x0FV[a\x10FV[a\x018a\x02k6`\x04a.\x0FV[a\x10\xDAV[_a\x02\xB1\x84\x84`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[_\x80[a\x02\xF6s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x87\x87a\x12\x0FV[\x90\x92P\x90P\x81\x15a\x02\xB4Wa\x03A\x86\x82`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FnextTickUp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[a\x03\xA5a\x03\x90a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x89a\x12\x81V[`\xA0\x1C`\x02\x0B\x90V[_\x88\x81R`\x05` R`@\x90 \x90\x87\x84a\x12\xB8V[\x96\x95PPPPPPV[a\x03\xB7a\x13XV[`\x03Ta\x03\xE9\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x13\xC9V[`\x03`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[_a\x048a\x16\x14V[_a\x04F\x85`@\x015a\x16\x83V[\x90P_a\x04R\x87a\x16\xC4V[\x90P_\x80a\x04\xAF\x83\x8Ba\x04h` \x8C\x01\x8Ca/\xB1V[a\x04x`@\x8D\x01` \x8E\x01a/\xB1V[`\x06R`\x03R_\x90\x81R``\x8B\x015`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R`\x04` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\x04\xF8a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x12\x81V[\x90P_a\x051\x82a\x05\x0C` \x8D\x01\x8Da/\xB1V[a\x05\x1C`@\x8E\x01` \x8F\x01a/\xB1V[_\x89\x81R`\x05` R`@\x90 \x92\x91\x90a\x12\xB8V[\x90P_a\x05us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x17\x10V[\x85T\x90\x91P_\x90a\x05\x98\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x17kV[a\x05\xA2\x91\x90a/\xF9V[\x90P\x80\x15a\x07vW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94\x8E_\x01` \x81\x01\x90a\x05\xF8\x91\x90a0\x0CV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06[W_\x80\xFD[PZ\xF1\x15\x80\x15a\x06mW=_\x80>=_\xFD[PPPPa\x06\xC5\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8F_\x01` \x81\x01\x90a\x06\xA8\x91\x90a0\x0CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90a\x17\x96V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07PW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07t\x91\x90a0'V[P[a\x07\xA5a\x07\x82\x89a\x17\xDFV[a\x07\x8C\x90\x84a0>V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17kV[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_a\x07\xE3a\x16\x14V[`@\x84\x015_a\x07\xF2\x87a\x16\xC4V[\x90P_a\x08\x02` \x88\x01\x88a/\xB1V[\x90P_a\x08\x15`@\x89\x01` \x8A\x01a/\xB1V[\x90P_`\x05_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x08L\x85\x8D\x86\x86\x8E``\x015`\x04a\x16\xD8\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x08\x93a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x12\x81V[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xB0Wa\x08\xB0a0fV[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xCFWa\x08\xCFa0fV[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\t\"W\x81\x83\x10\x15a\t\x11W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\t\nWa\t\na0fV[\x01Ua\t\x80V[a\t\x1B\x82\x84a/\xF9V[\x90Pa\t\x80V[\x83`\x02\x0B\x87`\x02\x0B\x13a\t_W\x82\x82\x10\x15a\tUW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\t\nWa\t\na0fV[a\t\x1B\x83\x83a/\xF9V[\x81\x83\x87c\x01\0\0\0\x01Ta\ts\x91\x90a/\xF9V[a\t}\x91\x90a/\xF9V[\x90P[_a\t\x8B\x82\x8Ca\x17kV[\x90P\x80\x86_\x01_\x82\x82Ta\t\x9F\x91\x90a0\x93V[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[a\t\xFDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x1630\x84a\x18\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\n<\x90\x84\x90a0\x93V[\x90\x91UPPPPV[_a\n\x86\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FlowerTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[a\n\xC6\x84\x83`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FupperTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x11@V[a\x0B!a\x0B\x0Ca\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87a\x12\x81V[_\x86\x81R`\x05` R`@\x90 \x90\x85\x85a\x12\xB8V[\x94\x93PPPPV[_\x84\x81R`\x05` R`@\x90 a\x0Bd\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x86\x86a\x18\\V[PPPPV[a\x0Bra\x13XV[`\x03Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x81\x14a\x0B\xD4W`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0B\xF4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83a\x18\xDDV[`\x03`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[a\x0C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x1630\x84a\x18\x04V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x0C\x9E\x90\x84\x90a0\x93V[\x90\x91UPPPPPV[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0C\xE0W\x91\x92\x91[_\x84\x81R` \x84\x90R`@\x81 `(\x1B`\x03T\x90\x91P_\x90a\r%\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x86a\x1A\x17V[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cbv\xCB\xBE`@Q\x80`\xA0\x01`@R\x80a\rv\x8A\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R_` \x80\x84\x01\x91\x90\x91R`\x02\x87\x81\x0B`@\x80\x86\x01\x91\x90\x91R0``\x95\x86\x01R\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x81R\x86Q\x85\x16`\x04\x82\x01R\x92\x86\x01Q\x84\x16`$\x84\x01R\x85\x01Qb\xFF\xFF\xFF\x16`D\x83\x01R\x92\x84\x01Q\x90\x92\x0B`d\x83\x01R`\x80\x90\x92\x01Q\x82\x16`\x84\x82\x01R\x90\x86\x16`\xA4\x82\x01R`\xC4\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0EWW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E{\x91\x90a0\xA6V[PPPPPPPV[\x81_a\x0E\x8F\x82a\x1A\x9AV[`\x03T\x91\x93P\x91P_\x90a\x0E\xC8\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1BkV[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R\x91\x94P\x91Pa\x0F2\x84\x82\x84a\x1D\x06V[\x93Pa\x0F?\x84\x87\x87a\x1FBV[PPPPPPV[a\x0FOa\x13XV[_[\x81\x81\x10\x15a\x0F\xDBW_\x83\x83\x83\x81\x81\x10a\x0FlWa\x0Fla0fV[\x90P` \x02\x01` \x81\x01\x90a\x0F\x81\x91\x90a0\x0CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x02` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x0FQV[PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x10\x1F\x90\x84\x90a/\xF9V[\x90\x91UPa\x0F\xDB\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x17\x96V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x10\xB5W`@Q\x7F(3e^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xD6s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x17\x96V[PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x01` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x11\x19\x90\x84\x90a/\xF9V[\x90\x91UPa\x10\xD6\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x17\x96V[_a\x11\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\x1FYV[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x11\x82a\x11\xA7\x85`\x02\x0Ba\x1F\xBEV[`@Q` \x01a\x11\xB8\x92\x91\x90a0\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x12\x08W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11\xFF\x91\x90a1BV[`@Q\x80\x91\x03\x90\xFD[PPPPPV[_\x80\x80\x80a\x124a\x12)\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01a1\x95V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa\x12d\x81a\x12^s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1F\xC9V[\x90a \rV[\x90\x94P\x90Pa\x12t\x82\x82\x87a \xCFV[\x92PPP\x94P\x94\x92PPPV[_\x81\x81R`\x06` R`@\x81 a\x12\xAEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a \xF9V[\x91PP[\x92\x91PPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x12\xD4Wa\x12\xD4a0fV[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x12\xF3Wa\x12\xF3a0fV[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x13\x18Wa\x13\x0F\x81\x83a/\xF9V[\x92PPPa\x0B!V[\x85`\x02\x0B\x84`\x02\x0B\x13a\x13/Wa\x13\x0F\x82\x82a/\xF9V[\x80\x82\x88c\x01\0\0\0\x01Ta\x13C\x91\x90a/\xF9V[a\x13M\x91\x90a/\xF9V[\x97\x96PPPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13\xC7W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x14\x02W\x92\x93\x92[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x14gW`@Q\x7FX}\xAA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x14\xA4W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x14\xE6W`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\x06\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!)V[\x90P_a\x15\x1F\x87\x87_\x91\x82R` R`@\x90 `(\x1B\x90V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x15\xA2W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x15\x96W\x82\x85RPa\x15\xA2V[P` \x84\x01\x93Pa\x15ZV[\x90\x81R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x16\t\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x13\xC7W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x82\x13\x15a\x16\xBEW`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`\x06\x91\x90\x91R`\x03\x91\x90\x91R_\x91\x82R`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R` \x92\x83R`@\x80\x82 \x83\x83R\x90\x93R\x91\x90\x91 \x91V[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x17OWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x17\x86Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x17\xD6Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x18\0Wa\x18\0a!LV[P\x90V[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a\x18OWcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a\x18\xA0W\x82`\x02\x0Ba\x18\x84\x82\x84`\x02\x0Ba!Y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a\x18\x9BWa\x18\x9B\x86\x85\x87\x86\x86\x86a!jV[a\x0F?V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a\x0F?W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a\x0F?Wa\x0F?\x86\x85\x87\x86\x86\x86a!\xFDV[_\x80a\x18\xFE\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!)V[\x90P\x80\x83\x10a\x199W`@Q\x7F\xD2\xC6\xAA\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x03a\x19GWPa\x12\xB2V[_`\x01a\x19T\x85\x84a/\xF9V[a\x19^\x91\x90a/\xF9V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8A<` \x87\x02\x82\x01\x91P` \x84\x02` \x83\x01\x83^\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x1A\x0F\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x1A\x86W`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x1A\xB8\x86\x84a/\xF9V[a\x1A\xC2\x91\x90a1\xD6V[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x1B_W_a\x1A\xEE` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1BUW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x1A\xD0V[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x1B\x8D\x8A\x84a/\xF9V[a\x1B\x97\x91\x90a1\xD6V[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x1C\xF9W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x1B\xDD\x90a\x1A\xE7\x90\x8C\x90`\xF0\x1Ca\"\x9CV[\x90P_a\x1B\xF1a\x1A\xE7\x8Ca\xFF\xFF\x86\x16a\"\x9CV[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x1C:WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1CqW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x1C\xB0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x1A\x17V[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x1B\xB5V[P\x93PPP\x93P\x93\x91PPV[`\x01\x83\x81\x01\x93_\x91\x905\x82\x1A\x90a\x1D\"\x90\x85\x90\x83\x16\x15\x15a\"\xFDV[`\x02\x85\x01\x945`\xF0\x1Ca\x1DIa\x1D8\x85\x83a#NV[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x88\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x88\x01R\x16` \x86\x01\x90\x81R`\xA0\x90 _`\x10\x88\x01\x885`\x80\x1C\x90\x98Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a\x1E\xB0W_a\x1D\xE6a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x12\x81V[\x90Pa\x1D\xF1\x83a#\xAEV[`\xE0\x8A\x01Ra\x1E \x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a$\tV[a\x1Eca\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x12\x81V[`\x80\x8A\x01Q_\x86\x81R`\x05` R`@\x90 \x91\x93Pa\x1E\xAA\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a\x18\\V[Pa\x1E\xF6V[a\x1E\xF3a\x03\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x12\x81V[\x90P[_a\x1F\x1D`\x02\x87\x16\x15\x15_\x86\x81R`\x05` R`@\x90 `\x80\x8C\x01Q\x8D\x91\x90\x88\x90\x87a$'V[` \x8B\x01Q\x91\x9BP\x91Pa\x1F3\x90_\x90\x83a&\xA4V[P\x98\x99\x98PPPPPPPPPV[\x80\x82\x01\x80\x84\x14a\x0BdWc\x01\x84/\x8C_R`\x04`\x1C\xFD[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa\x1F\x99WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[``a\x12\xB2\x82a&\xE7V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a \x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a \xF9V[\x95\x94PPPPPV[_\x80_a \xA8\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a \xBEW`\xFFa \xC5V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a \xE5`\x01\x87\x90\x0Ba\x01\0a2\x0EV[a \xEF\x91\x90a1\x95V[a\x0B!\x91\x90a2\x0EV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa! WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[_a\x12\xB2` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;a1\xD6V[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a!\x8Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a\x12\x0FV[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a!\xA6WPa\x0F?V[\x80\x15a!\xF7W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a!\xC6Wa!\xC6a0fV[\x01T\x87c\x01\0\0\0\x01Ta!\xDA\x91\x90a/\xF9V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a!\xF4Wa!\xF4a0fV[\x01U[Pa!jV[_a\" s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a'<V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a\"8WPa\x0F?V[\x80\x15a\"\x89W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\"XWa\"Xa0fV[\x01T\x87c\x01\0\0\0\x01Ta\"l\x91\x90a/\xF9V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\"\x86Wa\"\x86a0fV[\x01U[\x83a\"\x93\x81a24V[\x94PPPa!\xFDV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a\"\xEBW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x11\xFFV[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[\x80\x15\x15`\xC0\x83\x01R\x80a#$Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a#+V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a#\x9DW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x11\xFFV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x16\xBEW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x0F\xDBW`@Q=_\x82>P=_\xF3[_\x80\x87\x15a$\xC9W`\x10\x87\x01\x965`\x80\x1Ca$\x93\x81a$|\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a'\x81V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xB2V[\x87c\x01\0\0\0\x01_\x82\x82Ta$\xA8\x91\x90a0\x93V[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa&\x99V[_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a%@Wa%;\x83\x88\x87\x89\x85a'\xCDV[a%MV[a%M\x83\x88\x87\x89\x85a)\x16V[\x90\x9BP\x99P`\x10\x82\x01\x96P\x92P5`\x80\x1Ca%z\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16a'\xB2V[a%\x84\x90\x8Ba0\x93V[\x99Pa%\xA2o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84a0\x93V[\x92Pa%\xAE\x86\x86a*`V[\x81Q_\x90a%\xF3\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a'\x81V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&nW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x11\xFFV[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta&\x84\x91\x90a0\x93V[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a&\xDFa&\xD6\x82\\\x85a*\x99V[\x92P\x81\x83a*\xB1V[P\x93\x92PPPV[``_\x82\x12a&\xF9Wa\x12\xB2\x82a*\xB8V[a'\x06\x82\x19`\x01\x01a*\xB8V[\x80Q`-\x82R`\x01\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x01\x90\x81R\x92\x91PPV[_\x80\x80\x80a'Q\x85\x87\x07\x82\x13\x86\x88\x05\x03a\x12)V[\x90\x92P\x90Pa\x12d\x81a'{s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a\x1F\xC9V[\x90a+\x18V[_\x81\x81R`\x06` R`@\x81 _a \x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a \xF9V[_a\"\xF6\x82a'\xC9g\r\xE0\xB6\xB3\xA7d\0\0\x86a2\x90V[\x04\x90V[_\x80\x80\x80`\x01\x81\x80[\x82\x15a(\xA0W`\x10\x8A\x01\x995`\x80\x1Ca'\xEF\x81\x84a0\x93V[\x92Pa(\r\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xB2V[a(\x17\x90\x83a0\x93V[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a(4Wa(4a0fV[\x01_\x82\x82Ta(C\x91\x90a0\x93V[\x90\x91UPP\x88Q_\x90a(\x8E\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1FYV[\x91PPa(\x9B\x8B\x82a+\xE0V[\x9APPP[\x87Q` \x89\x01Qa(\xEA\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a+\xFAV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a'\xD6W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80\x80\x80`\x01\x81\x80[\x82\x15a)\xE9W`\x10\x8A\x01\x995`\x80\x1Ca)8\x81\x84a0\x93V[\x92Pa)V\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xB2V[a)`\x90\x83a0\x93V[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a)}Wa)}a0fV[\x01_\x82\x82Ta)\x8C\x91\x90a0\x93V[\x90\x91UPP\x88Q_\x90a)\xD7\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa\x1FYV[\x91PPa)\xE4\x8B\x82a,\x14V[\x9APPP[\x87Q` \x89\x01Qa*3\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a\x12\x0FV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a)\x1FW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a\x10\xD6W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x12\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a*\xD1WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[_\x80_\x83`\xFF\x03\x90P_a+\xB9\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a+\xCEW_a+\xD5V[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x12\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a'Qa\x12)`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03a2\xA7V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x12\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80`\x02\x0B\x81\x14a,<W_\x80\xFD[PV[_\x80_``\x84\x86\x03\x12\x15a,QW_\x80\xFD[\x835\x92P` \x84\x015a,c\x81a,.V[\x91P`@\x84\x015a,s\x81a,.V[\x80\x91PP\x92P\x92P\x92V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,<W_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a,\xB2W_\x80\xFD[\x845a,\xBD\x81a,~V[\x93P` \x85\x015a,\xCD\x81a,~V[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14a,\xE3W_\x80\xFD[\x91P``\x85\x015b\xFF\xFF\xFF\x81\x16\x81\x14a,\xFAW_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_\x80\x83`\x1F\x84\x01\x12a-\x15W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-,W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a-CW_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15a-`W_\x80\xFD[\x865a-k\x81a,~V[\x95P`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x01\x12\x15a-\x9CW_\x80\xFD[` \x87\x01\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a-\xD1W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a-\xF2W_\x80\xFD[a-\xFE\x88\x82\x89\x01a-\x05V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a. W_\x80\xFD[\x825a.+\x81a,~V[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a.LW_\x80\xFD[\x845\x93P` \x85\x015a.^\x81a,.V[\x92P`@\x85\x015a.n\x81a,.V[\x91P``\x85\x015a,\xFA\x81a,.V[_\x80_``\x84\x86\x03\x12\x15a.\x90W_\x80\xFD[\x835a.\x9B\x81a,~V[\x92P` \x84\x015a.\xAB\x81a,~V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x80_\x80`\x80\x85\x87\x03\x12\x15a.\xCFW_\x80\xFD[\x845a.\xDA\x81a,~V[\x93P` \x85\x015a.\xEA\x81a,~V[\x92P`@\x85\x015\x91P``\x85\x015a,\xFA\x81a,~V[_\x80` \x83\x85\x03\x12\x15a/\x12W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/(W_\x80\xFD[a/4\x85\x82\x86\x01a-\x05V[\x90\x96\x90\x95P\x93PPPPV[_\x80` \x83\x85\x03\x12\x15a/QW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/gW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a/wW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\x8DW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a/\xA1W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_` \x82\x84\x03\x12\x15a/\xC1W_\x80\xFD[\x815a\"\xF6\x81a,.V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12\xB2Wa\x12\xB2a/\xCCV[_` \x82\x84\x03\x12\x15a0\x1CW_\x80\xFD[\x815a\"\xF6\x81a,~V[_` \x82\x84\x03\x12\x15a07W_\x80\xFD[PQ\x91\x90PV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x12\xB2Wa\x12\xB2a/\xCCV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12\xB2Wa\x12\xB2a/\xCCV[_` \x82\x84\x03\x12\x15a0\xB6W_\x80\xFD[\x81Qa\"\xF6\x81a,.V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a0\xE3\x82\x85a0\xC1V[\x7F [\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra1\x13`\x02\x82\x01\x85a0\xC1V[\x7F] not initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x11\x01\x95\x94PPPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x12\xB2Wa\x12\xB2a/\xCCV[_\x82a2\tW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a2-Wa2-a/\xCCV[P\x92\x91PPV[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a2hWa2ha/\xCCV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12\xB2Wa\x12\xB2a/\xCCV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x12\xB2Wa\x12\xB2a/\xCCV\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Custom error with signature `DuplicateAsset()` and selector `0x587daa30`.
```solidity
error DuplicateAsset();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct DuplicateAsset {}
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
        impl ::core::convert::From<DuplicateAsset> for UnderlyingRustTuple<'_> {
            fn from(value: DuplicateAsset) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DuplicateAsset {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DuplicateAsset {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DuplicateAsset()";
            const SELECTOR: [u8; 4] = [88u8, 125u8, 170u8, 48u8];
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
    /**Custom error with signature `IndexMayHaveChanged()` and selector `0xf21fd99f`.
```solidity
error IndexMayHaveChanged();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct IndexMayHaveChanged {}
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
        impl ::core::convert::From<IndexMayHaveChanged> for UnderlyingRustTuple<'_> {
            fn from(value: IndexMayHaveChanged) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for IndexMayHaveChanged {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for IndexMayHaveChanged {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IndexMayHaveChanged()";
            const SELECTOR: [u8; 4] = [242u8, 31u8, 217u8, 159u8];
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
    /**Custom error with signature `InvalidStoreIndex()` and selector `0xd2c6aae6`.
```solidity
error InvalidStoreIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidStoreIndex {}
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
        impl ::core::convert::From<InvalidStoreIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidStoreIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidStoreIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidStoreIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidStoreIndex()";
            const SELECTOR: [u8; 4] = [210u8, 198u8, 170u8, 230u8];
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
    /**Custom error with signature `MissingHookPermissions()` and selector `0xd7ab5027`.
```solidity
error MissingHookPermissions();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MissingHookPermissions {}
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
        impl ::core::convert::From<MissingHookPermissions> for UnderlyingRustTuple<'_> {
            fn from(value: MissingHookPermissions) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MissingHookPermissions {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MissingHookPermissions {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MissingHookPermissions()";
            const SELECTOR: [u8; 4] = [215u8, 171u8, 80u8, 39u8];
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
    /**Custom error with signature `NotFeeMaster()` and selector `0x2833655e`.
```solidity
error NotFeeMaster();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NotFeeMaster {}
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
        impl ::core::convert::From<NotFeeMaster> for UnderlyingRustTuple<'_> {
            fn from(value: NotFeeMaster) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotFeeMaster {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotFeeMaster {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotFeeMaster()";
            const SELECTOR: [u8; 4] = [40u8, 51u8, 101u8, 94u8];
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
    /**Custom error with signature `NotFromHook()` and selector `0x7ad71ceb`.
```solidity
error NotFromHook();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NotFromHook {}
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
        impl ::core::convert::From<NotFromHook> for UnderlyingRustTuple<'_> {
            fn from(value: NotFromHook) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotFromHook {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotFromHook {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotFromHook()";
            const SELECTOR: [u8; 4] = [122u8, 215u8, 28u8, 235u8];
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
constructor(address uniV4, address controller);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub uniV4: alloy::sol_types::private::Address,
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
                    (value.uniV4, value.controller)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        uniV4: tuple.0,
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
                        &self.uniV4,
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
function configurePool(address assetA, address assetB, uint16 tickSpacing, uint24 feeInE6) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct configurePoolCall {
        pub assetA: alloy::sol_types::private::Address,
        pub assetB: alloy::sol_types::private::Address,
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
                    (value.assetA, value.assetB, value.tickSpacing, value.feeInE6)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for configurePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assetA: tuple.0,
                        assetB: tuple.1,
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
                        &self.assetA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetB,
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
    /**Function with signature `deposit(address,uint256)` and selector `0x47e7ef24`.
```solidity
function deposit(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deposit_0Call {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,uint256)`](deposit_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deposit_0Return {}
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
            impl ::core::convert::From<deposit_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Call) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Call {
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
            impl ::core::convert::From<deposit_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,uint256)";
            const SELECTOR: [u8; 4] = [71u8, 231u8, 239u8, 36u8];
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
    /**Function with signature `deposit(address,address,uint256)` and selector `0x8340f549`.
```solidity
function deposit(address asset, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deposit_1Call {
        pub asset: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`deposit(address,address,uint256)`](deposit_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct deposit_1Return {}
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
            impl ::core::convert::From<deposit_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Call) -> Self {
                    (value.asset, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        to: tuple.1,
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
            impl ::core::convert::From<deposit_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: deposit_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deposit_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deposit_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deposit_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deposit(address,address,uint256)";
            const SELECTOR: [u8; 4] = [131u8, 64u8, 245u8, 73u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
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
    /**Function with signature `initializePool(address,address,uint256,uint160)` and selector `0x8587f450`.
```solidity
function initializePool(address assetA, address assetB, uint256 storeIndex, uint160 sqrtPriceX96) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializePoolCall {
        pub assetA: alloy::sol_types::private::Address,
        pub assetB: alloy::sol_types::private::Address,
        pub storeIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`initializePool(address,address,uint256,uint160)`](initializePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializePoolReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.assetA, value.assetB, value.storeIndex, value.sqrtPriceX96)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        assetA: tuple.0,
                        assetB: tuple.1,
                        storeIndex: tuple.2,
                        sqrtPriceX96: tuple.3,
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
            impl ::core::convert::From<initializePoolReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializePoolReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<160>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializePoolReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializePool(address,address,uint256,uint160)";
            const SELECTOR: [u8; 4] = [133u8, 135u8, 244u8, 80u8];
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
                        &self.assetA,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.assetB,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.storeIndex),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
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
    /**Function with signature `pullFee(address,uint256)` and selector `0xd9e17f98`.
```solidity
function pullFee(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct pullFeeCall {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`pullFee(address,uint256)`](pullFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct pullFeeReturn {}
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
            impl ::core::convert::From<pullFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: pullFeeCall) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pullFeeCall {
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
            impl ::core::convert::From<pullFeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pullFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pullFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pullFeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pullFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pullFee(address,uint256)";
            const SELECTOR: [u8; 4] = [217u8, 225u8, 127u8, 152u8];
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
    /**Function with signature `removePool(address,uint256)` and selector `0x7d1f3226`.
```solidity
function removePool(address expectedStore, uint256 storeIndex) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct removePoolCall {
        pub expectedStore: alloy::sol_types::private::Address,
        pub storeIndex: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`removePool(address,uint256)`](removePoolCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct removePoolReturn {}
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
            impl ::core::convert::From<removePoolCall> for UnderlyingRustTuple<'_> {
                fn from(value: removePoolCall) -> Self {
                    (value.expectedStore, value.storeIndex)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePoolCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        expectedStore: tuple.0,
                        storeIndex: tuple.1,
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
            impl ::core::convert::From<removePoolReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removePoolReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePoolReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePoolCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePoolReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removePool(address,uint256)";
            const SELECTOR: [u8; 4] = [125u8, 31u8, 50u8, 38u8];
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
                        &self.expectedStore,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
    /**Function with signature `withdraw(address,address,uint256)` and selector `0xd9caed12`.
```solidity
function withdraw(address asset, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct withdraw_0Call {
        pub asset: alloy::sol_types::private::Address,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdraw(address,address,uint256)`](withdraw_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct withdraw_0Return {}
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
            impl ::core::convert::From<withdraw_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_0Call) -> Self {
                    (value.asset, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        asset: tuple.0,
                        to: tuple.1,
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
            impl ::core::convert::From<withdraw_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdraw_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdraw_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(address,address,uint256)";
            const SELECTOR: [u8; 4] = [217u8, 202u8, 237u8, 18u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
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
    /**Function with signature `withdraw(address,uint256)` and selector `0xf3fef3a3`.
```solidity
function withdraw(address asset, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct withdraw_1Call {
        pub asset: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdraw(address,uint256)`](withdraw_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct withdraw_1Return {}
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
            impl ::core::convert::From<withdraw_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_1Call) -> Self {
                    (value.asset, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_1Call {
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
            impl ::core::convert::From<withdraw_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: withdraw_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdraw_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdraw_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdraw_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdraw(address,uint256)";
            const SELECTOR: [u8; 4] = [243u8, 254u8, 243u8, 163u8];
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
    ///Container for all the [`MockRewardsManager`](self) function calls.
    pub enum MockRewardsManagerCalls {
        beforeAddLiquidity(beforeAddLiquidityCall),
        beforeRemoveLiquidity(beforeRemoveLiquidityCall),
        configurePool(configurePoolCall),
        consts(constsCall),
        deposit_0(deposit_0Call),
        deposit_1(deposit_1Call),
        getGrowthInsideRange(getGrowthInsideRangeCall),
        getGrowthInsideTick(getGrowthInsideTickCall),
        initializePool(initializePoolCall),
        pullFee(pullFeeCall),
        removePool(removePoolCall),
        toggleNodes(toggleNodesCall),
        update(updateCall),
        updateAfterTickMove(updateAfterTickMoveCall),
        withdraw_0(withdraw_0Call),
        withdraw_1(withdraw_1Call),
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
            [71u8, 231u8, 239u8, 36u8],
            [98u8, 136u8, 157u8, 214u8],
            [113u8, 204u8, 168u8, 27u8],
            [125u8, 31u8, 50u8, 38u8],
            [131u8, 64u8, 245u8, 73u8],
            [133u8, 135u8, 244u8, 80u8],
            [196u8, 62u8, 210u8, 200u8],
            [214u8, 207u8, 253u8, 30u8],
            [216u8, 109u8, 116u8, 78u8],
            [217u8, 202u8, 237u8, 18u8],
            [217u8, 225u8, 127u8, 152u8],
            [243u8, 254u8, 243u8, 163u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockRewardsManagerCalls {
        const NAME: &'static str = "MockRewardsManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 16usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::beforeAddLiquidity(_) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeRemoveLiquidity(_) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::configurePool(_) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::consts(_) => <constsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deposit_0(_) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_1(_) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGrowthInsideRange(_) => {
                    <getGrowthInsideRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGrowthInsideTick(_) => {
                    <getGrowthInsideTickCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializePool(_) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pullFee(_) => <pullFeeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::removePool(_) => {
                    <removePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::toggleNodes(_) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::update(_) => <updateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateAfterTickMove(_) => {
                    <updateAfterTickMoveCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw_0(_) => {
                    <withdraw_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdraw_1(_) => {
                    <withdraw_1Call as alloy_sol_types::SolCall>::SELECTOR
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
                    fn deposit_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <deposit_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::deposit_0)
                    }
                    deposit_0
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
                    fn removePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <removePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::removePool)
                    }
                    removePool
                },
                {
                    fn deposit_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <deposit_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::deposit_1)
                    }
                    deposit_1
                },
                {
                    fn initializePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <initializePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::initializePool)
                    }
                    initializePool
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
                {
                    fn withdraw_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <withdraw_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::withdraw_0)
                    }
                    withdraw_0
                },
                {
                    fn pullFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <pullFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::pullFee)
                    }
                    pullFee
                },
                {
                    fn withdraw_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <withdraw_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::withdraw_1)
                    }
                    withdraw_1
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
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pullFee(inner) => {
                    <pullFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::removePool(inner) => {
                    <removePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::withdraw_0(inner) => {
                    <withdraw_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::withdraw_1(inner) => {
                    <withdraw_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pullFee(inner) => {
                    <pullFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::removePool(inner) => {
                    <removePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::withdraw_0(inner) => {
                    <withdraw_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdraw_1(inner) => {
                    <withdraw_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
        BundleChangeNetNegative(BundleChangeNetNegative),
        DuplicateAsset(DuplicateAsset),
        FailedToDeployNewStore(FailedToDeployNewStore),
        FeeAboveMax(FeeAboveMax),
        IndexMayHaveChanged(IndexMayHaveChanged),
        InvalidPoolKey(InvalidPoolKey),
        InvalidStoreIndex(InvalidStoreIndex),
        InvalidTickSpacing(InvalidTickSpacing),
        MissingHookPermissions(MissingHookPermissions),
        NoEntry(NoEntry),
        NotController(NotController),
        NotFeeMaster(NotFeeMaster),
        NotFromHook(NotFromHook),
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
            [40u8, 51u8, 101u8, 94u8],
            [47u8, 101u8, 158u8, 68u8],
            [53u8, 39u8, 141u8, 18u8],
            [86u8, 112u8, 37u8, 135u8],
            [88u8, 125u8, 170u8, 48u8],
            [92u8, 210u8, 107u8, 104u8],
            [100u8, 41u8, 207u8, 210u8],
            [118u8, 163u8, 249u8, 93u8],
            [122u8, 215u8, 28u8, 235u8],
            [128u8, 241u8, 26u8, 207u8],
            [194u8, 86u8, 98u8, 43u8],
            [202u8, 204u8, 182u8, 217u8],
            [210u8, 198u8, 170u8, 230u8],
            [212u8, 155u8, 112u8, 245u8],
            [215u8, 171u8, 80u8, 39u8],
            [216u8, 166u8, 184u8, 155u8],
            [242u8, 31u8, 217u8, 159u8],
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
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AssetAccessOutOfBounds(_) => {
                    <AssetAccessOutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AssetsOutOfOrderOrNotUnique(_) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BundleChangeNetNegative(_) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DuplicateAsset(_) => {
                    <DuplicateAsset as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FailedToDeployNewStore(_) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeAboveMax(_) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexMayHaveChanged(_) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPoolKey(_) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidStoreIndex(_) => {
                    <InvalidStoreIndex as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotFeeMaster(_) => {
                    <NotFeeMaster as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotFromHook(_) => {
                    <NotFromHook as alloy_sol_types::SolError>::SELECTOR
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
                    fn NotFeeMaster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <NotFeeMaster as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::NotFeeMaster)
                    }
                    NotFeeMaster
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
                    fn DuplicateAsset(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <DuplicateAsset as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::DuplicateAsset)
                    }
                    DuplicateAsset
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
                    fn NotFromHook(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <NotFromHook as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::NotFromHook)
                    }
                    NotFromHook
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
                    fn InvalidStoreIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <InvalidStoreIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::InvalidStoreIndex)
                    }
                    InvalidStoreIndex
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
                    fn IndexMayHaveChanged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::IndexMayHaveChanged)
                    }
                    IndexMayHaveChanged
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
                Self::BundleChangeNetNegative(inner) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DuplicateAsset(inner) => {
                    <DuplicateAsset as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::IndexMayHaveChanged(inner) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPoolKey(inner) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidStoreIndex(inner) => {
                    <InvalidStoreIndex as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotFeeMaster(inner) => {
                    <NotFeeMaster as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::NotFromHook(inner) => {
                    <NotFromHook as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::BundleChangeNetNegative(inner) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DuplicateAsset(inner) => {
                    <DuplicateAsset as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::IndexMayHaveChanged(inner) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidStoreIndex(inner) => {
                    <InvalidStoreIndex as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotFeeMaster(inner) => {
                    <NotFeeMaster as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotFromHook(inner) => {
                    <NotFromHook as alloy_sol_types::SolError>::abi_encode_raw(
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
        uniV4: alloy::sol_types::private::Address,
        controller: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockRewardsManagerInstance<T, P, N>>,
    > {
        MockRewardsManagerInstance::<T, P, N>::deploy(provider, uniV4, controller)
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
        controller: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockRewardsManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, uniV4, controller)
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
            uniV4: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<MockRewardsManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, uniV4, controller);
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
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            uniV4,
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
            assetA: alloy::sol_types::private::Address,
            assetB: alloy::sol_types::private::Address,
            tickSpacing: u16,
            feeInE6: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, configurePoolCall, N> {
            self.call_builder(
                &configurePoolCall {
                    assetA,
                    assetB,
                    tickSpacing,
                    feeInE6,
                },
            )
        }
        ///Creates a new call builder for the [`consts`] function.
        pub fn consts(&self) -> alloy_contract::SolCallBuilder<T, &P, constsCall, N> {
            self.call_builder(&constsCall {})
        }
        ///Creates a new call builder for the [`deposit_0`] function.
        pub fn deposit_0(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_0Call, N> {
            self.call_builder(&deposit_0Call { asset, amount })
        }
        ///Creates a new call builder for the [`deposit_1`] function.
        pub fn deposit_1(
            &self,
            asset: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, deposit_1Call, N> {
            self.call_builder(&deposit_1Call { asset, to, amount })
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
        ///Creates a new call builder for the [`initializePool`] function.
        pub fn initializePool(
            &self,
            assetA: alloy::sol_types::private::Address,
            assetB: alloy::sol_types::private::Address,
            storeIndex: alloy::sol_types::private::primitives::aliases::U256,
            sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializePoolCall, N> {
            self.call_builder(
                &initializePoolCall {
                    assetA,
                    assetB,
                    storeIndex,
                    sqrtPriceX96,
                },
            )
        }
        ///Creates a new call builder for the [`pullFee`] function.
        pub fn pullFee(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, pullFeeCall, N> {
            self.call_builder(&pullFeeCall { asset, amount })
        }
        ///Creates a new call builder for the [`removePool`] function.
        pub fn removePool(
            &self,
            expectedStore: alloy::sol_types::private::Address,
            storeIndex: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePoolCall, N> {
            self.call_builder(
                &removePoolCall {
                    expectedStore,
                    storeIndex,
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
        ///Creates a new call builder for the [`withdraw_0`] function.
        pub fn withdraw_0(
            &self,
            asset: alloy::sol_types::private::Address,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdraw_0Call, N> {
            self.call_builder(
                &withdraw_0Call {
                    asset,
                    to,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`withdraw_1`] function.
        pub fn withdraw_1(
            &self,
            asset: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdraw_1Call, N> {
            self.call_builder(&withdraw_1Call { asset, amount })
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
