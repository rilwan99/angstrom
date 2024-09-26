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
    error NoEntry();
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

    constructor(address uniV4PoolManager, address controller);

    function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeInitialize(address, PoolKey memory poolKey, uint160, bytes memory storeIndexBytes) external view returns (bytes4);
    function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 feeInE6) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function execute(bytes memory encoded) external;
    function extsload(bytes32 slot) external view returns (bytes32);
    function invalidateNonce(uint64 nonce) external;
    function toggleNodes(address[] memory nodes) external;
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
    "name": "extsload",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "NoEntry",
    "inputs": []
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
    ///0x610160604052348015610010575f80fd5b50604051614c80380380614c8083398101604081905261002f9161016f565b306080524660a052808260608061007a6040805180820182526008815267416e677374726f6d60c01b60208083019190915282518084019093526002835261763160f01b9083015291565b815160209283012081519183019190912060c082905260e0819052604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8152938401929092529082015246606082015230608082015260a090206101005250506001600160a01b039081166101205216610140526100fc61208061010e565b610107610a0061010e565b50506101a0565b806001600160a01b03168130166001600160a01b03161461015157604051630ea7064560e31b81526001600160a01b038216600482015260240160405180910390fd5b50565b80516001600160a01b038116811461016a575f80fd5b919050565b5f8060408385031215610180575f80fd5b61018983610154565b915061019760208401610154565b90509250929050565b60805160a05160c05160e0516101005161012051610140516149d96102a75f395f81816102cf0152610cca01525f818161021b015281816103ce015281816104e0015281816105650152818161065401528181610735015281816108bc01528181610bb50152818161131d01528181611391015281816113fb015281816119cf01528181611d4801528181611e3501528181611e5c0152818161219a015281816121d6015281816122170152818161225b015281816122a7015281816129fb01528181612c76015281816138a3015281816139d301528181613b570152613bb201525f612df101525f612eab01525f612e8501525f612e3501525f612e1201526149d95ff3fe608060405234801561000f575f80fd5b50600436106100b9575f3560e01c8063259982e51161007257806384b0196e1161005857806384b0196e1461018857806391dd7346146101a3578063d6cffd1e146101c3575f80fd5b8063259982e5146101625780633440d82014610175575f80fd5b8063116a5550116100a2578063116a5550146100e55780631e2eaeaf146100f857806321d0ee701461011e575f80fd5b806309c5eabe146100bd5780631090641d146100d2575b5f80fd5b6100d06100cb366004614107565b6101d6565b005b6100d06100e036600461417e565b6102b7565b6100d06100f33660046141db565b61039e565b61010b610106366004614202565b6103ab565b6040519081526020015b60405180910390f35b61013161012c36600461422f565b6103b5565b6040517fffffffff000000000000000000000000000000000000000000000000000000009091168152602001610115565b61013161017036600461422f565b61063b565b6101316101833660046142ce565b6108a3565b610190610af3565b6040516101159796959493929190614374565b6101b66101b1366004614107565b610b9b565b6040516101159190614433565b6100d06101d1366004614445565b610cb2565b6101de610dad565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c894919061025290859085906004016144b6565b5f604051808303815f875af115801561026d573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526102b2919081019061452f565b505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610326576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6005546103589068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff1685858585610e80565b600560086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b6103a83382611087565b50565b5f81545f5260205ff35b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610425576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61043385604001356110c2565b90505f61043f87611103565b90505f806104bd838b61045560208c018c61461f565b61046560408d0160208e0161461f565b6040805160069283526003939093525f93845260608e810135602652603a600c2095855260208690529181529220915260081c7f10000000000000000000000000000000000000000000000000000000000000001791565b90925090505f61050f61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611171565b60a01c60020b90565b90505f6105488261052360208d018d61461f565b61053360408e0160208f0161461f565b5f8981526007602052604090209291906111a6565b90505f61058c73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168786611246565b85549091505f906105af846fffffffffffffffffffffffffffffffff85166112a1565b6105b9919061466c565b90506105d78e8e5f0160208101906105d1919061467f565b836112cc565b6106066105e38961146b565b6105ed908461469a565b84906fffffffffffffffffffffffffffffffff166112a1565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106ab576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408401355f6106ba87611103565b90505f6106ca602088018861461f565b90505f6106dd6040890160208a0161461f565b90505f60075f8581526020019081526020015f2090505f610714858d86868e6060013560066111179095949392919063ffffffff16565b5090505f61075b61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611171565b90505f8362ffffff871663010000008110610778576107786146c2565b015490505f8462ffffff871663010000008110610797576107976146c2565b015490505f8760020b8460020b12156107ea57818310156107d95781925082865f018962ffffff16630100000081106107d2576107d26146c2565b0155610848565b6107e3828461466c565b9050610848565b8360020b8760020b13610827578282101561081d57829150818662ffffff8916630100000081106107d2576107d26146c2565b6107e3838361466c565b8183876301000000015461083b919061466c565b610845919061466c565b90505b5f610853828c6112a1565b905080865f015f82825461086791906146ef565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610913576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610939610924602088018861467f565b6109346040890160208a0161467f565b611490565b90505f6109798261094a8688614702565b60055468010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16919060f01c6114e8565b5090506109ba6040518060400160405280600c81526020017f476f7420706f6f6c206b6579000000000000000000000000000000000000000081525061156b565b6109f86040518060400160405280601581526020017f436f6d706172696e67206b65792073706163696e67000000000000000000000081525061156b565b610a13610a0b6080890160608a0161461f565b60020b6115fa565b610a1f8160020b6115fa565b610a5660405180606001604052806027815260200161494d60279139610a4b60608a0160408b01614768565b62ffffff165f61168b565b600281900b610a6b6080890160608a0161461f565b60020b141580610a8f57505f610a876060890160408a01614768565b62ffffff1614155b15610ac6576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d82000000000000000000000000000000000000000000000000000000000979650505050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083610b89604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b60603373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610c0c576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b825f610c178261171e565b60055491935091505f90610c50908490849068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff166117ef565b9093509050610c5e8261198a565b610c6a83600283611a96565b9250610c768382611b22565b9250610c828382611bc2565b9250610c8f838787611c51565b610c9882611c68565b5050604080515f8152602081019091529150505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610d21576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b818110156102b2575f838383818110610d3e57610d3e6146c2565b9050602002016020810190610d53919061467f565b73ffffffffffffffffffffffffffffffffffffffff165f90815260046020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610d23565b6005544367ffffffffffffffff90911603610df4576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526004602052604090205460ff16610e3c576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610e4543611efa565b600580547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b5f8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1611610ee6576040517f9f38afb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f03610f23576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff83161115610f65576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610f858773ffffffffffffffffffffffffffffffffffffffff16611f13565b90505f610f928787611490565b90506040516020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156110065783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081168703610ffa5782855250611006565b50602084019350610fbe565b9081526b600b380380600b5f395ff3008452821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff8516915061107c9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b80600c5263daa050e9600452815f52601f600c20600160ff83161b808254188181166110ba57638cb888725f526004601cfd5b909155505050565b5f808213156110fd576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b604080516006939093526003939093525f938452602652603a600c209383526020849052938152606090912092905260089190911c7f10000000000000000000000000000000000000000000000000000000000000001791565b5f81815260066020526040812061119e73ffffffffffffffffffffffffffffffffffffffff851682611f36565b949350505050565b5f808562ffffff8516630100000081106111c2576111c26146c2565b015490505f8662ffffff8516630100000081106111e1576111e16146c2565b015490508460020b8660020b1215611206576111fd818361466c565b9250505061119e565b8560020b8460020b1361121d576111fd828261466c565b80828863010000000154611231919061466c565b61123b919061466c565b979650505050505050565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6112855763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f190483118202156112bc5763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b805f036112d857505050565b6040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a5841194906024015f604051808303815f87803b15801561135e575f80fd5b505af1158015611370573d5f803e3d5ffd5b506113b69250505073ffffffffffffffffffffffffffffffffffffffff83167f000000000000000000000000000000000000000000000000000000000000000083611f66565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af1158015611441573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114659190614781565b50505050565b5f700100000000000000000000000000000000821061148c5761148c611faf565b5090565b5f602883836040516020016114c892919073ffffffffffffffffffffffffffffffffffffffff92831681529116602082015260400190565b60405160208183030381529060405280519060200120901b905092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811685140280611557576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6103a88160405160240161157f9190614433565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052611fbc565b6103a88160405160240161161091815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f4e0c1d1d00000000000000000000000000000000000000000000000000000000179052611fbc565b6102b28383836040516024016116a393929190614798565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f969cdd0300000000000000000000000000000000000000000000000000000000179052611fbc565b6003818101915f918291803560e81c010181604461173c868461466c565b61174691906147bc565b905080602086901b1792505f805b828110156117e3575f611772602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16116117d9576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611754565b50829450505050915091565b6003838101935f91829182918291803560e81c01018160266118118a8461466c565b61181b91906147bc565b905060405193508060c0028401925082604052808460201b179450505f5b8284101561197d5760048901983560e081901c905f906118619061176b908c9060f01c611fc5565b90505f61187561176b8c61ffff8616611fc5565b90508363ffffffff168363ffffffff161115806118be57508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b156118f5576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f8061193473ffffffffffffffffffffffffffffffffffffffff8c1685856114e8565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611839565b5093505050935093915050565b63ffffffff81165f5b818110156102b25760448102602084901c01601481013560801c8015611a8c57813560601c73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016630b0d9c09826040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604481018590526064015f604051808303815f87803b158015611a65575f80fd5b505af1158015611a77573d5f803e3d5ffd5b50611a8a92506002915083905084612026565b505b5050600101611993565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81523060a082015261012080820152600385810195803560e81c0101905b818614611b1857611b118682878761205f565b9550611afe565b5093949350505050565b6003828101925f91813560e81c9091010181611b3c61231d565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101919091527fd29cd8ad130638ce44ea185648d2712345e8e456fef8153328ee40916df2790e81529091505b828614611b1857611bbb86828488612367565b9550611ba8565b5f80611bcc61231d565b60408051610160810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101919091526003868101969293509091803560e81c01015b808614611b1857611c4a868385886124fc565b9550611c37565b808201808414611465576301842f8c5f526004601cfd5b63ffffffff81165f5b818110156102b25760448102602084901c01803560601c6024820135608090811c906034840135901c5f611cb284611ca984866146ef565b60029190612717565b1215611d07576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024015b60405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff83165f9081526001602052604081208054849290611d3b9084906146ef565b90915550508015611eea577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611d9f8573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611e02575f80fd5b505af1158015611e14573d5f803e3d5ffd5b50611e5a9250505073ffffffffffffffffffffffffffffffffffffffff84167f000000000000000000000000000000000000000000000000000000000000000083611f66565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611ec4573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ee89190614781565b505b505060019092019150611c719050565b5f68010000000000000000821061148c5761148c611faf565b5f610cac602073ffffffffffffffffffffffffffffffffffffffff84163b6147bc565b5f81602052631e2eaeaf5f5260205f6024601c865afa611f5d5763535cf94b5f526004601cfd5b50505f51919050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716611fa6576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b6103a88161275a565b5f8163ffffffff841611612014576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611cfe565b602083901c60448302015b9392505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020611465612058825c8461277a565b8290612792565b6001848101945f919035821a9061207b90869083161515612799565b60408051808201909152600d81527f70616972496e6465783a20257300000000000000000000000000000000000000602082015260028701963560f01c906120c390826127ea565b6120e56120d48561ffff841661287f565b805160208201516040909201519092565b60020b608089015273ffffffffffffffffffffffffffffffffffffffff9081166040890152166020870190815260a090205f60108901893560801c60408051808201909152600c81527f616d6f756e74496e3a20257300000000000000000000000000000000000000006020820152919a506fffffffffffffffffffffffffffffffff16915061217590826127ea565b5f811561228a575f6121c061050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611171565b90506121cb836128df565b60e08b01526121fa8a7f000000000000000000000000000000000000000000000000000000000000000061293a565b61223d61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611171565b60808b01515f868152600760205260409020919350612284919086907f00000000000000000000000000000000000000000000000000000000000000009085908790612958565b506122d0565b6122cd61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611171565b90505b5f6122f76002871615155f86815260076020526040902060808d01518e91908890876129e1565b60208c0151919c50915061230d908a9083612717565b50999a9950505050505050505050565b5f612362612329612def565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a600181811615156060870152860135608090811c60208701526011870135901c604086015260238601956021013560f01c6123c36002831615156123b1868461287f565b9060051b602081188201519101519091565b73ffffffffffffffffffffffffffffffffffffffff90811660a089015216608087015250600481166123f657855f612400565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660c087015295505f61242c876008841615612ee7565b60e089015290975090505f6124636124578867ffffffffffffffff4316610100820152610120902090565b60228801526042872090565b905061246e81612f92565b5f60108416612486576124818983612fe3565b612490565b612490898361304d565b909950905061249f8382613091565b608088015160208901516124ba9183916001881615156130d9565b5f6124ca8960c0015183811c1890565b90506124ee818a60a001518b604001516124e98960ff16600116151590565b613163565b509798975050505050505050565b60018401935f9035811a61251085826131db565b600181161515608086015260028601955f90819081903560f01c61256160088616151561253d898461287f565b9060051b602081188201519082018051608090910151606090930151919390929190565b73ffffffffffffffffffffffffffffffffffffffff92831660c08e01529290911660a08c0152945092505050602088018835606089018190529098508210156125d6576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600283166125e557875f6125ef565b60148801883560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e089015297505f61261b896004861615612ee7565b6101008b01529099509050612631888a8661328e565b98505f806126428a8c8888886132d2565b919c50925090505f6126558b888c613424565b90505f6080881661266f5761266a8d83612fe3565b612679565b6126798d8361304d565b909d50905060108816156126b05761269c8c610140015164ffffffffff1661343e565b6126ab818d6101200151611087565b6126b9565b6126b982612f92565b6126c38582613091565b60a08c01516126da9082908660018c1615156130d9565b5f6126ea8d60e0015183811c1890565b9050612705818e60c00151866124e98d60ff16600116151590565b509b9c9b505050505050505050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120612752612749825c85613478565b92508183612792565b509392505050565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b81810182811215610cac5763c9654ed45f526004601cfd5b80825d5050565b80151560c0830152806127c05773fffd8963efd1fc6a506488495d951d5263988d256127c7565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b61287b82826040516024016128009291906147f4565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9710a9d000000000000000000000000000000000000000000000000000000000179052611fbc565b5050565b5f8163ffffffff8416116128ce576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611cfe565b5060c08102602083901c0192915050565b5f7f80000000000000000000000000000000000000000000000000000000000000008211156110fd576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af1806102b2576040513d5f823e503d5ff35b8260020b8260020b131561299c578260020b612980828460020b61349090919063ffffffff16565b60020b1315612997576129978685878686866134a1565b6129d9565b8260020b8260020b12156129d9575f600284900b828107919091129082900503810260020b8260020b12156129d9576129d9868587868686613534565b505050505050565b5f808715612a835760108701963560801c612a4d81612a367f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16896135d3565b6fffffffffffffffffffffffffffffffff1661360d565b876301000000015f828254612a6291906146ef565b90915550889350506fffffffffffffffffffffffffffffffff169050612de4565b60408051808201909152600d81527f73746172745469636b3a20257300000000000000000000000000000000000000602082015260038801975f9182913560e81d90612ad390600283900b613628565b60408051808201909152600d81527f6c69717569646974793a20257300000000000000000000000000000000000000602082015260108b019a3560801c90612b1b90826127ea565b5f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b1315612b7257612b6d83888789856136b9565b612b7f565b612b7f8388878985613a9a565b60408051606081019091526035808252929d50909b50929750909350612bad9190614998602083013961156b565b60408051808201909152601381527f646f6e617465546f43757272656e743a20257300000000000000000000000000602082015260108601953560801c90612bf590826127ea565b612c23816fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1661360d565b612c2d908b6146ef565b9950612c4b6fffffffffffffffffffffffffffffffff8216846146ef565b9250612c578686613c07565b81515f90612c9c9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906135d3565b9050612cef6040518060400160405280601481526020017f63757272656e744c69717569646974793a202573000000000000000000000000815250826fffffffffffffffffffffffffffffffff166127ea565b612d406040518060400160405280601081526020017f656e644c69717569646974793a202573000000000000000000000000000000008152508b6fffffffffffffffffffffffffffffffff166127ea565b806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1614612db9576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c16600483015282166024820152604401611cfe565b8a856301000000015f828254612dcf91906146ef565b90915550969c50929a50505050505050505050505b965096945050505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f0000000000000000000000000000000000000000000000000000000000000000461416612ee45750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47083612f8857843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f818152602081905260409020805c15612fd8576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61287b816001612792565b6017601483013560e81c8084018201935f92813560601c9291019061300a83868484613c40565b613040576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d61308a57638baa579f5f526004601cfd5b9293915050565b811561287b5763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166102b25763f959fdae5f526004601cfd5b816130e660028583612026565b811561313a5773ffffffffffffffffffffffffffffffffffffffff8086165f9081526003602090815260408083209388168352929052908120805483929061312f90849061466c565b9091555061315c9050565b61315c73ffffffffffffffffffffffffffffffffffffffff8516863084613c85565b5050505050565b8161317060028583612717565b5081156131ba5773ffffffffffffffffffffffffffffffffffffffff8086165f9081526003602090815260408083209388168352929052908120805483929061312f9084906146ef565b61315c73ffffffffffffffffffffffffffffffffffffffff85168683611f66565b602081161561323957601081161561321357507fa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e60009052565b507febd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac88619052565b601081161561326857507fef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f9052565b507f5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d49052565b5f60108216156132b85760088361013886013760056008840161015b860137600d830192506132ca565b67ffffffffffffffff43166101208501525b509092915050565b5f808080602087161561338657508635608090811c60208a810182905260108a0135831c60408c0181905260308b019a919091013590921c9181831015613345576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8083111561337f576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50506133b1565b5060108701963560801c6040871661339e575f6133a1565b60015b60ff1660208a0152604089018190525b604087161515806133c457506020871615155b156133f2579150816133d68682613cdd565b91506133eb826133e68188613cea565b613cf5565b9150613417565b9050806133ff8682613d00565b92506134148361340f8188613cea565b613d0b565b92505b5095979096509350505050565b5f61119e6134328585613d16565b60228401526042832090565b804211156103a8576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80820382811315610cac5763c9654ed45f526004601cfd5b5f8183071291819005919091030290565b5f6134c473ffffffffffffffffffffffffffffffffffffffff8716868685613d36565b94509050600284810b9084900b12156134dd57506129d9565b801561352e578662ffffff8516630100000081106134fd576134fd6146c2565b01548763010000000154613511919061466c565b8762ffffff86166301000000811061352b5761352b6146c2565b01555b506134a1565b5f61355773ffffffffffffffffffffffffffffffffffffffff8716868685613da8565b94509050600283810b9085900b1361356f57506129d9565b80156135c0578662ffffff85166301000000811061358f5761358f6146c2565b015487630100000001546135a3919061466c565b8762ffffff8616630100000081106135bd576135bd6146c2565b01555b836135ca81614815565b94505050613534565b5f8181526006602052604081205f61360473ffffffffffffffffffffffffffffffffffffffff861660038401611f36565b95945050505050565b5f61201f82613624670de0b6b3a764000086614871565b0490565b61287b828260405160240161363e9291906147f4565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167faf7faa3800000000000000000000000000000000000000000000000000000000179052611fbc565b5f805f805f600190505f806137026040518060400160405280600f81526020017f696e205f72657761726441626f7665000000000000000000000000000000000081525061156b565b6137536040518060400160405280601381526020017f7374617274206c69717569646974793a202573000000000000000000000000008152508a6fffffffffffffffffffffffffffffffff166127ea565b6137956040518060400160405280601981526020017f547279696e6720746f20726577617264207469636b3a202573000000000000008152508c60020b613628565b8215613973576137d96040518060400160405280601381526020017f5469636b20697320696e697469616c697a65640000000000000000000000000081525061156b565b60408051808201909152601581527f476f742072657761726420616d6f756e743a2025730000000000000000000000602082015260108b019a3560801c9061382190826127ea565b61382b81846146ef565b9250613849818b6fffffffffffffffffffffffffffffffff1661360d565b61385390836146ef565b9150818d8d62ffffff1663010000008110613870576138706146c2565b015f82825461387f91906146ef565b909155505088515f906138ca9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613ded565b91505061390f6040518060400160405280601081526020017f6e65744c69717569646974793a2025730000000000000000000000000000000081525082600f0b613628565b6139198b82613e52565b9a5061396c6040518060400160405280601181526020017f6e6577206c69717569646974793a2025730000000000000000000000000000008152508c6fffffffffffffffffffffffffffffffff166127ea565b50506139b1565b6139b16040518060400160405280601781526020017f5469636b206973206e6f7420696e697469616c697a656400000000000000000081525061156b565b875160208901516139fb9173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613e6c565b809c508194505050876040015160020b8b60020b1361375357613a536040518060400160405280601a81526020017f446f6e6520726577617264696e672e2020546f74616c3a202573000000000000815250836127ea565b613a87604051806060016040528060248152602001614974602491398a6fffffffffffffffffffffffffffffffff166127ea565b989b909a50979850959695505050505050565b5f805f805f600190505f80613ac660405180606001604052806023815260200161492a6023913961156b565b8215613b905760108a01993560801c613adf81846146ef565b9250613afd818b6fffffffffffffffffffffffffffffffff1661360d565b613b0790836146ef565b9150818d8d62ffffff1663010000008110613b2457613b246146c2565b015f828254613b3391906146ef565b909155505088515f90613b7e9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613ded565b915050613b8b8b82613e86565b9a5050505b87516020890151613bda9173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613d36565b809c508194505050876040015160020b8b60020b1315613ac657989b909a50979850959695505050505050565b80821461287b576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f51141716613cd057637939f4245f526004601cfd5b5f60605260405250505050565b5f61201f83835b90613ea0565b5f61201f8284613ce4565b5f61201f828461466c565b5f61201f8284613ec2565b5f61201f82846146ef565b5f8060108316613d2857610140613d2c565b6101605b9093209392505050565b5f808080613d5b613d508688078313878905036001614888565b600281900b60081d91565b9092509050613d8b81613d8573ffffffffffffffffffffffffffffffffffffffff8b168a86613eda565b90613f15565b9094509050613d9b828287613fd7565b9250505094509492505050565b5f808080613dbd858707821386880503613d50565b9092509050613d8b81613de773ffffffffffffffffffffffffffffffffffffffff8b168a86613eda565b90614001565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa613e2d5763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b808203608081901c15610cac5763c9654ed45f526004601cfd5b5f808080613dbd613d5060018789078413888a05036148c9565b818101608081901c15610cac5763c9654ed45f526004601cfd5b5f6b033b2e3c9fd0803ce8000000613eb88385614871565b61201f91906147bc565b5f81613eb86b033b2e3c9fd0803ce800000085614871565b5f828152600660209081526040808320848452600501909152812061360473ffffffffffffffffffffffffffffffffffffffff861682611f36565b5f805f613fb08460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082613fc65760ff613fcd565b8360ff1681015b9150509250929050565b5f8160ff8416613fed600187900b61010061490a565b613ff79190614888565b61119e919061490a565b5f805f8360ff0390505f6140a28260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b90508061010014159350836140b7575f6140be565b8160ff1681035b925050509250929050565b5f8083601f8401126140d9575f80fd5b50813567ffffffffffffffff8111156140f0575f80fd5b602083019150836020828501011115613046575f80fd5b5f8060208385031215614118575f80fd5b823567ffffffffffffffff81111561412e575f80fd5b61413a858286016140c9565b90969095509350505050565b73ffffffffffffffffffffffffffffffffffffffff811681146103a8575f80fd5b803562ffffff81168114614179575f80fd5b919050565b5f805f8060808587031215614191575f80fd5b843561419c81614146565b935060208501356141ac81614146565b9250604085013561ffff811681146141c2575f80fd5b91506141d060608601614167565b905092959194509250565b5f602082840312156141eb575f80fd5b813567ffffffffffffffff8116811461201f575f80fd5b5f60208284031215614212575f80fd5b5035919050565b5f60a08284031215614229575f80fd5b50919050565b5f805f805f858703610160811215614245575f80fd5b863561425081614146565b955061425f8860208901614219565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215614290575f80fd5b5060c08601925061014086013567ffffffffffffffff8111156142b1575f80fd5b6142bd888289016140c9565b969995985093965092949392505050565b5f805f805f61010086880312156142e3575f80fd5b85356142ee81614146565b94506142fd8760208801614219565b935060c086013561430d81614146565b925060e086013567ffffffffffffffff8111156142b1575f80fd5b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f6143ae60e0830189614328565b82810360408401526143c08189614328565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015614422578351835260209384019390920191600101614404565b50909b9a5050505050505050505050565b602081525f61201f6020830184614328565b5f8060208385031215614456575f80fd5b823567ffffffffffffffff81111561446c575f80fd5b8301601f8101851361447c575f80fd5b803567ffffffffffffffff811115614492575f80fd5b8560208260051b84010111156144a6575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561453f575f80fd5b815167ffffffffffffffff811115614555575f80fd5b8201601f81018413614565575f80fd5b805167ffffffffffffffff81111561457f5761457f614502565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156145eb576145eb614502565b604052818152828201602001861015614602575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f6020828403121561462f575f80fd5b81358060020b811461201f575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115610cac57610cac61463f565b5f6020828403121561468f575f80fd5b813561201f81614146565b6fffffffffffffffffffffffffffffffff8281168282160390811115610cac57610cac61463f565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115610cac57610cac61463f565b80357fffff0000000000000000000000000000000000000000000000000000000000008116906002841015614761577fffff000000000000000000000000000000000000000000000000000000000000808560020360031b1b82161691505b5092915050565b5f60208284031215614778575f80fd5b61201f82614167565b5f60208284031215614791575f80fd5b5051919050565b606081525f6147aa6060830186614328565b60208301949094525060400152919050565b5f826147ef577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b604081525f6148066040830185614328565b90508260208301529392505050565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000081036148495761484961463f565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b8082028115828204841417610cac57610cac61463f565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715610cac57610cac61463f565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715610cac57610cac61463f565b5f8260020b8260020b028060020b91508082146147615761476161463f56fe696e205f72657761726442656c6f77206275742077652073686f756c646e2774206265436f6d706172696e67206b65792066656520257320746f20657870656374656420666565202573446f6e6520726577617264696e672e202046696e616c206c69717569646974793a202573446f6e6520726577617264696e672c20747279696e6720746f206765742072657761726420666f722063757272656e74207469636ba164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01``@R4\x80\x15a\0\x10W_\x80\xFD[P`@QaL\x808\x03\x80aL\x80\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01oV[0`\x80RF`\xA0R\x80\x82``\x80a\0z`@\x80Q\x80\x82\x01\x82R`\x08\x81RgAngstrom`\xC0\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83Rav1`\xF0\x1B\x90\x83\x01R\x91V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xC0\x82\x90R`\xE0\x81\x90R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x93\x84\x01\x92\x90\x92R\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 a\x01\0RPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01 R\x16a\x01@Ra\0\xFCa \x80a\x01\x0EV[a\x01\x07a\n\0a\x01\x0EV[PPa\x01\xA0V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x810\x16`\x01`\x01`\xA0\x1B\x03\x16\x14a\x01QW`@Qc\x0E\xA7\x06E`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R`$\x01`@Q\x80\x91\x03\x90\xFD[PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01jW_\x80\xFD[\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x01\x80W_\x80\xFD[a\x01\x89\x83a\x01TV[\x91Pa\x01\x97` \x84\x01a\x01TV[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@QaI\xD9a\x02\xA7_9_\x81\x81a\x02\xCF\x01Ra\x0C\xCA\x01R_\x81\x81a\x02\x1B\x01R\x81\x81a\x03\xCE\x01R\x81\x81a\x04\xE0\x01R\x81\x81a\x05e\x01R\x81\x81a\x06T\x01R\x81\x81a\x075\x01R\x81\x81a\x08\xBC\x01R\x81\x81a\x0B\xB5\x01R\x81\x81a\x13\x1D\x01R\x81\x81a\x13\x91\x01R\x81\x81a\x13\xFB\x01R\x81\x81a\x19\xCF\x01R\x81\x81a\x1DH\x01R\x81\x81a\x1E5\x01R\x81\x81a\x1E\\\x01R\x81\x81a!\x9A\x01R\x81\x81a!\xD6\x01R\x81\x81a\"\x17\x01R\x81\x81a\"[\x01R\x81\x81a\"\xA7\x01R\x81\x81a)\xFB\x01R\x81\x81a,v\x01R\x81\x81a8\xA3\x01R\x81\x81a9\xD3\x01R\x81\x81a;W\x01Ra;\xB2\x01R_a-\xF1\x01R_a.\xAB\x01R_a.\x85\x01R_a.5\x01R_a.\x12\x01RaI\xD9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xB9W_5`\xE0\x1C\x80c%\x99\x82\xE5\x11a\0rW\x80c\x84\xB0\x19n\x11a\0XW\x80c\x84\xB0\x19n\x14a\x01\x88W\x80c\x91\xDDsF\x14a\x01\xA3W\x80c\xD6\xCF\xFD\x1E\x14a\x01\xC3W_\x80\xFD[\x80c%\x99\x82\xE5\x14a\x01bW\x80c4@\xD8 \x14a\x01uW_\x80\xFD[\x80c\x11jUP\x11a\0\xA2W\x80c\x11jUP\x14a\0\xE5W\x80c\x1E.\xAE\xAF\x14a\0\xF8W\x80c!\xD0\xEEp\x14a\x01\x1EW_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\0\xBDW\x80c\x10\x90d\x1D\x14a\0\xD2W[_\x80\xFD[a\0\xD0a\0\xCB6`\x04aA\x07V[a\x01\xD6V[\0[a\0\xD0a\0\xE06`\x04aA~V[a\x02\xB7V[a\0\xD0a\0\xF36`\x04aA\xDBV[a\x03\x9EV[a\x01\x0Ba\x01\x066`\x04aB\x02V[a\x03\xABV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x011a\x01,6`\x04aB/V[a\x03\xB5V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x01\x15V[a\x011a\x01p6`\x04aB/V[a\x06;V[a\x011a\x01\x836`\x04aB\xCEV[a\x08\xA3V[a\x01\x90a\n\xF3V[`@Qa\x01\x15\x97\x96\x95\x94\x93\x92\x91\x90aCtV[a\x01\xB6a\x01\xB16`\x04aA\x07V[a\x0B\x9BV[`@Qa\x01\x15\x91\x90aD3V[a\0\xD0a\x01\xD16`\x04aDEV[a\x0C\xB2V[a\x01\xDEa\r\xADV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x02R\x90\x85\x90\x85\x90`\x04\x01aD\xB6V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02mW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x02\xB2\x91\x90\x81\x01\x90aE/V[PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03&W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05Ta\x03X\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x0E\x80V[`\x05`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[a\x03\xA83\x82a\x10\x87V[PV[_\x81T_R` _\xF3[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04%W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x043\x85`@\x015a\x10\xC2V[\x90P_a\x04?\x87a\x11\x03V[\x90P_\x80a\x04\xBD\x83\x8Ba\x04U` \x8C\x01\x8CaF\x1FV[a\x04e`@\x8D\x01` \x8E\x01aF\x1FV[`@\x80Q`\x06\x92\x83R`\x03\x93\x90\x93R_\x93\x84R``\x8E\x81\x015`&R`:`\x0C \x95\x85R` \x86\x90R\x91\x81R\x92 \x91R`\x08\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[\x90\x92P\x90P_a\x05\x0Fa\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11qV[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\x05H\x82a\x05#` \x8D\x01\x8DaF\x1FV[a\x053`@\x8E\x01` \x8F\x01aF\x1FV[_\x89\x81R`\x07` R`@\x90 \x92\x91\x90a\x11\xA6V[\x90P_a\x05\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x12FV[\x85T\x90\x91P_\x90a\x05\xAF\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x12\xA1V[a\x05\xB9\x91\x90aFlV[\x90Pa\x05\xD7\x8E\x8E_\x01` \x81\x01\x90a\x05\xD1\x91\x90aF\x7FV[\x83a\x12\xCCV[a\x06\x06a\x05\xE3\x89a\x14kV[a\x05\xED\x90\x84aF\x9AV[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12\xA1V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xABW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x015_a\x06\xBA\x87a\x11\x03V[\x90P_a\x06\xCA` \x88\x01\x88aF\x1FV[\x90P_a\x06\xDD`@\x89\x01` \x8A\x01aF\x1FV[\x90P_`\x07_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x07\x14\x85\x8D\x86\x86\x8E``\x015`\x06a\x11\x17\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x07[a\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x11qV[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07xWa\x07xaF\xC2V[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\x97Wa\x07\x97aF\xC2V[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\x07\xEAW\x81\x83\x10\x15a\x07\xD9W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x07\xD2Wa\x07\xD2aF\xC2V[\x01Ua\x08HV[a\x07\xE3\x82\x84aFlV[\x90Pa\x08HV[\x83`\x02\x0B\x87`\x02\x0B\x13a\x08'W\x82\x82\x10\x15a\x08\x1DW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\x07\xD2Wa\x07\xD2aF\xC2V[a\x07\xE3\x83\x83aFlV[\x81\x83\x87c\x01\0\0\0\x01Ta\x08;\x91\x90aFlV[a\x08E\x91\x90aFlV[\x90P[_a\x08S\x82\x8Ca\x12\xA1V[\x90P\x80\x86_\x01_\x82\x82Ta\x08g\x91\x90aF\xEFV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x13W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t9a\t$` \x88\x01\x88aF\x7FV[a\t4`@\x89\x01` \x8A\x01aF\x7FV[a\x14\x90V[\x90P_a\ty\x82a\tJ\x86\x88aG\x02V[`\x05Th\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90`\xF0\x1Ca\x14\xE8V[P\x90Pa\t\xBA`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FGot pool key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[a\t\xF8`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7FComparing key spacing\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[a\n\x13a\n\x0B`\x80\x89\x01``\x8A\x01aF\x1FV[`\x02\x0Ba\x15\xFAV[a\n\x1F\x81`\x02\x0Ba\x15\xFAV[a\nV`@Q\x80``\x01`@R\x80`'\x81R` \x01aIM`'\x919a\nK``\x8A\x01`@\x8B\x01aGhV[b\xFF\xFF\xFF\x16_a\x16\x8BV[`\x02\x81\x90\x0Ba\nk`\x80\x89\x01``\x8A\x01aF\x1FV[`\x02\x0B\x14\x15\x80a\n\x8FWP_a\n\x87``\x89\x01`@\x8A\x01aGhV[b\xFF\xFF\xFF\x16\x14\x15[\x15a\n\xC6W`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x96PPPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x0B\x89`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\x0CW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_a\x0C\x17\x82a\x17\x1EV[`\x05T\x91\x93P\x91P_\x90a\x0CP\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17\xEFV[\x90\x93P\x90Pa\x0C^\x82a\x19\x8AV[a\x0Cj\x83`\x02\x83a\x1A\x96V[\x92Pa\x0Cv\x83\x82a\x1B\"V[\x92Pa\x0C\x82\x83\x82a\x1B\xC2V[\x92Pa\x0C\x8F\x83\x87\x87a\x1CQV[a\x0C\x98\x82a\x1ChV[PP`@\x80Q_\x81R` \x81\x01\x90\x91R\x91PP[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r!W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x02\xB2W_\x83\x83\x83\x81\x81\x10a\r>Wa\r>aF\xC2V[\x90P` \x02\x01` \x81\x01\x90a\rS\x91\x90aF\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\r#V[`\x05TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\r\xF4W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x04` R`@\x90 T`\xFF\x16a\x0E<W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EECa\x1E\xFAV[`\x05\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0E\xE6W`@Q\x7F\x9F8\xAF\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x0F#W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x0FeW`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\x85\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\x13V[\x90P_a\x0F\x92\x87\x87a\x14\x90V[\x90P`@Q` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x10\x06W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x0F\xFAW\x82\x85RPa\x10\x06V[P` \x84\x01\x93Pa\x0F\xBEV[\x90\x81Rk`\x0B8\x03\x80`\x0B_9_\xF3\0\x84R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x10|\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\x10\xBAWc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_\x80\x82\x13\x15a\x10\xFDW`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`@\x80Q`\x06\x93\x90\x93R`\x03\x93\x90\x93R_\x93\x84R`&R`:`\x0C \x93\x83R` \x84\x90R\x93\x81R``\x90\x91 \x92\x90R`\x08\x91\x90\x91\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[_\x81\x81R`\x06` R`@\x81 a\x11\x9Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a\x1F6V[\x94\x93PPPPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xC2Wa\x11\xC2aF\xC2V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xE1Wa\x11\xE1aF\xC2V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x12\x06Wa\x11\xFD\x81\x83aFlV[\x92PPPa\x11\x9EV[\x85`\x02\x0B\x84`\x02\x0B\x13a\x12\x1DWa\x11\xFD\x82\x82aFlV[\x80\x82\x88c\x01\0\0\0\x01Ta\x121\x91\x90aFlV[a\x12;\x91\x90aFlV[\x97\x96PPPPPPPV[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x12\x85WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x12\xBCWc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x80_\x03a\x12\xD8WPPPV[`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13^W_\x80\xFD[PZ\xF1\x15\x80\x15a\x13pW=_\x80>=_\xFD[Pa\x13\xB6\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1FfV[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14e\x91\x90aG\x81V[PPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x14\x8CWa\x14\x8Ca\x1F\xAFV[P\x90V[_`(\x83\x83`@Q` \x01a\x14\xC8\x92\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x1B\x90P\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x15WW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[a\x03\xA8\x81`@Q`$\x01a\x15\x7F\x91\x90aD3V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[a\x03\xA8\x81`@Q`$\x01a\x16\x10\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FN\x0C\x1D\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[a\x02\xB2\x83\x83\x83`@Q`$\x01a\x16\xA3\x93\x92\x91\x90aG\x98V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x96\x9C\xDD\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x17<\x86\x84aFlV[a\x17F\x91\x90aG\xBCV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x17\xE3W_a\x17r` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x17\xD9W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x17TV[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x18\x11\x8A\x84aFlV[a\x18\x1B\x91\x90aG\xBCV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x19}W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x18a\x90a\x17k\x90\x8C\x90`\xF0\x1Ca\x1F\xC5V[\x90P_a\x18ua\x17k\x8Ca\xFF\xFF\x86\x16a\x1F\xC5V[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x18\xBEWP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x18\xF5W`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x194s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x14\xE8V[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x189V[P\x93PPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02\xB2W`D\x81\x02` \x84\x90\x1C\x01`\x14\x81\x015`\x80\x1C\x80\x15a\x1A\x8CW\x815``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0B\r\x9C\t\x82`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AeW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1AwW=_\x80>=_\xFD[Pa\x1A\x8A\x92P`\x02\x91P\x83\x90P\x84a &V[P[PP`\x01\x01a\x19\x93V[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x85\x81\x01\x95\x805`\xE8\x1C\x01\x01\x90[\x81\x86\x14a\x1B\x18Wa\x1B\x11\x86\x82\x87\x87a _V[\x95Pa\x1A\xFEV[P\x93\x94\x93PPPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x1B<a#\x1DV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x7F\xD2\x9C\xD8\xAD\x13\x068\xCED\xEA\x18VH\xD2q#E\xE8\xE4V\xFE\xF8\x153(\xEE@\x91m\xF2y\x0E\x81R\x90\x91P[\x82\x86\x14a\x1B\x18Wa\x1B\xBB\x86\x82\x84\x88a#gV[\x95Pa\x1B\xA8V[_\x80a\x1B\xCCa#\x1DV[`@\x80Qa\x01`\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a\x1B\x18Wa\x1CJ\x86\x83\x85\x88a$\xFCV[\x95Pa\x1C7V[\x80\x82\x01\x80\x84\x14a\x14eWc\x01\x84/\x8C_R`\x04`\x1C\xFD[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02\xB2W`D\x81\x02` \x84\x90\x1C\x01\x805``\x1C`$\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a\x1C\xB2\x84a\x1C\xA9\x84\x86aF\xEFV[`\x02\x91\x90a'\x17V[\x12\x15a\x1D\x07W`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x84\x92\x90a\x1D;\x90\x84\x90aF\xEFV[\x90\x91UPP\x80\x15a\x1E\xEAW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1D\x9F\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\x02W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\x14W=_\x80>=_\xFD[Pa\x1EZ\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1FfV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\xC4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xE8\x91\x90aG\x81V[P[PP`\x01\x90\x92\x01\x91Pa\x1Cq\x90PV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x14\x8CWa\x14\x8Ca\x1F\xAFV[_a\x0C\xAC` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;aG\xBCV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa\x1F]WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1F\xA6Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[a\x03\xA8\x81a'ZV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a \x14W`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1C\xFEV[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x14ea X\x82\\\x84a'zV[\x82\x90a'\x92V[`\x01\x84\x81\x01\x94_\x91\x905\x82\x1A\x90a {\x90\x86\x90\x83\x16\x15\x15a'\x99V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FpairIndex: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x02\x87\x01\x965`\xF0\x1C\x90a \xC3\x90\x82a'\xEAV[a \xE5a \xD4\x85a\xFF\xFF\x84\x16a(\x7FV[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x89\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x89\x01R\x16` \x87\x01\x90\x81R`\xA0\x90 _`\x10\x89\x01\x895`\x80\x1C`@\x80Q\x80\x82\x01\x90\x91R`\x0C\x81R\x7FamountIn: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x9APo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91Pa!u\x90\x82a'\xEAV[_\x81\x15a\"\x8AW_a!\xC0a\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11qV[\x90Pa!\xCB\x83a(\xDFV[`\xE0\x8B\x01Ra!\xFA\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a):V[a\"=a\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11qV[`\x80\x8B\x01Q_\x86\x81R`\x07` R`@\x90 \x91\x93Pa\"\x84\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a)XV[Pa\"\xD0V[a\"\xCDa\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x11qV[\x90P[_a\"\xF7`\x02\x87\x16\x15\x15_\x86\x81R`\x07` R`@\x90 `\x80\x8D\x01Q\x8E\x91\x90\x88\x90\x87a)\xE1V[` \x8C\x01Q\x91\x9CP\x91Pa#\r\x90\x8A\x90\x83a'\x17V[P\x99\x9A\x99PPPPPPPPPPV[_a#ba#)a-\xEFV[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15``\x87\x01R\x86\x015`\x80\x90\x81\x1C` \x87\x01R`\x11\x87\x015\x90\x1C`@\x86\x01R`#\x86\x01\x95`!\x015`\xF0\x1Ca#\xC3`\x02\x83\x16\x15\x15a#\xB1\x86\x84a(\x7FV[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x89\x01R\x16`\x80\x87\x01RP`\x04\x81\x16a#\xF6W\x85_a$\0V[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x87\x01R\x95P_a$,\x87`\x08\x84\x16\x15a.\xE7V[`\xE0\x89\x01R\x90\x97P\x90P_a$ca$W\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01Ra\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90Pa$n\x81a/\x92V[_`\x10\x84\x16a$\x86Wa$\x81\x89\x83a/\xE3V[a$\x90V[a$\x90\x89\x83a0MV[\x90\x99P\x90Pa$\x9F\x83\x82a0\x91V[`\x80\x88\x01Q` \x89\x01Qa$\xBA\x91\x83\x91`\x01\x88\x16\x15\x15a0\xD9V[_a$\xCA\x89`\xC0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa$\xEE\x81\x8A`\xA0\x01Q\x8B`@\x01Qa$\xE9\x89`\xFF\x16`\x01\x16\x15\x15\x90V[a1cV[P\x97\x98\x97PPPPPPPPV[`\x01\x84\x01\x93_\x905\x81\x1Aa%\x10\x85\x82a1\xDBV[`\x01\x81\x16\x15\x15`\x80\x86\x01R`\x02\x86\x01\x95_\x90\x81\x90\x81\x905`\xF0\x1Ca%a`\x08\x86\x16\x15\x15a%=\x89\x84a(\x7FV[\x90`\x05\x1B` \x81\x18\x82\x01Q\x90\x82\x01\x80Q`\x80\x90\x91\x01Q``\x90\x93\x01Q\x91\x93\x90\x92\x91\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\xC0\x8E\x01R\x92\x90\x91\x16`\xA0\x8C\x01R\x94P\x92PPP` \x88\x01\x885``\x89\x01\x81\x90R\x90\x98P\x82\x10\x15a%\xD6W`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x83\x16a%\xE5W\x87_a%\xEFV[`\x14\x88\x01\x885``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x89\x01R\x97P_a&\x1B\x89`\x04\x86\x16\x15a.\xE7V[a\x01\0\x8B\x01R\x90\x99P\x90Pa&1\x88\x8A\x86a2\x8EV[\x98P_\x80a&B\x8A\x8C\x88\x88\x88a2\xD2V[\x91\x9CP\x92P\x90P_a&U\x8B\x88\x8Ca4$V[\x90P_`\x80\x88\x16a&oWa&j\x8D\x83a/\xE3V[a&yV[a&y\x8D\x83a0MV[\x90\x9DP\x90P`\x10\x88\x16\x15a&\xB0Wa&\x9C\x8Ca\x01@\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a4>V[a&\xAB\x81\x8Da\x01 \x01Qa\x10\x87V[a&\xB9V[a&\xB9\x82a/\x92V[a&\xC3\x85\x82a0\x91V[`\xA0\x8C\x01Qa&\xDA\x90\x82\x90\x86`\x01\x8C\x16\x15\x15a0\xD9V[_a&\xEA\x8D`\xE0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa'\x05\x81\x8E`\xC0\x01Q\x86a$\xE9\x8D`\xFF\x16`\x01\x16\x15\x15\x90V[P\x9B\x9C\x9BPPPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a'Ra'I\x82\\\x85a4xV[\x92P\x81\x83a'\x92V[P\x93\x92PPPV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[\x81\x81\x01\x82\x81\x12\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[\x80\x15\x15`\xC0\x83\x01R\x80a'\xC0Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a'\xC7V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[a({\x82\x82`@Q`$\x01a(\0\x92\x91\x90aG\xF4V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97\x10\xA9\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[PPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a(\xCEW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1C\xFEV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x10\xFDW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x02\xB2W`@Q=_\x82>P=_\xF3[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a)\x9CW\x82`\x02\x0Ba)\x80\x82\x84`\x02\x0Ba4\x90\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a)\x97Wa)\x97\x86\x85\x87\x86\x86\x86a4\xA1V[a)\xD9V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a)\xD9W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a)\xD9Wa)\xD9\x86\x85\x87\x86\x86\x86a54V[PPPPPPV[_\x80\x87\x15a*\x83W`\x10\x87\x01\x965`\x80\x1Ca*M\x81a*6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a5\xD3V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[\x87c\x01\0\0\0\x01_\x82\x82Ta*b\x91\x90aF\xEFV[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa-\xE4V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FstartTick: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x03\x88\x01\x97_\x91\x82\x915`\xE8\x1D\x90a*\xD3\x90`\x02\x83\x90\x0Ba6(V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7Fliquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a+\x1B\x90\x82a'\xEAV[_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a+rWa+m\x83\x88\x87\x89\x85a6\xB9V[a+\x7FV[a+\x7F\x83\x88\x87\x89\x85a:\x9AV[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x92\x9DP\x90\x9BP\x92\x97P\x90\x93Pa+\xAD\x91\x90aI\x98` \x83\x019a\x15kV[`@\x80Q\x80\x82\x01\x90\x91R`\x13\x81R\x7FdonateToCurrent: %s\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x86\x01\x955`\x80\x1C\x90a+\xF5\x90\x82a'\xEAV[a,#\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[a,-\x90\x8BaF\xEFV[\x99Pa,Ko\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84aF\xEFV[\x92Pa,W\x86\x86a<\x07V[\x81Q_\x90a,\x9C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a5\xD3V[\x90Pa,\xEF`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7FcurrentLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[a-@`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FendLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a-\xB9W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x1C\xFEV[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta-\xCF\x91\x90aF\xEFV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a.\xE4WP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a/\x88W\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a/\xD8W`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a({\x81`\x01a'\x92V[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90a0\n\x83\x86\x84\x84a<@V[a0@W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a0\x8AWc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81\x15a({Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x02\xB2Wc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x81a0\xE6`\x02\x85\x83a &V[\x81\x15a1:Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a1/\x90\x84\x90aFlV[\x90\x91UPa1\\\x90PV[a1\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a<\x85V[PPPPPV[\x81a1p`\x02\x85\x83a'\x17V[P\x81\x15a1\xBAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a1/\x90\x84\x90aF\xEFV[a1\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x1FfV[` \x81\x16\x15a29W`\x10\x81\x16\x15a2\x13WP\x7F\xA9\xE5)MDO\xBA\xEB\\\xA0_+$\xC5\xD8)D\x10\xF3\x14\x13\x04\x8ED]\x88\x1E.\xF6\x9E`\0\x90RV[P\x7F\xEB\xD5\t\x1C9oy\x05nEE_J\x93\xBB\xB0\x16\xC2\x171K\xB25k\"\xD1\xC183\xAC\x88a\x90RV[`\x10\x81\x16\x15a2hWP\x7F\xEF\x0C\xCE\x88\xFD\xA0J\xB3\xF8F\x18\x823r\xCFv\xF6KE\x11\xCE\x8Cyc\x0E\xAB\xC2\x9E\xBC\x9B\x96\x8F\x90RV[P\x7F[\x9DI\xCF\xEDH\xF8\xC9\xD1\xA8c\xF6\x1C$y\xC5y\xFA)\x9C%\xA32\x11\xFC\x85s\x90\xCE\xCC\xE6\xD4\x90RV[_`\x10\x82\x16\x15a2\xB8W`\x08\x83a\x018\x86\x017`\x05`\x08\x84\x01a\x01[\x86\x017`\r\x83\x01\x92Pa2\xCAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01 \x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x87\x16\x15a3\x86WP\x865`\x80\x90\x81\x1C` \x8A\x81\x01\x82\x90R`\x10\x8A\x015\x83\x1C`@\x8C\x01\x81\x90R`0\x8B\x01\x9A\x91\x90\x91\x015\x90\x92\x1C\x91\x81\x83\x10\x15a3EW`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a3\x7FW`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa3\xB1V[P`\x10\x87\x01\x965`\x80\x1C`@\x87\x16a3\x9EW_a3\xA1V[`\x01[`\xFF\x16` \x8A\x01R`@\x89\x01\x81\x90R[`@\x87\x16\x15\x15\x80a3\xC4WP` \x87\x16\x15\x15[\x15a3\xF2W\x91P\x81a3\xD6\x86\x82a<\xDDV[\x91Pa3\xEB\x82a3\xE6\x81\x88a<\xEAV[a<\xF5V[\x91Pa4\x17V[\x90P\x80a3\xFF\x86\x82a=\0V[\x92Pa4\x14\x83a4\x0F\x81\x88a<\xEAV[a=\x0BV[\x92P[P\x95\x97\x90\x96P\x93PPPPV[_a\x11\x9Ea42\x85\x85a=\x16V[`\"\x84\x01R`B\x83 \x90V[\x80B\x11\x15a\x03\xA8W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a4\xC4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a=6V[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a4\xDDWPa)\xD9V[\x80\x15a5.W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a4\xFDWa4\xFDaF\xC2V[\x01T\x87c\x01\0\0\0\x01Ta5\x11\x91\x90aFlV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a5+Wa5+aF\xC2V[\x01U[Pa4\xA1V[_a5Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a=\xA8V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a5oWPa)\xD9V[\x80\x15a5\xC0W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a5\x8FWa5\x8FaF\xC2V[\x01T\x87c\x01\0\0\0\x01Ta5\xA3\x91\x90aFlV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a5\xBDWa5\xBDaF\xC2V[\x01U[\x83a5\xCA\x81aH\x15V[\x94PPPa54V[_\x81\x81R`\x06` R`@\x81 _a6\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a\x1F6V[\x95\x94PPPPPV[_a \x1F\x82a6$g\r\xE0\xB6\xB3\xA7d\0\0\x86aHqV[\x04\x90V[a({\x82\x82`@Q`$\x01a6>\x92\x91\x90aG\xF4V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xAF\x7F\xAA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[_\x80_\x80_`\x01\x90P_\x80a7\x02`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7Fin _rewardAbove\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[a7S`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7Fstart liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[a7\x95`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FTrying to reward tick: %s\0\0\0\0\0\0\0\x81RP\x8C`\x02\x0Ba6(V[\x82\x15a9sWa7\xD9`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7FTick is initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[`@\x80Q\x80\x82\x01\x90\x91R`\x15\x81R\x7FGot reward amount: %s\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a8!\x90\x82a'\xEAV[a8+\x81\x84aF\xEFV[\x92Pa8I\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[a8S\x90\x83aF\xEFV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a8pWa8paF\xC2V[\x01_\x82\x82Ta8\x7F\x91\x90aF\xEFV[\x90\x91UPP\x88Q_\x90a8\xCA\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xEDV[\x91PPa9\x0F`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FnetLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82`\x0F\x0Ba6(V[a9\x19\x8B\x82a>RV[\x9APa9l`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7Fnew liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Co\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[PPa9\xB1V[a9\xB1`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FTick is not initialized\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[\x87Q` \x89\x01Qa9\xFB\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a>lV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a7SWa:S`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FDone rewarding.  Total: %s\0\0\0\0\0\0\x81RP\x83a'\xEAV[a:\x87`@Q\x80``\x01`@R\x80`$\x81R` \x01aIt`$\x919\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80_\x80_`\x01\x90P_\x80a:\xC6`@Q\x80``\x01`@R\x80`#\x81R` \x01aI*`#\x919a\x15kV[\x82\x15a;\x90W`\x10\x8A\x01\x995`\x80\x1Ca:\xDF\x81\x84aF\xEFV[\x92Pa:\xFD\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[a;\x07\x90\x83aF\xEFV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a;$Wa;$aF\xC2V[\x01_\x82\x82Ta;3\x91\x90aF\xEFV[\x90\x91UPP\x88Q_\x90a;~\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xEDV[\x91PPa;\x8B\x8B\x82a>\x86V[\x9APPP[\x87Q` \x89\x01Qa;\xDA\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a=6V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a:\xC6W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a({W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a<\xD0Wcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[_a \x1F\x83\x83[\x90a>\xA0V[_a \x1F\x82\x84a<\xE4V[_a \x1F\x82\x84aFlV[_a \x1F\x82\x84a>\xC2V[_a \x1F\x82\x84aF\xEFV[_\x80`\x10\x83\x16a=(Wa\x01@a=,V[a\x01`[\x90\x93 \x93\x92PPPV[_\x80\x80\x80a=[a=P\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01aH\x88V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa=\x8B\x81a=\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\xDAV[\x90a?\x15V[\x90\x94P\x90Pa=\x9B\x82\x82\x87a?\xD7V[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80a=\xBD\x85\x87\x07\x82\x13\x86\x88\x05\x03a=PV[\x90\x92P\x90Pa=\x8B\x81a=\xE7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\xDAV[\x90a@\x01V[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa>-WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a=\xBDa=P`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03aH\xC9V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a>\xB8\x83\x85aHqV[a \x1F\x91\x90aG\xBCV[_\x81a>\xB8k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85aHqV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a6\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a\x1F6V[_\x80_a?\xB0\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a?\xC6W`\xFFa?\xCDV[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a?\xED`\x01\x87\x90\x0Ba\x01\0aI\nV[a?\xF7\x91\x90aH\x88V[a\x11\x9E\x91\x90aI\nV[_\x80_\x83`\xFF\x03\x90P_a@\xA2\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a@\xB7W_a@\xBEV[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a@\xD9W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xF0W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a0FW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15aA\x18W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA.W_\x80\xFD[aA:\x85\x82\x86\x01a@\xC9V[\x90\x96\x90\x95P\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xA8W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14aAyW_\x80\xFD[\x91\x90PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15aA\x91W_\x80\xFD[\x845aA\x9C\x81aAFV[\x93P` \x85\x015aA\xAC\x81aAFV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14aA\xC2W_\x80\xFD[\x91PaA\xD0``\x86\x01aAgV[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15aA\xEBW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a \x1FW_\x80\xFD[_` \x82\x84\x03\x12\x15aB\x12W_\x80\xFD[P5\x91\x90PV[_`\xA0\x82\x84\x03\x12\x15aB)W_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15aBEW_\x80\xFD[\x865aBP\x81aAFV[\x95PaB_\x88` \x89\x01aB\x19V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15aB\x90W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xB1W_\x80\xFD[aB\xBD\x88\x82\x89\x01a@\xC9V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15aB\xE3W_\x80\xFD[\x855aB\xEE\x81aAFV[\x94PaB\xFD\x87` \x88\x01aB\x19V[\x93P`\xC0\x86\x015aC\r\x81aAFV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xB1W_\x80\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_aC\xAE`\xE0\x83\x01\x89aC(V[\x82\x81\x03`@\x84\x01RaC\xC0\x81\x89aC(V[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aD\"W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aD\x04V[P\x90\x9B\x9APPPPPPPPPPPV[` \x81R_a \x1F` \x83\x01\x84aC(V[_\x80` \x83\x85\x03\x12\x15aDVW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aDlW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD|W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x92W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15aD\xA6W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aE?W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEUW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aEeW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\x7FWaE\x7FaE\x02V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aE\xEBWaE\xEBaE\x02V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15aF\x02W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aF/W_\x80\xFD[\x815\x80`\x02\x0B\x81\x14a \x1FW_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\xACWa\x0C\xACaF?V[_` \x82\x84\x03\x12\x15aF\x8FW_\x80\xFD[\x815a \x1F\x81aAFV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xACWa\x0C\xACaF?V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\xACWa\x0C\xACaF?V[\x805\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x02\x84\x10\x15aGaW\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x02\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_` \x82\x84\x03\x12\x15aGxW_\x80\xFD[a \x1F\x82aAgV[_` \x82\x84\x03\x12\x15aG\x91W_\x80\xFD[PQ\x91\x90PV[``\x81R_aG\xAA``\x83\x01\x86aC(V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82aG\xEFW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[`@\x81R_aH\x06`@\x83\x01\x85aC(V[\x90P\x82` \x83\x01R\x93\x92PPPV[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03aHIWaHIaF?V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\xACWa\x0C\xACaF?V[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x0C\xACWa\x0C\xACaF?V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x0C\xACWa\x0C\xACaF?V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14aGaWaGaaF?V\xFEin _rewardBelow but we shouldn't beComparing key fee %s to expected fee %sDone rewarding.  Final liquidity: %sDone rewarding, trying to get reward for current tick\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100b9575f3560e01c8063259982e51161007257806384b0196e1161005857806384b0196e1461018857806391dd7346146101a3578063d6cffd1e146101c3575f80fd5b8063259982e5146101625780633440d82014610175575f80fd5b8063116a5550116100a2578063116a5550146100e55780631e2eaeaf146100f857806321d0ee701461011e575f80fd5b806309c5eabe146100bd5780631090641d146100d2575b5f80fd5b6100d06100cb366004614107565b6101d6565b005b6100d06100e036600461417e565b6102b7565b6100d06100f33660046141db565b61039e565b61010b610106366004614202565b6103ab565b6040519081526020015b60405180910390f35b61013161012c36600461422f565b6103b5565b6040517fffffffff000000000000000000000000000000000000000000000000000000009091168152602001610115565b61013161017036600461422f565b61063b565b6101316101833660046142ce565b6108a3565b610190610af3565b6040516101159796959493929190614374565b6101b66101b1366004614107565b610b9b565b6040516101159190614433565b6100d06101d1366004614445565b610cb2565b6101de610dad565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c894919061025290859085906004016144b6565b5f604051808303815f875af115801561026d573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526102b2919081019061452f565b505050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610326576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6005546103589068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff1685858585610e80565b600560086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b6103a83382611087565b50565b5f81545f5260205ff35b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610425576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61043385604001356110c2565b90505f61043f87611103565b90505f806104bd838b61045560208c018c61461f565b61046560408d0160208e0161461f565b6040805160069283526003939093525f93845260608e810135602652603a600c2095855260208690529181529220915260081c7f10000000000000000000000000000000000000000000000000000000000000001791565b90925090505f61050f61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611171565b60a01c60020b90565b90505f6105488261052360208d018d61461f565b61053360408e0160208f0161461f565b5f8981526007602052604090209291906111a6565b90505f61058c73ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000168786611246565b85549091505f906105af846fffffffffffffffffffffffffffffffff85166112a1565b6105b9919061466c565b90506105d78e8e5f0160208101906105d1919061467f565b836112cc565b6106066105e38961146b565b6105ed908461469a565b84906fffffffffffffffffffffffffffffffff166112a1565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146106ab576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60408401355f6106ba87611103565b90505f6106ca602088018861461f565b90505f6106dd6040890160208a0161461f565b90505f60075f8581526020019081526020015f2090505f610714858d86868e6060013560066111179095949392919063ffffffff16565b5090505f61075b61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611171565b90505f8362ffffff871663010000008110610778576107786146c2565b015490505f8462ffffff871663010000008110610797576107976146c2565b015490505f8760020b8460020b12156107ea57818310156107d95781925082865f018962ffffff16630100000081106107d2576107d26146c2565b0155610848565b6107e3828461466c565b9050610848565b8360020b8760020b13610827578282101561081d57829150818662ffffff8916630100000081106107d2576107d26146c2565b6107e3838361466c565b8183876301000000015461083b919061466c565b610845919061466c565b90505b5f610853828c6112a1565b905080865f015f82825461086791906146ef565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b5f3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610913576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610939610924602088018861467f565b6109346040890160208a0161467f565b611490565b90505f6109798261094a8688614702565b60055468010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16919060f01c6114e8565b5090506109ba6040518060400160405280600c81526020017f476f7420706f6f6c206b6579000000000000000000000000000000000000000081525061156b565b6109f86040518060400160405280601581526020017f436f6d706172696e67206b65792073706163696e67000000000000000000000081525061156b565b610a13610a0b6080890160608a0161461f565b60020b6115fa565b610a1f8160020b6115fa565b610a5660405180606001604052806027815260200161494d60279139610a4b60608a0160408b01614768565b62ffffff165f61168b565b600281900b610a6b6080890160608a0161461f565b60020b141580610a8f57505f610a876060890160408a01614768565b62ffffff1614155b15610ac6576040517fc256622b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b507f3440d82000000000000000000000000000000000000000000000000000000000979650505050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083610b89604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b60603373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610c0c576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b825f610c178261171e565b60055491935091505f90610c50908490849068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff166117ef565b9093509050610c5e8261198a565b610c6a83600283611a96565b9250610c768382611b22565b9250610c828382611bc2565b9250610c8f838787611c51565b610c9882611c68565b5050604080515f8152602081019091529150505b92915050565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614610d21576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f5b818110156102b2575f838383818110610d3e57610d3e6146c2565b9050602002016020810190610d53919061467f565b73ffffffffffffffffffffffffffffffffffffffff165f90815260046020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff9091161517905550600101610d23565b6005544367ffffffffffffffff90911603610df4576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526004602052604090205460ff16610e3c576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610e4543611efa565b600580547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b5f8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff1611610ee6576040517f9f38afb800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f03610f23576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff83161115610f65576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f610f858773ffffffffffffffffffffffffffffffffffffffff16611f13565b90505f610f928787611490565b90506040516020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156110065783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff000000000081168703610ffa5782855250611006565b50602084019350610fbe565b9081526b600b380380600b5f395ff3008452821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff8516915061107c9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b80600c5263daa050e9600452815f52601f600c20600160ff83161b808254188181166110ba57638cb888725f526004601cfd5b909155505050565b5f808213156110fd576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b604080516006939093526003939093525f938452602652603a600c209383526020849052938152606090912092905260089190911c7f10000000000000000000000000000000000000000000000000000000000000001791565b5f81815260066020526040812061119e73ffffffffffffffffffffffffffffffffffffffff851682611f36565b949350505050565b5f808562ffffff8516630100000081106111c2576111c26146c2565b015490505f8662ffffff8516630100000081106111e1576111e16146c2565b015490508460020b8660020b1215611206576111fd818361466c565b9250505061119e565b8560020b8460020b1361121d576111fd828261466c565b80828863010000000154611231919061466c565b61123b919061466c565b979650505050505050565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6112855763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f190483118202156112bc5763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b805f036112d857505050565b6040517fa584119400000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff83811660048301527f0000000000000000000000000000000000000000000000000000000000000000169063a5841194906024015f604051808303815f87803b15801561135e575f80fd5b505af1158015611370573d5f803e3d5ffd5b506113b69250505073ffffffffffffffffffffffffffffffffffffffff83167f000000000000000000000000000000000000000000000000000000000000000083611f66565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af1158015611441573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114659190614781565b50505050565b5f700100000000000000000000000000000000821061148c5761148c611faf565b5090565b5f602883836040516020016114c892919073ffffffffffffffffffffffffffffffffffffffff92831681529116602082015260400190565b60405160208183030381529060405280519060200120901b905092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811685140280611557576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6103a88160405160240161157f9190614433565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f41304fac00000000000000000000000000000000000000000000000000000000179052611fbc565b6103a88160405160240161161091815260200190565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f4e0c1d1d00000000000000000000000000000000000000000000000000000000179052611fbc565b6102b28383836040516024016116a393929190614798565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f969cdd0300000000000000000000000000000000000000000000000000000000179052611fbc565b6003818101915f918291803560e81c010181604461173c868461466c565b61174691906147bc565b905080602086901b1792505f805b828110156117e3575f611772602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff16116117d9576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611754565b50829450505050915091565b6003838101935f91829182918291803560e81c01018160266118118a8461466c565b61181b91906147bc565b905060405193508060c0028401925082604052808460201b179450505f5b8284101561197d5760048901983560e081901c905f906118619061176b908c9060f01c611fc5565b90505f61187561176b8c61ffff8616611fc5565b90508363ffffffff168363ffffffff161115806118be57508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b156118f5576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f8061193473ffffffffffffffffffffffffffffffffffffffff8c1685856114e8565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611839565b5093505050935093915050565b63ffffffff81165f5b818110156102b25760448102602084901c01601481013560801c8015611a8c57813560601c73ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016630b0d9c09826040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff9091166004820152306024820152604481018590526064015f604051808303815f87803b158015611a65575f80fd5b505af1158015611a77573d5f803e3d5ffd5b50611a8a92506002915083905084612026565b505b5050600101611993565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81523060a082015261012080820152600385810195803560e81c0101905b818614611b1857611b118682878761205f565b9550611afe565b5093949350505050565b6003828101925f91813560e81c9091010181611b3c61231d565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e081018290526101008101919091527fd29cd8ad130638ce44ea185648d2712345e8e456fef8153328ee40916df2790e81529091505b828614611b1857611bbb86828488612367565b9550611ba8565b5f80611bcc61231d565b60408051610160810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101829052610100810182905261012081018290526101408101919091526003868101969293509091803560e81c01015b808614611b1857611c4a868385886124fc565b9550611c37565b808201808414611465576301842f8c5f526004601cfd5b63ffffffff81165f5b818110156102b25760448102602084901c01803560601c6024820135608090811c906034840135901c5f611cb284611ca984866146ef565b60029190612717565b1215611d07576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841660048201526024015b60405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff83165f9081526001602052604081208054849290611d3b9084906146ef565b90915550508015611eea577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a5841194611d9f8573ffffffffffffffffffffffffffffffffffffffff1690565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015611e02575f80fd5b505af1158015611e14573d5f803e3d5ffd5b50611e5a9250505073ffffffffffffffffffffffffffffffffffffffff84167f000000000000000000000000000000000000000000000000000000000000000083611f66565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015611ec4573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611ee89190614781565b505b505060019092019150611c719050565b5f68010000000000000000821061148c5761148c611faf565b5f610cac602073ffffffffffffffffffffffffffffffffffffffff84163b6147bc565b5f81602052631e2eaeaf5f5260205f6024601c865afa611f5d5763535cf94b5f526004601cfd5b50505f51919050565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716611fa6576390b8ec185f526004601cfd5b5f603452505050565b6335278d125f526004601cfd5b6103a88161275a565b5f8163ffffffff841611612014576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611cfe565b602083901c60448302015b9392505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020611465612058825c8461277a565b8290612792565b6001848101945f919035821a9061207b90869083161515612799565b60408051808201909152600d81527f70616972496e6465783a20257300000000000000000000000000000000000000602082015260028701963560f01c906120c390826127ea565b6120e56120d48561ffff841661287f565b805160208201516040909201519092565b60020b608089015273ffffffffffffffffffffffffffffffffffffffff9081166040890152166020870190815260a090205f60108901893560801c60408051808201909152600c81527f616d6f756e74496e3a20257300000000000000000000000000000000000000006020820152919a506fffffffffffffffffffffffffffffffff16915061217590826127ea565b5f811561228a575f6121c061050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611171565b90506121cb836128df565b60e08b01526121fa8a7f000000000000000000000000000000000000000000000000000000000000000061293a565b61223d61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611171565b60808b01515f868152600760205260409020919350612284919086907f00000000000000000000000000000000000000000000000000000000000000009085908790612958565b506122d0565b6122cd61050673ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611171565b90505b5f6122f76002871615155f86815260076020526040902060808d01518e91908890876129e1565b60208c0151919c50915061230d908a9083612717565b50999a9950505050505050505050565b5f612362612329612def565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a600181811615156060870152860135608090811c60208701526011870135901c604086015260238601956021013560f01c6123c36002831615156123b1868461287f565b9060051b602081188201519101519091565b73ffffffffffffffffffffffffffffffffffffffff90811660a089015216608087015250600481166123f657855f612400565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660c087015295505f61242c876008841615612ee7565b60e089015290975090505f6124636124578867ffffffffffffffff4316610100820152610120902090565b60228801526042872090565b905061246e81612f92565b5f60108416612486576124818983612fe3565b612490565b612490898361304d565b909950905061249f8382613091565b608088015160208901516124ba9183916001881615156130d9565b5f6124ca8960c0015183811c1890565b90506124ee818a60a001518b604001516124e98960ff16600116151590565b613163565b509798975050505050505050565b60018401935f9035811a61251085826131db565b600181161515608086015260028601955f90819081903560f01c61256160088616151561253d898461287f565b9060051b602081188201519082018051608090910151606090930151919390929190565b73ffffffffffffffffffffffffffffffffffffffff92831660c08e01529290911660a08c0152945092505050602088018835606089018190529098508210156125d6576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600283166125e557875f6125ef565b60148801883560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e089015297505f61261b896004861615612ee7565b6101008b01529099509050612631888a8661328e565b98505f806126428a8c8888886132d2565b919c50925090505f6126558b888c613424565b90505f6080881661266f5761266a8d83612fe3565b612679565b6126798d8361304d565b909d50905060108816156126b05761269c8c610140015164ffffffffff1661343e565b6126ab818d6101200151611087565b6126b9565b6126b982612f92565b6126c38582613091565b60a08c01516126da9082908660018c1615156130d9565b5f6126ea8d60e0015183811c1890565b9050612705818e60c00151866124e98d60ff16600116151590565b509b9c9b505050505050505050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120612752612749825c85613478565b92508183612792565b509392505050565b80516a636f6e736f6c652e6c6f67602083015f808483855afa5050505050565b81810182811215610cac5763c9654ed45f526004601cfd5b80825d5050565b80151560c0830152806127c05773fffd8963efd1fc6a506488495d951d5263988d256127c7565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b61287b82826040516024016128009291906147f4565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9710a9d000000000000000000000000000000000000000000000000000000000179052611fbc565b5050565b5f8163ffffffff8416116128ce576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401611cfe565b5060c08102602083901c0192915050565b5f7f80000000000000000000000000000000000000000000000000000000000000008211156110fd576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af1806102b2576040513d5f823e503d5ff35b8260020b8260020b131561299c578260020b612980828460020b61349090919063ffffffff16565b60020b1315612997576129978685878686866134a1565b6129d9565b8260020b8260020b12156129d9575f600284900b828107919091129082900503810260020b8260020b12156129d9576129d9868587868686613534565b505050505050565b5f808715612a835760108701963560801c612a4d81612a367f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16896135d3565b6fffffffffffffffffffffffffffffffff1661360d565b876301000000015f828254612a6291906146ef565b90915550889350506fffffffffffffffffffffffffffffffff169050612de4565b60408051808201909152600d81527f73746172745469636b3a20257300000000000000000000000000000000000000602082015260038801975f9182913560e81d90612ad390600283900b613628565b60408051808201909152600d81527f6c69717569646974793a20257300000000000000000000000000000000000000602082015260108b019a3560801c90612b1b90826127ea565b5f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b1315612b7257612b6d83888789856136b9565b612b7f565b612b7f8388878985613a9a565b60408051606081019091526035808252929d50909b50929750909350612bad9190614998602083013961156b565b60408051808201909152601381527f646f6e617465546f43757272656e743a20257300000000000000000000000000602082015260108601953560801c90612bf590826127ea565b612c23816fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1661360d565b612c2d908b6146ef565b9950612c4b6fffffffffffffffffffffffffffffffff8216846146ef565b9250612c578686613c07565b81515f90612c9c9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906135d3565b9050612cef6040518060400160405280601481526020017f63757272656e744c69717569646974793a202573000000000000000000000000815250826fffffffffffffffffffffffffffffffff166127ea565b612d406040518060400160405280601081526020017f656e644c69717569646974793a202573000000000000000000000000000000008152508b6fffffffffffffffffffffffffffffffff166127ea565b806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1614612db9576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c16600483015282166024820152604401611cfe565b8a856301000000015f828254612dcf91906146ef565b90915550969c50929a50505050505050505050505b965096945050505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f0000000000000000000000000000000000000000000000000000000000000000461416612ee45750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a47083612f8857843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f818152602081905260409020805c15612fd8576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61287b816001612792565b6017601483013560e81c8084018201935f92813560601c9291019061300a83868484613c40565b613040576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d61308a57638baa579f5f526004601cfd5b9293915050565b811561287b5763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166102b25763f959fdae5f526004601cfd5b816130e660028583612026565b811561313a5773ffffffffffffffffffffffffffffffffffffffff8086165f9081526003602090815260408083209388168352929052908120805483929061312f90849061466c565b9091555061315c9050565b61315c73ffffffffffffffffffffffffffffffffffffffff8516863084613c85565b5050505050565b8161317060028583612717565b5081156131ba5773ffffffffffffffffffffffffffffffffffffffff8086165f9081526003602090815260408083209388168352929052908120805483929061312f9084906146ef565b61315c73ffffffffffffffffffffffffffffffffffffffff85168683611f66565b602081161561323957601081161561321357507fa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e60009052565b507febd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac88619052565b601081161561326857507fef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f9052565b507f5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d49052565b5f60108216156132b85760088361013886013760056008840161015b860137600d830192506132ca565b67ffffffffffffffff43166101208501525b509092915050565b5f808080602087161561338657508635608090811c60208a810182905260108a0135831c60408c0181905260308b019a919091013590921c9181831015613345576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8083111561337f576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50506133b1565b5060108701963560801c6040871661339e575f6133a1565b60015b60ff1660208a0152604089018190525b604087161515806133c457506020871615155b156133f2579150816133d68682613cdd565b91506133eb826133e68188613cea565b613cf5565b9150613417565b9050806133ff8682613d00565b92506134148361340f8188613cea565b613d0b565b92505b5095979096509350505050565b5f61119e6134328585613d16565b60228401526042832090565b804211156103a8576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80820382811315610cac5763c9654ed45f526004601cfd5b5f8183071291819005919091030290565b5f6134c473ffffffffffffffffffffffffffffffffffffffff8716868685613d36565b94509050600284810b9084900b12156134dd57506129d9565b801561352e578662ffffff8516630100000081106134fd576134fd6146c2565b01548763010000000154613511919061466c565b8762ffffff86166301000000811061352b5761352b6146c2565b01555b506134a1565b5f61355773ffffffffffffffffffffffffffffffffffffffff8716868685613da8565b94509050600283810b9085900b1361356f57506129d9565b80156135c0578662ffffff85166301000000811061358f5761358f6146c2565b015487630100000001546135a3919061466c565b8762ffffff8616630100000081106135bd576135bd6146c2565b01555b836135ca81614815565b94505050613534565b5f8181526006602052604081205f61360473ffffffffffffffffffffffffffffffffffffffff861660038401611f36565b95945050505050565b5f61201f82613624670de0b6b3a764000086614871565b0490565b61287b828260405160240161363e9291906147f4565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167faf7faa3800000000000000000000000000000000000000000000000000000000179052611fbc565b5f805f805f600190505f806137026040518060400160405280600f81526020017f696e205f72657761726441626f7665000000000000000000000000000000000081525061156b565b6137536040518060400160405280601381526020017f7374617274206c69717569646974793a202573000000000000000000000000008152508a6fffffffffffffffffffffffffffffffff166127ea565b6137956040518060400160405280601981526020017f547279696e6720746f20726577617264207469636b3a202573000000000000008152508c60020b613628565b8215613973576137d96040518060400160405280601381526020017f5469636b20697320696e697469616c697a65640000000000000000000000000081525061156b565b60408051808201909152601581527f476f742072657761726420616d6f756e743a2025730000000000000000000000602082015260108b019a3560801c9061382190826127ea565b61382b81846146ef565b9250613849818b6fffffffffffffffffffffffffffffffff1661360d565b61385390836146ef565b9150818d8d62ffffff1663010000008110613870576138706146c2565b015f82825461387f91906146ef565b909155505088515f906138ca9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613ded565b91505061390f6040518060400160405280601081526020017f6e65744c69717569646974793a2025730000000000000000000000000000000081525082600f0b613628565b6139198b82613e52565b9a5061396c6040518060400160405280601181526020017f6e6577206c69717569646974793a2025730000000000000000000000000000008152508c6fffffffffffffffffffffffffffffffff166127ea565b50506139b1565b6139b16040518060400160405280601781526020017f5469636b206973206e6f7420696e697469616c697a656400000000000000000081525061156b565b875160208901516139fb9173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613e6c565b809c508194505050876040015160020b8b60020b1361375357613a536040518060400160405280601a81526020017f446f6e6520726577617264696e672e2020546f74616c3a202573000000000000815250836127ea565b613a87604051806060016040528060248152602001614974602491398a6fffffffffffffffffffffffffffffffff166127ea565b989b909a50979850959695505050505050565b5f805f805f600190505f80613ac660405180606001604052806023815260200161492a6023913961156b565b8215613b905760108a01993560801c613adf81846146ef565b9250613afd818b6fffffffffffffffffffffffffffffffff1661360d565b613b0790836146ef565b9150818d8d62ffffff1663010000008110613b2457613b246146c2565b015f828254613b3391906146ef565b909155505088515f90613b7e9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613ded565b915050613b8b8b82613e86565b9a5050505b87516020890151613bda9173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613d36565b809c508194505050876040015160020b8b60020b1315613ac657989b909a50979850959695505050505050565b80821461287b576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f51141716613cd057637939f4245f526004601cfd5b5f60605260405250505050565b5f61201f83835b90613ea0565b5f61201f8284613ce4565b5f61201f828461466c565b5f61201f8284613ec2565b5f61201f82846146ef565b5f8060108316613d2857610140613d2c565b6101605b9093209392505050565b5f808080613d5b613d508688078313878905036001614888565b600281900b60081d91565b9092509050613d8b81613d8573ffffffffffffffffffffffffffffffffffffffff8b168a86613eda565b90613f15565b9094509050613d9b828287613fd7565b9250505094509492505050565b5f808080613dbd858707821386880503613d50565b9092509050613d8b81613de773ffffffffffffffffffffffffffffffffffffffff8b168a86613eda565b90614001565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa613e2d5763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b808203608081901c15610cac5763c9654ed45f526004601cfd5b5f808080613dbd613d5060018789078413888a05036148c9565b818101608081901c15610cac5763c9654ed45f526004601cfd5b5f6b033b2e3c9fd0803ce8000000613eb88385614871565b61201f91906147bc565b5f81613eb86b033b2e3c9fd0803ce800000085614871565b5f828152600660209081526040808320848452600501909152812061360473ffffffffffffffffffffffffffffffffffffffff861682611f36565b5f805f613fb08460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082613fc65760ff613fcd565b8360ff1681015b9150509250929050565b5f8160ff8416613fed600187900b61010061490a565b613ff79190614888565b61119e919061490a565b5f805f8360ff0390505f6140a28260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b90508061010014159350836140b7575f6140be565b8160ff1681035b925050509250929050565b5f8083601f8401126140d9575f80fd5b50813567ffffffffffffffff8111156140f0575f80fd5b602083019150836020828501011115613046575f80fd5b5f8060208385031215614118575f80fd5b823567ffffffffffffffff81111561412e575f80fd5b61413a858286016140c9565b90969095509350505050565b73ffffffffffffffffffffffffffffffffffffffff811681146103a8575f80fd5b803562ffffff81168114614179575f80fd5b919050565b5f805f8060808587031215614191575f80fd5b843561419c81614146565b935060208501356141ac81614146565b9250604085013561ffff811681146141c2575f80fd5b91506141d060608601614167565b905092959194509250565b5f602082840312156141eb575f80fd5b813567ffffffffffffffff8116811461201f575f80fd5b5f60208284031215614212575f80fd5b5035919050565b5f60a08284031215614229575f80fd5b50919050565b5f805f805f858703610160811215614245575f80fd5b863561425081614146565b955061425f8860208901614219565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215614290575f80fd5b5060c08601925061014086013567ffffffffffffffff8111156142b1575f80fd5b6142bd888289016140c9565b969995985093965092949392505050565b5f805f805f61010086880312156142e3575f80fd5b85356142ee81614146565b94506142fd8760208801614219565b935060c086013561430d81614146565b925060e086013567ffffffffffffffff8111156142b1575f80fd5b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f6143ae60e0830189614328565b82810360408401526143c08189614328565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b81811015614422578351835260209384019390920191600101614404565b50909b9a5050505050505050505050565b602081525f61201f6020830184614328565b5f8060208385031215614456575f80fd5b823567ffffffffffffffff81111561446c575f80fd5b8301601f8101851361447c575f80fd5b803567ffffffffffffffff811115614492575f80fd5b8560208260051b84010111156144a6575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561453f575f80fd5b815167ffffffffffffffff811115614555575f80fd5b8201601f81018413614565575f80fd5b805167ffffffffffffffff81111561457f5761457f614502565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156145eb576145eb614502565b604052818152828201602001861015614602575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b5f6020828403121561462f575f80fd5b81358060020b811461201f575f80fd5b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b81810381811115610cac57610cac61463f565b5f6020828403121561468f575f80fd5b813561201f81614146565b6fffffffffffffffffffffffffffffffff8281168282160390811115610cac57610cac61463f565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b80820180821115610cac57610cac61463f565b80357fffff0000000000000000000000000000000000000000000000000000000000008116906002841015614761577fffff000000000000000000000000000000000000000000000000000000000000808560020360031b1b82161691505b5092915050565b5f60208284031215614778575f80fd5b61201f82614167565b5f60208284031215614791575f80fd5b5051919050565b606081525f6147aa6060830186614328565b60208301949094525060400152919050565b5f826147ef577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b604081525f6148066040830185614328565b90508260208301529392505050565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000081036148495761484961463f565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b8082028115828204841417610cac57610cac61463f565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff80000082121715610cac57610cac61463f565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff82131715610cac57610cac61463f565b5f8260020b8260020b028060020b91508082146147615761476161463f56fe696e205f72657761726442656c6f77206275742077652073686f756c646e2774206265436f6d706172696e67206b65792066656520257320746f20657870656374656420666565202573446f6e6520726577617264696e672e202046696e616c206c69717569646974793a202573446f6e6520726577617264696e672c20747279696e6720746f206765742072657761726420666f722063757272656e74207469636ba164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xB9W_5`\xE0\x1C\x80c%\x99\x82\xE5\x11a\0rW\x80c\x84\xB0\x19n\x11a\0XW\x80c\x84\xB0\x19n\x14a\x01\x88W\x80c\x91\xDDsF\x14a\x01\xA3W\x80c\xD6\xCF\xFD\x1E\x14a\x01\xC3W_\x80\xFD[\x80c%\x99\x82\xE5\x14a\x01bW\x80c4@\xD8 \x14a\x01uW_\x80\xFD[\x80c\x11jUP\x11a\0\xA2W\x80c\x11jUP\x14a\0\xE5W\x80c\x1E.\xAE\xAF\x14a\0\xF8W\x80c!\xD0\xEEp\x14a\x01\x1EW_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\0\xBDW\x80c\x10\x90d\x1D\x14a\0\xD2W[_\x80\xFD[a\0\xD0a\0\xCB6`\x04aA\x07V[a\x01\xD6V[\0[a\0\xD0a\0\xE06`\x04aA~V[a\x02\xB7V[a\0\xD0a\0\xF36`\x04aA\xDBV[a\x03\x9EV[a\x01\x0Ba\x01\x066`\x04aB\x02V[a\x03\xABV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x011a\x01,6`\x04aB/V[a\x03\xB5V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x01\x15V[a\x011a\x01p6`\x04aB/V[a\x06;V[a\x011a\x01\x836`\x04aB\xCEV[a\x08\xA3V[a\x01\x90a\n\xF3V[`@Qa\x01\x15\x97\x96\x95\x94\x93\x92\x91\x90aCtV[a\x01\xB6a\x01\xB16`\x04aA\x07V[a\x0B\x9BV[`@Qa\x01\x15\x91\x90aD3V[a\0\xD0a\x01\xD16`\x04aDEV[a\x0C\xB2V[a\x01\xDEa\r\xADV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x02R\x90\x85\x90\x85\x90`\x04\x01aD\xB6V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x02mW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x02\xB2\x91\x90\x81\x01\x90aE/V[PPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03&W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05Ta\x03X\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x0E\x80V[`\x05`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[a\x03\xA83\x82a\x10\x87V[PV[_\x81T_R` _\xF3[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04%W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x043\x85`@\x015a\x10\xC2V[\x90P_a\x04?\x87a\x11\x03V[\x90P_\x80a\x04\xBD\x83\x8Ba\x04U` \x8C\x01\x8CaF\x1FV[a\x04e`@\x8D\x01` \x8E\x01aF\x1FV[`@\x80Q`\x06\x92\x83R`\x03\x93\x90\x93R_\x93\x84R``\x8E\x81\x015`&R`:`\x0C \x95\x85R` \x86\x90R\x91\x81R\x92 \x91R`\x08\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[\x90\x92P\x90P_a\x05\x0Fa\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11qV[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\x05H\x82a\x05#` \x8D\x01\x8DaF\x1FV[a\x053`@\x8E\x01` \x8F\x01aF\x1FV[_\x89\x81R`\x07` R`@\x90 \x92\x91\x90a\x11\xA6V[\x90P_a\x05\x8Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x12FV[\x85T\x90\x91P_\x90a\x05\xAF\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x12\xA1V[a\x05\xB9\x91\x90aFlV[\x90Pa\x05\xD7\x8E\x8E_\x01` \x81\x01\x90a\x05\xD1\x91\x90aF\x7FV[\x83a\x12\xCCV[a\x06\x06a\x05\xE3\x89a\x14kV[a\x05\xED\x90\x84aF\x9AV[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x12\xA1V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x06\xABW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x84\x015_a\x06\xBA\x87a\x11\x03V[\x90P_a\x06\xCA` \x88\x01\x88aF\x1FV[\x90P_a\x06\xDD`@\x89\x01` \x8A\x01aF\x1FV[\x90P_`\x07_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x07\x14\x85\x8D\x86\x86\x8E``\x015`\x06a\x11\x17\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x07[a\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x11qV[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07xWa\x07xaF\xC2V[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x07\x97Wa\x07\x97aF\xC2V[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\x07\xEAW\x81\x83\x10\x15a\x07\xD9W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\x07\xD2Wa\x07\xD2aF\xC2V[\x01Ua\x08HV[a\x07\xE3\x82\x84aFlV[\x90Pa\x08HV[\x83`\x02\x0B\x87`\x02\x0B\x13a\x08'W\x82\x82\x10\x15a\x08\x1DW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\x07\xD2Wa\x07\xD2aF\xC2V[a\x07\xE3\x83\x83aFlV[\x81\x83\x87c\x01\0\0\0\x01Ta\x08;\x91\x90aFlV[a\x08E\x91\x90aFlV[\x90P[_a\x08S\x82\x8Ca\x12\xA1V[\x90P\x80\x86_\x01_\x82\x82Ta\x08g\x91\x90aF\xEFV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\x13W`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\t9a\t$` \x88\x01\x88aF\x7FV[a\t4`@\x89\x01` \x8A\x01aF\x7FV[a\x14\x90V[\x90P_a\ty\x82a\tJ\x86\x88aG\x02V[`\x05Th\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90`\xF0\x1Ca\x14\xE8V[P\x90Pa\t\xBA`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01\x7FGot pool key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[a\t\xF8`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01\x7FComparing key spacing\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[a\n\x13a\n\x0B`\x80\x89\x01``\x8A\x01aF\x1FV[`\x02\x0Ba\x15\xFAV[a\n\x1F\x81`\x02\x0Ba\x15\xFAV[a\nV`@Q\x80``\x01`@R\x80`'\x81R` \x01aIM`'\x919a\nK``\x8A\x01`@\x8B\x01aGhV[b\xFF\xFF\xFF\x16_a\x16\x8BV[`\x02\x81\x90\x0Ba\nk`\x80\x89\x01``\x8A\x01aF\x1FV[`\x02\x0B\x14\x15\x80a\n\x8FWP_a\n\x87``\x89\x01`@\x8A\x01aGhV[b\xFF\xFF\xFF\x16\x14\x15[\x15a\n\xC6W`@Q\x7F\xC2Vb+\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x97\x96PPPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x0B\x89`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[``3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0C\x0CW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82_a\x0C\x17\x82a\x17\x1EV[`\x05T\x91\x93P\x91P_\x90a\x0CP\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x17\xEFV[\x90\x93P\x90Pa\x0C^\x82a\x19\x8AV[a\x0Cj\x83`\x02\x83a\x1A\x96V[\x92Pa\x0Cv\x83\x82a\x1B\"V[\x92Pa\x0C\x82\x83\x82a\x1B\xC2V[\x92Pa\x0C\x8F\x83\x87\x87a\x1CQV[a\x0C\x98\x82a\x1ChV[PP`@\x80Q_\x81R` \x81\x01\x90\x91R\x91PP[\x92\x91PPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\r!W`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x81\x81\x10\x15a\x02\xB2W_\x83\x83\x83\x81\x81\x10a\r>Wa\r>aF\xC2V[\x90P` \x02\x01` \x81\x01\x90a\rS\x91\x90aF\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x04` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\r#V[`\x05TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\r\xF4W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x04` R`@\x90 T`\xFF\x16a\x0E<W`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0EECa\x1E\xFAV[`\x05\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[_\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x0E\xE6W`@Q\x7F\x9F8\xAF\xB8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x0F#W`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x0FeW`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\x85\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1F\x13V[\x90P_a\x0F\x92\x87\x87a\x14\x90V[\x90P`@Q` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x10\x06W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x0F\xFAW\x82\x85RPa\x10\x06V[P` \x84\x01\x93Pa\x0F\xBEV[\x90\x81Rk`\x0B8\x03\x80`\x0B_9_\xF3\0\x84R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x10|\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\x10\xBAWc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[_\x80\x82\x13\x15a\x10\xFDW`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`@\x80Q`\x06\x93\x90\x93R`\x03\x93\x90\x93R_\x93\x84R`&R`:`\x0C \x93\x83R` \x84\x90R\x93\x81R``\x90\x91 \x92\x90R`\x08\x91\x90\x91\x1C\x7F\x10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x91V[_\x81\x81R`\x06` R`@\x81 a\x11\x9Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a\x1F6V[\x94\x93PPPPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xC2Wa\x11\xC2aF\xC2V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x11\xE1Wa\x11\xE1aF\xC2V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x12\x06Wa\x11\xFD\x81\x83aFlV[\x92PPPa\x11\x9EV[\x85`\x02\x0B\x84`\x02\x0B\x13a\x12\x1DWa\x11\xFD\x82\x82aFlV[\x80\x82\x88c\x01\0\0\0\x01Ta\x121\x91\x90aFlV[a\x12;\x91\x90aFlV[\x97\x96PPPPPPPV[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x12\x85WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x12\xBCWc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x80_\x03a\x12\xD8WPPPV[`@Q\x7F\xA5\x84\x11\x94\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA5\x84\x11\x94\x90`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x13^W_\x80\xFD[PZ\xF1\x15\x80\x15a\x13pW=_\x80>=_\xFD[Pa\x13\xB6\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1FfV[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x14AW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14e\x91\x90aG\x81V[PPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x14\x8CWa\x14\x8Ca\x1F\xAFV[P\x90V[_`(\x83\x83`@Q` \x01a\x14\xC8\x92\x91\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90\x1B\x90P\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x15WW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[a\x03\xA8\x81`@Q`$\x01a\x15\x7F\x91\x90aD3V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FA0O\xAC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[a\x03\xA8\x81`@Q`$\x01a\x16\x10\x91\x81R` \x01\x90V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FN\x0C\x1D\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[a\x02\xB2\x83\x83\x83`@Q`$\x01a\x16\xA3\x93\x92\x91\x90aG\x98V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x96\x9C\xDD\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x17<\x86\x84aFlV[a\x17F\x91\x90aG\xBCV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x17\xE3W_a\x17r` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x17\xD9W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x17TV[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x18\x11\x8A\x84aFlV[a\x18\x1B\x91\x90aG\xBCV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x19}W`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x18a\x90a\x17k\x90\x8C\x90`\xF0\x1Ca\x1F\xC5V[\x90P_a\x18ua\x17k\x8Ca\xFF\xFF\x86\x16a\x1F\xC5V[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x18\xBEWP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x18\xF5W`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x194s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x14\xE8V[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x189V[P\x93PPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02\xB2W`D\x81\x02` \x84\x90\x1C\x01`\x14\x81\x015`\x80\x1C\x80\x15a\x1A\x8CW\x815``\x1Cs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x0B\r\x9C\t\x82`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x85\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AeW_\x80\xFD[PZ\xF1\x15\x80\x15a\x1AwW=_\x80>=_\xFD[Pa\x1A\x8A\x92P`\x02\x91P\x83\x90P\x84a &V[P[PP`\x01\x01a\x19\x93V[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x85\x81\x01\x95\x805`\xE8\x1C\x01\x01\x90[\x81\x86\x14a\x1B\x18Wa\x1B\x11\x86\x82\x87\x87a _V[\x95Pa\x1A\xFEV[P\x93\x94\x93PPPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x1B<a#\x1DV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x91\x90\x91R\x7F\xD2\x9C\xD8\xAD\x13\x068\xCED\xEA\x18VH\xD2q#E\xE8\xE4V\xFE\xF8\x153(\xEE@\x91m\xF2y\x0E\x81R\x90\x91P[\x82\x86\x14a\x1B\x18Wa\x1B\xBB\x86\x82\x84\x88a#gV[\x95Pa\x1B\xA8V[_\x80a\x1B\xCCa#\x1DV[`@\x80Qa\x01`\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a\x1B\x18Wa\x1CJ\x86\x83\x85\x88a$\xFCV[\x95Pa\x1C7V[\x80\x82\x01\x80\x84\x14a\x14eWc\x01\x84/\x8C_R`\x04`\x1C\xFD[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x02\xB2W`D\x81\x02` \x84\x90\x1C\x01\x805``\x1C`$\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a\x1C\xB2\x84a\x1C\xA9\x84\x86aF\xEFV[`\x02\x91\x90a'\x17V[\x12\x15a\x1D\x07W`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x84\x92\x90a\x1D;\x90\x84\x90aF\xEFV[\x90\x91UPP\x80\x15a\x1E\xEAW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94a\x1D\x9F\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1E\x02W_\x80\xFD[PZ\xF1\x15\x80\x15a\x1E\x14W=_\x80>=_\xFD[Pa\x1EZ\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x1FfV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x1E\xC4W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\xE8\x91\x90aG\x81V[P[PP`\x01\x90\x92\x01\x91Pa\x1Cq\x90PV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x14\x8CWa\x14\x8Ca\x1F\xAFV[_a\x0C\xAC` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;aG\xBCV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa\x1F]WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1F\xA6Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[c5'\x8D\x12_R`\x04`\x1C\xFD[a\x03\xA8\x81a'ZV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a \x14W`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1C\xFEV[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x14ea X\x82\\\x84a'zV[\x82\x90a'\x92V[`\x01\x84\x81\x01\x94_\x91\x905\x82\x1A\x90a {\x90\x86\x90\x83\x16\x15\x15a'\x99V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FpairIndex: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x02\x87\x01\x965`\xF0\x1C\x90a \xC3\x90\x82a'\xEAV[a \xE5a \xD4\x85a\xFF\xFF\x84\x16a(\x7FV[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x89\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x89\x01R\x16` \x87\x01\x90\x81R`\xA0\x90 _`\x10\x89\x01\x895`\x80\x1C`@\x80Q\x80\x82\x01\x90\x91R`\x0C\x81R\x7FamountIn: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x91\x9APo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91Pa!u\x90\x82a'\xEAV[_\x81\x15a\"\x8AW_a!\xC0a\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11qV[\x90Pa!\xCB\x83a(\xDFV[`\xE0\x8B\x01Ra!\xFA\x8A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a):V[a\"=a\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x11qV[`\x80\x8B\x01Q_\x86\x81R`\x07` R`@\x90 \x91\x93Pa\"\x84\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a)XV[Pa\"\xD0V[a\"\xCDa\x05\x06s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x11qV[\x90P[_a\"\xF7`\x02\x87\x16\x15\x15_\x86\x81R`\x07` R`@\x90 `\x80\x8D\x01Q\x8E\x91\x90\x88\x90\x87a)\xE1V[` \x8C\x01Q\x91\x9CP\x91Pa#\r\x90\x8A\x90\x83a'\x17V[P\x99\x9A\x99PPPPPPPPPPV[_a#ba#)a-\xEFV[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15``\x87\x01R\x86\x015`\x80\x90\x81\x1C` \x87\x01R`\x11\x87\x015\x90\x1C`@\x86\x01R`#\x86\x01\x95`!\x015`\xF0\x1Ca#\xC3`\x02\x83\x16\x15\x15a#\xB1\x86\x84a(\x7FV[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xA0\x89\x01R\x16`\x80\x87\x01RP`\x04\x81\x16a#\xF6W\x85_a$\0V[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x87\x01R\x95P_a$,\x87`\x08\x84\x16\x15a.\xE7V[`\xE0\x89\x01R\x90\x97P\x90P_a$ca$W\x88g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01Ra\x01 \x90 \x90V[`\"\x88\x01R`B\x87 \x90V[\x90Pa$n\x81a/\x92V[_`\x10\x84\x16a$\x86Wa$\x81\x89\x83a/\xE3V[a$\x90V[a$\x90\x89\x83a0MV[\x90\x99P\x90Pa$\x9F\x83\x82a0\x91V[`\x80\x88\x01Q` \x89\x01Qa$\xBA\x91\x83\x91`\x01\x88\x16\x15\x15a0\xD9V[_a$\xCA\x89`\xC0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa$\xEE\x81\x8A`\xA0\x01Q\x8B`@\x01Qa$\xE9\x89`\xFF\x16`\x01\x16\x15\x15\x90V[a1cV[P\x97\x98\x97PPPPPPPPV[`\x01\x84\x01\x93_\x905\x81\x1Aa%\x10\x85\x82a1\xDBV[`\x01\x81\x16\x15\x15`\x80\x86\x01R`\x02\x86\x01\x95_\x90\x81\x90\x81\x905`\xF0\x1Ca%a`\x08\x86\x16\x15\x15a%=\x89\x84a(\x7FV[\x90`\x05\x1B` \x81\x18\x82\x01Q\x90\x82\x01\x80Q`\x80\x90\x91\x01Q``\x90\x93\x01Q\x91\x93\x90\x92\x91\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`\xC0\x8E\x01R\x92\x90\x91\x16`\xA0\x8C\x01R\x94P\x92PPP` \x88\x01\x885``\x89\x01\x81\x90R\x90\x98P\x82\x10\x15a%\xD6W`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x83\x16a%\xE5W\x87_a%\xEFV[`\x14\x88\x01\x885``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x89\x01R\x97P_a&\x1B\x89`\x04\x86\x16\x15a.\xE7V[a\x01\0\x8B\x01R\x90\x99P\x90Pa&1\x88\x8A\x86a2\x8EV[\x98P_\x80a&B\x8A\x8C\x88\x88\x88a2\xD2V[\x91\x9CP\x92P\x90P_a&U\x8B\x88\x8Ca4$V[\x90P_`\x80\x88\x16a&oWa&j\x8D\x83a/\xE3V[a&yV[a&y\x8D\x83a0MV[\x90\x9DP\x90P`\x10\x88\x16\x15a&\xB0Wa&\x9C\x8Ca\x01@\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a4>V[a&\xAB\x81\x8Da\x01 \x01Qa\x10\x87V[a&\xB9V[a&\xB9\x82a/\x92V[a&\xC3\x85\x82a0\x91V[`\xA0\x8C\x01Qa&\xDA\x90\x82\x90\x86`\x01\x8C\x16\x15\x15a0\xD9V[_a&\xEA\x8D`\xE0\x01Q\x83\x81\x1C\x18\x90V[\x90Pa'\x05\x81\x8E`\xC0\x01Q\x86a$\xE9\x8D`\xFF\x16`\x01\x16\x15\x15\x90V[P\x9B\x9C\x9BPPPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a'Ra'I\x82\\\x85a4xV[\x92P\x81\x83a'\x92V[P\x93\x92PPPV[\x80Qjconsole.log` \x83\x01_\x80\x84\x83\x85Z\xFAPPPPPV[\x81\x81\x01\x82\x81\x12\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[\x80\x15\x15`\xC0\x83\x01R\x80a'\xC0Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a'\xC7V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[a({\x82\x82`@Q`$\x01a(\0\x92\x91\x90aG\xF4V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x97\x10\xA9\xD0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[PPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a(\xCEW`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\x1C\xFEV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x10\xFDW`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x02\xB2W`@Q=_\x82>P=_\xF3[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a)\x9CW\x82`\x02\x0Ba)\x80\x82\x84`\x02\x0Ba4\x90\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a)\x97Wa)\x97\x86\x85\x87\x86\x86\x86a4\xA1V[a)\xD9V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a)\xD9W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a)\xD9Wa)\xD9\x86\x85\x87\x86\x86\x86a54V[PPPPPPV[_\x80\x87\x15a*\x83W`\x10\x87\x01\x965`\x80\x1Ca*M\x81a*6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a5\xD3V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[\x87c\x01\0\0\0\x01_\x82\x82Ta*b\x91\x90aF\xEFV[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa-\xE4V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7FstartTick: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x03\x88\x01\x97_\x91\x82\x915`\xE8\x1D\x90a*\xD3\x90`\x02\x83\x90\x0Ba6(V[`@\x80Q\x80\x82\x01\x90\x91R`\r\x81R\x7Fliquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a+\x1B\x90\x82a'\xEAV[_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a+rWa+m\x83\x88\x87\x89\x85a6\xB9V[a+\x7FV[a+\x7F\x83\x88\x87\x89\x85a:\x9AV[`@\x80Q``\x81\x01\x90\x91R`5\x80\x82R\x92\x9DP\x90\x9BP\x92\x97P\x90\x93Pa+\xAD\x91\x90aI\x98` \x83\x019a\x15kV[`@\x80Q\x80\x82\x01\x90\x91R`\x13\x81R\x7FdonateToCurrent: %s\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x86\x01\x955`\x80\x1C\x90a+\xF5\x90\x82a'\xEAV[a,#\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[a,-\x90\x8BaF\xEFV[\x99Pa,Ko\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84aF\xEFV[\x92Pa,W\x86\x86a<\x07V[\x81Q_\x90a,\x9C\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a5\xD3V[\x90Pa,\xEF`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01\x7FcurrentLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[a-@`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FendLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a-\xB9W`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\x1C\xFEV[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta-\xCF\x91\x90aF\xEFV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a.\xE4WP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a/\x88W\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a/\xD8W`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a({\x81`\x01a'\x92V[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90a0\n\x83\x86\x84\x84a<@V[a0@W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a0\x8AWc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81\x15a({Wc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x02\xB2Wc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x81a0\xE6`\x02\x85\x83a &V[\x81\x15a1:Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a1/\x90\x84\x90aFlV[\x90\x91UPa1\\\x90PV[a1\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a<\x85V[PPPPPV[\x81a1p`\x02\x85\x83a'\x17V[P\x81\x15a1\xBAWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a1/\x90\x84\x90aF\xEFV[a1\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x1FfV[` \x81\x16\x15a29W`\x10\x81\x16\x15a2\x13WP\x7F\xA9\xE5)MDO\xBA\xEB\\\xA0_+$\xC5\xD8)D\x10\xF3\x14\x13\x04\x8ED]\x88\x1E.\xF6\x9E`\0\x90RV[P\x7F\xEB\xD5\t\x1C9oy\x05nEE_J\x93\xBB\xB0\x16\xC2\x171K\xB25k\"\xD1\xC183\xAC\x88a\x90RV[`\x10\x81\x16\x15a2hWP\x7F\xEF\x0C\xCE\x88\xFD\xA0J\xB3\xF8F\x18\x823r\xCFv\xF6KE\x11\xCE\x8Cyc\x0E\xAB\xC2\x9E\xBC\x9B\x96\x8F\x90RV[P\x7F[\x9DI\xCF\xEDH\xF8\xC9\xD1\xA8c\xF6\x1C$y\xC5y\xFA)\x9C%\xA32\x11\xFC\x85s\x90\xCE\xCC\xE6\xD4\x90RV[_`\x10\x82\x16\x15a2\xB8W`\x08\x83a\x018\x86\x017`\x05`\x08\x84\x01a\x01[\x86\x017`\r\x83\x01\x92Pa2\xCAV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01 \x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x87\x16\x15a3\x86WP\x865`\x80\x90\x81\x1C` \x8A\x81\x01\x82\x90R`\x10\x8A\x015\x83\x1C`@\x8C\x01\x81\x90R`0\x8B\x01\x9A\x91\x90\x91\x015\x90\x92\x1C\x91\x81\x83\x10\x15a3EW`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a3\x7FW`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa3\xB1V[P`\x10\x87\x01\x965`\x80\x1C`@\x87\x16a3\x9EW_a3\xA1V[`\x01[`\xFF\x16` \x8A\x01R`@\x89\x01\x81\x90R[`@\x87\x16\x15\x15\x80a3\xC4WP` \x87\x16\x15\x15[\x15a3\xF2W\x91P\x81a3\xD6\x86\x82a<\xDDV[\x91Pa3\xEB\x82a3\xE6\x81\x88a<\xEAV[a<\xF5V[\x91Pa4\x17V[\x90P\x80a3\xFF\x86\x82a=\0V[\x92Pa4\x14\x83a4\x0F\x81\x88a<\xEAV[a=\x0BV[\x92P[P\x95\x97\x90\x96P\x93PPPPV[_a\x11\x9Ea42\x85\x85a=\x16V[`\"\x84\x01R`B\x83 \x90V[\x80B\x11\x15a\x03\xA8W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x82\x03\x82\x81\x13\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a4\xC4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a=6V[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a4\xDDWPa)\xD9V[\x80\x15a5.W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a4\xFDWa4\xFDaF\xC2V[\x01T\x87c\x01\0\0\0\x01Ta5\x11\x91\x90aFlV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a5+Wa5+aF\xC2V[\x01U[Pa4\xA1V[_a5Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a=\xA8V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a5oWPa)\xD9V[\x80\x15a5\xC0W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a5\x8FWa5\x8FaF\xC2V[\x01T\x87c\x01\0\0\0\x01Ta5\xA3\x91\x90aFlV[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a5\xBDWa5\xBDaF\xC2V[\x01U[\x83a5\xCA\x81aH\x15V[\x94PPPa54V[_\x81\x81R`\x06` R`@\x81 _a6\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a\x1F6V[\x95\x94PPPPPV[_a \x1F\x82a6$g\r\xE0\xB6\xB3\xA7d\0\0\x86aHqV[\x04\x90V[a({\x82\x82`@Q`$\x01a6>\x92\x91\x90aG\xF4V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xAF\x7F\xAA8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra\x1F\xBCV[_\x80_\x80_`\x01\x90P_\x80a7\x02`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01\x7Fin _rewardAbove\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[a7S`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7Fstart liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[a7\x95`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7FTrying to reward tick: %s\0\0\0\0\0\0\0\x81RP\x8C`\x02\x0Ba6(V[\x82\x15a9sWa7\xD9`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01\x7FTick is initialized\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[`@\x80Q\x80\x82\x01\x90\x91R`\x15\x81R\x7FGot reward amount: %s\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R`\x10\x8B\x01\x9A5`\x80\x1C\x90a8!\x90\x82a'\xEAV[a8+\x81\x84aF\xEFV[\x92Pa8I\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[a8S\x90\x83aF\xEFV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a8pWa8paF\xC2V[\x01_\x82\x82Ta8\x7F\x91\x90aF\xEFV[\x90\x91UPP\x88Q_\x90a8\xCA\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xEDV[\x91PPa9\x0F`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01\x7FnetLiquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x82`\x0F\x0Ba6(V[a9\x19\x8B\x82a>RV[\x9APa9l`@Q\x80`@\x01`@R\x80`\x11\x81R` \x01\x7Fnew liquidity: %s\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP\x8Co\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[PPa9\xB1V[a9\xB1`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FTick is not initialized\0\0\0\0\0\0\0\0\0\x81RPa\x15kV[\x87Q` \x89\x01Qa9\xFB\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a>lV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a7SWa:S`@Q\x80`@\x01`@R\x80`\x1A\x81R` \x01\x7FDone rewarding.  Total: %s\0\0\0\0\0\0\x81RP\x83a'\xEAV[a:\x87`@Q\x80``\x01`@R\x80`$\x81R` \x01aIt`$\x919\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a'\xEAV[\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80_\x80_`\x01\x90P_\x80a:\xC6`@Q\x80``\x01`@R\x80`#\x81R` \x01aI*`#\x919a\x15kV[\x82\x15a;\x90W`\x10\x8A\x01\x995`\x80\x1Ca:\xDF\x81\x84aF\xEFV[\x92Pa:\xFD\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\rV[a;\x07\x90\x83aF\xEFV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a;$Wa;$aF\xC2V[\x01_\x82\x82Ta;3\x91\x90aF\xEFV[\x90\x91UPP\x88Q_\x90a;~\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xEDV[\x91PPa;\x8B\x8B\x82a>\x86V[\x9APPP[\x87Q` \x89\x01Qa;\xDA\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a=6V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a:\xC6W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a({W`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a<\xD0Wcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[_a \x1F\x83\x83[\x90a>\xA0V[_a \x1F\x82\x84a<\xE4V[_a \x1F\x82\x84aFlV[_a \x1F\x82\x84a>\xC2V[_a \x1F\x82\x84aF\xEFV[_\x80`\x10\x83\x16a=(Wa\x01@a=,V[a\x01`[\x90\x93 \x93\x92PPPV[_\x80\x80\x80a=[a=P\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01aH\x88V[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa=\x8B\x81a=\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\xDAV[\x90a?\x15V[\x90\x94P\x90Pa=\x9B\x82\x82\x87a?\xD7V[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80a=\xBD\x85\x87\x07\x82\x13\x86\x88\x05\x03a=PV[\x90\x92P\x90Pa=\x8B\x81a=\xE7s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\xDAV[\x90a@\x01V[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa>-WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a=\xBDa=P`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03aH\xC9V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x0C\xACWc\xC9eN\xD4_R`\x04`\x1C\xFD[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a>\xB8\x83\x85aHqV[a \x1F\x91\x90aG\xBCV[_\x81a>\xB8k\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85aHqV[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a6\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a\x1F6V[_\x80_a?\xB0\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a?\xC6W`\xFFa?\xCDV[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a?\xED`\x01\x87\x90\x0Ba\x01\0aI\nV[a?\xF7\x91\x90aH\x88V[a\x11\x9E\x91\x90aI\nV[_\x80_\x83`\xFF\x03\x90P_a@\xA2\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a@\xB7W_a@\xBEV[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a@\xD9W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xF0W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a0FW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15aA\x18W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA.W_\x80\xFD[aA:\x85\x82\x86\x01a@\xC9V[\x90\x96\x90\x95P\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03\xA8W_\x80\xFD[\x805b\xFF\xFF\xFF\x81\x16\x81\x14aAyW_\x80\xFD[\x91\x90PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15aA\x91W_\x80\xFD[\x845aA\x9C\x81aAFV[\x93P` \x85\x015aA\xAC\x81aAFV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14aA\xC2W_\x80\xFD[\x91PaA\xD0``\x86\x01aAgV[\x90P\x92\x95\x91\x94P\x92PV[_` \x82\x84\x03\x12\x15aA\xEBW_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a \x1FW_\x80\xFD[_` \x82\x84\x03\x12\x15aB\x12W_\x80\xFD[P5\x91\x90PV[_`\xA0\x82\x84\x03\x12\x15aB)W_\x80\xFD[P\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15aBEW_\x80\xFD[\x865aBP\x81aAFV[\x95PaB_\x88` \x89\x01aB\x19V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15aB\x90W_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xB1W_\x80\xFD[aB\xBD\x88\x82\x89\x01a@\xC9V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15aB\xE3W_\x80\xFD[\x855aB\xEE\x81aAFV[\x94PaB\xFD\x87` \x88\x01aB\x19V[\x93P`\xC0\x86\x015aC\r\x81aAFV[\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xB1W_\x80\xFD[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_aC\xAE`\xE0\x83\x01\x89aC(V[\x82\x81\x03`@\x84\x01RaC\xC0\x81\x89aC(V[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aD\"W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aD\x04V[P\x90\x9B\x9APPPPPPPPPPPV[` \x81R_a \x1F` \x83\x01\x84aC(V[_\x80` \x83\x85\x03\x12\x15aDVW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aDlW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD|W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x92W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15aD\xA6W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aE?W_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEUW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aEeW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\x7FWaE\x7FaE\x02V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aE\xEBWaE\xEBaE\x02V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15aF\x02W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_` \x82\x84\x03\x12\x15aF/W_\x80\xFD[\x815\x80`\x02\x0B\x81\x14a \x1FW_\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C\xACWa\x0C\xACaF?V[_` \x82\x84\x03\x12\x15aF\x8FW_\x80\xFD[\x815a \x1F\x81aAFV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xACWa\x0C\xACaF?V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\xACWa\x0C\xACaF?V[\x805\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90`\x02\x84\x10\x15aGaW\x7F\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x85`\x02\x03`\x03\x1B\x1B\x82\x16\x16\x91P[P\x92\x91PPV[_` \x82\x84\x03\x12\x15aGxW_\x80\xFD[a \x1F\x82aAgV[_` \x82\x84\x03\x12\x15aG\x91W_\x80\xFD[PQ\x91\x90PV[``\x81R_aG\xAA``\x83\x01\x86aC(V[` \x83\x01\x94\x90\x94RP`@\x01R\x91\x90PV[_\x82aG\xEFW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[`@\x81R_aH\x06`@\x83\x01\x85aC(V[\x90P\x82` \x83\x01R\x93\x92PPPV[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03aHIWaHIaF?V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\xACWa\x0C\xACaF?V[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x0C\xACWa\x0C\xACaF?V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x0C\xACWa\x0C\xACaF?V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14aGaWaGaaF?V\xFEin _rewardBelow but we shouldn't beComparing key fee %s to expected fee %sDone rewarding.  Final liquidity: %sDone rewarding, trying to get reward for current tick\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Function with signature `extsload(bytes32)` and selector `0x1e2eaeaf`.
```solidity
function extsload(bytes32 slot) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsloadCall {
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`extsload(bytes32)`](extsloadCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsloadReturn {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsloadCall> for UnderlyingRustTuple<'_> {
                fn from(value: extsloadCall) -> Self {
                    (value.slot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsloadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slot: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsloadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: extsloadReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsloadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsloadCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsloadReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(bytes32)";
            const SELECTOR: [u8; 4] = [30u8, 46u8, 174u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
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
        extsload(extsloadCall),
        invalidateNonce(invalidateNonceCall),
        toggleNodes(toggleNodesCall),
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
            [30u8, 46u8, 174u8, 175u8],
            [33u8, 208u8, 238u8, 112u8],
            [37u8, 153u8, 130u8, 229u8],
            [52u8, 64u8, 216u8, 32u8],
            [132u8, 176u8, 25u8, 110u8],
            [145u8, 221u8, 115u8, 70u8],
            [214u8, 207u8, 253u8, 30u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromCalls {
        const NAME: &'static str = "AngstromCalls";
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
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extsload(_) => <extsloadCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::invalidateNonce(_) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::toggleNodes(_) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn extsload(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <extsloadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::extsload)
                    }
                    extsload
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
                    fn toggleNodes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <toggleNodesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::toggleNodes)
                    }
                    toggleNodes
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
                Self::extsload(inner) => {
                    <extsloadCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::toggleNodes(inner) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::extsload(inner) => {
                    <extsloadCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::toggleNodes(inner) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        NoEntry(NoEntry),
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
            [47u8, 101u8, 158u8, 68u8],
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
        const COUNT: usize = 27usize;
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
                Self::NoEntry(_) => <NoEntry as alloy_sol_types::SolError>::SELECTOR,
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
                    fn NoEntry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NoEntry as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NoEntry)
                    }
                    NoEntry
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
                Self::NoEntry(inner) => {
                    <NoEntry as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::NoEntry(inner) => {
                    <NoEntry as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
        controller: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AngstromInstance<T, P, N>>,
    > {
        AngstromInstance::<T, P, N>::deploy(provider, uniV4PoolManager, controller)
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
        AngstromInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, uniV4PoolManager, controller)
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
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<AngstromInstance<T, P, N>> {
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
        ///Creates a new call builder for the [`extsload`] function.
        pub fn extsload(
            &self,
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsloadCall, N> {
            self.call_builder(&extsloadCall { slot })
        }
        ///Creates a new call builder for the [`invalidateNonce`] function.
        pub fn invalidateNonce(
            &self,
            nonce: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, invalidateNonceCall, N> {
            self.call_builder(&invalidateNonceCall { nonce })
        }
        ///Creates a new call builder for the [`toggleNodes`] function.
        pub fn toggleNodes(
            &self,
            nodes: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, toggleNodesCall, N> {
            self.call_builder(&toggleNodesCall { nodes })
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
