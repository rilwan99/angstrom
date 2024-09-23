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
    type Currency is address;
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
    error Expired();
    error FailedToDeployNewStore();
    error FeeAboveMax();
    error FillingTooLittle();
    error FillingTooMuch();
    error InvalidPoolKey();
    error InvalidSignature();
    error InvalidTickSpacing();
    error LimitViolated();
    error MissingHookPermissions(uint160);
    error NonceReuse();
    error NotController();
    error NotNode();
    error NotUniswap();
    error OnlyOncePerBlock();
    error OrderAlreadyExecuted();
    error OutOfOrderOrDuplicatePairs();
    error Overflow();
    error PairAccessOutOfBounds(uint256 index, uint256 length);
    error ReaderNotAtEnd();
    error Underflow();
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4PoolManager, address governance);

    function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeInitialize(address, PoolKey memory poolKey, uint160, bytes memory) external view returns (bytes4);
    function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 feeInE6) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function execute(bytes memory encoded) external;
    function govToggleNodes(address[] memory nodes) external;
    function invalidateNonce(uint64 nonce) external;
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
    "name": "Expired",
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
    "name": "InvalidTickSpacing",
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
pub mod Angstrom {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610160604052348015610010575f80fd5b5060405161443138038061443183398101604081905261002f9161016f565b306080524660a052808260608061007a6040805180820182526008815267416e677374726f6d60c01b60208083019190915282518084019093526002835261763160f01b9083015291565b815160209283012081519183019190912060c082905260e0819052604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8152938401929092529082015246606082015230608082015260a090206101005250506001600160a01b039081166101205216610140526100fc61208061010e565b610107610a0061010e565b50506101a0565b806001600160a01b03168130166001600160a01b03161461015157604051630ea7064560e31b81526001600160a01b038216600482015260240160405180910390fd5b50565b80516001600160a01b038116811461016a575f80fd5b919050565b5f8060408385031215610180575f80fd5b61018983610154565b915061019760208401610154565b90509250929050565b60805160a05160c05160e05161010051610120516101405161419f6102925f395f81816102940152610c3601525f81816101e0015281816103fe01528181610514015281816105990152818161068801528181610769015281816108f001528181610b0d015281816112f80152818161136c015281816113d6015281816116e001528181611a8901528181611b7601528181611b9d01528181611f9001528181611fcc0152818161200d015281816120510152818161209d015281816131b4015281816135db01526139c601525f61299101525f612a4b01525f612a2501525f6129d501525f6129b2015261419f5ff3fe608060405234801561000f575f80fd5b506004361061009f575f3560e01c8063259982e51161007257806384b0196e1161005857806384b0196e1461014d57806391dd734614610168578063c6a98eb914610188575f80fd5b8063259982e5146101275780633440d8201461013a575f80fd5b806309c5eabe146100a35780631090641d146100b8578063116a5550146100cb57806321d0ee70146100de575b5f80fd5b6100b66100b1366004613a2b565b61019b565b005b6100b66100c6366004613aa2565b61027c565b6100b66100d9366004613aff565b6103d8565b6100f16100ec366004613b3c565b6103e5565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6100f1610135366004613b3c565b61066f565b6100f1610148366004613bdb565b6108d7565b610155610a4b565b60405161011e9796959493929190613c81565b61017b610176366004613a2b565b610af3565b60405161011e9190613d40565b6100b6610196366004613d52565b610c1e565b6101a3610d19565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c89491906102179085908590600401613dc3565b5f604051808303815f875af1158015610232573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526102779190810190613e3c565b505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146102eb576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8373ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff1611610350576040517f9f38afb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8481526020849052604090206006546103919060059068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16838686610dec565b600660086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050505050565b6103e23382611006565b50565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610455576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6104638560400135611041565b90505f61046f87611082565b90505f806104f1838b61048560208c018c613f2c565b61049560408d0160208e01613f2c565b604080516006929092526003929092525f92835260608d810135602652603a600c2094845260208590526007835290922091905260081c7f10000000000000000000000000000000000000000000000000000000000000001791565b90925090505f61054361053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016866110f0565b60a01c60020b90565b90505f61057c8261055760208d018d613f2c565b61056760408e0160208f01613f2c565b5f898152600860205260409020929190611197565b90505f6105c073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168786611237565b85549091505f906105e3846fffffffffffffffffffffffffffffffff851661127c565b6105ed9190613f79565b905061060b8e8e5f0160208101906106059190613f8c565b836112a7565b61063a61061789611446565b6106219084613fa7565b84906fffffffffffffffffffffffffffffffff1661127c565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106df576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408401355f6106ee87611082565b90505f6106fe6020880188613f2c565b90505f6107116040890160208a01613f2c565b90505f60085f8581526020019081526020015f2090505f610748858d86868e6060013560076110969095949392919063ffffffff16565b5090505f61078f61053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016886110f0565b90505f8362ffffff8716630100000081106107ac576107ac613fcf565b015490505f8462ffffff8716630100000081106107cb576107cb613fcf565b015490505f8760020b8460020b121561081e578183101561080d5781925082865f018962ffffff166301000000811061080657610806613fcf565b015561087c565b6108178284613f79565b905061087c565b8360020b8760020b1361085b578282101561085157829150818662ffffff89166301000000811061080657610806613fcf565b6108178383613f79565b8183876301000000015461086f9190613f79565b6108799190613f79565b90505b5f610887828c61127c565b905080865f015f82825461089b9190613ffc565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610947576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6109766109586020880188613f8c565b6109686040890160208a01613f8c565b5f9182526020526040902090565b5f8181526005602052604081209192509060081c7f2000000000000000000000000000000000000000000000000000000000000000175460020b9050806109c36080890160608a01613f2c565b60020b1415806109e757505f6109df6060890160408a0161400f565b62ffffff1614155b15610a1e576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d82000000000000000000000000000000000000000000000000000000000979650505050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083610ae1604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b60603373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610b64576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600183015f8435811a15801590610b9d5760065468010000000000000000900473ffffffffffffffffffffffffffffffffffffffff1691505b505f610ba88361146b565b90935090505f610bbb848360058661153c565b9094509050610bc9826116b0565b610bd5846002836117d7565b9350610be18482611863565b9350610bed8482611903565b9350610bfa848888611992565b610c03826119a9565b5050604080515f815260208101909152925050505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610c8d576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b81811015610277575f838383818110610caa57610caa613fcf565b9050602002016020810190610cbf9190613f8c565b73ffffffffffffffffffffffffffffffffffffffff165f90815260046020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610c8f565b6006544367ffffffffffffffff90911603610d60576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526004602052604090205460ff16610da8576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610db143611c3b565b600680547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b5f8261ffff165f03610e2a576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff83161115610e6c576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f84815260208790526040812060081c7f200000000000000000000000000000000000000000000000000000000000000017805462ffffff85166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000090911661ffff87161717815590505f610efe602073ffffffffffffffffffffffffffffffffffffffff89163b614028565b90505f610f0b8760281b90565b90506040516020810183602002806001838d3c64ffff00000060188a901b1662ffffff89161784178282018181525b80841015610f825783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081168703610f765782855250610f82565b50602084019350610f3a565b6b600b380380600b5f395ff3008552831460051b919091019050600c8101601484015ff096505073ffffffffffffffffffffffffffffffffffffffff86169150610ffa9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505095945050505050565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661103957638cb888725f526004601cfd5b909155505050565b5f8082131561107c576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b604080516006939093526003939093525f938452602652603a600c209383526020849052938152606090912092905260089190911c7f10000000000000000000000000000000000000000000000000000000000000001791565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa15801561116b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061118f9190614060565b949350505050565b5f808562ffffff8516630100000081106111b3576111b3613fcf565b015490505f8662ffffff8516630100000081106111d2576111d2613fcf565b015490508460020b8660020b12156111f7576111ee8183613f79565b9250505061118f565b8560020b8460020b1361120e576111ee8282613f79565b808288630100000001546112229190613f79565b61122c9190613f79565b979650505050505050565b5f828152600660208181526040808420858552909201905281205f61127273ffffffffffffffffffffffffffffffffffffffff871683611c54565b9695505050505050565b5f815f190483118202156112975763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b805f036112b357505050565b6040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a5841194906024015f604051808303815f87803b158015611339575f80fd5b505af115801561134b573d5f803e3d5ffd5b506113919250505073ffffffffffffffffffffffffffffffffffffffff83167f000000000000000000000000000000000000000000000000000000000000000083611cea565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af115801561141c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114409190614060565b50505050565b5f700100000000000000000000000000000000821061146757611467611d33565b5090565b6003818101915f918291803560e81c01018160446114898684613f79565b6114939190614028565b905080602086901b1792505f805b82811015611530575f6114bf602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611526576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b91506001016114a1565b50829450505050915091565b6003848101945f91829182918291803560e81c010181602661155e8b84613f79565b6115689190614028565b905060405193508060c0028401925082604052808460201b179450505f5b828410156116a15760048a01993560e081901c905f906115ae906114b8908d9060f01c611d40565b90505f6115c26114b88d61ffff8616611d40565b90508363ffffffff168363ffffffff1611158061160b57508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611642576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028c019b919250903560f01c5f806116698c8c8686611d9e565b60408a0191909152606089015250505060208b019a3590505f61168b82611e81565b60808701525060a085015260c090930192611586565b50935050505b94509492505050565b63ffffffff81165f5b818110156102775760448102602084901c01601481013560801c813560601c81156117cc577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16630b0d9c096117378373ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604481018590526064015f604051808303815f87803b1580156117a7575f80fd5b505af11580156117b9573d5f803e3d5ffd5b506117cc92506002915083905084611ea3565b5050506001016116b9565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81523060a082015261012080820152600385810195803560e81c0101905b8186146118595761185286828787611edc565b955061183f565b5093949350505050565b6003828101925f91813560e81c909101018161187d61210c565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101919091527fd29cd8ad130638ce44ea185648d2712345e8e456fef8153328ee40916df2790e81529091505b828614611859576118fc86828488612156565b95506118e9565b5f8061190d61210c565b60408051610160810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101919091526003868101969293509091803560e81c01015b8086146118595761198b868385886122eb565b9550611978565b808201808414611440576301842f8c5f526004601cfd5b63ffffffff81165f5b818110156102775760448102602084901c01803560601c6024820135608090811c906034840135901c5f6119f3846119ea8486613ffc565b60029190612506565b1215611a48576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024015b60405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff83165f9081526001602052604081208054849290611a7c908490613ffc565b90915550508015611c2b577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611ae08573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611b43575f80fd5b505af1158015611b55573d5f803e3d5ffd5b50611b9b9250505073ffffffffffffffffffffffffffffffffffffffff84167f000000000000000000000000000000000000000000000000000000000000000083611cea565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611c05573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c299190614060565b505b5050600190920191506119b29050565b5f68010000000000000000821061146757611467611d33565b6040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290525f9073ffffffffffffffffffffffffffffffffffffffff841690631e2eaeaf90602401602060405180830381865afa158015611cbf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ce39190614060565b9392505050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716611d2a576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b5f8163ffffffff841611611d8f576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611a3f565b602083901c6044830201611ce3565b5f8073ffffffffffffffffffffffffffffffffffffffff8516611e0a575f84815260208790526040812060081c7f20000000000000000000000000000000000000000000000000000000000000001754600281900b93506301000000900462ffffff169150611e459050565b5f611e36611e188660281b90565b73ffffffffffffffffffffffffffffffffffffffff88169086612549565b61ffff601882901c1693509150505b5f8260020b136116a7576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610c1882760a70c3c40a64e6c51999090b65f67d9240000000000000614028565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020611440611ed5825c84612587565b829061259f565b60018401935f9035811a1515611ef285826125a6565b60028601953560f01c611f19611f0885836125f7565b805160208201516040909201519092565b60020b608089015273ffffffffffffffffffffffffffffffffffffffff9081166040890152166020870190815260a090205f60108901893560801c9099506fffffffffffffffffffffffffffffffff1690505f8115612080575f611fb661053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016866110f0565b9050611fc183612657565b60e08b0152611ff08a7f00000000000000000000000000000000000000000000000000000000000000006126b2565b61203361053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016866110f0565b60808b01515f86815260086020526040902091935061207a919086907f000000000000000000000000000000000000000000000000000000000000000090859087906126d0565b506120c6565b6120c361053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016856110f0565b90505b5f83815260086020526040812060808b01516120e6918d91879086612773565b60208c0151919c5091506120fc908a9083612506565b50999a9950505050505050505050565b5f61215161211861298f565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a600181811615156060870152860135608090811c60208701526011870135901c604086015260238601956021013560f01c6121b26002831615156121a086846125f7565b9060051b602081188201519101519091565b73ffffffffffffffffffffffffffffffffffffffff90811660a089015216608087015250600481166121e557855f6121ef565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660c087015295505f61221b876008841615612a87565b60e089015290975090505f6122526122468867ffffffffffffffff4316610100820152610120902090565b60228801526042872090565b905061225d81612b32565b5f60108416612275576122708983612b87565b61227f565b61227f8983612bf1565b909950905061228e8382612c35565b608088015160208901516122a9918391600188161515612c7d565b5f6122b98960c0015183811c1890565b90506122dd818a60a001518b604001516122d88960ff16600116151590565b612d07565b509798975050505050505050565b60018401935f9035811a6122ff8582612d7f565b600181161515608086015260028601955f90819081903560f01c61235060088616151561232c89846125f7565b9060051b602081188201519082018051608090910151606090930151919390929190565b73ffffffffffffffffffffffffffffffffffffffff92831660c08e01529290911660a08c0152945092505050602088018835606089018190529098508210156123c5576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600283166123d457875f6123de565b60148801883560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e089015297505f61240a896004861615612a87565b6101008b01529099509050612420888a86612e32565b98505f806124318a8c888888612e76565b919c50925090505f6124448b888c612fc8565b90505f6080881661245e576124598d83612b87565b612468565b6124688d83612bf1565b909d509050601088161561249f5761248b8c610140015164ffffffffff16612fe2565b61249a818d6101200151611006565b6124a8565b6124a882612b32565b6124b28582612c35565b60a08c01516124c99082908660018c161515612c7d565b5f6124d98d60e0015183811c1890565b90506124f4818e60c00151866122d88d60ff16600116151590565b509b9c9b505050505050505050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120612541612538825c8561301c565b9250818361259f565b509392505050565b5f6020826020026001015f863c7fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811683145f510290509392505050565b81810182811215610c185763c9654ed45f526004601cfd5b80825d5050565b80151560c0830152806125cd5773fffd8963efd1fc6a506488495d951d5263988d256125d4565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b5f8163ffffffff841611612646576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611a3f565b5060c08102602083901c0192915050565b5f7f800000000000000000000000000000000000000000000000000000000000000082111561107c576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af180610277576040513d5f823e503d5ff35b6126de600284900b82613034565b92506126ee600283900b82613034565b91508260020b8260020b1315612734578260020b612718828460020b61303490919063ffffffff16565b60020b131561272f5761272f868587868686613047565b61276b565b8260020b8260020b121561276b57612750600284900b82613034565b60020b8260020b121561276b5761276b8685878686866130ea565b505050505050565b60018501945f90819035811a158015906127ec5760108801973560801c6127b48161279d89613199565b6fffffffffffffffffffffffffffffffff166131da565b886301000000015f8282546127c99190613ffc565b90915550899450506fffffffffffffffffffffffffffffffff1691506129859050565b505f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b13156128645761285f83888789856131f5565b612871565b61287183888789856132c9565b909b50995060108201965092503560801c61289e816fffffffffffffffffffffffffffffffff8b166131da565b6128a8908b613ffc565b99506128c66fffffffffffffffffffffffffffffffff821684613ffc565b92506128d2868661339e565b5f6128df835f0151613199565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff161461295a576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c16600483015282166024820152604401611a3f565b8a856301000000015f8282546129709190613ffc565b90915550969c50929a50505050505050505050505b9550959350505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f0000000000000000000000000000000000000000000000000000000000000000461416612a845750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47083612b2857843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f818152602081905260409020805c15612b78576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612b8381600161259f565b5050565b6017601483013560e81c8084018201935f92813560601c92910190612bae838684846133d7565b612be4576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d612c2e57638baa579f5f526004601cfd5b9293915050565b8115612b835763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166102775763f959fdae5f526004601cfd5b81612c8a60028583611ea3565b8115612cde5773ffffffffffffffffffffffffffffffffffffffff8086165f90815260036020908152604080832093881683529290529081208054839290612cd3908490613f79565b90915550612d009050565b612d0073ffffffffffffffffffffffffffffffffffffffff851686308461341c565b5050505050565b81612d1460028583612506565b508115612d5e5773ffffffffffffffffffffffffffffffffffffffff8086165f90815260036020908152604080832093881683529290529081208054839290612cd3908490613ffc565b612d0073ffffffffffffffffffffffffffffffffffffffff85168683611cea565b6020811615612ddd576010811615612db757507fa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e60009052565b507febd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac88619052565b6010811615612e0c57507fef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f9052565b507f5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d49052565b5f6010821615612e5c5760088361013886013760056008840161015b860137600d83019250612e6e565b67ffffffffffffffff43166101208501525b509092915050565b5f8080806020871615612f2a57508635608090811c60208a810182905260108a0135831c60408c0181905260308b019a919091013590921c9181831015612ee9576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80831115612f23576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050612f55565b5060108701963560801c60408716612f42575f612f45565b60015b60ff1660208a0152604089018190525b60408716151580612f6857506020871615155b15612f9657915081612f7a8682613474565b9150612f8f82612f8a8188613481565b61348c565b9150612fbb565b905080612fa38682613497565b9250612fb883612fb38188613481565b6134a2565b92505b5095979096509350505050565b5f61118f612fd685856134ad565b60228401526042832090565b804211156103e2576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80820382811315610c185763c9654ed45f526004601cfd5b5f611ce382808507831381860503614077565b63010000008601545b5f61307373ffffffffffffffffffffffffffffffffffffffff88168787866134cd565b95509050600284810b9086900b1380159061308b5750805b156130d3578762ffffff8616630100000081106130aa576130aa613fcf565b01546130b69083613f79565b8862ffffff8716630100000081106130d0576130d0613fcf565b01555b508260020b8460020b126130505750505050505050565b63010000008601545b5f61311673ffffffffffffffffffffffffffffffffffffffff8816878786613541565b95509050600285810b9085900b1215613177578015613172578762ffffff86166301000000811061314957613149613fcf565b01546131559083613f79565b8862ffffff87166301000000811061316f5761316f613fcf565b01555b61317d565b50613190565b846131878161409d565b955050506130f3565b50505050505050565b5f610c1873ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001683613586565b5f611ce3826131f1670de0b6b3a7640000866140f9565b0490565b5f808080600181805b821561328b5760108a01993560801c6132178184613ffc565b9250613235818b6fffffffffffffffffffffffffffffffff166131da565b61323f9083613ffc565b9150818d8d62ffffff166301000000811061325c5761325c613fcf565b015f82825461326b9190613ffc565b90915550508851613287908b90613282908f6135c0565b613602565b9950505b61329d885f01518c8a6020015161361c565b809c508194505050876040015160020b8b60020b136131fe57989b909a50979850959695505050505050565b5f808080600181805b821561335f5760108a01993560801c6132eb8184613ffc565b9250613309818b6fffffffffffffffffffffffffffffffff166131da565b6133139083613ffc565b9150818d8d62ffffff166301000000811061333057613330613fcf565b015f82825461333f9190613ffc565b9091555050885161335b908b90613356908f6135c0565b61366b565b9950505b613371885f01518c8a60200151613685565b809c508194505050876040015160020b8b60020b13156132d257989b909a50979850959695505050505050565b808214612b83576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f5114171661346757637939f4245f526004601cfd5b5f60605260405250505050565b5f611ce383835b906136ac565b5f611ce3828461347b565b5f611ce38284613f79565b5f611ce382846136ce565b5f611ce38284613ffc565b5f80601083166134bf576101406134c3565b6101605b9093209392505050565b5f8080806134f48587078213868805035b6134e9906001614110565b600281900b60081d91565b90925090506135248161351e73ffffffffffffffffffffffffffffffffffffffff8b168a866136e6565b90613721565b90945090506135348282876137e3565b9250505094509492505050565b5f8080806135568587078213868805036134e9565b90925090506135248161358073ffffffffffffffffffffffffffffffffffffffff8b168a866136e6565b9061380d565b5f8181526006602052604081205f6135b773ffffffffffffffffffffffffffffffffffffffff861660038401611c54565b95945050505050565b5f61118f73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684846138d5565b808203608081901c15610c185763c9654ed45f526004601cfd5b5f80808061363d6134e9613631600189614151565b875f8183071291050390565b9150915061364f8161358089856139ab565b909450905061365f8282876137e3565b92505050935093915050565b818101608081901c15610c185763c9654ed45f526004601cfd5b5f80808061369a8587078213868805036134de565b9150915061364f8161351e89856139ab565b5f6b033b2e3c9fd0803ce80000006136c483856140f9565b611ce39190614028565b5f816136c46b033b2e3c9fd0803ce8000000856140f9565b5f82815260066020908152604080832084845260050190915281206135b773ffffffffffffffffffffffffffffffffffffffff861682611c54565b5f805f6137bc8460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826137d25760ff6137d9565b8360ff1681015b9150509250929050565b5f8160ff84166137f9600187900b610100614077565b6138039190614110565b61118f9190614077565b5f805f8360ff0390505f6138ae8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b90508061010014159350836138c3575f6138ca565b8160ff1681035b925050509250929050565b5f8281526006602090815260408083208484526004019091528120819081906040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290529091505f9073ffffffffffffffffffffffffffffffffffffffff881690631e2eaeaf90602401602060405180830381865afa158015613962573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139869190614060565b6fffffffffffffffffffffffffffffffff81169860809190911d975095505050505050565b5f611ce373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684846136e6565b5f8083601f8401126139fd575f80fd5b50813567ffffffffffffffff811115613a14575f80fd5b602083019150836020828501011115612bea575f80fd5b5f8060208385031215613a3c575f80fd5b823567ffffffffffffffff811115613a52575f80fd5b613a5e858286016139ed565b90969095509350505050565b73ffffffffffffffffffffffffffffffffffffffff811681146103e2575f80fd5b803562ffffff81168114613a9d575f80fd5b919050565b5f805f8060808587031215613ab5575f80fd5b8435613ac081613a6a565b93506020850135613ad081613a6a565b9250604085013561ffff81168114613ae6575f80fd5b9150613af460608601613a8b565b905092959194509250565b5f60208284031215613b0f575f80fd5b813567ffffffffffffffff81168114611ce3575f80fd5b5f60a08284031215613b36575f80fd5b50919050565b5f805f805f858703610160811215613b52575f80fd5b8635613b5d81613a6a565b9550613b6c8860208901613b26565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215613b9d575f80fd5b5060c08601925061014086013567ffffffffffffffff811115613bbe575f80fd5b613bca888289016139ed565b969995985093965092949392505050565b5f805f805f6101008688031215613bf0575f80fd5b8535613bfb81613a6a565b9450613c0a8760208801613b26565b935060c0860135613c1a81613a6a565b925060e086013567ffffffffffffffff811115613bbe575f80fd5b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f613cbb60e0830189613c35565b8281036040840152613ccd8189613c35565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015613d2f578351835260209384019390920191600101613d11565b50909b9a5050505050505050505050565b602081525f611ce36020830184613c35565b5f8060208385031215613d63575f80fd5b823567ffffffffffffffff811115613d79575f80fd5b8301601f81018513613d89575f80fd5b803567ffffffffffffffff811115613d9f575f80fd5b8560208260051b8401011115613db3575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f60208284031215613e4c575f80fd5b815167ffffffffffffffff811115613e62575f80fd5b8201601f81018413613e72575f80fd5b805167ffffffffffffffff811115613e8c57613e8c613e0f565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff82111715613ef857613ef8613e0f565b604052818152828201602001861015613f0f575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215613f3c575f80fd5b81358060020b8114611ce3575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115610c1857610c18613f4c565b5f60208284031215613f9c575f80fd5b8135611ce381613a6a565b6fffffffffffffffffffffffffffffffff8281168282160390811115610c1857610c18613f4c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115610c1857610c18613f4c565b5f6020828403121561401f575f80fd5b611ce382613a8b565b5f8261405b577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f60208284031215614070575f80fd5b5051919050565b5f8260020b8260020b028060020b915080821461409657614096613f4c565b5092915050565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000081036140d1576140d1613f4c565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b8082028115828204841417610c1857610c18613f4c565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715610c1857610c18613f4c565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715610c1857610c18613f4c56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W_\x80\xFD[P`@QaD18\x03\x80aD1\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01oV[0`\x80RF`\xA0R\x80\x82``\x80a\0z`@\x80Q\x80\x82\x01\x82R`\x08\x81RgAngstrom`\xC0\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83Rav1`\xF0\x1B\x90\x83\x01R\x91V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xC0\x82\x90R`\xE0\x81\x90R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x93\x84\x01\x92\x90\x92R\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 a\x01\0RPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01 R\x16a\x01@Ra\0\xFCa \x80a\x01\x0EV[a\x01\x07a\n\0a\x01\x0EV[PPa\x01\xA0V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x810\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01QW`@Qc\x0E\xA7\x06E`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01jW_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x01\x80W_\x80\xFD[a\x01\x89\x83a\x01TV[\x91Pa\x01\x97` \x84\x01a\x01TV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@QaA\x9Fa\x02\x92_9_\x81\x81a\x02\x94\x01Ra\x0C6\x01R_\x81\x81a\x01\xE0\x01R\x81\x81a\x03\xFE\x01R\x81\x81a\x05\x14\x01R\x81\x81a\x05\x99\x01R\x81\x81a\x06\x88\x01R\x81\x81a\x07i\x01R\x81\x81a\x08\xF0\x01R\x81\x81a\x0B\r\x01R\x81\x81a\x12\xF8\x01R\x81\x81a\x13l\x01R\x81\x81a\x13\xD6\x01R\x81\x81a\x16\xE0\x01R\x81\x81a\x1A\x89\x01R\x81\x81a\x1Bv\x01R\x81\x81a\x1B\x9D\x01R\x81\x81a\x1F\x90\x01R\x81\x81a\x1F\xCC\x01R\x81\x81a \r\x01R\x81\x81a Q\x01R\x81\x81a \x9D\x01R\x81\x81a1\xB4\x01R\x81\x81a5\xDB\x01Ra9\xC6\x01R_a)\x91\x01R_a*K\x01R_a*%\x01R_a)\xD5\x01R_a)\xB2\x01RaA\x9F_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x9FW_5`\xE0\x1C\x80c%\x99\x82\xE5\x11a\0rW\x80c\x84\xB0\x19n\x11a\0XW\x80c\x84\xB0\x19n\x14a\x01MW\x80c\x91\xDDsF\x14a\x01hW\x80c\xC6\xA9\x8E\xB9\x14a\x01\x88W_\x80\xFD[\x80c%\x99\x82\xE5\x14a\x01'W\x80c4@\xD8 \x14a\x01:W_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\0\xA3W\x80c\x10\x90d\x1D\x14a\0\xB8W\x80c\x11jUP\x14a\0\xCBW\x80c!\xD0\xEEp\x14a\0\xDEW[_\x80\xFD[a\0\xB6a\0\xB16`\x04a:+V[a\x01\x9BV[\0[a\0\xB6a\0\xC66`\x04a:\xA2V[a\x02|V[a\0\xB6a\0\xD96`\x04a:\xFFV[a\x03\xD8V[a\0\xF1a\0\xEC6`\x04a;<V[a\x03\xE5V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF1a\x0156`\x04a;<V[a\x06oV[a\0\xF1a\x01H6`\x04a;\xDBV[a\x08\xD7V[a\x01Ua\nKV[`@Qa\x01\x1E\x97\x96\x95\x94\x93\x92\x91\x90a<\x81V[a\x01{a\x01v6`\x04a:+V[a\n\xF3V[`@Qa\x01\x1E\x91\x90a=@V[a\0\xB6a\x01\x966`\x04a=RV[a\x0C\x1EV[a\x01\xA3a\r\x19V[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x02\x17\x90\x85\x90\x85\x90`\x04\x01a=\xC3V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x022W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x02w\x91\x90\x81\x01\x90a><V[PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xEBW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x03PW`@Q\x7F\x9F8\xAF\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x81R` \x84\x90R`@\x90 `\x06Ta\x03\x91\x90`\x05\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x86\x86a\r\xECV[`\x06`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[a\x03\xE23\x82a\x10\x06V[PV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04UW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04c\x85`@\x015a\x10AV[\x90P_a\x04o\x87a\x10\x82V[\x90P_\x80a\x04\xF1\x83\x8Ba\x04\x85` \x8C\x01\x8Ca?,V[a\x04\x95`@\x8D\x01` \x8E\x01a?,V[`@\x80Q`\x06\x92\x90\x92R`\x03\x92\x90\x92R_\x92\x83R``\x8D\x81\x015`&R`:`\x0C \x94\x84R` \x85\x90R`\x07\x83R\x90\x92 \x91\x90R`\x08\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[\x90\x92P\x90P_a\x05Ca\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x10\xF0V[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\x05|\x82a\x05W` \x8D\x01\x8Da?,V[a\x05g`@\x8E\x01` \x8F\x01a?,V[_\x89\x81R`\x08` R`@\x90 \x92\x91\x90a\x11\x97V[\x90P_a\x05\xC0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x127V[\x85T\x90\x91P_\x90a\x05\xE3\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x12|V[a\x05\xED\x91\x90a?yV[\x90Pa\x06\x0B\x8E\x8E_\x01` \x81\x01\x90a\x06\x05\x91\x90a?\x8CV[\x83a\x12\xA7V[a\x06:a\x06\x17\x89a\x14FV[a\x06!\x90\x84a?\xA7V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12|V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xDFW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x015_a\x06\xEE\x87a\x10\x82V[\x90P_a\x06\xFE` \x88\x01\x88a?,V[\x90P_a\x07\x11`@\x89\x01` \x8A\x01a?,V[\x90P_`\x08_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x07H\x85\x8D\x86\x86\x8E``\x015`\x07a\x10\x96\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x07\x8Fa\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x10\xF0V[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xACWa\x07\xACa?\xCFV[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xCBWa\x07\xCBa?\xCFV[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\x08\x1EW\x81\x83\x10\x15a\x08\rW\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x08\x06Wa\x08\x06a?\xCFV[\x01Ua\x08|V[a\x08\x17\x82\x84a?yV[\x90Pa\x08|V[\x83`\x02\x0B\x87`\x02\x0B\x13a\x08[W\x82\x82\x10\x15a\x08QW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\x08\x06Wa\x08\x06a?\xCFV[a\x08\x17\x83\x83a?yV[\x81\x83\x87c\x01\0\0\0\x01Ta\x08o\x91\x90a?yV[a\x08y\x91\x90a?yV[\x90P[_a\x08\x87\x82\x8Ca\x12|V[\x90P\x80\x86_\x01_\x82\x82Ta\x08\x9B\x91\x90a?\xFCV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\tGW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\tva\tX` \x88\x01\x88a?\x8CV[a\th`@\x89\x01` \x8A\x01a?\x8CV[_\x91\x82R` R`@\x90 \x90V[_\x81\x81R`\x05` R`@\x81 \x91\x92P\x90`\x08\x1C\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17T`\x02\x0B\x90P\x80a\t\xC3`\x80\x89\x01``\x8A\x01a?,V[`\x02\x0B\x14\x15\x80a\t\xE7WP_a\t\xDF``\x89\x01`@\x8A\x01a@\x0FV[b\xFF\xFF\xFF\x16\x14\x15[\x15a\n\x1EW`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x96PPPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\n\xE1`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BdW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01_\x845\x81\x1A\x15\x80\x15\x90a\x0B\x9DW`\x06Th\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P[P_a\x0B\xA8\x83a\x14kV[\x90\x93P\x90P_a\x0B\xBB\x84\x83`\x05\x86a\x15<V[\x90\x94P\x90Pa\x0B\xC9\x82a\x16\xB0V[a\x0B\xD5\x84`\x02\x83a\x17\xD7V[\x93Pa\x0B\xE1\x84\x82a\x18cV[\x93Pa\x0B\xED\x84\x82a\x19\x03V[\x93Pa\x0B\xFA\x84\x88\x88a\x19\x92V[a\x0C\x03\x82a\x19\xA9V[PP`@\x80Q_\x81R` \x81\x01\x90\x91R\x92PPP[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\x8DW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x02wW_\x83\x83\x83\x81\x81\x10a\x0C\xAAWa\x0C\xAAa?\xCFV[\x90P` \x02\x01` \x81\x01\x90a\x0C\xBF\x91\x90a?\x8CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x0C\x8FV[`\x06TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\r`W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x04` R`@\x90 T`\xFF\x16a\r\xA8W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xB1Ca\x1C;V[`\x06\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x82a\xFF\xFF\x16_\x03a\x0E*W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x0ElW`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x81R` \x87\x90R`@\x81 `\x08\x1C\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x80Tb\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x91\x16a\xFF\xFF\x87\x16\x17\x17\x81U\x90P_a\x0E\xFE` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16;a@(V[\x90P_a\x0F\x0B\x87`(\x1B\x90V[\x90P`@Q` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x8A\x90\x1B\x16b\xFF\xFF\xFF\x89\x16\x17\x84\x17\x82\x82\x01\x81\x81R[\x80\x84\x10\x15a\x0F\x82W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x0FvW\x82\x85RPa\x0F\x82V[P` \x84\x01\x93Pa\x0F:V[k`\x0B8\x03\x80`\x0B_9_\xF3\0\x85R\x83\x14`\x05\x1B\x91\x90\x91\x01\x90P`\x0C\x81\x01`\x14\x84\x01_\xF0\x96PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91Pa\x0F\xFA\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x95\x94PPPPPV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\x109Wc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_\x80\x82\x13\x15a\x10|W`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`@\x80Q`\x06\x93\x90\x93R`\x03\x93\x90\x93R_\x93\x84R`&R`:`\x0C \x93\x83R` \x84\x90R\x93\x81R``\x90\x91 \x92\x90R`\x08\x91\x90\x91\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8F\x91\x90a@`V[\x94\x93PPPPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xB3Wa\x11\xB3a?\xCFV[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xD2Wa\x11\xD2a?\xCFV[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x11\xF7Wa\x11\xEE\x81\x83a?yV[\x92PPPa\x11\x8FV[\x85`\x02\x0B\x84`\x02\x0B\x13a\x12\x0EWa\x11\xEE\x82\x82a?yV[\x80\x82\x88c\x01\0\0\0\x01Ta\x12\"\x91\x90a?yV[a\x12,\x91\x90a?yV[\x97\x96PPPPPPPV[_\x82\x81R`\x06` \x81\x81R`@\x80\x84 \x85\x85R\x90\x92\x01\x90R\x81 _a\x12rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x83a\x1CTV[\x96\x95PPPPPPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x12\x97Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x80_\x03a\x12\xB3WPPPV[`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x139W_\x80\xFD[PZ\xF1\x15\x80\x15a\x13KW=_\x80>=_\xFD[Pa\x13\x91\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1C\xEAV[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14@\x91\x90a@`V[PPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x14gWa\x14ga\x1D3V[P\x90V[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x14\x89\x86\x84a?yV[a\x14\x93\x91\x90a@(V[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x150W_a\x14\xBF` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x15&W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x14\xA1V[P\x82\x94PPPP\x91P\x91V[`\x03\x84\x81\x01\x94_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x15^\x8B\x84a?yV[a\x15h\x91\x90a@(V[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x16\xA1W`\x04\x8A\x01\x995`\xE0\x81\x90\x1C\x90_\x90a\x15\xAE\x90a\x14\xB8\x90\x8D\x90`\xF0\x1Ca\x1D@V[\x90P_a\x15\xC2a\x14\xB8\x8Da\xFF\xFF\x86\x16a\x1D@V[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x16\x0BWP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x16BW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8C\x01\x9B\x91\x92P\x905`\xF0\x1C_\x80a\x16i\x8C\x8C\x86\x86a\x1D\x9EV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP` \x8B\x01\x9A5\x90P_a\x16\x8B\x82a\x1E\x81V[`\x80\x87\x01RP`\xA0\x85\x01R`\xC0\x90\x93\x01\x92a\x15\x86V[P\x93PPP[\x94P\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02wW`D\x81\x02` \x84\x90\x1C\x01`\x14\x81\x015`\x80\x1C\x815``\x1C\x81\x15a\x17\xCCW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0B\r\x9C\ta\x177\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xA7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xB9W=_\x80>=_\xFD[Pa\x17\xCC\x92P`\x02\x91P\x83\x90P\x84a\x1E\xA3V[PPP`\x01\x01a\x16\xB9V[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x85\x81\x01\x95\x805`\xE8\x1C\x01\x01\x90[\x81\x86\x14a\x18YWa\x18R\x86\x82\x87\x87a\x1E\xDCV[\x95Pa\x18?V[P\x93\x94\x93PPPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x18}a!\x0CV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x7F\xD2\x9C\xD8\xAD\x13\x068\xCED\xEA\x18VH\xD2q#E\xE8\xE4V\xFE\xF8\x153(\xEE@\x91m\xF2y\x0E\x81R\x90\x91P[\x82\x86\x14a\x18YWa\x18\xFC\x86\x82\x84\x88a!VV[\x95Pa\x18\xE9V[_\x80a\x19\ra!\x0CV[`@\x80Qa\x01`\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a\x18YWa\x19\x8B\x86\x83\x85\x88a\"\xEBV[\x95Pa\x19xV[\x80\x82\x01\x80\x84\x14a\x14@Wc\x01\x84/\x8C_R`\x04`\x1C\xFD[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02wW`D\x81\x02` \x84\x90\x1C\x01\x805``\x1C`$\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a\x19\xF3\x84a\x19\xEA\x84\x86a?\xFCV[`\x02\x91\x90a%\x06V[\x12\x15a\x1AHW`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x84\x92\x90a\x1A|\x90\x84\x90a?\xFCV[\x90\x91UPP\x80\x15a\x1C+W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1A\xE0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BCW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1BUW=_\x80>=_\xFD[Pa\x1B\x9B\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1C\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x05W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C)\x91\x90a@`V[P[PP`\x01\x90\x92\x01\x91Pa\x19\xB2\x90PV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x14gWa\x14ga\x1D3V[`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE3\x91\x90a@`V[\x93\x92PPPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1D*Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a\x1D\x8FW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1A?V[` \x83\x90\x1C`D\x83\x02\x01a\x1C\xE3V[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x1E\nW_\x84\x81R` \x87\x90R`@\x81 `\x08\x1C\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17T`\x02\x81\x90\x0B\x93Pc\x01\0\0\0\x90\x04b\xFF\xFF\xFF\x16\x91Pa\x1EE\x90PV[_a\x1E6a\x1E\x18\x86`(\x1B\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90\x86a%IV[a\xFF\xFF`\x18\x82\x90\x1C\x16\x93P\x91PP[_\x82`\x02\x0B\x13a\x16\xA7W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\x18\x82v\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0a@(V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x14@a\x1E\xD5\x82\\\x84a%\x87V[\x82\x90a%\x9FV[`\x01\x84\x01\x93_\x905\x81\x1A\x15\x15a\x1E\xF2\x85\x82a%\xA6V[`\x02\x86\x01\x955`\xF0\x1Ca\x1F\x19a\x1F\x08\x85\x83a%\xF7V[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x89\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x89\x01R\x16` \x87\x01\x90\x81R`\xA0\x90 _`\x10\x89\x01\x895`\x80\x1C\x90\x99Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a \x80W_a\x1F\xB6a\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x10\xF0V[\x90Pa\x1F\xC1\x83a&WV[`\xE0\x8B\x01Ra\x1F\xF0\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&\xB2V[a 3a\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x10\xF0V[`\x80\x8B\x01Q_\x86\x81R`\x08` R`@\x90 \x91\x93Pa z\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a&\xD0V[Pa \xC6V[a \xC3a\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x10\xF0V[\x90P[_\x83\x81R`\x08` R`@\x81 `\x80\x8B\x01Qa \xE6\x91\x8D\x91\x87\x90\x86a'sV[` \x8C\x01Q\x91\x9CP\x91Pa \xFC\x90\x8A\x90\x83a%\x06V[P\x99\x9A\x99PPPPPPPPPPV[_a!Qa!\x18a)\x8FV[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15``\x87\x01R\x86\x015`\x80\x90\x81\x1C` \x87\x01R`\x11\x87\x015\x90\x1C`@\x86\x01R`#\x86\x01\x95`!\x015`\xF0\x1Ca!\xB2`\x02\x83\x16\x15\x15a!\xA0\x86\x84a%\xF7V[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x89\x01R\x16`\x80\x87\x01RP`\x04\x81\x16a!\xE5W\x85_a!\xEFV[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x87\x01R\x95P_a\"\x1B\x87`\x08\x84\x16\x15a*\x87V[`\xE0\x89\x01R\x90\x97P\x90P_a\"Ra\"F\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01Ra\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90Pa\"]\x81a+2V[_`\x10\x84\x16a\"uWa\"p\x89\x83a+\x87V[a\"\x7FV[a\"\x7F\x89\x83a+\xF1V[\x90\x99P\x90Pa\"\x8E\x83\x82a,5V[`\x80\x88\x01Q` \x89\x01Qa\"\xA9\x91\x83\x91`\x01\x88\x16\x15\x15a,}V[_a\"\xB9\x89`\xC0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa\"\xDD\x81\x8A`\xA0\x01Q\x8B`@\x01Qa\"\xD8\x89`\xFF\x16`\x01\x16\x15\x15\x90V[a-\x07V[P\x97\x98\x97PPPPPPPPV[`\x01\x84\x01\x93_\x905\x81\x1Aa\"\xFF\x85\x82a-\x7FV[`\x01\x81\x16\x15\x15`\x80\x86\x01R`\x02\x86\x01\x95_\x90\x81\x90\x81\x905`\xF0\x1Ca#P`\x08\x86\x16\x15\x15a#,\x89\x84a%\xF7V[\x90`\x05\x1B` \x81\x18\x82\x01Q\x90\x82\x01\x80Q`\x80\x90\x91\x01Q``\x90\x93\x01Q\x91\x93\x90\x92\x91\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\xC0\x8E\x01R\x92\x90\x91\x16`\xA0\x8C\x01R\x94P\x92PPP` \x88\x01\x885``\x89\x01\x81\x90R\x90\x98P\x82\x10\x15a#\xC5W`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x83\x16a#\xD4W\x87_a#\xDEV[`\x14\x88\x01\x885``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x89\x01R\x97P_a$\n\x89`\x04\x86\x16\x15a*\x87V[a\x01\0\x8B\x01R\x90\x99P\x90Pa$ \x88\x8A\x86a.2V[\x98P_\x80a$1\x8A\x8C\x88\x88\x88a.vV[\x91\x9CP\x92P\x90P_a$D\x8B\x88\x8Ca/\xC8V[\x90P_`\x80\x88\x16a$^Wa$Y\x8D\x83a+\x87V[a$hV[a$h\x8D\x83a+\xF1V[\x90\x9DP\x90P`\x10\x88\x16\x15a$\x9FWa$\x8B\x8Ca\x01@\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a/\xE2V[a$\x9A\x81\x8Da\x01 \x01Qa\x10\x06V[a$\xA8V[a$\xA8\x82a+2V[a$\xB2\x85\x82a,5V[`\xA0\x8C\x01Qa$\xC9\x90\x82\x90\x86`\x01\x8C\x16\x15\x15a,}V[_a$\xD9\x8D`\xE0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa$\xF4\x81\x8E`\xC0\x01Q\x86a\"\xD8\x8D`\xFF\x16`\x01\x16\x15\x15\x90V[P\x9B\x9C\x9BPPPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a%Aa%8\x82\\\x85a0\x1CV[\x92P\x81\x83a%\x9FV[P\x93\x92PPPV[_` \x82` \x02`\x01\x01_\x86<\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x83\x14_Q\x02\x90P\x93\x92PPPV[\x81\x81\x01\x82\x81\x12\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[\x80\x15\x15`\xC0\x83\x01R\x80a%\xCDWs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a%\xD4V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a&FW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1A?V[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x10|W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x02wW`@Q=_\x82>P=_\xF3[a&\xDE`\x02\x84\x90\x0B\x82a04V[\x92Pa&\xEE`\x02\x83\x90\x0B\x82a04V[\x91P\x82`\x02\x0B\x82`\x02\x0B\x13\x15a'4W\x82`\x02\x0Ba'\x18\x82\x84`\x02\x0Ba04\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a'/Wa'/\x86\x85\x87\x86\x86\x86a0GV[a'kV[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a'kWa'P`\x02\x84\x90\x0B\x82a04V[`\x02\x0B\x82`\x02\x0B\x12\x15a'kWa'k\x86\x85\x87\x86\x86\x86a0\xEAV[PPPPPPV[`\x01\x85\x01\x94_\x90\x81\x905\x81\x1A\x15\x80\x15\x90a'\xECW`\x10\x88\x01\x975`\x80\x1Ca'\xB4\x81a'\x9D\x89a1\x99V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\xDAV[\x88c\x01\0\0\0\x01_\x82\x82Ta'\xC9\x91\x90a?\xFCV[\x90\x91UP\x89\x94PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91Pa)\x85\x90PV[P_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a(dWa(_\x83\x88\x87\x89\x85a1\xF5V[a(qV[a(q\x83\x88\x87\x89\x85a2\xC9V[\x90\x9BP\x99P`\x10\x82\x01\x96P\x92P5`\x80\x1Ca(\x9E\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16a1\xDAV[a(\xA8\x90\x8Ba?\xFCV[\x99Pa(\xC6o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84a?\xFCV[\x92Pa(\xD2\x86\x86a3\x9EV[_a(\xDF\x83_\x01Qa1\x99V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)ZW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x1A?V[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta)p\x91\x90a?\xFCV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x95P\x95\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a*\x84WP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a+(W\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a+xW`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a+\x83\x81`\x01a%\x9FV[PPV[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90a+\xAE\x83\x86\x84\x84a3\xD7V[a+\xE4W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a,.Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81\x15a+\x83Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x02wWc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x81a,\x8A`\x02\x85\x83a\x1E\xA3V[\x81\x15a,\xDEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a,\xD3\x90\x84\x90a?yV[\x90\x91UPa-\0\x90PV[a-\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a4\x1CV[PPPPPV[\x81a-\x14`\x02\x85\x83a%\x06V[P\x81\x15a-^Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a,\xD3\x90\x84\x90a?\xFCV[a-\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x1C\xEAV[` \x81\x16\x15a-\xDDW`\x10\x81\x16\x15a-\xB7WP\x7F\xA9\xE5)MDO\xBA\xEB\\\xA0_+$\xC5\xD8)D\x10\xF3\x14\x13\x04\x8ED]\x88\x1E.\xF6\x9E`\0\x90RV[P\x7F\xEB\xD5\t\x1C9oy\x05nEE_J\x93\xBB\xB0\x16\xC2\x171K\xB25k\"\xD1\xC183\xAC\x88a\x90RV[`\x10\x81\x16\x15a.\x0CWP\x7F\xEF\x0C\xCE\x88\xFD\xA0J\xB3\xF8F\x18\x823r\xCFv\xF6KE\x11\xCE\x8Cyc\x0E\xAB\xC2\x9E\xBC\x9B\x96\x8F\x90RV[P\x7F[\x9DI\xCF\xEDH\xF8\xC9\xD1\xA8c\xF6\x1C$y\xC5y\xFA)\x9C%\xA32\x11\xFC\x85s\x90\xCE\xCC\xE6\xD4\x90RV[_`\x10\x82\x16\x15a.\\W`\x08\x83a\x018\x86\x017`\x05`\x08\x84\x01a\x01[\x86\x017`\r\x83\x01\x92Pa.nV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01 \x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x87\x16\x15a/*WP\x865`\x80\x90\x81\x1C` \x8A\x81\x01\x82\x90R`\x10\x8A\x015\x83\x1C`@\x8C\x01\x81\x90R`0\x8B\x01\x9A\x91\x90\x91\x015\x90\x92\x1C\x91\x81\x83\x10\x15a.\xE9W`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a/#W`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa/UV[P`\x10\x87\x01\x965`\x80\x1C`@\x87\x16a/BW_a/EV[`\x01[`\xFF\x16` \x8A\x01R`@\x89\x01\x81\x90R[`@\x87\x16\x15\x15\x80a/hWP` \x87\x16\x15\x15[\x15a/\x96W\x91P\x81a/z\x86\x82a4tV[\x91Pa/\x8F\x82a/\x8A\x81\x88a4\x81V[a4\x8CV[\x91Pa/\xBBV[\x90P\x80a/\xA3\x86\x82a4\x97V[\x92Pa/\xB8\x83a/\xB3\x81\x88a4\x81V[a4\xA2V[\x92P[P\x95\x97\x90\x96P\x93PPPPV[_a\x11\x8Fa/\xD6\x85\x85a4\xADV[`\"\x84\x01R`B\x83 \x90V[\x80B\x11\x15a\x03\xE2W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_a\x1C\xE3\x82\x80\x85\x07\x83\x13\x81\x86\x05\x03a@wV[c\x01\0\0\0\x86\x01T[_a0ss\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x87\x87\x86a4\xCDV[\x95P\x90P`\x02\x84\x81\x0B\x90\x86\x90\x0B\x13\x80\x15\x90a0\x8BWP\x80[\x15a0\xD3W\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a0\xAAWa0\xAAa?\xCFV[\x01Ta0\xB6\x90\x83a?yV[\x88b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a0\xD0Wa0\xD0a?\xCFV[\x01U[P\x82`\x02\x0B\x84`\x02\x0B\x12a0PWPPPPPPPV[c\x01\0\0\0\x86\x01T[_a1\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x87\x87\x86a5AV[\x95P\x90P`\x02\x85\x81\x0B\x90\x85\x90\x0B\x12\x15a1wW\x80\x15a1rW\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a1IWa1Ia?\xCFV[\x01Ta1U\x90\x83a?yV[\x88b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a1oWa1oa?\xCFV[\x01U[a1}V[Pa1\x90V[\x84a1\x87\x81a@\x9DV[\x95PPPa0\xF3V[PPPPPPPV[_a\x0C\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x83a5\x86V[_a\x1C\xE3\x82a1\xF1g\r\xE0\xB6\xB3\xA7d\0\0\x86a@\xF9V[\x04\x90V[_\x80\x80\x80`\x01\x81\x80[\x82\x15a2\x8BW`\x10\x8A\x01\x995`\x80\x1Ca2\x17\x81\x84a?\xFCV[\x92Pa25\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\xDAV[a2?\x90\x83a?\xFCV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a2\\Wa2\\a?\xCFV[\x01_\x82\x82Ta2k\x91\x90a?\xFCV[\x90\x91UPP\x88Qa2\x87\x90\x8B\x90a2\x82\x90\x8Fa5\xC0V[a6\x02V[\x99PP[a2\x9D\x88_\x01Q\x8C\x8A` \x01Qa6\x1CV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a1\xFEW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80\x80\x80`\x01\x81\x80[\x82\x15a3_W`\x10\x8A\x01\x995`\x80\x1Ca2\xEB\x81\x84a?\xFCV[\x92Pa3\t\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\xDAV[a3\x13\x90\x83a?\xFCV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a30Wa30a?\xCFV[\x01_\x82\x82Ta3?\x91\x90a?\xFCV[\x90\x91UPP\x88Qa3[\x90\x8B\x90a3V\x90\x8Fa5\xC0V[a6kV[\x99PP[a3q\x88_\x01Q\x8C\x8A` \x01Qa6\x85V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a2\xD2W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a+\x83W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a4gWcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[_a\x1C\xE3\x83\x83[\x90a6\xACV[_a\x1C\xE3\x82\x84a4{V[_a\x1C\xE3\x82\x84a?yV[_a\x1C\xE3\x82\x84a6\xCEV[_a\x1C\xE3\x82\x84a?\xFCV[_\x80`\x10\x83\x16a4\xBFWa\x01@a4\xC3V[a\x01`[\x90\x93 \x93\x92PPPV[_\x80\x80\x80a4\xF4\x85\x87\x07\x82\x13\x86\x88\x05\x03[a4\xE9\x90`\x01aA\x10V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa5$\x81a5\x1Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a6\xE6V[\x90a7!V[\x90\x94P\x90Pa54\x82\x82\x87a7\xE3V[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80a5V\x85\x87\x07\x82\x13\x86\x88\x05\x03a4\xE9V[\x90\x92P\x90Pa5$\x81a5\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a6\xE6V[\x90a8\rV[_\x81\x81R`\x06` R`@\x81 _a5\xB7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a\x1CTV[\x95\x94PPPPPV[_a\x11\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a8\xD5V[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a6=a4\xE9a61`\x01\x89aAQV[\x87_\x81\x83\x07\x12\x91\x05\x03\x90V[\x91P\x91Pa6O\x81a5\x80\x89\x85a9\xABV[\x90\x94P\x90Pa6_\x82\x82\x87a7\xE3V[\x92PPP\x93P\x93\x91PPV[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a6\x9A\x85\x87\x07\x82\x13\x86\x88\x05\x03a4\xDEV[\x91P\x91Pa6O\x81a5\x1E\x89\x85a9\xABV[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a6\xC4\x83\x85a@\xF9V[a\x1C\xE3\x91\x90a@(V[_\x81a6\xC4k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a@\xF9V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a5\xB7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a\x1CTV[_\x80_a7\xBC\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a7\xD2W`\xFFa7\xD9V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a7\xF9`\x01\x87\x90\x0Ba\x01\0a@wV[a8\x03\x91\x90aA\x10V[a\x11\x8F\x91\x90a@wV[_\x80_\x83`\xFF\x03\x90P_a8\xAE\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a8\xC3W_a8\xCAV[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x04\x01\x90\x91R\x81 \x81\x90\x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9bW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x86\x91\x90a@`V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\x80\x91\x90\x91\x1D\x97P\x95PPPPPPV[_a\x1C\xE3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a6\xE6V[_\x80\x83`\x1F\x84\x01\x12a9\xFDW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x14W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a+\xEAW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a:<W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:RW_\x80\xFD[a:^\x85\x82\x86\x01a9\xEDV[\x90\x96\x90\x95P\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xE2W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a:\x9DW_\x80\xFD[\x91\x90PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a:\xB5W_\x80\xFD[\x845a:\xC0\x81a:jV[\x93P` \x85\x015a:\xD0\x81a:jV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14a:\xE6W_\x80\xFD[\x91Pa:\xF4``\x86\x01a:\x8BV[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a;\x0FW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xE3W_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a;6W_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15a;RW_\x80\xFD[\x865a;]\x81a:jV[\x95Pa;l\x88` \x89\x01a;&V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a;\x9DW_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xBEW_\x80\xFD[a;\xCA\x88\x82\x89\x01a9\xEDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a;\xF0W_\x80\xFD[\x855a;\xFB\x81a:jV[\x94Pa<\n\x87` \x88\x01a;&V[\x93P`\xC0\x86\x015a<\x1A\x81a:jV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xBEW_\x80\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_a<\xBB`\xE0\x83\x01\x89a<5V[\x82\x81\x03`@\x84\x01Ra<\xCD\x81\x89a<5V[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a=/W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a=\x11V[P\x90\x9B\x9APPPPPPPPPPPV[` \x81R_a\x1C\xE3` \x83\x01\x84a<5V[_\x80` \x83\x85\x03\x12\x15a=cW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=yW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a=\x89W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\x9FW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a=\xB3W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a>LW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>bW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>rW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x8CWa>\x8Ca>\x0FV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a>\xF8Wa>\xF8a>\x0FV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a?\x0FW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a?<W_\x80\xFD[\x815\x80`\x02\x0B\x81\x14a\x1C\xE3W_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\x18Wa\x0C\x18a?LV[_` \x82\x84\x03\x12\x15a?\x9CW_\x80\xFD[\x815a\x1C\xE3\x81a:jV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\x18Wa\x0C\x18a?LV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\x18Wa\x0C\x18a?LV[_` \x82\x84\x03\x12\x15a@\x1FW_\x80\xFD[a\x1C\xE3\x82a:\x8BV[_\x82a@[W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a@pW_\x80\xFD[PQ\x91\x90PV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a@\x96Wa@\x96a?LV[P\x92\x91PPV[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a@\xD1Wa@\xD1a?LV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x18Wa\x0C\x18a?LV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x0C\x18Wa\x0C\x18a?LV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x0C\x18Wa\x0C\x18a?LV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b506004361061009f575f3560e01c8063259982e51161007257806384b0196e1161005857806384b0196e1461014d57806391dd734614610168578063c6a98eb914610188575f80fd5b8063259982e5146101275780633440d8201461013a575f80fd5b806309c5eabe146100a35780631090641d146100b8578063116a5550146100cb57806321d0ee70146100de575b5f80fd5b6100b66100b1366004613a2b565b61019b565b005b6100b66100c6366004613aa2565b61027c565b6100b66100d9366004613aff565b6103d8565b6100f16100ec366004613b3c565b6103e5565b6040517fffffffff0000000000000000000000000000000000000000000000000000000090911681526020015b60405180910390f35b6100f1610135366004613b3c565b61066f565b6100f1610148366004613bdb565b6108d7565b610155610a4b565b60405161011e9796959493929190613c81565b61017b610176366004613a2b565b610af3565b60405161011e9190613d40565b6100b6610196366004613d52565b610c1e565b6101a3610d19565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c89491906102179085908590600401613dc3565b5f604051808303815f875af1158015610232573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526102779190810190613e3c565b505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146102eb576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8373ffffffffffffffffffffffffffffffffffffffff168373ffffffffffffffffffffffffffffffffffffffff1611610350576040517f9f38afb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8481526020849052604090206006546103919060059068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16838686610dec565b600660086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505050505050565b6103e23382611006565b50565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610455576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6104638560400135611041565b90505f61046f87611082565b90505f806104f1838b61048560208c018c613f2c565b61049560408d0160208e01613f2c565b604080516006929092526003929092525f92835260608d810135602652603a600c2094845260208590526007835290922091905260081c7f10000000000000000000000000000000000000000000000000000000000000001791565b90925090505f61054361053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016866110f0565b60a01c60020b90565b90505f61057c8261055760208d018d613f2c565b61056760408e0160208f01613f2c565b5f898152600860205260409020929190611197565b90505f6105c073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168786611237565b85549091505f906105e3846fffffffffffffffffffffffffffffffff851661127c565b6105ed9190613f79565b905061060b8e8e5f0160208101906106059190613f8c565b836112a7565b61063a61061789611446565b6106219084613fa7565b84906fffffffffffffffffffffffffffffffff1661127c565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106df576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408401355f6106ee87611082565b90505f6106fe6020880188613f2c565b90505f6107116040890160208a01613f2c565b90505f60085f8581526020019081526020015f2090505f610748858d86868e6060013560076110969095949392919063ffffffff16565b5090505f61078f61053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016886110f0565b90505f8362ffffff8716630100000081106107ac576107ac613fcf565b015490505f8462ffffff8716630100000081106107cb576107cb613fcf565b015490505f8760020b8460020b121561081e578183101561080d5781925082865f018962ffffff166301000000811061080657610806613fcf565b015561087c565b6108178284613f79565b905061087c565b8360020b8760020b1361085b578282101561085157829150818662ffffff89166301000000811061080657610806613fcf565b6108178383613f79565b8183876301000000015461086f9190613f79565b6108799190613f79565b90505b5f610887828c61127c565b905080865f015f82825461089b9190613ffc565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610947576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f6109766109586020880188613f8c565b6109686040890160208a01613f8c565b5f9182526020526040902090565b5f8181526005602052604081209192509060081c7f2000000000000000000000000000000000000000000000000000000000000000175460020b9050806109c36080890160608a01613f2c565b60020b1415806109e757505f6109df6060890160408a0161400f565b62ffffff1614155b15610a1e576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d82000000000000000000000000000000000000000000000000000000000979650505050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083610ae1604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b60603373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610b64576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600183015f8435811a15801590610b9d5760065468010000000000000000900473ffffffffffffffffffffffffffffffffffffffff1691505b505f610ba88361146b565b90935090505f610bbb848360058661153c565b9094509050610bc9826116b0565b610bd5846002836117d7565b9350610be18482611863565b9350610bed8482611903565b9350610bfa848888611992565b610c03826119a9565b5050604080515f815260208101909152925050505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610c8d576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b81811015610277575f838383818110610caa57610caa613fcf565b9050602002016020810190610cbf9190613f8c565b73ffffffffffffffffffffffffffffffffffffffff165f90815260046020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610c8f565b6006544367ffffffffffffffff90911603610d60576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526004602052604090205460ff16610da8576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610db143611c3b565b600680547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b5f8261ffff165f03610e2a576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff83161115610e6c576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f84815260208790526040812060081c7f200000000000000000000000000000000000000000000000000000000000000017805462ffffff85166301000000027fffffffffffffffffffffffffffffffffffffffffffffffffffff00000000000090911661ffff87161717815590505f610efe602073ffffffffffffffffffffffffffffffffffffffff89163b614028565b90505f610f0b8760281b90565b90506040516020810183602002806001838d3c64ffff00000060188a901b1662ffffff89161784178282018181525b80841015610f825783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081168703610f765782855250610f82565b50602084019350610f3a565b6b600b380380600b5f395ff3008552831460051b919091019050600c8101601484015ff096505073ffffffffffffffffffffffffffffffffffffffff86169150610ffa9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50505095945050505050565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661103957638cb888725f526004601cfd5b909155505050565b5f8082131561107c576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b604080516006939093526003939093525f938452602652603a600c209383526020849052938152606090912092905260089190911c7f10000000000000000000000000000000000000000000000000000000000000001791565b5f81815260066020526040812081906040517f1e2eaeaf0000000000000000000000000000000000000000000000000000000081526004810182905290915073ffffffffffffffffffffffffffffffffffffffff851690631e2eaeaf90602401602060405180830381865afa15801561116b573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061118f9190614060565b949350505050565b5f808562ffffff8516630100000081106111b3576111b3613fcf565b015490505f8662ffffff8516630100000081106111d2576111d2613fcf565b015490508460020b8660020b12156111f7576111ee8183613f79565b9250505061118f565b8560020b8460020b1361120e576111ee8282613f79565b808288630100000001546112229190613f79565b61122c9190613f79565b979650505050505050565b5f828152600660208181526040808420858552909201905281205f61127273ffffffffffffffffffffffffffffffffffffffff871683611c54565b9695505050505050565b5f815f190483118202156112975763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b805f036112b357505050565b6040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a5841194906024015f604051808303815f87803b158015611339575f80fd5b505af115801561134b573d5f803e3d5ffd5b506113919250505073ffffffffffffffffffffffffffffffffffffffff83167f000000000000000000000000000000000000000000000000000000000000000083611cea565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af115801561141c573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114409190614060565b50505050565b5f700100000000000000000000000000000000821061146757611467611d33565b5090565b6003818101915f918291803560e81c01018160446114898684613f79565b6114939190614028565b905080602086901b1792505f805b82811015611530575f6114bf602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611526576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b91506001016114a1565b50829450505050915091565b6003848101945f91829182918291803560e81c010181602661155e8b84613f79565b6115689190614028565b905060405193508060c0028401925082604052808460201b179450505f5b828410156116a15760048a01993560e081901c905f906115ae906114b8908d9060f01c611d40565b90505f6115c26114b88d61ffff8616611d40565b90508363ffffffff168363ffffffff1611158061160b57508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611642576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028c019b919250903560f01c5f806116698c8c8686611d9e565b60408a0191909152606089015250505060208b019a3590505f61168b82611e81565b60808701525060a085015260c090930192611586565b50935050505b94509492505050565b63ffffffff81165f5b818110156102775760448102602084901c01601481013560801c813560601c81156117cc577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16630b0d9c096117378373ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604481018590526064015f604051808303815f87803b1580156117a7575f80fd5b505af11580156117b9573d5f803e3d5ffd5b506117cc92506002915083905084611ea3565b5050506001016116b9565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81523060a082015261012080820152600385810195803560e81c0101905b8186146118595761185286828787611edc565b955061183f565b5093949350505050565b6003828101925f91813560e81c909101018161187d61210c565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101919091527fd29cd8ad130638ce44ea185648d2712345e8e456fef8153328ee40916df2790e81529091505b828614611859576118fc86828488612156565b95506118e9565b5f8061190d61210c565b60408051610160810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101919091526003868101969293509091803560e81c01015b8086146118595761198b868385886122eb565b9550611978565b808201808414611440576301842f8c5f526004601cfd5b63ffffffff81165f5b818110156102775760448102602084901c01803560601c6024820135608090811c906034840135901c5f6119f3846119ea8486613ffc565b60029190612506565b1215611a48576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024015b60405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff83165f9081526001602052604081208054849290611a7c908490613ffc565b90915550508015611c2b577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611ae08573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611b43575f80fd5b505af1158015611b55573d5f803e3d5ffd5b50611b9b9250505073ffffffffffffffffffffffffffffffffffffffff84167f000000000000000000000000000000000000000000000000000000000000000083611cea565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611c05573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611c299190614060565b505b5050600190920191506119b29050565b5f68010000000000000000821061146757611467611d33565b6040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290525f9073ffffffffffffffffffffffffffffffffffffffff841690631e2eaeaf90602401602060405180830381865afa158015611cbf573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ce39190614060565b9392505050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716611d2a576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b5f8163ffffffff841611611d8f576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611a3f565b602083901c6044830201611ce3565b5f8073ffffffffffffffffffffffffffffffffffffffff8516611e0a575f84815260208790526040812060081c7f20000000000000000000000000000000000000000000000000000000000000001754600281900b93506301000000900462ffffff169150611e459050565b5f611e36611e188660281b90565b73ffffffffffffffffffffffffffffffffffffffff88169086612549565b61ffff601882901c1693509150505b5f8260020b136116a7576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610c1882760a70c3c40a64e6c51999090b65f67d9240000000000000614028565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020611440611ed5825c84612587565b829061259f565b60018401935f9035811a1515611ef285826125a6565b60028601953560f01c611f19611f0885836125f7565b805160208201516040909201519092565b60020b608089015273ffffffffffffffffffffffffffffffffffffffff9081166040890152166020870190815260a090205f60108901893560801c9099506fffffffffffffffffffffffffffffffff1690505f8115612080575f611fb661053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016866110f0565b9050611fc183612657565b60e08b0152611ff08a7f00000000000000000000000000000000000000000000000000000000000000006126b2565b61203361053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016866110f0565b60808b01515f86815260086020526040902091935061207a919086907f000000000000000000000000000000000000000000000000000000000000000090859087906126d0565b506120c6565b6120c361053a73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016856110f0565b90505b5f83815260086020526040812060808b01516120e6918d91879086612773565b60208c0151919c5091506120fc908a9083612506565b50999a9950505050505050505050565b5f61215161211861298f565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a600181811615156060870152860135608090811c60208701526011870135901c604086015260238601956021013560f01c6121b26002831615156121a086846125f7565b9060051b602081188201519101519091565b73ffffffffffffffffffffffffffffffffffffffff90811660a089015216608087015250600481166121e557855f6121ef565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660c087015295505f61221b876008841615612a87565b60e089015290975090505f6122526122468867ffffffffffffffff4316610100820152610120902090565b60228801526042872090565b905061225d81612b32565b5f60108416612275576122708983612b87565b61227f565b61227f8983612bf1565b909950905061228e8382612c35565b608088015160208901516122a9918391600188161515612c7d565b5f6122b98960c0015183811c1890565b90506122dd818a60a001518b604001516122d88960ff16600116151590565b612d07565b509798975050505050505050565b60018401935f9035811a6122ff8582612d7f565b600181161515608086015260028601955f90819081903560f01c61235060088616151561232c89846125f7565b9060051b602081188201519082018051608090910151606090930151919390929190565b73ffffffffffffffffffffffffffffffffffffffff92831660c08e01529290911660a08c0152945092505050602088018835606089018190529098508210156123c5576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600283166123d457875f6123de565b60148801883560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e089015297505f61240a896004861615612a87565b6101008b01529099509050612420888a86612e32565b98505f806124318a8c888888612e76565b919c50925090505f6124448b888c612fc8565b90505f6080881661245e576124598d83612b87565b612468565b6124688d83612bf1565b909d509050601088161561249f5761248b8c610140015164ffffffffff16612fe2565b61249a818d6101200151611006565b6124a8565b6124a882612b32565b6124b28582612c35565b60a08c01516124c99082908660018c161515612c7d565b5f6124d98d60e0015183811c1890565b90506124f4818e60c00151866122d88d60ff16600116151590565b509b9c9b505050505050505050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120612541612538825c8561301c565b9250818361259f565b509392505050565b5f6020826020026001015f863c7fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811683145f510290509392505050565b81810182811215610c185763c9654ed45f526004601cfd5b80825d5050565b80151560c0830152806125cd5773fffd8963efd1fc6a506488495d951d5263988d256125d4565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b5f8163ffffffff841611612646576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611a3f565b5060c08102602083901c0192915050565b5f7f800000000000000000000000000000000000000000000000000000000000000082111561107c576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af180610277576040513d5f823e503d5ff35b6126de600284900b82613034565b92506126ee600283900b82613034565b91508260020b8260020b1315612734578260020b612718828460020b61303490919063ffffffff16565b60020b131561272f5761272f868587868686613047565b61276b565b8260020b8260020b121561276b57612750600284900b82613034565b60020b8260020b121561276b5761276b8685878686866130ea565b505050505050565b60018501945f90819035811a158015906127ec5760108801973560801c6127b48161279d89613199565b6fffffffffffffffffffffffffffffffff166131da565b886301000000015f8282546127c99190613ffc565b90915550899450506fffffffffffffffffffffffffffffffff1691506129859050565b505f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b13156128645761285f83888789856131f5565b612871565b61287183888789856132c9565b909b50995060108201965092503560801c61289e816fffffffffffffffffffffffffffffffff8b166131da565b6128a8908b613ffc565b99506128c66fffffffffffffffffffffffffffffffff821684613ffc565b92506128d2868661339e565b5f6128df835f0151613199565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff161461295a576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c16600483015282166024820152604401611a3f565b8a856301000000015f8282546129709190613ffc565b90915550969c50929a50505050505050505050505b9550959350505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f0000000000000000000000000000000000000000000000000000000000000000461416612a845750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47083612b2857843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f818152602081905260409020805c15612b78576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612b8381600161259f565b5050565b6017601483013560e81c8084018201935f92813560601c92910190612bae838684846133d7565b612be4576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d612c2e57638baa579f5f526004601cfd5b9293915050565b8115612b835763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166102775763f959fdae5f526004601cfd5b81612c8a60028583611ea3565b8115612cde5773ffffffffffffffffffffffffffffffffffffffff8086165f90815260036020908152604080832093881683529290529081208054839290612cd3908490613f79565b90915550612d009050565b612d0073ffffffffffffffffffffffffffffffffffffffff851686308461341c565b5050505050565b81612d1460028583612506565b508115612d5e5773ffffffffffffffffffffffffffffffffffffffff8086165f90815260036020908152604080832093881683529290529081208054839290612cd3908490613ffc565b612d0073ffffffffffffffffffffffffffffffffffffffff85168683611cea565b6020811615612ddd576010811615612db757507fa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e60009052565b507febd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac88619052565b6010811615612e0c57507fef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f9052565b507f5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d49052565b5f6010821615612e5c5760088361013886013760056008840161015b860137600d83019250612e6e565b67ffffffffffffffff43166101208501525b509092915050565b5f8080806020871615612f2a57508635608090811c60208a810182905260108a0135831c60408c0181905260308b019a919091013590921c9181831015612ee9576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80831115612f23576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050612f55565b5060108701963560801c60408716612f42575f612f45565b60015b60ff1660208a0152604089018190525b60408716151580612f6857506020871615155b15612f9657915081612f7a8682613474565b9150612f8f82612f8a8188613481565b61348c565b9150612fbb565b905080612fa38682613497565b9250612fb883612fb38188613481565b6134a2565b92505b5095979096509350505050565b5f61118f612fd685856134ad565b60228401526042832090565b804211156103e2576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80820382811315610c185763c9654ed45f526004601cfd5b5f611ce382808507831381860503614077565b63010000008601545b5f61307373ffffffffffffffffffffffffffffffffffffffff88168787866134cd565b95509050600284810b9086900b1380159061308b5750805b156130d3578762ffffff8616630100000081106130aa576130aa613fcf565b01546130b69083613f79565b8862ffffff8716630100000081106130d0576130d0613fcf565b01555b508260020b8460020b126130505750505050505050565b63010000008601545b5f61311673ffffffffffffffffffffffffffffffffffffffff8816878786613541565b95509050600285810b9085900b1215613177578015613172578762ffffff86166301000000811061314957613149613fcf565b01546131559083613f79565b8862ffffff87166301000000811061316f5761316f613fcf565b01555b61317d565b50613190565b846131878161409d565b955050506130f3565b50505050505050565b5f610c1873ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001683613586565b5f611ce3826131f1670de0b6b3a7640000866140f9565b0490565b5f808080600181805b821561328b5760108a01993560801c6132178184613ffc565b9250613235818b6fffffffffffffffffffffffffffffffff166131da565b61323f9083613ffc565b9150818d8d62ffffff166301000000811061325c5761325c613fcf565b015f82825461326b9190613ffc565b90915550508851613287908b90613282908f6135c0565b613602565b9950505b61329d885f01518c8a6020015161361c565b809c508194505050876040015160020b8b60020b136131fe57989b909a50979850959695505050505050565b5f808080600181805b821561335f5760108a01993560801c6132eb8184613ffc565b9250613309818b6fffffffffffffffffffffffffffffffff166131da565b6133139083613ffc565b9150818d8d62ffffff166301000000811061333057613330613fcf565b015f82825461333f9190613ffc565b9091555050885161335b908b90613356908f6135c0565b61366b565b9950505b613371885f01518c8a60200151613685565b809c508194505050876040015160020b8b60020b13156132d257989b909a50979850959695505050505050565b808214612b83576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f5114171661346757637939f4245f526004601cfd5b5f60605260405250505050565b5f611ce383835b906136ac565b5f611ce3828461347b565b5f611ce38284613f79565b5f611ce382846136ce565b5f611ce38284613ffc565b5f80601083166134bf576101406134c3565b6101605b9093209392505050565b5f8080806134f48587078213868805035b6134e9906001614110565b600281900b60081d91565b90925090506135248161351e73ffffffffffffffffffffffffffffffffffffffff8b168a866136e6565b90613721565b90945090506135348282876137e3565b9250505094509492505050565b5f8080806135568587078213868805036134e9565b90925090506135248161358073ffffffffffffffffffffffffffffffffffffffff8b168a866136e6565b9061380d565b5f8181526006602052604081205f6135b773ffffffffffffffffffffffffffffffffffffffff861660038401611c54565b95945050505050565b5f61118f73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684846138d5565b808203608081901c15610c185763c9654ed45f526004601cfd5b5f80808061363d6134e9613631600189614151565b875f8183071291050390565b9150915061364f8161358089856139ab565b909450905061365f8282876137e3565b92505050935093915050565b818101608081901c15610c185763c9654ed45f526004601cfd5b5f80808061369a8587078213868805036134de565b9150915061364f8161351e89856139ab565b5f6b033b2e3c9fd0803ce80000006136c483856140f9565b611ce39190614028565b5f816136c46b033b2e3c9fd0803ce8000000856140f9565b5f82815260066020908152604080832084845260050190915281206135b773ffffffffffffffffffffffffffffffffffffffff861682611c54565b5f805f6137bc8460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b90508061010014159250826137d25760ff6137d9565b8360ff1681015b9150509250929050565b5f8160ff84166137f9600187900b610100614077565b6138039190614110565b61118f9190614077565b5f805f8360ff0390505f6138ae8260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b90508061010014159350836138c3575f6138ca565b8160ff1681035b925050509250929050565b5f8281526006602090815260408083208484526004019091528120819081906040517f1e2eaeaf000000000000000000000000000000000000000000000000000000008152600481018290529091505f9073ffffffffffffffffffffffffffffffffffffffff881690631e2eaeaf90602401602060405180830381865afa158015613962573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906139869190614060565b6fffffffffffffffffffffffffffffffff81169860809190911d975095505050505050565b5f611ce373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001684846136e6565b5f8083601f8401126139fd575f80fd5b50813567ffffffffffffffff811115613a14575f80fd5b602083019150836020828501011115612bea575f80fd5b5f8060208385031215613a3c575f80fd5b823567ffffffffffffffff811115613a52575f80fd5b613a5e858286016139ed565b90969095509350505050565b73ffffffffffffffffffffffffffffffffffffffff811681146103e2575f80fd5b803562ffffff81168114613a9d575f80fd5b919050565b5f805f8060808587031215613ab5575f80fd5b8435613ac081613a6a565b93506020850135613ad081613a6a565b9250604085013561ffff81168114613ae6575f80fd5b9150613af460608601613a8b565b905092959194509250565b5f60208284031215613b0f575f80fd5b813567ffffffffffffffff81168114611ce3575f80fd5b5f60a08284031215613b36575f80fd5b50919050565b5f805f805f858703610160811215613b52575f80fd5b8635613b5d81613a6a565b9550613b6c8860208901613b26565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215613b9d575f80fd5b5060c08601925061014086013567ffffffffffffffff811115613bbe575f80fd5b613bca888289016139ed565b969995985093965092949392505050565b5f805f805f6101008688031215613bf0575f80fd5b8535613bfb81613a6a565b9450613c0a8760208801613b26565b935060c0860135613c1a81613a6a565b925060e086013567ffffffffffffffff811115613bbe575f80fd5b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f613cbb60e0830189613c35565b8281036040840152613ccd8189613c35565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015613d2f578351835260209384019390920191600101613d11565b50909b9a5050505050505050505050565b602081525f611ce36020830184613c35565b5f8060208385031215613d63575f80fd5b823567ffffffffffffffff811115613d79575f80fd5b8301601f81018513613d89575f80fd5b803567ffffffffffffffff811115613d9f575f80fd5b8560208260051b8401011115613db3575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f60208284031215613e4c575f80fd5b815167ffffffffffffffff811115613e62575f80fd5b8201601f81018413613e72575f80fd5b805167ffffffffffffffff811115613e8c57613e8c613e0f565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff82111715613ef857613ef8613e0f565b604052818152828201602001861015613f0f575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f60208284031215613f3c575f80fd5b81358060020b8114611ce3575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115610c1857610c18613f4c565b5f60208284031215613f9c575f80fd5b8135611ce381613a6a565b6fffffffffffffffffffffffffffffffff8281168282160390811115610c1857610c18613f4c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115610c1857610c18613f4c565b5f6020828403121561401f575f80fd5b611ce382613a8b565b5f8261405b577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b5f60208284031215614070575f80fd5b5051919050565b5f8260020b8260020b028060020b915080821461409657614096613f4c565b5092915050565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000081036140d1576140d1613f4c565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b8082028115828204841417610c1857610c18613f4c565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715610c1857610c18613f4c565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715610c1857610c18613f4c56fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\x9FW_5`\xE0\x1C\x80c%\x99\x82\xE5\x11a\0rW\x80c\x84\xB0\x19n\x11a\0XW\x80c\x84\xB0\x19n\x14a\x01MW\x80c\x91\xDDsF\x14a\x01hW\x80c\xC6\xA9\x8E\xB9\x14a\x01\x88W_\x80\xFD[\x80c%\x99\x82\xE5\x14a\x01'W\x80c4@\xD8 \x14a\x01:W_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\0\xA3W\x80c\x10\x90d\x1D\x14a\0\xB8W\x80c\x11jUP\x14a\0\xCBW\x80c!\xD0\xEEp\x14a\0\xDEW[_\x80\xFD[a\0\xB6a\0\xB16`\x04a:+V[a\x01\x9BV[\0[a\0\xB6a\0\xC66`\x04a:\xA2V[a\x02|V[a\0\xB6a\0\xD96`\x04a:\xFFV[a\x03\xD8V[a\0\xF1a\0\xEC6`\x04a;<V[a\x03\xE5V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xF1a\x0156`\x04a;<V[a\x06oV[a\0\xF1a\x01H6`\x04a;\xDBV[a\x08\xD7V[a\x01Ua\nKV[`@Qa\x01\x1E\x97\x96\x95\x94\x93\x92\x91\x90a<\x81V[a\x01{a\x01v6`\x04a:+V[a\n\xF3V[`@Qa\x01\x1E\x91\x90a=@V[a\0\xB6a\x01\x966`\x04a=RV[a\x0C\x1EV[a\x01\xA3a\r\x19V[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x02\x17\x90\x85\x90\x85\x90`\x04\x01a=\xC3V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x022W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x02w\x91\x90\x81\x01\x90a><V[PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x02\xEBW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x03PW`@Q\x7F\x9F8\xAF\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x81R` \x84\x90R`@\x90 `\x06Ta\x03\x91\x90`\x05\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x86\x86a\r\xECV[`\x06`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[a\x03\xE23\x82a\x10\x06V[PV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04UW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x04c\x85`@\x015a\x10AV[\x90P_a\x04o\x87a\x10\x82V[\x90P_\x80a\x04\xF1\x83\x8Ba\x04\x85` \x8C\x01\x8Ca?,V[a\x04\x95`@\x8D\x01` \x8E\x01a?,V[`@\x80Q`\x06\x92\x90\x92R`\x03\x92\x90\x92R_\x92\x83R``\x8D\x81\x015`&R`:`\x0C \x94\x84R` \x85\x90R`\x07\x83R\x90\x92 \x91\x90R`\x08\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[\x90\x92P\x90P_a\x05Ca\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x10\xF0V[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\x05|\x82a\x05W` \x8D\x01\x8Da?,V[a\x05g`@\x8E\x01` \x8F\x01a?,V[_\x89\x81R`\x08` R`@\x90 \x92\x91\x90a\x11\x97V[\x90P_a\x05\xC0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x127V[\x85T\x90\x91P_\x90a\x05\xE3\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x12|V[a\x05\xED\x91\x90a?yV[\x90Pa\x06\x0B\x8E\x8E_\x01` \x81\x01\x90a\x06\x05\x91\x90a?\x8CV[\x83a\x12\xA7V[a\x06:a\x06\x17\x89a\x14FV[a\x06!\x90\x84a?\xA7V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12|V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xDFW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x015_a\x06\xEE\x87a\x10\x82V[\x90P_a\x06\xFE` \x88\x01\x88a?,V[\x90P_a\x07\x11`@\x89\x01` \x8A\x01a?,V[\x90P_`\x08_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x07H\x85\x8D\x86\x86\x8E``\x015`\x07a\x10\x96\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x07\x8Fa\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x10\xF0V[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xACWa\x07\xACa?\xCFV[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\xCBWa\x07\xCBa?\xCFV[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\x08\x1EW\x81\x83\x10\x15a\x08\rW\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x08\x06Wa\x08\x06a?\xCFV[\x01Ua\x08|V[a\x08\x17\x82\x84a?yV[\x90Pa\x08|V[\x83`\x02\x0B\x87`\x02\x0B\x13a\x08[W\x82\x82\x10\x15a\x08QW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\x08\x06Wa\x08\x06a?\xCFV[a\x08\x17\x83\x83a?yV[\x81\x83\x87c\x01\0\0\0\x01Ta\x08o\x91\x90a?yV[a\x08y\x91\x90a?yV[\x90P[_a\x08\x87\x82\x8Ca\x12|V[\x90P\x80\x86_\x01_\x82\x82Ta\x08\x9B\x91\x90a?\xFCV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\tGW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\tva\tX` \x88\x01\x88a?\x8CV[a\th`@\x89\x01` \x8A\x01a?\x8CV[_\x91\x82R` R`@\x90 \x90V[_\x81\x81R`\x05` R`@\x81 \x91\x92P\x90`\x08\x1C\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17T`\x02\x0B\x90P\x80a\t\xC3`\x80\x89\x01``\x8A\x01a?,V[`\x02\x0B\x14\x15\x80a\t\xE7WP_a\t\xDF``\x89\x01`@\x8A\x01a@\x0FV[b\xFF\xFF\xFF\x16\x14\x15[\x15a\n\x1EW`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x96PPPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\n\xE1`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0BdW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x83\x01_\x845\x81\x1A\x15\x80\x15\x90a\x0B\x9DW`\x06Th\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91P[P_a\x0B\xA8\x83a\x14kV[\x90\x93P\x90P_a\x0B\xBB\x84\x83`\x05\x86a\x15<V[\x90\x94P\x90Pa\x0B\xC9\x82a\x16\xB0V[a\x0B\xD5\x84`\x02\x83a\x17\xD7V[\x93Pa\x0B\xE1\x84\x82a\x18cV[\x93Pa\x0B\xED\x84\x82a\x19\x03V[\x93Pa\x0B\xFA\x84\x88\x88a\x19\x92V[a\x0C\x03\x82a\x19\xA9V[PP`@\x80Q_\x81R` \x81\x01\x90\x91R\x92PPP[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\x8DW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x02wW_\x83\x83\x83\x81\x81\x10a\x0C\xAAWa\x0C\xAAa?\xCFV[\x90P` \x02\x01` \x81\x01\x90a\x0C\xBF\x91\x90a?\x8CV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x0C\x8FV[`\x06TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\r`W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x04` R`@\x90 T`\xFF\x16a\r\xA8W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xB1Ca\x1C;V[`\x06\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x82a\xFF\xFF\x16_\x03a\x0E*W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x0ElW`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x81R` \x87\x90R`@\x81 `\x08\x1C\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x80Tb\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\x90\x91\x16a\xFF\xFF\x87\x16\x17\x17\x81U\x90P_a\x0E\xFE` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16;a@(V[\x90P_a\x0F\x0B\x87`(\x1B\x90V[\x90P`@Q` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x8A\x90\x1B\x16b\xFF\xFF\xFF\x89\x16\x17\x84\x17\x82\x82\x01\x81\x81R[\x80\x84\x10\x15a\x0F\x82W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x0FvW\x82\x85RPa\x0F\x82V[P` \x84\x01\x93Pa\x0F:V[k`\x0B8\x03\x80`\x0B_9_\xF3\0\x85R\x83\x14`\x05\x1B\x91\x90\x91\x01\x90P`\x0C\x81\x01`\x14\x84\x01_\xF0\x96PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91Pa\x0F\xFA\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x95\x94PPPPPV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\x109Wc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_\x80\x82\x13\x15a\x10|W`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`@\x80Q`\x06\x93\x90\x93R`\x03\x93\x90\x93R_\x93\x84R`&R`:`\x0C \x93\x83R` \x84\x90R\x93\x81R``\x90\x91 \x92\x90R`\x08\x91\x90\x91\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[_\x81\x81R`\x06` R`@\x81 \x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11kW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x8F\x91\x90a@`V[\x94\x93PPPPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xB3Wa\x11\xB3a?\xCFV[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xD2Wa\x11\xD2a?\xCFV[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x11\xF7Wa\x11\xEE\x81\x83a?yV[\x92PPPa\x11\x8FV[\x85`\x02\x0B\x84`\x02\x0B\x13a\x12\x0EWa\x11\xEE\x82\x82a?yV[\x80\x82\x88c\x01\0\0\0\x01Ta\x12\"\x91\x90a?yV[a\x12,\x91\x90a?yV[\x97\x96PPPPPPPV[_\x82\x81R`\x06` \x81\x81R`@\x80\x84 \x85\x85R\x90\x92\x01\x90R\x81 _a\x12rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x83a\x1CTV[\x96\x95PPPPPPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x12\x97Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x80_\x03a\x12\xB3WPPPV[`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x139W_\x80\xFD[PZ\xF1\x15\x80\x15a\x13KW=_\x80>=_\xFD[Pa\x13\x91\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1C\xEAV[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14\x1CW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14@\x91\x90a@`V[PPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x14gWa\x14ga\x1D3V[P\x90V[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x14\x89\x86\x84a?yV[a\x14\x93\x91\x90a@(V[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x150W_a\x14\xBF` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x15&W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x14\xA1V[P\x82\x94PPPP\x91P\x91V[`\x03\x84\x81\x01\x94_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x15^\x8B\x84a?yV[a\x15h\x91\x90a@(V[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x16\xA1W`\x04\x8A\x01\x995`\xE0\x81\x90\x1C\x90_\x90a\x15\xAE\x90a\x14\xB8\x90\x8D\x90`\xF0\x1Ca\x1D@V[\x90P_a\x15\xC2a\x14\xB8\x8Da\xFF\xFF\x86\x16a\x1D@V[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x16\x0BWP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x16BW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8C\x01\x9B\x91\x92P\x905`\xF0\x1C_\x80a\x16i\x8C\x8C\x86\x86a\x1D\x9EV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP` \x8B\x01\x9A5\x90P_a\x16\x8B\x82a\x1E\x81V[`\x80\x87\x01RP`\xA0\x85\x01R`\xC0\x90\x93\x01\x92a\x15\x86V[P\x93PPP[\x94P\x94\x92PPPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02wW`D\x81\x02` \x84\x90\x1C\x01`\x14\x81\x015`\x80\x1C\x815``\x1C\x81\x15a\x17\xCCW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0B\r\x9C\ta\x177\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\xA7W_\x80\xFD[PZ\xF1\x15\x80\x15a\x17\xB9W=_\x80>=_\xFD[Pa\x17\xCC\x92P`\x02\x91P\x83\x90P\x84a\x1E\xA3V[PPP`\x01\x01a\x16\xB9V[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x85\x81\x01\x95\x805`\xE8\x1C\x01\x01\x90[\x81\x86\x14a\x18YWa\x18R\x86\x82\x87\x87a\x1E\xDCV[\x95Pa\x18?V[P\x93\x94\x93PPPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x18}a!\x0CV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x7F\xD2\x9C\xD8\xAD\x13\x068\xCED\xEA\x18VH\xD2q#E\xE8\xE4V\xFE\xF8\x153(\xEE@\x91m\xF2y\x0E\x81R\x90\x91P[\x82\x86\x14a\x18YWa\x18\xFC\x86\x82\x84\x88a!VV[\x95Pa\x18\xE9V[_\x80a\x19\ra!\x0CV[`@\x80Qa\x01`\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a\x18YWa\x19\x8B\x86\x83\x85\x88a\"\xEBV[\x95Pa\x19xV[\x80\x82\x01\x80\x84\x14a\x14@Wc\x01\x84/\x8C_R`\x04`\x1C\xFD[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02wW`D\x81\x02` \x84\x90\x1C\x01\x805``\x1C`$\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a\x19\xF3\x84a\x19\xEA\x84\x86a?\xFCV[`\x02\x91\x90a%\x06V[\x12\x15a\x1AHW`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x84\x92\x90a\x1A|\x90\x84\x90a?\xFCV[\x90\x91UPP\x80\x15a\x1C+W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1A\xE0\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1BCW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1BUW=_\x80>=_\xFD[Pa\x1B\x9B\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1C\xEAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1C\x05W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C)\x91\x90a@`V[P[PP`\x01\x90\x92\x01\x91Pa\x19\xB2\x90PV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x14gWa\x14ga\x1D3V[`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xBFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C\xE3\x91\x90a@`V[\x93\x92PPPV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1D*Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a\x1D\x8FW`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1A?V[` \x83\x90\x1C`D\x83\x02\x01a\x1C\xE3V[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x1E\nW_\x84\x81R` \x87\x90R`@\x81 `\x08\x1C\x7F \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17T`\x02\x81\x90\x0B\x93Pc\x01\0\0\0\x90\x04b\xFF\xFF\xFF\x16\x91Pa\x1EE\x90PV[_a\x1E6a\x1E\x18\x86`(\x1B\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90\x86a%IV[a\xFF\xFF`\x18\x82\x90\x1C\x16\x93P\x91PP[_\x82`\x02\x0B\x13a\x16\xA7W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0C\x18\x82v\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0a@(V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x14@a\x1E\xD5\x82\\\x84a%\x87V[\x82\x90a%\x9FV[`\x01\x84\x01\x93_\x905\x81\x1A\x15\x15a\x1E\xF2\x85\x82a%\xA6V[`\x02\x86\x01\x955`\xF0\x1Ca\x1F\x19a\x1F\x08\x85\x83a%\xF7V[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x89\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x89\x01R\x16` \x87\x01\x90\x81R`\xA0\x90 _`\x10\x89\x01\x895`\x80\x1C\x90\x99Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a \x80W_a\x1F\xB6a\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x10\xF0V[\x90Pa\x1F\xC1\x83a&WV[`\xE0\x8B\x01Ra\x1F\xF0\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a&\xB2V[a 3a\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x10\xF0V[`\x80\x8B\x01Q_\x86\x81R`\x08` R`@\x90 \x91\x93Pa z\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a&\xD0V[Pa \xC6V[a \xC3a\x05:s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x10\xF0V[\x90P[_\x83\x81R`\x08` R`@\x81 `\x80\x8B\x01Qa \xE6\x91\x8D\x91\x87\x90\x86a'sV[` \x8C\x01Q\x91\x9CP\x91Pa \xFC\x90\x8A\x90\x83a%\x06V[P\x99\x9A\x99PPPPPPPPPPV[_a!Qa!\x18a)\x8FV[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15``\x87\x01R\x86\x015`\x80\x90\x81\x1C` \x87\x01R`\x11\x87\x015\x90\x1C`@\x86\x01R`#\x86\x01\x95`!\x015`\xF0\x1Ca!\xB2`\x02\x83\x16\x15\x15a!\xA0\x86\x84a%\xF7V[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x89\x01R\x16`\x80\x87\x01RP`\x04\x81\x16a!\xE5W\x85_a!\xEFV[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x87\x01R\x95P_a\"\x1B\x87`\x08\x84\x16\x15a*\x87V[`\xE0\x89\x01R\x90\x97P\x90P_a\"Ra\"F\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01Ra\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90Pa\"]\x81a+2V[_`\x10\x84\x16a\"uWa\"p\x89\x83a+\x87V[a\"\x7FV[a\"\x7F\x89\x83a+\xF1V[\x90\x99P\x90Pa\"\x8E\x83\x82a,5V[`\x80\x88\x01Q` \x89\x01Qa\"\xA9\x91\x83\x91`\x01\x88\x16\x15\x15a,}V[_a\"\xB9\x89`\xC0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa\"\xDD\x81\x8A`\xA0\x01Q\x8B`@\x01Qa\"\xD8\x89`\xFF\x16`\x01\x16\x15\x15\x90V[a-\x07V[P\x97\x98\x97PPPPPPPPV[`\x01\x84\x01\x93_\x905\x81\x1Aa\"\xFF\x85\x82a-\x7FV[`\x01\x81\x16\x15\x15`\x80\x86\x01R`\x02\x86\x01\x95_\x90\x81\x90\x81\x905`\xF0\x1Ca#P`\x08\x86\x16\x15\x15a#,\x89\x84a%\xF7V[\x90`\x05\x1B` \x81\x18\x82\x01Q\x90\x82\x01\x80Q`\x80\x90\x91\x01Q``\x90\x93\x01Q\x91\x93\x90\x92\x91\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\xC0\x8E\x01R\x92\x90\x91\x16`\xA0\x8C\x01R\x94P\x92PPP` \x88\x01\x885``\x89\x01\x81\x90R\x90\x98P\x82\x10\x15a#\xC5W`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x83\x16a#\xD4W\x87_a#\xDEV[`\x14\x88\x01\x885``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x89\x01R\x97P_a$\n\x89`\x04\x86\x16\x15a*\x87V[a\x01\0\x8B\x01R\x90\x99P\x90Pa$ \x88\x8A\x86a.2V[\x98P_\x80a$1\x8A\x8C\x88\x88\x88a.vV[\x91\x9CP\x92P\x90P_a$D\x8B\x88\x8Ca/\xC8V[\x90P_`\x80\x88\x16a$^Wa$Y\x8D\x83a+\x87V[a$hV[a$h\x8D\x83a+\xF1V[\x90\x9DP\x90P`\x10\x88\x16\x15a$\x9FWa$\x8B\x8Ca\x01@\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a/\xE2V[a$\x9A\x81\x8Da\x01 \x01Qa\x10\x06V[a$\xA8V[a$\xA8\x82a+2V[a$\xB2\x85\x82a,5V[`\xA0\x8C\x01Qa$\xC9\x90\x82\x90\x86`\x01\x8C\x16\x15\x15a,}V[_a$\xD9\x8D`\xE0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa$\xF4\x81\x8E`\xC0\x01Q\x86a\"\xD8\x8D`\xFF\x16`\x01\x16\x15\x15\x90V[P\x9B\x9C\x9BPPPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a%Aa%8\x82\\\x85a0\x1CV[\x92P\x81\x83a%\x9FV[P\x93\x92PPPV[_` \x82` \x02`\x01\x01_\x86<\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x83\x14_Q\x02\x90P\x93\x92PPPV[\x81\x81\x01\x82\x81\x12\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[\x80\x15\x15`\xC0\x83\x01R\x80a%\xCDWs\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a%\xD4V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a&FW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1A?V[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x10|W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x02wW`@Q=_\x82>P=_\xF3[a&\xDE`\x02\x84\x90\x0B\x82a04V[\x92Pa&\xEE`\x02\x83\x90\x0B\x82a04V[\x91P\x82`\x02\x0B\x82`\x02\x0B\x13\x15a'4W\x82`\x02\x0Ba'\x18\x82\x84`\x02\x0Ba04\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a'/Wa'/\x86\x85\x87\x86\x86\x86a0GV[a'kV[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a'kWa'P`\x02\x84\x90\x0B\x82a04V[`\x02\x0B\x82`\x02\x0B\x12\x15a'kWa'k\x86\x85\x87\x86\x86\x86a0\xEAV[PPPPPPV[`\x01\x85\x01\x94_\x90\x81\x905\x81\x1A\x15\x80\x15\x90a'\xECW`\x10\x88\x01\x975`\x80\x1Ca'\xB4\x81a'\x9D\x89a1\x99V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\xDAV[\x88c\x01\0\0\0\x01_\x82\x82Ta'\xC9\x91\x90a?\xFCV[\x90\x91UP\x89\x94PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91Pa)\x85\x90PV[P_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a(dWa(_\x83\x88\x87\x89\x85a1\xF5V[a(qV[a(q\x83\x88\x87\x89\x85a2\xC9V[\x90\x9BP\x99P`\x10\x82\x01\x96P\x92P5`\x80\x1Ca(\x9E\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16a1\xDAV[a(\xA8\x90\x8Ba?\xFCV[\x99Pa(\xC6o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84a?\xFCV[\x92Pa(\xD2\x86\x86a3\x9EV[_a(\xDF\x83_\x01Qa1\x99V[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a)ZW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x1A?V[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta)p\x91\x90a?\xFCV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x95P\x95\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a*\x84WP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a+(W\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a+xW`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a+\x83\x81`\x01a%\x9FV[PPV[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90a+\xAE\x83\x86\x84\x84a3\xD7V[a+\xE4W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a,.Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81\x15a+\x83Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x02wWc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x81a,\x8A`\x02\x85\x83a\x1E\xA3V[\x81\x15a,\xDEWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a,\xD3\x90\x84\x90a?yV[\x90\x91UPa-\0\x90PV[a-\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a4\x1CV[PPPPPV[\x81a-\x14`\x02\x85\x83a%\x06V[P\x81\x15a-^Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a,\xD3\x90\x84\x90a?\xFCV[a-\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x1C\xEAV[` \x81\x16\x15a-\xDDW`\x10\x81\x16\x15a-\xB7WP\x7F\xA9\xE5)MDO\xBA\xEB\\\xA0_+$\xC5\xD8)D\x10\xF3\x14\x13\x04\x8ED]\x88\x1E.\xF6\x9E`\0\x90RV[P\x7F\xEB\xD5\t\x1C9oy\x05nEE_J\x93\xBB\xB0\x16\xC2\x171K\xB25k\"\xD1\xC183\xAC\x88a\x90RV[`\x10\x81\x16\x15a.\x0CWP\x7F\xEF\x0C\xCE\x88\xFD\xA0J\xB3\xF8F\x18\x823r\xCFv\xF6KE\x11\xCE\x8Cyc\x0E\xAB\xC2\x9E\xBC\x9B\x96\x8F\x90RV[P\x7F[\x9DI\xCF\xEDH\xF8\xC9\xD1\xA8c\xF6\x1C$y\xC5y\xFA)\x9C%\xA32\x11\xFC\x85s\x90\xCE\xCC\xE6\xD4\x90RV[_`\x10\x82\x16\x15a.\\W`\x08\x83a\x018\x86\x017`\x05`\x08\x84\x01a\x01[\x86\x017`\r\x83\x01\x92Pa.nV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01 \x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x87\x16\x15a/*WP\x865`\x80\x90\x81\x1C` \x8A\x81\x01\x82\x90R`\x10\x8A\x015\x83\x1C`@\x8C\x01\x81\x90R`0\x8B\x01\x9A\x91\x90\x91\x015\x90\x92\x1C\x91\x81\x83\x10\x15a.\xE9W`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a/#W`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa/UV[P`\x10\x87\x01\x965`\x80\x1C`@\x87\x16a/BW_a/EV[`\x01[`\xFF\x16` \x8A\x01R`@\x89\x01\x81\x90R[`@\x87\x16\x15\x15\x80a/hWP` \x87\x16\x15\x15[\x15a/\x96W\x91P\x81a/z\x86\x82a4tV[\x91Pa/\x8F\x82a/\x8A\x81\x88a4\x81V[a4\x8CV[\x91Pa/\xBBV[\x90P\x80a/\xA3\x86\x82a4\x97V[\x92Pa/\xB8\x83a/\xB3\x81\x88a4\x81V[a4\xA2V[\x92P[P\x95\x97\x90\x96P\x93PPPPV[_a\x11\x8Fa/\xD6\x85\x85a4\xADV[`\"\x84\x01R`B\x83 \x90V[\x80B\x11\x15a\x03\xE2W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_a\x1C\xE3\x82\x80\x85\x07\x83\x13\x81\x86\x05\x03a@wV[c\x01\0\0\0\x86\x01T[_a0ss\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x87\x87\x86a4\xCDV[\x95P\x90P`\x02\x84\x81\x0B\x90\x86\x90\x0B\x13\x80\x15\x90a0\x8BWP\x80[\x15a0\xD3W\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a0\xAAWa0\xAAa?\xCFV[\x01Ta0\xB6\x90\x83a?yV[\x88b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a0\xD0Wa0\xD0a?\xCFV[\x01U[P\x82`\x02\x0B\x84`\x02\x0B\x12a0PWPPPPPPPV[c\x01\0\0\0\x86\x01T[_a1\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x87\x87\x86a5AV[\x95P\x90P`\x02\x85\x81\x0B\x90\x85\x90\x0B\x12\x15a1wW\x80\x15a1rW\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a1IWa1Ia?\xCFV[\x01Ta1U\x90\x83a?yV[\x88b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a1oWa1oa?\xCFV[\x01U[a1}V[Pa1\x90V[\x84a1\x87\x81a@\x9DV[\x95PPPa0\xF3V[PPPPPPPV[_a\x0C\x18s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x83a5\x86V[_a\x1C\xE3\x82a1\xF1g\r\xE0\xB6\xB3\xA7d\0\0\x86a@\xF9V[\x04\x90V[_\x80\x80\x80`\x01\x81\x80[\x82\x15a2\x8BW`\x10\x8A\x01\x995`\x80\x1Ca2\x17\x81\x84a?\xFCV[\x92Pa25\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\xDAV[a2?\x90\x83a?\xFCV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a2\\Wa2\\a?\xCFV[\x01_\x82\x82Ta2k\x91\x90a?\xFCV[\x90\x91UPP\x88Qa2\x87\x90\x8B\x90a2\x82\x90\x8Fa5\xC0V[a6\x02V[\x99PP[a2\x9D\x88_\x01Q\x8C\x8A` \x01Qa6\x1CV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a1\xFEW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80\x80\x80`\x01\x81\x80[\x82\x15a3_W`\x10\x8A\x01\x995`\x80\x1Ca2\xEB\x81\x84a?\xFCV[\x92Pa3\t\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1\xDAV[a3\x13\x90\x83a?\xFCV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a30Wa30a?\xCFV[\x01_\x82\x82Ta3?\x91\x90a?\xFCV[\x90\x91UPP\x88Qa3[\x90\x8B\x90a3V\x90\x8Fa5\xC0V[a6kV[\x99PP[a3q\x88_\x01Q\x8C\x8A` \x01Qa6\x85V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a2\xD2W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a+\x83W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a4gWcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[_a\x1C\xE3\x83\x83[\x90a6\xACV[_a\x1C\xE3\x82\x84a4{V[_a\x1C\xE3\x82\x84a?yV[_a\x1C\xE3\x82\x84a6\xCEV[_a\x1C\xE3\x82\x84a?\xFCV[_\x80`\x10\x83\x16a4\xBFWa\x01@a4\xC3V[a\x01`[\x90\x93 \x93\x92PPPV[_\x80\x80\x80a4\xF4\x85\x87\x07\x82\x13\x86\x88\x05\x03[a4\xE9\x90`\x01aA\x10V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa5$\x81a5\x1Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a6\xE6V[\x90a7!V[\x90\x94P\x90Pa54\x82\x82\x87a7\xE3V[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80a5V\x85\x87\x07\x82\x13\x86\x88\x05\x03a4\xE9V[\x90\x92P\x90Pa5$\x81a5\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a6\xE6V[\x90a8\rV[_\x81\x81R`\x06` R`@\x81 _a5\xB7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a\x1CTV[\x95\x94PPPPPV[_a\x11\x8Fs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a8\xD5V[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a6=a4\xE9a61`\x01\x89aAQV[\x87_\x81\x83\x07\x12\x91\x05\x03\x90V[\x91P\x91Pa6O\x81a5\x80\x89\x85a9\xABV[\x90\x94P\x90Pa6_\x82\x82\x87a7\xE3V[\x92PPP\x93P\x93\x91PPV[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x0C\x18Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a6\x9A\x85\x87\x07\x82\x13\x86\x88\x05\x03a4\xDEV[\x91P\x91Pa6O\x81a5\x1E\x89\x85a9\xABV[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a6\xC4\x83\x85a@\xF9V[a\x1C\xE3\x91\x90a@(V[_\x81a6\xC4k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85a@\xF9V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a5\xB7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a\x1CTV[_\x80_a7\xBC\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a7\xD2W`\xFFa7\xD9V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a7\xF9`\x01\x87\x90\x0Ba\x01\0a@wV[a8\x03\x91\x90aA\x10V[a\x11\x8F\x91\x90a@wV[_\x80_\x83`\xFF\x03\x90P_a8\xAE\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a8\xC3W_a8\xCAV[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x04\x01\x90\x91R\x81 \x81\x90\x81\x90`@Q\x7F\x1E.\xAE\xAF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x82\x90R\x90\x91P_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90c\x1E.\xAE\xAF\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9bW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x86\x91\x90a@`V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x98`\x80\x91\x90\x91\x1D\x97P\x95PPPPPPV[_a\x1C\xE3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x84\x84a6\xE6V[_\x80\x83`\x1F\x84\x01\x12a9\xFDW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x14W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a+\xEAW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a:<W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:RW_\x80\xFD[a:^\x85\x82\x86\x01a9\xEDV[\x90\x96\x90\x95P\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xE2W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a:\x9DW_\x80\xFD[\x91\x90PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a:\xB5W_\x80\xFD[\x845a:\xC0\x81a:jV[\x93P` \x85\x015a:\xD0\x81a:jV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14a:\xE6W_\x80\xFD[\x91Pa:\xF4``\x86\x01a:\x8BV[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15a;\x0FW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1C\xE3W_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a;6W_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15a;RW_\x80\xFD[\x865a;]\x81a:jV[\x95Pa;l\x88` \x89\x01a;&V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15a;\x9DW_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xBEW_\x80\xFD[a;\xCA\x88\x82\x89\x01a9\xEDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a;\xF0W_\x80\xFD[\x855a;\xFB\x81a:jV[\x94Pa<\n\x87` \x88\x01a;&V[\x93P`\xC0\x86\x015a<\x1A\x81a:jV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xBEW_\x80\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_a<\xBB`\xE0\x83\x01\x89a<5V[\x82\x81\x03`@\x84\x01Ra<\xCD\x81\x89a<5V[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15a=/W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a=\x11V[P\x90\x9B\x9APPPPPPPPPPPV[` \x81R_a\x1C\xE3` \x83\x01\x84a<5V[_\x80` \x83\x85\x03\x12\x15a=cW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=yW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a=\x89W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\x9FW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a=\xB3W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a>LW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>bW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>rW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x8CWa>\x8Ca>\x0FV[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a>\xF8Wa>\xF8a>\x0FV[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15a?\x0FW_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15a?<W_\x80\xFD[\x815\x80`\x02\x0B\x81\x14a\x1C\xE3W_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\x18Wa\x0C\x18a?LV[_` \x82\x84\x03\x12\x15a?\x9CW_\x80\xFD[\x815a\x1C\xE3\x81a:jV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\x18Wa\x0C\x18a?LV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\x18Wa\x0C\x18a?LV[_` \x82\x84\x03\x12\x15a@\x1FW_\x80\xFD[a\x1C\xE3\x82a:\x8BV[_\x82a@[W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[_` \x82\x84\x03\x12\x15a@pW_\x80\xFD[PQ\x91\x90PV[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14a@\x96Wa@\x96a?LV[P\x92\x91PPV[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03a@\xD1Wa@\xD1a?LV[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\x18Wa\x0C\x18a?LV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x0C\x18Wa\x0C\x18a?LV[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x0C\x18Wa\x0C\x18a?LV\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
        beforeAddLiquidity(beforeAddLiquidityCall),
        beforeInitialize(beforeInitializeCall),
        beforeRemoveLiquidity(beforeRemoveLiquidityCall),
        configurePool(configurePoolCall),
        eip712Domain(eip712DomainCall),
        execute(executeCall),
        govToggleNodes(govToggleNodesCall),
        invalidateNonce(invalidateNonceCall),
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
            [16u8, 144u8, 100u8, 29u8],
            [17u8, 106u8, 85u8, 80u8],
            [33u8, 208u8, 238u8, 112u8],
            [37u8, 153u8, 130u8, 229u8],
            [52u8, 64u8, 216u8, 32u8],
            [132u8, 176u8, 25u8, 110u8],
            [145u8, 221u8, 115u8, 70u8],
            [198u8, 169u8, 142u8, 185u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromCalls {
        const NAME: &'static str = "AngstromCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 9usize;
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
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::govToggleNodes(_) => {
                    <govToggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::invalidateNonce(_) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn configurePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <configurePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::configurePool)
                    }
                    configurePool
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
                    fn beforeRemoveLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::beforeRemoveLiquidity)
                    }
                    beforeRemoveLiquidity
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
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        AssetAccessOutOfBounds(AssetAccessOutOfBounds),
        AssetsOutOfOrderOrNotUnique(AssetsOutOfOrderOrNotUnique),
        AssetsUnsorted(AssetsUnsorted),
        BundleChangeNetNegative(BundleChangeNetNegative),
        Expired(Expired),
        FailedToDeployNewStore(FailedToDeployNewStore),
        FeeAboveMax(FeeAboveMax),
        FillingTooLittle(FillingTooLittle),
        FillingTooMuch(FillingTooMuch),
        InvalidPoolKey(InvalidPoolKey),
        InvalidSignature(InvalidSignature),
        InvalidTickSpacing(InvalidTickSpacing),
        LimitViolated(LimitViolated),
        MissingHookPermissions(MissingHookPermissions),
        NonceReuse(NonceReuse),
        NotController(NotController),
        NotNode(NotNode),
        NotUniswap(NotUniswap),
        OnlyOncePerBlock(OnlyOncePerBlock),
        OrderAlreadyExecuted(OrderAlreadyExecuted),
        OutOfOrderOrDuplicatePairs(OutOfOrderOrDuplicatePairs),
        Overflow(Overflow),
        PairAccessOutOfBounds(PairAccessOutOfBounds),
        ReaderNotAtEnd(ReaderNotAtEnd),
        Underflow(Underflow),
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
            [39u8, 8u8, 21u8, 160u8],
            [53u8, 39u8, 141u8, 18u8],
            [68u8, 24u8, 35u8, 49u8],
            [86u8, 112u8, 37u8, 135u8],
            [92u8, 210u8, 107u8, 104u8],
            [100u8, 41u8, 207u8, 210u8],
            [117u8, 56u8, 50u8, 40u8],
            [118u8, 163u8, 249u8, 93u8],
            [128u8, 241u8, 26u8, 207u8],
            [138u8, 46u8, 241u8, 22u8],
            [139u8, 170u8, 87u8, 159u8],
            [140u8, 184u8, 136u8, 114u8],
            [142u8, 30u8, 223u8, 164u8],
            [159u8, 56u8, 175u8, 184u8],
            [194u8, 86u8, 98u8, 43u8],
            [196u8, 218u8, 240u8, 3u8],
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
    impl alloy_sol_types::SolInterface for AngstromErrors {
        const NAME: &'static str = "AngstromErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
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
                Self::Expired(_) => <Expired as alloy_sol_types::SolError>::SELECTOR,
                Self::FailedToDeployNewStore(_) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::SELECTOR
                }
                Self::FeeAboveMax(_) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
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
                Self::InvalidTickSpacing(_) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::SELECTOR
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
                    fn InvalidTickSpacing(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidTickSpacing as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidTickSpacing)
                    }
                    InvalidTickSpacing
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
                    fn FailedToDeployNewStore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FailedToDeployNewStore)
                    }
                    FailedToDeployNewStore
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
                    fn FeeAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <FeeAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::FeeAboveMax)
                    }
                    FeeAboveMax
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
                    fn AssetsUnsorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <AssetsUnsorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::AssetsUnsorted)
                    }
                    AssetsUnsorted
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
                    fn Underflow(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <Underflow as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::Underflow)
                    }
                    Underflow
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
                    fn PairAccessOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <PairAccessOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::PairAccessOutOfBounds)
                    }
                    PairAccessOutOfBounds
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
                {
                    fn AssetAccessOutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <AssetAccessOutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::AssetAccessOutOfBounds)
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
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::FailedToDeployNewStore(inner) => {
                    <FailedToDeployNewStore as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::FeeAboveMax(inner) => {
                    <FeeAboveMax as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidTickSpacing(inner) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::Expired(inner) => {
                    <Expired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InvalidTickSpacing(inner) => {
                    <InvalidTickSpacing as alloy_sol_types::SolError>::abi_encode_raw(
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
        ///Creates a new call builder for the [`invalidateNonce`] function.
        pub fn invalidateNonce(
            &self,
            nonce: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, invalidateNonceCall, N> {
            self.call_builder(&invalidateNonceCall { nonce })
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
