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

interface Angstrom {
    type BalanceDelta is int256;
    type Currency is address;
    struct PoolKey {
        Currency currency0;
        Currency currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    error AssetsOutOfOrderOrNotUnique();
    error BundleChangeNetNegative(address asset);
    error Expired();
    error FillingTooLittle();
    error FillingTooMuch();
    error InvalidPoolKey();
    error InvalidSignature();
    error LimitViolated();
    error MissingHookPermissions(uint160);
    error NonceReuse();
    error NotController();
    error NotNode();
    error NotUniswap();
    error OnlyOncePerBlock();
    error OrderAlreadyExecuted();
    error OutOfBoundRead(uint256 arrayIndex, uint256 arrayLength);
    error OutOfOrderOrDuplicatePairs();
    error Overflow();
    error ReaderNotAtEnd();
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4PoolManager, address governance);

    function afterRemoveLiquidity(address, PoolKey memory, IPoolManager.ModifyLiquidityParams memory, BalanceDelta, bytes memory) external pure returns (bytes4, BalanceDelta);
    function beforeAddLiquidity(address, PoolKey memory, IPoolManager.ModifyLiquidityParams memory, bytes memory) external view returns (bytes4);
    function beforeInitialize(address, PoolKey memory poolKey, uint160, bytes memory) external view returns (bytes4);
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function execute(bytes memory encoded) external;
    function govToggleNodes(address[] memory nodes) external;
    function govUpdateHalfSpread(uint96 newHalfSpreadRay) external;
    function halfSpreadRay() external view returns (uint96);
    function invalidateNonce(uint64 nonce) external;
    function lastBlockUpdated() external view returns (uint64);
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
        "name": "uniV4PoolManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "governance",
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
    "name": "eip712Domain",
    "inputs": [],
    "outputs": [
      {
        "name": "fields",
        "type": "bytes1",
        "internalType": "bytes1"
      },
      {
        "name": "name",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "version",
        "type": "string",
        "internalType": "string"
      },
      {
        "name": "chainId",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "verifyingContract",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "extensions",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "execute",
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
    "name": "govToggleNodes",
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
    "name": "govUpdateHalfSpread",
    "inputs": [
      {
        "name": "newHalfSpreadRay",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "halfSpreadRay",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "invalidateNonce",
    "inputs": [
      {
        "name": "nonce",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "lastBlockUpdated",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
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
    "name": "Expired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FillingTooLittle",
    "inputs": []
  },
  {
    "type": "error",
    "name": "FillingTooMuch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPoolKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LimitViolated",
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
    "name": "NonceReuse",
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
    "name": "OrderAlreadyExecuted",
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
pub mod Angstrom {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610160604052348015610010575f80fd5b50604051613ec2380380613ec283398101604081905261002f91610175565b306080524660a052808260608061007a6040805180820182526008815267416e677374726f6d60c01b60208083019190915282518084019093526002835261763160f01b9083015291565b815160209283012081519183019190912060c082905260e0819052604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8152938401929092529082015246606082015230608082015260a090206101005250506001600160a01b0316610120526100f5610900610114565b6001600160a01b03166101405261010d612080610114565b50506101a6565b806001600160a01b03168130166001600160a01b03161461015757604051630ea7064560e31b81526001600160a01b038216600482015260240160405180910390fd5b50565b80516001600160a01b0381168114610170575f80fd5b919050565b5f8060408385031215610186575f80fd5b61018f8361015a565b915061019d6020840161015a565b90509250929050565b60805160a05160c05160e051610100516101205161014051613c5b6102675f395f818161040f015261076701525f81816102b301528181610375015281816104ca0152818161067b015281816109df01528181610d4201528181610e2f01528181610e56015281816112570152818161128a015281816113a1015281816113de01528181612585015281816127a101528181612b38015261321601525f611bdc01525f611c9601525f611c7001525f611c2001525f611bfd0152613c5b5ff3fe608060405234801561000f575f80fd5b50600436106100c4575f3560e01c8063760f5f271161007d57806391dd73461161005857806391dd7346146101fa57806397125bee1461021a578063c6a98eb91461025b575f80fd5b8063760f5f271461015f57806384b0196e1461018c5780638db2b652146101a7575f80fd5b8063259982e5116100ad578063259982e5146100f05780632a6330cf146101395780633440d8201461014c575f80fd5b806309c5eabe146100c8578063116a5550146100dd575b5f80fd5b6100db6100d6366004613354565b61026e565b005b6100db6100eb366004613393565b61034f565b6101036100fe366004613401565b61035c565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6100db610147366004613478565b6103f7565b61010361015a3660046134a3565b6104b1565b6007546101739067ffffffffffffffff1681565b60405167ffffffffffffffff9091168152602001610130565b6101946105b9565b6040516101309796959493929190613549565b6101c26101b5366004613608565b5f80965096945050505050565b604080517fffffffff000000000000000000000000000000000000000000000000000000009093168352602083019190915201610130565b61020d610208366004613354565b610661565b6040516101309190613689565b60075461023e906801000000000000000090046bffffffffffffffffffffffff1681565b6040516bffffffffffffffffffffffff9091168152602001610130565b6100db61026936600461369b565b61074f565b61027661084a565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c89491906102ea908590859060040161370c565b5f604051808303815f875af1158015610305573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261034a9190810190613785565b505050565b610359338261091d565b50565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146103cc576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f259982e50000000000000000000000000000000000000000000000000000000095945050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610466576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600780546bffffffffffffffffffffffff90921668010000000000000000027fffffffffffffffffffffffff000000000000000000000000ffffffffffffffff909216919091179055565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610521576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603c6105336080870160608801613875565b60020b14158061055757505f61054f6060870160408801613895565b62ffffff1614155b1561058e576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d8200000000000000000000000000000000000000000000000000000000095945050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f80808361064f604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b60603373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106d2576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b825f6106dd82610958565b90925090505f6106ec8361097f565b90935090506106fa8261099c565b61070683600484610ad4565b92506107128383610b08565b925061071f838383610bb2565b925061072a82610c4d565b610735838787610ef4565b5050604080515f8152602081019091529150505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146107be576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b8181101561034a575f8383838181106107db576107db6138b7565b90506020020160208101906107f091906138e4565b73ffffffffffffffffffffffffffffffffffffffff165f90815260066020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff90911615179055506001016107c0565b6007544367ffffffffffffffff90911603610891576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526006602052604090205460ff166108d9576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6108e243610f11565b600780547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661095057638cb888725f526004601cfd5b909155505050565b5f805f610966846044610f2e565b90945090508361097582610f68565b9250925050915091565b5f805f61098d846024610f2e565b90945090508361097582611004565b5f6109a68261109e565b90505f5b8181101561034a575f6109bd84836110a9565b90505f6109c9826110c4565b90505f6109d5836110ec565b90508115610abd577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16630b0d9c09610a368373ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604481018590526064015f604051808303815f87803b158015610aa6575f80fd5b505af1158015610ab8573d5f803e3d5ffd5b505050505b610ac9600482846110f7565b5050506001016109aa565b6003838101935f91813560e81c909101015b808514610aff57610af8858585611130565b9450610ae6565b50929392505050565b6003828101925f91813560e81c9091010181610b22611440565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101919091527fd29cd8ad130638ce44ea185648d2712345e8e456fef8153328ee40916df2790e81529091505b828614610ba857610ba18682848861148a565b9550610b8e565b5093949350505050565b5f80610bbc611440565b60408051610160810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101919091526003878101979293509091803560e81c01015b808714610c4257610c3b8783858989611637565b9650610c27565b509495945050505050565b5f610c578261109e565b90505f5b8181101561034a575f610c6e84836110a9565b90505f610c7a826110ec565b90505f610c8683611841565b90505f610c928461184d565b90505f610cac84610ca3848661392c565b60049190611859565b1215610d01576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024015b60405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff83165f9081526003602052604081208054849290610d3590849061392c565b90915550508015610ee4577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194610d998573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015610dfc575f80fd5b505af1158015610e0e573d5f803e3d5ffd5b50610e549250505073ffffffffffffffffffffffffffffffffffffffff84167f00000000000000000000000000000000000000000000000000000000000000008361189c565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610ebe573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ee2919061393f565b505b505060019092019150610c5b9050565b808201808414610f0b576301842f8c5f526004601cfd5b50505050565b5f680100000000000000008210610f2a57610f2a6118e5565b5090565b6003828101925f918291803560e81c01018184610f4b8784613956565b610f559190613996565b9193505060201b841790505b9250929050565b5f80610f738361109e565b90505f805b82811015610aff575f610f93610f8e87846118f2565b6110ec565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611610ffa576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101610f78565b5f8061100f8361109e565b9050805f0361101f575090919050565b5f61103261102d8583611924565b61193a565b905060015b82811015610aff575f61104d61102d8784611924565b90508263ffffffff168163ffffffff1611611094576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611037565b5f6107498260201c90565b5f604482026110bc845b63ffffffff1690565b019392505050565b5f6110d46014835b013560801c90565b6fffffffffffffffffffffffffffffffff1692915050565b5f813560601c610749565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020610f0b611129825c84611945565b829061195d565b60028301925f908190819081903560f01c8161114f610f8e88846118f2565b60028a01993560f01c925090505f61116a610f8e89856118f2565b90508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16109350836111a95780826111ac565b81815b90965094506111ba92505050565b6040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a8116825289169281019290925294810191909152603c938101939093523083830152822060108a019990919035901c8015611407575f61128661127d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611964565b60a01c60020b90565b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c85604051806060016040528089151581526020016112e187611a0b565b8152602001896113055773fffd8963efd1fc6a506488495d951d5263988d2561130c565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b81526004016113429291906139a9565b6020604051808303815f875af115801561135e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611382919061393f565b505f6113c761127d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611964565b5f85815260026020526040902090915061140490857f00000000000000000000000000000000000000000000000000000000000000008585611a6c565b50505b5f828152600260205260408120611420908c9085611aae565b909b5090506114308a8883611859565b50999a9950505050505050505050565b5f61148561144c611bda565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b833560f81c600181811615156060860152850135608090811c60208601526011860135901c604085015260238501945f91906021013560f01c6114d0610f8e85836118f2565b73ffffffffffffffffffffffffffffffffffffffff16608087015260028701963560f01c611501610f8e86836118f2565b73ffffffffffffffffffffffffffffffffffffffff1660a088015250506002811661152d57855f611537565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660c087015295505f611563876004841615611cd2565b60e089015290975090505f61159a61158e8867ffffffffffffffff4316610100820152610120902090565b60228801526042872090565b90506115a581611d7d565b5f608084166115bd576115b88983611dce565b6115c7565b6115c78983611e46565b90995090506115d68382611e8a565b608088015160208901516115f1918391600188161515611ed2565b5f6116018960c0015183811c1890565b9050611625818a60a001518b604001516116208960ff16600116151590565b611f55565b509798975050505050505050565b5050565b60018501945f903560f81c61164c8682611fcd565b60018116151560808701525f8061166a858a88600887161515612080565b73ffffffffffffffffffffffffffffffffffffffff91821660c08d0152911660a08b0152813560608b018190526020929092019a50925082101590506116dc576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600282166116eb57875f6116f5565b60148801883560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e089015297505f611721896004851615611cd2565b6101008b01529099509050611737888a85612111565b6007549099505f90819061176c908b908d90889088906801000000000000000090046bffffffffffffffffffffffff16612155565b919c50925090505f61177f8b878c6122a7565b90505f60808716611799576117948d83611dce565b6117a3565b6117a38d83611e46565b909d50905060108716156117da576117c68c610140015164ffffffffff166122c1565b6117d5818d610120015161091d565b6117e3565b6117e382611d7d565b6117ed8582611e8a565b60a08c01516118049082908660018b161515611ed2565b5f6118148d60e0015183811c1890565b905061182f818e60c00151866116208c60ff16600116151590565b509b9c9b505050505050505050505050565b5f6110d46024836110cc565b5f6110d46034836110cc565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260208490526040812061189461188b825c856122fb565b9250818361195d565b509392505050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f511417166118dc576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b5f6118ff82845b90612313565b61190a604483613a73565b611913846110b3565b61191d919061392c565b9392505050565b5f61192f82846118f9565b61190a602483613a73565b5f813560e01c610749565b818101828112156107495763c9654ed45f526004601cfd5b80825d5050565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa1580156119df573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a03919061393f565b949350505050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611a66576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b8160020b8160020b1315611a8c57611a878584868585612447565b611aa7565b8160020b8160020b1215611aa757611aa785848685856124db565b5050505050565b6001838101935f9182918291829135821a16151560038801973560e81d5f611ad588612567565b6013808c019b919250813560801c91601081013560e81c010184611b0757611b028b848e878e87876125ab565b611b16565b611b168b848e878e87876126a0565b929e5090995097509550611b2b945050505050565b5f611b3586612786565b9050806fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff1614611bb0576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808416600483015282166024820152604401610cf8565b82876301000000015f828254611bc6919061392c565b90915550889550505050505b935093915050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f0000000000000000000000000000000000000000000000000000000000000000461416611ccf5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47083611d7357843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f818152602081905260409020805c15611dc3576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61163381600161195d565b60148201915f903560601c3682611df286803560e81c808201600390810193920191565b91975092509050611e05838684846127c7565b611e3b576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b509394909350915050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d611e8357638baa579f5f526004601cfd5b9293915050565b81156116335763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d1116811661034a5763f959fdae5f526004601cfd5b81611edf600485836110f7565b8115611f335773ffffffffffffffffffffffffffffffffffffffff8086165f90815260056020908152604080832093881683529290529081208054839290611f28908490613956565b90915550611aa79050565b611aa773ffffffffffffffffffffffffffffffffffffffff851686308461280c565b81611f6260048583611859565b508115611fac5773ffffffffffffffffffffffffffffffffffffffff8086165f90815260056020908152604080832093881683529290529081208054839290611f2890849061392c565b611aa773ffffffffffffffffffffffffffffffffffffffff8516868361189c565b602081161561202b57601081161561200557507fa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e60009052565b507febd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac88619052565b601081161561205a57507fef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f9052565b507f5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d49052565b60028301925f908190819081903560f01c8161209c8a83611924565b90505f6120a882612864565b90505f6120c5610f8e6120ba8561286f565b8c9061ffff166118f2565b90505f6120e2610f8e6120d78661287e565b8d9061ffff166118f2565b9050896120f1578082846120fc565b81816120fc8561288a565b9d9f919e509c9b509950505050505050505050565b5f601082161561213b5760088361013886013760056008840161015b860137600d8301925061214d565b67ffffffffffffffff43166101208501525b509092915050565b5f808080602087161561220957508635608090811c60208a810182905260108a0135831c60408c0181905260308b019a919091013590921c91818310156121c8576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80831115612202576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050612234565b5060108701963560801c60408716612221575f612224565b60015b60ff1660208a0152604089018190525b6040871615158061224757506020871615155b156122755791508161225986846128dd565b915061226e8261226981886128ea565b6128f5565b915061229a565b9050806122828683612900565b92506122978361229281886128ea565b61290b565b92505b5095979096509350505050565b5f611a036122b5858561295a565b60228401526042832090565b80421115610359576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808203828113156107495763c9654ed45f526004601cfd5b5f61231e8360201c90565b905080821061034a576040517fbc5f997c0000000000000000000000000000000000000000000000000000000081526004810183905260248101829052604401610cf8565b60606107498261297a565b8060011660010361239e57818760405160200161238c929190613aa1565b60405160208183030381529060405296505b600181901c905081826040516020016123b8929190613aa1565b60405160208183030381529060405291505f811161236e57509495945050505050565b8860ff168151101561240e57806040516020016123f89190613af6565b60405160208183030381529060405290506123db565b61241783612363565b81604051602001612429929190613b27565b60405160208183030381529060405296505050505050509392505050565b63010000008501545b8160020b8360020b12156124d3575f61248073ffffffffffffffffffffffffffffffffffffffff87168686612a42565b9450905080156124cd578662ffffff8516630100000081106124a4576124a46138b7565b01546124b09083613956565b8762ffffff8616630100000081106124ca576124ca6138b7565b01555b50612450565b505050505050565b63010000008501545b8160020b8360020b13156124d3575f61251473ffffffffffffffffffffffffffffffffffffffff87168686612ab6565b945090508015612561578662ffffff851663010000008110612538576125386138b7565b01546125449083613956565b8762ffffff86166301000000811061255e5761255e6138b7565b01555b506124e4565b5f61074961127d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684611964565b5f80808060015f805b821561265f575f888d146125cd575060108c019b3560801c5b6125d7818461392c565b92506125f5818b6fffffffffffffffffffffffffffffffff16612b02565b6125ff908361392c565b91508d60020b8c60020b136126145750612681565b818f8d62ffffff166301000000811061262f5761262f6138b7565b015f82825461263e919061392c565b9091555061265790508a6126528d8f612b1d565b612b5f565b99505061266e565b8c60020b8b60020b1315612681575b6126788a8c612b79565b9b5092506125b4565b61268b8c89612bca565b9a9d909c50999a509598975050505050505050565b5f80808060015f805b82156127565760108c019b3560801c6126c2818461392c565b92506126e0818b6fffffffffffffffffffffffffffffffff16612b02565b6126ea908361392c565b91508d60020b8c60020b13156127005750612777565b818f8d62ffffff166301000000811061271b5761271b6138b7565b015f82825461272a919061392c565b909155505f905061273b8c8e612b1d565b90506127478b82612c03565b9a506127509050565b50612764565b8c60020b8b60020b13612777575b61276e8a8c612c1d565b9b5092506126a9565b6127818c89612bca565b61268b565b5f61074973ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001683612c46565b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f5114171661285757637939f4245f526004601cfd5b5f60605260405250505050565b5f6004820135610749565b5f61074981835b013560f01c90565b5f610749600283612876565b5f61074982760a70c3c40a64e6c51999090b65f67d9240000000000000613996565b60208110156128c55782811a156128c5576001016128ac565b80825260031b6101000391821c90911b602090910152565b5f61191d83835b90612d04565b5f61191d82846128e4565b5f61191d8284613956565b5f61191d8284612d26565b5f8183611913565b8181101561034a575f61292782602061392c565b9050828111156129345750815b612947826129428184613956565b612d3e565b5061295360208261392c565b9050612913565b5f806010831661296c57610140612970565b6101605b9093209392505050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a9004806129935750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b600f811651948201946001860153600f8160041c1651855360081c8483036129da578015612a0f57632194895a5f526004601cfd5b5050508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe090910190815292915050565b5f808080612a6b603c8087078313908705035b612a60906001613b62565b600281900b60081d91565b9092509050612a9b81612a9573ffffffffffffffffffffffffffffffffffffffff8a168986612ee8565b90612f71565b9094509050612aaa8282613033565b92505050935093915050565b5f808080612ad2612a606001603c808907851390890503613ba3565b9092509050612a9b81612afc73ffffffffffffffffffffffffffffffffffffffff8a168986612ee8565b9061305e565b5f61191d82612b19670de0b6b3a764000086613a73565b0490565b5f611a0373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484613125565b808203608081901c156107495763c9654ed45f526004601cfd5b5f808080612b9e612a60612b8e600188613ba3565b5f603c8083079190911291050390565b91509150612bb081612afc88856131fb565b9094509050612bbf8282613033565b925050509250929050565b808214611633576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b818101608081901c156107495763c9654ed45f526004601cfd5b5f808080612c34603c808707831390870503612a55565b91509150612bb081612a9588856131fb565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526003820160048201529091505f9073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa158015612cc5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ce9919061393f565b95945050505050565b6132de80610f0b848463ffffffff8416565b5f6b033b2e3c9fd0803ce8000000612d1c8385613a73565b61191d9190613996565b5f81612d1c6b033b2e3c9fd0803ce800000085613a73565b60608210612e59577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0820180517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0840180517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe086018051630be77f569095526020909252908490529091612de6612dd6604487613956565b612de186604461392c565b612cf2565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08501929092527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08401527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08301525050565b5f808080612e67858761392c565b805160208201516040830151919650945092509050612e9186612e8b81606061392c565b876132f5565b630be77f56865260208087015260408601859052612ebe612eb387601c61392c565b612de187604461392c565b612ed3612ecc87606061392c565b87876132f5565b92835260208301919091526040909101525050565b5f828152600660209081526040808320848452600501909152812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa158015612cc5573d5f803e3d5ffd5b5f805f61300c8460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826130225760ff613029565b8360ff1681015b9150509250929050565b5f603c60ff831661304a600186900b610100613be4565b6130549190613b62565b61191d9190613be4565b5f805f8360ff0390505f6130ff8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083613114575f612bbf565b8160ff168103925050509250929050565b5f8281526006602090815260408083208484526004019091528120819081906040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290529091505f9073ffffffffffffffffffffffffffffffffffffffff881690631e2eaeaf90602401602060405180830381865afa1580156131b2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131d6919061393f565b6fffffffffffffffffffffffffffffffff81169860809190911d975095505050505050565b5f61191d73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484612ee8565b8860ff1681511015613270578060405160200161325a9190613af6565b604051602081830303815290604052905061323d565b5f8a1261328b5760405180602001604052805f8152506132c2565b6040518060400160405280600181526020017f2d000000000000000000000000000000000000000000000000000000000000008152505b6132cb84612363565b8260405160200161242993929190613c0a565b5f8082846a636f6e736f6c652e6c6f675afa505050565b61330880611aa785858563ffffffff8516565b8082828560045afa50505050565b5f8083601f840112613326575f80fd5b50813567ffffffffffffffff81111561333d575f80fd5b602083019150836020828501011115610f61575f80fd5b5f8060208385031215613365575f80fd5b823567ffffffffffffffff81111561337b575f80fd5b61338785828601613316565b90969095509350505050565b5f602082840312156133a3575f80fd5b813567ffffffffffffffff8116811461191d575f80fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610359575f80fd5b5f60a082840312156133eb575f80fd5b50919050565b5f608082840312156133eb575f80fd5b5f805f805f6101608688031215613416575f80fd5b8535613421816133ba565b945061343087602088016133db565b935061343f8760c088016133f1565b925061014086013567ffffffffffffffff81111561345b575f80fd5b61346788828901613316565b969995985093965092949392505050565b5f60208284031215613488575f80fd5b81356bffffffffffffffffffffffff8116811461191d575f80fd5b5f805f805f61010086880312156134b8575f80fd5b85356134c3816133ba565b94506134d287602088016133db565b935060c08601356134e2816133ba565b925060e086013567ffffffffffffffff81111561345b575f80fd5b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f61358360e08301896134fd565b828103604084015261359581896134fd565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b818110156135f75783518352602093840193909201916001016135d9565b50909b9a5050505050505050505050565b5f805f805f80610180878903121561361e575f80fd5b8635613629816133ba565b955061363888602089016133db565b94506136478860c089016133f1565b9350610140870135925061016087013567ffffffffffffffff81111561366b575f80fd5b61367789828a01613316565b979a9699509497509295939492505050565b602081525f61191d60208301846134fd565b5f80602083850312156136ac575f80fd5b823567ffffffffffffffff8111156136c2575f80fd5b8301601f810185136136d2575f80fd5b803567ffffffffffffffff8111156136e8575f80fd5b8560208260051b84010111156136fc575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f60208284031215613795575f80fd5b815167ffffffffffffffff8111156137ab575f80fd5b8201601f810184136137bb575f80fd5b805167ffffffffffffffff8111156137d5576137d5613758565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff8211171561384157613841613758565b604052818152828201602001861015613858575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215613885575f80fd5b81358060020b811461191d575f80fd5b5f602082840312156138a5575f80fd5b813562ffffff8116811461191d575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f602082840312156138f4575f80fd5b813561191d816133ba565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610749576107496138ff565b5f6020828403121561394f575f80fd5b5051919050565b81810381811115610749576107496138ff565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826139a4576139a4613969565b500490565b73ffffffffffffffffffffffffffffffffffffffff835116815273ffffffffffffffffffffffffffffffffffffffff602084015116602082015262ffffff6040840151166040820152606083015160020b606082015273ffffffffffffffffffffffffffffffffffffffff6080840151166080820152613a5860a08201838051151582526020808201519083015260409081015173ffffffffffffffffffffffffffffffffffffffff16910152565b6101206101008201525f611a0361012083015f815260200190565b8082028115828204841417610749576107496138ff565b5f81518060208401855e5f93019283525090919050565b5f611a03613aaf8386613a8a565b84613a8a565b9695505050505050565b6001841115611bd257808504811115613ada57613ada6138ff565b6001841615613ae857908102905b60019390931c928002613abf565b7f300000000000000000000000000000000000000000000000000000000000000081525f61191d6001830184613a8a565b5f613b328285613a8a565b7f2e000000000000000000000000000000000000000000000000000000000000008152612ce96001820185613a8a565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715610749576107496138ff565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715610749576107496138ff565b5f8260020b8260020b028060020b9150808214613c0357613c036138ff565b5092915050565b5f613c1e613c188387613a8a565b85613a8a565b7f2e000000000000000000000000000000000000000000000000000000000000008152613ab56001820185613a8a56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W_\x80\xFD[P`@Qa>\xC28\x03\x80a>\xC2\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01uV[0`\x80RF`\xA0R\x80\x82``\x80a\0z`@\x80Q\x80\x82\x01\x82R`\x08\x81RgAngstrom`\xC0\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83Rav1`\xF0\x1B\x90\x83\x01R\x91V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xC0\x82\x90R`\xE0\x81\x90R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x93\x84\x01\x92\x90\x92R\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 a\x01\0RPP`\x01`\x01`\xA0\x1B\x03\x16a\x01 Ra\0\xF5a\t\0a\x01\x14V[`\x01`\x01`\xA0\x1B\x03\x16a\x01@Ra\x01\ra \x80a\x01\x14V[PPa\x01\xA6V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x810\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01WW`@Qc\x0E\xA7\x06E`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01pW_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x01\x86W_\x80\xFD[a\x01\x8F\x83a\x01ZV[\x91Pa\x01\x9D` \x84\x01a\x01ZV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa<[a\x02g_9_\x81\x81a\x04\x0F\x01Ra\x07g\x01R_\x81\x81a\x02\xB3\x01R\x81\x81a\x03u\x01R\x81\x81a\x04\xCA\x01R\x81\x81a\x06{\x01R\x81\x81a\t\xDF\x01R\x81\x81a\rB\x01R\x81\x81a\x0E/\x01R\x81\x81a\x0EV\x01R\x81\x81a\x12W\x01R\x81\x81a\x12\x8A\x01R\x81\x81a\x13\xA1\x01R\x81\x81a\x13\xDE\x01R\x81\x81a%\x85\x01R\x81\x81a'\xA1\x01R\x81\x81a+8\x01Ra2\x16\x01R_a\x1B\xDC\x01R_a\x1C\x96\x01R_a\x1Cp\x01R_a\x1C \x01R_a\x1B\xFD\x01Ra<[_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xC4W_5`\xE0\x1C\x80cv\x0F_'\x11a\0}W\x80c\x91\xDDsF\x11a\0XW\x80c\x91\xDDsF\x14a\x01\xFAW\x80c\x97\x12[\xEE\x14a\x02\x1AW\x80c\xC6\xA9\x8E\xB9\x14a\x02[W_\x80\xFD[\x80cv\x0F_'\x14a\x01_W\x80c\x84\xB0\x19n\x14a\x01\x8CW\x80c\x8D\xB2\xB6R\x14a\x01\xA7W_\x80\xFD[\x80c%\x99\x82\xE5\x11a\0\xADW\x80c%\x99\x82\xE5\x14a\0\xF0W\x80c*c0\xCF\x14a\x019W\x80c4@\xD8 \x14a\x01LW_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\0\xC8W\x80c\x11jUP\x14a\0\xDDW[_\x80\xFD[a\0\xDBa\0\xD66`\x04a3TV[a\x02nV[\0[a\0\xDBa\0\xEB6`\x04a3\x93V[a\x03OV[a\x01\x03a\0\xFE6`\x04a4\x01V[a\x03\\V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\x01G6`\x04a4xV[a\x03\xF7V[a\x01\x03a\x01Z6`\x04a4\xA3V[a\x04\xB1V[`\x07Ta\x01s\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x010V[a\x01\x94a\x05\xB9V[`@Qa\x010\x97\x96\x95\x94\x93\x92\x91\x90a5IV[a\x01\xC2a\x01\xB56`\x04a6\x08V[_\x80\x96P\x96\x94PPPPPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x010V[a\x02\ra\x02\x086`\x04a3TV[a\x06aV[`@Qa\x010\x91\x90a6\x89V[`\x07Ta\x02>\x90h\x01\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x010V[a\0\xDBa\x02i6`\x04a6\x9BV[a\x07OV[a\x02va\x08JV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x02\xEA\x90\x85\x90\x85\x90`\x04\x01a7\x0CV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\x05W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03J\x91\x90\x81\x01\x90a7\x85V[PPPV[a\x03Y3\x82a\t\x1DV[PV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xCCW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04fW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05!W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`<a\x053`\x80\x87\x01``\x88\x01a8uV[`\x02\x0B\x14\x15\x80a\x05WWP_a\x05O``\x87\x01`@\x88\x01a8\x95V[b\xFF\xFF\xFF\x16\x14\x15[\x15a\x05\x8EW`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x06O`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xD2W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_a\x06\xDD\x82a\tXV[\x90\x92P\x90P_a\x06\xEC\x83a\t\x7FV[\x90\x93P\x90Pa\x06\xFA\x82a\t\x9CV[a\x07\x06\x83`\x04\x84a\n\xD4V[\x92Pa\x07\x12\x83\x83a\x0B\x08V[\x92Pa\x07\x1F\x83\x83\x83a\x0B\xB2V[\x92Pa\x07*\x82a\x0CMV[a\x075\x83\x87\x87a\x0E\xF4V[PP`@\x80Q_\x81R` \x81\x01\x90\x91R\x91PP[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xBEW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x03JW_\x83\x83\x83\x81\x81\x10a\x07\xDBWa\x07\xDBa8\xB7V[\x90P` \x02\x01` \x81\x01\x90a\x07\xF0\x91\x90a8\xE4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x06` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x07\xC0V[`\x07TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\x08\x91W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x06` R`@\x90 T`\xFF\x16a\x08\xD9W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xE2Ca\x0F\x11V[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\tPWc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_\x80_a\tf\x84`Da\x0F.V[\x90\x94P\x90P\x83a\tu\x82a\x0FhV[\x92P\x92PP\x91P\x91V[_\x80_a\t\x8D\x84`$a\x0F.V[\x90\x94P\x90P\x83a\tu\x82a\x10\x04V[_a\t\xA6\x82a\x10\x9EV[\x90P_[\x81\x81\x10\x15a\x03JW_a\t\xBD\x84\x83a\x10\xA9V[\x90P_a\t\xC9\x82a\x10\xC4V[\x90P_a\t\xD5\x83a\x10\xECV[\x90P\x81\x15a\n\xBDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0B\r\x9C\ta\n6\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\xA6W_\x80\xFD[PZ\xF1\x15\x80\x15a\n\xB8W=_\x80>=_\xFD[PPPP[a\n\xC9`\x04\x82\x84a\x10\xF7V[PPP`\x01\x01a\t\xAAV[`\x03\x83\x81\x01\x93_\x91\x815`\xE8\x1C\x90\x91\x01\x01[\x80\x85\x14a\n\xFFWa\n\xF8\x85\x85\x85a\x110V[\x94Pa\n\xE6V[P\x92\x93\x92PPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x0B\"a\x14@V[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x7F\xD2\x9C\xD8\xAD\x13\x068\xCED\xEA\x18VH\xD2q#E\xE8\xE4V\xFE\xF8\x153(\xEE@\x91m\xF2y\x0E\x81R\x90\x91P[\x82\x86\x14a\x0B\xA8Wa\x0B\xA1\x86\x82\x84\x88a\x14\x8AV[\x95Pa\x0B\x8EV[P\x93\x94\x93PPPPV[_\x80a\x0B\xBCa\x14@V[`@\x80Qa\x01`\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91R`\x03\x87\x81\x01\x97\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x87\x14a\x0CBWa\x0C;\x87\x83\x85\x89\x89a\x167V[\x96Pa\x0C'V[P\x94\x95\x94PPPPPV[_a\x0CW\x82a\x10\x9EV[\x90P_[\x81\x81\x10\x15a\x03JW_a\x0Cn\x84\x83a\x10\xA9V[\x90P_a\x0Cz\x82a\x10\xECV[\x90P_a\x0C\x86\x83a\x18AV[\x90P_a\x0C\x92\x84a\x18MV[\x90P_a\x0C\xAC\x84a\x0C\xA3\x84\x86a9,V[`\x04\x91\x90a\x18YV[\x12\x15a\r\x01W`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x03` R`@\x81 \x80T\x84\x92\x90a\r5\x90\x84\x90a9,V[\x90\x91UPP\x80\x15a\x0E\xE4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\r\x99\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\xFCW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0EW=_\x80>=_\xFD[Pa\x0ET\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x18\x9CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE2\x91\x90a9?V[P[PP`\x01\x90\x92\x01\x91Pa\x0C[\x90PV[\x80\x82\x01\x80\x84\x14a\x0F\x0BWc\x01\x84/\x8C_R`\x04`\x1C\xFD[PPPPV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x0F*Wa\x0F*a\x18\xE5V[P\x90V[`\x03\x82\x81\x01\x92_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81\x84a\x0FK\x87\x84a9VV[a\x0FU\x91\x90a9\x96V[\x91\x93PP` \x1B\x84\x17\x90P[\x92P\x92\x90PV[_\x80a\x0Fs\x83a\x10\x9EV[\x90P_\x80[\x82\x81\x10\x15a\n\xFFW_a\x0F\x93a\x0F\x8E\x87\x84a\x18\xF2V[a\x10\xECV[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0F\xFAW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x0FxV[_\x80a\x10\x0F\x83a\x10\x9EV[\x90P\x80_\x03a\x10\x1FWP\x90\x91\x90PV[_a\x102a\x10-\x85\x83a\x19$V[a\x19:V[\x90P`\x01[\x82\x81\x10\x15a\n\xFFW_a\x10Ma\x10-\x87\x84a\x19$V[\x90P\x82c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x10\x94W`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x107V[_a\x07I\x82` \x1C\x90V[_`D\x82\x02a\x10\xBC\x84[c\xFF\xFF\xFF\xFF\x16\x90V[\x01\x93\x92PPPV[_a\x10\xD4`\x14\x83[\x015`\x80\x1C\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x815``\x1Ca\x07IV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x0F\x0Ba\x11)\x82\\\x84a\x19EV[\x82\x90a\x19]V[`\x02\x83\x01\x92_\x90\x81\x90\x81\x90\x81\x905`\xF0\x1C\x81a\x11Oa\x0F\x8E\x88\x84a\x18\xF2V[`\x02\x8A\x01\x995`\xF0\x1C\x92P\x90P_a\x11ja\x0F\x8E\x89\x85a\x18\xF2V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x93P\x83a\x11\xA9W\x80\x82a\x11\xACV[\x81\x81[\x90\x96P\x94Pa\x11\xBA\x92PPPV[`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x16\x92\x81\x01\x92\x90\x92R\x94\x81\x01\x91\x90\x91R`<\x93\x81\x01\x93\x90\x93R0\x83\x83\x01R\x82 `\x10\x8A\x01\x99\x90\x91\x905\x90\x1C\x80\x15a\x14\x07W_a\x12\x86a\x12}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x19dV[`\xA0\x1C`\x02\x0B\x90V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x85`@Q\x80``\x01`@R\x80\x89\x15\x15\x81R` \x01a\x12\xE1\x87a\x1A\x0BV[\x81R` \x01\x89a\x13\x05Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x13\x0CV[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13B\x92\x91\x90a9\xA9V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x82\x91\x90a9?V[P_a\x13\xC7a\x12}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x19dV[_\x85\x81R`\x02` R`@\x90 \x90\x91Pa\x14\x04\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a\x1AlV[PP[_\x82\x81R`\x02` R`@\x81 a\x14 \x90\x8C\x90\x85a\x1A\xAEV[\x90\x9BP\x90Pa\x140\x8A\x88\x83a\x18YV[P\x99\x9A\x99PPPPPPPPPPV[_a\x14\x85a\x14La\x1B\xDAV[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835`\xF8\x1C`\x01\x81\x81\x16\x15\x15``\x86\x01R\x85\x015`\x80\x90\x81\x1C` \x86\x01R`\x11\x86\x015\x90\x1C`@\x85\x01R`#\x85\x01\x94_\x91\x90`!\x015`\xF0\x1Ca\x14\xD0a\x0F\x8E\x85\x83a\x18\xF2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01R`\x02\x87\x01\x965`\xF0\x1Ca\x15\x01a\x0F\x8E\x86\x83a\x18\xF2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x88\x01RPP`\x02\x81\x16a\x15-W\x85_a\x157V[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x87\x01R\x95P_a\x15c\x87`\x04\x84\x16\x15a\x1C\xD2V[`\xE0\x89\x01R\x90\x97P\x90P_a\x15\x9Aa\x15\x8E\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01Ra\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90Pa\x15\xA5\x81a\x1D}V[_`\x80\x84\x16a\x15\xBDWa\x15\xB8\x89\x83a\x1D\xCEV[a\x15\xC7V[a\x15\xC7\x89\x83a\x1EFV[\x90\x99P\x90Pa\x15\xD6\x83\x82a\x1E\x8AV[`\x80\x88\x01Q` \x89\x01Qa\x15\xF1\x91\x83\x91`\x01\x88\x16\x15\x15a\x1E\xD2V[_a\x16\x01\x89`\xC0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa\x16%\x81\x8A`\xA0\x01Q\x8B`@\x01Qa\x16 \x89`\xFF\x16`\x01\x16\x15\x15\x90V[a\x1FUV[P\x97\x98\x97PPPPPPPPV[PPV[`\x01\x85\x01\x94_\x905`\xF8\x1Ca\x16L\x86\x82a\x1F\xCDV[`\x01\x81\x16\x15\x15`\x80\x87\x01R_\x80a\x16j\x85\x8A\x88`\x08\x87\x16\x15\x15a \x80V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\xC0\x8D\x01R\x91\x16`\xA0\x8B\x01R\x815``\x8B\x01\x81\x90R` \x92\x90\x92\x01\x9AP\x92P\x82\x10\x15\x90Pa\x16\xDCW`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16a\x16\xEBW\x87_a\x16\xF5V[`\x14\x88\x01\x885``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x89\x01R\x97P_a\x17!\x89`\x04\x85\x16\x15a\x1C\xD2V[a\x01\0\x8B\x01R\x90\x99P\x90Pa\x177\x88\x8A\x85a!\x11V[`\x07T\x90\x99P_\x90\x81\x90a\x17l\x90\x8B\x90\x8D\x90\x88\x90\x88\x90h\x01\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!UV[\x91\x9CP\x92P\x90P_a\x17\x7F\x8B\x87\x8Ca\"\xA7V[\x90P_`\x80\x87\x16a\x17\x99Wa\x17\x94\x8D\x83a\x1D\xCEV[a\x17\xA3V[a\x17\xA3\x8D\x83a\x1EFV[\x90\x9DP\x90P`\x10\x87\x16\x15a\x17\xDAWa\x17\xC6\x8Ca\x01@\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a\"\xC1V[a\x17\xD5\x81\x8Da\x01 \x01Qa\t\x1DV[a\x17\xE3V[a\x17\xE3\x82a\x1D}V[a\x17\xED\x85\x82a\x1E\x8AV[`\xA0\x8C\x01Qa\x18\x04\x90\x82\x90\x86`\x01\x8B\x16\x15\x15a\x1E\xD2V[_a\x18\x14\x8D`\xE0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa\x18/\x81\x8E`\xC0\x01Q\x86a\x16 \x8C`\xFF\x16`\x01\x16\x15\x15\x90V[P\x9B\x9C\x9BPPPPPPPPPPPPV[_a\x10\xD4`$\x83a\x10\xCCV[_a\x10\xD4`4\x83a\x10\xCCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a\x18\x94a\x18\x8B\x82\\\x85a\"\xFBV[\x92P\x81\x83a\x19]V[P\x93\x92PPPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x18\xDCWc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[_a\x18\xFF\x82\x84[\x90a#\x13V[a\x19\n`D\x83a:sV[a\x19\x13\x84a\x10\xB3V[a\x19\x1D\x91\x90a9,V[\x93\x92PPPV[_a\x19/\x82\x84a\x18\xF9V[a\x19\n`$\x83a:sV[_\x815`\xE0\x1Ca\x07IV[\x81\x81\x01\x82\x81\x12\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xDFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x03\x91\x90a9?V[\x94\x93PPPPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x1AfW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[\x81`\x02\x0B\x81`\x02\x0B\x13\x15a\x1A\x8CWa\x1A\x87\x85\x84\x86\x85\x85a$GV[a\x1A\xA7V[\x81`\x02\x0B\x81`\x02\x0B\x12\x15a\x1A\xA7Wa\x1A\xA7\x85\x84\x86\x85\x85a$\xDBV[PPPPPV[`\x01\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x915\x82\x1A\x16\x15\x15`\x03\x88\x01\x975`\xE8\x1D_a\x1A\xD5\x88a%gV[`\x13\x80\x8C\x01\x9B\x91\x92P\x815`\x80\x1C\x91`\x10\x81\x015`\xE8\x1C\x01\x01\x84a\x1B\x07Wa\x1B\x02\x8B\x84\x8E\x87\x8E\x87\x87a%\xABV[a\x1B\x16V[a\x1B\x16\x8B\x84\x8E\x87\x8E\x87\x87a&\xA0V[\x92\x9EP\x90\x99P\x97P\x95Pa\x1B+\x94PPPPPV[_a\x1B5\x86a'\x86V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1B\xB0W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x0C\xF8V[\x82\x87c\x01\0\0\0\x01_\x82\x82Ta\x1B\xC6\x91\x90a9,V[\x90\x91UP\x88\x95PPPPP[\x93P\x93\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a\x1C\xCFWP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a\x1DsW\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a\x1D\xC3W`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x163\x81`\x01a\x19]V[`\x14\x82\x01\x91_\x905``\x1C6\x82a\x1D\xF2\x86\x805`\xE8\x1C\x80\x82\x01`\x03\x90\x81\x01\x93\x92\x01\x91V[\x91\x97P\x92P\x90Pa\x1E\x05\x83\x86\x84\x84a'\xC7V[a\x1E;W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x93\x94\x90\x93P\x91PPV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a\x1E\x83Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81\x15a\x163Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x03JWc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x81a\x1E\xDF`\x04\x85\x83a\x10\xF7V[\x81\x15a\x1F3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x1F(\x90\x84\x90a9VV[\x90\x91UPa\x1A\xA7\x90PV[a\x1A\xA7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a(\x0CV[\x81a\x1Fb`\x04\x85\x83a\x18YV[P\x81\x15a\x1F\xACWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x1F(\x90\x84\x90a9,V[a\x1A\xA7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x18\x9CV[` \x81\x16\x15a +W`\x10\x81\x16\x15a \x05WP\x7F\xA9\xE5)MDO\xBA\xEB\\\xA0_+$\xC5\xD8)D\x10\xF3\x14\x13\x04\x8ED]\x88\x1E.\xF6\x9E`\0\x90RV[P\x7F\xEB\xD5\t\x1C9oy\x05nEE_J\x93\xBB\xB0\x16\xC2\x171K\xB25k\"\xD1\xC183\xAC\x88a\x90RV[`\x10\x81\x16\x15a ZWP\x7F\xEF\x0C\xCE\x88\xFD\xA0J\xB3\xF8F\x18\x823r\xCFv\xF6KE\x11\xCE\x8Cyc\x0E\xAB\xC2\x9E\xBC\x9B\x96\x8F\x90RV[P\x7F[\x9DI\xCF\xEDH\xF8\xC9\xD1\xA8c\xF6\x1C$y\xC5y\xFA)\x9C%\xA32\x11\xFC\x85s\x90\xCE\xCC\xE6\xD4\x90RV[`\x02\x83\x01\x92_\x90\x81\x90\x81\x90\x81\x905`\xF0\x1C\x81a \x9C\x8A\x83a\x19$V[\x90P_a \xA8\x82a(dV[\x90P_a \xC5a\x0F\x8Ea \xBA\x85a(oV[\x8C\x90a\xFF\xFF\x16a\x18\xF2V[\x90P_a \xE2a\x0F\x8Ea \xD7\x86a(~V[\x8D\x90a\xFF\xFF\x16a\x18\xF2V[\x90P\x89a \xF1W\x80\x82\x84a \xFCV[\x81\x81a \xFC\x85a(\x8AV[\x9D\x9F\x91\x9EP\x9C\x9BP\x99PPPPPPPPPPV[_`\x10\x82\x16\x15a!;W`\x08\x83a\x018\x86\x017`\x05`\x08\x84\x01a\x01[\x86\x017`\r\x83\x01\x92Pa!MV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01 \x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x87\x16\x15a\"\tWP\x865`\x80\x90\x81\x1C` \x8A\x81\x01\x82\x90R`\x10\x8A\x015\x83\x1C`@\x8C\x01\x81\x90R`0\x8B\x01\x9A\x91\x90\x91\x015\x90\x92\x1C\x91\x81\x83\x10\x15a!\xC8W`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a\"\x02W`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\"4V[P`\x10\x87\x01\x965`\x80\x1C`@\x87\x16a\"!W_a\"$V[`\x01[`\xFF\x16` \x8A\x01R`@\x89\x01\x81\x90R[`@\x87\x16\x15\x15\x80a\"GWP` \x87\x16\x15\x15[\x15a\"uW\x91P\x81a\"Y\x86\x84a(\xDDV[\x91Pa\"n\x82a\"i\x81\x88a(\xEAV[a(\xF5V[\x91Pa\"\x9AV[\x90P\x80a\"\x82\x86\x83a)\0V[\x92Pa\"\x97\x83a\"\x92\x81\x88a(\xEAV[a)\x0BV[\x92P[P\x95\x97\x90\x96P\x93PPPPV[_a\x1A\x03a\"\xB5\x85\x85a)ZV[`\"\x84\x01R`B\x83 \x90V[\x80B\x11\x15a\x03YW`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[_a#\x1E\x83` \x1C\x90V[\x90P\x80\x82\x10a\x03JW`@Q\x7F\xBC_\x99|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x0C\xF8V[``a\x07I\x82a)zV[\x80`\x01\x16`\x01\x03a#\x9EW\x81\x87`@Q` \x01a#\x8C\x92\x91\x90a:\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P[`\x01\x81\x90\x1C\x90P\x81\x82`@Q` \x01a#\xB8\x92\x91\x90a:\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P_\x81\x11a#nWP\x94\x95\x94PPPPPV[\x88`\xFF\x16\x81Q\x10\x15a$\x0EW\x80`@Q` \x01a#\xF8\x91\x90a:\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa#\xDBV[a$\x17\x83a#cV[\x81`@Q` \x01a$)\x92\x91\x90a;'V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96PPPPPPP\x93\x92PPPV[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x12\x15a$\xD3W_a$\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a*BV[\x94P\x90P\x80\x15a$\xCDW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a$\xA4Wa$\xA4a8\xB7V[\x01Ta$\xB0\x90\x83a9VV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a$\xCAWa$\xCAa8\xB7V[\x01U[Pa$PV[PPPPPPV[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x13\x15a$\xD3W_a%\x14s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a*\xB6V[\x94P\x90P\x80\x15a%aW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a%8Wa%8a8\xB7V[\x01Ta%D\x90\x83a9VV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a%^Wa%^a8\xB7V[\x01U[Pa$\xE4V[_a\x07Ia\x12}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\x19dV[_\x80\x80\x80`\x01_\x80[\x82\x15a&_W_\x88\x8D\x14a%\xCDWP`\x10\x8C\x01\x9B5`\x80\x1C[a%\xD7\x81\x84a9,V[\x92Pa%\xF5\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a+\x02V[a%\xFF\x90\x83a9,V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13a&\x14WPa&\x81V[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a&/Wa&/a8\xB7V[\x01_\x82\x82Ta&>\x91\x90a9,V[\x90\x91UPa&W\x90P\x8Aa&R\x8D\x8Fa+\x1DV[a+_V[\x99PPa&nV[\x8C`\x02\x0B\x8B`\x02\x0B\x13\x15a&\x81W[a&x\x8A\x8Ca+yV[\x9BP\x92Pa%\xB4V[a&\x8B\x8C\x89a+\xCAV[\x9A\x9D\x90\x9CP\x99\x9AP\x95\x98\x97PPPPPPPPV[_\x80\x80\x80`\x01_\x80[\x82\x15a'VW`\x10\x8C\x01\x9B5`\x80\x1Ca&\xC2\x81\x84a9,V[\x92Pa&\xE0\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a+\x02V[a&\xEA\x90\x83a9,V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13\x15a'\0WPa'wV[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a'\x1BWa'\x1Ba8\xB7V[\x01_\x82\x82Ta'*\x91\x90a9,V[\x90\x91UP_\x90Pa';\x8C\x8Ea+\x1DV[\x90Pa'G\x8B\x82a,\x03V[\x9APa'P\x90PV[Pa'dV[\x8C`\x02\x0B\x8B`\x02\x0B\x13a'wW[a'n\x8A\x8Ca,\x1DV[\x9BP\x92Pa&\xA9V[a'\x81\x8C\x89a+\xCAV[a&\x8BV[_a\x07Is\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x83a,FV[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a(WWcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[_`\x04\x82\x015a\x07IV[_a\x07I\x81\x83[\x015`\xF0\x1C\x90V[_a\x07I`\x02\x83a(vV[_a\x07I\x82v\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0a9\x96V[` \x81\x10\x15a(\xC5W\x82\x81\x1A\x15a(\xC5W`\x01\x01a(\xACV[\x80\x82R`\x03\x1Ba\x01\0\x03\x91\x82\x1C\x90\x91\x1B` \x90\x91\x01RV[_a\x19\x1D\x83\x83[\x90a-\x04V[_a\x19\x1D\x82\x84a(\xE4V[_a\x19\x1D\x82\x84a9VV[_a\x19\x1D\x82\x84a-&V[_\x81\x83a\x19\x13V[\x81\x81\x10\x15a\x03JW_a)'\x82` a9,V[\x90P\x82\x81\x11\x15a)4WP\x81[a)G\x82a)B\x81\x84a9VV[a->V[Pa)S` \x82a9,V[\x90Pa)\x13V[_\x80`\x10\x83\x16a)lWa\x01@a)pV[a\x01`[\x90\x93 \x93\x92PPPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a)\x93WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[`\x0F\x81\x16Q\x94\x82\x01\x94`\x01\x86\x01S`\x0F\x81`\x04\x1C\x16Q\x85S`\x08\x1C\x84\x83\x03a)\xDAW\x80\x15a*\x0FWc!\x94\x89Z_R`\x04`\x1C\xFD[PPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x92\x91PPV[_\x80\x80\x80a*k`<\x80\x87\x07\x83\x13\x90\x87\x05\x03[a*`\x90`\x01a;bV[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa*\x9B\x81a*\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a.\xE8V[\x90a/qV[\x90\x94P\x90Pa*\xAA\x82\x82a03V[\x92PPP\x93P\x93\x91PPV[_\x80\x80\x80a*\xD2a*``\x01`<\x80\x89\x07\x85\x13\x90\x89\x05\x03a;\xA3V[\x90\x92P\x90Pa*\x9B\x81a*\xFCs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a.\xE8V[\x90a0^V[_a\x19\x1D\x82a+\x19g\r\xE0\xB6\xB3\xA7d\0\0\x86a:sV[\x04\x90V[_a\x1A\x03s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a1%V[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a+\x9Ea*`a+\x8E`\x01\x88a;\xA3V[_`<\x80\x83\x07\x91\x90\x91\x12\x91\x05\x03\x90V[\x91P\x91Pa+\xB0\x81a*\xFC\x88\x85a1\xFBV[\x90\x94P\x90Pa+\xBF\x82\x82a03V[\x92PPP\x92P\x92\x90PV[\x80\x82\x14a\x163W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a,4`<\x80\x87\x07\x83\x13\x90\x87\x05\x03a*UV[\x91P\x91Pa+\xB0\x81a*\x95\x88\x85a1\xFBV[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x03\x82\x01`\x04\x82\x01R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xE9\x91\x90a9?V[\x95\x94PPPPPV[a2\xDE\x80a\x0F\x0B\x84\x84c\xFF\xFF\xFF\xFF\x84\x16V[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a-\x1C\x83\x85a:sV[a\x19\x1D\x91\x90a9\x96V[_\x81a-\x1Ck\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a:sV[``\x82\x10a.YW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x01\x80Qc\x0B\xE7\x7FV\x90\x95R` \x90\x92R\x90\x84\x90R\x90\x91a-\xE6a-\xD6`D\x87a9VV[a-\xE1\x86`Da9,V[a,\xF2V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x85\x01\x92\x90\x92R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x01RPPV[_\x80\x80\x80a.g\x85\x87a9,V[\x80Q` \x82\x01Q`@\x83\x01Q\x91\x96P\x94P\x92P\x90Pa.\x91\x86a.\x8B\x81``a9,V[\x87a2\xF5V[c\x0B\xE7\x7FV\x86R` \x80\x87\x01R`@\x86\x01\x85\x90Ra.\xBEa.\xB3\x87`\x1Ca9,V[a-\xE1\x87`Da9,V[a.\xD3a.\xCC\x87``a9,V[\x87\x87a2\xF5V[\x92\x83R` \x83\x01\x91\x90\x91R`@\x90\x91\x01RPPV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC5W=_\x80>=_\xFD[_\x80_a0\x0C\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a0\"W`\xFFa0)V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_`<`\xFF\x83\x16a0J`\x01\x86\x90\x0Ba\x01\0a;\xE4V[a0T\x91\x90a;bV[a\x19\x1D\x91\x90a;\xE4V[_\x80_\x83`\xFF\x03\x90P_a0\xFF\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a1\x14W_a+\xBFV[\x81`\xFF\x16\x81\x03\x92PPP\x92P\x92\x90PV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x04\x01\x90\x91R\x81 \x81\x90\x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xB2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xD6\x91\x90a9?V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\x80\x91\x90\x91\x1D\x97P\x95PPPPPPV[_a\x19\x1Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a.\xE8V[\x88`\xFF\x16\x81Q\x10\x15a2pW\x80`@Q` \x01a2Z\x91\x90a:\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa2=V[_\x8A\x12a2\x8BW`@Q\x80` \x01`@R\x80_\x81RPa2\xC2V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP[a2\xCB\x84a#cV[\x82`@Q` \x01a$)\x93\x92\x91\x90a<\nV[_\x80\x82\x84jconsole.logZ\xFAPPPV[a3\x08\x80a\x1A\xA7\x85\x85\x85c\xFF\xFF\xFF\xFF\x85\x16V[\x80\x82\x82\x85`\x04Z\xFAPPPPV[_\x80\x83`\x1F\x84\x01\x12a3&W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3=W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0FaW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a3eW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3{W_\x80\xFD[a3\x87\x85\x82\x86\x01a3\x16V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a3\xA3W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x19\x1DW_\x80\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03YW_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a3\xEBW_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a3\xEBW_\x80\xFD[_\x80_\x80_a\x01`\x86\x88\x03\x12\x15a4\x16W_\x80\xFD[\x855a4!\x81a3\xBAV[\x94Pa40\x87` \x88\x01a3\xDBV[\x93Pa4?\x87`\xC0\x88\x01a3\xF1V[\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4[W_\x80\xFD[a4g\x88\x82\x89\x01a3\x16V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a4\x88W_\x80\xFD[\x815k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x19\x1DW_\x80\xFD[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a4\xB8W_\x80\xFD[\x855a4\xC3\x81a3\xBAV[\x94Pa4\xD2\x87` \x88\x01a3\xDBV[\x93P`\xC0\x86\x015a4\xE2\x81a3\xBAV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4[W_\x80\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_a5\x83`\xE0\x83\x01\x89a4\xFDV[\x82\x81\x03`@\x84\x01Ra5\x95\x81\x89a4\xFDV[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a5\xF7W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a5\xD9V[P\x90\x9B\x9APPPPPPPPPPPV[_\x80_\x80_\x80a\x01\x80\x87\x89\x03\x12\x15a6\x1EW_\x80\xFD[\x865a6)\x81a3\xBAV[\x95Pa68\x88` \x89\x01a3\xDBV[\x94Pa6G\x88`\xC0\x89\x01a3\xF1V[\x93Pa\x01@\x87\x015\x92Pa\x01`\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6kW_\x80\xFD[a6w\x89\x82\x8A\x01a3\x16V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[` \x81R_a\x19\x1D` \x83\x01\x84a4\xFDV[_\x80` \x83\x85\x03\x12\x15a6\xACW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xC2W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a6\xD2W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xE8W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a6\xFCW_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a7\x95W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xABW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a7\xBBW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xD5Wa7\xD5a7XV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8AWa8Aa7XV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a8XW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a8\x85W_\x80\xFD[\x815\x80`\x02\x0B\x81\x14a\x19\x1DW_\x80\xFD[_` \x82\x84\x03\x12\x15a8\xA5W_\x80\xFD[\x815b\xFF\xFF\xFF\x81\x16\x81\x14a\x19\x1DW_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a8\xF4W_\x80\xFD[\x815a\x19\x1D\x81a3\xBAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07IWa\x07Ia8\xFFV[_` \x82\x84\x03\x12\x15a9OW_\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x07IWa\x07Ia8\xFFV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a9\xA4Wa9\xA4a9iV[P\x04\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x82\x01Rb\xFF\xFF\xFF`@\x84\x01Q\x16`@\x82\x01R``\x83\x01Q`\x02\x0B``\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x82\x01Ra:X`\xA0\x82\x01\x83\x80Q\x15\x15\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[a\x01 a\x01\0\x82\x01R_a\x1A\x03a\x01 \x83\x01_\x81R` \x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07IWa\x07Ia8\xFFV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x1A\x03a:\xAF\x83\x86a:\x8AV[\x84a:\x8AV[\x96\x95PPPPPPV[`\x01\x84\x11\x15a\x1B\xD2W\x80\x85\x04\x81\x11\x15a:\xDAWa:\xDAa8\xFFV[`\x01\x84\x16\x15a:\xE8W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a:\xBFV[\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a\x19\x1D`\x01\x83\x01\x84a:\x8AV[_a;2\x82\x85a:\x8AV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra,\xE9`\x01\x82\x01\x85a:\x8AV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x07IWa\x07Ia8\xFFV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x07IWa\x07Ia8\xFFV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a<\x03Wa<\x03a8\xFFV[P\x92\x91PPV[_a<\x1Ea<\x18\x83\x87a:\x8AV[\x85a:\x8AV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra:\xB5`\x01\x82\x01\x85a:\x8AV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100c4575f3560e01c8063760f5f271161007d57806391dd73461161005857806391dd7346146101fa57806397125bee1461021a578063c6a98eb91461025b575f80fd5b8063760f5f271461015f57806384b0196e1461018c5780638db2b652146101a7575f80fd5b8063259982e5116100ad578063259982e5146100f05780632a6330cf146101395780633440d8201461014c575f80fd5b806309c5eabe146100c8578063116a5550146100dd575b5f80fd5b6100db6100d6366004613354565b61026e565b005b6100db6100eb366004613393565b61034f565b6101036100fe366004613401565b61035c565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6100db610147366004613478565b6103f7565b61010361015a3660046134a3565b6104b1565b6007546101739067ffffffffffffffff1681565b60405167ffffffffffffffff9091168152602001610130565b6101946105b9565b6040516101309796959493929190613549565b6101c26101b5366004613608565b5f80965096945050505050565b604080517fffffffff000000000000000000000000000000000000000000000000000000009093168352602083019190915201610130565b61020d610208366004613354565b610661565b6040516101309190613689565b60075461023e906801000000000000000090046bffffffffffffffffffffffff1681565b6040516bffffffffffffffffffffffff9091168152602001610130565b6100db61026936600461369b565b61074f565b61027661084a565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c89491906102ea908590859060040161370c565b5f604051808303815f875af1158015610305573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016820160405261034a9190810190613785565b505050565b610359338261091d565b50565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146103cc576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f259982e50000000000000000000000000000000000000000000000000000000095945050505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610466576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600780546bffffffffffffffffffffffff90921668010000000000000000027fffffffffffffffffffffffff000000000000000000000000ffffffffffffffff909216919091179055565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610521576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b603c6105336080870160608801613875565b60020b14158061055757505f61054f6060870160408801613895565b62ffffff1614155b1561058e576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d8200000000000000000000000000000000000000000000000000000000095945050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f80808361064f604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b60603373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106d2576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b825f6106dd82610958565b90925090505f6106ec8361097f565b90935090506106fa8261099c565b61070683600484610ad4565b92506107128383610b08565b925061071f838383610bb2565b925061072a82610c4d565b610735838787610ef4565b5050604080515f8152602081019091529150505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146107be576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b8181101561034a575f8383838181106107db576107db6138b7565b90506020020160208101906107f091906138e4565b73ffffffffffffffffffffffffffffffffffffffff165f90815260066020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff90911615179055506001016107c0565b6007544367ffffffffffffffff90911603610891576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526006602052604090205460ff166108d9576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6108e243610f11565b600780547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661095057638cb888725f526004601cfd5b909155505050565b5f805f610966846044610f2e565b90945090508361097582610f68565b9250925050915091565b5f805f61098d846024610f2e565b90945090508361097582611004565b5f6109a68261109e565b90505f5b8181101561034a575f6109bd84836110a9565b90505f6109c9826110c4565b90505f6109d5836110ec565b90508115610abd577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16630b0d9c09610a368373ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604481018590526064015f604051808303815f87803b158015610aa6575f80fd5b505af1158015610ab8573d5f803e3d5ffd5b505050505b610ac9600482846110f7565b5050506001016109aa565b6003838101935f91813560e81c909101015b808514610aff57610af8858585611130565b9450610ae6565b50929392505050565b6003828101925f91813560e81c9091010181610b22611440565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101919091527fd29cd8ad130638ce44ea185648d2712345e8e456fef8153328ee40916df2790e81529091505b828614610ba857610ba18682848861148a565b9550610b8e565b5093949350505050565b5f80610bbc611440565b60408051610160810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101919091526003878101979293509091803560e81c01015b808714610c4257610c3b8783858989611637565b9650610c27565b509495945050505050565b5f610c578261109e565b90505f5b8181101561034a575f610c6e84836110a9565b90505f610c7a826110ec565b90505f610c8683611841565b90505f610c928461184d565b90505f610cac84610ca3848661392c565b60049190611859565b1215610d01576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024015b60405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff83165f9081526003602052604081208054849290610d3590849061392c565b90915550508015610ee4577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194610d998573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015610dfc575f80fd5b505af1158015610e0e573d5f803e3d5ffd5b50610e549250505073ffffffffffffffffffffffffffffffffffffffff84167f00000000000000000000000000000000000000000000000000000000000000008361189c565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015610ebe573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ee2919061393f565b505b505060019092019150610c5b9050565b808201808414610f0b576301842f8c5f526004601cfd5b50505050565b5f680100000000000000008210610f2a57610f2a6118e5565b5090565b6003828101925f918291803560e81c01018184610f4b8784613956565b610f559190613996565b9193505060201b841790505b9250929050565b5f80610f738361109e565b90505f805b82811015610aff575f610f93610f8e87846118f2565b6110ec565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611610ffa576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101610f78565b5f8061100f8361109e565b9050805f0361101f575090919050565b5f61103261102d8583611924565b61193a565b905060015b82811015610aff575f61104d61102d8784611924565b90508263ffffffff168163ffffffff1611611094576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611037565b5f6107498260201c90565b5f604482026110bc845b63ffffffff1690565b019392505050565b5f6110d46014835b013560801c90565b6fffffffffffffffffffffffffffffffff1692915050565b5f813560601c610749565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020610f0b611129825c84611945565b829061195d565b60028301925f908190819081903560f01c8161114f610f8e88846118f2565b60028a01993560f01c925090505f61116a610f8e89856118f2565b90508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff16109350836111a95780826111ac565b81815b90965094506111ba92505050565b6040805160a080820183525f8083526020808401829052838501829052606080850183905260809485018390528551808501875273ffffffffffffffffffffffffffffffffffffffff8a8116825289169281019290925294810191909152603c938101939093523083830152822060108a019990919035901c8015611407575f61128661127d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611964565b60a01c60020b90565b90507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663f3cd914c85604051806060016040528089151581526020016112e187611a0b565b8152602001896113055773fffd8963efd1fc6a506488495d951d5263988d2561130c565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff168152506040518363ffffffff1660e01b81526004016113429291906139a9565b6020604051808303815f875af115801561135e573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611382919061393f565b505f6113c761127d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611964565b5f85815260026020526040902090915061140490857f00000000000000000000000000000000000000000000000000000000000000008585611a6c565b50505b5f828152600260205260408120611420908c9085611aae565b909b5090506114308a8883611859565b50999a9950505050505050505050565b5f61148561144c611bda565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b833560f81c600181811615156060860152850135608090811c60208601526011860135901c604085015260238501945f91906021013560f01c6114d0610f8e85836118f2565b73ffffffffffffffffffffffffffffffffffffffff16608087015260028701963560f01c611501610f8e86836118f2565b73ffffffffffffffffffffffffffffffffffffffff1660a088015250506002811661152d57855f611537565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660c087015295505f611563876004841615611cd2565b60e089015290975090505f61159a61158e8867ffffffffffffffff4316610100820152610120902090565b60228801526042872090565b90506115a581611d7d565b5f608084166115bd576115b88983611dce565b6115c7565b6115c78983611e46565b90995090506115d68382611e8a565b608088015160208901516115f1918391600188161515611ed2565b5f6116018960c0015183811c1890565b9050611625818a60a001518b604001516116208960ff16600116151590565b611f55565b509798975050505050505050565b5050565b60018501945f903560f81c61164c8682611fcd565b60018116151560808701525f8061166a858a88600887161515612080565b73ffffffffffffffffffffffffffffffffffffffff91821660c08d0152911660a08b0152813560608b018190526020929092019a50925082101590506116dc576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600282166116eb57875f6116f5565b60148801883560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e089015297505f611721896004851615611cd2565b6101008b01529099509050611737888a85612111565b6007549099505f90819061176c908b908d90889088906801000000000000000090046bffffffffffffffffffffffff16612155565b919c50925090505f61177f8b878c6122a7565b90505f60808716611799576117948d83611dce565b6117a3565b6117a38d83611e46565b909d50905060108716156117da576117c68c610140015164ffffffffff166122c1565b6117d5818d610120015161091d565b6117e3565b6117e382611d7d565b6117ed8582611e8a565b60a08c01516118049082908660018b161515611ed2565b5f6118148d60e0015183811c1890565b905061182f818e60c00151866116208c60ff16600116151590565b509b9c9b505050505050505050505050565b5f6110d46024836110cc565b5f6110d46034836110cc565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260208490526040812061189461188b825c856122fb565b9250818361195d565b509392505050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f511417166118dc576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b5f6118ff82845b90612313565b61190a604483613a73565b611913846110b3565b61191d919061392c565b9392505050565b5f61192f82846118f9565b61190a602483613a73565b5f813560e01c610749565b818101828112156107495763c9654ed45f526004601cfd5b80825d5050565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa1580156119df573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611a03919061393f565b949350505050565b5f7f8000000000000000000000000000000000000000000000000000000000000000821115611a66576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b8160020b8160020b1315611a8c57611a878584868585612447565b611aa7565b8160020b8160020b1215611aa757611aa785848685856124db565b5050505050565b6001838101935f9182918291829135821a16151560038801973560e81d5f611ad588612567565b6013808c019b919250813560801c91601081013560e81c010184611b0757611b028b848e878e87876125ab565b611b16565b611b168b848e878e87876126a0565b929e5090995097509550611b2b945050505050565b5f611b3586612786565b9050806fffffffffffffffffffffffffffffffff16826fffffffffffffffffffffffffffffffff1614611bb0576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808416600483015282166024820152604401610cf8565b82876301000000015f828254611bc6919061392c565b90915550889550505050505b935093915050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f0000000000000000000000000000000000000000000000000000000000000000461416611ccf5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47083611d7357843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f818152602081905260409020805c15611dc3576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61163381600161195d565b60148201915f903560601c3682611df286803560e81c808201600390810193920191565b91975092509050611e05838684846127c7565b611e3b576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b509394909350915050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d611e8357638baa579f5f526004601cfd5b9293915050565b81156116335763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d1116811661034a5763f959fdae5f526004601cfd5b81611edf600485836110f7565b8115611f335773ffffffffffffffffffffffffffffffffffffffff8086165f90815260056020908152604080832093881683529290529081208054839290611f28908490613956565b90915550611aa79050565b611aa773ffffffffffffffffffffffffffffffffffffffff851686308461280c565b81611f6260048583611859565b508115611fac5773ffffffffffffffffffffffffffffffffffffffff8086165f90815260056020908152604080832093881683529290529081208054839290611f2890849061392c565b611aa773ffffffffffffffffffffffffffffffffffffffff8516868361189c565b602081161561202b57601081161561200557507fa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e60009052565b507febd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac88619052565b601081161561205a57507fef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f9052565b507f5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d49052565b60028301925f908190819081903560f01c8161209c8a83611924565b90505f6120a882612864565b90505f6120c5610f8e6120ba8561286f565b8c9061ffff166118f2565b90505f6120e2610f8e6120d78661287e565b8d9061ffff166118f2565b9050896120f1578082846120fc565b81816120fc8561288a565b9d9f919e509c9b509950505050505050505050565b5f601082161561213b5760088361013886013760056008840161015b860137600d8301925061214d565b67ffffffffffffffff43166101208501525b509092915050565b5f808080602087161561220957508635608090811c60208a810182905260108a0135831c60408c0181905260308b019a919091013590921c91818310156121c8576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80831115612202576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050612234565b5060108701963560801c60408716612221575f612224565b60015b60ff1660208a0152604089018190525b6040871615158061224757506020871615155b156122755791508161225986846128dd565b915061226e8261226981886128ea565b6128f5565b915061229a565b9050806122828683612900565b92506122978361229281886128ea565b61290b565b92505b5095979096509350505050565b5f611a036122b5858561295a565b60228401526042832090565b80421115610359576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808203828113156107495763c9654ed45f526004601cfd5b5f61231e8360201c90565b905080821061034a576040517fbc5f997c0000000000000000000000000000000000000000000000000000000081526004810183905260248101829052604401610cf8565b60606107498261297a565b8060011660010361239e57818760405160200161238c929190613aa1565b60405160208183030381529060405296505b600181901c905081826040516020016123b8929190613aa1565b60405160208183030381529060405291505f811161236e57509495945050505050565b8860ff168151101561240e57806040516020016123f89190613af6565b60405160208183030381529060405290506123db565b61241783612363565b81604051602001612429929190613b27565b60405160208183030381529060405296505050505050509392505050565b63010000008501545b8160020b8360020b12156124d3575f61248073ffffffffffffffffffffffffffffffffffffffff87168686612a42565b9450905080156124cd578662ffffff8516630100000081106124a4576124a46138b7565b01546124b09083613956565b8762ffffff8616630100000081106124ca576124ca6138b7565b01555b50612450565b505050505050565b63010000008501545b8160020b8360020b13156124d3575f61251473ffffffffffffffffffffffffffffffffffffffff87168686612ab6565b945090508015612561578662ffffff851663010000008110612538576125386138b7565b01546125449083613956565b8762ffffff86166301000000811061255e5761255e6138b7565b01555b506124e4565b5f61074961127d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684611964565b5f80808060015f805b821561265f575f888d146125cd575060108c019b3560801c5b6125d7818461392c565b92506125f5818b6fffffffffffffffffffffffffffffffff16612b02565b6125ff908361392c565b91508d60020b8c60020b136126145750612681565b818f8d62ffffff166301000000811061262f5761262f6138b7565b015f82825461263e919061392c565b9091555061265790508a6126528d8f612b1d565b612b5f565b99505061266e565b8c60020b8b60020b1315612681575b6126788a8c612b79565b9b5092506125b4565b61268b8c89612bca565b9a9d909c50999a509598975050505050505050565b5f80808060015f805b82156127565760108c019b3560801c6126c2818461392c565b92506126e0818b6fffffffffffffffffffffffffffffffff16612b02565b6126ea908361392c565b91508d60020b8c60020b13156127005750612777565b818f8d62ffffff166301000000811061271b5761271b6138b7565b015f82825461272a919061392c565b909155505f905061273b8c8e612b1d565b90506127478b82612c03565b9a506127509050565b50612764565b8c60020b8b60020b13612777575b61276e8a8c612c1d565b9b5092506126a9565b6127818c89612bca565b61268b565b5f61074973ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001683612c46565b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f5114171661285757637939f4245f526004601cfd5b5f60605260405250505050565b5f6004820135610749565b5f61074981835b013560f01c90565b5f610749600283612876565b5f61074982760a70c3c40a64e6c51999090b65f67d9240000000000000613996565b60208110156128c55782811a156128c5576001016128ac565b80825260031b6101000391821c90911b602090910152565b5f61191d83835b90612d04565b5f61191d82846128e4565b5f61191d8284613956565b5f61191d8284612d26565b5f8183611913565b8181101561034a575f61292782602061392c565b9050828111156129345750815b612947826129428184613956565b612d3e565b5061295360208261392c565b9050612913565b5f806010831661296c57610140612970565b6101605b9093209392505050565b60606080604051019050602081016040525f8152805f19835b928101926030600a8206018453600a9004806129935750508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0909101908152919050565b600f811651948201946001860153600f8160041c1651855360081c8483036129da578015612a0f57632194895a5f526004601cfd5b5050508190037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe090910190815292915050565b5f808080612a6b603c8087078313908705035b612a60906001613b62565b600281900b60081d91565b9092509050612a9b81612a9573ffffffffffffffffffffffffffffffffffffffff8a168986612ee8565b90612f71565b9094509050612aaa8282613033565b92505050935093915050565b5f808080612ad2612a606001603c808907851390890503613ba3565b9092509050612a9b81612afc73ffffffffffffffffffffffffffffffffffffffff8a168986612ee8565b9061305e565b5f61191d82612b19670de0b6b3a764000086613a73565b0490565b5f611a0373ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484613125565b808203608081901c156107495763c9654ed45f526004601cfd5b5f808080612b9e612a60612b8e600188613ba3565b5f603c8083079190911291050390565b91509150612bb081612afc88856131fb565b9094509050612bbf8282613033565b925050509250929050565b808214611633576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b818101608081901c156107495763c9654ed45f526004601cfd5b5f808080612c34603c808707831390870503612a55565b91509150612bb081612a9588856131fb565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526003820160048201529091505f9073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa158015612cc5573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612ce9919061393f565b95945050505050565b6132de80610f0b848463ffffffff8416565b5f6b033b2e3c9fd0803ce8000000612d1c8385613a73565b61191d9190613996565b5f81612d1c6b033b2e3c9fd0803ce800000085613a73565b60608210612e59577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa0820180517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0840180517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe086018051630be77f569095526020909252908490529091612de6612dd6604487613956565b612de186604461392c565b612cf2565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa08501929092527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc08401527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08301525050565b5f808080612e67858761392c565b805160208201516040830151919650945092509050612e9186612e8b81606061392c565b876132f5565b630be77f56865260208087015260408601859052612ebe612eb387601c61392c565b612de187604461392c565b612ed3612ecc87606061392c565b87876132f5565b92835260208301919091526040909101525050565b5f828152600660209081526040808320848452600501909152812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff861690631e2eaeaf90602401602060405180830381865afa158015612cc5573d5f803e3d5ffd5b5f805f61300c8460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826130225760ff613029565b8360ff1681015b9150509250929050565b5f603c60ff831661304a600186900b610100613be4565b6130549190613b62565b61191d9190613be4565b5f805f8360ff0390505f6130ff8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b9050806101001415935083613114575f612bbf565b8160ff168103925050509250929050565b5f8281526006602090815260408083208484526004019091528120819081906040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290529091505f9073ffffffffffffffffffffffffffffffffffffffff881690631e2eaeaf90602401602060405180830381865afa1580156131b2573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906131d6919061393f565b6fffffffffffffffffffffffffffffffff81169860809190911d975095505050505050565b5f61191d73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168484612ee8565b8860ff1681511015613270578060405160200161325a9190613af6565b604051602081830303815290604052905061323d565b5f8a1261328b5760405180602001604052805f8152506132c2565b6040518060400160405280600181526020017f2d000000000000000000000000000000000000000000000000000000000000008152505b6132cb84612363565b8260405160200161242993929190613c0a565b5f8082846a636f6e736f6c652e6c6f675afa505050565b61330880611aa785858563ffffffff8516565b8082828560045afa50505050565b5f8083601f840112613326575f80fd5b50813567ffffffffffffffff81111561333d575f80fd5b602083019150836020828501011115610f61575f80fd5b5f8060208385031215613365575f80fd5b823567ffffffffffffffff81111561337b575f80fd5b61338785828601613316565b90969095509350505050565b5f602082840312156133a3575f80fd5b813567ffffffffffffffff8116811461191d575f80fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610359575f80fd5b5f60a082840312156133eb575f80fd5b50919050565b5f608082840312156133eb575f80fd5b5f805f805f6101608688031215613416575f80fd5b8535613421816133ba565b945061343087602088016133db565b935061343f8760c088016133f1565b925061014086013567ffffffffffffffff81111561345b575f80fd5b61346788828901613316565b969995985093965092949392505050565b5f60208284031215613488575f80fd5b81356bffffffffffffffffffffffff8116811461191d575f80fd5b5f805f805f61010086880312156134b8575f80fd5b85356134c3816133ba565b94506134d287602088016133db565b935060c08601356134e2816133ba565b925060e086013567ffffffffffffffff81111561345b575f80fd5b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f61358360e08301896134fd565b828103604084015261359581896134fd565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b818110156135f75783518352602093840193909201916001016135d9565b50909b9a5050505050505050505050565b5f805f805f80610180878903121561361e575f80fd5b8635613629816133ba565b955061363888602089016133db565b94506136478860c089016133f1565b9350610140870135925061016087013567ffffffffffffffff81111561366b575f80fd5b61367789828a01613316565b979a9699509497509295939492505050565b602081525f61191d60208301846134fd565b5f80602083850312156136ac575f80fd5b823567ffffffffffffffff8111156136c2575f80fd5b8301601f810185136136d2575f80fd5b803567ffffffffffffffff8111156136e8575f80fd5b8560208260051b84010111156136fc575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f60208284031215613795575f80fd5b815167ffffffffffffffff8111156137ab575f80fd5b8201601f810184136137bb575f80fd5b805167ffffffffffffffff8111156137d5576137d5613758565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff8211171561384157613841613758565b604052818152828201602001861015613858575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215613885575f80fd5b81358060020b811461191d575f80fd5b5f602082840312156138a5575f80fd5b813562ffffff8116811461191d575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f602082840312156138f4575f80fd5b813561191d816133ba565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b80820180821115610749576107496138ff565b5f6020828403121561394f575f80fd5b5051919050565b81810381811115610749576107496138ff565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f826139a4576139a4613969565b500490565b73ffffffffffffffffffffffffffffffffffffffff835116815273ffffffffffffffffffffffffffffffffffffffff602084015116602082015262ffffff6040840151166040820152606083015160020b606082015273ffffffffffffffffffffffffffffffffffffffff6080840151166080820152613a5860a08201838051151582526020808201519083015260409081015173ffffffffffffffffffffffffffffffffffffffff16910152565b6101206101008201525f611a0361012083015f815260200190565b8082028115828204841417610749576107496138ff565b5f81518060208401855e5f93019283525090919050565b5f611a03613aaf8386613a8a565b84613a8a565b9695505050505050565b6001841115611bd257808504811115613ada57613ada6138ff565b6001841615613ae857908102905b60019390931c928002613abf565b7f300000000000000000000000000000000000000000000000000000000000000081525f61191d6001830184613a8a565b5f613b328285613a8a565b7f2e000000000000000000000000000000000000000000000000000000000000008152612ce96001820185613a8a565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715610749576107496138ff565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715610749576107496138ff565b5f8260020b8260020b028060020b9150808214613c0357613c036138ff565b5092915050565b5f613c1e613c188387613a8a565b85613a8a565b7f2e000000000000000000000000000000000000000000000000000000000000008152613ab56001820185613a8a56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xC4W_5`\xE0\x1C\x80cv\x0F_'\x11a\0}W\x80c\x91\xDDsF\x11a\0XW\x80c\x91\xDDsF\x14a\x01\xFAW\x80c\x97\x12[\xEE\x14a\x02\x1AW\x80c\xC6\xA9\x8E\xB9\x14a\x02[W_\x80\xFD[\x80cv\x0F_'\x14a\x01_W\x80c\x84\xB0\x19n\x14a\x01\x8CW\x80c\x8D\xB2\xB6R\x14a\x01\xA7W_\x80\xFD[\x80c%\x99\x82\xE5\x11a\0\xADW\x80c%\x99\x82\xE5\x14a\0\xF0W\x80c*c0\xCF\x14a\x019W\x80c4@\xD8 \x14a\x01LW_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\0\xC8W\x80c\x11jUP\x14a\0\xDDW[_\x80\xFD[a\0\xDBa\0\xD66`\x04a3TV[a\x02nV[\0[a\0\xDBa\0\xEB6`\x04a3\x93V[a\x03OV[a\x01\x03a\0\xFE6`\x04a4\x01V[a\x03\\V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\x01G6`\x04a4xV[a\x03\xF7V[a\x01\x03a\x01Z6`\x04a4\xA3V[a\x04\xB1V[`\x07Ta\x01s\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x010V[a\x01\x94a\x05\xB9V[`@Qa\x010\x97\x96\x95\x94\x93\x92\x91\x90a5IV[a\x01\xC2a\x01\xB56`\x04a6\x08V[_\x80\x96P\x96\x94PPPPPV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x93\x16\x83R` \x83\x01\x91\x90\x91R\x01a\x010V[a\x02\ra\x02\x086`\x04a3TV[a\x06aV[`@Qa\x010\x91\x90a6\x89V[`\x07Ta\x02>\x90h\x01\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x010V[a\0\xDBa\x02i6`\x04a6\x9BV[a\x07OV[a\x02va\x08JV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x02\xEA\x90\x85\x90\x85\x90`\x04\x01a7\x0CV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03\x05W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03J\x91\x90\x81\x01\x90a7\x85V[PPPV[a\x03Y3\x82a\t\x1DV[PV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03\xCCW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04fW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x07\x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16h\x01\0\0\0\0\0\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x05!W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`<a\x053`\x80\x87\x01``\x88\x01a8uV[`\x02\x0B\x14\x15\x80a\x05WWP_a\x05O``\x87\x01`@\x88\x01a8\x95V[b\xFF\xFF\xFF\x16\x14\x15[\x15a\x05\x8EW`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x95\x94PPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x06O`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xD2W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_a\x06\xDD\x82a\tXV[\x90\x92P\x90P_a\x06\xEC\x83a\t\x7FV[\x90\x93P\x90Pa\x06\xFA\x82a\t\x9CV[a\x07\x06\x83`\x04\x84a\n\xD4V[\x92Pa\x07\x12\x83\x83a\x0B\x08V[\x92Pa\x07\x1F\x83\x83\x83a\x0B\xB2V[\x92Pa\x07*\x82a\x0CMV[a\x075\x83\x87\x87a\x0E\xF4V[PP`@\x80Q_\x81R` \x81\x01\x90\x91R\x91PP[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xBEW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x03JW_\x83\x83\x83\x81\x81\x10a\x07\xDBWa\x07\xDBa8\xB7V[\x90P` \x02\x01` \x81\x01\x90a\x07\xF0\x91\x90a8\xE4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x06` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x07\xC0V[`\x07TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\x08\x91W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x06` R`@\x90 T`\xFF\x16a\x08\xD9W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08\xE2Ca\x0F\x11V[`\x07\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\tPWc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_\x80_a\tf\x84`Da\x0F.V[\x90\x94P\x90P\x83a\tu\x82a\x0FhV[\x92P\x92PP\x91P\x91V[_\x80_a\t\x8D\x84`$a\x0F.V[\x90\x94P\x90P\x83a\tu\x82a\x10\x04V[_a\t\xA6\x82a\x10\x9EV[\x90P_[\x81\x81\x10\x15a\x03JW_a\t\xBD\x84\x83a\x10\xA9V[\x90P_a\t\xC9\x82a\x10\xC4V[\x90P_a\t\xD5\x83a\x10\xECV[\x90P\x81\x15a\n\xBDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0B\r\x9C\ta\n6\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\n\xA6W_\x80\xFD[PZ\xF1\x15\x80\x15a\n\xB8W=_\x80>=_\xFD[PPPP[a\n\xC9`\x04\x82\x84a\x10\xF7V[PPP`\x01\x01a\t\xAAV[`\x03\x83\x81\x01\x93_\x91\x815`\xE8\x1C\x90\x91\x01\x01[\x80\x85\x14a\n\xFFWa\n\xF8\x85\x85\x85a\x110V[\x94Pa\n\xE6V[P\x92\x93\x92PPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x0B\"a\x14@V[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x7F\xD2\x9C\xD8\xAD\x13\x068\xCED\xEA\x18VH\xD2q#E\xE8\xE4V\xFE\xF8\x153(\xEE@\x91m\xF2y\x0E\x81R\x90\x91P[\x82\x86\x14a\x0B\xA8Wa\x0B\xA1\x86\x82\x84\x88a\x14\x8AV[\x95Pa\x0B\x8EV[P\x93\x94\x93PPPPV[_\x80a\x0B\xBCa\x14@V[`@\x80Qa\x01`\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91R`\x03\x87\x81\x01\x97\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x87\x14a\x0CBWa\x0C;\x87\x83\x85\x89\x89a\x167V[\x96Pa\x0C'V[P\x94\x95\x94PPPPPV[_a\x0CW\x82a\x10\x9EV[\x90P_[\x81\x81\x10\x15a\x03JW_a\x0Cn\x84\x83a\x10\xA9V[\x90P_a\x0Cz\x82a\x10\xECV[\x90P_a\x0C\x86\x83a\x18AV[\x90P_a\x0C\x92\x84a\x18MV[\x90P_a\x0C\xAC\x84a\x0C\xA3\x84\x86a9,V[`\x04\x91\x90a\x18YV[\x12\x15a\r\x01W`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x03` R`@\x81 \x80T\x84\x92\x90a\r5\x90\x84\x90a9,V[\x90\x91UPP\x80\x15a\x0E\xE4W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\r\x99\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\xFCW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\x0EW=_\x80>=_\xFD[Pa\x0ET\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x18\x9CV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0E\xBEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xE2\x91\x90a9?V[P[PP`\x01\x90\x92\x01\x91Pa\x0C[\x90PV[\x80\x82\x01\x80\x84\x14a\x0F\x0BWc\x01\x84/\x8C_R`\x04`\x1C\xFD[PPPPV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x0F*Wa\x0F*a\x18\xE5V[P\x90V[`\x03\x82\x81\x01\x92_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81\x84a\x0FK\x87\x84a9VV[a\x0FU\x91\x90a9\x96V[\x91\x93PP` \x1B\x84\x17\x90P[\x92P\x92\x90PV[_\x80a\x0Fs\x83a\x10\x9EV[\x90P_\x80[\x82\x81\x10\x15a\n\xFFW_a\x0F\x93a\x0F\x8E\x87\x84a\x18\xF2V[a\x10\xECV[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0F\xFAW`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x0FxV[_\x80a\x10\x0F\x83a\x10\x9EV[\x90P\x80_\x03a\x10\x1FWP\x90\x91\x90PV[_a\x102a\x10-\x85\x83a\x19$V[a\x19:V[\x90P`\x01[\x82\x81\x10\x15a\n\xFFW_a\x10Ma\x10-\x87\x84a\x19$V[\x90P\x82c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11a\x10\x94W`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x107V[_a\x07I\x82` \x1C\x90V[_`D\x82\x02a\x10\xBC\x84[c\xFF\xFF\xFF\xFF\x16\x90V[\x01\x93\x92PPPV[_a\x10\xD4`\x14\x83[\x015`\x80\x1C\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x815``\x1Ca\x07IV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x0F\x0Ba\x11)\x82\\\x84a\x19EV[\x82\x90a\x19]V[`\x02\x83\x01\x92_\x90\x81\x90\x81\x90\x81\x905`\xF0\x1C\x81a\x11Oa\x0F\x8E\x88\x84a\x18\xF2V[`\x02\x8A\x01\x995`\xF0\x1C\x92P\x90P_a\x11ja\x0F\x8E\x89\x85a\x18\xF2V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x93P\x83a\x11\xA9W\x80\x82a\x11\xACV[\x81\x81[\x90\x96P\x94Pa\x11\xBA\x92PPPV[`@\x80Q`\xA0\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R``\x80\x85\x01\x83\x90R`\x80\x94\x85\x01\x83\x90R\x85Q\x80\x85\x01\x87Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x81\x16\x82R\x89\x16\x92\x81\x01\x92\x90\x92R\x94\x81\x01\x91\x90\x91R`<\x93\x81\x01\x93\x90\x93R0\x83\x83\x01R\x82 `\x10\x8A\x01\x99\x90\x91\x905\x90\x1C\x80\x15a\x14\x07W_a\x12\x86a\x12}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x19dV[`\xA0\x1C`\x02\x0B\x90V[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xF3\xCD\x91L\x85`@Q\x80``\x01`@R\x80\x89\x15\x15\x81R` \x01a\x12\xE1\x87a\x1A\x0BV[\x81R` \x01\x89a\x13\x05Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a\x13\x0CV[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13B\x92\x91\x90a9\xA9V[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13^W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x82\x91\x90a9?V[P_a\x13\xC7a\x12}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x19dV[_\x85\x81R`\x02` R`@\x90 \x90\x91Pa\x14\x04\x90\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85\x85a\x1AlV[PP[_\x82\x81R`\x02` R`@\x81 a\x14 \x90\x8C\x90\x85a\x1A\xAEV[\x90\x9BP\x90Pa\x140\x8A\x88\x83a\x18YV[P\x99\x9A\x99PPPPPPPPPPV[_a\x14\x85a\x14La\x1B\xDAV[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835`\xF8\x1C`\x01\x81\x81\x16\x15\x15``\x86\x01R\x85\x015`\x80\x90\x81\x1C` \x86\x01R`\x11\x86\x015\x90\x1C`@\x85\x01R`#\x85\x01\x94_\x91\x90`!\x015`\xF0\x1Ca\x14\xD0a\x0F\x8E\x85\x83a\x18\xF2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x87\x01R`\x02\x87\x01\x965`\xF0\x1Ca\x15\x01a\x0F\x8E\x86\x83a\x18\xF2V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x88\x01RPP`\x02\x81\x16a\x15-W\x85_a\x157V[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x87\x01R\x95P_a\x15c\x87`\x04\x84\x16\x15a\x1C\xD2V[`\xE0\x89\x01R\x90\x97P\x90P_a\x15\x9Aa\x15\x8E\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01Ra\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90Pa\x15\xA5\x81a\x1D}V[_`\x80\x84\x16a\x15\xBDWa\x15\xB8\x89\x83a\x1D\xCEV[a\x15\xC7V[a\x15\xC7\x89\x83a\x1EFV[\x90\x99P\x90Pa\x15\xD6\x83\x82a\x1E\x8AV[`\x80\x88\x01Q` \x89\x01Qa\x15\xF1\x91\x83\x91`\x01\x88\x16\x15\x15a\x1E\xD2V[_a\x16\x01\x89`\xC0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa\x16%\x81\x8A`\xA0\x01Q\x8B`@\x01Qa\x16 \x89`\xFF\x16`\x01\x16\x15\x15\x90V[a\x1FUV[P\x97\x98\x97PPPPPPPPV[PPV[`\x01\x85\x01\x94_\x905`\xF8\x1Ca\x16L\x86\x82a\x1F\xCDV[`\x01\x81\x16\x15\x15`\x80\x87\x01R_\x80a\x16j\x85\x8A\x88`\x08\x87\x16\x15\x15a \x80V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16`\xC0\x8D\x01R\x91\x16`\xA0\x8B\x01R\x815``\x8B\x01\x81\x90R` \x92\x90\x92\x01\x9AP\x92P\x82\x10\x15\x90Pa\x16\xDCW`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16a\x16\xEBW\x87_a\x16\xF5V[`\x14\x88\x01\x885``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x89\x01R\x97P_a\x17!\x89`\x04\x85\x16\x15a\x1C\xD2V[a\x01\0\x8B\x01R\x90\x99P\x90Pa\x177\x88\x8A\x85a!\x11V[`\x07T\x90\x99P_\x90\x81\x90a\x17l\x90\x8B\x90\x8D\x90\x88\x90\x88\x90h\x01\0\0\0\0\0\0\0\0\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a!UV[\x91\x9CP\x92P\x90P_a\x17\x7F\x8B\x87\x8Ca\"\xA7V[\x90P_`\x80\x87\x16a\x17\x99Wa\x17\x94\x8D\x83a\x1D\xCEV[a\x17\xA3V[a\x17\xA3\x8D\x83a\x1EFV[\x90\x9DP\x90P`\x10\x87\x16\x15a\x17\xDAWa\x17\xC6\x8Ca\x01@\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a\"\xC1V[a\x17\xD5\x81\x8Da\x01 \x01Qa\t\x1DV[a\x17\xE3V[a\x17\xE3\x82a\x1D}V[a\x17\xED\x85\x82a\x1E\x8AV[`\xA0\x8C\x01Qa\x18\x04\x90\x82\x90\x86`\x01\x8B\x16\x15\x15a\x1E\xD2V[_a\x18\x14\x8D`\xE0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa\x18/\x81\x8E`\xC0\x01Q\x86a\x16 \x8C`\xFF\x16`\x01\x16\x15\x15\x90V[P\x9B\x9C\x9BPPPPPPPPPPPPV[_a\x10\xD4`$\x83a\x10\xCCV[_a\x10\xD4`4\x83a\x10\xCCV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a\x18\x94a\x18\x8B\x82\\\x85a\"\xFBV[\x92P\x81\x83a\x19]V[P\x93\x92PPPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x18\xDCWc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[_a\x18\xFF\x82\x84[\x90a#\x13V[a\x19\n`D\x83a:sV[a\x19\x13\x84a\x10\xB3V[a\x19\x1D\x91\x90a9,V[\x93\x92PPPV[_a\x19/\x82\x84a\x18\xF9V[a\x19\n`$\x83a:sV[_\x815`\xE0\x1Ca\x07IV[\x81\x81\x01\x82\x81\x12\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\xDFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\x03\x91\x90a9?V[\x94\x93PPPPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x1AfW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[\x81`\x02\x0B\x81`\x02\x0B\x13\x15a\x1A\x8CWa\x1A\x87\x85\x84\x86\x85\x85a$GV[a\x1A\xA7V[\x81`\x02\x0B\x81`\x02\x0B\x12\x15a\x1A\xA7Wa\x1A\xA7\x85\x84\x86\x85\x85a$\xDBV[PPPPPV[`\x01\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x915\x82\x1A\x16\x15\x15`\x03\x88\x01\x975`\xE8\x1D_a\x1A\xD5\x88a%gV[`\x13\x80\x8C\x01\x9B\x91\x92P\x815`\x80\x1C\x91`\x10\x81\x015`\xE8\x1C\x01\x01\x84a\x1B\x07Wa\x1B\x02\x8B\x84\x8E\x87\x8E\x87\x87a%\xABV[a\x1B\x16V[a\x1B\x16\x8B\x84\x8E\x87\x8E\x87\x87a&\xA0V[\x92\x9EP\x90\x99P\x97P\x95Pa\x1B+\x94PPPPPV[_a\x1B5\x86a'\x86V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a\x1B\xB0W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x0C\xF8V[\x82\x87c\x01\0\0\0\x01_\x82\x82Ta\x1B\xC6\x91\x90a9,V[\x90\x91UP\x88\x95PPPPP[\x93P\x93\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a\x1C\xCFWP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a\x1DsW\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a\x1D\xC3W`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x163\x81`\x01a\x19]V[`\x14\x82\x01\x91_\x905``\x1C6\x82a\x1D\xF2\x86\x805`\xE8\x1C\x80\x82\x01`\x03\x90\x81\x01\x93\x92\x01\x91V[\x91\x97P\x92P\x90Pa\x1E\x05\x83\x86\x84\x84a'\xC7V[a\x1E;W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x93\x94\x90\x93P\x91PPV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a\x1E\x83Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81\x15a\x163Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x03JWc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x81a\x1E\xDF`\x04\x85\x83a\x10\xF7V[\x81\x15a\x1F3Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x1F(\x90\x84\x90a9VV[\x90\x91UPa\x1A\xA7\x90PV[a\x1A\xA7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a(\x0CV[\x81a\x1Fb`\x04\x85\x83a\x18YV[P\x81\x15a\x1F\xACWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x1F(\x90\x84\x90a9,V[a\x1A\xA7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x18\x9CV[` \x81\x16\x15a +W`\x10\x81\x16\x15a \x05WP\x7F\xA9\xE5)MDO\xBA\xEB\\\xA0_+$\xC5\xD8)D\x10\xF3\x14\x13\x04\x8ED]\x88\x1E.\xF6\x9E`\0\x90RV[P\x7F\xEB\xD5\t\x1C9oy\x05nEE_J\x93\xBB\xB0\x16\xC2\x171K\xB25k\"\xD1\xC183\xAC\x88a\x90RV[`\x10\x81\x16\x15a ZWP\x7F\xEF\x0C\xCE\x88\xFD\xA0J\xB3\xF8F\x18\x823r\xCFv\xF6KE\x11\xCE\x8Cyc\x0E\xAB\xC2\x9E\xBC\x9B\x96\x8F\x90RV[P\x7F[\x9DI\xCF\xEDH\xF8\xC9\xD1\xA8c\xF6\x1C$y\xC5y\xFA)\x9C%\xA32\x11\xFC\x85s\x90\xCE\xCC\xE6\xD4\x90RV[`\x02\x83\x01\x92_\x90\x81\x90\x81\x90\x81\x905`\xF0\x1C\x81a \x9C\x8A\x83a\x19$V[\x90P_a \xA8\x82a(dV[\x90P_a \xC5a\x0F\x8Ea \xBA\x85a(oV[\x8C\x90a\xFF\xFF\x16a\x18\xF2V[\x90P_a \xE2a\x0F\x8Ea \xD7\x86a(~V[\x8D\x90a\xFF\xFF\x16a\x18\xF2V[\x90P\x89a \xF1W\x80\x82\x84a \xFCV[\x81\x81a \xFC\x85a(\x8AV[\x9D\x9F\x91\x9EP\x9C\x9BP\x99PPPPPPPPPPV[_`\x10\x82\x16\x15a!;W`\x08\x83a\x018\x86\x017`\x05`\x08\x84\x01a\x01[\x86\x017`\r\x83\x01\x92Pa!MV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01 \x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x87\x16\x15a\"\tWP\x865`\x80\x90\x81\x1C` \x8A\x81\x01\x82\x90R`\x10\x8A\x015\x83\x1C`@\x8C\x01\x81\x90R`0\x8B\x01\x9A\x91\x90\x91\x015\x90\x92\x1C\x91\x81\x83\x10\x15a!\xC8W`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a\"\x02W`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa\"4V[P`\x10\x87\x01\x965`\x80\x1C`@\x87\x16a\"!W_a\"$V[`\x01[`\xFF\x16` \x8A\x01R`@\x89\x01\x81\x90R[`@\x87\x16\x15\x15\x80a\"GWP` \x87\x16\x15\x15[\x15a\"uW\x91P\x81a\"Y\x86\x84a(\xDDV[\x91Pa\"n\x82a\"i\x81\x88a(\xEAV[a(\xF5V[\x91Pa\"\x9AV[\x90P\x80a\"\x82\x86\x83a)\0V[\x92Pa\"\x97\x83a\"\x92\x81\x88a(\xEAV[a)\x0BV[\x92P[P\x95\x97\x90\x96P\x93PPPPV[_a\x1A\x03a\"\xB5\x85\x85a)ZV[`\"\x84\x01R`B\x83 \x90V[\x80B\x11\x15a\x03YW`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[_a#\x1E\x83` \x1C\x90V[\x90P\x80\x82\x10a\x03JW`@Q\x7F\xBC_\x99|\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x0C\xF8V[``a\x07I\x82a)zV[\x80`\x01\x16`\x01\x03a#\x9EW\x81\x87`@Q` \x01a#\x8C\x92\x91\x90a:\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96P[`\x01\x81\x90\x1C\x90P\x81\x82`@Q` \x01a#\xB8\x92\x91\x90a:\xA1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91P_\x81\x11a#nWP\x94\x95\x94PPPPPV[\x88`\xFF\x16\x81Q\x10\x15a$\x0EW\x80`@Q` \x01a#\xF8\x91\x90a:\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa#\xDBV[a$\x17\x83a#cV[\x81`@Q` \x01a$)\x92\x91\x90a;'V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x96PPPPPPP\x93\x92PPPV[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x12\x15a$\xD3W_a$\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a*BV[\x94P\x90P\x80\x15a$\xCDW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a$\xA4Wa$\xA4a8\xB7V[\x01Ta$\xB0\x90\x83a9VV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a$\xCAWa$\xCAa8\xB7V[\x01U[Pa$PV[PPPPPPV[c\x01\0\0\0\x85\x01T[\x81`\x02\x0B\x83`\x02\x0B\x13\x15a$\xD3W_a%\x14s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86a*\xB6V[\x94P\x90P\x80\x15a%aW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a%8Wa%8a8\xB7V[\x01Ta%D\x90\x83a9VV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a%^Wa%^a8\xB7V[\x01U[Pa$\xE4V[_a\x07Ia\x12}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84a\x19dV[_\x80\x80\x80`\x01_\x80[\x82\x15a&_W_\x88\x8D\x14a%\xCDWP`\x10\x8C\x01\x9B5`\x80\x1C[a%\xD7\x81\x84a9,V[\x92Pa%\xF5\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a+\x02V[a%\xFF\x90\x83a9,V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13a&\x14WPa&\x81V[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a&/Wa&/a8\xB7V[\x01_\x82\x82Ta&>\x91\x90a9,V[\x90\x91UPa&W\x90P\x8Aa&R\x8D\x8Fa+\x1DV[a+_V[\x99PPa&nV[\x8C`\x02\x0B\x8B`\x02\x0B\x13\x15a&\x81W[a&x\x8A\x8Ca+yV[\x9BP\x92Pa%\xB4V[a&\x8B\x8C\x89a+\xCAV[\x9A\x9D\x90\x9CP\x99\x9AP\x95\x98\x97PPPPPPPPV[_\x80\x80\x80`\x01_\x80[\x82\x15a'VW`\x10\x8C\x01\x9B5`\x80\x1Ca&\xC2\x81\x84a9,V[\x92Pa&\xE0\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a+\x02V[a&\xEA\x90\x83a9,V[\x91P\x8D`\x02\x0B\x8C`\x02\x0B\x13\x15a'\0WPa'wV[\x81\x8F\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a'\x1BWa'\x1Ba8\xB7V[\x01_\x82\x82Ta'*\x91\x90a9,V[\x90\x91UP_\x90Pa';\x8C\x8Ea+\x1DV[\x90Pa'G\x8B\x82a,\x03V[\x9APa'P\x90PV[Pa'dV[\x8C`\x02\x0B\x8B`\x02\x0B\x13a'wW[a'n\x8A\x8Ca,\x1DV[\x9BP\x92Pa&\xA9V[a'\x81\x8C\x89a+\xCAV[a&\x8BV[_a\x07Is\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x83a,FV[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a(WWcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[_`\x04\x82\x015a\x07IV[_a\x07I\x81\x83[\x015`\xF0\x1C\x90V[_a\x07I`\x02\x83a(vV[_a\x07I\x82v\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0a9\x96V[` \x81\x10\x15a(\xC5W\x82\x81\x1A\x15a(\xC5W`\x01\x01a(\xACV[\x80\x82R`\x03\x1Ba\x01\0\x03\x91\x82\x1C\x90\x91\x1B` \x90\x91\x01RV[_a\x19\x1D\x83\x83[\x90a-\x04V[_a\x19\x1D\x82\x84a(\xE4V[_a\x19\x1D\x82\x84a9VV[_a\x19\x1D\x82\x84a-&V[_\x81\x83a\x19\x13V[\x81\x81\x10\x15a\x03JW_a)'\x82` a9,V[\x90P\x82\x81\x11\x15a)4WP\x81[a)G\x82a)B\x81\x84a9VV[a->V[Pa)S` \x82a9,V[\x90Pa)\x13V[_\x80`\x10\x83\x16a)lWa\x01@a)pV[a\x01`[\x90\x93 \x93\x92PPPV[```\x80`@Q\x01\x90P` \x81\x01`@R_\x81R\x80_\x19\x83[\x92\x81\x01\x92`0`\n\x82\x06\x01\x84S`\n\x90\x04\x80a)\x93WPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x91\x90PV[`\x0F\x81\x16Q\x94\x82\x01\x94`\x01\x86\x01S`\x0F\x81`\x04\x1C\x16Q\x85S`\x08\x1C\x84\x83\x03a)\xDAW\x80\x15a*\x0FWc!\x94\x89Z_R`\x04`\x1C\xFD[PPP\x81\x90\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x90\x91\x01\x90\x81R\x92\x91PPV[_\x80\x80\x80a*k`<\x80\x87\x07\x83\x13\x90\x87\x05\x03[a*`\x90`\x01a;bV[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa*\x9B\x81a*\x95s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a.\xE8V[\x90a/qV[\x90\x94P\x90Pa*\xAA\x82\x82a03V[\x92PPP\x93P\x93\x91PPV[_\x80\x80\x80a*\xD2a*``\x01`<\x80\x89\x07\x85\x13\x90\x89\x05\x03a;\xA3V[\x90\x92P\x90Pa*\x9B\x81a*\xFCs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x89\x86a.\xE8V[\x90a0^V[_a\x19\x1D\x82a+\x19g\r\xE0\xB6\xB3\xA7d\0\0\x86a:sV[\x04\x90V[_a\x1A\x03s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a1%V[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a+\x9Ea*`a+\x8E`\x01\x88a;\xA3V[_`<\x80\x83\x07\x91\x90\x91\x12\x91\x05\x03\x90V[\x91P\x91Pa+\xB0\x81a*\xFC\x88\x85a1\xFBV[\x90\x94P\x90Pa+\xBF\x82\x82a03V[\x92PPP\x92P\x92\x90PV[\x80\x82\x14a\x163W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x07IWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a,4`<\x80\x87\x07\x83\x13\x90\x87\x05\x03a*UV[\x91P\x91Pa+\xB0\x81a*\x95\x88\x85a1\xFBV[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x03\x82\x01`\x04\x82\x01R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xE9\x91\x90a9?V[\x95\x94PPPPPV[a2\xDE\x80a\x0F\x0B\x84\x84c\xFF\xFF\xFF\xFF\x84\x16V[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a-\x1C\x83\x85a:sV[a\x19\x1D\x91\x90a9\x96V[_\x81a-\x1Ck\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a:sV[``\x82\x10a.YW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x82\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x01\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x86\x01\x80Qc\x0B\xE7\x7FV\x90\x95R` \x90\x92R\x90\x84\x90R\x90\x91a-\xE6a-\xD6`D\x87a9VV[a-\xE1\x86`Da9,V[a,\xF2V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xA0\x85\x01\x92\x90\x92R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xC0\x84\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x83\x01RPPV[_\x80\x80\x80a.g\x85\x87a9,V[\x80Q` \x82\x01Q`@\x83\x01Q\x91\x96P\x94P\x92P\x90Pa.\x91\x86a.\x8B\x81``a9,V[\x87a2\xF5V[c\x0B\xE7\x7FV\x86R` \x80\x87\x01R`@\x86\x01\x85\x90Ra.\xBEa.\xB3\x87`\x1Ca9,V[a-\xE1\x87`Da9,V[a.\xD3a.\xCC\x87``a9,V[\x87\x87a2\xF5V[\x92\x83R` \x83\x01\x91\x90\x91R`@\x90\x91\x01RPPV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xC5W=_\x80>=_\xFD[_\x80_a0\x0C\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a0\"W`\xFFa0)V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_`<`\xFF\x83\x16a0J`\x01\x86\x90\x0Ba\x01\0a;\xE4V[a0T\x91\x90a;bV[a\x19\x1D\x91\x90a;\xE4V[_\x80_\x83`\xFF\x03\x90P_a0\xFF\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a1\x14W_a+\xBFV[\x81`\xFF\x16\x81\x03\x92PPP\x92P\x92\x90PV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x04\x01\x90\x91R\x81 \x81\x90\x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xB2W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a1\xD6\x91\x90a9?V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\x80\x91\x90\x91\x1D\x97P\x95PPPPPPV[_a\x19\x1Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a.\xE8V[\x88`\xFF\x16\x81Q\x10\x15a2pW\x80`@Q` \x01a2Z\x91\x90a:\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa2=V[_\x8A\x12a2\x8BW`@Q\x80` \x01`@R\x80_\x81RPa2\xC2V[`@Q\x80`@\x01`@R\x80`\x01\x81R` \x01\x7F-\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP[a2\xCB\x84a#cV[\x82`@Q` \x01a$)\x93\x92\x91\x90a<\nV[_\x80\x82\x84jconsole.logZ\xFAPPPV[a3\x08\x80a\x1A\xA7\x85\x85\x85c\xFF\xFF\xFF\xFF\x85\x16V[\x80\x82\x82\x85`\x04Z\xFAPPPPV[_\x80\x83`\x1F\x84\x01\x12a3&W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3=W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0FaW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a3eW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3{W_\x80\xFD[a3\x87\x85\x82\x86\x01a3\x16V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15a3\xA3W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x19\x1DW_\x80\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03YW_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a3\xEBW_\x80\xFD[P\x91\x90PV[_`\x80\x82\x84\x03\x12\x15a3\xEBW_\x80\xFD[_\x80_\x80_a\x01`\x86\x88\x03\x12\x15a4\x16W_\x80\xFD[\x855a4!\x81a3\xBAV[\x94Pa40\x87` \x88\x01a3\xDBV[\x93Pa4?\x87`\xC0\x88\x01a3\xF1V[\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4[W_\x80\xFD[a4g\x88\x82\x89\x01a3\x16V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a4\x88W_\x80\xFD[\x815k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x19\x1DW_\x80\xFD[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a4\xB8W_\x80\xFD[\x855a4\xC3\x81a3\xBAV[\x94Pa4\xD2\x87` \x88\x01a3\xDBV[\x93P`\xC0\x86\x015a4\xE2\x81a3\xBAV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4[W_\x80\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_a5\x83`\xE0\x83\x01\x89a4\xFDV[\x82\x81\x03`@\x84\x01Ra5\x95\x81\x89a4\xFDV[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a5\xF7W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a5\xD9V[P\x90\x9B\x9APPPPPPPPPPPV[_\x80_\x80_\x80a\x01\x80\x87\x89\x03\x12\x15a6\x1EW_\x80\xFD[\x865a6)\x81a3\xBAV[\x95Pa68\x88` \x89\x01a3\xDBV[\x94Pa6G\x88`\xC0\x89\x01a3\xF1V[\x93Pa\x01@\x87\x015\x92Pa\x01`\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6kW_\x80\xFD[a6w\x89\x82\x8A\x01a3\x16V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[` \x81R_a\x19\x1D` \x83\x01\x84a4\xFDV[_\x80` \x83\x85\x03\x12\x15a6\xACW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xC2W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a6\xD2W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\xE8W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a6\xFCW_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a7\x95W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xABW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a7\xBBW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\xD5Wa7\xD5a7XV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a8AWa8Aa7XV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a8XW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a8\x85W_\x80\xFD[\x815\x80`\x02\x0B\x81\x14a\x19\x1DW_\x80\xFD[_` \x82\x84\x03\x12\x15a8\xA5W_\x80\xFD[\x815b\xFF\xFF\xFF\x81\x16\x81\x14a\x19\x1DW_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a8\xF4W_\x80\xFD[\x815a\x19\x1D\x81a3\xBAV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07IWa\x07Ia8\xFFV[_` \x82\x84\x03\x12\x15a9OW_\x80\xFD[PQ\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x07IWa\x07Ia8\xFFV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82a9\xA4Wa9\xA4a9iV[P\x04\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83Q\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x84\x01Q\x16` \x82\x01Rb\xFF\xFF\xFF`@\x84\x01Q\x16`@\x82\x01R``\x83\x01Q`\x02\x0B``\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x84\x01Q\x16`\x80\x82\x01Ra:X`\xA0\x82\x01\x83\x80Q\x15\x15\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x01RV[a\x01 a\x01\0\x82\x01R_a\x1A\x03a\x01 \x83\x01_\x81R` \x01\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07IWa\x07Ia8\xFFV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a\x1A\x03a:\xAF\x83\x86a:\x8AV[\x84a:\x8AV[\x96\x95PPPPPPV[`\x01\x84\x11\x15a\x1B\xD2W\x80\x85\x04\x81\x11\x15a:\xDAWa:\xDAa8\xFFV[`\x01\x84\x16\x15a:\xE8W\x90\x81\x02\x90[`\x01\x93\x90\x93\x1C\x92\x80\x02a:\xBFV[\x7F0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R_a\x19\x1D`\x01\x83\x01\x84a:\x8AV[_a;2\x82\x85a:\x8AV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra,\xE9`\x01\x82\x01\x85a:\x8AV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x07IWa\x07Ia8\xFFV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x07IWa\x07Ia8\xFFV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a<\x03Wa<\x03a8\xFFV[P\x92\x91PPV[_a<\x1Ea<\x18\x83\x87a:\x8AV[\x85a:\x8AV[\x7F.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ra:\xB5`\x01\x82\x01\x85a:\x8AV\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Custom error with signature `Expired()` and selector `0x203d82d8`.
```solidity
error Expired();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Expired {}
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
        impl ::core::convert::From<Expired> for UnderlyingRustTuple<'_> {
            fn from(value: Expired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Expired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Expired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Expired()";
            const SELECTOR: [u8; 4] = [32u8, 61u8, 130u8, 216u8];
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
    /**Custom error with signature `FillingTooLittle()` and selector `0xc4daf003`.
```solidity
error FillingTooLittle();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FillingTooLittle {}
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
        impl ::core::convert::From<FillingTooLittle> for UnderlyingRustTuple<'_> {
            fn from(value: FillingTooLittle) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FillingTooLittle {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FillingTooLittle {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FillingTooLittle()";
            const SELECTOR: [u8; 4] = [196u8, 218u8, 240u8, 3u8];
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
    /**Custom error with signature `FillingTooMuch()` and selector `0x44182331`.
```solidity
error FillingTooMuch();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct FillingTooMuch {}
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
        impl ::core::convert::From<FillingTooMuch> for UnderlyingRustTuple<'_> {
            fn from(value: FillingTooMuch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FillingTooMuch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for FillingTooMuch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "FillingTooMuch()";
            const SELECTOR: [u8; 4] = [68u8, 24u8, 35u8, 49u8];
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
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
    /**Custom error with signature `LimitViolated()` and selector `0x8e1edfa4`.
```solidity
error LimitViolated();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct LimitViolated {}
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
        impl ::core::convert::From<LimitViolated> for UnderlyingRustTuple<'_> {
            fn from(value: LimitViolated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LimitViolated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LimitViolated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LimitViolated()";
            const SELECTOR: [u8; 4] = [142u8, 30u8, 223u8, 164u8];
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
    /**Custom error with signature `NonceReuse()` and selector `0x8cb88872`.
```solidity
error NonceReuse();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NonceReuse {}
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
        impl ::core::convert::From<NonceReuse> for UnderlyingRustTuple<'_> {
            fn from(value: NonceReuse) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonceReuse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonceReuse {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonceReuse()";
            const SELECTOR: [u8; 4] = [140u8, 184u8, 136u8, 114u8];
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
    /**Custom error with signature `OrderAlreadyExecuted()` and selector `0x8a2ef116`.
```solidity
error OrderAlreadyExecuted();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct OrderAlreadyExecuted {}
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
        impl ::core::convert::From<OrderAlreadyExecuted> for UnderlyingRustTuple<'_> {
            fn from(value: OrderAlreadyExecuted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OrderAlreadyExecuted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OrderAlreadyExecuted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OrderAlreadyExecuted()";
            const SELECTOR: [u8; 4] = [138u8, 46u8, 241u8, 22u8];
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
constructor(address uniV4PoolManager, address governance);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub uniV4PoolManager: alloy::sol_types::private::Address,
        pub governance: alloy::sol_types::private::Address,
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
                    (value.uniV4PoolManager, value.governance)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        uniV4PoolManager: tuple.0,
                        governance: tuple.1,
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
                        &self.governance,
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
    /**Function with signature `beforeInitialize(address,(address,address,uint24,int24,address),uint160,bytes)` and selector `0x3440d820`.
```solidity
function beforeInitialize(address, PoolKey memory poolKey, uint160, bytes memory) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct beforeInitializeCall {
        pub _0: alloy::sol_types::private::Address,
        pub poolKey: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub _2: alloy::sol_types::private::primitives::aliases::U160,
        pub _3: alloy::sol_types::private::Bytes,
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
                    (value._0, value.poolKey, value._2, value._3)
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
    /**Function with signature `eip712Domain()` and selector `0x84b0196e`.
```solidity
function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct eip712DomainCall {}
    ///Container type for the return parameters of the [`eip712Domain()`](eip712DomainCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct eip712DomainReturn {
        pub fields: alloy::sol_types::private::FixedBytes<1>,
        pub name: alloy::sol_types::private::String,
        pub version: alloy::sol_types::private::String,
        pub chainId: alloy::sol_types::private::primitives::aliases::U256,
        pub verifyingContract: alloy::sol_types::private::Address,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub extensions: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            impl ::core::convert::From<eip712DomainCall> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<1>,
                alloy::sol_types::private::String,
                alloy::sol_types::private::String,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
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
            impl ::core::convert::From<eip712DomainReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eip712DomainReturn) -> Self {
                    (
                        value.fields,
                        value.name,
                        value.version,
                        value.chainId,
                        value.verifyingContract,
                        value.salt,
                        value.extensions,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eip712DomainReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        fields: tuple.0,
                        name: tuple.1,
                        version: tuple.2,
                        chainId: tuple.3,
                        verifyingContract: tuple.4,
                        salt: tuple.5,
                        extensions: tuple.6,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eip712DomainCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eip712DomainReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<1>,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eip712Domain()";
            const SELECTOR: [u8; 4] = [132u8, 176u8, 25u8, 110u8];
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
    /**Function with signature `execute(bytes)` and selector `0x09c5eabe`.
```solidity
function execute(bytes memory encoded) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct executeCall {
        pub encoded: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`execute(bytes)`](executeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct executeReturn {}
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
            impl ::core::convert::From<executeCall> for UnderlyingRustTuple<'_> {
                fn from(value: executeCall) -> Self {
                    (value.encoded,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeCall {
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
            impl ::core::convert::From<executeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: executeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for executeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for executeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = executeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "execute(bytes)";
            const SELECTOR: [u8; 4] = [9u8, 197u8, 234u8, 190u8];
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
    /**Function with signature `govToggleNodes(address[])` and selector `0xc6a98eb9`.
```solidity
function govToggleNodes(address[] memory nodes) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct govToggleNodesCall {
        pub nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`govToggleNodes(address[])`](govToggleNodesCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct govToggleNodesReturn {}
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
            impl ::core::convert::From<govToggleNodesCall> for UnderlyingRustTuple<'_> {
                fn from(value: govToggleNodesCall) -> Self {
                    (value.nodes,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for govToggleNodesCall {
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
            impl ::core::convert::From<govToggleNodesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: govToggleNodesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for govToggleNodesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for govToggleNodesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = govToggleNodesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "govToggleNodes(address[])";
            const SELECTOR: [u8; 4] = [198u8, 169u8, 142u8, 185u8];
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
    /**Function with signature `govUpdateHalfSpread(uint96)` and selector `0x2a6330cf`.
```solidity
function govUpdateHalfSpread(uint96 newHalfSpreadRay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct govUpdateHalfSpreadCall {
        pub newHalfSpreadRay: alloy::sol_types::private::primitives::aliases::U96,
    }
    ///Container type for the return parameters of the [`govUpdateHalfSpread(uint96)`](govUpdateHalfSpreadCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct govUpdateHalfSpreadReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<govUpdateHalfSpreadCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: govUpdateHalfSpreadCall) -> Self {
                    (value.newHalfSpreadRay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for govUpdateHalfSpreadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newHalfSpreadRay: tuple.0 }
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
            impl ::core::convert::From<govUpdateHalfSpreadReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: govUpdateHalfSpreadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for govUpdateHalfSpreadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for govUpdateHalfSpreadCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = govUpdateHalfSpreadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "govUpdateHalfSpread(uint96)";
            const SELECTOR: [u8; 4] = [42u8, 99u8, 48u8, 207u8];
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.newHalfSpreadRay),
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
    /**Function with signature `halfSpreadRay()` and selector `0x97125bee`.
```solidity
function halfSpreadRay() external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct halfSpreadRayCall {}
    ///Container type for the return parameters of the [`halfSpreadRay()`](halfSpreadRayCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct halfSpreadRayReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<halfSpreadRayCall> for UnderlyingRustTuple<'_> {
                fn from(value: halfSpreadRayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for halfSpreadRayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<halfSpreadRayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: halfSpreadRayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for halfSpreadRayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for halfSpreadRayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = halfSpreadRayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "halfSpreadRay()";
            const SELECTOR: [u8; 4] = [151u8, 18u8, 91u8, 238u8];
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
    /**Function with signature `invalidateNonce(uint64)` and selector `0x116a5550`.
```solidity
function invalidateNonce(uint64 nonce) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct invalidateNonceCall {
        pub nonce: u64,
    }
    ///Container type for the return parameters of the [`invalidateNonce(uint64)`](invalidateNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct invalidateNonceReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<invalidateNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: invalidateNonceCall) -> Self {
                    (value.nonce,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for invalidateNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nonce: tuple.0 }
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
            impl ::core::convert::From<invalidateNonceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: invalidateNonceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for invalidateNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for invalidateNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = invalidateNonceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "invalidateNonce(uint64)";
            const SELECTOR: [u8; 4] = [17u8, 106u8, 85u8, 80u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonce),
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
    /**Function with signature `lastBlockUpdated()` and selector `0x760f5f27`.
```solidity
function lastBlockUpdated() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct lastBlockUpdatedCall {}
    ///Container type for the return parameters of the [`lastBlockUpdated()`](lastBlockUpdatedCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct lastBlockUpdatedReturn {
        pub _0: u64,
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
            impl ::core::convert::From<lastBlockUpdatedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastBlockUpdatedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastBlockUpdatedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
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
            impl ::core::convert::From<lastBlockUpdatedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastBlockUpdatedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastBlockUpdatedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastBlockUpdatedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastBlockUpdatedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastBlockUpdated()";
            const SELECTOR: [u8; 4] = [118u8, 15u8, 95u8, 39u8];
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
    ///Container for all the [`Angstrom`](self) function calls.
    pub enum AngstromCalls {
        afterRemoveLiquidity(afterRemoveLiquidityCall),
        beforeAddLiquidity(beforeAddLiquidityCall),
        beforeInitialize(beforeInitializeCall),
        eip712Domain(eip712DomainCall),
        execute(executeCall),
        govToggleNodes(govToggleNodesCall),
        govUpdateHalfSpread(govUpdateHalfSpreadCall),
        halfSpreadRay(halfSpreadRayCall),
        invalidateNonce(invalidateNonceCall),
        lastBlockUpdated(lastBlockUpdatedCall),
        unlockCallback(unlockCallbackCall),
    }
    #[automatically_derived]
    impl AngstromCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 197u8, 234u8, 190u8],
            [17u8, 106u8, 85u8, 80u8],
            [37u8, 153u8, 130u8, 229u8],
            [42u8, 99u8, 48u8, 207u8],
            [52u8, 64u8, 216u8, 32u8],
            [118u8, 15u8, 95u8, 39u8],
            [132u8, 176u8, 25u8, 110u8],
            [141u8, 178u8, 182u8, 82u8],
            [145u8, 221u8, 115u8, 70u8],
            [151u8, 18u8, 91u8, 238u8],
            [198u8, 169u8, 142u8, 185u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromCalls {
        const NAME: &'static str = "AngstromCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::afterRemoveLiquidity(_) => {
                    <afterRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeAddLiquidity(_) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeInitialize(_) => {
                    <beforeInitializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::govToggleNodes(_) => {
                    <govToggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::govUpdateHalfSpread(_) => {
                    <govUpdateHalfSpreadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::halfSpreadRay(_) => {
                    <halfSpreadRayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::invalidateNonce(_) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastBlockUpdated(_) => {
                    <lastBlockUpdatedCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<AngstromCalls>] = &[
                {
                    fn execute(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <executeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::execute)
                    }
                    execute
                },
                {
                    fn invalidateNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <invalidateNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::invalidateNonce)
                    }
                    invalidateNonce
                },
                {
                    fn beforeAddLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <beforeAddLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::beforeAddLiquidity)
                    }
                    beforeAddLiquidity
                },
                {
                    fn govUpdateHalfSpread(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <govUpdateHalfSpreadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::govUpdateHalfSpread)
                    }
                    govUpdateHalfSpread
                },
                {
                    fn beforeInitialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <beforeInitializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::beforeInitialize)
                    }
                    beforeInitialize
                },
                {
                    fn lastBlockUpdated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <lastBlockUpdatedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::lastBlockUpdated)
                    }
                    lastBlockUpdated
                },
                {
                    fn eip712Domain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <eip712DomainCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::eip712Domain)
                    }
                    eip712Domain
                },
                {
                    fn afterRemoveLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <afterRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::afterRemoveLiquidity)
                    }
                    afterRemoveLiquidity
                },
                {
                    fn unlockCallback(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <unlockCallbackCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::unlockCallback)
                    }
                    unlockCallback
                },
                {
                    fn halfSpreadRay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <halfSpreadRayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::halfSpreadRay)
                    }
                    halfSpreadRay
                },
                {
                    fn govToggleNodes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <govToggleNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::govToggleNodes)
                    }
                    govToggleNodes
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
            println!("UH OH? - DECODE_SHIMS");
           let t =  (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate);
           println!("OK!! - DECODE_SHIMS");
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
                Self::beforeInitialize(inner) => {
                    <beforeInitializeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::govToggleNodes(inner) => {
                    <govToggleNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::govUpdateHalfSpread(inner) => {
                    <govUpdateHalfSpreadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::halfSpreadRay(inner) => {
                    <halfSpreadRayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::lastBlockUpdated(inner) => {
                    <lastBlockUpdatedCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::beforeInitialize(inner) => {
                    <beforeInitializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eip712Domain(inner) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::execute(inner) => {
                    <executeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::govToggleNodes(inner) => {
                    <govToggleNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::govUpdateHalfSpread(inner) => {
                    <govUpdateHalfSpreadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::halfSpreadRay(inner) => {
                    <halfSpreadRayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::lastBlockUpdated(inner) => {
                    <lastBlockUpdatedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`Angstrom`](self) custom errors.
    pub enum AngstromErrors {
        AssetsOutOfOrderOrNotUnique(AssetsOutOfOrderOrNotUnique),
        BundleChangeNetNegative(BundleChangeNetNegative),
        Expired(Expired),
        FillingTooLittle(FillingTooLittle),
        FillingTooMuch(FillingTooMuch),
        InvalidPoolKey(InvalidPoolKey),
        InvalidSignature(InvalidSignature),
        LimitViolated(LimitViolated),
        MissingHookPermissions(MissingHookPermissions),
        NonceReuse(NonceReuse),
        NotController(NotController),
        NotNode(NotNode),
        NotUniswap(NotUniswap),
        OnlyOncePerBlock(OnlyOncePerBlock),
        OrderAlreadyExecuted(OrderAlreadyExecuted),
        OutOfBoundRead(OutOfBoundRead),
        OutOfOrderOrDuplicatePairs(OutOfOrderOrDuplicatePairs),
        Overflow(Overflow),
        ReaderNotAtEnd(ReaderNotAtEnd),
        WrongEndLiquidity(WrongEndLiquidity),
    }
    #[automatically_derived]
    impl AngstromErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [1u8, 132u8, 47u8, 140u8],
            [32u8, 61u8, 130u8, 216u8],
            [35u8, 1u8, 158u8, 103u8],
            [53u8, 39u8, 141u8, 18u8],
            [68u8, 24u8, 35u8, 49u8],
            [92u8, 210u8, 107u8, 104u8],
            [100u8, 41u8, 207u8, 210u8],
            [117u8, 56u8, 50u8, 40u8],
            [128u8, 241u8, 26u8, 207u8],
            [138u8, 46u8, 241u8, 22u8],
            [139u8, 170u8, 87u8, 159u8],
            [140u8, 184u8, 136u8, 114u8],
            [142u8, 30u8, 223u8, 164u8],
            [188u8, 95u8, 153u8, 124u8],
            [194u8, 86u8, 98u8, 43u8],
            [196u8, 218u8, 240u8, 3u8],
            [212u8, 155u8, 112u8, 245u8],
            [216u8, 166u8, 184u8, 155u8],
            [243u8, 95u8, 147u8, 153u8],
            [248u8, 50u8, 134u8, 20u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromErrors {
        const NAME: &'static str = "AngstromErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 20usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AssetsOutOfOrderOrNotUnique(_) => {
                    <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BundleChangeNetNegative(_) => {
                    <BundleChangeNetNegative as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Expired(_) => <Expired as alloy_sol_types::SolError>::SELECTOR,
                Self::FillingTooLittle(_) => {
                    <FillingTooLittle as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FillingTooMuch(_) => {
                    <FillingTooMuch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPoolKey(_) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LimitViolated(_) => {
                    <LimitViolated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MissingHookPermissions(_) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonceReuse(_) => {
                    <NonceReuse as alloy_sol_types::SolError>::SELECTOR
                }
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
                Self::OrderAlreadyExecuted(_) => {
                    <OrderAlreadyExecuted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBoundRead(_) => {
                    <OutOfBoundRead as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfOrderOrDuplicatePairs(_) => {
                    <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<AngstromErrors>] = &[
                {
                    fn ReaderNotAtEnd(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <ReaderNotAtEnd as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::ReaderNotAtEnd)
                    }
                    ReaderNotAtEnd
                },
                {
                    fn Expired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <Expired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::Expired)
                    }
                    Expired
                },
                {
                    fn NotController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotController as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotController)
                    }
                    NotController
                },
                {
                    fn Overflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <Overflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::Overflow)
                    }
                    Overflow
                },
                {
                    fn FillingTooMuch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FillingTooMuch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FillingTooMuch)
                    }
                    FillingTooMuch
                },
                {
                    fn NotNode(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotNode as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotNode)
                    }
                    NotNode
                },
                {
                    fn WrongEndLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <WrongEndLiquidity as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::WrongEndLiquidity)
                    }
                    WrongEndLiquidity
                },
                {
                    fn MissingHookPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <MissingHookPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::MissingHookPermissions)
                    }
                    MissingHookPermissions
                },
                {
                    fn AssetsOutOfOrderOrNotUnique(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <AssetsOutOfOrderOrNotUnique as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::AssetsOutOfOrderOrNotUnique)
                    }
                    AssetsOutOfOrderOrNotUnique
                },
                {
                    fn OrderAlreadyExecuted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OrderAlreadyExecuted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OrderAlreadyExecuted)
                    }
                    OrderAlreadyExecuted
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn NonceReuse(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NonceReuse as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NonceReuse)
                    }
                    NonceReuse
                },
                {
                    fn LimitViolated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <LimitViolated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::LimitViolated)
                    }
                    LimitViolated
                },
                {
                    fn OutOfBoundRead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OutOfBoundRead as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OutOfBoundRead)
                    }
                    OutOfBoundRead
                },
                {
                    fn InvalidPoolKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidPoolKey as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidPoolKey)
                    }
                    InvalidPoolKey
                },
                {
                    fn FillingTooLittle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FillingTooLittle as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FillingTooLittle)
                    }
                    FillingTooLittle
                },
                {
                    fn BundleChangeNetNegative(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <BundleChangeNetNegative as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::BundleChangeNetNegative)
                    }
                    BundleChangeNetNegative
                },
                {
                    fn OnlyOncePerBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OnlyOncePerBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OnlyOncePerBlock)
                    }
                    OnlyOncePerBlock
                },
                {
                    fn OutOfOrderOrDuplicatePairs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <OutOfOrderOrDuplicatePairs as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::OutOfOrderOrDuplicatePairs)
                    }
                    OutOfOrderOrDuplicatePairs
                },
                {
                    fn NotUniswap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotUniswap as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotUniswap)
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
            println!("UH OH? - DECODE_SHIMS 2");
            let t =             (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate);
            println!("OK!! - DECODE_SHIMS 2");
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
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FillingTooLittle(inner) => {
                    <FillingTooLittle as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FillingTooMuch(inner) => {
                    <FillingTooMuch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPoolKey(inner) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LimitViolated(inner) => {
                    <LimitViolated as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MissingHookPermissions(inner) => {
                    <MissingHookPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonceReuse(inner) => {
                    <NonceReuse as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::OrderAlreadyExecuted(inner) => {
                    <OrderAlreadyExecuted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfBoundRead(inner) => {
                    <OutOfBoundRead as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::FillingTooLittle(inner) => {
                    <FillingTooLittle as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::FillingTooMuch(inner) => {
                    <FillingTooMuch as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LimitViolated(inner) => {
                    <LimitViolated as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NonceReuse(inner) => {
                    <NonceReuse as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::OrderAlreadyExecuted(inner) => {
                    <OrderAlreadyExecuted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OutOfBoundRead(inner) => {
                    <OutOfBoundRead as alloy_sol_types::SolError>::abi_encode_raw(
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
    /**Creates a new wrapper around an on-chain [`Angstrom`](self) contract instance.

See the [wrapper's documentation](`AngstromInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AngstromInstance<T, P, N> {
        AngstromInstance::<T, P, N>::new(address, provider)
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
        governance: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AngstromInstance<T, P, N>>,
    > {
        AngstromInstance::<T, P, N>::deploy(provider, uniV4PoolManager, governance)
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
        governance: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AngstromInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, uniV4PoolManager, governance)
    }
    /**A [`Angstrom`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`Angstrom`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AngstromInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AngstromInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AngstromInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AngstromInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`Angstrom`](self) contract instance.

See the [wrapper's documentation](`AngstromInstance`) for more details.*/
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
            governance: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<AngstromInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                uniV4PoolManager,
                governance,
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
            governance: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            uniV4PoolManager,
                            governance,
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
    impl<T, P: ::core::clone::Clone, N> AngstromInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AngstromInstance<T, P, N> {
            AngstromInstance {
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
    > AngstromInstance<T, P, N> {
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
        ///Creates a new call builder for the [`beforeInitialize`] function.
        pub fn beforeInitialize(
            &self,
            _0: alloy::sol_types::private::Address,
            poolKey: <PoolKey as alloy::sol_types::SolType>::RustType,
            _2: alloy::sol_types::private::primitives::aliases::U160,
            _3: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, beforeInitializeCall, N> {
            self.call_builder(
                &beforeInitializeCall {
                    _0,
                    poolKey,
                    _2,
                    _3,
                },
            )
        }
        ///Creates a new call builder for the [`eip712Domain`] function.
        pub fn eip712Domain(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eip712DomainCall, N> {
            self.call_builder(&eip712DomainCall {})
        }
        ///Creates a new call builder for the [`execute`] function.
        pub fn execute(
            &self,
            encoded: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, executeCall, N> {
            self.call_builder(&executeCall { encoded })
        }
        ///Creates a new call builder for the [`govToggleNodes`] function.
        pub fn govToggleNodes(
            &self,
            nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, govToggleNodesCall, N> {
            self.call_builder(&govToggleNodesCall { nodes })
        }
        ///Creates a new call builder for the [`govUpdateHalfSpread`] function.
        pub fn govUpdateHalfSpread(
            &self,
            newHalfSpreadRay: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<T, &P, govUpdateHalfSpreadCall, N> {
            self.call_builder(
                &govUpdateHalfSpreadCall {
                    newHalfSpreadRay,
                },
            )
        }
        ///Creates a new call builder for the [`halfSpreadRay`] function.
        pub fn halfSpreadRay(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, halfSpreadRayCall, N> {
            self.call_builder(&halfSpreadRayCall {})
        }
        ///Creates a new call builder for the [`invalidateNonce`] function.
        pub fn invalidateNonce(
            &self,
            nonce: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, invalidateNonceCall, N> {
            self.call_builder(&invalidateNonceCall { nonce })
        }
        ///Creates a new call builder for the [`lastBlockUpdated`] function.
        pub fn lastBlockUpdated(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastBlockUpdatedCall, N> {
            self.call_builder(&lastBlockUpdatedCall {})
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
    > AngstromInstance<T, P, N> {
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
