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
    type BalanceDelta is int256;
    type Currency is address;
    type PoolId is bytes32;
    struct PoolKey {
        Currency currency0;
        Currency currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    error AssetsOutOfOrderOrNotUnique();
    error BundleChangeNetNegative(address asset);
    error MissingHookPermissions(uint160);
    error NotUniswap();
    error OutOfBoundRead(uint256 arrayIndex, uint256 arrayLength);
    error Overflow();
    error ReaderNotAtEnd();
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4PoolManager);

    function afterRemoveLiquidity(address, PoolKey memory, IPoolManager.ModifyLiquidityParams memory, BalanceDelta, bytes memory) external pure returns (bytes4, BalanceDelta);
    function beforeAddLiquidity(address, PoolKey memory, IPoolManager.ModifyLiquidityParams memory, bytes memory) external view returns (bytes4);
    function consts() external pure returns (int24 tickSpacing, uint24 poolFee);
    function getGrowthInsideRange(PoolId id, int24 lowerTick, int24 upperTick) external view returns (uint256);
    function getGrowthInsideTick(PoolId id, int24 tick) external view returns (uint256);
    function update(bytes memory encoded) external;
    function updateAfterTickMove(PoolId id, int24 lastTick, int24 newTick) external;
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
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "afterRemoveLiquidity",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
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
        "type": "int256",
        "internalType": "BalanceDelta"
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
      },
      {
        "name": "",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "beforeAddLiquidity",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
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
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "consts",
    "inputs": [],
    "outputs": [
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      },
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
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "NotUniswap",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfBoundRead",
    "inputs": [
      {
        "name": "arrayIndex",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "arrayLength",
        "type": "uint256",
        "internalType": "uint256"
      }
    ]
  },
  {
    "type": "error",
    "name": "Overflow",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ReaderNotAtEnd",
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
    ///0x60a060405234801561000f575f80fd5b5060405161237638038061237683398101604081905261002e91610149565b6001600160a01b038116608052610046610900610089565b60408051808201909152601881527f72657761726473206d616e61676572206465706c6f79656400000000000000006020820152610083906100cf565b506101ab565b806001600160a01b03168130166001600160a01b0316146100cc57604051630ea7064560e31b81526001600160a01b038216600482015260240160405180910390fd5b50565b6100cc816040516024016100e39190610176565b60408051601f198184030181529190526020810180516001600160e01b0390811663104c13eb60e21b1790915261011616565b6100cc8161012960201b6104471760201c565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b5f60208284031215610159575f80fd5b81516001600160a01b038116811461016f575f80fd5b9392505050565b602081525f82518060208401528060208501604085015e5f604082850101526040601f19601f83011684010191505092915050565b60805161216d6102095f395f81816101a3015281816102fe015281816104f40152818161060b015281816108ef0152818161091901528181610a3001528181610a6d01528181610bd7015281816115b4015261176e015261216d5ff3fe608060405234801561000f575f80fd5b506004361061007a575f3560e01c806362889dd61161005857806362889dd6146100fd5780638db2b65214610110578063c43ed2c814610163578063d86d744e14610176575f80fd5b8063259982e51461007e57806335e81c70146100c75780635cd03714146100e8575b5f80fd5b61009161008c366004611adb565b61018a565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6100da6100d5366004611b61565b610225565b6040519081526020016100be565b6100fb6100f6366004611b8b565b6102ea565b005b6100da61010b366004611b8b565b610329565b61012b61011e366004611bc4565b5f80965096945050505050565b604080517fffffffff0000000000000000000000000000000000000000000000000000000090931683526020830191909152016100be565b6100fb610171366004611c43565b6103d3565b60408051603c81525f6020820152016100be565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146101fa576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f259982e50000000000000000000000000000000000000000000000000000000095945050505050565b5f61026683836040518060400160405280600481526020017f7469636b00000000000000000000000000000000000000000000000000000000815250610467565b5f805b6102738585610592565b90925090508115610269576102be85826040518060400160405280600a81526020017f6e6578745469636b557000000000000000000000000000000000000000000000815250610467565b6102df6102ca866105ed565b5f87815260046020526040902090868461063a565b925050505b92915050565b5f83815260046020526040902061032490847f000000000000000000000000000000000000000000000000000000000000000085856106cf565b505050565b5f61036a84846040518060400160405280600981526020017f6c6f7765725469636b0000000000000000000000000000000000000000000000815250610467565b6103aa84836040518060400160405280600981526020017f75707065725469636b0000000000000000000000000000000000000000000000815250610467565b6103cb6103b6856105ed565b5f86815260046020526040902090858561063a565b949350505050565b5f8290505f6103f960405180606001604052806023815260200161213e6023913961070a565b6104028261079c565b909250905061042860405180606001604052806023815260200161211b6023913961070a565b610434826001836107c3565b9150610441828585610acf565b50505050565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b610472603c83611caf565b60020b5f14816040516020016104889190611ce7565b604051602081830303815290604052906104d8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104cf9190611d6b565b60405180910390fd5b505f61051b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168585610ae6565b506fffffffffffffffffffffffffffffffff1690505f8111826040516020016105449190611d7d565b6040516020818303038152906040529061058b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104cf9190611d6b565b5050505050565b5f8080806105bb603c8087078313908705035b6105b0906001611de2565b600281900b60081d91565b915091506105d3816105cd8885610bbc565b90610c05565b90945090506105e28282610cc7565b925050509250929050565b5f6102e461063173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684610cf2565b60a01c60020b90565b5f808562ffffff85166301000000811061065657610656611e23565b015490505f8662ffffff85166301000000811061067557610675611e23565b015490508460020b8660020b121561069a576106918183611e50565b925050506103cb565b8360020b8660020b126106b1576106918282611e50565b808288630100000001546106c59190611e50565b6106919190611e50565b8160020b8160020b13156106ef576106ea8584868585610d91565b61058b565b8160020b8160020b121561058b5761058b8584868585610e25565b6107998160405160240161071e9190611d6b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052610eb1565b50565b5f805f6107aa846044610eba565b9094509050836107b982610ef4565b9250925050915091565b60028301925f908190819081903560f01c816107e76107e28884610f98565b610fbe565b60028a01993560f01c925090505f6108026107e28985610f98565b90508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610935083610841578082610844565b81815b909650945061085292505050565b6040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a8116825289169281019290925294810191909152603c938101939093523083830152822060108a019990919035901c8015610a96575f61091561063173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685610cf2565b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c856040518060600160405280891515815260200161097087610fc9565b8152602001896109945773fffd8963efd1fc6a506488495d951d5263988d2561099b565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b81526004016109d1929190611e63565b6020604051808303815f875af11580156109ed573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a119190611f2d565b505f610a5661063173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686610cf2565b5f858152600460205260409020909150610a9390857f000000000000000000000000000000000000000000000000000000000000000085856106cf565b50505b5f828152600460205260408120610aaf908c908561102a565b909b509050610abf8a8883611156565b50999a9950505050505050505050565b808201808414610441576301842f8c5f526004601cfd5b5f8281526006602090815260408083208484526004019091528120819081906040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290529091505f9073ffffffffffffffffffffffffffffffffffffffff881690631e2eaeaf90602401602060405180830381865afa158015610b73573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b979190611f2d565b6fffffffffffffffffffffffffffffffff81169860809190911d975095505050505050565b5f610bfe73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484611199565b9392505050565b5f805f610ca08460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082610cb65760ff610cbd565b8360ff1681015b9150509250929050565b5f603c60ff8316610cde600186900b610100611f44565b610ce89190611de2565b610bfe9190611f44565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa158015610d6d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103cb9190611f2d565b63010000008501545b8160020b8360020b1215610e1d575f610dca73ffffffffffffffffffffffffffffffffffffffff8716868661124f565b945090508015610e17578662ffffff851663010000008110610dee57610dee611e23565b0154610dfa9083611e50565b8762ffffff861663010000008110610e1457610e14611e23565b01555b50610d9a565b505050505050565b63010000008501545b8160020b8360020b1315610e1d575f610e5e73ffffffffffffffffffffffffffffffffffffffff871686866112ab565b945090508015610eab578662ffffff851663010000008110610e8257610e82611e23565b0154610e8e9083611e50565b8762ffffff861663010000008110610ea857610ea8611e23565b01555b50610e2e565b61079981610447565b6003828101925f918291803560e81c01018184610ed78784611e50565b610ee19190611f6a565b9193505060201b841790505b9250929050565b5f80610eff836112f7565b90505f805b82811015610f8b575f610f1a6107e28784610f98565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611610f81576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101610f04565b50929392505050565b5050565b5f610fa38383611302565b610fae604483611f7d565b610bfe9063ffffffff8516611f94565b5f813560601c6102e4565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611024576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6001838101935f9182918291829135821a16151560038801973560e81d5f611051886105ed565b6013808c019b919250813560801c91601081013560e81c0101846110835761107e8b848e878e87876113be565b611092565b6110928b848e878e87876114b3565b929e50909950975095506110a7945050505050565b5f6110b186611599565b9050806fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff161461112c576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff8084166004830152821660248201526044016104cf565b82876301000000015f8282546111429190611f94565b90915550889550505050505b935093915050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120611191611188825c856115da565b925081836115f2565b509392505050565b5f828152600660209081526040808320848452600501909152812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa158015611222573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112469190611f2d565b95945050505050565b5f808080611266603c8087078313908705036105a5565b9092509050611290816105cd73ffffffffffffffffffffffffffffffffffffffff8a168986611199565b909450905061129f8282610cc7565b92505050935093915050565b5f8080806112c76105b06001603c808907851390890503611fa7565b9092509050611290816112f173ffffffffffffffffffffffffffffffffffffffff8a168986611199565b906115f9565b5f6102e48260201c90565b5f61130d8360201c90565b9050808210610324576040517fbc5f997c00000000000000000000000000000000000000000000000000000000815260048101839052602481018290526044016104cf565b8860ff1681511015611385578060405160200161136f919061201f565b6040516020818303038152906040529050611352565b61138e836116c0565b816040516020016113a0929190612050565b60405160208183030381529060405296505050505050509392505050565b5f80808060015f805b8215611472575f888d146113e0575060108c019b3560801c5b6113ea8184611f94565b9250611408818b6fffffffffffffffffffffffffffffffff16611738565b6114129083611f94565b91508d60020b8c60020b136114275750611494565b818f8d62ffffff166301000000811061144257611442611e23565b015f8282546114519190611f94565b9091555061146a90508a6114658d8f611753565b611795565b995050611481565b8c60020b8b60020b1315611494575b61148b8a8c6117af565b9b5092506113c7565b61149e8c896117e6565b9a9d909c50999a509598975050505050505050565b5f80808060015f805b82156115695760108c019b3560801c6114d58184611f94565b92506114f3818b6fffffffffffffffffffffffffffffffff16611738565b6114fd9083611f94565b91508d60020b8c60020b1315611513575061158a565b818f8d62ffffff166301000000811061152e5761152e611e23565b015f82825461153d9190611f94565b909155505f905061154e8c8e611753565b905061155a8b8261181f565b9a506115639050565b50611577565b8c60020b8b60020b1361158a575b6115818a8c610592565b9b5092506114bc565b6115948c896117e6565b61149e565b5f6102e473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001683611839565b808203828113156102e45763c9654ed45f526004601cfd5b80825d5050565b5f805f8360ff0390505f61169a8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b90508061010014159350836116af575f6105e2565b8160ff168103925050509250929050565b60606102e4826118dc565b806001166001036116fb5781876040516020016116e992919061208b565b60405160208183030381529060405296505b600181901c9050818260405160200161171592919061208b565b60405160208183030381529060405291505f81116116cb57505050929392505050565b5f610bfe8261174f670de0b6b3a764000086611f7d565b0490565b5f6103cb73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484610ae6565b808203608081901c156102e45763c9654ed45f526004601cfd5b5f8080806117d46105b06117c4600188611fa7565b5f603c8083079190911291050390565b915091506105d3816112f18885610bbc565b808214610f94576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b818101608081901c156102e45763c9654ed45f526004601cfd5b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526003820160048201529091505f9073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa1580156118b8573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102df9190611f2d565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a9004806118f55750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b600f811651948201946001860153600f8160041c1651855360081c84830361193c57801561197157632194895a5f526004601cfd5b5050508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe090910190815292915050565b8860ff16815110156119d757806040516020016119c1919061201f565b60405160208183030381529060405290506119a4565b5f8a126119f25760405180602001604052805f815250611a29565b6040518060400160405280600181526020017f2d000000000000000000000000000000000000000000000000000000000000008152505b611a32846116c0565b826040516020016113a0939291906120a9565b611a4d6120ed565b565b803573ffffffffffffffffffffffffffffffffffffffff81168114611a72575f80fd5b919050565b5f60a08284031215611a87575f80fd5b50919050565b5f60808284031215611a87575f80fd5b5f8083601f840112611aad575f80fd5b50813567ffffffffffffffff811115611ac4575f80fd5b602083019150836020828501011115610eed575f80fd5b5f805f805f6101608688031215611af0575f80fd5b611af986611a4f565b9450611b088760208801611a77565b9350611b178760c08801611a8d565b925061014086013567ffffffffffffffff811115611b33575f80fd5b611b3f88828901611a9d565b969995985093965092949392505050565b8035600281900b8114611a72575f80fd5b5f8060408385031215611b72575f80fd5b82359150611b8260208401611b50565b90509250929050565b5f805f60608486031215611b9d575f80fd5b83359250611bad60208501611b50565b9150611bbb60408501611b50565b90509250925092565b5f805f805f806101808789031215611bda575f80fd5b611be387611a4f565b9550611bf28860208901611a77565b9450611c018860c08901611a8d565b9350610140870135925061016087013567ffffffffffffffff811115611c25575f80fd5b611c3189828a01611a9d565b979a9699509497509295939492505050565b5f8060208385031215611c54575f80fd5b823567ffffffffffffffff811115611c6a575f80fd5b611c7685828601611a9d565b90969095509350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f8260020b80611cc157611cc1611c82565b808360020b0791505092915050565b5f81518060208401855e5f93019283525090919050565b5f611cf28284611cd0565b7f206e6f742066726f6d20726567756c61722073706163696e670000000000000081526019019392505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f610bfe6020830184611d1f565b5f611d888284611cd0565b7f206e6f7420696e697469616c697a65640000000000000000000000000000000081526010019392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156102e4576102e4611db5565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b818103818111156102e4576102e4611db5565b73ffffffffffffffffffffffffffffffffffffffff835116815273ffffffffffffffffffffffffffffffffffffffff602084015116602082015262ffffff6040840151166040820152606083015160020b606082015273ffffffffffffffffffffffffffffffffffffffff6080840151166080820152611f1260a08201838051151582526020808201519083015260409081015173ffffffffffffffffffffffffffffffffffffffff16910152565b6101206101008201525f6103cb61012083015f815260200190565b5f60208284031215611f3d575f80fd5b5051919050565b5f8260020b8260020b028060020b9150808214611f6357611f63611db5565b5092915050565b5f82611f7857611f78611c82565b500490565b80820281158282048414176102e4576102e4611db5565b808201808211156102e4576102e4611db5565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156102e4576102e4611db5565b600184111561114e5780850481111561200357612003611db5565b600184161561201157908102905b60019390931c928002611fe8565b7f300000000000000000000000000000000000000000000000000000000000000081525f610bfe6001830184611cd0565b5f61205b8285611cd0565b7f2e0000000000000000000000000000000000000000000000000000000000000081526112466001820185611cd0565b5f6103cb6120998386611cd0565b84611cd0565b9695505050505050565b5f6120bd6120b78387611cd0565b85611cd0565b7f2e00000000000000000000000000000000000000000000000000000000000000815261209f6001820185611cd0565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52605160045260245ffdfe5b4d6f636b526577617264734d616e616765725d20726577617264696e6720706f6f6c5b4d6f636b526577617264734d616e616765725d206c6f6164696e6720617373657473a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15a\0\x0FW_\x80\xFD[P`@Qa#v8\x03\x80a#v\x839\x81\x01`@\x81\x90Ra\0.\x91a\x01IV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Ra\0Fa\t\0a\0\x89V[`@\x80Q\x80\x82\x01\x90\x91R`\x18\x81R\x7Frewards manager deployed\0\0\0\0\0\0\0\0` \x82\x01Ra\0\x83\x90a\0\xCFV[Pa\x01\xABV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x810\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\0\xCCW`@Qc\x0E\xA7\x06E`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PV[a\0\xCC\x81`@Q`$\x01a\0\xE3\x91\x90a\x01vV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x90\x81\x16c\x10L\x13\xEB`\xE2\x1B\x17\x90\x91Ra\x01\x16\x16V[a\0\xCC\x81a\x01)` \x1Ba\x04G\x17` \x1CV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[_` \x82\x84\x03\x12\x15a\x01YW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01oW_\x80\xFD[\x93\x92PPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[`\x80Qa!ma\x02\t_9_\x81\x81a\x01\xA3\x01R\x81\x81a\x02\xFE\x01R\x81\x81a\x04\xF4\x01R\x81\x81a\x06\x0B\x01R\x81\x81a\x08\xEF\x01R\x81\x81a\t\x19\x01R\x81\x81a\n0\x01R\x81\x81a\nm\x01R\x81\x81a\x0B\xD7\x01R\x81\x81a\x15\xB4\x01Ra\x17n\x01Ra!m_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cb\x88\x9D\xD6\x11a\0XW\x80cb\x88\x9D\xD6\x14a\0\xFDW\x80c\x8D\xB2\xB6R\x14a\x01\x10W\x80c\xC4>\xD2\xC8\x14a\x01cW\x80c\xD8mtN\x14a\x01vW_\x80\xFD[\x80c%\x99\x82\xE5\x14a\0~W\x80c5\xE8\x1Cp\x14a\0\xC7W\x80c\\\xD07\x14\x14a\0\xE8W[_\x80\xFD[a\0\x91a\0\x8C6`\x04a\x1A\xDBV[a\x01\x8AV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDAa\0\xD56`\x04a\x1BaV[a\x02%V[`@Q\x90\x81R` \x01a\0\xBEV[a\0\xFBa\0\xF66`\x04a\x1B\x8BV[a\x02\xEAV[\0[a\0\xDAa\x01\x0B6`\x04a\x1B\x8BV[a\x03)V[a\x01+a\x01\x1E6`\x04a\x1B\xC4V[_\x80\x96P\x96\x94PPPPPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\0\xBEV[a\0\xFBa\x01q6`\x04a\x1CCV[a\x03\xD3V[`@\x80Q`<\x81R_` \x82\x01R\x01a\0\xBEV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01\xFAW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[_a\x02f\x83\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[_\x80[a\x02s\x85\x85a\x05\x92V[\x90\x92P\x90P\x81\x15a\x02iWa\x02\xBE\x85\x82`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FnextTickUp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[a\x02\xDFa\x02\xCA\x86a\x05\xEDV[_\x87\x81R`\x04` R`@\x90 \x90\x86\x84a\x06:V[\x92PPP[\x92\x91PPV[_\x83\x81R`\x04` R`@\x90 a\x03$\x90\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a\x06\xCFV[PPPV[_a\x03j\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FlowerTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[a\x03\xAA\x84\x83`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FupperTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[a\x03\xCBa\x03\xB6\x85a\x05\xEDV[_\x86\x81R`\x04` R`@\x90 \x90\x85\x85a\x06:V[\x94\x93PPPPV[_\x82\x90P_a\x03\xF9`@Q\x80``\x01`@R\x80`#\x81R` \x01a!>`#\x919a\x07\nV[a\x04\x02\x82a\x07\x9CV[\x90\x92P\x90Pa\x04(`@Q\x80``\x01`@R\x80`#\x81R` \x01a!\x1B`#\x919a\x07\nV[a\x044\x82`\x01\x83a\x07\xC3V[\x91Pa\x04A\x82\x85\x85a\n\xCFV[PPPPV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[a\x04r`<\x83a\x1C\xAFV[`\x02\x0B_\x14\x81`@Q` \x01a\x04\x88\x91\x90a\x1C\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x04\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xCF\x91\x90a\x1DkV[`@Q\x80\x91\x03\x90\xFD[P_a\x05\x1Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\n\xE6V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x11\x82`@Q` \x01a\x05D\x91\x90a\x1D}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x05\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xCF\x91\x90a\x1DkV[PPPPPV[_\x80\x80\x80a\x05\xBB`<\x80\x87\x07\x83\x13\x90\x87\x05\x03[a\x05\xB0\x90`\x01a\x1D\xE2V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x91P\x91Pa\x05\xD3\x81a\x05\xCD\x88\x85a\x0B\xBCV[\x90a\x0C\x05V[\x90\x94P\x90Pa\x05\xE2\x82\x82a\x0C\xC7V[\x92PPP\x92P\x92\x90PV[_a\x02\xE4a\x061s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\x0C\xF2V[`\xA0\x1C`\x02\x0B\x90V[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x06VWa\x06Va\x1E#V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x06uWa\x06ua\x1E#V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x06\x9AWa\x06\x91\x81\x83a\x1EPV[\x92PPPa\x03\xCBV[\x83`\x02\x0B\x86`\x02\x0B\x12a\x06\xB1Wa\x06\x91\x82\x82a\x1EPV[\x80\x82\x88c\x01\0\0\0\x01Ta\x06\xC5\x91\x90a\x1EPV[a\x06\x91\x91\x90a\x1EPV[\x81`\x02\x0B\x81`\x02\x0B\x13\x15a\x06\xEFWa\x06\xEA\x85\x84\x86\x85\x85a\r\x91V[a\x05\x8BV[\x81`\x02\x0B\x81`\x02\x0B\x12\x15a\x05\x8BWa\x05\x8B\x85\x84\x86\x85\x85a\x0E%V[a\x07\x99\x81`@Q`$\x01a\x07\x1E\x91\x90a\x1DkV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x0E\xB1V[PV[_\x80_a\x07\xAA\x84`Da\x0E\xBAV[\x90\x94P\x90P\x83a\x07\xB9\x82a\x0E\xF4V[\x92P\x92PP\x91P\x91V[`\x02\x83\x01\x92_\x90\x81\x90\x81\x90\x81\x905`\xF0\x1C\x81a\x07\xE7a\x07\xE2\x88\x84a\x0F\x98V[a\x0F\xBEV[`\x02\x8A\x01\x995`\xF0\x1C\x92P\x90P_a\x08\x02a\x07\xE2\x89\x85a\x0F\x98V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x93P\x83a\x08AW\x80\x82a\x08DV[\x81\x81[\x90\x96P\x94Pa\x08R\x92PPPV[`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x16\x92\x81\x01\x92\x90\x92R\x94\x81\x01\x91\x90\x91R`<\x93\x81\x01\x93\x90\x93R0\x83\x83\x01R\x82 `\x10\x8A\x01\x99\x90\x91\x905\x90\x1C\x80\x15a\n\x96W_a\t\x15a\x061s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x0C\xF2V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x85`@Q\x80``\x01`@R\x80\x89\x15\x15\x81R` \x01a\tp\x87a\x0F\xC9V[\x81R` \x01\x89a\t\x94Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\t\x9BV[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xD1\x92\x91\x90a\x1EcV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xEDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x11\x91\x90a\x1F-V[P_a\nVa\x061s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x0C\xF2V[_\x85\x81R`\x04` R`@\x90 \x90\x91Pa\n\x93\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a\x06\xCFV[PP[_\x82\x81R`\x04` R`@\x81 a\n\xAF\x90\x8C\x90\x85a\x10*V[\x90\x9BP\x90Pa\n\xBF\x8A\x88\x83a\x11VV[P\x99\x9A\x99PPPPPPPPPPV[\x80\x82\x01\x80\x84\x14a\x04AWc\x01\x84/\x8C_R`\x04`\x1C\xFD[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x04\x01\x90\x91R\x81 \x81\x90\x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BsW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x97\x91\x90a\x1F-V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\x80\x91\x90\x91\x1D\x97P\x95PPPPPPV[_a\x0B\xFEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\x11\x99V[\x93\x92PPPV[_\x80_a\x0C\xA0\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a\x0C\xB6W`\xFFa\x0C\xBDV[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_`<`\xFF\x83\x16a\x0C\xDE`\x01\x86\x90\x0Ba\x01\0a\x1FDV[a\x0C\xE8\x91\x90a\x1D\xE2V[a\x0B\xFE\x91\x90a\x1FDV[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rmW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCB\x91\x90a\x1F-V[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x12\x15a\x0E\x1DW_a\r\xCAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a\x12OV[\x94P\x90P\x80\x15a\x0E\x17W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\r\xEEWa\r\xEEa\x1E#V[\x01Ta\r\xFA\x90\x83a\x1EPV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\x0E\x14Wa\x0E\x14a\x1E#V[\x01U[Pa\r\x9AV[PPPPPPV[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\x0E\x1DW_a\x0E^s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a\x12\xABV[\x94P\x90P\x80\x15a\x0E\xABW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x0E\x82Wa\x0E\x82a\x1E#V[\x01Ta\x0E\x8E\x90\x83a\x1EPV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\x0E\xA8Wa\x0E\xA8a\x1E#V[\x01U[Pa\x0E.V[a\x07\x99\x81a\x04GV[`\x03\x82\x81\x01\x92_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81\x84a\x0E\xD7\x87\x84a\x1EPV[a\x0E\xE1\x91\x90a\x1FjV[\x91\x93PP` \x1B\x84\x17\x90P[\x92P\x92\x90PV[_\x80a\x0E\xFF\x83a\x12\xF7V[\x90P_\x80[\x82\x81\x10\x15a\x0F\x8BW_a\x0F\x1Aa\x07\xE2\x87\x84a\x0F\x98V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0F\x81W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x0F\x04V[P\x92\x93\x92PPPV[PPV[_a\x0F\xA3\x83\x83a\x13\x02V[a\x0F\xAE`D\x83a\x1F}V[a\x0B\xFE\x90c\xFF\xFF\xFF\xFF\x85\x16a\x1F\x94V[_\x815``\x1Ca\x02\xE4V[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x10$W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`\x01\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x915\x82\x1A\x16\x15\x15`\x03\x88\x01\x975`\xE8\x1D_a\x10Q\x88a\x05\xEDV[`\x13\x80\x8C\x01\x9B\x91\x92P\x815`\x80\x1C\x91`\x10\x81\x015`\xE8\x1C\x01\x01\x84a\x10\x83Wa\x10~\x8B\x84\x8E\x87\x8E\x87\x87a\x13\xBEV[a\x10\x92V[a\x10\x92\x8B\x84\x8E\x87\x8E\x87\x87a\x14\xB3V[\x92\x9EP\x90\x99P\x97P\x95Pa\x10\xA7\x94PPPPPV[_a\x10\xB1\x86a\x15\x99V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11,W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x04\xCFV[\x82\x87c\x01\0\0\0\x01_\x82\x82Ta\x11B\x91\x90a\x1F\x94V[\x90\x91UP\x88\x95PPPPP[\x93P\x93\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a\x11\x91a\x11\x88\x82\\\x85a\x15\xDAV[\x92P\x81\x83a\x15\xF2V[P\x93\x92PPPV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12F\x91\x90a\x1F-V[\x95\x94PPPPPV[_\x80\x80\x80a\x12f`<\x80\x87\x07\x83\x13\x90\x87\x05\x03a\x05\xA5V[\x90\x92P\x90Pa\x12\x90\x81a\x05\xCDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a\x11\x99V[\x90\x94P\x90Pa\x12\x9F\x82\x82a\x0C\xC7V[\x92PPP\x93P\x93\x91PPV[_\x80\x80\x80a\x12\xC7a\x05\xB0`\x01`<\x80\x89\x07\x85\x13\x90\x89\x05\x03a\x1F\xA7V[\x90\x92P\x90Pa\x12\x90\x81a\x12\xF1s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a\x11\x99V[\x90a\x15\xF9V[_a\x02\xE4\x82` \x1C\x90V[_a\x13\r\x83` \x1C\x90V[\x90P\x80\x82\x10a\x03$W`@Q\x7F\xBC_\x99|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x04\xCFV[\x88`\xFF\x16\x81Q\x10\x15a\x13\x85W\x80`@Q` \x01a\x13o\x91\x90a \x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x13RV[a\x13\x8E\x83a\x16\xC0V[\x81`@Q` \x01a\x13\xA0\x92\x91\x90a PV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96PPPPPPP\x93\x92PPPV[_\x80\x80\x80`\x01_\x80[\x82\x15a\x14rW_\x88\x8D\x14a\x13\xE0WP`\x10\x8C\x01\x9B5`\x80\x1C[a\x13\xEA\x81\x84a\x1F\x94V[\x92Pa\x14\x08\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x178V[a\x14\x12\x90\x83a\x1F\x94V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13a\x14'WPa\x14\x94V[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x14BWa\x14Ba\x1E#V[\x01_\x82\x82Ta\x14Q\x91\x90a\x1F\x94V[\x90\x91UPa\x14j\x90P\x8Aa\x14e\x8D\x8Fa\x17SV[a\x17\x95V[\x99PPa\x14\x81V[\x8C`\x02\x0B\x8B`\x02\x0B\x13\x15a\x14\x94W[a\x14\x8B\x8A\x8Ca\x17\xAFV[\x9BP\x92Pa\x13\xC7V[a\x14\x9E\x8C\x89a\x17\xE6V[\x9A\x9D\x90\x9CP\x99\x9AP\x95\x98\x97PPPPPPPPV[_\x80\x80\x80`\x01_\x80[\x82\x15a\x15iW`\x10\x8C\x01\x9B5`\x80\x1Ca\x14\xD5\x81\x84a\x1F\x94V[\x92Pa\x14\xF3\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x178V[a\x14\xFD\x90\x83a\x1F\x94V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13\x15a\x15\x13WPa\x15\x8AV[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x15.Wa\x15.a\x1E#V[\x01_\x82\x82Ta\x15=\x91\x90a\x1F\x94V[\x90\x91UP_\x90Pa\x15N\x8C\x8Ea\x17SV[\x90Pa\x15Z\x8B\x82a\x18\x1FV[\x9APa\x15c\x90PV[Pa\x15wV[\x8C`\x02\x0B\x8B`\x02\x0B\x13a\x15\x8AW[a\x15\x81\x8A\x8Ca\x05\x92V[\x9BP\x92Pa\x14\xBCV[a\x15\x94\x8C\x89a\x17\xE6V[a\x14\x9EV[_a\x02\xE4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x83a\x189V[\x80\x82\x03\x82\x81\x13\x15a\x02\xE4Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[_\x80_\x83`\xFF\x03\x90P_a\x16\x9A\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a\x16\xAFW_a\x05\xE2V[\x81`\xFF\x16\x81\x03\x92PPP\x92P\x92\x90PV[``a\x02\xE4\x82a\x18\xDCV[\x80`\x01\x16`\x01\x03a\x16\xFBW\x81\x87`@Q` \x01a\x16\xE9\x92\x91\x90a \x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P[`\x01\x81\x90\x1C\x90P\x81\x82`@Q` \x01a\x17\x15\x92\x91\x90a \x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P_\x81\x11a\x16\xCBWPPP\x92\x93\x92PPPV[_a\x0B\xFE\x82a\x17Og\r\xE0\xB6\xB3\xA7d\0\0\x86a\x1F}V[\x04\x90V[_a\x03\xCBs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\n\xE6V[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x02\xE4Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a\x17\xD4a\x05\xB0a\x17\xC4`\x01\x88a\x1F\xA7V[_`<\x80\x83\x07\x91\x90\x91\x12\x91\x05\x03\x90V[\x91P\x91Pa\x05\xD3\x81a\x12\xF1\x88\x85a\x0B\xBCV[\x80\x82\x14a\x0F\x94W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x02\xE4Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x03\x82\x01`\x04\x82\x01R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xB8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90a\x1F-V[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a\x18\xF5WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[`\x0F\x81\x16Q\x94\x82\x01\x94`\x01\x86\x01S`\x0F\x81`\x04\x1C\x16Q\x85S`\x08\x1C\x84\x83\x03a\x19<W\x80\x15a\x19qWc!\x94\x89Z_R`\x04`\x1C\xFD[PPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x92\x91PPV[\x88`\xFF\x16\x81Q\x10\x15a\x19\xD7W\x80`@Q` \x01a\x19\xC1\x91\x90a \x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x19\xA4V[_\x8A\x12a\x19\xF2W`@Q\x80` \x01`@R\x80_\x81RPa\x1A)V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP[a\x1A2\x84a\x16\xC0V[\x82`@Q` \x01a\x13\xA0\x93\x92\x91\x90a \xA9V[a\x1AMa \xEDV[V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1ArW_\x80\xFD[\x91\x90PV[_`\xA0\x82\x84\x03\x12\x15a\x1A\x87W_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x1A\x87W_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a\x1A\xADW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xC4W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0E\xEDW_\x80\xFD[_\x80_\x80_a\x01`\x86\x88\x03\x12\x15a\x1A\xF0W_\x80\xFD[a\x1A\xF9\x86a\x1AOV[\x94Pa\x1B\x08\x87` \x88\x01a\x1AwV[\x93Pa\x1B\x17\x87`\xC0\x88\x01a\x1A\x8DV[\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B3W_\x80\xFD[a\x1B?\x88\x82\x89\x01a\x1A\x9DV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x805`\x02\x81\x90\x0B\x81\x14a\x1ArW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\x1BrW_\x80\xFD[\x825\x91Pa\x1B\x82` \x84\x01a\x1BPV[\x90P\x92P\x92\x90PV[_\x80_``\x84\x86\x03\x12\x15a\x1B\x9DW_\x80\xFD[\x835\x92Pa\x1B\xAD` \x85\x01a\x1BPV[\x91Pa\x1B\xBB`@\x85\x01a\x1BPV[\x90P\x92P\x92P\x92V[_\x80_\x80_\x80a\x01\x80\x87\x89\x03\x12\x15a\x1B\xDAW_\x80\xFD[a\x1B\xE3\x87a\x1AOV[\x95Pa\x1B\xF2\x88` \x89\x01a\x1AwV[\x94Pa\x1C\x01\x88`\xC0\x89\x01a\x1A\x8DV[\x93Pa\x01@\x87\x015\x92Pa\x01`\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C%W_\x80\xFD[a\x1C1\x89\x82\x8A\x01a\x1A\x9DV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80` \x83\x85\x03\x12\x15a\x1CTW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1CjW_\x80\xFD[a\x1Cv\x85\x82\x86\x01a\x1A\x9DV[\x90\x96\x90\x95P\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82`\x02\x0B\x80a\x1C\xC1Wa\x1C\xC1a\x1C\x82V[\x80\x83`\x02\x0B\x07\x91PP\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x1C\xF2\x82\x84a\x1C\xD0V[\x7F not from regular spacing\0\0\0\0\0\0\0\x81R`\x19\x01\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B\xFE` \x83\x01\x84a\x1D\x1FV[_a\x1D\x88\x82\x84a\x1C\xD0V[\x7F not initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x10\x01\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x82\x01Rb\xFF\xFF\xFF`@\x84\x01Q\x16`@\x82\x01R``\x83\x01Q`\x02\x0B``\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x82\x01Ra\x1F\x12`\xA0\x82\x01\x83\x80Q\x15\x15\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[a\x01 a\x01\0\x82\x01R_a\x03\xCBa\x01 \x83\x01_\x81R` \x01\x90V[_` \x82\x84\x03\x12\x15a\x1F=W_\x80\xFD[PQ\x91\x90PV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a\x1FcWa\x1Fca\x1D\xB5V[P\x92\x91PPV[_\x82a\x1FxWa\x1Fxa\x1C\x82V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xE4Wa\x02\xE4a\x1D\xB5V[\x80\x82\x01\x80\x82\x11\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[`\x01\x84\x11\x15a\x11NW\x80\x85\x04\x81\x11\x15a \x03Wa \x03a\x1D\xB5V[`\x01\x84\x16\x15a \x11W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x1F\xE8V[\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a\x0B\xFE`\x01\x83\x01\x84a\x1C\xD0V[_a [\x82\x85a\x1C\xD0V[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x12F`\x01\x82\x01\x85a\x1C\xD0V[_a\x03\xCBa \x99\x83\x86a\x1C\xD0V[\x84a\x1C\xD0V[\x96\x95PPPPPPV[_a \xBDa \xB7\x83\x87a\x1C\xD0V[\x85a\x1C\xD0V[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra \x9F`\x01\x82\x01\x85a\x1C\xD0V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`Q`\x04R`$_\xFD\xFE[MockRewardsManager] rewarding pool[MockRewardsManager] loading assets\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506004361061007a575f3560e01c806362889dd61161005857806362889dd6146100fd5780638db2b65214610110578063c43ed2c814610163578063d86d744e14610176575f80fd5b8063259982e51461007e57806335e81c70146100c75780635cd03714146100e8575b5f80fd5b61009161008c366004611adb565b61018a565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6100da6100d5366004611b61565b610225565b6040519081526020016100be565b6100fb6100f6366004611b8b565b6102ea565b005b6100da61010b366004611b8b565b610329565b61012b61011e366004611bc4565b5f80965096945050505050565b604080517fffffffff0000000000000000000000000000000000000000000000000000000090931683526020830191909152016100be565b6100fb610171366004611c43565b6103d3565b60408051603c81525f6020820152016100be565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146101fa576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f259982e50000000000000000000000000000000000000000000000000000000095945050505050565b5f61026683836040518060400160405280600481526020017f7469636b00000000000000000000000000000000000000000000000000000000815250610467565b5f805b6102738585610592565b90925090508115610269576102be85826040518060400160405280600a81526020017f6e6578745469636b557000000000000000000000000000000000000000000000815250610467565b6102df6102ca866105ed565b5f87815260046020526040902090868461063a565b925050505b92915050565b5f83815260046020526040902061032490847f000000000000000000000000000000000000000000000000000000000000000085856106cf565b505050565b5f61036a84846040518060400160405280600981526020017f6c6f7765725469636b0000000000000000000000000000000000000000000000815250610467565b6103aa84836040518060400160405280600981526020017f75707065725469636b0000000000000000000000000000000000000000000000815250610467565b6103cb6103b6856105ed565b5f86815260046020526040902090858561063a565b949350505050565b5f8290505f6103f960405180606001604052806023815260200161213e6023913961070a565b6104028261079c565b909250905061042860405180606001604052806023815260200161211b6023913961070a565b610434826001836107c3565b9150610441828585610acf565b50505050565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b610472603c83611caf565b60020b5f14816040516020016104889190611ce7565b604051602081830303815290604052906104d8576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104cf9190611d6b565b60405180910390fd5b505f61051b73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168585610ae6565b506fffffffffffffffffffffffffffffffff1690505f8111826040516020016105449190611d7d565b6040516020818303038152906040529061058b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016104cf9190611d6b565b5050505050565b5f8080806105bb603c8087078313908705035b6105b0906001611de2565b600281900b60081d91565b915091506105d3816105cd8885610bbc565b90610c05565b90945090506105e28282610cc7565b925050509250929050565b5f6102e461063173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684610cf2565b60a01c60020b90565b5f808562ffffff85166301000000811061065657610656611e23565b015490505f8662ffffff85166301000000811061067557610675611e23565b015490508460020b8660020b121561069a576106918183611e50565b925050506103cb565b8360020b8660020b126106b1576106918282611e50565b808288630100000001546106c59190611e50565b6106919190611e50565b8160020b8160020b13156106ef576106ea8584868585610d91565b61058b565b8160020b8160020b121561058b5761058b8584868585610e25565b6107998160405160240161071e9190611d6b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052610eb1565b50565b5f805f6107aa846044610eba565b9094509050836107b982610ef4565b9250925050915091565b60028301925f908190819081903560f01c816107e76107e28884610f98565b610fbe565b60028a01993560f01c925090505f6108026107e28985610f98565b90508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610935083610841578082610844565b81815b909650945061085292505050565b6040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a8116825289169281019290925294810191909152603c938101939093523083830152822060108a019990919035901c8015610a96575f61091561063173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685610cf2565b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c856040518060600160405280891515815260200161097087610fc9565b8152602001896109945773fffd8963efd1fc6a506488495d951d5263988d2561099b565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b81526004016109d1929190611e63565b6020604051808303815f875af11580156109ed573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610a119190611f2d565b505f610a5661063173ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686610cf2565b5f858152600460205260409020909150610a9390857f000000000000000000000000000000000000000000000000000000000000000085856106cf565b50505b5f828152600460205260408120610aaf908c908561102a565b909b509050610abf8a8883611156565b50999a9950505050505050505050565b808201808414610441576301842f8c5f526004601cfd5b5f8281526006602090815260408083208484526004019091528120819081906040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290529091505f9073ffffffffffffffffffffffffffffffffffffffff881690631e2eaeaf90602401602060405180830381865afa158015610b73573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610b979190611f2d565b6fffffffffffffffffffffffffffffffff81169860809190911d975095505050505050565b5f610bfe73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484611199565b9392505050565b5f805f610ca08460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082610cb65760ff610cbd565b8360ff1681015b9150509250929050565b5f603c60ff8316610cde600186900b610100611f44565b610ce89190611de2565b610bfe9190611f44565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa158015610d6d573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906103cb9190611f2d565b63010000008501545b8160020b8360020b1215610e1d575f610dca73ffffffffffffffffffffffffffffffffffffffff8716868661124f565b945090508015610e17578662ffffff851663010000008110610dee57610dee611e23565b0154610dfa9083611e50565b8762ffffff861663010000008110610e1457610e14611e23565b01555b50610d9a565b505050505050565b63010000008501545b8160020b8360020b1315610e1d575f610e5e73ffffffffffffffffffffffffffffffffffffffff871686866112ab565b945090508015610eab578662ffffff851663010000008110610e8257610e82611e23565b0154610e8e9083611e50565b8762ffffff861663010000008110610ea857610ea8611e23565b01555b50610e2e565b61079981610447565b6003828101925f918291803560e81c01018184610ed78784611e50565b610ee19190611f6a565b9193505060201b841790505b9250929050565b5f80610eff836112f7565b90505f805b82811015610f8b575f610f1a6107e28784610f98565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611610f81576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101610f04565b50929392505050565b5050565b5f610fa38383611302565b610fae604483611f7d565b610bfe9063ffffffff8516611f94565b5f813560601c6102e4565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611024576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6001838101935f9182918291829135821a16151560038801973560e81d5f611051886105ed565b6013808c019b919250813560801c91601081013560e81c0101846110835761107e8b848e878e87876113be565b611092565b6110928b848e878e87876114b3565b929e50909950975095506110a7945050505050565b5f6110b186611599565b9050806fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff161461112c576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff8084166004830152821660248201526044016104cf565b82876301000000015f8282546111429190611f94565b90915550889550505050505b935093915050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120611191611188825c856115da565b925081836115f2565b509392505050565b5f828152600660209081526040808320848452600501909152812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa158015611222573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112469190611f2d565b95945050505050565b5f808080611266603c8087078313908705036105a5565b9092509050611290816105cd73ffffffffffffffffffffffffffffffffffffffff8a168986611199565b909450905061129f8282610cc7565b92505050935093915050565b5f8080806112c76105b06001603c808907851390890503611fa7565b9092509050611290816112f173ffffffffffffffffffffffffffffffffffffffff8a168986611199565b906115f9565b5f6102e48260201c90565b5f61130d8360201c90565b9050808210610324576040517fbc5f997c00000000000000000000000000000000000000000000000000000000815260048101839052602481018290526044016104cf565b8860ff1681511015611385578060405160200161136f919061201f565b6040516020818303038152906040529050611352565b61138e836116c0565b816040516020016113a0929190612050565b60405160208183030381529060405296505050505050509392505050565b5f80808060015f805b8215611472575f888d146113e0575060108c019b3560801c5b6113ea8184611f94565b9250611408818b6fffffffffffffffffffffffffffffffff16611738565b6114129083611f94565b91508d60020b8c60020b136114275750611494565b818f8d62ffffff166301000000811061144257611442611e23565b015f8282546114519190611f94565b9091555061146a90508a6114658d8f611753565b611795565b995050611481565b8c60020b8b60020b1315611494575b61148b8a8c6117af565b9b5092506113c7565b61149e8c896117e6565b9a9d909c50999a509598975050505050505050565b5f80808060015f805b82156115695760108c019b3560801c6114d58184611f94565b92506114f3818b6fffffffffffffffffffffffffffffffff16611738565b6114fd9083611f94565b91508d60020b8c60020b1315611513575061158a565b818f8d62ffffff166301000000811061152e5761152e611e23565b015f82825461153d9190611f94565b909155505f905061154e8c8e611753565b905061155a8b8261181f565b9a506115639050565b50611577565b8c60020b8b60020b1361158a575b6115818a8c610592565b9b5092506114bc565b6115948c896117e6565b61149e565b5f6102e473ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001683611839565b808203828113156102e45763c9654ed45f526004601cfd5b80825d5050565b5f805f8360ff0390505f61169a8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b90508061010014159350836116af575f6105e2565b8160ff168103925050509250929050565b60606102e4826118dc565b806001166001036116fb5781876040516020016116e992919061208b565b60405160208183030381529060405296505b600181901c9050818260405160200161171592919061208b565b60405160208183030381529060405291505f81116116cb57505050929392505050565b5f610bfe8261174f670de0b6b3a764000086611f7d565b0490565b5f6103cb73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484610ae6565b808203608081901c156102e45763c9654ed45f526004601cfd5b5f8080806117d46105b06117c4600188611fa7565b5f603c8083079190911291050390565b915091506105d3816112f18885610bbc565b808214610f94576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b818101608081901c156102e45763c9654ed45f526004601cfd5b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526003820160048201529091505f9073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa1580156118b8573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906102df9190611f2d565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a9004806118f55750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b600f811651948201946001860153600f8160041c1651855360081c84830361193c57801561197157632194895a5f526004601cfd5b5050508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe090910190815292915050565b8860ff16815110156119d757806040516020016119c1919061201f565b60405160208183030381529060405290506119a4565b5f8a126119f25760405180602001604052805f815250611a29565b6040518060400160405280600181526020017f2d000000000000000000000000000000000000000000000000000000000000008152505b611a32846116c0565b826040516020016113a0939291906120a9565b611a4d6120ed565b565b803573ffffffffffffffffffffffffffffffffffffffff81168114611a72575f80fd5b919050565b5f60a08284031215611a87575f80fd5b50919050565b5f60808284031215611a87575f80fd5b5f8083601f840112611aad575f80fd5b50813567ffffffffffffffff811115611ac4575f80fd5b602083019150836020828501011115610eed575f80fd5b5f805f805f6101608688031215611af0575f80fd5b611af986611a4f565b9450611b088760208801611a77565b9350611b178760c08801611a8d565b925061014086013567ffffffffffffffff811115611b33575f80fd5b611b3f88828901611a9d565b969995985093965092949392505050565b8035600281900b8114611a72575f80fd5b5f8060408385031215611b72575f80fd5b82359150611b8260208401611b50565b90509250929050565b5f805f60608486031215611b9d575f80fd5b83359250611bad60208501611b50565b9150611bbb60408501611b50565b90509250925092565b5f805f805f806101808789031215611bda575f80fd5b611be387611a4f565b9550611bf28860208901611a77565b9450611c018860c08901611a8d565b9350610140870135925061016087013567ffffffffffffffff811115611c25575f80fd5b611c3189828a01611a9d565b979a9699509497509295939492505050565b5f8060208385031215611c54575f80fd5b823567ffffffffffffffff811115611c6a575f80fd5b611c7685828601611a9d565b90969095509350505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f8260020b80611cc157611cc1611c82565b808360020b0791505092915050565b5f81518060208401855e5f93019283525090919050565b5f611cf28284611cd0565b7f206e6f742066726f6d20726567756c61722073706163696e670000000000000081526019019392505050565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b602081525f610bfe6020830184611d1f565b5f611d888284611cd0565b7f206e6f7420696e697469616c697a65640000000000000000000000000000000081526010019392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156102e4576102e4611db5565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b818103818111156102e4576102e4611db5565b73ffffffffffffffffffffffffffffffffffffffff835116815273ffffffffffffffffffffffffffffffffffffffff602084015116602082015262ffffff6040840151166040820152606083015160020b606082015273ffffffffffffffffffffffffffffffffffffffff6080840151166080820152611f1260a08201838051151582526020808201519083015260409081015173ffffffffffffffffffffffffffffffffffffffff16910152565b6101206101008201525f6103cb61012083015f815260200190565b5f60208284031215611f3d575f80fd5b5051919050565b5f8260020b8260020b028060020b9150808214611f6357611f63611db5565b5092915050565b5f82611f7857611f78611c82565b500490565b80820281158282048414176102e4576102e4611db5565b808201808211156102e4576102e4611db5565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156102e4576102e4611db5565b600184111561114e5780850481111561200357612003611db5565b600184161561201157908102905b60019390931c928002611fe8565b7f300000000000000000000000000000000000000000000000000000000000000081525f610bfe6001830184611cd0565b5f61205b8285611cd0565b7f2e0000000000000000000000000000000000000000000000000000000000000081526112466001820185611cd0565b5f6103cb6120998386611cd0565b84611cd0565b9695505050505050565b5f6120bd6120b78387611cd0565b85611cd0565b7f2e00000000000000000000000000000000000000000000000000000000000000815261209f6001820185611cd0565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52605160045260245ffdfe5b4d6f636b526577617264734d616e616765725d20726577617264696e6720706f6f6c5b4d6f636b526577617264734d616e616765725d206c6f6164696e6720617373657473a164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0zW_5`\xE0\x1C\x80cb\x88\x9D\xD6\x11a\0XW\x80cb\x88\x9D\xD6\x14a\0\xFDW\x80c\x8D\xB2\xB6R\x14a\x01\x10W\x80c\xC4>\xD2\xC8\x14a\x01cW\x80c\xD8mtN\x14a\x01vW_\x80\xFD[\x80c%\x99\x82\xE5\x14a\0~W\x80c5\xE8\x1Cp\x14a\0\xC7W\x80c\\\xD07\x14\x14a\0\xE8W[_\x80\xFD[a\0\x91a\0\x8C6`\x04a\x1A\xDBV[a\x01\x8AV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDAa\0\xD56`\x04a\x1BaV[a\x02%V[`@Q\x90\x81R` \x01a\0\xBEV[a\0\xFBa\0\xF66`\x04a\x1B\x8BV[a\x02\xEAV[\0[a\0\xDAa\x01\x0B6`\x04a\x1B\x8BV[a\x03)V[a\x01+a\x01\x1E6`\x04a\x1B\xC4V[_\x80\x96P\x96\x94PPPPPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\0\xBEV[a\0\xFBa\x01q6`\x04a\x1CCV[a\x03\xD3V[`@\x80Q`<\x81R_` \x82\x01R\x01a\0\xBEV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x01\xFAW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[_a\x02f\x83\x83`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01\x7Ftick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[_\x80[a\x02s\x85\x85a\x05\x92V[\x90\x92P\x90P\x81\x15a\x02iWa\x02\xBE\x85\x82`@Q\x80`@\x01`@R\x80`\n\x81R` \x01\x7FnextTickUp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[a\x02\xDFa\x02\xCA\x86a\x05\xEDV[_\x87\x81R`\x04` R`@\x90 \x90\x86\x84a\x06:V[\x92PPP[\x92\x91PPV[_\x83\x81R`\x04` R`@\x90 a\x03$\x90\x84\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a\x06\xCFV[PPPV[_a\x03j\x84\x84`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FlowerTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[a\x03\xAA\x84\x83`@Q\x80`@\x01`@R\x80`\t\x81R` \x01\x7FupperTick\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x04gV[a\x03\xCBa\x03\xB6\x85a\x05\xEDV[_\x86\x81R`\x04` R`@\x90 \x90\x85\x85a\x06:V[\x94\x93PPPPV[_\x82\x90P_a\x03\xF9`@Q\x80``\x01`@R\x80`#\x81R` \x01a!>`#\x919a\x07\nV[a\x04\x02\x82a\x07\x9CV[\x90\x92P\x90Pa\x04(`@Q\x80``\x01`@R\x80`#\x81R` \x01a!\x1B`#\x919a\x07\nV[a\x044\x82`\x01\x83a\x07\xC3V[\x91Pa\x04A\x82\x85\x85a\n\xCFV[PPPPV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[a\x04r`<\x83a\x1C\xAFV[`\x02\x0B_\x14\x81`@Q` \x01a\x04\x88\x91\x90a\x1C\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x04\xD8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xCF\x91\x90a\x1DkV[`@Q\x80\x91\x03\x90\xFD[P_a\x05\x1Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85\x85a\n\xE6V[Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x11\x82`@Q` \x01a\x05D\x91\x90a\x1D}V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90a\x05\x8BW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x04\xCF\x91\x90a\x1DkV[PPPPPV[_\x80\x80\x80a\x05\xBB`<\x80\x87\x07\x83\x13\x90\x87\x05\x03[a\x05\xB0\x90`\x01a\x1D\xE2V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x91P\x91Pa\x05\xD3\x81a\x05\xCD\x88\x85a\x0B\xBCV[\x90a\x0C\x05V[\x90\x94P\x90Pa\x05\xE2\x82\x82a\x0C\xC7V[\x92PPP\x92P\x92\x90PV[_a\x02\xE4a\x061s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\x0C\xF2V[`\xA0\x1C`\x02\x0B\x90V[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x06VWa\x06Va\x1E#V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x06uWa\x06ua\x1E#V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x06\x9AWa\x06\x91\x81\x83a\x1EPV[\x92PPPa\x03\xCBV[\x83`\x02\x0B\x86`\x02\x0B\x12a\x06\xB1Wa\x06\x91\x82\x82a\x1EPV[\x80\x82\x88c\x01\0\0\0\x01Ta\x06\xC5\x91\x90a\x1EPV[a\x06\x91\x91\x90a\x1EPV[\x81`\x02\x0B\x81`\x02\x0B\x13\x15a\x06\xEFWa\x06\xEA\x85\x84\x86\x85\x85a\r\x91V[a\x05\x8BV[\x81`\x02\x0B\x81`\x02\x0B\x12\x15a\x05\x8BWa\x05\x8B\x85\x84\x86\x85\x85a\x0E%V[a\x07\x99\x81`@Q`$\x01a\x07\x1E\x91\x90a\x1DkV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x0E\xB1V[PV[_\x80_a\x07\xAA\x84`Da\x0E\xBAV[\x90\x94P\x90P\x83a\x07\xB9\x82a\x0E\xF4V[\x92P\x92PP\x91P\x91V[`\x02\x83\x01\x92_\x90\x81\x90\x81\x90\x81\x905`\xF0\x1C\x81a\x07\xE7a\x07\xE2\x88\x84a\x0F\x98V[a\x0F\xBEV[`\x02\x8A\x01\x995`\xF0\x1C\x92P\x90P_a\x08\x02a\x07\xE2\x89\x85a\x0F\x98V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x93P\x83a\x08AW\x80\x82a\x08DV[\x81\x81[\x90\x96P\x94Pa\x08R\x92PPPV[`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x16\x92\x81\x01\x92\x90\x92R\x94\x81\x01\x91\x90\x91R`<\x93\x81\x01\x93\x90\x93R0\x83\x83\x01R\x82 `\x10\x8A\x01\x99\x90\x91\x905\x90\x1C\x80\x15a\n\x96W_a\t\x15a\x061s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x0C\xF2V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x85`@Q\x80``\x01`@R\x80\x89\x15\x15\x81R` \x01a\tp\x87a\x0F\xC9V[\x81R` \x01\x89a\t\x94Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\t\x9BV[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xD1\x92\x91\x90a\x1EcV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\t\xEDW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\x11\x91\x90a\x1F-V[P_a\nVa\x061s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x0C\xF2V[_\x85\x81R`\x04` R`@\x90 \x90\x91Pa\n\x93\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a\x06\xCFV[PP[_\x82\x81R`\x04` R`@\x81 a\n\xAF\x90\x8C\x90\x85a\x10*V[\x90\x9BP\x90Pa\n\xBF\x8A\x88\x83a\x11VV[P\x99\x9A\x99PPPPPPPPPPV[\x80\x82\x01\x80\x84\x14a\x04AWc\x01\x84/\x8C_R`\x04`\x1C\xFD[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x04\x01\x90\x91R\x81 \x81\x90\x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BsW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\x97\x91\x90a\x1F-V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\x80\x91\x90\x91\x1D\x97P\x95PPPPPPV[_a\x0B\xFEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\x11\x99V[\x93\x92PPPV[_\x80_a\x0C\xA0\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a\x0C\xB6W`\xFFa\x0C\xBDV[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_`<`\xFF\x83\x16a\x0C\xDE`\x01\x86\x90\x0Ba\x01\0a\x1FDV[a\x0C\xE8\x91\x90a\x1D\xE2V[a\x0B\xFE\x91\x90a\x1FDV[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rmW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xCB\x91\x90a\x1F-V[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x12\x15a\x0E\x1DW_a\r\xCAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a\x12OV[\x94P\x90P\x80\x15a\x0E\x17W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\r\xEEWa\r\xEEa\x1E#V[\x01Ta\r\xFA\x90\x83a\x1EPV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\x0E\x14Wa\x0E\x14a\x1E#V[\x01U[Pa\r\x9AV[PPPPPPV[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x13\x15a\x0E\x1DW_a\x0E^s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a\x12\xABV[\x94P\x90P\x80\x15a\x0E\xABW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x0E\x82Wa\x0E\x82a\x1E#V[\x01Ta\x0E\x8E\x90\x83a\x1EPV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a\x0E\xA8Wa\x0E\xA8a\x1E#V[\x01U[Pa\x0E.V[a\x07\x99\x81a\x04GV[`\x03\x82\x81\x01\x92_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81\x84a\x0E\xD7\x87\x84a\x1EPV[a\x0E\xE1\x91\x90a\x1FjV[\x91\x93PP` \x1B\x84\x17\x90P[\x92P\x92\x90PV[_\x80a\x0E\xFF\x83a\x12\xF7V[\x90P_\x80[\x82\x81\x10\x15a\x0F\x8BW_a\x0F\x1Aa\x07\xE2\x87\x84a\x0F\x98V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0F\x81W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x0F\x04V[P\x92\x93\x92PPPV[PPV[_a\x0F\xA3\x83\x83a\x13\x02V[a\x0F\xAE`D\x83a\x1F}V[a\x0B\xFE\x90c\xFF\xFF\xFF\xFF\x85\x16a\x1F\x94V[_\x815``\x1Ca\x02\xE4V[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x10$W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`\x01\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x915\x82\x1A\x16\x15\x15`\x03\x88\x01\x975`\xE8\x1D_a\x10Q\x88a\x05\xEDV[`\x13\x80\x8C\x01\x9B\x91\x92P\x815`\x80\x1C\x91`\x10\x81\x015`\xE8\x1C\x01\x01\x84a\x10\x83Wa\x10~\x8B\x84\x8E\x87\x8E\x87\x87a\x13\xBEV[a\x10\x92V[a\x10\x92\x8B\x84\x8E\x87\x8E\x87\x87a\x14\xB3V[\x92\x9EP\x90\x99P\x97P\x95Pa\x10\xA7\x94PPPPPV[_a\x10\xB1\x86a\x15\x99V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x11,W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x04\xCFV[\x82\x87c\x01\0\0\0\x01_\x82\x82Ta\x11B\x91\x90a\x1F\x94V[\x90\x91UP\x88\x95PPPPP[\x93P\x93\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a\x11\x91a\x11\x88\x82\\\x85a\x15\xDAV[\x92P\x81\x83a\x15\xF2V[P\x93\x92PPPV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\"W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12F\x91\x90a\x1F-V[\x95\x94PPPPPV[_\x80\x80\x80a\x12f`<\x80\x87\x07\x83\x13\x90\x87\x05\x03a\x05\xA5V[\x90\x92P\x90Pa\x12\x90\x81a\x05\xCDs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a\x11\x99V[\x90\x94P\x90Pa\x12\x9F\x82\x82a\x0C\xC7V[\x92PPP\x93P\x93\x91PPV[_\x80\x80\x80a\x12\xC7a\x05\xB0`\x01`<\x80\x89\x07\x85\x13\x90\x89\x05\x03a\x1F\xA7V[\x90\x92P\x90Pa\x12\x90\x81a\x12\xF1s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a\x11\x99V[\x90a\x15\xF9V[_a\x02\xE4\x82` \x1C\x90V[_a\x13\r\x83` \x1C\x90V[\x90P\x80\x82\x10a\x03$W`@Q\x7F\xBC_\x99|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x04\xCFV[\x88`\xFF\x16\x81Q\x10\x15a\x13\x85W\x80`@Q` \x01a\x13o\x91\x90a \x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x13RV[a\x13\x8E\x83a\x16\xC0V[\x81`@Q` \x01a\x13\xA0\x92\x91\x90a PV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96PPPPPPP\x93\x92PPPV[_\x80\x80\x80`\x01_\x80[\x82\x15a\x14rW_\x88\x8D\x14a\x13\xE0WP`\x10\x8C\x01\x9B5`\x80\x1C[a\x13\xEA\x81\x84a\x1F\x94V[\x92Pa\x14\x08\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x178V[a\x14\x12\x90\x83a\x1F\x94V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13a\x14'WPa\x14\x94V[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x14BWa\x14Ba\x1E#V[\x01_\x82\x82Ta\x14Q\x91\x90a\x1F\x94V[\x90\x91UPa\x14j\x90P\x8Aa\x14e\x8D\x8Fa\x17SV[a\x17\x95V[\x99PPa\x14\x81V[\x8C`\x02\x0B\x8B`\x02\x0B\x13\x15a\x14\x94W[a\x14\x8B\x8A\x8Ca\x17\xAFV[\x9BP\x92Pa\x13\xC7V[a\x14\x9E\x8C\x89a\x17\xE6V[\x9A\x9D\x90\x9CP\x99\x9AP\x95\x98\x97PPPPPPPPV[_\x80\x80\x80`\x01_\x80[\x82\x15a\x15iW`\x10\x8C\x01\x9B5`\x80\x1Ca\x14\xD5\x81\x84a\x1F\x94V[\x92Pa\x14\xF3\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x178V[a\x14\xFD\x90\x83a\x1F\x94V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13\x15a\x15\x13WPa\x15\x8AV[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x15.Wa\x15.a\x1E#V[\x01_\x82\x82Ta\x15=\x91\x90a\x1F\x94V[\x90\x91UP_\x90Pa\x15N\x8C\x8Ea\x17SV[\x90Pa\x15Z\x8B\x82a\x18\x1FV[\x9APa\x15c\x90PV[Pa\x15wV[\x8C`\x02\x0B\x8B`\x02\x0B\x13a\x15\x8AW[a\x15\x81\x8A\x8Ca\x05\x92V[\x9BP\x92Pa\x14\xBCV[a\x15\x94\x8C\x89a\x17\xE6V[a\x14\x9EV[_a\x02\xE4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x83a\x189V[\x80\x82\x03\x82\x81\x13\x15a\x02\xE4Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[_\x80_\x83`\xFF\x03\x90P_a\x16\x9A\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a\x16\xAFW_a\x05\xE2V[\x81`\xFF\x16\x81\x03\x92PPP\x92P\x92\x90PV[``a\x02\xE4\x82a\x18\xDCV[\x80`\x01\x16`\x01\x03a\x16\xFBW\x81\x87`@Q` \x01a\x16\xE9\x92\x91\x90a \x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P[`\x01\x81\x90\x1C\x90P\x81\x82`@Q` \x01a\x17\x15\x92\x91\x90a \x8BV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P_\x81\x11a\x16\xCBWPPP\x92\x93\x92PPPV[_a\x0B\xFE\x82a\x17Og\r\xE0\xB6\xB3\xA7d\0\0\x86a\x1F}V[\x04\x90V[_a\x03\xCBs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a\n\xE6V[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x02\xE4Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a\x17\xD4a\x05\xB0a\x17\xC4`\x01\x88a\x1F\xA7V[_`<\x80\x83\x07\x91\x90\x91\x12\x91\x05\x03\x90V[\x91P\x91Pa\x05\xD3\x81a\x12\xF1\x88\x85a\x0B\xBCV[\x80\x82\x14a\x0F\x94W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x02\xE4Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x03\x82\x01`\x04\x82\x01R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xB8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xDF\x91\x90a\x1F-V[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a\x18\xF5WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[`\x0F\x81\x16Q\x94\x82\x01\x94`\x01\x86\x01S`\x0F\x81`\x04\x1C\x16Q\x85S`\x08\x1C\x84\x83\x03a\x19<W\x80\x15a\x19qWc!\x94\x89Z_R`\x04`\x1C\xFD[PPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x92\x91PPV[\x88`\xFF\x16\x81Q\x10\x15a\x19\xD7W\x80`@Q` \x01a\x19\xC1\x91\x90a \x1FV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x19\xA4V[_\x8A\x12a\x19\xF2W`@Q\x80` \x01`@R\x80_\x81RPa\x1A)V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP[a\x1A2\x84a\x16\xC0V[\x82`@Q` \x01a\x13\xA0\x93\x92\x91\x90a \xA9V[a\x1AMa \xEDV[V[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1ArW_\x80\xFD[\x91\x90PV[_`\xA0\x82\x84\x03\x12\x15a\x1A\x87W_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a\x1A\x87W_\x80\xFD[_\x80\x83`\x1F\x84\x01\x12a\x1A\xADW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1A\xC4W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0E\xEDW_\x80\xFD[_\x80_\x80_a\x01`\x86\x88\x03\x12\x15a\x1A\xF0W_\x80\xFD[a\x1A\xF9\x86a\x1AOV[\x94Pa\x1B\x08\x87` \x88\x01a\x1AwV[\x93Pa\x1B\x17\x87`\xC0\x88\x01a\x1A\x8DV[\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B3W_\x80\xFD[a\x1B?\x88\x82\x89\x01a\x1A\x9DV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[\x805`\x02\x81\x90\x0B\x81\x14a\x1ArW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\x1BrW_\x80\xFD[\x825\x91Pa\x1B\x82` \x84\x01a\x1BPV[\x90P\x92P\x92\x90PV[_\x80_``\x84\x86\x03\x12\x15a\x1B\x9DW_\x80\xFD[\x835\x92Pa\x1B\xAD` \x85\x01a\x1BPV[\x91Pa\x1B\xBB`@\x85\x01a\x1BPV[\x90P\x92P\x92P\x92V[_\x80_\x80_\x80a\x01\x80\x87\x89\x03\x12\x15a\x1B\xDAW_\x80\xFD[a\x1B\xE3\x87a\x1AOV[\x95Pa\x1B\xF2\x88` \x89\x01a\x1AwV[\x94Pa\x1C\x01\x88`\xC0\x89\x01a\x1A\x8DV[\x93Pa\x01@\x87\x015\x92Pa\x01`\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1C%W_\x80\xFD[a\x1C1\x89\x82\x8A\x01a\x1A\x9DV[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[_\x80` \x83\x85\x03\x12\x15a\x1CTW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1CjW_\x80\xFD[a\x1Cv\x85\x82\x86\x01a\x1A\x9DV[\x90\x96\x90\x95P\x93PPPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82`\x02\x0B\x80a\x1C\xC1Wa\x1C\xC1a\x1C\x82V[\x80\x83`\x02\x0B\x07\x91PP\x92\x91PPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x1C\xF2\x82\x84a\x1C\xD0V[\x7F not from regular spacing\0\0\0\0\0\0\0\x81R`\x19\x01\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R_a\x0B\xFE` \x83\x01\x84a\x1D\x1FV[_a\x1D\x88\x82\x84a\x1C\xD0V[\x7F not initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x10\x01\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x82\x01Rb\xFF\xFF\xFF`@\x84\x01Q\x16`@\x82\x01R``\x83\x01Q`\x02\x0B``\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x82\x01Ra\x1F\x12`\xA0\x82\x01\x83\x80Q\x15\x15\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[a\x01 a\x01\0\x82\x01R_a\x03\xCBa\x01 \x83\x01_\x81R` \x01\x90V[_` \x82\x84\x03\x12\x15a\x1F=W_\x80\xFD[PQ\x91\x90PV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a\x1FcWa\x1Fca\x1D\xB5V[P\x92\x91PPV[_\x82a\x1FxWa\x1Fxa\x1C\x82V[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x02\xE4Wa\x02\xE4a\x1D\xB5V[\x80\x82\x01\x80\x82\x11\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x02\xE4Wa\x02\xE4a\x1D\xB5V[`\x01\x84\x11\x15a\x11NW\x80\x85\x04\x81\x11\x15a \x03Wa \x03a\x1D\xB5V[`\x01\x84\x16\x15a \x11W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a\x1F\xE8V[\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a\x0B\xFE`\x01\x83\x01\x84a\x1C\xD0V[_a [\x82\x85a\x1C\xD0V[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra\x12F`\x01\x82\x01\x85a\x1C\xD0V[_a\x03\xCBa \x99\x83\x86a\x1C\xD0V[\x84a\x1C\xD0V[\x96\x95PPPPPPV[_a \xBDa \xB7\x83\x87a\x1C\xD0V[\x85a\x1C\xD0V[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra \x9F`\x01\x82\x01\x85a\x1C\xD0V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`Q`\x04R`$_\xFD\xFE[MockRewardsManager] rewarding pool[MockRewardsManager] loading assets\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Custom error with signature `OutOfBoundRead(uint256,uint256)` and selector `0xbc5f997c`.
```solidity
error OutOfBoundRead(uint256 arrayIndex, uint256 arrayLength);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OutOfBoundRead {
        pub arrayIndex: alloy::sol_types::private::primitives::aliases::U256,
        pub arrayLength: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<OutOfBoundRead> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfBoundRead) -> Self {
                (value.arrayIndex, value.arrayLength)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfBoundRead {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    arrayIndex: tuple.0,
                    arrayLength: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfBoundRead {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfBoundRead(uint256,uint256)";
            const SELECTOR: [u8; 4] = [188u8, 95u8, 153u8, 124u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.arrayIndex),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.arrayLength),
                )
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
constructor(address uniV4PoolManager);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub uniV4PoolManager: alloy::sol_types::private::Address,
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
                    (value.uniV4PoolManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { uniV4PoolManager: tuple.0 }
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
                        &self.uniV4PoolManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `afterRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),int256,bytes)` and selector `0x8db2b652`.
```solidity
function afterRemoveLiquidity(address, PoolKey memory, IPoolManager.ModifyLiquidityParams memory, BalanceDelta, bytes memory) external pure returns (bytes4, BalanceDelta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct afterRemoveLiquidityCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub _2: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        pub _3: <BalanceDelta as alloy::sol_types::SolType>::RustType,
        pub _4: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`afterRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),int256,bytes)`](afterRemoveLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct afterRemoveLiquidityReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
        pub _1: <BalanceDelta as alloy::sol_types::SolType>::RustType,
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
                BalanceDelta,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<afterRemoveLiquidityCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: afterRemoveLiquidityCall) -> Self {
                    (value._0, value._1, value._2, value._3, value._4)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for afterRemoveLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                        _3: tuple.3,
                        _4: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                BalanceDelta,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<afterRemoveLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: afterRemoveLiquidityReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for afterRemoveLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for afterRemoveLiquidityCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                BalanceDelta,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = afterRemoveLiquidityReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<4>,
                BalanceDelta,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "afterRemoveLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),int256,bytes)";
            const SELECTOR: [u8; 4] = [141u8, 178u8, 182u8, 82u8];
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
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self._1),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self._2,
                    ),
                    <BalanceDelta as alloy_sol_types::SolType>::tokenize(&self._3),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._4,
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
    /**Function with signature `beforeAddLiquidity(address,(address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x259982e5`.
```solidity
function beforeAddLiquidity(address, PoolKey memory, IPoolManager.ModifyLiquidityParams memory, bytes memory) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeAddLiquidityCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub _2: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
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
                    (value._0, value._1, value._2, value._3)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for beforeAddLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
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
                        &self._0,
                    ),
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self._1),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self._2,
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
    /**Function with signature `consts()` and selector `0xd86d744e`.
```solidity
function consts() external pure returns (int24 tickSpacing, uint24 poolFee);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constsCall {}
    ///Container type for the return parameters of the [`consts()`](constsCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constsReturn {
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I24,
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
                    (value.tickSpacing, value.poolFee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tickSpacing: tuple.0,
                        poolFee: tuple.1,
                    }
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
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<24>,
            );
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
    /**Function with signature `getGrowthInsideTick(bytes32,int24)` and selector `0x35e81c70`.
```solidity
function getGrowthInsideTick(PoolId id, int24 tick) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct getGrowthInsideTickCall {
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`getGrowthInsideTick(bytes32,int24)`](getGrowthInsideTickCall) function.
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
            type UnderlyingSolTuple<'a> = (PoolId, alloy::sol_types::sol_data::Int<24>);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolId as alloy::sol_types::SolType>::RustType,
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
                    (value.id, value.tick)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getGrowthInsideTickCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { id: tuple.0, tick: tuple.1 }
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
            type Parameters<'a> = (PoolId, alloy::sol_types::sol_data::Int<24>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getGrowthInsideTickReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getGrowthInsideTick(bytes32,int24)";
            const SELECTOR: [u8; 4] = [53u8, 232u8, 28u8, 112u8];
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
    /**Function with signature `updateAfterTickMove(bytes32,int24,int24)` and selector `0x5cd03714`.
```solidity
function updateAfterTickMove(PoolId id, int24 lastTick, int24 newTick) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateAfterTickMoveCall {
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        pub lastTick: alloy::sol_types::private::primitives::aliases::I24,
        pub newTick: alloy::sol_types::private::primitives::aliases::I24,
    }
    ///Container type for the return parameters of the [`updateAfterTickMove(bytes32,int24,int24)`](updateAfterTickMoveCall) function.
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
            impl ::core::convert::From<updateAfterTickMoveCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAfterTickMoveCall) -> Self {
                    (value.id, value.lastTick, value.newTick)
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
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAfterTickMoveReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAfterTickMove(bytes32,int24,int24)";
            const SELECTOR: [u8; 4] = [92u8, 208u8, 55u8, 20u8];
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
        afterRemoveLiquidity(afterRemoveLiquidityCall),
        beforeAddLiquidity(beforeAddLiquidityCall),
        consts(constsCall),
        getGrowthInsideRange(getGrowthInsideRangeCall),
        getGrowthInsideTick(getGrowthInsideTickCall),
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
            [37u8, 153u8, 130u8, 229u8],
            [53u8, 232u8, 28u8, 112u8],
            [92u8, 208u8, 55u8, 20u8],
            [98u8, 136u8, 157u8, 214u8],
            [141u8, 178u8, 182u8, 82u8],
            [196u8, 62u8, 210u8, 200u8],
            [216u8, 109u8, 116u8, 78u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockRewardsManagerCalls {
        const NAME: &'static str = "MockRewardsManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 7usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::afterRemoveLiquidity(_) => {
                    <afterRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeAddLiquidity(_) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::consts(_) => <constsCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getGrowthInsideRange(_) => {
                    <getGrowthInsideRangeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getGrowthInsideTick(_) => {
                    <getGrowthInsideTickCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn afterRemoveLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerCalls> {
                        <afterRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerCalls::afterRemoveLiquidity)
                    }
                    afterRemoveLiquidity
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

            println!("UH OH? - DECODE_SHIMS 3");
            let t = (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate);
            println!("OK!! - DECODE_SHIMS 3");
            t
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::afterRemoveLiquidity(inner) => {
                    <afterRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beforeAddLiquidity(inner) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::afterRemoveLiquidity(inner) => {
                    <afterRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::beforeAddLiquidity(inner) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        AssetsOutOfOrderOrNotUnique(AssetsOutOfOrderOrNotUnique),
        BundleChangeNetNegative(BundleChangeNetNegative),
        MissingHookPermissions(MissingHookPermissions),
        NotUniswap(NotUniswap),
        OutOfBoundRead(OutOfBoundRead),
        Overflow(Overflow),
        ReaderNotAtEnd(ReaderNotAtEnd),
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
            [53u8, 39u8, 141u8, 18u8],
            [100u8, 41u8, 207u8, 210u8],
            [117u8, 56u8, 50u8, 40u8],
            [128u8, 241u8, 26u8, 207u8],
            [188u8, 95u8, 153u8, 124u8],
            [212u8, 155u8, 112u8, 245u8],
            [248u8, 50u8, 134u8, 20u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockRewardsManagerErrors {
        const NAME: &'static str = "MockRewardsManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 8usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AssetsOutOfOrderOrNotUnique(_) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BundleChangeNetNegative(_) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MissingHookPermissions(_) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotUniswap(_) => {
                    <NotUniswap as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBoundRead(_) => {
                    <OutOfBoundRead as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Overflow(_) => <Overflow as alloy_sol_types::SolError>::SELECTOR,
                Self::ReaderNotAtEnd(_) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::SELECTOR
                }
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
                    fn OutOfBoundRead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<MockRewardsManagerErrors> {
                        <OutOfBoundRead as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(MockRewardsManagerErrors::OutOfBoundRead)
                    }
                    OutOfBoundRead
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
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            println!("UH OH? - DECODE_SHIMS 4");
            let t = (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate);
            println!("OK!! - DECODE_SHIMS 4");
            t
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
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
                Self::MissingHookPermissions(inner) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotUniswap(inner) => {
                    <NotUniswap as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OutOfBoundRead(inner) => {
                    <OutOfBoundRead as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ReaderNotAtEnd(inner) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
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
                Self::MissingHookPermissions(inner) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotUniswap(inner) => {
                    <NotUniswap as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OutOfBoundRead(inner) => {
                    <OutOfBoundRead as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::Overflow(inner) => {
                    <Overflow as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ReaderNotAtEnd(inner) => {
                    <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<MockRewardsManagerInstance<T, P, N>>,
    > {
        MockRewardsManagerInstance::<T, P, N>::deploy(provider, uniV4PoolManager)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockRewardsManagerInstance::<T, P, N>::deploy_builder(provider, uniV4PoolManager)
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
        ) -> alloy_contract::Result<MockRewardsManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, uniV4PoolManager);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            uniV4PoolManager,
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
        ///Creates a new call builder for the [`afterRemoveLiquidity`] function.
        pub fn afterRemoveLiquidity(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: <PoolKey as alloy::sol_types::SolType>::RustType,
            _2: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            _3: <BalanceDelta as alloy::sol_types::SolType>::RustType,
            _4: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, afterRemoveLiquidityCall, N> {
            self.call_builder(
                &afterRemoveLiquidityCall {
                    _0,
                    _1,
                    _2,
                    _3,
                    _4,
                },
            )
        }
        ///Creates a new call builder for the [`beforeAddLiquidity`] function.
        pub fn beforeAddLiquidity(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: <PoolKey as alloy::sol_types::SolType>::RustType,
            _2: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeAddLiquidityCall, N> {
            self.call_builder(
                &beforeAddLiquidityCall {
                    _0,
                    _1,
                    _2,
                    _3,
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
        ) -> alloy_contract::SolCallBuilder<T, &P, getGrowthInsideTickCall, N> {
            self.call_builder(
                &getGrowthInsideTickCall {
                    id,
                    tick,
                },
            )
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
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAfterTickMoveCall, N> {
            self.call_builder(
                &updateAfterTickMoveCall {
                    id,
                    lastTick,
                    newTick,
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
