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
    error BundleChangeNetNegative(address asset);
    error DuplicateAsset();
    error Expired();
    error FailedToDeployNewStore();
    error FeeAboveMax();
    error FillingTooLittle();
    error FillingTooMuch();
    error GasAboveMax();
    error IndexMayHaveChanged();
    error InvalidPermitType(uint8);
    error InvalidPoolKey();
    error InvalidSignature();
    error InvalidStoreIndex();
    error InvalidTickSpacing();
    error LimitViolated();
    error MissingHookPermissions();
    error NoEntry();
    error NonceReuse();
    error NotController();
    error NotFeeMaster();
    error NotFromHook();
    error NotNode();
    error NotUniswap();
    error OnlyOncePerBlock();
    error OrderAlreadyExecuted();
    error OutOfOrderOrDuplicatePairs();
    error Overflow();
    error PairAccessOutOfBounds(uint256 index, uint256 length);
    error ReaderNotAtEnd();
    error ToBGasUsedAboveMax();
    error Underflow();
    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    constructor(address uniV4, address controller, address feeMaster);

    function beforeAddLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function beforeRemoveLiquidity(address sender, PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory) external returns (bytes4);
    function compose(address from, bytes memory payload) external returns (uint32);
    function configurePool(address assetA, address assetB, uint16 tickSpacing, uint24 feeInE6) external;
    function deposit(address asset, uint256 amount) external;
    function deposit(address asset, address to, uint256 amount) external;
    function eip712Domain() external view returns (bytes1 fields, string memory name, string memory version, uint256 chainId, address verifyingContract, bytes32 salt, uint256[] memory extensions);
    function execute(bytes memory encoded) external;
    function extsload(bytes32 slot) external view returns (bytes32);
    function initializePool(address assetA, address assetB, uint256 storeIndex, uint160 sqrtPriceX96) external;
    function invalidateNonce(uint64 nonce) external;
    function pullFee(address asset, uint256 amount) external;
    function removePool(address expectedStore, uint256 storeIndex) external;
    function toggleNodes(address[] memory nodes) external;
    function unlockCallback(bytes memory data) external returns (bytes memory);
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
      },
      {
        "name": "feeMaster",
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
    "name": "compose",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "payload",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "GasAboveMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "IndexMayHaveChanged",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidPermitType",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ]
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
    "name": "LimitViolated",
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
    "name": "ToBGasUsedAboveMax",
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
    ///0x610180604052348015610010575f80fd5b50604051614b88380380614b8883398101604081905261002f91610148565b306080524660a05281818460608061007b6040805180820182526008815267416e677374726f6d60c01b60208083019190915282518084019093526002835261763160f01b9083015291565b815160209283012081519183019190912060c082905260e0819052604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f8152938401929092529082015246606082015230608082015260a090206101005250506001600160a01b0390811661012052908116610140521661016052610101610109565b505050610192565b30613fff16612a801461012f5760405163d7ab502760e01b815260040160405180910390fd5b565b6001600160a01b0381168114610145575f80fd5b50565b5f805f6060848603121561015a575f80fd5b835161016581610131565b602085015190935061017681610131565b604085015190925061018781610131565b809150509250925092565b60805160a05160c05160e051610100516101205161014051610160516148fa61028e5f395f6114a601525f6112d901525f8181610311015281816104e70152818161056c015281816105ca01528181610694015281816107280152818161088b01528181610fcc0152818161179d015281816121b8015281816122740152818161229b01528181612485015281816125be015281816125fa0152818161263b0152818161267f015281816126cb01528181612dd201528181612f5e01528181613a5501528181613ab001528181613b9e0152613bf901525f61303701525f6130f101525f6130cb01525f61307b01525f61305801526148fa5ff3fe608060405234801561000f575f80fd5b5060043610610115575f3560e01c80637d1f3226116100ad57806391dd73461161007d578063d9caed1211610063578063d9caed1214610293578063d9e17f98146102a6578063f3fef3a3146102b9575f80fd5b806391dd734614610260578063d6cffd1e14610280575f80fd5b80637d1f32261461020c5780638340f5491461021f57806384b0196e146102325780638587f4501461024d575f80fd5b806321d0ee70116100e857806321d0ee701461017a578063259982e5146101be57806347e7ef24146101d15780637407905c146101e4575f80fd5b806309c5eabe146101195780631090641d1461012e578063116a5550146101415780631e2eaeaf14610154575b5f80fd5b61012c6101273660046140be565b6102cc565b005b61012c61013c36600461411e565b6103ad565b61012c61014f366004614184565b61042d565b6101676101623660046141ab565b61043a565b6040519081526020015b60405180910390f35b61018d6101883660046141c2565b610444565b6040517fffffffff000000000000000000000000000000000000000000000000000000009091168152602001610171565b61018d6101cc3660046141c2565b6107f8565b61012c6101df366004614287565b6109f9565b6101f76101f23660046142b1565b610a63565b60405163ffffffff9091168152602001610171565b61012c61021a366004614287565b610d64565b61012c61022d366004614302565b610e33565b61023a610ea2565b604051610171979695949392919061438c565b61012c61025b36600461444b565b610f4a565b61027361026e3660046140be565b611126565b6040516101719190614490565b61012c61028e3660046144a2565b6111c7565b61012c6102a1366004614302565b61125b565b61012c6102b4366004614287565b6112c1565b61012c6102c7366004614287565b611355565b6102d46113bb565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c89491906103489085908590600401614513565b5f604051808303815f875af1158015610363573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526103a8919081019061458c565b505050565b6103b561148e565b6004546103e79068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16858585856114ff565b600460086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b610437338261174a565b50565b5f81545f5260205ff35b5f61044d611785565b5f61045b85604001356117f4565b90505f61046787611835565b90505f806104c4838b61047d60208c018c61468a565b61048d60408d0160208e0161468a565b6006526003525f90815260608b01356026908152603a600c2090829052918152600560209081526040808320848452909152902091565b90925090505f61051661050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611881565b60a01c60020b90565b90505f61054f8261052a60208d018d61468a565b61053a60408e0160208f0161468a565b5f8981526006602052604090209291906118b8565b90505f61059373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016878661195a565b85549091505f906105b6846fffffffffffffffffffffffffffffffff85166119b5565b6105c091906146d2565b90508015610794577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a58411948e5f01602081019061061691906146e5565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015610679575f80fd5b505af115801561068b573d5f803e3d5ffd5b505050506106e37f0000000000000000000000000000000000000000000000000000000000000000828f5f0160208101906106c691906146e5565b73ffffffffffffffffffffffffffffffffffffffff1691906119e0565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8f811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af115801561076e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107929190614700565b505b6107c36107a089611a29565b6107aa9084614717565b84906fffffffffffffffffffffffffffffffff166119b5565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f610801611785565b60408401355f61081087611835565b90505f610820602088018861468a565b90505f6108336040890160208a0161468a565b90505f60065f8581526020019081526020015f2090505f61086a858d86868e6060013560056118499095949392919063ffffffff16565b5090505f6108b161050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611881565b90505f8362ffffff8716630100000081106108ce576108ce61473f565b015490505f8462ffffff8716630100000081106108ed576108ed61473f565b015490505f8760020b8460020b1215610940578183101561092f5781925082865f018962ffffff16630100000081106109285761092861473f565b015561099e565b61093982846146d2565b905061099e565b8360020b8760020b1361097d578282101561097357829150818662ffffff8916630100000081106109285761092861473f565b61093983836146d2565b8183876301000000015461099191906146d2565b61099b91906146d2565b90505b5f6109a9828c6119b5565b905080865f015f8282546109bd919061476c565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b610a1b73ffffffffffffffffffffffffffffffffffffffff8316333084611a4e565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260026020908152604080832033845290915281208054839290610a5a90849061476c565b90915550505050565b5f600183018335821a80610b6d57604080517fd505accf00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff881660048201523360248201527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6044820152601484013560d81c6064820181905260198501355f90811a60848401819052601a87013560a48501819052603a88013560c486018190529551605a8901983560601c969495929491939192879263d505accf9260e48084019382900301818387803b158015610b4d575f80fd5b505af1158015610b5f573d5f803e3d5ffd5b505050505050505050610d4a565b60018160ff1603610c55576040517fd505accf00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8716600482015233602482810191909152601484013560801c604483018190529084013560d81c6064830181905260298501355f1a60848401819052602a86013560a48501819052604a87013560c48601819052606a8801973560601c95869063d505accf9060e4015b5f604051808303815f87803b158015610c34575f80fd5b505af1158015610c46573d5f803e3d5ffd5b50505050505050505050610d4a565b60028160ff1603610d0e576040517f8fcbaf0c00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff87166004820152336024820152601483013560e01c60448201819052601884013560d81c6064830181905260016084840152601d8501355f1a60a48401819052601e86013560c48501819052603e87013560e48601819052605e8801973560601c958690638fcbaf0c9061010401610c1d565b6040517f6f1d150900000000000000000000000000000000000000000000000000000000815260ff821660048201526024015b60405180910390fd5b610d55828686611aa6565b506324a2e44b95945050505050565b610d6c61148e565b60045473ffffffffffffffffffffffffffffffffffffffff6801000000000000000090910481169083168114610dce576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610dee73ffffffffffffffffffffffffffffffffffffffff821683611ac3565b600460086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050565b610e5573ffffffffffffffffffffffffffffffffffffffff8416333084611a4e565b73ffffffffffffffffffffffffffffffffffffffff8084165f90815260026020908152604080832093861683529290529081208054839290610e9890849061476c565b9091555050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083610f38604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b8273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff161115610f82579192915b5f84815260208490526040812060281b6004549091505f90610fc79068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff168386611bfd565b5090507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16636276cbbe6040518060a001604052806110188a90565b73ffffffffffffffffffffffffffffffffffffffff1681526020018873ffffffffffffffffffffffffffffffffffffffff90811682525f602080840191909152600287810b6040808601919091523060609586015280517fffffffff0000000000000000000000000000000000000000000000000000000060e089901b168152865185166004820152928601518416602484015285015162ffffff1660448301529284015190920b60648301526080909201518216608482015290861660a482015260c4016020604051808303815f875af11580156110f9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061111d919061477f565b50505050505050565b6060611130611785565b825f61113b82611c80565b60045491935091505f90611174908490849068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16611d51565b909350905061118282611eec565b61118c8382611f17565b92506111988382611fa1565b92506111a48382612054565b92506111b1838787611aa6565b6111ba826120f3565b60205f525f60205260405ff35b6111cf61148e565b5f5b818110156103a8575f8383838181106111ec576111ec61473f565b905060200201602081019061120191906146e5565b73ffffffffffffffffffffffffffffffffffffffff165f90815260036020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff90911615179055506001016111d1565b73ffffffffffffffffffffffffffffffffffffffff83165f9081526002602090815260408083203384529091528120805483929061129a9084906146d2565b909155506103a8905073ffffffffffffffffffffffffffffffffffffffff841683836119e0565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614611330576040517f2833655e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61135173ffffffffffffffffffffffffffffffffffffffff831633836119e0565b5050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152600260209081526040808320338452909152812080548392906113949084906146d2565b90915550611351905073ffffffffffffffffffffffffffffffffffffffff831633836119e0565b6004544367ffffffffffffffff90911603611402576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526003602052604090205460ff1661144a576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61145343612359565b600480547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146114fd576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff161115611538579293925b8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff160361159d576040517f587daa3000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f036115da576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff8316111561161c576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61163c8773ffffffffffffffffffffffffffffffffffffffff16612372565b90505f61165587875f9182526020526040902060281b90565b90506040516b600b380380600b5f395ff30081526020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156116d85783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811687036116cc57828552506116d8565b50602084019350611690565b908152821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff8516915061173f9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661177d57638cb888725f526004601cfd5b909155505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146114fd576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8082131561182f576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b6006919091526003919091525f9182526026908152603a600c2090829052918152602092835260408082208383529093529190912091565b5f8181526006602052604081206118ae73ffffffffffffffffffffffffffffffffffffffff851682612395565b9150505b92915050565b5f808562ffffff8516630100000081106118d4576118d461473f565b015490505f8662ffffff8516630100000081106118f3576118f361473f565b015490508460020b8660020b12156119185761190f81836146d2565b92505050611952565b8560020b8460020b1361192f5761190f82826146d2565b8082886301000000015461194391906146d2565b61194d91906146d2565b925050505b949350505050565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6119995763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f190483118202156119d05763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716611a20576390b8ec185f526004601cfd5b5f603452505050565b5f7001000000000000000000000000000000008210611a4a57611a4a6123c5565b5090565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f51141716611a9957637939f4245f526004601cfd5b5f60605260405250505050565b808201808414611abd576301842f8c5f526004601cfd5b50505050565b5f80611ae48473ffffffffffffffffffffffffffffffffffffffff16612372565b9050808310611b1f576040517fd2c6aae600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80600103611b2d57506118b2565b5f6001611b3a85846146d2565b611b4491906146d2565b90506040516b600b380380600b5f395ff30081526020810183602002806001838a3c60208702820191506020840260208301835e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff85169150611bf59050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811685140280611c6c576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6003818101915f918291803560e81c0101816044611c9e86846146d2565b611ca8919061479a565b905080602086901b1792505f805b82811015611d45575f611cd4602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611d3b576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611cb6565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026611d738a846146d2565b611d7d919061479a565b905060405193508060c0028401925082604052808460201b179450505f5b82841015611edf5760048901983560e081901c905f90611dc390611ccd908c9060f01c6123d2565b90505f611dd7611ccd8c61ffff86166123d2565b90508363ffffffff168363ffffffff16111580611e2057508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611e57576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80611e9673ffffffffffffffffffffffffffffffffffffffff8c168585611bfd565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611d9b565b5093505050935093915050565b63ffffffff81165f5b818110156103a857611f0f602084901c6044830201612433565b600101611ef5565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81523060a082015261012080820152600384810194803560e81c0101905b818514611f9857611f91858286612504565b9450611f7f565b50929392505050565b6003828101925f91813560e81c9091010181611fbb612741565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101919091527f695b2e2b8250e7bc714aad670b4cda55f770437f41399d6f92ff0e64a36ddbfa815267ffffffffffffffff43166101008201529091505b82861461204a576120438682848861278b565b9550612030565b5093949350505050565b5f8061205e612741565b604080516101a0810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101829052610140810182905261016081018290526101808101919091526003868101969293509091803560e81c01015b80861461204a576120ec86838588612990565b95506120d9565b6040805163ffffffff8316602481028201909252805f5b838110156123465760448102602086901c01803560601c6014820135608090811c906034840135901c5f61214b84612142848661476c565b60019190612b80565b121561219b576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84166004820152602401610d41565b80156123295773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001663a5841194846040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015612241575f80fd5b505af1158015612253573d5f803e3d5ffd5b506122999250505073ffffffffffffffffffffffffffffffffffffffff84167f0000000000000000000000000000000000000000000000000000000000000000836119e0565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612303573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123279190614700565b505b6123338487612bc3565b505050602492909201915060010161210a565b506024830282205f5260205fa050505050565b5f680100000000000000008210611a4a57611a4a6123c5565b5f6118b2602073ffffffffffffffffffffffffffffffffffffffff84163b61479a565b5f81602052631e2eaeaf5f5260205f6024601c865afa6123bc5763535cf94b5f526004601cfd5b50505f51919050565b6335278d125f526004601cfd5b5f8163ffffffff841611612421576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401610d41565b602083901c60448302015b9392505050565b602481013560801c801561135157604080517f0b0d9c09000000000000000000000000000000000000000000000000000000008152833560601c600482018190523060248301526044820184905291517f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1691630b0d9c09916064808301925f92919082900301818387803b1580156124df575f80fd5b505af11580156124f1573d5f803e3d5ffd5b506103a892506001915083905084612bcc565b6001838101935f919035821a9061252090859083161515612c05565b60028501943560f01c6125476125368583612c56565b805160208201516040909201519092565b60020b608088015273ffffffffffffffffffffffffffffffffffffffff9081166040880152166020860190815260a090205f60108801883560801c9098506fffffffffffffffffffffffffffffffff1690505f81156126ae575f6125e461050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611881565b90506125ef83612cb6565b60e08a015261261e897f0000000000000000000000000000000000000000000000000000000000000000612d11565b61266161050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611881565b60808a01515f8681526006602052604090209193506126a8919086907f00000000000000000000000000000000000000000000000000000000000000009085908790612d2f565b506126f4565b6126f161050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611881565b90505b5f61271b6002871615155f86815260066020526040902060808c01518d9190889087612db8565b60208b0151919b5091506127329060019083612b80565b50989998505050505050505050565b5f61278661274d613035565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a6001818116151560808781019190915290870135811c60208701526011870135811c60408701526021870135811c6060870181905260418801976031013590911c9081111561280c576040517f2bae6c5200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600282161561284157806fffffffffffffffffffffffffffffffff1686602001818151612839919061476c565b905250612869565b806fffffffffffffffffffffffffffffffff168660400181815161286591906146d2565b9052505b506002868101963560f01c9061289990831615156128878684612c56565b9060051b602081188201519101519091565b73ffffffffffffffffffffffffffffffffffffffff90811660c08901521660a087015250600481166128cc57855f6128d6565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e087015295505f61290f61290387610120902090565b60228701526042862090565b905061291a8161312d565b5f600883166129325761292d888361317e565b61293c565b61293c88836131e8565b60e089015160a08a015160208b0151939b5091935080158402179161296891849160018816151561322c565b60c088015160408901516129839183916001881615156132b6565b5096979650505050505050565b5f8061299c858761332e565b60028201975091505f9081903560f01c6129c56008851615156129bf8884612c56565b9061340e565b73ffffffffffffffffffffffffffffffffffffffff9182166101008c0152911660e08a01529250505060208701873560a08801819052909750811015612a37576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60028216612a4657865f612a50565b60148701873560601c5b73ffffffffffffffffffffffffffffffffffffffff1661012088015296505f612a7d886004851615613450565b6101408a01529098509050612a938789856134fb565b97505f80612aa3898b8787613543565b919b50925090505f612ac4612ab88b8861374a565b60228b015260428a2090565b90505f60808716612ade57612ad98c8361317e565b612ae8565b612ae88c836131e8565b909c5090506010871615612b1f57612b0b8b610180015164ffffffffff1661376a565b612b1a818c610160015161174a565b612b28565b612b288261312d565b6101208b01516101008c01518115830290911790612b4e9082908660018c1615156132b6565b612b5886836137a4565b60e08c0151612b6f9083908760018c16151561322c565b509a9b9a5050505050505050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120612bbb612bb2825c856137ec565b92508183613804565b509392505050565b60248282375050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020611abd612bfe825c8461380b565b8290613804565b80151560c083015280612c2c5773fffd8963efd1fc6a506488495d951d5263988d25612c33565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b5f8163ffffffff841611612ca5576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401610d41565b5060c08102602083901c0192915050565b5f7f800000000000000000000000000000000000000000000000000000000000000082111561182f576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af1806103a8576040513d5f823e503d5ff35b8260020b8260020b1315612d73578260020b612d57828460020b61382390919063ffffffff16565b60020b1315612d6e57612d6e868587868686613834565b612db0565b8260020b8260020b1215612db0575f600284900b828107919091129082900503810260020b8260020b1215612db057612db08685878686866138c7565b505050505050565b5f808715612e5a5760108701963560801c612e2481612e0d7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1689613966565b6fffffffffffffffffffffffffffffffff166139a0565b876301000000015f828254612e39919061476c565b90915550889350506fffffffffffffffffffffffffffffffff16905061302a565b5f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b1315612ed157612ecc83888789856139bb565b612ede565b612ede8388878985613b04565b909b50995060108201965092503560801c612f0b816fffffffffffffffffffffffffffffffff8b166139a0565b612f15908b61476c565b9950612f336fffffffffffffffffffffffffffffffff82168461476c565b9250612f3f8686613c4e565b81515f90612f849073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690613966565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1614612fff576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c16600483015282166024820152604401610d41565b8a856301000000015f828254613015919061476c565b90915550969c50929a50505050505050505050505b965096945050505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f000000000000000000000000000000000000000000000000000000000000000046141661312a5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f818152602081905260409020805c15613173576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611351816001613804565b6017601483013560e81c8084018201935f92813560601c929101906131a583868484613c87565b6131db576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d61322557638baa579f5f526004601cfd5b9293915050565b8161323960018583612bcc565b811561328d5773ffffffffffffffffffffffffffffffffffffffff8086165f908152600260209081526040808320938816835292905290812080548392906132829084906146d2565b909155506132af9050565b6132af73ffffffffffffffffffffffffffffffffffffffff8516863084611a4e565b5050505050565b816132c360018583612b80565b50811561330d5773ffffffffffffffffffffffffffffffffffffffff8086165f9081526002602090815260408083209388168352929052908120805483929061328290849061476c565b6132af73ffffffffffffffffffffffffffffffffffffffff851686836119e0565b60018101905f9035811a600483603c8601376004929092019160208116156133a7576010811661337e577f6ee89dee573705c024a086e19a128ee0a5ee0547e3283adfa72fbe336a4c4b6c6133a0565b7f6be5f22bdcd037f6f35250c32e478fad62195ac2bbab1e2932f8c97af926b4915b84526133fa565b601081166133d5577f022e170cdf338f45bc718f58d29bfafbf3956c2f9ea8d19ccc9b72e42dbbb7b06133f7565b7fb0617b84f694c245e54fb8032ebdc9f56eb26ea2c1b65a46c58f50dbd516e2865b84525b60018116151560c094909401939093525091565b600581901b6020811883015190830180516080909101516060850151620f42409081039061343c82846147d2565b613446919061479a565b9150509250925092565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470836134f157843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f6010821615613529576008836101788601376008929092019160058361019b86013760058301925061353b565b67ffffffffffffffff43166101608501525b509092915050565b5f80808060208616156135f557508535608090811c604089018190526010880135821c60608a0181905260308901986020013590921c91818310156135b4576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156135ee576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050613620565b5060108601953560801c6040861661360d575f613610565b60015b60ff166040890152606088018190525b60208701966010810135608090811c9135901c8082111561366d576040517f668fef1b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff1660808a015260088716156136f65760608716156136c5576136b26fffffffffffffffffffffffffffffffff8216836146d2565b93506136be8685613ccc565b925061373c565b90915081906136ef6136d78784613cd7565b826fffffffffffffffffffffffffffffffff16613ce2565b935061373c565b60608716156137115790925082906136be6136d78784613ccc565b61372d6fffffffffffffffffffffffffffffffff8216836146d2565b92506137398684613cd7565b93505b509597919650945092505050565b5f806010831661375c57610180613760565b6101a05b9093209392505050565b80421115610437576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81156113515763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166103a85763f959fdae5f526004601cfd5b808203828113156118b25763c9654ed45f526004601cfd5b80825d5050565b818101828112156118b25763c9654ed45f526004601cfd5b5f8183071291819005919091030290565b5f61385773ffffffffffffffffffffffffffffffffffffffff8716868685613ced565b94509050600284810b9084900b12156138705750612db0565b80156138c1578662ffffff8516630100000081106138905761389061473f565b015487630100000001546138a491906146d2565b8762ffffff8616630100000081106138be576138be61473f565b01555b50613834565b5f6138ea73ffffffffffffffffffffffffffffffffffffffff8716868685613d5f565b94509050600283810b9085900b136139025750612db0565b8015613953578662ffffff8516630100000081106139225761392261473f565b0154876301000000015461393691906146d2565b8762ffffff8616630100000081106139505761395061473f565b01555b8361395d816147e9565b945050506138c7565b5f8181526006602052604081205f61399773ffffffffffffffffffffffffffffffffffffffff861660038401612395565b95945050505050565b5f61242c826139b7670de0b6b3a7640000866147d2565b0490565b5f808080600181805b8215613a8e5760108a01993560801c6139dd818461476c565b92506139fb818b6fffffffffffffffffffffffffffffffff166139a0565b613a05908361476c565b9150818d8d62ffffff1663010000008110613a2257613a2261473f565b015f828254613a31919061476c565b909155505088515f90613a7c9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613da4565b915050613a898b82613e09565b9a5050505b87516020890151613ad89173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613e23565b809c508194505050876040015160020b8b60020b136139c457989b909a50979850959695505050505050565b5f808080600181805b8215613bd75760108a01993560801c613b26818461476c565b9250613b44818b6fffffffffffffffffffffffffffffffff166139a0565b613b4e908361476c565b9150818d8d62ffffff1663010000008110613b6b57613b6b61473f565b015f828254613b7a919061476c565b909155505088515f90613bc59073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613da4565b915050613bd28b82613e3d565b9a5050505b87516020890151613c219173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613ced565b809c508194505050876040015160020b8b60020b1315613b0d57989b909a50979850959695505050505050565b808214611351576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b5f61242c8284613e57565b5f61242c8284613e79565b5f61242c82846146d2565b5f808080613d12613d078688078313878905036001614845565b600281900b60081d91565b9092509050613d4281613d3c73ffffffffffffffffffffffffffffffffffffffff8b168a86613e91565b90613ecc565b9094509050613d52828287613f8e565b9250505094509492505050565b5f808080613d74858707821386880503613d07565b9092509050613d4281613d9e73ffffffffffffffffffffffffffffffffffffffff8b168a86613e91565b90613fb8565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa613de45763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b808203608081901c156118b25763c9654ed45f526004601cfd5b5f808080613d74613d0760018789078413888a0503614886565b818101608081901c156118b25763c9654ed45f526004601cfd5b5f6b033b2e3c9fd0803ce8000000613e6f83856147d2565b61242c919061479a565b5f81613e6f6b033b2e3c9fd0803ce8000000856147d2565b5f828152600660209081526040808320848452600501909152812061399773ffffffffffffffffffffffffffffffffffffffff861682612395565b5f805f613f678460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082613f7d5760ff613f84565b8360ff1681015b9150509250929050565b5f8160ff8416613fa4600187900b6101006148c7565b613fae9190614845565b61195291906148c7565b5f805f8360ff0390505f6140598260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b905080610100141593508361406e575f614075565b8160ff1681035b925050509250929050565b5f8083601f840112614090575f80fd5b50813567ffffffffffffffff8111156140a7575f80fd5b6020830191508360208285010111156131e1575f80fd5b5f80602083850312156140cf575f80fd5b823567ffffffffffffffff8111156140e5575f80fd5b6140f185828601614080565b90969095509350505050565b73ffffffffffffffffffffffffffffffffffffffff81168114610437575f80fd5b5f805f8060808587031215614131575f80fd5b843561413c816140fd565b9350602085013561414c816140fd565b9250604085013561ffff81168114614162575f80fd5b9150606085013562ffffff81168114614179575f80fd5b939692955090935050565b5f60208284031215614194575f80fd5b813567ffffffffffffffff8116811461242c575f80fd5b5f602082840312156141bb575f80fd5b5035919050565b5f805f805f8587036101608112156141d8575f80fd5b86356141e3816140fd565b955060a07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe082011215614214575f80fd5b60208701945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215614249575f80fd5b5060c08601925061014086013567ffffffffffffffff81111561426a575f80fd5b61427688828901614080565b969995985093965092949392505050565b5f8060408385031215614298575f80fd5b82356142a3816140fd565b946020939093013593505050565b5f805f604084860312156142c3575f80fd5b83356142ce816140fd565b9250602084013567ffffffffffffffff8111156142e9575f80fd5b6142f586828701614080565b9497909650939450505050565b5f805f60608486031215614314575f80fd5b833561431f816140fd565b9250602084013561432f816140fd565b929592945050506040919091013590565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f6143c660e0830189614340565b82810360408401526143d88189614340565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b8181101561443a57835183526020938401939092019160010161441c565b50909b9a5050505050505050505050565b5f805f806080858703121561445e575f80fd5b8435614469816140fd565b93506020850135614479816140fd565b9250604085013591506060850135614179816140fd565b602081525f61242c6020830184614340565b5f80602083850312156144b3575f80fd5b823567ffffffffffffffff8111156144c9575f80fd5b8301601f810185136144d9575f80fd5b803567ffffffffffffffff8111156144ef575f80fd5b8560208260051b8401011115614503575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561459c575f80fd5b815167ffffffffffffffff8111156145b2575f80fd5b8201601f810184136145c2575f80fd5b805167ffffffffffffffff8111156145dc576145dc61455f565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156146485761464861455f565b60405281815282820160200186101561465f575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b8060020b8114610437575f80fd5b5f6020828403121561469a575f80fd5b813561242c8161467c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b818103818111156118b2576118b26146a5565b5f602082840312156146f5575f80fd5b813561242c816140fd565b5f60208284031215614710575f80fd5b5051919050565b6fffffffffffffffffffffffffffffffff82811682821603908111156118b2576118b26146a5565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b808201808211156118b2576118b26146a5565b5f6020828403121561478f575f80fd5b815161242c8161467c565b5f826147cd577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b80820281158282048414176118b2576118b26146a5565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000810361481d5761481d6146a5565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156118b2576118b26146a5565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156118b2576118b26146a5565b5f8260020b8260020b028060020b91508082146148e6576148e66146a5565b509291505056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4\x80\x15a\0\x10W_\x80\xFD[P`@QaK\x888\x03\x80aK\x88\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01HV[0`\x80RF`\xA0R\x81\x81\x84``\x80a\0{`@\x80Q\x80\x82\x01\x82R`\x08\x81RgAngstrom`\xC0\x1B` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83Rav1`\xF0\x1B\x90\x83\x01R\x91V[\x81Q` \x92\x83\x01 \x81Q\x91\x83\x01\x91\x90\x91 `\xC0\x82\x90R`\xE0\x81\x90R`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x93\x84\x01\x92\x90\x92R\x90\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 a\x01\0RPP`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01 R\x90\x81\x16a\x01@R\x16a\x01`Ra\x01\x01a\x01\tV[PPPa\x01\x92V[0a?\xFF\x16a*\x80\x14a\x01/W`@Qc\xD7\xABP'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01EW_\x80\xFD[PV[_\x80_``\x84\x86\x03\x12\x15a\x01ZW_\x80\xFD[\x83Qa\x01e\x81a\x011V[` \x85\x01Q\x90\x93Pa\x01v\x81a\x011V[`@\x85\x01Q\x90\x92Pa\x01\x87\x81a\x011V[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`QaH\xFAa\x02\x8E_9_a\x14\xA6\x01R_a\x12\xD9\x01R_\x81\x81a\x03\x11\x01R\x81\x81a\x04\xE7\x01R\x81\x81a\x05l\x01R\x81\x81a\x05\xCA\x01R\x81\x81a\x06\x94\x01R\x81\x81a\x07(\x01R\x81\x81a\x08\x8B\x01R\x81\x81a\x0F\xCC\x01R\x81\x81a\x17\x9D\x01R\x81\x81a!\xB8\x01R\x81\x81a\"t\x01R\x81\x81a\"\x9B\x01R\x81\x81a$\x85\x01R\x81\x81a%\xBE\x01R\x81\x81a%\xFA\x01R\x81\x81a&;\x01R\x81\x81a&\x7F\x01R\x81\x81a&\xCB\x01R\x81\x81a-\xD2\x01R\x81\x81a/^\x01R\x81\x81a:U\x01R\x81\x81a:\xB0\x01R\x81\x81a;\x9E\x01Ra;\xF9\x01R_a07\x01R_a0\xF1\x01R_a0\xCB\x01R_a0{\x01R_a0X\x01RaH\xFA_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x15W_5`\xE0\x1C\x80c}\x1F2&\x11a\0\xADW\x80c\x91\xDDsF\x11a\0}W\x80c\xD9\xCA\xED\x12\x11a\0cW\x80c\xD9\xCA\xED\x12\x14a\x02\x93W\x80c\xD9\xE1\x7F\x98\x14a\x02\xA6W\x80c\xF3\xFE\xF3\xA3\x14a\x02\xB9W_\x80\xFD[\x80c\x91\xDDsF\x14a\x02`W\x80c\xD6\xCF\xFD\x1E\x14a\x02\x80W_\x80\xFD[\x80c}\x1F2&\x14a\x02\x0CW\x80c\x83@\xF5I\x14a\x02\x1FW\x80c\x84\xB0\x19n\x14a\x022W\x80c\x85\x87\xF4P\x14a\x02MW_\x80\xFD[\x80c!\xD0\xEEp\x11a\0\xE8W\x80c!\xD0\xEEp\x14a\x01zW\x80c%\x99\x82\xE5\x14a\x01\xBEW\x80cG\xE7\xEF$\x14a\x01\xD1W\x80ct\x07\x90\\\x14a\x01\xE4W_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\x01\x19W\x80c\x10\x90d\x1D\x14a\x01.W\x80c\x11jUP\x14a\x01AW\x80c\x1E.\xAE\xAF\x14a\x01TW[_\x80\xFD[a\x01,a\x01'6`\x04a@\xBEV[a\x02\xCCV[\0[a\x01,a\x01<6`\x04aA\x1EV[a\x03\xADV[a\x01,a\x01O6`\x04aA\x84V[a\x04-V[a\x01ga\x01b6`\x04aA\xABV[a\x04:V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Da\x01\x886`\x04aA\xC2V[a\x04DV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x01qV[a\x01\x8Da\x01\xCC6`\x04aA\xC2V[a\x07\xF8V[a\x01,a\x01\xDF6`\x04aB\x87V[a\t\xF9V[a\x01\xF7a\x01\xF26`\x04aB\xB1V[a\ncV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01qV[a\x01,a\x02\x1A6`\x04aB\x87V[a\rdV[a\x01,a\x02-6`\x04aC\x02V[a\x0E3V[a\x02:a\x0E\xA2V[`@Qa\x01q\x97\x96\x95\x94\x93\x92\x91\x90aC\x8CV[a\x01,a\x02[6`\x04aDKV[a\x0FJV[a\x02sa\x02n6`\x04a@\xBEV[a\x11&V[`@Qa\x01q\x91\x90aD\x90V[a\x01,a\x02\x8E6`\x04aD\xA2V[a\x11\xC7V[a\x01,a\x02\xA16`\x04aC\x02V[a\x12[V[a\x01,a\x02\xB46`\x04aB\x87V[a\x12\xC1V[a\x01,a\x02\xC76`\x04aB\x87V[a\x13UV[a\x02\xD4a\x13\xBBV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x03H\x90\x85\x90\x85\x90`\x04\x01aE\x13V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03cW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03\xA8\x91\x90\x81\x01\x90aE\x8CV[PPPV[a\x03\xB5a\x14\x8EV[`\x04Ta\x03\xE7\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x14\xFFV[`\x04`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[a\x0473\x82a\x17JV[PV[_\x81T_R` _\xF3[_a\x04Ma\x17\x85V[_a\x04[\x85`@\x015a\x17\xF4V[\x90P_a\x04g\x87a\x185V[\x90P_\x80a\x04\xC4\x83\x8Ba\x04}` \x8C\x01\x8CaF\x8AV[a\x04\x8D`@\x8D\x01` \x8E\x01aF\x8AV[`\x06R`\x03R_\x90\x81R``\x8B\x015`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R`\x05` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\x05\x16a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x18\x81V[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\x05O\x82a\x05*` \x8D\x01\x8DaF\x8AV[a\x05:`@\x8E\x01` \x8F\x01aF\x8AV[_\x89\x81R`\x06` R`@\x90 \x92\x91\x90a\x18\xB8V[\x90P_a\x05\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x19ZV[\x85T\x90\x91P_\x90a\x05\xB6\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x19\xB5V[a\x05\xC0\x91\x90aF\xD2V[\x90P\x80\x15a\x07\x94W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94\x8E_\x01` \x81\x01\x90a\x06\x16\x91\x90aF\xE5V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06yW_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x8BW=_\x80>=_\xFD[PPPPa\x06\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8F_\x01` \x81\x01\x90a\x06\xC6\x91\x90aF\xE5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90a\x19\xE0V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07nW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x92\x91\x90aG\0V[P[a\x07\xC3a\x07\xA0\x89a\x1A)V[a\x07\xAA\x90\x84aG\x17V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\xB5V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_a\x08\x01a\x17\x85V[`@\x84\x015_a\x08\x10\x87a\x185V[\x90P_a\x08 ` \x88\x01\x88aF\x8AV[\x90P_a\x083`@\x89\x01` \x8A\x01aF\x8AV[\x90P_`\x06_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x08j\x85\x8D\x86\x86\x8E``\x015`\x05a\x18I\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x08\xB1a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x18\x81V[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xCEWa\x08\xCEaG?V[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xEDWa\x08\xEDaG?V[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\t@W\x81\x83\x10\x15a\t/W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\t(Wa\t(aG?V[\x01Ua\t\x9EV[a\t9\x82\x84aF\xD2V[\x90Pa\t\x9EV[\x83`\x02\x0B\x87`\x02\x0B\x13a\t}W\x82\x82\x10\x15a\tsW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\t(Wa\t(aG?V[a\t9\x83\x83aF\xD2V[\x81\x83\x87c\x01\0\0\0\x01Ta\t\x91\x91\x90aF\xD2V[a\t\x9B\x91\x90aF\xD2V[\x90P[_a\t\xA9\x82\x8Ca\x19\xB5V[\x90P\x80\x86_\x01_\x82\x82Ta\t\xBD\x91\x90aGlV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[a\n\x1Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x1630\x84a\x1ANV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\nZ\x90\x84\x90aGlV[\x90\x91UPPPPV[_`\x01\x83\x01\x835\x82\x1A\x80a\x0BmW`@\x80Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16`\x04\x82\x01R3`$\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D\x82\x01R`\x14\x84\x015`\xD8\x1C`d\x82\x01\x81\x90R`\x19\x85\x015_\x90\x81\x1A`\x84\x84\x01\x81\x90R`\x1A\x87\x015`\xA4\x85\x01\x81\x90R`:\x88\x015`\xC4\x86\x01\x81\x90R\x95Q`Z\x89\x01\x985``\x1C\x96\x94\x95\x92\x94\x91\x93\x91\x92\x87\x92c\xD5\x05\xAC\xCF\x92`\xE4\x80\x84\x01\x93\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x0BMW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0B_W=_\x80>=_\xFD[PPPPPPPPPa\rJV[`\x01\x81`\xFF\x16\x03a\x0CUW`@Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R3`$\x82\x81\x01\x91\x90\x91R`\x14\x84\x015`\x80\x1C`D\x83\x01\x81\x90R\x90\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`)\x85\x015_\x1A`\x84\x84\x01\x81\x90R`*\x86\x015`\xA4\x85\x01\x81\x90R`J\x87\x015`\xC4\x86\x01\x81\x90R`j\x88\x01\x975``\x1C\x95\x86\x90c\xD5\x05\xAC\xCF\x90`\xE4\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0CFW=_\x80>=_\xFD[PPPPPPPPPPa\rJV[`\x02\x81`\xFF\x16\x03a\r\x0EW`@Q\x7F\x8F\xCB\xAF\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R3`$\x82\x01R`\x14\x83\x015`\xE0\x1C`D\x82\x01\x81\x90R`\x18\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`\x01`\x84\x84\x01R`\x1D\x85\x015_\x1A`\xA4\x84\x01\x81\x90R`\x1E\x86\x015`\xC4\x85\x01\x81\x90R`>\x87\x015`\xE4\x86\x01\x81\x90R`^\x88\x01\x975``\x1C\x95\x86\x90c\x8F\xCB\xAF\x0C\x90a\x01\x04\x01a\x0C\x1DV[`@Q\x7Fo\x1D\x15\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\rU\x82\x86\x86a\x1A\xA6V[Pc$\xA2\xE4K\x95\x94PPPPPV[a\rla\x14\x8EV[`\x04Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x81\x14a\r\xCEW`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xEEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83a\x1A\xC3V[`\x04`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[a\x0EUs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x1630\x84a\x1ANV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x0E\x98\x90\x84\x90aGlV[\x90\x91UPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x0F8`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0F\x82W\x91\x92\x91[_\x84\x81R` \x84\x90R`@\x81 `(\x1B`\x04T\x90\x91P_\x90a\x0F\xC7\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x86a\x1B\xFDV[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cbv\xCB\xBE`@Q\x80`\xA0\x01`@R\x80a\x10\x18\x8A\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R_` \x80\x84\x01\x91\x90\x91R`\x02\x87\x81\x0B`@\x80\x86\x01\x91\x90\x91R0``\x95\x86\x01R\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x81R\x86Q\x85\x16`\x04\x82\x01R\x92\x86\x01Q\x84\x16`$\x84\x01R\x85\x01Qb\xFF\xFF\xFF\x16`D\x83\x01R\x92\x84\x01Q\x90\x92\x0B`d\x83\x01R`\x80\x90\x92\x01Q\x82\x16`\x84\x82\x01R\x90\x86\x16`\xA4\x82\x01R`\xC4\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xF9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x1D\x91\x90aG\x7FV[PPPPPPPV[``a\x110a\x17\x85V[\x82_a\x11;\x82a\x1C\x80V[`\x04T\x91\x93P\x91P_\x90a\x11t\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1DQV[\x90\x93P\x90Pa\x11\x82\x82a\x1E\xECV[a\x11\x8C\x83\x82a\x1F\x17V[\x92Pa\x11\x98\x83\x82a\x1F\xA1V[\x92Pa\x11\xA4\x83\x82a TV[\x92Pa\x11\xB1\x83\x87\x87a\x1A\xA6V[a\x11\xBA\x82a \xF3V[` _R_` R`@_\xF3[a\x11\xCFa\x14\x8EV[_[\x81\x81\x10\x15a\x03\xA8W_\x83\x83\x83\x81\x81\x10a\x11\xECWa\x11\xECaG?V[\x90P` \x02\x01` \x81\x01\x90a\x12\x01\x91\x90aF\xE5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x03` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x11\xD1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x12\x9A\x90\x84\x90aF\xD2V[\x90\x91UPa\x03\xA8\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x19\xE0V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x130W`@Q\x7F(3e^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x19\xE0V[PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x13\x94\x90\x84\x90aF\xD2V[\x90\x91UPa\x13Q\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x19\xE0V[`\x04TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\x14\x02W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\x14JW`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14SCa#YV[`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14\xFDW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x158W\x92\x93\x92[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15\x9DW`@Q\x7FX}\xAA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x15\xDAW`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x16\x1CW`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16<\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a#rV[\x90P_a\x16U\x87\x87_\x91\x82R` R`@\x90 `(\x1B\x90V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x16\xD8W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x16\xCCW\x82\x85RPa\x16\xD8V[P` \x84\x01\x93Pa\x16\x90V[\x90\x81R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x17?\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\x17}Wc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14\xFDW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x82\x13\x15a\x18/W`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`\x06\x91\x90\x91R`\x03\x91\x90\x91R_\x91\x82R`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R` \x92\x83R`@\x80\x82 \x83\x83R\x90\x93R\x91\x90\x91 \x91V[_\x81\x81R`\x06` R`@\x81 a\x18\xAEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a#\x95V[\x91PP[\x92\x91PPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x18\xD4Wa\x18\xD4aG?V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x18\xF3Wa\x18\xF3aG?V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x19\x18Wa\x19\x0F\x81\x83aF\xD2V[\x92PPPa\x19RV[\x85`\x02\x0B\x84`\x02\x0B\x13a\x19/Wa\x19\x0F\x82\x82aF\xD2V[\x80\x82\x88c\x01\0\0\0\x01Ta\x19C\x91\x90aF\xD2V[a\x19M\x91\x90aF\xD2V[\x92PPP[\x94\x93PPPPV[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x19\x99WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x19\xD0Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1A Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x1AJWa\x1AJa#\xC5V[P\x90V[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1A\x99Wcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[\x80\x82\x01\x80\x84\x14a\x1A\xBDWc\x01\x84/\x8C_R`\x04`\x1C\xFD[PPPPV[_\x80a\x1A\xE4\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a#rV[\x90P\x80\x83\x10a\x1B\x1FW`@Q\x7F\xD2\xC6\xAA\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x03a\x1B-WPa\x18\xB2V[_`\x01a\x1B:\x85\x84aF\xD2V[a\x1BD\x91\x90aF\xD2V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8A<` \x87\x02\x82\x01\x91P` \x84\x02` \x83\x01\x83^\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x1B\xF5\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x1ClW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x1C\x9E\x86\x84aF\xD2V[a\x1C\xA8\x91\x90aG\x9AV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x1DEW_a\x1C\xD4` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1D;W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x1C\xB6V[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x1Ds\x8A\x84aF\xD2V[a\x1D}\x91\x90aG\x9AV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x1E\xDFW`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x1D\xC3\x90a\x1C\xCD\x90\x8C\x90`\xF0\x1Ca#\xD2V[\x90P_a\x1D\xD7a\x1C\xCD\x8Ca\xFF\xFF\x86\x16a#\xD2V[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x1E WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1EWW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x1E\x96s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x1B\xFDV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x1D\x9BV[P\x93PPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x03\xA8Wa\x1F\x0F` \x84\x90\x1C`D\x83\x02\x01a$3V[`\x01\x01a\x1E\xF5V[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x84\x81\x01\x94\x805`\xE8\x1C\x01\x01\x90[\x81\x85\x14a\x1F\x98Wa\x1F\x91\x85\x82\x86a%\x04V[\x94Pa\x1F\x7FV[P\x92\x93\x92PPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x1F\xBBa'AV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x7Fi[.+\x82P\xE7\xBCqJ\xADg\x0BL\xDAU\xF7pC\x7FA9\x9Do\x92\xFF\x0Ed\xA3m\xDB\xFA\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01R\x90\x91P[\x82\x86\x14a JWa C\x86\x82\x84\x88a'\x8BV[\x95Pa 0V[P\x93\x94\x93PPPPV[_\x80a ^a'AV[`@\x80Qa\x01\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Ra\x01`\x81\x01\x82\x90Ra\x01\x80\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a JWa \xEC\x86\x83\x85\x88a)\x90V[\x95Pa \xD9V[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16`$\x81\x02\x82\x01\x90\x92R\x80_[\x83\x81\x10\x15a#FW`D\x81\x02` \x86\x90\x1C\x01\x805``\x1C`\x14\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a!K\x84a!B\x84\x86aGlV[`\x01\x91\x90a+\x80V[\x12\x15a!\x9BW`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\rAV[\x80\x15a#)Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5\x84\x11\x94\x84`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"AW_\x80\xFD[PZ\xF1\x15\x80\x15a\"SW=_\x80>=_\xFD[Pa\"\x99\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x19\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#'\x91\x90aG\0V[P[a#3\x84\x87a+\xC3V[PPP`$\x92\x90\x92\x01\x91P`\x01\x01a!\nV[P`$\x83\x02\x82 _R` _\xA0PPPPV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x1AJWa\x1AJa#\xC5V[_a\x18\xB2` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;aG\x9AV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa#\xBCWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a$!W`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\rAV[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[`$\x81\x015`\x80\x1C\x80\x15a\x13QW`@\x80Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x835``\x1C`\x04\x82\x01\x81\x90R0`$\x83\x01R`D\x82\x01\x84\x90R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x0B\r\x9C\t\x91`d\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a$\xDFW_\x80\xFD[PZ\xF1\x15\x80\x15a$\xF1W=_\x80>=_\xFD[Pa\x03\xA8\x92P`\x01\x91P\x83\x90P\x84a+\xCCV[`\x01\x83\x81\x01\x93_\x91\x905\x82\x1A\x90a% \x90\x85\x90\x83\x16\x15\x15a,\x05V[`\x02\x85\x01\x945`\xF0\x1Ca%Ga%6\x85\x83a,VV[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x88\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x88\x01R\x16` \x86\x01\x90\x81R`\xA0\x90 _`\x10\x88\x01\x885`\x80\x1C\x90\x98Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a&\xAEW_a%\xE4a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x18\x81V[\x90Pa%\xEF\x83a,\xB6V[`\xE0\x8A\x01Ra&\x1E\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a-\x11V[a&aa\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x18\x81V[`\x80\x8A\x01Q_\x86\x81R`\x06` R`@\x90 \x91\x93Pa&\xA8\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a-/V[Pa&\xF4V[a&\xF1a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x18\x81V[\x90P[_a'\x1B`\x02\x87\x16\x15\x15_\x86\x81R`\x06` R`@\x90 `\x80\x8C\x01Q\x8D\x91\x90\x88\x90\x87a-\xB8V[` \x8B\x01Q\x91\x9BP\x91Pa'2\x90`\x01\x90\x83a+\x80V[P\x98\x99\x98PPPPPPPPPV[_a'\x86a'Ma05V[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15`\x80\x87\x81\x01\x91\x90\x91R\x90\x87\x015\x81\x1C` \x87\x01R`\x11\x87\x015\x81\x1C`@\x87\x01R`!\x87\x015\x81\x1C``\x87\x01\x81\x90R`A\x88\x01\x97`1\x015\x90\x91\x1C\x90\x81\x11\x15a(\x0CW`@Q\x7F+\xAElR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16\x15a(AW\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86` \x01\x81\x81Qa(9\x91\x90aGlV[\x90RPa(iV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86`@\x01\x81\x81Qa(e\x91\x90aF\xD2V[\x90RP[P`\x02\x86\x81\x01\x965`\xF0\x1C\x90a(\x99\x90\x83\x16\x15\x15a(\x87\x86\x84a,VV[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01RP`\x04\x81\x16a(\xCCW\x85_a(\xD6V[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x87\x01R\x95P_a)\x0Fa)\x03\x87a\x01 \x90 \x90V[`\"\x87\x01R`B\x86 \x90V[\x90Pa)\x1A\x81a1-V[_`\x08\x83\x16a)2Wa)-\x88\x83a1~V[a)<V[a)<\x88\x83a1\xE8V[`\xE0\x89\x01Q`\xA0\x8A\x01Q` \x8B\x01Q\x93\x9BP\x91\x93P\x80\x15\x84\x02\x17\x91a)h\x91\x84\x91`\x01\x88\x16\x15\x15a2,V[`\xC0\x88\x01Q`@\x89\x01Qa)\x83\x91\x83\x91`\x01\x88\x16\x15\x15a2\xB6V[P\x96\x97\x96PPPPPPPV[_\x80a)\x9C\x85\x87a3.V[`\x02\x82\x01\x97P\x91P_\x90\x81\x905`\xF0\x1Ca)\xC5`\x08\x85\x16\x15\x15a)\xBF\x88\x84a,VV[\x90a4\x0EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16a\x01\0\x8C\x01R\x91\x16`\xE0\x8A\x01R\x92PPP` \x87\x01\x875`\xA0\x88\x01\x81\x90R\x90\x97P\x81\x10\x15a*7W`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16a*FW\x86_a*PV[`\x14\x87\x01\x875``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x88\x01R\x96P_a*}\x88`\x04\x85\x16\x15a4PV[a\x01@\x8A\x01R\x90\x98P\x90Pa*\x93\x87\x89\x85a4\xFBV[\x97P_\x80a*\xA3\x89\x8B\x87\x87a5CV[\x91\x9BP\x92P\x90P_a*\xC4a*\xB8\x8B\x88a7JV[`\"\x8B\x01R`B\x8A \x90V[\x90P_`\x80\x87\x16a*\xDEWa*\xD9\x8C\x83a1~V[a*\xE8V[a*\xE8\x8C\x83a1\xE8V[\x90\x9CP\x90P`\x10\x87\x16\x15a+\x1FWa+\x0B\x8Ba\x01\x80\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a7jV[a+\x1A\x81\x8Ca\x01`\x01Qa\x17JV[a+(V[a+(\x82a1-V[a\x01 \x8B\x01Qa\x01\0\x8C\x01Q\x81\x15\x83\x02\x90\x91\x17\x90a+N\x90\x82\x90\x86`\x01\x8C\x16\x15\x15a2\xB6V[a+X\x86\x83a7\xA4V[`\xE0\x8C\x01Qa+o\x90\x83\x90\x87`\x01\x8C\x16\x15\x15a2,V[P\x9A\x9B\x9APPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a+\xBBa+\xB2\x82\\\x85a7\xECV[\x92P\x81\x83a8\x04V[P\x93\x92PPPV[`$\x82\x827PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x1A\xBDa+\xFE\x82\\\x84a8\x0BV[\x82\x90a8\x04V[\x80\x15\x15`\xC0\x83\x01R\x80a,,Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a,3V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a,\xA5W`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\rAV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x18/W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x03\xA8W`@Q=_\x82>P=_\xF3[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a-sW\x82`\x02\x0Ba-W\x82\x84`\x02\x0Ba8#\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a-nWa-n\x86\x85\x87\x86\x86\x86a84V[a-\xB0V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a-\xB0W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a-\xB0Wa-\xB0\x86\x85\x87\x86\x86\x86a8\xC7V[PPPPPPV[_\x80\x87\x15a.ZW`\x10\x87\x01\x965`\x80\x1Ca.$\x81a.\r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a9fV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a9\xA0V[\x87c\x01\0\0\0\x01_\x82\x82Ta.9\x91\x90aGlV[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa0*V[_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a.\xD1Wa.\xCC\x83\x88\x87\x89\x85a9\xBBV[a.\xDEV[a.\xDE\x83\x88\x87\x89\x85a;\x04V[\x90\x9BP\x99P`\x10\x82\x01\x96P\x92P5`\x80\x1Ca/\x0B\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16a9\xA0V[a/\x15\x90\x8BaGlV[\x99Pa/3o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84aGlV[\x92Pa/?\x86\x86a<NV[\x81Q_\x90a/\x84\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a9fV[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/\xFFW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\rAV[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta0\x15\x91\x90aGlV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a1*WP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a1sW`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13Q\x81`\x01a8\x04V[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90a1\xA5\x83\x86\x84\x84a<\x87V[a1\xDBW`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a2%Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81a29`\x01\x85\x83a+\xCCV[\x81\x15a2\x8DWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a2\x82\x90\x84\x90aF\xD2V[\x90\x91UPa2\xAF\x90PV[a2\xAFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a\x1ANV[PPPPPV[\x81a2\xC3`\x01\x85\x83a+\x80V[P\x81\x15a3\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a2\x82\x90\x84\x90aGlV[a2\xAFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x19\xE0V[`\x01\x81\x01\x90_\x905\x81\x1A`\x04\x83`<\x86\x017`\x04\x92\x90\x92\x01\x91` \x81\x16\x15a3\xA7W`\x10\x81\x16a3~W\x7Fn\xE8\x9D\xEEW7\x05\xC0$\xA0\x86\xE1\x9A\x12\x8E\xE0\xA5\xEE\x05G\xE3(:\xDF\xA7/\xBE3jLKla3\xA0V[\x7Fk\xE5\xF2+\xDC\xD07\xF6\xF3RP\xC3.G\x8F\xADb\x19Z\xC2\xBB\xAB\x1E)2\xF8\xC9z\xF9&\xB4\x91[\x84Ra3\xFAV[`\x10\x81\x16a3\xD5W\x7F\x02.\x17\x0C\xDF3\x8FE\xBCq\x8FX\xD2\x9B\xFA\xFB\xF3\x95l/\x9E\xA8\xD1\x9C\xCC\x9Br\xE4-\xBB\xB7\xB0a3\xF7V[\x7F\xB0a{\x84\xF6\x94\xC2E\xE5O\xB8\x03.\xBD\xC9\xF5n\xB2n\xA2\xC1\xB6ZF\xC5\x8FP\xDB\xD5\x16\xE2\x86[\x84R[`\x01\x81\x16\x15\x15`\xC0\x94\x90\x94\x01\x93\x90\x93RP\x91V[`\x05\x81\x90\x1B` \x81\x18\x83\x01Q\x90\x83\x01\x80Q`\x80\x90\x91\x01Q``\x85\x01Qb\x0FB@\x90\x81\x03\x90a4<\x82\x84aG\xD2V[a4F\x91\x90aG\x9AV[\x91PP\x92P\x92P\x92V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a4\xF1W\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_`\x10\x82\x16\x15a5)W`\x08\x83a\x01x\x86\x017`\x08\x92\x90\x92\x01\x91`\x05\x83a\x01\x9B\x86\x017`\x05\x83\x01\x92Pa5;V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01`\x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x86\x16\x15a5\xF5WP\x855`\x80\x90\x81\x1C`@\x89\x01\x81\x90R`\x10\x88\x015\x82\x1C``\x8A\x01\x81\x90R`0\x89\x01\x98` \x015\x90\x92\x1C\x91\x81\x83\x10\x15a5\xB4W`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a5\xEEW`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa6 V[P`\x10\x86\x01\x955`\x80\x1C`@\x86\x16a6\rW_a6\x10V[`\x01[`\xFF\x16`@\x89\x01R``\x88\x01\x81\x90R[` \x87\x01\x96`\x10\x81\x015`\x80\x90\x81\x1C\x915\x90\x1C\x80\x82\x11\x15a6mW`@Q\x7Ff\x8F\xEF\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8A\x01R`\x08\x87\x16\x15a6\xF6W``\x87\x16\x15a6\xC5Wa6\xB2o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83aF\xD2V[\x93Pa6\xBE\x86\x85a<\xCCV[\x92Pa7<V[\x90\x91P\x81\x90a6\xEFa6\xD7\x87\x84a<\xD7V[\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a<\xE2V[\x93Pa7<V[``\x87\x16\x15a7\x11W\x90\x92P\x82\x90a6\xBEa6\xD7\x87\x84a<\xCCV[a7-o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83aF\xD2V[\x92Pa79\x86\x84a<\xD7V[\x93P[P\x95\x97\x91\x96P\x94P\x92PPPV[_\x80`\x10\x83\x16a7\\Wa\x01\x80a7`V[a\x01\xA0[\x90\x93 \x93\x92PPPV[\x80B\x11\x15a\x047W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\x13QWc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x03\xA8Wc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x80\x82\x03\x82\x81\x13\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[\x81\x81\x01\x82\x81\x12\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a8Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a<\xEDV[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a8pWPa-\xB0V[\x80\x15a8\xC1W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a8\x90Wa8\x90aG?V[\x01T\x87c\x01\0\0\0\x01Ta8\xA4\x91\x90aF\xD2V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a8\xBEWa8\xBEaG?V[\x01U[Pa84V[_a8\xEAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a=_V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a9\x02WPa-\xB0V[\x80\x15a9SW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a9\"Wa9\"aG?V[\x01T\x87c\x01\0\0\0\x01Ta96\x91\x90aF\xD2V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a9PWa9PaG?V[\x01U[\x83a9]\x81aG\xE9V[\x94PPPa8\xC7V[_\x81\x81R`\x06` R`@\x81 _a9\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a#\x95V[\x95\x94PPPPPV[_a$,\x82a9\xB7g\r\xE0\xB6\xB3\xA7d\0\0\x86aG\xD2V[\x04\x90V[_\x80\x80\x80`\x01\x81\x80[\x82\x15a:\x8EW`\x10\x8A\x01\x995`\x80\x1Ca9\xDD\x81\x84aGlV[\x92Pa9\xFB\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a9\xA0V[a:\x05\x90\x83aGlV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a:\"Wa:\"aG?V[\x01_\x82\x82Ta:1\x91\x90aGlV[\x90\x91UPP\x88Q_\x90a:|\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xA4V[\x91PPa:\x89\x8B\x82a>\tV[\x9APPP[\x87Q` \x89\x01Qa:\xD8\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a>#V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a9\xC4W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80\x80\x80`\x01\x81\x80[\x82\x15a;\xD7W`\x10\x8A\x01\x995`\x80\x1Ca;&\x81\x84aGlV[\x92Pa;D\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a9\xA0V[a;N\x90\x83aGlV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a;kWa;kaG?V[\x01_\x82\x82Ta;z\x91\x90aGlV[\x90\x91UPP\x88Q_\x90a;\xC5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xA4V[\x91PPa;\xD2\x8B\x82a>=V[\x9APPP[\x87Q` \x89\x01Qa<!\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a<\xEDV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a;\rW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a\x13QW`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[_a$,\x82\x84a>WV[_a$,\x82\x84a>yV[_a$,\x82\x84aF\xD2V[_\x80\x80\x80a=\x12a=\x07\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01aHEV[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa=B\x81a=<s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\x91V[\x90a>\xCCV[\x90\x94P\x90Pa=R\x82\x82\x87a?\x8EV[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80a=t\x85\x87\x07\x82\x13\x86\x88\x05\x03a=\x07V[\x90\x92P\x90Pa=B\x81a=\x9Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\x91V[\x90a?\xB8V[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa=\xE4WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a=ta=\x07`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03aH\x86V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a>o\x83\x85aG\xD2V[a$,\x91\x90aG\x9AV[_\x81a>ok\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85aG\xD2V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a9\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a#\x95V[_\x80_a?g\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a?}W`\xFFa?\x84V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a?\xA4`\x01\x87\x90\x0Ba\x01\0aH\xC7V[a?\xAE\x91\x90aHEV[a\x19R\x91\x90aH\xC7V[_\x80_\x83`\xFF\x03\x90P_a@Y\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a@nW_a@uV[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a@\x90W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xA7W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a1\xE1W_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a@\xCFW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xE5W_\x80\xFD[a@\xF1\x85\x82\x86\x01a@\x80V[\x90\x96\x90\x95P\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x047W_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15aA1W_\x80\xFD[\x845aA<\x81a@\xFDV[\x93P` \x85\x015aAL\x81a@\xFDV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14aAbW_\x80\xFD[\x91P``\x85\x015b\xFF\xFF\xFF\x81\x16\x81\x14aAyW_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15aA\x94W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$,W_\x80\xFD[_` \x82\x84\x03\x12\x15aA\xBBW_\x80\xFD[P5\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15aA\xD8W_\x80\xFD[\x865aA\xE3\x81a@\xFDV[\x95P`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x01\x12\x15aB\x14W_\x80\xFD[` \x87\x01\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15aBIW_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aBjW_\x80\xFD[aBv\x88\x82\x89\x01a@\x80V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aB\x98W_\x80\xFD[\x825aB\xA3\x81a@\xFDV[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_`@\x84\x86\x03\x12\x15aB\xC3W_\x80\xFD[\x835aB\xCE\x81a@\xFDV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xE9W_\x80\xFD[aB\xF5\x86\x82\x87\x01a@\x80V[\x94\x97\x90\x96P\x93\x94PPPPV[_\x80_``\x84\x86\x03\x12\x15aC\x14W_\x80\xFD[\x835aC\x1F\x81a@\xFDV[\x92P` \x84\x015aC/\x81a@\xFDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_aC\xC6`\xE0\x83\x01\x89aC@V[\x82\x81\x03`@\x84\x01RaC\xD8\x81\x89aC@V[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aD:W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aD\x1CV[P\x90\x9B\x9APPPPPPPPPPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15aD^W_\x80\xFD[\x845aDi\x81a@\xFDV[\x93P` \x85\x015aDy\x81a@\xFDV[\x92P`@\x85\x015\x91P``\x85\x015aAy\x81a@\xFDV[` \x81R_a$,` \x83\x01\x84aC@V[_\x80` \x83\x85\x03\x12\x15aD\xB3W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xC9W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD\xD9W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xEFW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15aE\x03W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aE\x9CW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xB2W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aE\xC2W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xDCWaE\xDCaE_V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aFHWaFHaE_V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15aF_W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x80`\x02\x0B\x81\x14a\x047W_\x80\xFD[_` \x82\x84\x03\x12\x15aF\x9AW_\x80\xFD[\x815a$,\x81aF|V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18\xB2Wa\x18\xB2aF\xA5V[_` \x82\x84\x03\x12\x15aF\xF5W_\x80\xFD[\x815a$,\x81a@\xFDV[_` \x82\x84\x03\x12\x15aG\x10W_\x80\xFD[PQ\x91\x90PV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x18\xB2Wa\x18\xB2aF\xA5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x18\xB2Wa\x18\xB2aF\xA5V[_` \x82\x84\x03\x12\x15aG\x8FW_\x80\xFD[\x81Qa$,\x81aF|V[_\x82aG\xCDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18\xB2Wa\x18\xB2aF\xA5V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03aH\x1DWaH\x1DaF\xA5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x18\xB2Wa\x18\xB2aF\xA5V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x18\xB2Wa\x18\xB2aF\xA5V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14aH\xE6WaH\xE6aF\xA5V[P\x92\x91PPV\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610115575f3560e01c80637d1f3226116100ad57806391dd73461161007d578063d9caed1211610063578063d9caed1214610293578063d9e17f98146102a6578063f3fef3a3146102b9575f80fd5b806391dd734614610260578063d6cffd1e14610280575f80fd5b80637d1f32261461020c5780638340f5491461021f57806384b0196e146102325780638587f4501461024d575f80fd5b806321d0ee70116100e857806321d0ee701461017a578063259982e5146101be57806347e7ef24146101d15780637407905c146101e4575f80fd5b806309c5eabe146101195780631090641d1461012e578063116a5550146101415780631e2eaeaf14610154575b5f80fd5b61012c6101273660046140be565b6102cc565b005b61012c61013c36600461411e565b6103ad565b61012c61014f366004614184565b61042d565b6101676101623660046141ab565b61043a565b6040519081526020015b60405180910390f35b61018d6101883660046141c2565b610444565b6040517fffffffff000000000000000000000000000000000000000000000000000000009091168152602001610171565b61018d6101cc3660046141c2565b6107f8565b61012c6101df366004614287565b6109f9565b6101f76101f23660046142b1565b610a63565b60405163ffffffff9091168152602001610171565b61012c61021a366004614287565b610d64565b61012c61022d366004614302565b610e33565b61023a610ea2565b604051610171979695949392919061438c565b61012c61025b36600461444b565b610f4a565b61027361026e3660046140be565b611126565b6040516101719190614490565b61012c61028e3660046144a2565b6111c7565b61012c6102a1366004614302565b61125b565b61012c6102b4366004614287565b6112c1565b61012c6102c7366004614287565b611355565b6102d46113bb565b6040517f48c8949100000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906348c89491906103489085908590600401614513565b5f604051808303815f875af1158015610363573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe01682016040526103a8919081019061458c565b505050565b6103b561148e565b6004546103e79068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16858585856114ff565b600460086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050505050565b610437338261174a565b50565b5f81545f5260205ff35b5f61044d611785565b5f61045b85604001356117f4565b90505f61046787611835565b90505f806104c4838b61047d60208c018c61468a565b61048d60408d0160208e0161468a565b6006526003525f90815260608b01356026908152603a600c2090829052918152600560209081526040808320848452909152902091565b90925090505f61051661050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611881565b60a01c60020b90565b90505f61054f8261052a60208d018d61468a565b61053a60408e0160208f0161468a565b5f8981526006602052604090209291906118b8565b90505f61059373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016878661195a565b85549091505f906105b6846fffffffffffffffffffffffffffffffff85166119b5565b6105c091906146d2565b90508015610794577f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663a58411948e5f01602081019061061691906146e5565b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015610679575f80fd5b505af115801561068b573d5f803e3d5ffd5b505050506106e37f0000000000000000000000000000000000000000000000000000000000000000828f5f0160208101906106c691906146e5565b73ffffffffffffffffffffffffffffffffffffffff1691906119e0565b6040517f3dd45adb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8f811660048301527f00000000000000000000000000000000000000000000000000000000000000001690633dd45adb906024016020604051808303815f875af115801561076e573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107929190614700565b505b6107c36107a089611a29565b6107aa9084614717565b84906fffffffffffffffffffffffffffffffff166119b5565b909555507f21d0ee70000000000000000000000000000000000000000000000000000000009c9b505050505050505050505050565b5f610801611785565b60408401355f61081087611835565b90505f610820602088018861468a565b90505f6108336040890160208a0161468a565b90505f60065f8581526020019081526020015f2090505f61086a858d86868e6060013560056118499095949392919063ffffffff16565b5090505f6108b161050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001688611881565b90505f8362ffffff8716630100000081106108ce576108ce61473f565b015490505f8462ffffff8716630100000081106108ed576108ed61473f565b015490505f8760020b8460020b1215610940578183101561092f5781925082865f018962ffffff16630100000081106109285761092861473f565b015561099e565b61093982846146d2565b905061099e565b8360020b8760020b1361097d578282101561097357829150818662ffffff8916630100000081106109285761092861473f565b61093983836146d2565b8183876301000000015461099191906146d2565b61099b91906146d2565b90505b5f6109a9828c6119b5565b905080865f015f8282546109bd919061476c565b909155507f259982e5000000000000000000000000000000000000000000000000000000009c5050505050505050505050505095945050505050565b610a1b73ffffffffffffffffffffffffffffffffffffffff8316333084611a4e565b73ffffffffffffffffffffffffffffffffffffffff82165f90815260026020908152604080832033845290915281208054839290610a5a90849061476c565b90915550505050565b5f600183018335821a80610b6d57604080517fd505accf00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff881660048201523360248201527fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6044820152601484013560d81c6064820181905260198501355f90811a60848401819052601a87013560a48501819052603a88013560c486018190529551605a8901983560601c969495929491939192879263d505accf9260e48084019382900301818387803b158015610b4d575f80fd5b505af1158015610b5f573d5f803e3d5ffd5b505050505050505050610d4a565b60018160ff1603610c55576040517fd505accf00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8716600482015233602482810191909152601484013560801c604483018190529084013560d81c6064830181905260298501355f1a60848401819052602a86013560a48501819052604a87013560c48601819052606a8801973560601c95869063d505accf9060e4015b5f604051808303815f87803b158015610c34575f80fd5b505af1158015610c46573d5f803e3d5ffd5b50505050505050505050610d4a565b60028160ff1603610d0e576040517f8fcbaf0c00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff87166004820152336024820152601483013560e01c60448201819052601884013560d81c6064830181905260016084840152601d8501355f1a60a48401819052601e86013560c48501819052603e87013560e48601819052605e8801973560601c958690638fcbaf0c9061010401610c1d565b6040517f6f1d150900000000000000000000000000000000000000000000000000000000815260ff821660048201526024015b60405180910390fd5b610d55828686611aa6565b506324a2e44b95945050505050565b610d6c61148e565b60045473ffffffffffffffffffffffffffffffffffffffff6801000000000000000090910481169083168114610dce576040517ff21fd99f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610dee73ffffffffffffffffffffffffffffffffffffffff821683611ac3565b600460086101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550505050565b610e5573ffffffffffffffffffffffffffffffffffffffff8416333084611a4e565b73ffffffffffffffffffffffffffffffffffffffff8084165f90815260026020908152604080832093861683529290529081208054839290610e9890849061476c565b9091555050505050565b7f0f000000000000000000000000000000000000000000000000000000000000006060805f808083610f38604080518082018252600881527f416e677374726f6d0000000000000000000000000000000000000000000000006020808301919091528251808401909352600283527f76310000000000000000000000000000000000000000000000000000000000009083015291565b97989097965046955030945091925090565b8273ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff161115610f82579192915b5f84815260208490526040812060281b6004549091505f90610fc79068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff168386611bfd565b5090507f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16636276cbbe6040518060a001604052806110188a90565b73ffffffffffffffffffffffffffffffffffffffff1681526020018873ffffffffffffffffffffffffffffffffffffffff90811682525f602080840191909152600287810b6040808601919091523060609586015280517fffffffff0000000000000000000000000000000000000000000000000000000060e089901b168152865185166004820152928601518416602484015285015162ffffff1660448301529284015190920b60648301526080909201518216608482015290861660a482015260c4016020604051808303815f875af11580156110f9573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061111d919061477f565b50505050505050565b6060611130611785565b825f61113b82611c80565b60045491935091505f90611174908490849068010000000000000000900473ffffffffffffffffffffffffffffffffffffffff16611d51565b909350905061118282611eec565b61118c8382611f17565b92506111988382611fa1565b92506111a48382612054565b92506111b1838787611aa6565b6111ba826120f3565b60205f525f60205260405ff35b6111cf61148e565b5f5b818110156103a8575f8383838181106111ec576111ec61473f565b905060200201602081019061120191906146e5565b73ffffffffffffffffffffffffffffffffffffffff165f90815260036020526040902080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00811660ff90911615179055506001016111d1565b73ffffffffffffffffffffffffffffffffffffffff83165f9081526002602090815260408083203384529091528120805483929061129a9084906146d2565b909155506103a8905073ffffffffffffffffffffffffffffffffffffffff841683836119e0565b3373ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614611330576040517f2833655e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61135173ffffffffffffffffffffffffffffffffffffffff831633836119e0565b5050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152600260209081526040808320338452909152812080548392906113949084906146d2565b90915550611351905073ffffffffffffffffffffffffffffffffffffffff831633836119e0565b6004544367ffffffffffffffff90911603611402576040517fd8a6b89b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b335f9081526003602052604090205460ff1661144a576040517f5cd26b6800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61145343612359565b600480547fffffffffffffffffffffffffffffffffffffffffffffffff00000000000000001667ffffffffffffffff92909216919091179055565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146114fd576040517f23019e6700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff161115611538579293925b8473ffffffffffffffffffffffffffffffffffffffff168473ffffffffffffffffffffffffffffffffffffffff160361159d576040517f587daa3000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8261ffff165f036115da576040517f270815a000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b62030d4062ffffff8316111561161c576040517f76a3f95d00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f61163c8773ffffffffffffffffffffffffffffffffffffffff16612372565b90505f61165587875f9182526020526040902060281b90565b90506040516b600b380380600b5f395ff30081526020810183602002806001838d3c64ffff000000601889901b1662ffffff88161784178282015b808410156116d85783517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811687036116cc57828552506116d8565b50602084019350611690565b908152821460051b01600c8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff8516915061173f9050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505095945050505050565b80600c5263daa050e9600452815f52601f600c20600160ff83161b8082541881811661177d57638cb888725f526004601cfd5b909155505050565b3373ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146114fd576040517ff832861400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f8082131561182f576040517fcaccb6d900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505f0390565b6040515f9060a083823760a0902092915050565b6006919091526003919091525f9182526026908152603a600c2090829052918152602092835260408082208383529093529190912091565b5f8181526006602052604081206118ae73ffffffffffffffffffffffffffffffffffffffff851682612395565b9150505b92915050565b5f808562ffffff8516630100000081106118d4576118d461473f565b015490505f8662ffffff8516630100000081106118f3576118f361473f565b015490508460020b8660020b12156119185761190f81836146d2565b92505050611952565b8560020b8460020b1361192f5761190f82826146d2565b8082886301000000015461194391906146d2565b61194d91906146d2565b925050505b949350505050565b5f6006602052825f52600660405f2001602052815f5260405f20602052631e2eaeaf5f5260205f6024601c875afa6119995763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff1692915050565b5f815f190483118202156119d05763bac65e5b5f526004601cfd5b50670de0b6b3a764000091020490565b81601452806034526fa9059cbb0000000000000000000000005f5260205f604460105f875af13d1560015f51141716611a20576390b8ec185f526004601cfd5b5f603452505050565b5f7001000000000000000000000000000000008210611a4a57611a4a6123c5565b5090565b60405181606052826040528360601b602c526f23b872dd000000000000000000000000600c5260205f6064601c5f895af13d1560015f51141716611a9957637939f4245f526004601cfd5b5f60605260405250505050565b808201808414611abd576301842f8c5f526004601cfd5b50505050565b5f80611ae48473ffffffffffffffffffffffffffffffffffffffff16612372565b9050808310611b1f576040517fd2c6aae600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80600103611b2d57506118b2565b5f6001611b3a85846146d2565b611b4491906146d2565b90506040516b600b380380600b5f395ff30081526020810183602002806001838a3c60208702820191506020840260208301835e7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffec8101601484015ff095505073ffffffffffffffffffffffffffffffffffffffff85169150611bf59050576040517f5670258700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b505092915050565b5f805f6020846020026001015f883c505f517fffffffffffffffffffffffffffffffffffffffffffffffffffffff0000000000811685140280611c6c576040517f2f659e4400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61ffff601882901c16969095509350505050565b6003818101915f918291803560e81c0101816044611c9e86846146d2565b611ca8919061479a565b905080602086901b1792505f805b82811015611d45575f611cd4602087901c60448402015b3560601c90565b90508273ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1611611d3b576040517f80f11acf00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b9150600101611cb6565b50829450505050915091565b6003838101935f91829182918291803560e81c0101816026611d738a846146d2565b611d7d919061479a565b905060405193508060c0028401925082604052808460201b179450505f5b82841015611edf5760048901983560e081901c905f90611dc390611ccd908c9060f01c6123d2565b90505f611dd7611ccd8c61ffff86166123d2565b90508363ffffffff168363ffffffff16111580611e2057508073ffffffffffffffffffffffffffffffffffffffff168273ffffffffffffffffffffffffffffffffffffffff1610155b15611e57576040517ff35f939900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b90865260208601526040852060028b019a91925060281b903560f01c5f80611e9673ffffffffffffffffffffffffffffffffffffffff8c168585611bfd565b60408a01919091526060890152505050893560808601819052760a70c3c40a64e6c51999090b65f67d92400000000000000460a08601525060209098019760c090930192611d9b565b5093505050935093915050565b63ffffffff81165f5b818110156103a857611f0f602084901c6044830201612433565b600101611ef5565b60408051610160810182525f60208201819052918101829052606081018290526080810182905260c0810182905260e081018290526101008101829052610140810182905263f3cd914c81523060a082015261012080820152600384810194803560e81c0101905b818514611f9857611f91858286612504565b9450611f7f565b50929392505050565b6003828101925f91813560e81c9091010181611fbb612741565b60408051610120810182525f60208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e08101919091527f695b2e2b8250e7bc714aad670b4cda55f770437f41399d6f92ff0e64a36ddbfa815267ffffffffffffffff43166101008201529091505b82861461204a576120438682848861278b565b9550612030565b5093949350505050565b5f8061205e612741565b604080516101a0810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810182905261010081018290526101208101829052610140810182905261016081018290526101808101919091526003868101969293509091803560e81c01015b80861461204a576120ec86838588612990565b95506120d9565b6040805163ffffffff8316602481028201909252805f5b838110156123465760448102602086901c01803560601c6014820135608090811c906034840135901c5f61214b84612142848661476c565b60019190612b80565b121561219b576040517fd49b70f500000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff84166004820152602401610d41565b80156123295773ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001663a5841194846040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b16815273ffffffffffffffffffffffffffffffffffffffff90911660048201526024015f604051808303815f87803b158015612241575f80fd5b505af1158015612253573d5f803e3d5ffd5b506122999250505073ffffffffffffffffffffffffffffffffffffffff84167f0000000000000000000000000000000000000000000000000000000000000000836119e0565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166311da60b46040518163ffffffff1660e01b81526004016020604051808303815f875af1158015612303573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906123279190614700565b505b6123338487612bc3565b505050602492909201915060010161210a565b506024830282205f5260205fa050505050565b5f680100000000000000008210611a4a57611a4a6123c5565b5f6118b2602073ffffffffffffffffffffffffffffffffffffffff84163b61479a565b5f81602052631e2eaeaf5f5260205f6024601c865afa6123bc5763535cf94b5f526004601cfd5b50505f51919050565b6335278d125f526004601cfd5b5f8163ffffffff841611612421576040517fffc31e710000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401610d41565b602083901c60448302015b9392505050565b602481013560801c801561135157604080517f0b0d9c09000000000000000000000000000000000000000000000000000000008152833560601c600482018190523060248301526044820184905291517f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1691630b0d9c09916064808301925f92919082900301818387803b1580156124df575f80fd5b505af11580156124f1573d5f803e3d5ffd5b506103a892506001915083905084612bcc565b6001838101935f919035821a9061252090859083161515612c05565b60028501943560f01c6125476125368583612c56565b805160208201516040909201519092565b60020b608088015273ffffffffffffffffffffffffffffffffffffffff9081166040880152166020860190815260a090205f60108801883560801c9098506fffffffffffffffffffffffffffffffff1690505f81156126ae575f6125e461050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611881565b90506125ef83612cb6565b60e08a015261261e897f0000000000000000000000000000000000000000000000000000000000000000612d11565b61266161050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001686611881565b60808a01515f8681526006602052604090209193506126a8919086907f00000000000000000000000000000000000000000000000000000000000000009085908790612d2f565b506126f4565b6126f161050d73ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001685611881565b90505b5f61271b6002871615155f86815260066020526040902060808c01518d9190889087612db8565b60208b0151919b5091506127329060019083612b80565b50989998505050505050505050565b5f61278661274d613035565b60408051604281019091527f19010000000000000000000000000000000000000000000000000000000000008152600281019190915290565b905090565b83355f90811a6001818116151560808781019190915290870135811c60208701526011870135811c60408701526021870135811c6060870181905260418801976031013590911c9081111561280c576040517f2bae6c5200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600282161561284157806fffffffffffffffffffffffffffffffff1686602001818151612839919061476c565b905250612869565b806fffffffffffffffffffffffffffffffff168660400181815161286591906146d2565b9052505b506002868101963560f01c9061289990831615156128878684612c56565b9060051b602081188201519101519091565b73ffffffffffffffffffffffffffffffffffffffff90811660c08901521660a087015250600481166128cc57855f6128d6565b60148601863560601c5b73ffffffffffffffffffffffffffffffffffffffff1660e087015295505f61290f61290387610120902090565b60228701526042862090565b905061291a8161312d565b5f600883166129325761292d888361317e565b61293c565b61293c88836131e8565b60e089015160a08a015160208b0151939b5091935080158402179161296891849160018816151561322c565b60c088015160408901516129839183916001881615156132b6565b5096979650505050505050565b5f8061299c858761332e565b60028201975091505f9081903560f01c6129c56008851615156129bf8884612c56565b9061340e565b73ffffffffffffffffffffffffffffffffffffffff9182166101008c0152911660e08a01529250505060208701873560a08801819052909750811015612a37576040517f8e1edfa400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60028216612a4657865f612a50565b60148701873560601c5b73ffffffffffffffffffffffffffffffffffffffff1661012088015296505f612a7d886004851615613450565b6101408a01529098509050612a938789856134fb565b97505f80612aa3898b8787613543565b919b50925090505f612ac4612ab88b8861374a565b60228b015260428a2090565b90505f60808716612ade57612ad98c8361317e565b612ae8565b612ae88c836131e8565b909c5090506010871615612b1f57612b0b8b610180015164ffffffffff1661376a565b612b1a818c610160015161174a565b612b28565b612b288261312d565b6101208b01516101008c01518115830290911790612b4e9082908660018c1615156132b6565b612b5886836137a4565b60e08c0151612b6f9083908760018c16151561322c565b509a9b9a5050505050505050505050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260408120612bbb612bb2825c856137ec565b92508183613804565b509392505050565b60248282375050565b73ffffffffffffffffffffffffffffffffffffffff82165f908152602084905260409020611abd612bfe825c8461380b565b8290613804565b80151560c083015280612c2c5773fffd8963efd1fc6a506488495d951d5263988d25612c33565b6401000276a45b73ffffffffffffffffffffffffffffffffffffffff166101009092019190915250565b5f8163ffffffff841611612ca5576040517ff6601b500000000000000000000000000000000000000000000000000000000081526004810183905263ffffffff84166024820152604401610d41565b5060c08102602083901c0192915050565b5f7f800000000000000000000000000000000000000000000000000000000000000082111561182f576040517f35278d1200000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f80610144601c85015f855af1806103a8576040513d5f823e503d5ff35b8260020b8260020b1315612d73578260020b612d57828460020b61382390919063ffffffff16565b60020b1315612d6e57612d6e868587868686613834565b612db0565b8260020b8260020b1215612db0575f600284900b828107919091129082900503810260020b8260020b1215612db057612db08685878686866138c7565b505050505050565b5f808715612e5a5760108701963560801c612e2481612e0d7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1689613966565b6fffffffffffffffffffffffffffffffff166139a0565b876301000000015f828254612e39919061476c565b90915550889350506fffffffffffffffffffffffffffffffff16905061302a565b5f808060038a018a3560e81d909a5090505f60108b018b3560801c909b5090505f806003808e01908e3560e81c8f0101604080516060810182528e815260028e810b60208301528d810b9282018390529395509193508e925f92919088900b1315612ed157612ecc83888789856139bb565b612ede565b612ede8388878985613b04565b909b50995060108201965092503560801c612f0b816fffffffffffffffffffffffffffffffff8b166139a0565b612f15908b61476c565b9950612f336fffffffffffffffffffffffffffffffff82168461476c565b9250612f3f8686613c4e565b81515f90612f849073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001690613966565b9050806fffffffffffffffffffffffffffffffff168a6fffffffffffffffffffffffffffffffff1614612fff576040517f6429cfd20000000000000000000000000000000000000000000000000000000081526fffffffffffffffffffffffffffffffff808c16600483015282166024820152604401610d41565b8a856301000000015f828254613015919061476c565b90915550969c50929a50505050505050505050505b965096945050505050565b7f00000000000000000000000000000000000000000000000000000000000000007f000000000000000000000000000000000000000000000000000000000000000030147f000000000000000000000000000000000000000000000000000000000000000046141661312a5750604080517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f81527f000000000000000000000000000000000000000000000000000000000000000060208201527f00000000000000000000000000000000000000000000000000000000000000009181019190915246606082015230608082015260a0902090565b90565b5f818152602081905260409020805c15613173576040517f8a2ef11600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b611351816001613804565b6017601483013560e81c8084018201935f92813560601c929101906131a583868484613c87565b6131db576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b85935050505b9250929050565b5f806040518381525f6020820152604185603f8301376041850194506020600160808360015afa519150503d61322557638baa579f5f526004601cfd5b9293915050565b8161323960018583612bcc565b811561328d5773ffffffffffffffffffffffffffffffffffffffff8086165f908152600260209081526040808320938816835292905290812080548392906132829084906146d2565b909155506132af9050565b6132af73ffffffffffffffffffffffffffffffffffffffff8516863084611a4e565b5050505050565b816132c360018583612b80565b50811561330d5773ffffffffffffffffffffffffffffffffffffffff8086165f9081526002602090815260408083209388168352929052908120805483929061328290849061476c565b6132af73ffffffffffffffffffffffffffffffffffffffff851686836119e0565b60018101905f9035811a600483603c8601376004929092019160208116156133a7576010811661337e577f6ee89dee573705c024a086e19a128ee0a5ee0547e3283adfa72fbe336a4c4b6c6133a0565b7f6be5f22bdcd037f6f35250c32e478fad62195ac2bbab1e2932f8c97af926b4915b84526133fa565b601081166133d5577f022e170cdf338f45bc718f58d29bfafbf3956c2f9ea8d19ccc9b72e42dbbb7b06133f7565b7fb0617b84f694c245e54fb8032ebdc9f56eb26ea2c1b65a46c58f50dbd516e2865b84525b60018116151560c094909401939093525091565b600581901b6020811883015190830180516080909101516060850151620f42409081039061343c82846147d2565b613446919061479a565b9150509250925092565b5f807fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470836134f157843560e81c6003860195506040516014606403810182810160405282888237828120935050818701965060448101517f7407905c00000000000000000000000000000000000000000000000000000000825260406024830152601483039250826044830152606483018160201b178260c01b1794505050505b8492509250925092565b5f6010821615613529576008836101788601376008929092019160058361019b86013760058301925061353b565b67ffffffffffffffff43166101608501525b509092915050565b5f80808060208616156135f557508535608090811c604089018190526010880135821c60608a0181905260308901986020013590921c91818310156135b4576040517fc4daf00300000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b808311156135ee576040517f4418233100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5050613620565b5060108601953560801c6040861661360d575f613610565b60015b60ff166040890152606088018190525b60208701966010810135608090811c9135901c8082111561366d576040517f668fef1b00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6fffffffffffffffffffffffffffffffff1660808a015260088716156136f65760608716156136c5576136b26fffffffffffffffffffffffffffffffff8216836146d2565b93506136be8685613ccc565b925061373c565b90915081906136ef6136d78784613cd7565b826fffffffffffffffffffffffffffffffff16613ce2565b935061373c565b60608716156137115790925082906136be6136d78784613ccc565b61372d6fffffffffffffffffffffffffffffffff8216836146d2565b92506137398684613cd7565b93505b509597919650945092505050565b5f806010831661375c57610180613760565b6101a05b9093209392505050565b80421115610437576040517f203d82d800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b81156113515763ffffffff82168260c01c8260048201528360201c60205f84845f855af1925050506324a2e44b5f5114601f3d111681166103a85763f959fdae5f526004601cfd5b808203828113156118b25763c9654ed45f526004601cfd5b80825d5050565b818101828112156118b25763c9654ed45f526004601cfd5b5f8183071291819005919091030290565b5f61385773ffffffffffffffffffffffffffffffffffffffff8716868685613ced565b94509050600284810b9084900b12156138705750612db0565b80156138c1578662ffffff8516630100000081106138905761389061473f565b015487630100000001546138a491906146d2565b8762ffffff8616630100000081106138be576138be61473f565b01555b50613834565b5f6138ea73ffffffffffffffffffffffffffffffffffffffff8716868685613d5f565b94509050600283810b9085900b136139025750612db0565b8015613953578662ffffff8516630100000081106139225761392261473f565b0154876301000000015461393691906146d2565b8762ffffff8616630100000081106139505761395061473f565b01555b8361395d816147e9565b945050506138c7565b5f8181526006602052604081205f61399773ffffffffffffffffffffffffffffffffffffffff861660038401612395565b95945050505050565b5f61242c826139b7670de0b6b3a7640000866147d2565b0490565b5f808080600181805b8215613a8e5760108a01993560801c6139dd818461476c565b92506139fb818b6fffffffffffffffffffffffffffffffff166139a0565b613a05908361476c565b9150818d8d62ffffff1663010000008110613a2257613a2261473f565b015f828254613a31919061476c565b909155505088515f90613a7c9073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613da4565b915050613a898b82613e09565b9a5050505b87516020890151613ad89173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613e23565b809c508194505050876040015160020b8b60020b136139c457989b909a50979850959695505050505050565b5f808080600181805b8215613bd75760108a01993560801c613b26818461476c565b9250613b44818b6fffffffffffffffffffffffffffffffff166139a0565b613b4e908361476c565b9150818d8d62ffffff1663010000008110613b6b57613b6b61473f565b015f828254613b7a919061476c565b909155505088515f90613bc59073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016908f613da4565b915050613bd28b82613e3d565b9a5050505b87516020890151613c219173ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016918e90613ced565b809c508194505050876040015160020b8b60020b1315613b0d57989b909a50979850959695505050505050565b808214611351576040517f01842f8c00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b5f604051631626ba7e60e01b80825285600483015260248201604081528460448401528486606485013760208160648701858b5afa9051909114169695505050505050565b5f61242c8284613e57565b5f61242c8284613e79565b5f61242c82846146d2565b5f808080613d12613d078688078313878905036001614845565b600281900b60081d91565b9092509050613d4281613d3c73ffffffffffffffffffffffffffffffffffffffff8b168a86613e91565b90613ecc565b9094509050613d52828287613f8e565b9250505094509492505050565b5f808080613d74858707821386880503613d07565b9092509050613d4281613d9e73ffffffffffffffffffffffffffffffffffffffff8b168a86613e91565b90613fb8565b5f806006602052835f52600460405f2001602052825f5260405f20602052631e2eaeaf5f5260205f6024601c885afa613de45763535cf94b5f526004601cfd5b50505f516fffffffffffffffffffffffffffffffff81169460809190911d9350915050565b808203608081901c156118b25763c9654ed45f526004601cfd5b5f808080613d74613d0760018789078413888a0503614886565b818101608081901c156118b25763c9654ed45f526004601cfd5b5f6b033b2e3c9fd0803ce8000000613e6f83856147d2565b61242c919061479a565b5f81613e6f6b033b2e3c9fd0803ce8000000856147d2565b5f828152600660209081526040808320848452600501909152812061399773ffffffffffffffffffffffffffffffffffffffff861682612395565b5f805f613f678460ff1686901c7e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b150316040581196001019091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b9050806101001415925082613f7d5760ff613f84565b8360ff1681015b9150509250929050565b5f8160ff8416613fa4600187900b6101006148c7565b613fae9190614845565b61195291906148c7565b5f805f8360ff0390505f6140598260ff1687901b7f0706060506020504060203020504030106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be831560081b6fffffffffffffffffffffffffffffffff851160071b1784811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b905080610100141593508361406e575f614075565b8160ff1681035b925050509250929050565b5f8083601f840112614090575f80fd5b50813567ffffffffffffffff8111156140a7575f80fd5b6020830191508360208285010111156131e1575f80fd5b5f80602083850312156140cf575f80fd5b823567ffffffffffffffff8111156140e5575f80fd5b6140f185828601614080565b90969095509350505050565b73ffffffffffffffffffffffffffffffffffffffff81168114610437575f80fd5b5f805f8060808587031215614131575f80fd5b843561413c816140fd565b9350602085013561414c816140fd565b9250604085013561ffff81168114614162575f80fd5b9150606085013562ffffff81168114614179575f80fd5b939692955090935050565b5f60208284031215614194575f80fd5b813567ffffffffffffffff8116811461242c575f80fd5b5f602082840312156141bb575f80fd5b5035919050565b5f805f805f8587036101608112156141d8575f80fd5b86356141e3816140fd565b955060a07fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe082011215614214575f80fd5b60208701945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff4082011215614249575f80fd5b5060c08601925061014086013567ffffffffffffffff81111561426a575f80fd5b61427688828901614080565b969995985093965092949392505050565b5f8060408385031215614298575f80fd5b82356142a3816140fd565b946020939093013593505050565b5f805f604084860312156142c3575f80fd5b83356142ce816140fd565b9250602084013567ffffffffffffffff8111156142e9575f80fd5b6142f586828701614080565b9497909650939450505050565b5f805f60608486031215614314575f80fd5b833561431f816140fd565b9250602084013561432f816140fd565b929592945050506040919091013590565b5f81518084528060208401602086015e5f6020828601015260207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010191505092915050565b7fff000000000000000000000000000000000000000000000000000000000000008816815260e060208201525f6143c660e0830189614340565b82810360408401526143d88189614340565b6060840188905273ffffffffffffffffffffffffffffffffffffffff8716608085015260a0840186905283810360c0850152845180825260208087019350909101905f5b8181101561443a57835183526020938401939092019160010161441c565b50909b9a5050505050505050505050565b5f805f806080858703121561445e575f80fd5b8435614469816140fd565b93506020850135614479816140fd565b9250604085013591506060850135614179816140fd565b602081525f61242c6020830184614340565b5f80602083850312156144b3575f80fd5b823567ffffffffffffffff8111156144c9575f80fd5b8301601f810185136144d9575f80fd5b803567ffffffffffffffff8111156144ef575f80fd5b8560208260051b8401011115614503575f80fd5b6020919091019590945092505050565b60208152816020820152818360408301375f818301604090810191909152601f9092017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0160101919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b5f6020828403121561459c575f80fd5b815167ffffffffffffffff8111156145b2575f80fd5b8201601f810184136145c2575f80fd5b805167ffffffffffffffff8111156145dc576145dc61455f565b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f8501160116810181811067ffffffffffffffff821117156146485761464861455f565b60405281815282820160200186101561465f575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b8060020b8114610437575f80fd5b5f6020828403121561469a575f80fd5b813561242c8161467c565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b818103818111156118b2576118b26146a5565b5f602082840312156146f5575f80fd5b813561242c816140fd565b5f60208284031215614710575f80fd5b5051919050565b6fffffffffffffffffffffffffffffffff82811682821603908111156118b2576118b26146a5565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b808201808211156118b2576118b26146a5565b5f6020828403121561478f575f80fd5b815161242c8161467c565b5f826147cd577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b80820281158282048414176118b2576118b26146a5565b5f8160020b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000810361481d5761481d6146a5565b7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0192915050565b600281810b9083900b01627fffff81137fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff800000821217156118b2576118b26146a5565b600282810b9082900b037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8000008112627fffff821317156118b2576118b26146a5565b5f8260020b8260020b028060020b91508082146148e6576148e66146a5565b509291505056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x01\x15W_5`\xE0\x1C\x80c}\x1F2&\x11a\0\xADW\x80c\x91\xDDsF\x11a\0}W\x80c\xD9\xCA\xED\x12\x11a\0cW\x80c\xD9\xCA\xED\x12\x14a\x02\x93W\x80c\xD9\xE1\x7F\x98\x14a\x02\xA6W\x80c\xF3\xFE\xF3\xA3\x14a\x02\xB9W_\x80\xFD[\x80c\x91\xDDsF\x14a\x02`W\x80c\xD6\xCF\xFD\x1E\x14a\x02\x80W_\x80\xFD[\x80c}\x1F2&\x14a\x02\x0CW\x80c\x83@\xF5I\x14a\x02\x1FW\x80c\x84\xB0\x19n\x14a\x022W\x80c\x85\x87\xF4P\x14a\x02MW_\x80\xFD[\x80c!\xD0\xEEp\x11a\0\xE8W\x80c!\xD0\xEEp\x14a\x01zW\x80c%\x99\x82\xE5\x14a\x01\xBEW\x80cG\xE7\xEF$\x14a\x01\xD1W\x80ct\x07\x90\\\x14a\x01\xE4W_\x80\xFD[\x80c\t\xC5\xEA\xBE\x14a\x01\x19W\x80c\x10\x90d\x1D\x14a\x01.W\x80c\x11jUP\x14a\x01AW\x80c\x1E.\xAE\xAF\x14a\x01TW[_\x80\xFD[a\x01,a\x01'6`\x04a@\xBEV[a\x02\xCCV[\0[a\x01,a\x01<6`\x04aA\x1EV[a\x03\xADV[a\x01,a\x01O6`\x04aA\x84V[a\x04-V[a\x01ga\x01b6`\x04aA\xABV[a\x04:V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Da\x01\x886`\x04aA\xC2V[a\x04DV[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x81R` \x01a\x01qV[a\x01\x8Da\x01\xCC6`\x04aA\xC2V[a\x07\xF8V[a\x01,a\x01\xDF6`\x04aB\x87V[a\t\xF9V[a\x01\xF7a\x01\xF26`\x04aB\xB1V[a\ncV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01qV[a\x01,a\x02\x1A6`\x04aB\x87V[a\rdV[a\x01,a\x02-6`\x04aC\x02V[a\x0E3V[a\x02:a\x0E\xA2V[`@Qa\x01q\x97\x96\x95\x94\x93\x92\x91\x90aC\x8CV[a\x01,a\x02[6`\x04aDKV[a\x0FJV[a\x02sa\x02n6`\x04a@\xBEV[a\x11&V[`@Qa\x01q\x91\x90aD\x90V[a\x01,a\x02\x8E6`\x04aD\xA2V[a\x11\xC7V[a\x01,a\x02\xA16`\x04aC\x02V[a\x12[V[a\x01,a\x02\xB46`\x04aB\x87V[a\x12\xC1V[a\x01,a\x02\xC76`\x04aB\x87V[a\x13UV[a\x02\xD4a\x13\xBBV[`@Q\x7FH\xC8\x94\x91\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cH\xC8\x94\x91\x90a\x03H\x90\x85\x90\x85\x90`\x04\x01aE\x13V[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x03cW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\x03\xA8\x91\x90\x81\x01\x90aE\x8CV[PPPV[a\x03\xB5a\x14\x8EV[`\x04Ta\x03\xE7\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85\x85\x85a\x14\xFFV[`\x04`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPV[a\x0473\x82a\x17JV[PV[_\x81T_R` _\xF3[_a\x04Ma\x17\x85V[_a\x04[\x85`@\x015a\x17\xF4V[\x90P_a\x04g\x87a\x185V[\x90P_\x80a\x04\xC4\x83\x8Ba\x04}` \x8C\x01\x8CaF\x8AV[a\x04\x8D`@\x8D\x01` \x8E\x01aF\x8AV[`\x06R`\x03R_\x90\x81R``\x8B\x015`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R`\x05` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x91V[\x90\x92P\x90P_a\x05\x16a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x18\x81V[`\xA0\x1C`\x02\x0B\x90V[\x90P_a\x05O\x82a\x05*` \x8D\x01\x8DaF\x8AV[a\x05:`@\x8E\x01` \x8F\x01aF\x8AV[_\x89\x81R`\x06` R`@\x90 \x92\x91\x90a\x18\xB8V[\x90P_a\x05\x93s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x87\x86a\x19ZV[\x85T\x90\x91P_\x90a\x05\xB6\x84o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16a\x19\xB5V[a\x05\xC0\x91\x90aF\xD2V[\x90P\x80\x15a\x07\x94W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xA5\x84\x11\x94\x8E_\x01` \x81\x01\x90a\x06\x16\x91\x90aF\xE5V[`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x06yW_\x80\xFD[PZ\xF1\x15\x80\x15a\x06\x8BW=_\x80>=_\xFD[PPPPa\x06\xE3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x8F_\x01` \x81\x01\x90a\x06\xC6\x91\x90aF\xE5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x90a\x19\xE0V[`@Q\x7F=\xD4Z\xDB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c=\xD4Z\xDB\x90`$\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x07nW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x92\x91\x90aG\0V[P[a\x07\xC3a\x07\xA0\x89a\x1A)V[a\x07\xAA\x90\x84aG\x17V[\x84\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x19\xB5V[\x90\x95UP\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9C\x9BPPPPPPPPPPPPV[_a\x08\x01a\x17\x85V[`@\x84\x015_a\x08\x10\x87a\x185V[\x90P_a\x08 ` \x88\x01\x88aF\x8AV[\x90P_a\x083`@\x89\x01` \x8A\x01aF\x8AV[\x90P_`\x06_\x85\x81R` \x01\x90\x81R` \x01_ \x90P_a\x08j\x85\x8D\x86\x86\x8E``\x015`\x05a\x18I\x90\x95\x94\x93\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x90P_a\x08\xB1a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x88a\x18\x81V[\x90P_\x83b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xCEWa\x08\xCEaG?V[\x01T\x90P_\x84b\xFF\xFF\xFF\x87\x16c\x01\0\0\0\x81\x10a\x08\xEDWa\x08\xEDaG?V[\x01T\x90P_\x87`\x02\x0B\x84`\x02\x0B\x12\x15a\t@W\x81\x83\x10\x15a\t/W\x81\x92P\x82\x86_\x01\x89b\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a\t(Wa\t(aG?V[\x01Ua\t\x9EV[a\t9\x82\x84aF\xD2V[\x90Pa\t\x9EV[\x83`\x02\x0B\x87`\x02\x0B\x13a\t}W\x82\x82\x10\x15a\tsW\x82\x91P\x81\x86b\xFF\xFF\xFF\x89\x16c\x01\0\0\0\x81\x10a\t(Wa\t(aG?V[a\t9\x83\x83aF\xD2V[\x81\x83\x87c\x01\0\0\0\x01Ta\t\x91\x91\x90aF\xD2V[a\t\x9B\x91\x90aF\xD2V[\x90P[_a\t\xA9\x82\x8Ca\x19\xB5V[\x90P\x80\x86_\x01_\x82\x82Ta\t\xBD\x91\x90aGlV[\x90\x91UP\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x9CPPPPPPPPPPPPP\x95\x94PPPPPV[a\n\x1Bs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x1630\x84a\x1ANV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\nZ\x90\x84\x90aGlV[\x90\x91UPPPPV[_`\x01\x83\x01\x835\x82\x1A\x80a\x0BmW`@\x80Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16`\x04\x82\x01R3`$\x82\x01R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`D\x82\x01R`\x14\x84\x015`\xD8\x1C`d\x82\x01\x81\x90R`\x19\x85\x015_\x90\x81\x1A`\x84\x84\x01\x81\x90R`\x1A\x87\x015`\xA4\x85\x01\x81\x90R`:\x88\x015`\xC4\x86\x01\x81\x90R\x95Q`Z\x89\x01\x985``\x1C\x96\x94\x95\x92\x94\x91\x93\x91\x92\x87\x92c\xD5\x05\xAC\xCF\x92`\xE4\x80\x84\x01\x93\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a\x0BMW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0B_W=_\x80>=_\xFD[PPPPPPPPPa\rJV[`\x01\x81`\xFF\x16\x03a\x0CUW`@Q\x7F\xD5\x05\xAC\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R3`$\x82\x81\x01\x91\x90\x91R`\x14\x84\x015`\x80\x1C`D\x83\x01\x81\x90R\x90\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`)\x85\x015_\x1A`\x84\x84\x01\x81\x90R`*\x86\x015`\xA4\x85\x01\x81\x90R`J\x87\x015`\xC4\x86\x01\x81\x90R`j\x88\x01\x975``\x1C\x95\x86\x90c\xD5\x05\xAC\xCF\x90`\xE4\x01[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C4W_\x80\xFD[PZ\xF1\x15\x80\x15a\x0CFW=_\x80>=_\xFD[PPPPPPPPPPa\rJV[`\x02\x81`\xFF\x16\x03a\r\x0EW`@Q\x7F\x8F\xCB\xAF\x0C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x04\x82\x01R3`$\x82\x01R`\x14\x83\x015`\xE0\x1C`D\x82\x01\x81\x90R`\x18\x84\x015`\xD8\x1C`d\x83\x01\x81\x90R`\x01`\x84\x84\x01R`\x1D\x85\x015_\x1A`\xA4\x84\x01\x81\x90R`\x1E\x86\x015`\xC4\x85\x01\x81\x90R`>\x87\x015`\xE4\x86\x01\x81\x90R`^\x88\x01\x975``\x1C\x95\x86\x90c\x8F\xCB\xAF\x0C\x90a\x01\x04\x01a\x0C\x1DV[`@Q\x7Fo\x1D\x15\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\xFF\x82\x16`\x04\x82\x01R`$\x01[`@Q\x80\x91\x03\x90\xFD[a\rU\x82\x86\x86a\x1A\xA6V[Pc$\xA2\xE4K\x95\x94PPPPPV[a\rla\x14\x8EV[`\x04Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFh\x01\0\0\0\0\0\0\0\0\x90\x91\x04\x81\x16\x90\x83\x16\x81\x14a\r\xCEW`@Q\x7F\xF2\x1F\xD9\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\xEEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83a\x1A\xC3V[`\x04`\x08a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPV[a\x0EUs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x1630\x84a\x1ANV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x84\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a\x0E\x98\x90\x84\x90aGlV[\x90\x91UPPPPPV[\x7F\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x80_\x80\x80\x83a\x0F8`@\x80Q\x80\x82\x01\x82R`\x08\x81R\x7FAngstrom\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x80\x83\x01\x91\x90\x91R\x82Q\x80\x84\x01\x90\x93R`\x02\x83R\x7Fv1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x83\x01R\x91V[\x97\x98\x90\x97\x96PF\x95P0\x94P\x91\x92P\x90V[\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x0F\x82W\x91\x92\x91[_\x84\x81R` \x84\x90R`@\x81 `(\x1B`\x04T\x90\x91P_\x90a\x0F\xC7\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x86a\x1B\xFDV[P\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16cbv\xCB\xBE`@Q\x80`\xA0\x01`@R\x80a\x10\x18\x8A\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x82R_` \x80\x84\x01\x91\x90\x91R`\x02\x87\x81\x0B`@\x80\x86\x01\x91\x90\x91R0``\x95\x86\x01R\x80Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x89\x90\x1B\x16\x81R\x86Q\x85\x16`\x04\x82\x01R\x92\x86\x01Q\x84\x16`$\x84\x01R\x85\x01Qb\xFF\xFF\xFF\x16`D\x83\x01R\x92\x84\x01Q\x90\x92\x0B`d\x83\x01R`\x80\x90\x92\x01Q\x82\x16`\x84\x82\x01R\x90\x86\x16`\xA4\x82\x01R`\xC4\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xF9W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x1D\x91\x90aG\x7FV[PPPPPPPV[``a\x110a\x17\x85V[\x82_a\x11;\x82a\x1C\x80V[`\x04T\x91\x93P\x91P_\x90a\x11t\x90\x84\x90\x84\x90h\x01\0\0\0\0\0\0\0\0\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x1DQV[\x90\x93P\x90Pa\x11\x82\x82a\x1E\xECV[a\x11\x8C\x83\x82a\x1F\x17V[\x92Pa\x11\x98\x83\x82a\x1F\xA1V[\x92Pa\x11\xA4\x83\x82a TV[\x92Pa\x11\xB1\x83\x87\x87a\x1A\xA6V[a\x11\xBA\x82a \xF3V[` _R_` R`@_\xF3[a\x11\xCFa\x14\x8EV[_[\x81\x81\x10\x15a\x03\xA8W_\x83\x83\x83\x81\x81\x10a\x11\xECWa\x11\xECaG?V[\x90P` \x02\x01` \x81\x01\x90a\x12\x01\x91\x90aF\xE5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R`\x03` R`@\x90 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x81\x16`\xFF\x90\x91\x16\x15\x17\x90UP`\x01\x01a\x11\xD1V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x12\x9A\x90\x84\x90aF\xD2V[\x90\x91UPa\x03\xA8\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x19\xE0V[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x130W`@Q\x7F(3e^\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x19\xE0V[PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x13\x94\x90\x84\x90aF\xD2V[\x90\x91UPa\x13Q\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x163\x83a\x19\xE0V[`\x04TCg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x03a\x14\x02W`@Q\x7F\xD8\xA6\xB8\x9B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3_\x90\x81R`\x03` R`@\x90 T`\xFF\x16a\x14JW`@Q\x7F\\\xD2kh\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14SCa#YV[`\x04\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14\xFDW`@Q\x7F#\x01\x9Eg\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[V[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x158W\x92\x93\x92[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a\x15\x9DW`@Q\x7FX}\xAA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\xFF\xFF\x16_\x03a\x15\xDAW`@Q\x7F'\x08\x15\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[b\x03\r@b\xFF\xFF\xFF\x83\x16\x11\x15a\x16\x1CW`@Q\x7Fv\xA3\xF9]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x16<\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a#rV[\x90P_a\x16U\x87\x87_\x91\x82R` R`@\x90 `(\x1B\x90V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8D<d\xFF\xFF\0\0\0`\x18\x89\x90\x1B\x16b\xFF\xFF\xFF\x88\x16\x17\x84\x17\x82\x82\x01[\x80\x84\x10\x15a\x16\xD8W\x83Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x87\x03a\x16\xCCW\x82\x85RPa\x16\xD8V[P` \x84\x01\x93Pa\x16\x90V[\x90\x81R\x82\x14`\x05\x1B\x01`\x0C\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x17?\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x95\x94PPPPPV[\x80`\x0CRc\xDA\xA0P\xE9`\x04R\x81_R`\x1F`\x0C `\x01`\xFF\x83\x16\x1B\x80\x82T\x18\x81\x81\x16a\x17}Wc\x8C\xB8\x88r_R`\x04`\x1C\xFD[\x90\x91UPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14\xFDW`@Q\x7F\xF82\x86\x14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80\x82\x13\x15a\x18/W`@Q\x7F\xCA\xCC\xB6\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P_\x03\x90V[`@Q_\x90`\xA0\x83\x827`\xA0\x90 \x92\x91PPV[`\x06\x91\x90\x91R`\x03\x91\x90\x91R_\x91\x82R`&\x90\x81R`:`\x0C \x90\x82\x90R\x91\x81R` \x92\x83R`@\x80\x82 \x83\x83R\x90\x93R\x91\x90\x91 \x91V[_\x81\x81R`\x06` R`@\x81 a\x18\xAEs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x82a#\x95V[\x91PP[\x92\x91PPV[_\x80\x85b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x18\xD4Wa\x18\xD4aG?V[\x01T\x90P_\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a\x18\xF3Wa\x18\xF3aG?V[\x01T\x90P\x84`\x02\x0B\x86`\x02\x0B\x12\x15a\x19\x18Wa\x19\x0F\x81\x83aF\xD2V[\x92PPPa\x19RV[\x85`\x02\x0B\x84`\x02\x0B\x13a\x19/Wa\x19\x0F\x82\x82aF\xD2V[\x80\x82\x88c\x01\0\0\0\x01Ta\x19C\x91\x90aF\xD2V[a\x19M\x91\x90aF\xD2V[\x92PPP[\x94\x93PPPPV[_`\x06` R\x82_R`\x06`@_ \x01` R\x81_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x87Z\xFAa\x19\x99WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92\x91PPV[_\x81_\x19\x04\x83\x11\x82\x02\x15a\x19\xD0Wc\xBA\xC6^[_R`\x04`\x1C\xFD[Pg\r\xE0\xB6\xB3\xA7d\0\0\x91\x02\x04\x90V[\x81`\x14R\x80`4Ro\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0_R` _`D`\x10_\x87Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1A Wc\x90\xB8\xEC\x18_R`\x04`\x1C\xFD[_`4RPPPV[_p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x1AJWa\x1AJa#\xC5V[P\x90V[`@Q\x81``R\x82`@R\x83``\x1B`,Ro#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0`\x0CR` _`d`\x1C_\x89Z\xF1=\x15`\x01_Q\x14\x17\x16a\x1A\x99Wcy9\xF4$_R`\x04`\x1C\xFD[_``R`@RPPPPV[\x80\x82\x01\x80\x84\x14a\x1A\xBDWc\x01\x84/\x8C_R`\x04`\x1C\xFD[PPPPV[_\x80a\x1A\xE4\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a#rV[\x90P\x80\x83\x10a\x1B\x1FW`@Q\x7F\xD2\xC6\xAA\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\x01\x03a\x1B-WPa\x18\xB2V[_`\x01a\x1B:\x85\x84aF\xD2V[a\x1BD\x91\x90aF\xD2V[\x90P`@Qk`\x0B8\x03\x80`\x0B_9_\xF3\0\x81R` \x81\x01\x83` \x02\x80`\x01\x83\x8A<` \x87\x02\x82\x01\x91P` \x84\x02` \x83\x01\x83^\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xEC\x81\x01`\x14\x84\x01_\xF0\x95PPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x91Pa\x1B\xF5\x90PW`@Q\x7FVp%\x87\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP\x92\x91PPV[_\x80_` \x84` \x02`\x01\x01_\x88<P_Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x81\x16\x85\x14\x02\x80a\x1ClW`@Q\x7F/e\x9ED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\xFF\xFF`\x18\x82\x90\x1C\x16\x96\x90\x95P\x93PPPPV[`\x03\x81\x81\x01\x91_\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`Da\x1C\x9E\x86\x84aF\xD2V[a\x1C\xA8\x91\x90aG\x9AV[\x90P\x80` \x86\x90\x1B\x17\x92P_\x80[\x82\x81\x10\x15a\x1DEW_a\x1C\xD4` \x87\x90\x1C`D\x84\x02\x01[5``\x1C\x90V[\x90P\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a\x1D;W`@Q\x7F\x80\xF1\x1A\xCF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P`\x01\x01a\x1C\xB6V[P\x82\x94PPPP\x91P\x91V[`\x03\x83\x81\x01\x93_\x91\x82\x91\x82\x91\x82\x91\x805`\xE8\x1C\x01\x01\x81`&a\x1Ds\x8A\x84aF\xD2V[a\x1D}\x91\x90aG\x9AV[\x90P`@Q\x93P\x80`\xC0\x02\x84\x01\x92P\x82`@R\x80\x84` \x1B\x17\x94PP_[\x82\x84\x10\x15a\x1E\xDFW`\x04\x89\x01\x985`\xE0\x81\x90\x1C\x90_\x90a\x1D\xC3\x90a\x1C\xCD\x90\x8C\x90`\xF0\x1Ca#\xD2V[\x90P_a\x1D\xD7a\x1C\xCD\x8Ca\xFF\xFF\x86\x16a#\xD2V[\x90P\x83c\xFF\xFF\xFF\xFF\x16\x83c\xFF\xFF\xFF\xFF\x16\x11\x15\x80a\x1E WP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15[\x15a\x1EWW`@Q\x7F\xF3_\x93\x99\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x90\x86R` \x86\x01R`@\x85 `\x02\x8B\x01\x9A\x91\x92P`(\x1B\x905`\xF0\x1C_\x80a\x1E\x96s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x85\x85a\x1B\xFDV[`@\x8A\x01\x91\x90\x91R``\x89\x01RPPP\x895`\x80\x86\x01\x81\x90Rv\np\xC3\xC4\nd\xE6\xC5\x19\x99\t\x0Be\xF6}\x92@\0\0\0\0\0\0\x04`\xA0\x86\x01RP` \x90\x98\x01\x97`\xC0\x90\x93\x01\x92a\x1D\x9BV[P\x93PPP\x93P\x93\x91PPV[c\xFF\xFF\xFF\xFF\x81\x16_[\x81\x81\x10\x15a\x03\xA8Wa\x1F\x0F` \x84\x90\x1C`D\x83\x02\x01a$3V[`\x01\x01a\x1E\xF5V[`@\x80Qa\x01`\x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Rc\xF3\xCD\x91L\x81R0`\xA0\x82\x01Ra\x01 \x80\x82\x01R`\x03\x84\x81\x01\x94\x805`\xE8\x1C\x01\x01\x90[\x81\x85\x14a\x1F\x98Wa\x1F\x91\x85\x82\x86a%\x04V[\x94Pa\x1F\x7FV[P\x92\x93\x92PPPV[`\x03\x82\x81\x01\x92_\x91\x815`\xE8\x1C\x90\x91\x01\x01\x81a\x1F\xBBa'AV[`@\x80Qa\x01 \x81\x01\x82R_` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x7Fi[.+\x82P\xE7\xBCqJ\xADg\x0BL\xDAU\xF7pC\x7FA9\x9Do\x92\xFF\x0Ed\xA3m\xDB\xFA\x81Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01\0\x82\x01R\x90\x91P[\x82\x86\x14a JWa C\x86\x82\x84\x88a'\x8BV[\x95Pa 0V[P\x93\x94\x93PPPPV[_\x80a ^a'AV[`@\x80Qa\x01\xA0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x82\x90Ra\x01\0\x81\x01\x82\x90Ra\x01 \x81\x01\x82\x90Ra\x01@\x81\x01\x82\x90Ra\x01`\x81\x01\x82\x90Ra\x01\x80\x81\x01\x91\x90\x91R`\x03\x86\x81\x01\x96\x92\x93P\x90\x91\x805`\xE8\x1C\x01\x01[\x80\x86\x14a JWa \xEC\x86\x83\x85\x88a)\x90V[\x95Pa \xD9V[`@\x80Qc\xFF\xFF\xFF\xFF\x83\x16`$\x81\x02\x82\x01\x90\x92R\x80_[\x83\x81\x10\x15a#FW`D\x81\x02` \x86\x90\x1C\x01\x805``\x1C`\x14\x82\x015`\x80\x90\x81\x1C\x90`4\x84\x015\x90\x1C_a!K\x84a!B\x84\x86aGlV[`\x01\x91\x90a+\x80V[\x12\x15a!\x9BW`@Q\x7F\xD4\x9Bp\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R`$\x01a\rAV[\x80\x15a#)Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5\x84\x11\x94\x84`@Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xE0\x84\x90\x1B\x16\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\"AW_\x80\xFD[PZ\xF1\x15\x80\x15a\"SW=_\x80>=_\xFD[Pa\"\x99\x92PPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a\x19\xE0V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x11\xDA`\xB4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a#\x03W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#'\x91\x90aG\0V[P[a#3\x84\x87a+\xC3V[PPP`$\x92\x90\x92\x01\x91P`\x01\x01a!\nV[P`$\x83\x02\x82 _R` _\xA0PPPPV[_h\x01\0\0\0\0\0\0\0\0\x82\x10a\x1AJWa\x1AJa#\xC5V[_a\x18\xB2` s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16;aG\x9AV[_\x81` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x86Z\xFAa#\xBCWcS\\\xF9K_R`\x04`\x1C\xFD[PP_Q\x91\x90PV[c5'\x8D\x12_R`\x04`\x1C\xFD[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a$!W`@Q\x7F\xFF\xC3\x1Eq\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\rAV[` \x83\x90\x1C`D\x83\x02\x01[\x93\x92PPPV[`$\x81\x015`\x80\x1C\x80\x15a\x13QW`@\x80Q\x7F\x0B\r\x9C\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R\x835``\x1C`\x04\x82\x01\x81\x90R0`$\x83\x01R`D\x82\x01\x84\x90R\x91Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91c\x0B\r\x9C\t\x91`d\x80\x83\x01\x92_\x92\x91\x90\x82\x90\x03\x01\x81\x83\x87\x80;\x15\x80\x15a$\xDFW_\x80\xFD[PZ\xF1\x15\x80\x15a$\xF1W=_\x80>=_\xFD[Pa\x03\xA8\x92P`\x01\x91P\x83\x90P\x84a+\xCCV[`\x01\x83\x81\x01\x93_\x91\x905\x82\x1A\x90a% \x90\x85\x90\x83\x16\x15\x15a,\x05V[`\x02\x85\x01\x945`\xF0\x1Ca%Ga%6\x85\x83a,VV[\x80Q` \x82\x01Q`@\x90\x92\x01Q\x90\x92V[`\x02\x0B`\x80\x88\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`@\x88\x01R\x16` \x86\x01\x90\x81R`\xA0\x90 _`\x10\x88\x01\x885`\x80\x1C\x90\x98Po\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P_\x81\x15a&\xAEW_a%\xE4a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x18\x81V[\x90Pa%\xEF\x83a,\xB6V[`\xE0\x8A\x01Ra&\x1E\x89\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a-\x11V[a&aa\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x86a\x18\x81V[`\x80\x8A\x01Q_\x86\x81R`\x06` R`@\x90 \x91\x93Pa&\xA8\x91\x90\x86\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x85\x90\x87\x90a-/V[Pa&\xF4V[a&\xF1a\x05\rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x85a\x18\x81V[\x90P[_a'\x1B`\x02\x87\x16\x15\x15_\x86\x81R`\x06` R`@\x90 `\x80\x8C\x01Q\x8D\x91\x90\x88\x90\x87a-\xB8V[` \x8B\x01Q\x91\x9BP\x91Pa'2\x90`\x01\x90\x83a+\x80V[P\x98\x99\x98PPPPPPPPPV[_a'\x86a'Ma05V[`@\x80Q`B\x81\x01\x90\x91R\x7F\x19\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x02\x81\x01\x91\x90\x91R\x90V[\x90P\x90V[\x835_\x90\x81\x1A`\x01\x81\x81\x16\x15\x15`\x80\x87\x81\x01\x91\x90\x91R\x90\x87\x015\x81\x1C` \x87\x01R`\x11\x87\x015\x81\x1C`@\x87\x01R`!\x87\x015\x81\x1C``\x87\x01\x81\x90R`A\x88\x01\x97`1\x015\x90\x91\x1C\x90\x81\x11\x15a(\x0CW`@Q\x7F+\xAElR\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16\x15a(AW\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86` \x01\x81\x81Qa(9\x91\x90aGlV[\x90RPa(iV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86`@\x01\x81\x81Qa(e\x91\x90aF\xD2V[\x90RP[P`\x02\x86\x81\x01\x965`\xF0\x1C\x90a(\x99\x90\x83\x16\x15\x15a(\x87\x86\x84a,VV[\x90`\x05\x1B` \x81\x18\x82\x01Q\x91\x01Q\x90\x91V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16`\xC0\x89\x01R\x16`\xA0\x87\x01RP`\x04\x81\x16a(\xCCW\x85_a(\xD6V[`\x14\x86\x01\x865``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x87\x01R\x95P_a)\x0Fa)\x03\x87a\x01 \x90 \x90V[`\"\x87\x01R`B\x86 \x90V[\x90Pa)\x1A\x81a1-V[_`\x08\x83\x16a)2Wa)-\x88\x83a1~V[a)<V[a)<\x88\x83a1\xE8V[`\xE0\x89\x01Q`\xA0\x8A\x01Q` \x8B\x01Q\x93\x9BP\x91\x93P\x80\x15\x84\x02\x17\x91a)h\x91\x84\x91`\x01\x88\x16\x15\x15a2,V[`\xC0\x88\x01Q`@\x89\x01Qa)\x83\x91\x83\x91`\x01\x88\x16\x15\x15a2\xB6V[P\x96\x97\x96PPPPPPPV[_\x80a)\x9C\x85\x87a3.V[`\x02\x82\x01\x97P\x91P_\x90\x81\x905`\xF0\x1Ca)\xC5`\x08\x85\x16\x15\x15a)\xBF\x88\x84a,VV[\x90a4\x0EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x16a\x01\0\x8C\x01R\x91\x16`\xE0\x8A\x01R\x92PPP` \x87\x01\x875`\xA0\x88\x01\x81\x90R\x90\x97P\x81\x10\x15a*7W`@Q\x7F\x8E\x1E\xDF\xA4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02\x82\x16a*FW\x86_a*PV[`\x14\x87\x01\x875``\x1C[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01 \x88\x01R\x96P_a*}\x88`\x04\x85\x16\x15a4PV[a\x01@\x8A\x01R\x90\x98P\x90Pa*\x93\x87\x89\x85a4\xFBV[\x97P_\x80a*\xA3\x89\x8B\x87\x87a5CV[\x91\x9BP\x92P\x90P_a*\xC4a*\xB8\x8B\x88a7JV[`\"\x8B\x01R`B\x8A \x90V[\x90P_`\x80\x87\x16a*\xDEWa*\xD9\x8C\x83a1~V[a*\xE8V[a*\xE8\x8C\x83a1\xE8V[\x90\x9CP\x90P`\x10\x87\x16\x15a+\x1FWa+\x0B\x8Ba\x01\x80\x01Qd\xFF\xFF\xFF\xFF\xFF\x16a7jV[a+\x1A\x81\x8Ca\x01`\x01Qa\x17JV[a+(V[a+(\x82a1-V[a\x01 \x8B\x01Qa\x01\0\x8C\x01Q\x81\x15\x83\x02\x90\x91\x17\x90a+N\x90\x82\x90\x86`\x01\x8C\x16\x15\x15a2\xB6V[a+X\x86\x83a7\xA4V[`\xE0\x8C\x01Qa+o\x90\x83\x90\x87`\x01\x8C\x16\x15\x15a2,V[P\x9A\x9B\x9APPPPPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x81 a+\xBBa+\xB2\x82\\\x85a7\xECV[\x92P\x81\x83a8\x04V[P\x93\x92PPPV[`$\x82\x827PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16_\x90\x81R` \x84\x90R`@\x90 a\x1A\xBDa+\xFE\x82\\\x84a8\x0BV[\x82\x90a8\x04V[\x80\x15\x15`\xC0\x83\x01R\x80a,,Ws\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D%a,3V[d\x01\0\x02v\xA4[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x90\x92\x01\x91\x90\x91RPV[_\x81c\xFF\xFF\xFF\xFF\x84\x16\x11a,\xA5W`@Q\x7F\xF6`\x1BP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x81\x01\x83\x90Rc\xFF\xFF\xFF\xFF\x84\x16`$\x82\x01R`D\x01a\rAV[P`\xC0\x81\x02` \x83\x90\x1C\x01\x92\x91PPV[_\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x11\x15a\x18/W`@Q\x7F5'\x8D\x12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x01D`\x1C\x85\x01_\x85Z\xF1\x80a\x03\xA8W`@Q=_\x82>P=_\xF3[\x82`\x02\x0B\x82`\x02\x0B\x13\x15a-sW\x82`\x02\x0Ba-W\x82\x84`\x02\x0Ba8#\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x02\x0B\x13\x15a-nWa-n\x86\x85\x87\x86\x86\x86a84V[a-\xB0V[\x82`\x02\x0B\x82`\x02\x0B\x12\x15a-\xB0W_`\x02\x84\x90\x0B\x82\x81\x07\x91\x90\x91\x12\x90\x82\x90\x05\x03\x81\x02`\x02\x0B\x82`\x02\x0B\x12\x15a-\xB0Wa-\xB0\x86\x85\x87\x86\x86\x86a8\xC7V[PPPPPPV[_\x80\x87\x15a.ZW`\x10\x87\x01\x965`\x80\x1Ca.$\x81a.\r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89a9fV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a9\xA0V[\x87c\x01\0\0\0\x01_\x82\x82Ta.9\x91\x90aGlV[\x90\x91UP\x88\x93PPo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90Pa0*V[_\x80\x80`\x03\x8A\x01\x8A5`\xE8\x1D\x90\x9AP\x90P_`\x10\x8B\x01\x8B5`\x80\x1C\x90\x9BP\x90P_\x80`\x03\x80\x8E\x01\x90\x8E5`\xE8\x1C\x8F\x01\x01`@\x80Q``\x81\x01\x82R\x8E\x81R`\x02\x8E\x81\x0B` \x83\x01R\x8D\x81\x0B\x92\x82\x01\x83\x90R\x93\x95P\x91\x93P\x8E\x92_\x92\x91\x90\x88\x90\x0B\x13\x15a.\xD1Wa.\xCC\x83\x88\x87\x89\x85a9\xBBV[a.\xDEV[a.\xDE\x83\x88\x87\x89\x85a;\x04V[\x90\x9BP\x99P`\x10\x82\x01\x96P\x92P5`\x80\x1Ca/\x0B\x81o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16a9\xA0V[a/\x15\x90\x8BaGlV[\x99Pa/3o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x84aGlV[\x92Pa/?\x86\x86a<NV[\x81Q_\x90a/\x84\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90a9fV[\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8Ao\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a/\xFFW`@Q\x7Fd)\xCF\xD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8C\x16`\x04\x83\x01R\x82\x16`$\x82\x01R`D\x01a\rAV[\x8A\x85c\x01\0\0\0\x01_\x82\x82Ta0\x15\x91\x90aGlV[\x90\x91UP\x96\x9CP\x92\x9APPPPPPPPPPP[\x96P\x96\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x000\x14\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x16a1*WP`@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x81\x01\x91\x90\x91RF``\x82\x01R0`\x80\x82\x01R`\xA0\x90 \x90V[\x90V[_\x81\x81R` \x81\x90R`@\x90 \x80\\\x15a1sW`@Q\x7F\x8A.\xF1\x16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x13Q\x81`\x01a8\x04V[`\x17`\x14\x83\x015`\xE8\x1C\x80\x84\x01\x82\x01\x93_\x92\x815``\x1C\x92\x91\x01\x90a1\xA5\x83\x86\x84\x84a<\x87V[a1\xDBW`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x93PPP[\x92P\x92\x90PV[_\x80`@Q\x83\x81R_` \x82\x01R`A\x85`?\x83\x017`A\x85\x01\x94P` `\x01`\x80\x83`\x01Z\xFAQ\x91PP=a2%Wc\x8B\xAAW\x9F_R`\x04`\x1C\xFD[\x92\x93\x91PPV[\x81a29`\x01\x85\x83a+\xCCV[\x81\x15a2\x8DWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a2\x82\x90\x84\x90aF\xD2V[\x90\x91UPa2\xAF\x90PV[a2\xAFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x860\x84a\x1ANV[PPPPPV[\x81a2\xC3`\x01\x85\x83a+\x80V[P\x81\x15a3\rWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16_\x90\x81R`\x02` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 \x80T\x83\x92\x90a2\x82\x90\x84\x90aGlV[a2\xAFs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86\x83a\x19\xE0V[`\x01\x81\x01\x90_\x905\x81\x1A`\x04\x83`<\x86\x017`\x04\x92\x90\x92\x01\x91` \x81\x16\x15a3\xA7W`\x10\x81\x16a3~W\x7Fn\xE8\x9D\xEEW7\x05\xC0$\xA0\x86\xE1\x9A\x12\x8E\xE0\xA5\xEE\x05G\xE3(:\xDF\xA7/\xBE3jLKla3\xA0V[\x7Fk\xE5\xF2+\xDC\xD07\xF6\xF3RP\xC3.G\x8F\xADb\x19Z\xC2\xBB\xAB\x1E)2\xF8\xC9z\xF9&\xB4\x91[\x84Ra3\xFAV[`\x10\x81\x16a3\xD5W\x7F\x02.\x17\x0C\xDF3\x8FE\xBCq\x8FX\xD2\x9B\xFA\xFB\xF3\x95l/\x9E\xA8\xD1\x9C\xCC\x9Br\xE4-\xBB\xB7\xB0a3\xF7V[\x7F\xB0a{\x84\xF6\x94\xC2E\xE5O\xB8\x03.\xBD\xC9\xF5n\xB2n\xA2\xC1\xB6ZF\xC5\x8FP\xDB\xD5\x16\xE2\x86[\x84R[`\x01\x81\x16\x15\x15`\xC0\x94\x90\x94\x01\x93\x90\x93RP\x91V[`\x05\x81\x90\x1B` \x81\x18\x83\x01Q\x90\x83\x01\x80Q`\x80\x90\x91\x01Q``\x85\x01Qb\x0FB@\x90\x81\x03\x90a4<\x82\x84aG\xD2V[a4F\x91\x90aG\x9AV[\x91PP\x92P\x92P\x92V[_\x80\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x83a4\xF1W\x845`\xE8\x1C`\x03\x86\x01\x95P`@Q`\x14`d\x03\x81\x01\x82\x81\x01`@R\x82\x88\x827\x82\x81 \x93PP\x81\x87\x01\x96P`D\x81\x01Q\x7Ft\x07\x90\\\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R`@`$\x83\x01R`\x14\x83\x03\x92P\x82`D\x83\x01R`d\x83\x01\x81` \x1B\x17\x82`\xC0\x1B\x17\x94PPPP[\x84\x92P\x92P\x92P\x92V[_`\x10\x82\x16\x15a5)W`\x08\x83a\x01x\x86\x017`\x08\x92\x90\x92\x01\x91`\x05\x83a\x01\x9B\x86\x017`\x05\x83\x01\x92Pa5;V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFFC\x16a\x01`\x85\x01R[P\x90\x92\x91PPV[_\x80\x80\x80` \x86\x16\x15a5\xF5WP\x855`\x80\x90\x81\x1C`@\x89\x01\x81\x90R`\x10\x88\x015\x82\x1C``\x8A\x01\x81\x90R`0\x89\x01\x98` \x015\x90\x92\x1C\x91\x81\x83\x10\x15a5\xB4W`@Q\x7F\xC4\xDA\xF0\x03\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x83\x11\x15a5\xEEW`@Q\x7FD\x18#1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPa6 V[P`\x10\x86\x01\x955`\x80\x1C`@\x86\x16a6\rW_a6\x10V[`\x01[`\xFF\x16`@\x89\x01R``\x88\x01\x81\x90R[` \x87\x01\x96`\x10\x81\x015`\x80\x90\x81\x1C\x915\x90\x1C\x80\x82\x11\x15a6mW`@Q\x7Ff\x8F\xEF\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x8A\x01R`\x08\x87\x16\x15a6\xF6W``\x87\x16\x15a6\xC5Wa6\xB2o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83aF\xD2V[\x93Pa6\xBE\x86\x85a<\xCCV[\x92Pa7<V[\x90\x91P\x81\x90a6\xEFa6\xD7\x87\x84a<\xD7V[\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a<\xE2V[\x93Pa7<V[``\x87\x16\x15a7\x11W\x90\x92P\x82\x90a6\xBEa6\xD7\x87\x84a<\xCCV[a7-o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x83aF\xD2V[\x92Pa79\x86\x84a<\xD7V[\x93P[P\x95\x97\x91\x96P\x94P\x92PPPV[_\x80`\x10\x83\x16a7\\Wa\x01\x80a7`V[a\x01\xA0[\x90\x93 \x93\x92PPPV[\x80B\x11\x15a\x047W`@Q\x7F =\x82\xD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x15a\x13QWc\xFF\xFF\xFF\xFF\x82\x16\x82`\xC0\x1C\x82`\x04\x82\x01R\x83` \x1C` _\x84\x84_\x85Z\xF1\x92PPPc$\xA2\xE4K_Q\x14`\x1F=\x11\x16\x81\x16a\x03\xA8Wc\xF9Y\xFD\xAE_R`\x04`\x1C\xFD[\x80\x82\x03\x82\x81\x13\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[\x80\x82]PPV[\x81\x81\x01\x82\x81\x12\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x81\x83\x07\x12\x91\x81\x90\x05\x91\x90\x91\x03\x02\x90V[_a8Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a<\xEDV[\x94P\x90P`\x02\x84\x81\x0B\x90\x84\x90\x0B\x12\x15a8pWPa-\xB0V[\x80\x15a8\xC1W\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a8\x90Wa8\x90aG?V[\x01T\x87c\x01\0\0\0\x01Ta8\xA4\x91\x90aF\xD2V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a8\xBEWa8\xBEaG?V[\x01U[Pa84V[_a8\xEAs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x86\x86\x85a=_V[\x94P\x90P`\x02\x83\x81\x0B\x90\x85\x90\x0B\x13a9\x02WPa-\xB0V[\x80\x15a9SW\x86b\xFF\xFF\xFF\x85\x16c\x01\0\0\0\x81\x10a9\"Wa9\"aG?V[\x01T\x87c\x01\0\0\0\x01Ta96\x91\x90aF\xD2V[\x87b\xFF\xFF\xFF\x86\x16c\x01\0\0\0\x81\x10a9PWa9PaG?V[\x01U[\x83a9]\x81aG\xE9V[\x94PPPa8\xC7V[_\x81\x81R`\x06` R`@\x81 _a9\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16`\x03\x84\x01a#\x95V[\x95\x94PPPPPV[_a$,\x82a9\xB7g\r\xE0\xB6\xB3\xA7d\0\0\x86aG\xD2V[\x04\x90V[_\x80\x80\x80`\x01\x81\x80[\x82\x15a:\x8EW`\x10\x8A\x01\x995`\x80\x1Ca9\xDD\x81\x84aGlV[\x92Pa9\xFB\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a9\xA0V[a:\x05\x90\x83aGlV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a:\"Wa:\"aG?V[\x01_\x82\x82Ta:1\x91\x90aGlV[\x90\x91UPP\x88Q_\x90a:|\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xA4V[\x91PPa:\x89\x8B\x82a>\tV[\x9APPP[\x87Q` \x89\x01Qa:\xD8\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a>#V[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13a9\xC4W\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[_\x80\x80\x80`\x01\x81\x80[\x82\x15a;\xD7W`\x10\x8A\x01\x995`\x80\x1Ca;&\x81\x84aGlV[\x92Pa;D\x81\x8Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a9\xA0V[a;N\x90\x83aGlV[\x91P\x81\x8D\x8Db\xFF\xFF\xFF\x16c\x01\0\0\0\x81\x10a;kWa;kaG?V[\x01_\x82\x82Ta;z\x91\x90aGlV[\x90\x91UPP\x88Q_\x90a;\xC5\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90\x8Fa=\xA4V[\x91PPa;\xD2\x8B\x82a>=V[\x9APPP[\x87Q` \x89\x01Qa<!\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x8E\x90a<\xEDV[\x80\x9CP\x81\x94PPP\x87`@\x01Q`\x02\x0B\x8B`\x02\x0B\x13\x15a;\rW\x98\x9B\x90\x9AP\x97\x98P\x95\x96\x95PPPPPPV[\x80\x82\x14a\x13QW`@Q\x7F\x01\x84/\x8C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Qc\x16&\xBA~`\xE0\x1B\x80\x82R\x85`\x04\x83\x01R`$\x82\x01`@\x81R\x84`D\x84\x01R\x84\x86`d\x85\x017` \x81`d\x87\x01\x85\x8BZ\xFA\x90Q\x90\x91\x14\x16\x96\x95PPPPPPV[_a$,\x82\x84a>WV[_a$,\x82\x84a>yV[_a$,\x82\x84aF\xD2V[_\x80\x80\x80a=\x12a=\x07\x86\x88\x07\x83\x13\x87\x89\x05\x03`\x01aHEV[`\x02\x81\x90\x0B`\x08\x1D\x91V[\x90\x92P\x90Pa=B\x81a=<s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\x91V[\x90a>\xCCV[\x90\x94P\x90Pa=R\x82\x82\x87a?\x8EV[\x92PPP\x94P\x94\x92PPPV[_\x80\x80\x80a=t\x85\x87\x07\x82\x13\x86\x88\x05\x03a=\x07V[\x90\x92P\x90Pa=B\x81a=\x9Es\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x8A\x86a>\x91V[\x90a?\xB8V[_\x80`\x06` R\x83_R`\x04`@_ \x01` R\x82_R`@_ ` Rc\x1E.\xAE\xAF_R` _`$`\x1C\x88Z\xFAa=\xE4WcS\\\xF9K_R`\x04`\x1C\xFD[PP_Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x94`\x80\x91\x90\x91\x1D\x93P\x91PPV[\x80\x82\x03`\x80\x81\x90\x1C\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_\x80\x80\x80a=ta=\x07`\x01\x87\x89\x07\x84\x13\x88\x8A\x05\x03aH\x86V[\x81\x81\x01`\x80\x81\x90\x1C\x15a\x18\xB2Wc\xC9eN\xD4_R`\x04`\x1C\xFD[_k\x03;.<\x9F\xD0\x80<\xE8\0\0\0a>o\x83\x85aG\xD2V[a$,\x91\x90aG\x9AV[_\x81a>ok\x03;.<\x9F\xD0\x80<\xE8\0\0\0\x85aG\xD2V[_\x82\x81R`\x06` \x90\x81R`@\x80\x83 \x84\x84R`\x05\x01\x90\x91R\x81 a9\x97s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x82a#\x95V[_\x80_a?g\x84`\xFF\x16\x86\x90\x1C~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05\x81\x19`\x01\x01\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x92P\x82a?}W`\xFFa?\x84V[\x83`\xFF\x16\x81\x01[\x91PP\x92P\x92\x90PV[_\x81`\xFF\x84\x16a?\xA4`\x01\x87\x90\x0Ba\x01\0aH\xC7V[a?\xAE\x91\x90aHEV[a\x19R\x91\x90aH\xC7V[_\x80_\x83`\xFF\x03\x90P_a@Y\x82`\xFF\x16\x87\x90\x1B\x7F\x07\x06\x06\x05\x06\x02\x05\x04\x06\x02\x03\x02\x05\x04\x03\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBE\x83\x15`\x08\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x11`\x07\x1B\x17\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[\x90P\x80a\x01\0\x14\x15\x93P\x83a@nW_a@uV[\x81`\xFF\x16\x81\x03[\x92PPP\x92P\x92\x90PV[_\x80\x83`\x1F\x84\x01\x12a@\x90W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xA7W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a1\xE1W_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a@\xCFW_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a@\xE5W_\x80\xFD[a@\xF1\x85\x82\x86\x01a@\x80V[\x90\x96\x90\x95P\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x047W_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15aA1W_\x80\xFD[\x845aA<\x81a@\xFDV[\x93P` \x85\x015aAL\x81a@\xFDV[\x92P`@\x85\x015a\xFF\xFF\x81\x16\x81\x14aAbW_\x80\xFD[\x91P``\x85\x015b\xFF\xFF\xFF\x81\x16\x81\x14aAyW_\x80\xFD[\x93\x96\x92\x95P\x90\x93PPV[_` \x82\x84\x03\x12\x15aA\x94W_\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a$,W_\x80\xFD[_` \x82\x84\x03\x12\x15aA\xBBW_\x80\xFD[P5\x91\x90PV[_\x80_\x80_\x85\x87\x03a\x01`\x81\x12\x15aA\xD8W_\x80\xFD[\x865aA\xE3\x81a@\xFDV[\x95P`\xA0\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x82\x01\x12\x15aB\x14W_\x80\xFD[` \x87\x01\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF@\x82\x01\x12\x15aBIW_\x80\xFD[P`\xC0\x86\x01\x92Pa\x01@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aBjW_\x80\xFD[aBv\x88\x82\x89\x01a@\x80V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15aB\x98W_\x80\xFD[\x825aB\xA3\x81a@\xFDV[\x94` \x93\x90\x93\x015\x93PPPV[_\x80_`@\x84\x86\x03\x12\x15aB\xC3W_\x80\xFD[\x835aB\xCE\x81a@\xFDV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aB\xE9W_\x80\xFD[aB\xF5\x86\x82\x87\x01a@\x80V[\x94\x97\x90\x96P\x93\x94PPPPV[_\x80_``\x84\x86\x03\x12\x15aC\x14W_\x80\xFD[\x835aC\x1F\x81a@\xFDV[\x92P` \x84\x015aC/\x81a@\xFDV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x88\x16\x81R`\xE0` \x82\x01R_aC\xC6`\xE0\x83\x01\x89aC@V[\x82\x81\x03`@\x84\x01RaC\xD8\x81\x89aC@V[``\x84\x01\x88\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16`\x80\x85\x01R`\xA0\x84\x01\x86\x90R\x83\x81\x03`\xC0\x85\x01R\x84Q\x80\x82R` \x80\x87\x01\x93P\x90\x91\x01\x90_[\x81\x81\x10\x15aD:W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aD\x1CV[P\x90\x9B\x9APPPPPPPPPPPV[_\x80_\x80`\x80\x85\x87\x03\x12\x15aD^W_\x80\xFD[\x845aDi\x81a@\xFDV[\x93P` \x85\x015aDy\x81a@\xFDV[\x92P`@\x85\x015\x91P``\x85\x015aAy\x81a@\xFDV[` \x81R_a$,` \x83\x01\x84aC@V[_\x80` \x83\x85\x03\x12\x15aD\xB3W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xC9W_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13aD\xD9W_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\xEFW_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15aE\x03W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[` \x81R\x81` \x82\x01R\x81\x83`@\x83\x017_\x81\x83\x01`@\x90\x81\x01\x91\x90\x91R`\x1F\x90\x92\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x01\x01\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aE\x9CW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xB2W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aE\xC2W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aE\xDCWaE\xDCaE_V[`@Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`?\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x85\x01\x16\x01\x16\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aFHWaFHaE_V[`@R\x81\x81R\x82\x82\x01` \x01\x86\x10\x15aF_W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[\x80`\x02\x0B\x81\x14a\x047W_\x80\xFD[_` \x82\x84\x03\x12\x15aF\x9AW_\x80\xFD[\x815a$,\x81aF|V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18\xB2Wa\x18\xB2aF\xA5V[_` \x82\x84\x03\x12\x15aF\xF5W_\x80\xFD[\x815a$,\x81a@\xFDV[_` \x82\x84\x03\x12\x15aG\x10W_\x80\xFD[PQ\x91\x90PV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x18\xB2Wa\x18\xB2aF\xA5V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x18\xB2Wa\x18\xB2aF\xA5V[_` \x82\x84\x03\x12\x15aG\x8FW_\x80\xFD[\x81Qa$,\x81aF|V[_\x82aG\xCDW\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18\xB2Wa\x18\xB2aF\xA5V[_\x81`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x03aH\x1DWaH\x1DaF\xA5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x01\x92\x91PPV[`\x02\x81\x81\x0B\x90\x83\x90\x0B\x01b\x7F\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x82\x12\x17\x15a\x18\xB2Wa\x18\xB2aF\xA5V[`\x02\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\x81\x12b\x7F\xFF\xFF\x82\x13\x17\x15a\x18\xB2Wa\x18\xB2aF\xA5V[_\x82`\x02\x0B\x82`\x02\x0B\x02\x80`\x02\x0B\x91P\x80\x82\x14aH\xE6WaH\xE6aF\xA5V[P\x92\x91PPV\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Custom error with signature `GasAboveMax()` and selector `0x668fef1b`.
```solidity
error GasAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct GasAboveMax {}
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
        impl ::core::convert::From<GasAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: GasAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for GasAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for GasAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GasAboveMax()";
            const SELECTOR: [u8; 4] = [102u8, 143u8, 239u8, 27u8];
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
    /**Custom error with signature `InvalidPermitType(uint8)` and selector `0x6f1d1509`.
```solidity
error InvalidPermitType(uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidPermitType {
        pub _0: u8,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u8,);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidPermitType> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPermitType) -> Self {
                (value._0,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPermitType {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { _0: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPermitType {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPermitType(uint8)";
            const SELECTOR: [u8; 4] = [111u8, 29u8, 21u8, 9u8];
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                )
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
    /**Custom error with signature `ToBGasUsedAboveMax()` and selector `0x2bae6c52`.
```solidity
error ToBGasUsedAboveMax();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ToBGasUsedAboveMax {}
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
        impl ::core::convert::From<ToBGasUsedAboveMax> for UnderlyingRustTuple<'_> {
            fn from(value: ToBGasUsedAboveMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ToBGasUsedAboveMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ToBGasUsedAboveMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ToBGasUsedAboveMax()";
            const SELECTOR: [u8; 4] = [43u8, 174u8, 108u8, 82u8];
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
constructor(address uniV4, address controller, address feeMaster);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub uniV4: alloy::sol_types::private::Address,
        pub controller: alloy::sol_types::private::Address,
        pub feeMaster: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.uniV4, value.controller, value.feeMaster)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        uniV4: tuple.0,
                        controller: tuple.1,
                        feeMaster: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.feeMaster,
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
    /**Function with signature `compose(address,bytes)` and selector `0x7407905c`.
```solidity
function compose(address from, bytes memory payload) external returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct composeCall {
        pub from: alloy::sol_types::private::Address,
        pub payload: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`compose(address,bytes)`](composeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct composeReturn {
        pub _0: u32,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<composeCall> for UnderlyingRustTuple<'_> {
                fn from(value: composeCall) -> Self {
                    (value.from, value.payload)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for composeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        payload: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<composeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: composeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for composeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for composeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = composeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "compose(address,bytes)";
            const SELECTOR: [u8; 4] = [116u8, 7u8, 144u8, 92u8];
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
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.payload,
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
    ///Container for all the [`Angstrom`](self) function calls.
    pub enum AngstromCalls {
        beforeAddLiquidity(beforeAddLiquidityCall),
        beforeRemoveLiquidity(beforeRemoveLiquidityCall),
        compose(composeCall),
        configurePool(configurePoolCall),
        deposit_0(deposit_0Call),
        deposit_1(deposit_1Call),
        eip712Domain(eip712DomainCall),
        execute(executeCall),
        extsload(extsloadCall),
        initializePool(initializePoolCall),
        invalidateNonce(invalidateNonceCall),
        pullFee(pullFeeCall),
        removePool(removePoolCall),
        toggleNodes(toggleNodesCall),
        unlockCallback(unlockCallbackCall),
        withdraw_0(withdraw_0Call),
        withdraw_1(withdraw_1Call),
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
            [71u8, 231u8, 239u8, 36u8],
            [116u8, 7u8, 144u8, 92u8],
            [125u8, 31u8, 50u8, 38u8],
            [131u8, 64u8, 245u8, 73u8],
            [132u8, 176u8, 25u8, 110u8],
            [133u8, 135u8, 244u8, 80u8],
            [145u8, 221u8, 115u8, 70u8],
            [214u8, 207u8, 253u8, 30u8],
            [217u8, 202u8, 237u8, 18u8],
            [217u8, 225u8, 127u8, 152u8],
            [243u8, 254u8, 243u8, 163u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AngstromCalls {
        const NAME: &'static str = "AngstromCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::beforeAddLiquidity(_) => {
                    <beforeAddLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beforeRemoveLiquidity(_) => {
                    <beforeRemoveLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::compose(_) => <composeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::configurePool(_) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_0(_) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deposit_1(_) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eip712Domain(_) => {
                    <eip712DomainCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::execute(_) => <executeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extsload(_) => <extsloadCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::initializePool(_) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::invalidateNonce(_) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pullFee(_) => <pullFeeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::removePool(_) => {
                    <removePoolCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::toggleNodes(_) => {
                    <toggleNodesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlockCallback(_) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn deposit_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <deposit_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::deposit_0)
                    }
                    deposit_0
                },
                {
                    fn compose(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <composeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::compose)
                    }
                    compose
                },
                {
                    fn removePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <removePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::removePool)
                    }
                    removePool
                },
                {
                    fn deposit_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <deposit_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::deposit_1)
                    }
                    deposit_1
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
                    fn initializePool(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <initializePoolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::initializePool)
                    }
                    initializePool
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
                {
                    fn withdraw_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <withdraw_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::withdraw_0)
                    }
                    withdraw_0
                },
                {
                    fn pullFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <pullFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::pullFee)
                    }
                    pullFee
                },
                {
                    fn withdraw_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromCalls> {
                        <withdraw_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromCalls::withdraw_1)
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
                Self::compose(inner) => {
                    <composeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::configurePool(inner) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deposit_0(inner) => {
                    <deposit_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deposit_1(inner) => {
                    <deposit_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::invalidateNonce(inner) => {
                    <invalidateNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::unlockCallback(inner) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::compose(inner) => {
                    <composeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::configurePool(inner) => {
                    <configurePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::initializePool(inner) => {
                    <initializePoolCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::unlockCallback(inner) => {
                    <unlockCallbackCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`Angstrom`](self) custom errors.
    pub enum AngstromErrors {
        AssetAccessOutOfBounds(AssetAccessOutOfBounds),
        AssetsOutOfOrderOrNotUnique(AssetsOutOfOrderOrNotUnique),
        BundleChangeNetNegative(BundleChangeNetNegative),
        DuplicateAsset(DuplicateAsset),
        Expired(Expired),
        FailedToDeployNewStore(FailedToDeployNewStore),
        FeeAboveMax(FeeAboveMax),
        FillingTooLittle(FillingTooLittle),
        FillingTooMuch(FillingTooMuch),
        GasAboveMax(GasAboveMax),
        IndexMayHaveChanged(IndexMayHaveChanged),
        InvalidPermitType(InvalidPermitType),
        InvalidPoolKey(InvalidPoolKey),
        InvalidSignature(InvalidSignature),
        InvalidStoreIndex(InvalidStoreIndex),
        InvalidTickSpacing(InvalidTickSpacing),
        LimitViolated(LimitViolated),
        MissingHookPermissions(MissingHookPermissions),
        NoEntry(NoEntry),
        NonceReuse(NonceReuse),
        NotController(NotController),
        NotFeeMaster(NotFeeMaster),
        NotFromHook(NotFromHook),
        NotNode(NotNode),
        NotUniswap(NotUniswap),
        OnlyOncePerBlock(OnlyOncePerBlock),
        OrderAlreadyExecuted(OrderAlreadyExecuted),
        OutOfOrderOrDuplicatePairs(OutOfOrderOrDuplicatePairs),
        Overflow(Overflow),
        PairAccessOutOfBounds(PairAccessOutOfBounds),
        ReaderNotAtEnd(ReaderNotAtEnd),
        ToBGasUsedAboveMax(ToBGasUsedAboveMax),
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
            [40u8, 51u8, 101u8, 94u8],
            [43u8, 174u8, 108u8, 82u8],
            [47u8, 101u8, 158u8, 68u8],
            [53u8, 39u8, 141u8, 18u8],
            [68u8, 24u8, 35u8, 49u8],
            [86u8, 112u8, 37u8, 135u8],
            [88u8, 125u8, 170u8, 48u8],
            [92u8, 210u8, 107u8, 104u8],
            [100u8, 41u8, 207u8, 210u8],
            [102u8, 143u8, 239u8, 27u8],
            [111u8, 29u8, 21u8, 9u8],
            [118u8, 163u8, 249u8, 93u8],
            [122u8, 215u8, 28u8, 235u8],
            [128u8, 241u8, 26u8, 207u8],
            [138u8, 46u8, 241u8, 22u8],
            [139u8, 170u8, 87u8, 159u8],
            [140u8, 184u8, 136u8, 114u8],
            [142u8, 30u8, 223u8, 164u8],
            [194u8, 86u8, 98u8, 43u8],
            [196u8, 218u8, 240u8, 3u8],
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
    impl alloy_sol_types::SolInterface for AngstromErrors {
        const NAME: &'static str = "AngstromErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 34usize;
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
                Self::GasAboveMax(_) => {
                    <GasAboveMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::IndexMayHaveChanged(_) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermitType(_) => {
                    <InvalidPermitType as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPoolKey(_) => {
                    <InvalidPoolKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidStoreIndex(_) => {
                    <InvalidStoreIndex as alloy_sol_types::SolError>::SELECTOR
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
                Self::ToBGasUsedAboveMax(_) => {
                    <ToBGasUsedAboveMax as alloy_sol_types::SolError>::SELECTOR
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
                    fn NotFeeMaster(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotFeeMaster as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotFeeMaster)
                    }
                    NotFeeMaster
                },
                {
                    fn ToBGasUsedAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <ToBGasUsedAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::ToBGasUsedAboveMax)
                    }
                    ToBGasUsedAboveMax
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
                    fn DuplicateAsset(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <DuplicateAsset as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::DuplicateAsset)
                    }
                    DuplicateAsset
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
                    fn GasAboveMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <GasAboveMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::GasAboveMax)
                    }
                    GasAboveMax
                },
                {
                    fn InvalidPermitType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidPermitType as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidPermitType)
                    }
                    InvalidPermitType
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
                    fn NotFromHook(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <NotFromHook as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::NotFromHook)
                    }
                    NotFromHook
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
                    fn InvalidStoreIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <InvalidStoreIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::InvalidStoreIndex)
                    }
                    InvalidStoreIndex
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
                    fn IndexMayHaveChanged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AngstromErrors> {
                        <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AngstromErrors::IndexMayHaveChanged)
                    }
                    IndexMayHaveChanged
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
                Self::GasAboveMax(inner) => {
                    <GasAboveMax as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::IndexMayHaveChanged(inner) => {
                    <IndexMayHaveChanged as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPermitType(inner) => {
                    <InvalidPermitType as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::ToBGasUsedAboveMax(inner) => {
                    <ToBGasUsedAboveMax as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::GasAboveMax(inner) => {
                    <GasAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidPermitType(inner) => {
                    <InvalidPermitType as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::ToBGasUsedAboveMax(inner) => {
                    <ToBGasUsedAboveMax as alloy_sol_types::SolError>::abi_encode_raw(
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
        uniV4: alloy::sol_types::private::Address,
        controller: alloy::sol_types::private::Address,
        feeMaster: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AngstromInstance<T, P, N>>,
    > {
        AngstromInstance::<T, P, N>::deploy(provider, uniV4, controller, feeMaster)
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
        feeMaster: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AngstromInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, uniV4, controller, feeMaster)
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
            uniV4: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
            feeMaster: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<AngstromInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                uniV4,
                controller,
                feeMaster,
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
            uniV4: alloy::sol_types::private::Address,
            controller: alloy::sol_types::private::Address,
            feeMaster: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            uniV4,
                            controller,
                            feeMaster,
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
        ///Creates a new call builder for the [`compose`] function.
        pub fn compose(
            &self,
            from: alloy::sol_types::private::Address,
            payload: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, composeCall, N> {
            self.call_builder(&composeCall { from, payload })
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
        ///Creates a new call builder for the [`invalidateNonce`] function.
        pub fn invalidateNonce(
            &self,
            nonce: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, invalidateNonceCall, N> {
            self.call_builder(&invalidateNonceCall { nonce })
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
        ///Creates a new call builder for the [`unlockCallback`] function.
        pub fn unlockCallback(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockCallbackCall, N> {
            self.call_builder(&unlockCallbackCall { data })
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
