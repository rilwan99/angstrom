///Module containing a contract's types and functions.
/**

```solidity
library IPoolManager {
    struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
    struct SwapParams { bool zeroForOne; int256 amountSpecified; uint160 sqrtPriceLimitX96; }
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
    /**```solidity
struct SwapParams { bool zeroForOne; int256 amountSpecified; uint160 sqrtPriceLimitX96; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SwapParams {
        pub zeroForOne: bool,
        pub amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
        pub sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::Uint<160>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            bool,
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
        impl ::core::convert::From<SwapParams> for UnderlyingRustTuple<'_> {
            fn from(value: SwapParams) -> Self {
                (value.zeroForOne, value.amountSpecified, value.sqrtPriceLimitX96)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    zeroForOne: tuple.0,
                    amountSpecified: tuple.1,
                    sqrtPriceLimitX96: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SwapParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SwapParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.zeroForOne,
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
        impl alloy_sol_types::SolType for SwapParams {
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
        impl alloy_sol_types::SolStruct for SwapParams {
            const NAME: &'static str = "SwapParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SwapParams(bool zeroForOne,int256 amountSpecified,uint160 sqrtPriceLimitX96)",
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.zeroForOne,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.amountSpecified,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sqrtPriceLimitX96,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SwapParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.zeroForOne,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountSpecified,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sqrtPriceLimitX96,
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
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.zeroForOne,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountSpecified,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sqrtPriceLimitX96,
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
    struct SwapParams {
        bool zeroForOne;
        int256 amountSpecified;
        uint160 sqrtPriceLimitX96;
    }
}

interface PoolManager {
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

    error AlreadyUnlocked();
    error ContractUnlocked();
    error CurrenciesOutOfOrderOrEqual(address currency0, address currency1);
    error CurrencyNotSettled();
    error DelegateCallNotAllowed();
    error InvalidCaller();
    error ManagerLocked();
    error MustClearExactPositiveDelta();
    error NonzeroNativeValue();
    error PoolNotInitialized();
    error ProtocolFeeTooLarge(uint24 fee);
    error SwapAmountCannotBeZero();
    error TickSpacingTooLarge(int24 tickSpacing);
    error TickSpacingTooSmall(int24 tickSpacing);
    error UnauthorizedDynamicLPFeeUpdate();

    event Approval(address indexed owner, address indexed spender, uint256 indexed id, uint256 amount);
    event Donate(PoolId indexed id, address indexed sender, uint256 amount0, uint256 amount1);
    event Initialize(PoolId indexed id, Currency indexed currency0, Currency indexed currency1, uint24 fee, int24 tickSpacing, address hooks, uint160 sqrtPriceX96, int24 tick);
    event ModifyLiquidity(PoolId indexed id, address indexed sender, int24 tickLower, int24 tickUpper, int256 liquidityDelta, bytes32 salt);
    event OperatorSet(address indexed owner, address indexed operator, bool approved);
    event OwnershipTransferred(address indexed user, address indexed newOwner);
    event ProtocolFeeControllerUpdated(address indexed protocolFeeController);
    event ProtocolFeeUpdated(PoolId indexed id, uint24 protocolFee);
    event Swap(PoolId indexed id, address indexed sender, int128 amount0, int128 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick, uint24 fee);
    event Transfer(address caller, address indexed from, address indexed to, uint256 indexed id, uint256 amount);

    function allowance(address owner, address spender, uint256 id) external view returns (uint256 amount);
    function approve(address spender, uint256 id, uint256 amount) external returns (bool);
    function balanceOf(address owner, uint256 id) external view returns (uint256 balance);
    function burn(address from, uint256 id, uint256 amount) external;
    function clear(Currency currency, uint256 amount) external;
    function collectProtocolFees(address recipient, Currency currency, uint256 amount) external returns (uint256 amountCollected);
    function donate(PoolKey memory key, uint256 amount0, uint256 amount1, bytes memory hookData) external returns (BalanceDelta delta);
    function extsload(bytes32 slot) external view returns (bytes32);
    function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory);
    function extsload(bytes32[] memory slots) external view returns (bytes32[] memory);
    function exttload(bytes32[] memory slots) external view returns (bytes32[] memory);
    function exttload(bytes32 slot) external view returns (bytes32);
    function initialize(PoolKey memory key, uint160 sqrtPriceX96) external returns (int24 tick);
    function isOperator(address owner, address operator) external view returns (bool isOperator);
    function mint(address to, uint256 id, uint256 amount) external;
    function modifyLiquidity(PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory hookData) external returns (BalanceDelta callerDelta, BalanceDelta feesAccrued);
    function owner() external view returns (address);
    function protocolFeeController() external view returns (address);
    function protocolFeesAccrued(Currency currency) external view returns (uint256 amount);
    function setOperator(address operator, bool approved) external returns (bool);
    function setProtocolFee(PoolKey memory key, uint24 newProtocolFee) external;
    function setProtocolFeeController(address controller) external;
    function settle() external payable returns (uint256);
    function settleFor(address recipient) external payable returns (uint256);
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function swap(PoolKey memory key, IPoolManager.SwapParams memory params, bytes memory hookData) external returns (BalanceDelta swapDelta);
    function sync(Currency currency) external;
    function take(Currency currency, address to, uint256 amount) external;
    function transfer(address receiver, uint256 id, uint256 amount) external returns (bool);
    function transferFrom(address sender, address receiver, uint256 id, uint256 amount) external returns (bool);
    function transferOwnership(address newOwner) external;
    function unlock(bytes memory data) external returns (bytes memory result);
    function updateDynamicLPFee(PoolKey memory key, uint24 newDynamicLPFee) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "allowance",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "approve",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "burn",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "clear",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
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
    "name": "collectProtocolFees",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountCollected",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "donate",
    "inputs": [
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
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "extsload",
    "inputs": [
      {
        "name": "startSlot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "nSlots",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "extsload",
    "inputs": [
      {
        "name": "slots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "exttload",
    "inputs": [
      {
        "name": "slots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "exttload",
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
    "name": "initialize",
    "inputs": [
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
        "name": "sqrtPriceX96",
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
    "name": "isOperator",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "isOperator",
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
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "modifyLiquidity",
    "inputs": [
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
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "callerDelta",
        "type": "int256",
        "internalType": "BalanceDelta"
      },
      {
        "name": "feesAccrued",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "owner",
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
    "name": "protocolFeeController",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IProtocolFeeController"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "protocolFeesAccrued",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      }
    ],
    "outputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProtocolFee",
    "inputs": [
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
        "name": "newProtocolFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProtocolFeeController",
    "inputs": [
      {
        "name": "controller",
        "type": "address",
        "internalType": "contract IProtocolFeeController"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "settle",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "settleFor",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
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
    "name": "swap",
    "inputs": [
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
        "internalType": "struct IPoolManager.SwapParams",
        "components": [
          {
            "name": "zeroForOne",
            "type": "bool",
            "internalType": "bool"
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
        ]
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "sync",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "take",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
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
    "name": "transfer",
    "inputs": [
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unlock",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "result",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateDynamicLPFee",
    "inputs": [
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
        "name": "newDynamicLPFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Donate",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount0",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialize",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "currency0",
        "type": "address",
        "indexed": true,
        "internalType": "Currency"
      },
      {
        "name": "currency1",
        "type": "address",
        "indexed": true,
        "internalType": "Currency"
      },
      {
        "name": "fee",
        "type": "uint24",
        "indexed": false,
        "internalType": "uint24"
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "hooks",
        "type": "address",
        "indexed": false,
        "internalType": "contract IHooks"
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "indexed": false,
        "internalType": "uint160"
      },
      {
        "name": "tick",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ModifyLiquidity",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tickLower",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "liquidityDelta",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSet",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolFeeControllerUpdated",
    "inputs": [
      {
        "name": "protocolFeeController",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolFeeUpdated",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "protocolFee",
        "type": "uint24",
        "indexed": false,
        "internalType": "uint24"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Swap",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount0",
        "type": "int128",
        "indexed": false,
        "internalType": "int128"
      },
      {
        "name": "amount1",
        "type": "int128",
        "indexed": false,
        "internalType": "int128"
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "indexed": false,
        "internalType": "uint160"
      },
      {
        "name": "liquidity",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      },
      {
        "name": "tick",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "fee",
        "type": "uint24",
        "indexed": false,
        "internalType": "uint24"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyUnlocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ContractUnlocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrenciesOutOfOrderOrEqual",
    "inputs": [
      {
        "name": "currency0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currency1",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "CurrencyNotSettled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DelegateCallNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCaller",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ManagerLocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustClearExactPositiveDelta",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonzeroNativeValue",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PoolNotInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProtocolFeeTooLarge",
    "inputs": [
      {
        "name": "fee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ]
  },
  {
    "type": "error",
    "name": "SwapAmountCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TickSpacingTooLarge",
    "inputs": [
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ]
  },
  {
    "type": "error",
    "name": "TickSpacingTooSmall",
    "inputs": [
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnauthorizedDynamicLPFeeUpdate",
    "inputs": []
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod PoolManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052348015600e575f80fd5b505f80546001600160a01b031916339081178255604051909182917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908290a3503060805260805161682261006b5f395f61205101526168225ff3fe6080604052600436106101f4575f3560e01c80635a6bcfda11610117578063a5841194116100ac578063f135baaa1161007c578063f3cd914c11610062578063f3cd914c14610676578063f5298aca14610695578063fe99049a146106b4575f80fd5b8063f135baaa14610638578063f2fde38b14610657575f80fd5b8063a584119414610595578063b6363cf2146105b4578063dbd035ff146105ed578063f02de3b21461060c575f80fd5b80638161b874116100e75780638161b874146104dc5780638da5cb5b146104fb57806397e8cd4e1461054b5780639bf6645f14610576575f80fd5b80635a6bcfda146104385780636276cbbe1461046c5780637e87ce7d1461049e57806380f0b44c146104bd575f80fd5b80632d7713891161018d57806348c894911161015d57806348c894911461039257806352759651146103be578063558a7297146103dd578063598af9e7146103fc575f80fd5b80632d7713891461031557806335fd631a146103345780633dd45adb14610360578063426a849314610373575f80fd5b806311da60b4116101c857806311da60b4146102b0578063156e29f6146102b85780631e2eaeaf146102d7578063234266d7146102f6575f80fd5b8062fdd58e146101f857806301ffc9a714610241578063095bcdb6146102705780630b0d9c091461028f575b5f80fd5b348015610203575f80fd5b5061022e61021236600461584b565b600460209081525f928352604080842090915290825290205481565b6040519081526020015b60405180910390f35b34801561024c575f80fd5b5061026061025b366004615875565b6106d3565b6040519015158152602001610238565b34801561027b575f80fd5b5061026061028a3660046158b4565b61076b565b34801561029a575f80fd5b506102ae6102a93660046158e6565b61083f565b005b61022e6108c9565b3480156102c3575f80fd5b506102ae6102d23660046158b4565b610927565b3480156102e2575f80fd5b5061022e6102f1366004615924565b6109ab565b348015610301575f80fd5b5061022e610310366004615ade565b6109b5565b348015610320575f80fd5b506102ae61032f366004615b43565b610ad9565b34801561033f575f80fd5b5061035361034e366004615b5e565b610bcc565b6040516102389190615b7e565b61022e61036e366004615b43565b610c09565b34801561037e575f80fd5b5061026061038d3660046158b4565b610c67565b34801561039d575f80fd5b506103b16103ac366004615bc0565b610cd8565b6040516102389190615bff565b3480156103c9575f80fd5b506102ae6103d8366004615c52565b610e2a565b3480156103e8575f80fd5b506102606103f7366004615c93565b610ecc565b348015610407575f80fd5b5061022e6104163660046158e6565b600560209081525f938452604080852082529284528284209052825290205481565b348015610443575f80fd5b50610457610452366004615cbd565b610f66565b60408051928352602083019190915201610238565b348015610477575f80fd5b5061048b610486366004615d7e565b611165565b60405160029190910b8152602001610238565b3480156104a9575f80fd5b506102ae6104b8366004615c52565b6113fd565b3480156104c8575f80fd5b506102ae6104d736600461584b565b6114ee565b3480156104e7575f80fd5b5061022e6104f63660046158e6565b6115ad565b348015610506575f80fd5b505f546105269073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610238565b348015610556575f80fd5b5061022e610565366004615b43565b60016020525f908152604090205481565b348015610581575f80fd5b50610353610590366004615db4565b6116d9565b3480156105a0575f80fd5b506102ae6105af366004615b43565b611712565b3480156105bf575f80fd5b506102606105ce366004615e25565b600360209081525f928352604080842090915290825290205460ff1681565b3480156105f8575f80fd5b50610353610607366004615db4565b6117b7565b348015610617575f80fd5b506002546105269073ffffffffffffffffffffffffffffffffffffffff1681565b348015610643575f80fd5b5061022e610652366004615924565b6117ee565b348015610662575f80fd5b506102ae610671366004615b43565b6117f8565b348015610681575f80fd5b5061022e610690366004615e51565b6118e7565b3480156106a0575f80fd5b506102ae6106af3660046158b4565b611a99565b3480156106bf575f80fd5b506102606106ce366004615f0f565b611b1d565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316148061076557507f0f632fb3000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b335f908152600460209081526040808320858452909152812080548391908390610796908490615f7f565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f908152600460209081526040808320868452909152812080548492906107da908490615f92565b9091555050604080513380825260208201859052859273ffffffffffffffffffffffffffffffffffffffff8816927f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a45060015b9392505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61088e5761088e7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b6108a38361089b83611d11565b5f0333611d56565b6108c473ffffffffffffffffffffffffffffffffffffffff84168383611db6565b505050565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610919576109197f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b61092233611eb1565b905090565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610976576109767f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b816109848161089b84611d11565b6109a58473ffffffffffffffffffffffffffffffffffffffff831684611f9a565b50505050565b5f81545f5260205ff35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610a0557610a057f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b610a0d612039565b60a086205f818152600660205260409020610a27816120a1565b6080880151610a509073ffffffffffffffffffffffffffffffffffffffff1689898989896120e8565b610a5b8188886121cb565b9250610a688884336122c5565b6040805188815260208101889052339184917f29ef05caaff9404b7cb6d1c0e9bbae9eaa7ab2541feba1a9c4248594c08156cb910160405180910390a36080880151610ace9073ffffffffffffffffffffffffffffffffffffffff1689898989896122ed565b505095945050505050565b5f5473ffffffffffffffffffffffffffffffffffffffff163314610b5e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a4544000000000000000000000000000000000000000060448201526064015b60405180910390fd5b600280547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081179091556040517fb4bd8ef53df690b9943d3318996006dbb82a25f54719d8c8035b516a2a5b8acc905f90a250565b6060604051808360051b6020835284602084015260408301925080830190505b85548352602083019250600186019550808310610bec5781810382f35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610c5957610c597f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b61076582611eb1565b919050565b335f81815260056020908152604080832073ffffffffffffffffffffffffffffffffffffffff881680855290835281842087855290925280832085905551919285927fb3fd5071835887567a0671151121894ddccc2842f1d10bedad13e0d17cace9a79061082c9087815260200190565b60607fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c15610d2a57610d2a7f5090d6c600000000000000000000000000000000000000000000000000000000611d09565b610d326123c5565b6040517f91dd734600000000000000000000000000000000000000000000000000000000815233906391dd734690610d709086908690600401615fec565b5f604051808303815f875af1158015610d8b573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610dd09190810190615fff565b90507f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c15610e2257610e227f5212cba100000000000000000000000000000000000000000000000000000000611d09565b6107656123eb565b604082015162ffffff1662800000141580610e755750816080015173ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b15610ea357610ea37f30d2164100000000000000000000000000000000000000000000000000000000611d09565b610eb18162ffffff16612410565b60a082205f8181526006602052604090206108c4908361244f565b335f81815260036020908152604080832073ffffffffffffffffffffffffffffffffffffffff871680855290835281842080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001687151590811790915591519182529293917fceb576d9f15e4e200fdb5096d64d5dfd667e16def20c1eefd14256d8e3faa267910160405180910390a350600192915050565b5f807fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610fb757610fb77f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b610fbf612039565b60a086205f818152600660205260409020610fd9816120a1565b60808801516110019073ffffffffffffffffffffffffffffffffffffffff16898989896124a8565b5f6110756040518060c001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020018a5f015160020b81526020018a6020015160020b81526020016110528b60400151612669565b600f0b81526060808d015160020b60208301528b0151604090910152839061269e565b945090506110838185612ae3565b945050503373ffffffffffffffffffffffffffffffffffffffff16817ff208f4912782fd25c7f114ca3723a2d5dd6f3bcc3ac8db5af63baa85f711d5ec885f015189602001518a604001518b606001516040516111019493929190600294850b81529290930b60208301526040820152606081019190915260800190565b60405180910390a360808701515f906111359073ffffffffffffffffffffffffffffffffffffffff16898987878b8b612b16565b9094509050801561114f5761114f88828a608001516122c5565b61115a8885336122c5565b505094509492505050565b5f61116e612039565b6060830151617fff60029190910b13156111b25760608301516111b2907fb70024f80000000000000000000000000000000000000000000000000000000090612ced565b600160020b836060015160020b12156111f55760608301516111f5907fe9e905880000000000000000000000000000000000000000000000000000000090612ced565b8251602084015173ffffffffffffffffffffffffffffffffffffffff90811691161061124d578251602084015161124d917f6e6c98300000000000000000000000000000000000000000000000000000000091612cfc565b61127e8360400151846080015173ffffffffffffffffffffffffffffffffffffffff16612d3f90919063ffffffff16565b6112b25760808301516112b2907fe65af6a00000000000000000000000000000000000000000000000000000000090612e0d565b5f6112c5846040015162ffffff16612e2f565b60808501519091506112ee9073ffffffffffffffffffffffffffffffffffffffff168585612e54565b60a084205f81815260066020526040902061130a908584612f27565b60808601519093506113349073ffffffffffffffffffffffffffffffffffffffff16868686612fdf565b846020015173ffffffffffffffffffffffffffffffffffffffff16855f015173ffffffffffffffffffffffffffffffffffffffff16827fdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d6438886040015189606001518a608001518a8a6040516113ed95949392919062ffffff959095168552600293840b602086015273ffffffffffffffffffffffffffffffffffffffff928316604086015291166060840152900b608082015260a00190565b60405180910390a4505092915050565b60025473ffffffffffffffffffffffffffffffffffffffff163314611445576114457f48f5c3ed00000000000000000000000000000000000000000000000000000000611d09565b6103e9610fff821610623e900062fff0008316101661148d5761148d7fa7abe2f70000000000000000000000000000000000000000000000000000000062ffffff8316612e0d565b60a082206114af826114a9835f90815260066020526040902090565b906130b4565b60405162ffffff8316815281907fe9c42593e71f84403b84352cd168d693e2c9fcd1fdbcc3feb21d92b43e6696f99060200160405180910390a2505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61153d5761153d7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b335f90815273ffffffffffffffffffffffffffffffffffffffff8316602052604081205c9061156b83611d11565b90508181600f0b146115a0576115a07fbda73abf00000000000000000000000000000000000000000000000000000000611d09565b6109a584825f0333611d56565b6002545f9073ffffffffffffffffffffffffffffffffffffffff1633146115f7576115f77f48f5c3ed00000000000000000000000000000000000000000000000000000000611d09565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c15611647576116477f3e5f4fd600000000000000000000000000000000000000000000000000000000611d09565b81156116535781611679565b73ffffffffffffffffffffffffffffffffffffffff83165f908152600160205260409020545b73ffffffffffffffffffffffffffffffffffffffff84165f908152600160205260408120805492935083929091906116b2908490615f7f565b90915550610838905073ffffffffffffffffffffffffffffffffffffffff84168583611db6565b606060405180602082528360208301526040820191508360051b8201855b80355c8452602093840193018184106116f7575b5081810382f35b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611761576117617f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b73ffffffffffffffffffffffffffffffffffffffff811661178757611784613108565b50565b5f6117a78273ffffffffffffffffffffffffffffffffffffffff1661312d565b90506117b382826131dc565b5050565b606060405180602082528360208301526040820191508360051b8201855b8035548452602093840193018184101561170b576117d5565b5f815c5f5260205ff35b5f5473ffffffffffffffffffffffffffffffffffffffff163314611878576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a454400000000000000000000000000000000000000006044820152606401610b55565b5f80547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081178255604051909133917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a350565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611937576119377f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b61193f612039565b83602001515f03611973576119737fbe8b850700000000000000000000000000000000000000000000000000000000611d09565b60a085205f81815260066020526040902061198d816120a1565b60808701515f90819081906119bb9073ffffffffffffffffffffffffffffffffffffffff168b8b8b8b61323c565b809350819550829450505050611a3784866040518060a001604052808681526020018e6060015160020b81526020018d5f0151151581526020018d6040015173ffffffffffffffffffffffffffffffffffffffff1681526020018562ffffff168152508c5f0151611a30578d602001516133e3565b8d516133e3565b60808b01519096505f9250611a68915073ffffffffffffffffffffffffffffffffffffffff168a8a888b8b886134e4565b90955090508015611a8257611a8289828b608001516122c5565b611a8d8986336122c5565b50505050949350505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611ae857611ae87f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b81611afc81611af684611d11565b33611d56565b6109a58473ffffffffffffffffffffffffffffffffffffffff83168461367d565b5f3373ffffffffffffffffffffffffffffffffffffffff861614801590611b74575073ffffffffffffffffffffffffffffffffffffffff85165f90815260036020908152604080832033845290915290205460ff16155b15611c1d5773ffffffffffffffffffffffffffffffffffffffff85165f90815260056020908152604080832033845282528083208684529091529020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114611c1b57611be28382615f7f565b73ffffffffffffffffffffffffffffffffffffffff87165f90815260056020908152604080832033845282528083208884529091529020555b505b73ffffffffffffffffffffffffffffffffffffffff85165f90815260046020908152604080832086845290915281208054849290611c5c908490615f7f565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f90815260046020908152604080832086845290915281208054849290611ca0908490615f92565b90915550506040805133815260208101849052849173ffffffffffffffffffffffffffffffffffffffff80881692908916917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859910160405180910390a45060015b949350505050565b805f5260045ffd5b5f6f800000000000000000000000000000008210611d5257611d527f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b5090565b81600f0b5f03611d6557505050565b5f80611d8873ffffffffffffffffffffffffffffffffffffffff8616848661378e565b91509150805f03611da057611d9b6137d4565b611daf565b815f03611daf57611daf613822565b5050505050565b5f73ffffffffffffffffffffffffffffffffffffffff8416611e10575f805f8085875af1905080611e0b57611e0b7f8549db590000000000000000000000000000000000000000000000000000000084613870565b6109a5565b6040517fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8416600482015282602482015260205f6044835f895af13d15601f3d1160015f511416171691505f81525f60208201525f604082015250806109a5576109a57fb12c5f9c0000000000000000000000000000000000000000000000000000000085613870565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95c73ffffffffffffffffffffffffffffffffffffffff8116611ef757349150611f81565b3415611f2657611f267fb0ec849e00000000000000000000000000000000000000000000000000000000611d09565b7f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955c5f611f6873ffffffffffffffffffffffffffffffffffffffff841661312d565b9050611f748282615f7f565b9350611f7e613108565b50505b611f9481611f8e84611d11565b85611d56565b50919050565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290611fd9908490615f92565b90915550506040805133815260208101839052839173ffffffffffffffffffffffffffffffffffffffff8616915f917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a4505050565b3073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461209f5761209f7f0d89438e00000000000000000000000000000000000000000000000000000000611d09565b565b805473ffffffffffffffffffffffffffffffffffffffff165f03611784576117847f486aa30700000000000000000000000000000000000000000000000000000000611d09565b853373ffffffffffffffffffffffffffffffffffffffff8216146121c25760208716156121c2576121c033878787878760405160240161212d969594939291906160b1565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb6a8b0fa0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8916906138a3565b505b50505050505050565b60038301545f906fffffffffffffffffffffffffffffffff16808203612214576122147fa74f97ab00000000000000000000000000000000000000000000000000000000611d09565b61224b61222085611d11565b5f0361222b85611d11565b5f0360809190911b6fffffffffffffffffffffffffffffffff9091161790565b91508315612285576001850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008702040190555b82156122bd576002850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008602040190555b509392505050565b82516122db906122d58460801d90565b83611d56565b6108c483602001516122d584600f0b90565b853373ffffffffffffffffffffffffffffffffffffffff8216146121c25760108716156121c2576121c0338787878787604051602401612332969594939291906160b1565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe1b4af690000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8916906138a3565b60017fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b620f424062ffffff82161115611784576117847f140021130000000000000000000000000000000000000000000000000000000062ffffff8316612e0d565b612458826120a1565b81547fffffff000000ffffffffffffffffffffffffffffffffffffffffffffffffffff167cffffff000000000000000000000000000000000000000000000000000060d083901b16175b90915550565b843373ffffffffffffffffffffffffffffffffffffffff821614612661575f84604001511380156124dc5750610800861615155b156125965761259033868686866040516024016124fd95949392919061617d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f259982e50000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8816906138a3565b50612661565b5f8460400151131580156125ad5750610200861615155b15612661576121c233868686866040516024016125ce95949392919061617d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f21d0ee700000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8816906138a3565b505050505050565b80600f81900b8114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b6060810151602082015160408301515f92839290916126bd8282613996565b604080516080810182525f80825260208201819052918101829052606081019190915283600f0b5f1461288d576126f68884865f613a5d565b6fffffffffffffffffffffffffffffffff1660208301521515815261271e8883866001613a5d565b6fffffffffffffffffffffffffffffffff166060830152151560408201525f600f85900b126128525760808701515f9060020b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761881810783139082900503620d89e891909105036001016fffffffffffffffffffffffffffffffff049050806fffffffffffffffffffffffffffffffff1682602001516fffffffffffffffffffffffffffffffff1611156127f6576127f67fb8e3c3850000000000000000000000000000000000000000000000000000000085612ced565b806fffffffffffffffffffffffffffffffff1682606001516fffffffffffffffffffffffffffffffff161115612850576128507fb8e3c3850000000000000000000000000000000000000000000000000000000084612ced565b505b80511561286e57608087015161286e9060058a01908590613b46565b80604001511561288d57608087015161288d9060058a01908490613b46565b5f8061289a8a8686613b98565b8a5160a08c015160408051602681019290925260068083018a9052600383018b9052928252603a600c8301205f838301819052602080850182905293819052908152928f019091528120929450909250806128f7838a8787613c4c565b9150915061292c61290783611d11565b61291083611d11565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b995050505050505f84600f0b12156129955780511561296857600283810b5f90815260048a016020526040812081815560018101829055909101555b80604001511561299557600282810b5f90815260048a016020526040812081815560018101829055909101555b5082600f0b5f14612ad95786545f806129b18360a01c60020b90565b73ffffffffffffffffffffffffffffffffffffffff8416915091508460020b8260020b1215612a0d57612a06612a006129fb6129ec88613d7d565b6129f588613d7d565b8a61407a565b612669565b60801b90565b9750612ad5565b8360020b8260020b1215612ab057612a44612a2e6129fb836129f588613d7d565b6129106129fb612a3d89613d7d565b858b6140b2565b60038b0154909850612a68906fffffffffffffffffffffffffffffffff16876140de565b60038b0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff92909216919091179055612ad5565b612ad25f6129106129fb612ac389613d7d565b612acc89613d7d565b8b6140b2565b97505b5050505b5050509250929050565b5f608082811d9084901d01600f83810b9085900b01612b0d612b0483612669565b61291083612669565b95945050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff89163303612b3f57508490505f612ce1565b8591505f87604001511315612c2757610400891615612c2257612c1333898989898989604051602401612b78979695949392919061625f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9f063efc0000000000000000000000000000000000000000000000000000000017905260028b1615155b73ffffffffffffffffffffffffffffffffffffffff8c16919061410e565b9050612c1f8282614168565b91505b612ce1565b610100891615612ce157612cd233898989898989604051602401612c51979695949392919061625f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f6c2bbe7e0000000000000000000000000000000000000000000000000000000017905260018b161515612bf5565b9050612cde8282614168565b91505b97509795505050505050565b815f528060020b60045260245ffd5b60405183815273ffffffffffffffffffffffffffffffffffffffff8316600482015273ffffffffffffffffffffffffffffffffffffffff82166024820152604481fd5b5f60808316158015612d5357506008831615155b15612d5f57505f610765565b60408316158015612d7257506004831615155b15612d7e57505f610765565b6104008316158015612d9257506002831615155b15612d9e57505f610765565b6101008316158015612db257506001831615155b15612dbe57505f610765565b73ffffffffffffffffffffffffffffffffffffffff831615612dfc57613fff8316151580612df757506280000062ffffff831614610838565b610838565b5062ffffff16628000001415919050565b815f5273ffffffffffffffffffffffffffffffffffffffff811660045260245ffd5b5f6280000062ffffff831603612e4657505f919050565b611d528262ffffff16612410565b823373ffffffffffffffffffffffffffffffffffffffff8216146109a5576120008416156109a557611daf338484604051602401612e949392919061634f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fdc98354e0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8616906138a3565b82545f9073ffffffffffffffffffffffffffffffffffffffff1615612f6f57612f6f7f7983c05100000000000000000000000000000000000000000000000000000000611d09565b612f7883614189565b90507cffffff000000000000000000000000000000000000000000000000000060d083901b1673ffffffffffffffffffffffffffffffffffffffff841660a083901b76ffffff00000000000000000000000000000000000000001617179093555090919050565b833373ffffffffffffffffffffffffffffffffffffffff821614611daf57611000851615611daf57612661338585856040516024016130219493929190616412565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f6fe7e6eb0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8716906138a3565b6130bd826120a1565b81547fffffffffffff000000ffffffffffffffffffffffffffffffffffffffffffffff1679ffffff000000000000000000000000000000000000000000000060b883901b16176124a2565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d565b5f73ffffffffffffffffffffffffffffffffffffffff8216613150575047919050565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff8316906370a0823190602401602060405180830381865afa1580156131b8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061076591906164e0565b73ffffffffffffffffffffffffffffffffffffffff82167f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d807f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955d5050565b60208301515f8073ffffffffffffffffffffffffffffffffffffffff88163303613268575f91506133d8565b60808816156133d8575f61330a89338a8a8a8a60405160240161328f9594939291906164f7565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f575e24b4000000000000000000000000000000000000000000000000000000001790526138a3565b9050805160601461333e5761333e7f1e048e1d00000000000000000000000000000000000000000000000000000000611d09565b604088015162ffffff16628000000361335957606081015191505b60088916156133d657604081015192505f6133748460801d90565b905080600f0b5f146133d4575f8512613391600f83900b876165df565b9550806133a0575f86126133a4565b5f86135b156133d2576133d27ffa0b71d600000000000000000000000000000000000000000000000000000000611d09565b505b505b505b955095509592505050565b5f808080806133f289886144ad565b93509350935093505f83111561342d5773ffffffffffffffffffffffffffffffffffffffff86165f9081526001602052604090208054840190555b33887f40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f61345a8760801d90565b61346488600f0b90565b85516040808801516020808a01518351600f97880b81529590960b9085015273ffffffffffffffffffffffffffffffffffffffff909216908301526fffffffffffffffffffffffffffffffff16606082015260029190910b608082015262ffffff861660a082015260c00160405180910390a35091979650505050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff8916330361350d57508490505f612ce1565b5f6135188460801d90565b90505f61352585600f0b90565b905060408b16156135f8576135eb6129fb338c8c8c8c8c60405160240161355196959493929190616606565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb47b2fb10000000000000000000000000000000000000000000000000000000017905260048e16151573ffffffffffffffffffffffffffffffffffffffff8f16919061410e565b6135f590826166f5565b90505b5f81600f0b5f14158061360e575082600f0b5f14155b1561366b57895160208b01515f1390151514613642576fffffffffffffffffffffffffffffffff8316608083901b1761365c565b6fffffffffffffffffffffffffffffffff8216608084901b175b90506136688982614168565b98505b979b979a509698505050505050505050565b3373ffffffffffffffffffffffffffffffffffffffff841681148015906136d6575073ffffffffffffffffffffffffffffffffffffffff8085165f9081526003602090815260408083209385168352929052205460ff16155b156137835773ffffffffffffffffffffffffffffffffffffffff8085165f9081526005602090815260408083209385168352928152828220868352905220547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114613781576137468382615f7f565b73ffffffffffffffffffffffffffffffffffffffff8087165f9081526005602090815260408083209387168352928152828220888352905220555b505b6109a5848484614d3f565b73ffffffffffffffffffffffffffffffffffffffff8281165f90815290841660205260408120805c91906137c6600f85900b846165df565b915081815d50935093915050565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181039050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181019050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b3d60405183815282600482015260406024820152816044820152815f606483013e602080601f8401040260640191508181fd5b60605f805f8451602086015f885af19050806138e3576138e37f319d54c30000000000000000000000000000000000000000000000000000000085613870565b6040519150601f19603f3d011682016040523d82523d5f602084013e602082511080613961575060208301517fffffffff000000000000000000000000000000000000000000000000000000001661393c836020015190565b7fffffffff000000000000000000000000000000000000000000000000000000001614155b1561398f5761398f7f1e048e1d00000000000000000000000000000000000000000000000000000000611d09565b5092915050565b8060020b8260020b126139ce576139ce7fc4433ed5000000000000000000000000000000000000000000000000000000008383614dd5565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618600283900b1215613a2457613a247fd5e2f7ab0000000000000000000000000000000000000000000000000000000083612ced565b620d89e8600282900b13156117b3576117b37f1ad777f80000000000000000000000000000000000000000000000000000000082612ced565b600283900b5f908152600485016020526040812080548291906fffffffffffffffffffffffffffffffff8116907001000000000000000000000000000000009004600f0b613aab82886140de565b6fffffffffffffffffffffffffffffffff808216159084168015919091141596509094505f03613afe57885460a01c60020b60020b8860020b13613afe576001808a0154908401556002808a0154908401555b5f86613b1357613b0e88836166f5565b613b1d565b613b1d8883616743565b90508060801b6fffffffffffffffffffffffffffffffff86161784555050505094509492505050565b600291820b910b80820715613b735760405163d4d8f3e681528260208201528160408201526044601c8201fd5b80820591508160081d5f528260205260405f20600160ff84161b815418815550505050565b600282810b5f81815260048601602052604080822085850b83529082208754929485949293919260a09290921c900b90811215613bee578160010154836001015403945081600201548360020154039350613c41565b8560020b8160020b12613c1a578260010154826001015403945082600201548260020154039350613c41565b81600101548360010154896001015403039450816002015483600201548960020154030393505b505050935093915050565b83545f9081906fffffffffffffffffffffffffffffffff16600f86900b8203613cb657806fffffffffffffffffffffffffffffffff165f03613cb157613cb17faefeb92400000000000000000000000000000000000000000000000000000000611d09565b613cfd565b613cc081876140de565b87547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff919091161787555b613d3187600101548603826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614df2565b9250613d6787600201548503826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614df2565b6001880195909555505060029094015591929050565b60020b5f60ff82901d80830118620d89e8811115613dbf57613dbf7f8b86327a0000000000000000000000000000000000000000000000000000000084612ced565b7001fffcb933bd6fad37aa2d162d1a5940016001821602700100000000000000000000000000000000186002821615613e08576ffff97272373d413259a46990580e213a0260801c5b6004821615613e27576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b6008821615613e46576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b6010821615613e65576fffcb9843d60f6159c9db58835c9266440260801c5b6020821615613e84576fff973b41fa98c081472e6896dfb254c00260801c5b6040821615613ea3576fff2ea16466c96a3843ec78b326b528610260801c5b6080821615613ec2576ffe5dee046a99a2a811c461f1969c30530260801c5b610100821615613ee2576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b610200821615613f02576ff987a7253ac413176f2b074cf7815e540260801c5b610400821615613f22576ff3392b0822b70005940c7a398e4b70f30260801c5b610800821615613f42576fe7159475a2c29b7443b29c7fa6e889d90260801c5b611000821615613f62576fd097f3bdfd2022b8845ad8f792aa58250260801c5b612000821615613f82576fa9f746462d870fdf8a65dc1f90e061e50260801c5b614000821615613fa2576f70d869a156d2a1b890bb3df62baf32f70260801c5b618000821615613fc2576f31be135f97d08fd981231505542fcfa60260801c5b62010000821615613fe3576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b62020000821615614003576e5d6af8dedb81196699c329225ee6040260801c5b62040000821615614022576d2216e584f5fa1ea926041bedfe980260801c5b6208000082161561403f576b048a170391f7dc42444e8fa20260801c5b5f84131561406a577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff045b63ffffffff0160201c9392505050565b5f8082600f0b126140a15761409a6140958585856001614ead565b614fea565b5f03611d01565b611d016140958585855f035f614ead565b5f8082600f0b126140cd5761409a614095858585600161501c565b611d016140958585855f035f61501c565b6fffffffffffffffffffffffffffffffff8216600f82900b01608081901c15610765576393dafdf15f526004601cfd5b5f8061411a85856138a3565b90508261412a575f915050610838565b805160401461415c5761415c7f1e048e1d00000000000000000000000000000000000000000000000000000000611d09565b60400151949350505050565b5f608082811d9084901d03600f83810b9085900b03612b0d612b0483612669565b5f73fffd8963efd1fc6a506488495d951d51639616827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffefffd895d830173ffffffffffffffffffffffffffffffffffffffff161115614208576142087f614875240000000000000000000000000000000000000000000000000000000083612e0d565b77ffffffffffffffffffffffffffffffffffffffff00000000602083901b16805f61423282615087565b60ff1690506080811061424d57607f810383901c9150614257565b80607f0383901b91505b908002607f81811c60ff83811c9190911c800280831c81831c1c800280841c81841c1c800280851c81851c1c800280861c81861c1c800280871c81871c1c800280881c81881c1c800280891c81891c1c8002808a1c818a1c1c8002808b1c818b1c1c8002808c1c818c1c1c8002808d1c818d1c1c8002808e1c9c81901c9c909c1c80029c8d901c9e9d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808f0160401b60c09190911c678000000000000000161760c19b909b1c674000000000000000169a909a1760c29990991c672000000000000000169890981760c39790971c671000000000000000169690961760c49590951c670800000000000000169490941760c59390931c670400000000000000169290921760c69190911c670200000000000000161760c79190911c670100000000000000161760c89190911c6680000000000000161760c99190911c6640000000000000161760ca9190911c6620000000000000161760cb9190911c6610000000000000161760cc9190911c6608000000000000161760cd9190911c66040000000000001617693627a301d71055774c8581027ffffffffffffffffffffffffffffffffffd709b7e5480fba5a50fed5e62ffc5568101608090811d906fdb2df09e81959a81455e260799a0632f8301901d600281810b9083900b1461449e578873ffffffffffffffffffffffffffffffffffffffff1661447682613d7d565b73ffffffffffffffffffffffffffffffffffffffff16111561449857816144a0565b806144a0565b815b9998505050505050505050565b604080516060810182525f8082526020820181905291810182905281908190855460408601515f816144e757610fff60c484901c166144f1565b610fff60b884901c165b885173ffffffffffffffffffffffffffffffffffffffff8516865261ffff9190911691505f60a085901c60020b60020b602087015260038b01546fffffffffffffffffffffffffffffffff16604087015260808a01515f9062400000166145615760d086901c62ffffff16614573565b6145738b6080015162ffffff1661511b565b905083156145a157620f4240610fff851662ffffff83168181028381061515939004929092019101036145a3565b805b975050620f42408762ffffff16106145e75789515f12156145e7576145e77f9620624600000000000000000000000000000000000000000000000000000000611d09565b89515f036145ff575f80985098505050505050614d36565b83156146e25760608a015173ffffffffffffffffffffffffffffffffffffffff8681169116106146715761467173ffffffffffffffffffffffffffffffffffffffff86165b60608c01517f7c9c6e8f000000000000000000000000000000000000000000000000000000009190612cfc565b6401000276a373ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff16116146dd5760608a01516146dd907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e0d565b6147a0565b60608a015173ffffffffffffffffffffffffffffffffffffffff8681169116116147255761472573ffffffffffffffffffffffffffffffffffffffff8616614644565b73fffd8963efd1fc6a506488495d951d5263988d2673ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff16106147a05760608a01516147a0907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e0d565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152846147ef578b600201546147f5565b8b600101545b60e08201525b82158061483a57508a6060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff16145b614bc257865173ffffffffffffffffffffffffffffffffffffffff168152602080880151908c01516148719160058f01918861512a565b1515604083015260020b602082018190527ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618126148cf577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761860208201525b620d89e860020b816020015160020b126148ed57620d89e860208201525b6148fa8160200151613d7d565b73ffffffffffffffffffffffffffffffffffffffff90811660608381018290528951908e0151614944939192911680821891811160018a161891909102188960400151868c615255565b60c085015260a0840152608083015273ffffffffffffffffffffffffffffffffffffffff1687528a515f12156149ad576149818160a00151614fea565b8303925061499c8160c0015182608001516140959190615f92565b6149a69083616791565b91506149de565b6149c08160c00151826080015101614fea565b830192506149d18160a00151614fea565b6149db90836165df565b91505b8315614a1a575f620f4240858360c001518460800151010281614a0357614a036167b0565b60c084018051929091049182900390529990990198505b60408701516fffffffffffffffffffffffffffffffff1615614a7957614a6d8160c0015170010000000000000000000000000000000089604001516fffffffffffffffffffffffffffffffff1691020490565b60e08201805190910190525b806060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff1603614b8f57806040015115614b6a575f8086614ad3578d600101548360e00151614ade565b8260e001518e600201545b915091505f614b368f85602001518585600292830b5f908152600490940160205260409093206001810180549092039091559081018054909203909155547001000000000000000000000000000000009004600f0b90565b90508715614b41575f035b614b4f8a60400151826140de565b6fffffffffffffffffffffffffffffffff1660408b01525050505b84614b79578060200151614b82565b60018160200151035b60020b60208801526147fb565b8051875173ffffffffffffffffffffffffffffffffffffffff908116911614614bbd578651614b8290614189565b6147fb565b86516020880151614c579190614c1990899060a01b76ffffff0000000000000000000000000000000000000000167fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff919091161790565b7fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9091161790565b8c55604087015160038d01546fffffffffffffffffffffffffffffffff908116911614614cc657604087015160038d0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff9092169190911790555b84614cda5760e081015160028d0155614ce5565b60e081015160018d01555b8a515f1385151514614d1257614d0b614cfd83612669565b612910858e5f015103612669565b9950614d2f565b614d2c614d23848d5f015103612669565b61291084612669565b99505b5050505050505b92959194509250565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290614d7e908490615f7f565b9091555050604080513381526020810183905283915f9173ffffffffffffffffffffffffffffffffffffffff8716917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859910161202c565b6040518381528260020b60048201528160020b6024820152604481fd5b5f838302817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff85870982811083820303915050808411614e30575f80fd5b805f03614e4257508290049050610838565b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff161115614ee6579293925b73ffffffffffffffffffffffffffffffffffffffff8516614f0d5762bfc9215f526004601cfd5b7bffffffffffffffffffffffffffffffff000000000000000000000000606084901b1673ffffffffffffffffffffffffffffffffffffffff8686031683614f99578673ffffffffffffffffffffffffffffffffffffffff16614f8683838973ffffffffffffffffffffffffffffffffffffffff16614df2565b81614f9357614f936167b0565b04614fdf565b614fdf614fbd83838973ffffffffffffffffffffffffffffffffffffffff166153c5565b8873ffffffffffffffffffffffffffffffffffffffff16808204910615150190565b979650505050505050565b805f811215610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b5f73ffffffffffffffffffffffffffffffffffffffff8481169086160360ff81901d908101186c010000000000000000000000006fffffffffffffffffffffffffffffffff851661506e818484614df2565b9350845f83858409111684019350505050949350505050565b5f808211615093575f80fd5b507f0706060506020500060203020504000106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be6fffffffffffffffffffffffffffffffff841160071b84811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b62bfffff8116610c6281612410565b5f80600284810b9086900b81810783139190050383156151c857600281900b60081d600181900b5f908152602089905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff808516908190039190911c9182168015159550909190856151aa57888360ff168603026151bd565b886151b482615087565b840360ff168603025b96505050505061524b565b6001908101600281900b60081d80830b5f90815260208a905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff841694851b01199081168015159550929391928561523157888360ff0360ff16860102615244565b888361523c836153f5565b0360ff168601025b9650505050505b5094509492505050565b5f80808062ffffff851673ffffffffffffffffffffffffffffffffffffffff808a16908b1610158288128015615338575f61529b8a5f0385620f424003620f4240614df2565b9050826152b4576152af8d8d8d600161501c565b6152c1565b6152c18c8e8d6001614ead565b96508681106152f5578b9750620f424084146152ec576152e7878586620f4240036153c5565b6152ee565b865b945061530e565b8096506153048d8c838661548f565b9750868a5f030394505b826153245761531f8d898d5f614ead565b615330565b615330888e8d5f61501c565b9550506153b6565b8161534e576153498c8c8c5f614ead565b61535a565b61535a8b8d8c5f61501c565b945084891061536b578a965061537d565b88945061537a8c8b87856154f3565b96505b816153945761538f8c888c600161501c565b6153a1565b6153a1878d8c6001614ead565b95506153b3868485620f4240036153c5565b93505b50505095509550955095915050565b5f6153d1848484614df2565b905081806153e1576153e16167b0565b838509156108385760010180610838575f80fd5b5f808211615401575f80fd5b507e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b15031604055f8290039091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff86161517156154cf57634f2461b85f526004601cfd5b816154e6576154e1858585600161554c565b612b0d565b612b0d85858560016156ae565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff861615171561553357634f2461b85f526004601cfd5b81615544576154e18585855f6156ae565b612b0d8585855f5b5f81156155f1575f73ffffffffffffffffffffffffffffffffffffffff84111561559f5761559a846c01000000000000000000000000876fffffffffffffffffffffffffffffffff16614df2565b6155bf565b6155bf6fffffffffffffffffffffffffffffffff8616606086901b6167dd565b90506155e96155e48273ffffffffffffffffffffffffffffffffffffffff8916615f92565b6157e3565b915050611d01565b5f73ffffffffffffffffffffffffffffffffffffffff84111561563d57615638846c01000000000000000000000000876fffffffffffffffffffffffffffffffff166153c5565b615663565b615663606085901b6fffffffffffffffffffffffffffffffff8716808204910615150190565b90508073ffffffffffffffffffffffffffffffffffffffff87161161568f57634323a5555f526004601cfd5b73ffffffffffffffffffffffffffffffffffffffff8616039050611d01565b5f825f036156bd575083611d01565b7bffffffffffffffffffffffffffffffff000000000000000000000000606085901b1682156157885773ffffffffffffffffffffffffffffffffffffffff861684810290858281615710576157106167b0565b040361574d5781810182811061574b57615741838973ffffffffffffffffffffffffffffffffffffffff16836153c5565b9350505050611d01565b505b506155e9818561577373ffffffffffffffffffffffffffffffffffffffff8a16836167dd565b61577d9190615f92565b808204910615150190565b73ffffffffffffffffffffffffffffffffffffffff86168481029085820414818311166157bc5763f5c787f15f526004601cfd5b8082036157416155e48473ffffffffffffffffffffffffffffffffffffffff8b16846153c5565b8073ffffffffffffffffffffffffffffffffffffffff81168114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b73ffffffffffffffffffffffffffffffffffffffff81168114611784575f80fd5b5f806040838503121561585c575f80fd5b82356158678161582a565b946020939093013593505050565b5f60208284031215615885575f80fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114610838575f80fd5b5f805f606084860312156158c6575f80fd5b83356158d18161582a565b95602085013595506040909401359392505050565b5f805f606084860312156158f8575f80fd5b83356159038161582a565b925060208401356159138161582a565b929592945050506040919091013590565b5f60208284031215615934575f80fd5b5035919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040516080810167ffffffffffffffff8111828210171561598b5761598b61593b565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156159d8576159d861593b565b604052919050565b803562ffffff81168114610c62575f80fd5b8035600281900b8114610c62575f80fd5b5f60a08284031215615a13575f80fd5b60405160a0810167ffffffffffffffff81118282101715615a3657615a3661593b565b6040529050808235615a478161582a565b81526020830135615a578161582a565b6020820152615a68604084016159e0565b6040820152615a79606084016159f2565b60608201526080830135615a8c8161582a565b6080919091015292915050565b5f8083601f840112615aa9575f80fd5b50813567ffffffffffffffff811115615ac0575f80fd5b602083019150836020828501011115615ad7575f80fd5b9250929050565b5f805f805f6101008688031215615af3575f80fd5b615afd8787615a03565b945060a0860135935060c0860135925060e086013567ffffffffffffffff811115615b26575f80fd5b615b3288828901615a99565b969995985093965092949392505050565b5f60208284031215615b53575f80fd5b81356108388161582a565b5f8060408385031215615b6f575f80fd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b81811015615bb5578351835260209384019390920191600101615b97565b509095945050505050565b5f8060208385031215615bd1575f80fd5b823567ffffffffffffffff811115615be7575f80fd5b615bf385828601615a99565b90969095509350505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f8060c08385031215615c63575f80fd5b615c6d8484615a03565b9150615c7b60a084016159e0565b90509250929050565b80358015158114610c62575f80fd5b5f8060408385031215615ca4575f80fd5b8235615caf8161582a565b9150615c7b60208401615c84565b5f805f80848603610140811215615cd2575f80fd5b615cdc8787615a03565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215615d0d575f80fd5b50615d16615968565b615d2260a087016159f2565b8152615d3060c087016159f2565b602082015260e086013560408201526101008601356060820152925061012085013567ffffffffffffffff811115615d66575f80fd5b615d7287828801615a99565b95989497509550505050565b5f8060c08385031215615d8f575f80fd5b615d998484615a03565b915060a0830135615da98161582a565b809150509250929050565b5f8060208385031215615dc5575f80fd5b823567ffffffffffffffff811115615ddb575f80fd5b8301601f81018513615deb575f80fd5b803567ffffffffffffffff811115615e01575f80fd5b8560208260051b8401011115615e15575f80fd5b6020919091019590945092505050565b5f8060408385031215615e36575f80fd5b8235615e418161582a565b91506020830135615da98161582a565b5f805f80848603610120811215615e66575f80fd5b615e708787615a03565b945060607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215615ea1575f80fd5b506040516060810167ffffffffffffffff81118282101715615ec557615ec561593b565b604052615ed460a08701615c84565b815260c0860135602082015260e0860135615eee8161582a565b6040820152925061010085013567ffffffffffffffff811115615d66575f80fd5b5f805f8060808587031215615f22575f80fd5b8435615f2d8161582a565b93506020850135615f3d8161582a565b93969395505050506040820135916060013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8181038181111561076557610765615f52565b8082018082111561076557610765615f52565b81835281816020850137505f602082840101525f60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b602081525f611d01602083018486615fa5565b5f6020828403121561600f575f80fd5b815167ffffffffffffffff811115616025575f80fd5b8201601f81018413616035575f80fd5b805167ffffffffffffffff81111561604f5761604f61593b565b61608060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601615991565b818152856020838501011115616094575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b73ffffffffffffffffffffffffffffffffffffffff8716815261614c602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8460c08201528360e08201526101206101008201525f61617161012083018486615fa5565b98975050505050505050565b73ffffffffffffffffffffffffffffffffffffffff86168152616218602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351600290810b60c08301526020850151900b60e0820152604084015161010082015260608401516101208201526101606101408201525f614fdf61016083018486615fa5565b73ffffffffffffffffffffffffffffffffffffffff881681526162fa602082018873ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8551600290810b60c08301526020870151900b60e08201526040860151610100820152606086015161012082015284610140820152836101608201526101a06101808201525f6144a06101a083018486615fa5565b73ffffffffffffffffffffffffffffffffffffffff8416815260e081016163ee602083018573ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff831660c0830152949350505050565b73ffffffffffffffffffffffffffffffffffffffff8516815261010081016164b2602083018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff841660c08301528260020b60e083015295945050505050565b5f602082840312156164f0575f80fd5b5051919050565b73ffffffffffffffffffffffffffffffffffffffff86168152616592602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351151560c0820152602084015160e0820152604084015173ffffffffffffffffffffffffffffffffffffffff166101008201526101406101208201525f614fdf61014083018486615fa5565b8082018281125f8312801582168215821617156165fe576165fe615f52565b505092915050565b73ffffffffffffffffffffffffffffffffffffffff871681526166a1602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8451151560c0820152602085015160e0820152604085015173ffffffffffffffffffffffffffffffffffffffff16610100820152836101208201526101606101408201525f61617161016083018486615fa5565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81137fffffffffffffffffffffffffffffffff800000000000000000000000000000008212171561076557610765615f52565b600f82810b9082900b037fffffffffffffffffffffffffffffffff8000000000000000000000000000000081126f7fffffffffffffffffffffffffffffff8213171561076557610765615f52565b8181035f83128015838313168383128216171561398f5761398f615f52565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82616810577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b50049056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15`\x0EW_\x80\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P0`\x80R`\x80Qah\"a\0k_9_a Q\x01Rah\"_\xF3\xFE`\x80`@R`\x046\x10a\x01\xF4W_5`\xE0\x1C\x80cZk\xCF\xDA\x11a\x01\x17W\x80c\xA5\x84\x11\x94\x11a\0\xACW\x80c\xF15\xBA\xAA\x11a\0|W\x80c\xF3\xCD\x91L\x11a\0bW\x80c\xF3\xCD\x91L\x14a\x06vW\x80c\xF5)\x8A\xCA\x14a\x06\x95W\x80c\xFE\x99\x04\x9A\x14a\x06\xB4W_\x80\xFD[\x80c\xF15\xBA\xAA\x14a\x068W\x80c\xF2\xFD\xE3\x8B\x14a\x06WW_\x80\xFD[\x80c\xA5\x84\x11\x94\x14a\x05\x95W\x80c\xB66<\xF2\x14a\x05\xB4W\x80c\xDB\xD05\xFF\x14a\x05\xEDW\x80c\xF0-\xE3\xB2\x14a\x06\x0CW_\x80\xFD[\x80c\x81a\xB8t\x11a\0\xE7W\x80c\x81a\xB8t\x14a\x04\xDCW\x80c\x8D\xA5\xCB[\x14a\x04\xFBW\x80c\x97\xE8\xCDN\x14a\x05KW\x80c\x9B\xF6d_\x14a\x05vW_\x80\xFD[\x80cZk\xCF\xDA\x14a\x048W\x80cbv\xCB\xBE\x14a\x04lW\x80c~\x87\xCE}\x14a\x04\x9EW\x80c\x80\xF0\xB4L\x14a\x04\xBDW_\x80\xFD[\x80c-w\x13\x89\x11a\x01\x8DW\x80cH\xC8\x94\x91\x11a\x01]W\x80cH\xC8\x94\x91\x14a\x03\x92W\x80cRu\x96Q\x14a\x03\xBEW\x80cU\x8Ar\x97\x14a\x03\xDDW\x80cY\x8A\xF9\xE7\x14a\x03\xFCW_\x80\xFD[\x80c-w\x13\x89\x14a\x03\x15W\x80c5\xFDc\x1A\x14a\x034W\x80c=\xD4Z\xDB\x14a\x03`W\x80cBj\x84\x93\x14a\x03sW_\x80\xFD[\x80c\x11\xDA`\xB4\x11a\x01\xC8W\x80c\x11\xDA`\xB4\x14a\x02\xB0W\x80c\x15n)\xF6\x14a\x02\xB8W\x80c\x1E.\xAE\xAF\x14a\x02\xD7W\x80c#Bf\xD7\x14a\x02\xF6W_\x80\xFD[\x80b\xFD\xD5\x8E\x14a\x01\xF8W\x80c\x01\xFF\xC9\xA7\x14a\x02AW\x80c\t[\xCD\xB6\x14a\x02pW\x80c\x0B\r\x9C\t\x14a\x02\x8FW[_\x80\xFD[4\x80\x15a\x02\x03W_\x80\xFD[Pa\x02.a\x02\x126`\x04aXKV[`\x04` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02LW_\x80\xFD[Pa\x02`a\x02[6`\x04aXuV[a\x06\xD3V[`@Q\x90\x15\x15\x81R` \x01a\x028V[4\x80\x15a\x02{W_\x80\xFD[Pa\x02`a\x02\x8A6`\x04aX\xB4V[a\x07kV[4\x80\x15a\x02\x9AW_\x80\xFD[Pa\x02\xAEa\x02\xA96`\x04aX\xE6V[a\x08?V[\0[a\x02.a\x08\xC9V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02\xAEa\x02\xD26`\x04aX\xB4V[a\t'V[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x02.a\x02\xF16`\x04aY$V[a\t\xABV[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x02.a\x03\x106`\x04aZ\xDEV[a\t\xB5V[4\x80\x15a\x03 W_\x80\xFD[Pa\x02\xAEa\x03/6`\x04a[CV[a\n\xD9V[4\x80\x15a\x03?W_\x80\xFD[Pa\x03Sa\x03N6`\x04a[^V[a\x0B\xCCV[`@Qa\x028\x91\x90a[~V[a\x02.a\x03n6`\x04a[CV[a\x0C\tV[4\x80\x15a\x03~W_\x80\xFD[Pa\x02`a\x03\x8D6`\x04aX\xB4V[a\x0CgV[4\x80\x15a\x03\x9DW_\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04a[\xC0V[a\x0C\xD8V[`@Qa\x028\x91\x90a[\xFFV[4\x80\x15a\x03\xC9W_\x80\xFD[Pa\x02\xAEa\x03\xD86`\x04a\\RV[a\x0E*V[4\x80\x15a\x03\xE8W_\x80\xFD[Pa\x02`a\x03\xF76`\x04a\\\x93V[a\x0E\xCCV[4\x80\x15a\x04\x07W_\x80\xFD[Pa\x02.a\x04\x166`\x04aX\xE6V[`\x05` \x90\x81R_\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T\x81V[4\x80\x15a\x04CW_\x80\xFD[Pa\x04Wa\x04R6`\x04a\\\xBDV[a\x0FfV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x028V[4\x80\x15a\x04wW_\x80\xFD[Pa\x04\x8Ba\x04\x866`\x04a]~V[a\x11eV[`@Q`\x02\x91\x90\x91\x0B\x81R` \x01a\x028V[4\x80\x15a\x04\xA9W_\x80\xFD[Pa\x02\xAEa\x04\xB86`\x04a\\RV[a\x13\xFDV[4\x80\x15a\x04\xC8W_\x80\xFD[Pa\x02\xAEa\x04\xD76`\x04aXKV[a\x14\xEEV[4\x80\x15a\x04\xE7W_\x80\xFD[Pa\x02.a\x04\xF66`\x04aX\xE6V[a\x15\xADV[4\x80\x15a\x05\x06W_\x80\xFD[P_Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x028V[4\x80\x15a\x05VW_\x80\xFD[Pa\x02.a\x05e6`\x04a[CV[`\x01` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\x81W_\x80\xFD[Pa\x03Sa\x05\x906`\x04a]\xB4V[a\x16\xD9V[4\x80\x15a\x05\xA0W_\x80\xFD[Pa\x02\xAEa\x05\xAF6`\x04a[CV[a\x17\x12V[4\x80\x15a\x05\xBFW_\x80\xFD[Pa\x02`a\x05\xCE6`\x04a^%V[`\x03` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xF8W_\x80\xFD[Pa\x03Sa\x06\x076`\x04a]\xB4V[a\x17\xB7V[4\x80\x15a\x06\x17W_\x80\xFD[P`\x02Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x06CW_\x80\xFD[Pa\x02.a\x06R6`\x04aY$V[a\x17\xEEV[4\x80\x15a\x06bW_\x80\xFD[Pa\x02\xAEa\x06q6`\x04a[CV[a\x17\xF8V[4\x80\x15a\x06\x81W_\x80\xFD[Pa\x02.a\x06\x906`\x04a^QV[a\x18\xE7V[4\x80\x15a\x06\xA0W_\x80\xFD[Pa\x02\xAEa\x06\xAF6`\x04aX\xB4V[a\x1A\x99V[4\x80\x15a\x06\xBFW_\x80\xFD[Pa\x02`a\x06\xCE6`\x04a_\x0FV[a\x1B\x1DV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14\x80a\x07eWP\x7F\x0Fc/\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x91\x90\x83\x90a\x07\x96\x90\x84\x90a_\x7FV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x07\xDA\x90\x84\x90a_\x92V[\x90\x91UPP`@\x80Q3\x80\x82R` \x82\x01\x85\x90R\x85\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x92\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4P`\x01[\x93\x92PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x08\x8EWa\x08\x8E\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x08\xA3\x83a\x08\x9B\x83a\x1D\x11V[_\x033a\x1DVV[a\x08\xC4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x1D\xB6V[PPPV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\t\x19Wa\t\x19\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\t\"3a\x1E\xB1V[\x90P\x90V[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\tvWa\tv\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x81a\t\x84\x81a\x08\x9B\x84a\x1D\x11V[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a\x1F\x9AV[PPPPV[_\x81T_R` _\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\n\x05Wa\n\x05\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\n\ra 9V[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\n'\x81a \xA1V[`\x80\x88\x01Qa\nP\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a \xE8V[a\n[\x81\x88\x88a!\xCBV[\x92Pa\nh\x88\x843a\"\xC5V[`@\x80Q\x88\x81R` \x81\x01\x88\x90R3\x91\x84\x91\x7F)\xEF\x05\xCA\xAF\xF9@K|\xB6\xD1\xC0\xE9\xBB\xAE\x9E\xAAz\xB2T\x1F\xEB\xA1\xA9\xC4$\x85\x94\xC0\x81V\xCB\x91\x01`@Q\x80\x91\x03\x90\xA3`\x80\x88\x01Qa\n\xCE\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a\"\xEDV[PP\x95\x94PPPPPV[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0B^W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB4\xBD\x8E\xF5=\xF6\x90\xB9\x94=3\x18\x99`\x06\xDB\xB8*%\xF5G\x19\xD8\xC8\x03[Qj*[\x8A\xCC\x90_\x90\xA2PV[```@Q\x80\x83`\x05\x1B` \x83R\x84` \x84\x01R`@\x83\x01\x92P\x80\x83\x01\x90P[\x85T\x83R` \x83\x01\x92P`\x01\x86\x01\x95P\x80\x83\x10a\x0B\xECW\x81\x81\x03\x82\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0CYWa\x0CY\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x07e\x82a\x1E\xB1V[\x91\x90PV[3_\x81\x81R`\x05` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x80\x85R\x90\x83R\x81\x84 \x87\x85R\x90\x92R\x80\x83 \x85\x90UQ\x91\x92\x85\x92\x7F\xB3\xFDPq\x83X\x87Vz\x06q\x15\x11!\x89M\xDC\xCC(B\xF1\xD1\x0B\xED\xAD\x13\xE0\xD1|\xAC\xE9\xA7\x90a\x08,\x90\x87\x81R` \x01\x90V[``\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\r*Wa\r*\x7FP\x90\xD6\xC6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\r2a#\xC5V[`@Q\x7F\x91\xDDsF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90c\x91\xDDsF\x90a\rp\x90\x86\x90\x86\x90`\x04\x01a_\xECV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\r\xD0\x91\x90\x81\x01\x90a_\xFFV[\x90P\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\\x15a\x0E\"Wa\x0E\"\x7FR\x12\xCB\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x07ea#\xEBV[`@\x82\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x80a\x0EuWP\x81`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x0E\xA3Wa\x0E\xA3\x7F0\xD2\x16A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x0E\xB1\x81b\xFF\xFF\xFF\x16a$\x10V[`\xA0\x82 _\x81\x81R`\x06` R`@\x90 a\x08\xC4\x90\x83a$OV[3_\x81\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x85R\x90\x83R\x81\x84 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x87\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x92\x93\x91\x7F\xCE\xB5v\xD9\xF1^N \x0F\xDBP\x96\xD6M]\xFDf~\x16\xDE\xF2\x0C\x1E\xEF\xD1BV\xD8\xE3\xFA\xA2g\x91\x01`@Q\x80\x91\x03\x90\xA3P`\x01\x92\x91PPV[_\x80\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0F\xB7Wa\x0F\xB7\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x0F\xBFa 9V[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\x0F\xD9\x81a \xA1V[`\x80\x88\x01Qa\x10\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89a$\xA8V[_a\x10u`@Q\x80`\xC0\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A_\x01Q`\x02\x0B\x81R` \x01\x8A` \x01Q`\x02\x0B\x81R` \x01a\x10R\x8B`@\x01Qa&iV[`\x0F\x0B\x81R``\x80\x8D\x01Q`\x02\x0B` \x83\x01R\x8B\x01Q`@\x90\x91\x01R\x83\x90a&\x9EV[\x94P\x90Pa\x10\x83\x81\x85a*\xE3V[\x94PPP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xF2\x08\xF4\x91'\x82\xFD%\xC7\xF1\x14\xCA7#\xA2\xD5\xDDo;\xCC:\xC8\xDBZ\xF6;\xAA\x85\xF7\x11\xD5\xEC\x88_\x01Q\x89` \x01Q\x8A`@\x01Q\x8B``\x01Q`@Qa\x11\x01\x94\x93\x92\x91\x90`\x02\x94\x85\x0B\x81R\x92\x90\x93\x0B` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x80\x87\x01Q_\x90a\x115\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x87\x87\x8B\x8Ba+\x16V[\x90\x94P\x90P\x80\x15a\x11OWa\x11O\x88\x82\x8A`\x80\x01Qa\"\xC5V[a\x11Z\x88\x853a\"\xC5V[PP\x94P\x94\x92PPPV[_a\x11na 9V[``\x83\x01Qa\x7F\xFF`\x02\x91\x90\x91\x0B\x13\x15a\x11\xB2W``\x83\x01Qa\x11\xB2\x90\x7F\xB7\0$\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a,\xEDV[`\x01`\x02\x0B\x83``\x01Q`\x02\x0B\x12\x15a\x11\xF5W``\x83\x01Qa\x11\xF5\x90\x7F\xE9\xE9\x05\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a,\xEDV[\x82Q` \x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x10a\x12MW\x82Q` \x84\x01Qa\x12M\x91\x7Fnl\x980\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a,\xFCV[a\x12~\x83`@\x01Q\x84`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a-?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x12\xB2W`\x80\x83\x01Qa\x12\xB2\x90\x7F\xE6Z\xF6\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\rV[_a\x12\xC5\x84`@\x01Qb\xFF\xFF\xFF\x16a./V[`\x80\x85\x01Q\x90\x91Pa\x12\xEE\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85a.TV[`\xA0\x84 _\x81\x81R`\x06` R`@\x90 a\x13\n\x90\x85\x84a/'V[`\x80\x86\x01Q\x90\x93Pa\x134\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x86\x86a/\xDFV[\x84` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\xDDFngN\xA5W\xF5b\x95\xE2\xD0!\x8A\x12^\xA4\xB4\xF0\xF6\xF30{\x95\xF8^a\x10\x83\x8Dd8\x88`@\x01Q\x89``\x01Q\x8A`\x80\x01Q\x8A\x8A`@Qa\x13\xED\x95\x94\x93\x92\x91\x90b\xFF\xFF\xFF\x95\x90\x95\x16\x85R`\x02\x93\x84\x0B` \x86\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`@\x86\x01R\x91\x16``\x84\x01R\x90\x0B`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA4PP\x92\x91PPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x14EWa\x14E\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x03\xE9a\x0F\xFF\x82\x16\x10b>\x90\0b\xFF\xF0\0\x83\x16\x10\x16a\x14\x8DWa\x14\x8D\x7F\xA7\xAB\xE2\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\rV[`\xA0\x82 a\x14\xAF\x82a\x14\xA9\x83_\x90\x81R`\x06` R`@\x90 \x90V[\x90a0\xB4V[`@Qb\xFF\xFF\xFF\x83\x16\x81R\x81\x90\x7F\xE9\xC4%\x93\xE7\x1F\x84@;\x845,\xD1h\xD6\x93\xE2\xC9\xFC\xD1\xFD\xBC\xC3\xFE\xB2\x1D\x92\xB4>f\x96\xF9\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x15=Wa\x15=\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[3_\x90\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` R`@\x81 \\\x90a\x15k\x83a\x1D\x11V[\x90P\x81\x81`\x0F\x0B\x14a\x15\xA0Wa\x15\xA0\x7F\xBD\xA7:\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\t\xA5\x84\x82_\x033a\x1DVV[`\x02T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x15\xF7Wa\x15\xF7\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\x16GWa\x16G\x7F>_O\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x81\x15a\x16SW\x81a\x16yV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x01` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x16\xB2\x90\x84\x90a_\x7FV[\x90\x91UPa\x088\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x85\x83a\x1D\xB6V[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805\\\x84R` \x93\x84\x01\x93\x01\x81\x84\x10a\x16\xF7W[P\x81\x81\x03\x82\xF3[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x17aWa\x17a\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x17\x87Wa\x17\x84a1\x08V[PV[_a\x17\xA7\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1-V[\x90Pa\x17\xB3\x82\x82a1\xDCV[PPV[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805T\x84R` \x93\x84\x01\x93\x01\x81\x84\x10\x15a\x17\x0BWa\x17\xD5V[_\x81\\_R` _\xF3[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x18xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0BUV[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x197Wa\x197\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x19?a 9V[\x83` \x01Q_\x03a\x19sWa\x19s\x7F\xBE\x8B\x85\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[`\xA0\x85 _\x81\x81R`\x06` R`@\x90 a\x19\x8D\x81a \xA1V[`\x80\x87\x01Q_\x90\x81\x90\x81\x90a\x19\xBB\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8B\x8B\x8B\x8Ba2<V[\x80\x93P\x81\x95P\x82\x94PPPPa\x1A7\x84\x86`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x8E``\x01Q`\x02\x0B\x81R` \x01\x8D_\x01Q\x15\x15\x81R` \x01\x8D`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85b\xFF\xFF\xFF\x16\x81RP\x8C_\x01Qa\x1A0W\x8D` \x01Qa3\xE3V[\x8DQa3\xE3V[`\x80\x8B\x01Q\x90\x96P_\x92Pa\x1Ah\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A\x8A\x88\x8B\x8B\x88a4\xE4V[\x90\x95P\x90P\x80\x15a\x1A\x82Wa\x1A\x82\x89\x82\x8B`\x80\x01Qa\"\xC5V[a\x1A\x8D\x89\x863a\"\xC5V[PPPP\x94\x93PPPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x1A\xE8Wa\x1A\xE8\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x81a\x1A\xFC\x81a\x1A\xF6\x84a\x1D\x11V[3a\x1DVV[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a6}V[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x14\x80\x15\x90a\x1BtWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15[\x15a\x1C\x1DWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x86\x84R\x90\x91R\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x1C\x1BWa\x1B\xE2\x83\x82a_\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x88\x84R\x90\x91R\x90 U[P[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1C\\\x90\x84\x90a_\x7FV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1C\xA0\x90\x84\x90a_\x92V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x84\x90R\x84\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x92\x90\x89\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01`@Q\x80\x91\x03\x90\xA4P`\x01[\x94\x93PPPPV[\x80_R`\x04_\xFD[_o\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x1DRWa\x1DR\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[P\x90V[\x81`\x0F\x0B_\x03a\x1DeWPPPV[_\x80a\x1D\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x86a7\x8EV[\x91P\x91P\x80_\x03a\x1D\xA0Wa\x1D\x9Ba7\xD4V[a\x1D\xAFV[\x81_\x03a\x1D\xAFWa\x1D\xAFa8\"V[PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x1E\x10W_\x80_\x80\x85\x87Z\xF1\x90P\x80a\x1E\x0BWa\x1E\x0B\x7F\x85I\xDBY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a8pV[a\t\xA5V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91P_\x81R_` \x82\x01R_`@\x82\x01RP\x80a\t\xA5Wa\t\xA5\x7F\xB1,_\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a8pV[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1E\xF7W4\x91Pa\x1F\x81V[4\x15a\x1F&Wa\x1F&\x7F\xB0\xEC\x84\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95\\_a\x1Fhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a1-V[\x90Pa\x1Ft\x82\x82a_\x7FV[\x93Pa\x1F~a1\x08V[PP[a\x1F\x94\x81a\x1F\x8E\x84a\x1D\x11V[\x85a\x1DVV[P\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x1F\xD9\x90\x84\x90a_\x92V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91_\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a \x9FWa \x9F\x7F\r\x89C\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x17\x84Wa\x17\x84\x7FHj\xA3\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xC2W` \x87\x16\x15a!\xC2Wa!\xC03\x87\x87\x87\x87\x87`@Q`$\x01a!-\x96\x95\x94\x93\x92\x91\x90a`\xB1V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\xA8\xB0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a8\xA3V[P[PPPPPPPV[`\x03\x83\x01T_\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x03a\"\x14Wa\"\x14\x7F\xA7O\x97\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\"Ka\" \x85a\x1D\x11V[_\x03a\"+\x85a\x1D\x11V[_\x03`\x80\x91\x90\x91\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x91P\x83\x15a\"\x85W`\x01\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x02\x04\x01\x90U[\x82\x15a\"\xBDW`\x02\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x02\x04\x01\x90U[P\x93\x92PPPV[\x82Qa\"\xDB\x90a\"\xD5\x84`\x80\x1D\x90V[\x83a\x1DVV[a\x08\xC4\x83` \x01Qa\"\xD5\x84`\x0F\x0B\x90V[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xC2W`\x10\x87\x16\x15a!\xC2Wa!\xC03\x87\x87\x87\x87\x87`@Q`$\x01a#2\x96\x95\x94\x93\x92\x91\x90a`\xB1V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE1\xB4\xAFi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a8\xA3V[`\x01\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[b\x0FB@b\xFF\xFF\xFF\x82\x16\x11\x15a\x17\x84Wa\x17\x84\x7F\x14\0!\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\rV[a$X\x82a \xA1V[\x81T\x7F\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16\x17[\x90\x91UPV[\x843s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a&aW_\x84`@\x01Q\x13\x80\x15a$\xDCWPa\x08\0\x86\x16\x15\x15[\x15a%\x96Wa%\x903\x86\x86\x86\x86`@Q`$\x01a$\xFD\x95\x94\x93\x92\x91\x90aa}V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a8\xA3V[Pa&aV[_\x84`@\x01Q\x13\x15\x80\x15a%\xADWPa\x02\0\x86\x16\x15\x15[\x15a&aWa!\xC23\x86\x86\x86\x86`@Q`$\x01a%\xCE\x95\x94\x93\x92\x91\x90aa}V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a8\xA3V[PPPPPPV[\x80`\x0F\x81\x90\x0B\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[``\x81\x01Q` \x82\x01Q`@\x83\x01Q_\x92\x83\x92\x90\x91a&\xBD\x82\x82a9\x96V[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x83`\x0F\x0B_\x14a(\x8DWa&\xF6\x88\x84\x86_a:]V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01R\x15\x15\x81Ra'\x1E\x88\x83\x86`\x01a:]V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x83\x01R\x15\x15`@\x82\x01R_`\x0F\x85\x90\x0B\x12a(RW`\x80\x87\x01Q_\x90`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x81\x81\x07\x83\x13\x90\x82\x90\x05\x03b\r\x89\xE8\x91\x90\x91\x05\x03`\x01\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a'\xF6Wa'\xF6\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a,\xEDV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82``\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a(PWa(P\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a,\xEDV[P[\x80Q\x15a(nW`\x80\x87\x01Qa(n\x90`\x05\x8A\x01\x90\x85\x90a;FV[\x80`@\x01Q\x15a(\x8DW`\x80\x87\x01Qa(\x8D\x90`\x05\x8A\x01\x90\x84\x90a;FV[_\x80a(\x9A\x8A\x86\x86a;\x98V[\x8AQ`\xA0\x8C\x01Q`@\x80Q`&\x81\x01\x92\x90\x92R`\x06\x80\x83\x01\x8A\x90R`\x03\x83\x01\x8B\x90R\x92\x82R`:`\x0C\x83\x01 _\x83\x83\x01\x81\x90R` \x80\x85\x01\x82\x90R\x93\x81\x90R\x90\x81R\x92\x8F\x01\x90\x91R\x81 \x92\x94P\x90\x92P\x80a(\xF7\x83\x8A\x87\x87a<LV[\x91P\x91Pa),a)\x07\x83a\x1D\x11V[a)\x10\x83a\x1D\x11V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x99PPPPPP_\x84`\x0F\x0B\x12\x15a)\x95W\x80Q\x15a)hW`\x02\x83\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[\x80`@\x01Q\x15a)\x95W`\x02\x82\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[P\x82`\x0F\x0B_\x14a*\xD9W\x86T_\x80a)\xB1\x83`\xA0\x1C`\x02\x0B\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x91P\x91P\x84`\x02\x0B\x82`\x02\x0B\x12\x15a*\rWa*\x06a*\0a)\xFBa)\xEC\x88a=}V[a)\xF5\x88a=}V[\x8Aa@zV[a&iV[`\x80\x1B\x90V[\x97Pa*\xD5V[\x83`\x02\x0B\x82`\x02\x0B\x12\x15a*\xB0Wa*Da*.a)\xFB\x83a)\xF5\x88a=}V[a)\x10a)\xFBa*=\x89a=}V[\x85\x8Ba@\xB2V[`\x03\x8B\x01T\x90\x98Pa*h\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87a@\xDEV[`\x03\x8B\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua*\xD5V[a*\xD2_a)\x10a)\xFBa*\xC3\x89a=}V[a*\xCC\x89a=}V[\x8Ba@\xB2V[\x97P[PPP[PPP\x92P\x92\x90PV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a+\ra+\x04\x83a&iV[a)\x10\x83a&iV[\x95\x94PPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a+?WP\x84\x90P_a,\xE1V[\x85\x91P_\x87`@\x01Q\x13\x15a,'Wa\x04\0\x89\x16\x15a,\"Wa,\x133\x89\x89\x89\x89\x89\x89`@Q`$\x01a+x\x97\x96\x95\x94\x93\x92\x91\x90ab_V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x9F\x06>\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x02\x8B\x16\x15\x15[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x91\x90aA\x0EV[\x90Pa,\x1F\x82\x82aAhV[\x91P[a,\xE1V[a\x01\0\x89\x16\x15a,\xE1Wa,\xD23\x89\x89\x89\x89\x89\x89`@Q`$\x01a,Q\x97\x96\x95\x94\x93\x92\x91\x90ab_V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fl+\xBE~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x01\x8B\x16\x15\x15a+\xF5V[\x90Pa,\xDE\x82\x82aAhV[\x91P[\x97P\x97\x95PPPPPPV[\x81_R\x80`\x02\x0B`\x04R`$_\xFD[`@Q\x83\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x81\xFD[_`\x80\x83\x16\x15\x80\x15a-SWP`\x08\x83\x16\x15\x15[\x15a-_WP_a\x07eV[`@\x83\x16\x15\x80\x15a-rWP`\x04\x83\x16\x15\x15[\x15a-~WP_a\x07eV[a\x04\0\x83\x16\x15\x80\x15a-\x92WP`\x02\x83\x16\x15\x15[\x15a-\x9EWP_a\x07eV[a\x01\0\x83\x16\x15\x80\x15a-\xB2WP`\x01\x83\x16\x15\x15[\x15a-\xBEWP_a\x07eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15a-\xFCWa?\xFF\x83\x16\x15\x15\x80a-\xF7WPb\x80\0\0b\xFF\xFF\xFF\x83\x16\x14a\x088V[a\x088V[Pb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x91\x90PV[\x81_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x04R`$_\xFD[_b\x80\0\0b\xFF\xFF\xFF\x83\x16\x03a.FWP_\x91\x90PV[a\x1DR\x82b\xFF\xFF\xFF\x16a$\x10V[\x823s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a\t\xA5Wa \0\x84\x16\x15a\t\xA5Wa\x1D\xAF3\x84\x84`@Q`$\x01a.\x94\x93\x92\x91\x90acOV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDC\x985N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90a8\xA3V[\x82T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a/oWa/o\x7Fy\x83\xC0Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a/x\x83aA\x89V[\x90P|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\xA0\x83\x90\x1Bv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x17\x90\x93UP\x90\x91\x90PV[\x833s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a\x1D\xAFWa\x10\0\x85\x16\x15a\x1D\xAFWa&a3\x85\x85\x85`@Q`$\x01a0!\x94\x93\x92\x91\x90ad\x12V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fo\xE7\xE6\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90a8\xA3V[a0\xBD\x82a \xA1V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16y\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xB8\x83\x90\x1B\x16\x17a$\xA2V[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a1PWPG\x91\x90PV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xB8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07e\x91\x90ad\xE0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]\x80\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95]PPV[` \x83\x01Q_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x163\x03a2hW_\x91Pa3\xD8V[`\x80\x88\x16\x15a3\xD8W_a3\n\x893\x8A\x8A\x8A\x8A`@Q`$\x01a2\x8F\x95\x94\x93\x92\x91\x90ad\xF7V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FW^$\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra8\xA3V[\x90P\x80Q``\x14a3>Wa3>\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[`@\x88\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x03a3YW``\x81\x01Q\x91P[`\x08\x89\x16\x15a3\xD6W`@\x81\x01Q\x92P_a3t\x84`\x80\x1D\x90V[\x90P\x80`\x0F\x0B_\x14a3\xD4W_\x85\x12a3\x91`\x0F\x83\x90\x0B\x87ae\xDFV[\x95P\x80a3\xA0W_\x86\x12a3\xA4V[_\x86\x13[\x15a3\xD2Wa3\xD2\x7F\xFA\x0Bq\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[P[P[P[\x95P\x95P\x95\x92PPPV[_\x80\x80\x80\x80a3\xF2\x89\x88aD\xADV[\x93P\x93P\x93P\x93P_\x83\x11\x15a4-Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16_\x90\x81R`\x01` R`@\x90 \x80T\x84\x01\x90U[3\x88\x7F@\xE9\xCE\xCB\x9F_\x1F\x1C[\x9C\x97\xDE\xC2\x91{~\xE9.W\xBAUcp\x8D\xAC\xA9M\xD8J\xD7\x11/a4Z\x87`\x80\x1D\x90V[a4d\x88`\x0F\x0B\x90V[\x85Q`@\x80\x88\x01Q` \x80\x8A\x01Q\x83Q`\x0F\x97\x88\x0B\x81R\x95\x90\x96\x0B\x90\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x90\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x02\x91\x90\x91\x0B`\x80\x82\x01Rb\xFF\xFF\xFF\x86\x16`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90\xA3P\x91\x97\x96PPPPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a5\rWP\x84\x90P_a,\xE1V[_a5\x18\x84`\x80\x1D\x90V[\x90P_a5%\x85`\x0F\x0B\x90V[\x90P`@\x8B\x16\x15a5\xF8Wa5\xEBa)\xFB3\x8C\x8C\x8C\x8C\x8C`@Q`$\x01a5Q\x96\x95\x94\x93\x92\x91\x90af\x06V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB4{/\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x04\x8E\x16\x15\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x16\x91\x90aA\x0EV[a5\xF5\x90\x82af\xF5V[\x90P[_\x81`\x0F\x0B_\x14\x15\x80a6\x0EWP\x82`\x0F\x0B_\x14\x15[\x15a6kW\x89Q` \x8B\x01Q_\x13\x90\x15\x15\x14a6BWo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x90\x1B\x17a6\\V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x80\x84\x90\x1B\x17[\x90Pa6h\x89\x82aAhV[\x98P[\x97\x9B\x97\x9AP\x96\x98PPPPPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x14\x80\x15\x90a6\xD6WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T`\xFF\x16\x15[\x15a7\x83Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x86\x83R\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a7\x81Wa7F\x83\x82a_\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x81R\x82\x82 \x88\x83R\x90R U[P[a\t\xA5\x84\x84\x84aM?V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16_\x90\x81R\x90\x84\x16` R`@\x81 \x80\\\x91\x90a7\xC6`\x0F\x85\x90\x0B\x84ae\xDFV[\x91P\x81\x81]P\x93P\x93\x91PPV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x03\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x01\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[=`@Q\x83\x81R\x82`\x04\x82\x01R`@`$\x82\x01R\x81`D\x82\x01R\x81_`d\x83\x01>` \x80`\x1F\x84\x01\x04\x02`d\x01\x91P\x81\x81\xFD[``_\x80_\x84Q` \x86\x01_\x88Z\xF1\x90P\x80a8\xE3Wa8\xE3\x7F1\x9DT\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a8pV[`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>` \x82Q\x10\x80a9aWP` \x83\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a9<\x83` \x01Q\x90V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15[\x15a9\x8FWa9\x8F\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[P\x92\x91PPV[\x80`\x02\x0B\x82`\x02\x0B\x12a9\xCEWa9\xCE\x7F\xC4C>\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x83aM\xD5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18`\x02\x83\x90\x0B\x12\x15a:$Wa:$\x7F\xD5\xE2\xF7\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a,\xEDV[b\r\x89\xE8`\x02\x82\x90\x0B\x13\x15a\x17\xB3Wa\x17\xB3\x7F\x1A\xD7w\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a,\xEDV[`\x02\x83\x90\x0B_\x90\x81R`\x04\x85\x01` R`@\x81 \x80T\x82\x91\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0Ba:\xAB\x82\x88a@\xDEV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x15\x90\x84\x16\x80\x15\x91\x90\x91\x14\x15\x96P\x90\x94P_\x03a:\xFEW\x88T`\xA0\x1C`\x02\x0B`\x02\x0B\x88`\x02\x0B\x13a:\xFEW`\x01\x80\x8A\x01T\x90\x84\x01U`\x02\x80\x8A\x01T\x90\x84\x01U[_\x86a;\x13Wa;\x0E\x88\x83af\xF5V[a;\x1DV[a;\x1D\x88\x83agCV[\x90P\x80`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x17\x84UPPPP\x94P\x94\x92PPPV[`\x02\x91\x82\x0B\x91\x0B\x80\x82\x07\x15a;sW`@Qc\xD4\xD8\xF3\xE6\x81R\x82` \x82\x01R\x81`@\x82\x01R`D`\x1C\x82\x01\xFD[\x80\x82\x05\x91P\x81`\x08\x1D_R\x82` R`@_ `\x01`\xFF\x84\x16\x1B\x81T\x18\x81UPPPPV[`\x02\x82\x81\x0B_\x81\x81R`\x04\x86\x01` R`@\x80\x82 \x85\x85\x0B\x83R\x90\x82 \x87T\x92\x94\x85\x94\x92\x93\x91\x92`\xA0\x92\x90\x92\x1C\x90\x0B\x90\x81\x12\x15a;\xEEW\x81`\x01\x01T\x83`\x01\x01T\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x03\x93Pa<AV[\x85`\x02\x0B\x81`\x02\x0B\x12a<\x1AW\x82`\x01\x01T\x82`\x01\x01T\x03\x94P\x82`\x02\x01T\x82`\x02\x01T\x03\x93Pa<AV[\x81`\x01\x01T\x83`\x01\x01T\x89`\x01\x01T\x03\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x89`\x02\x01T\x03\x03\x93P[PPP\x93P\x93\x91PPV[\x83T_\x90\x81\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0F\x86\x90\x0B\x82\x03a<\xB6W\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a<\xB1Wa<\xB1\x7F\xAE\xFE\xB9$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a<\xFDV[a<\xC0\x81\x87a@\xDEV[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x87U[a=1\x87`\x01\x01T\x86\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aM\xF2V[\x92Pa=g\x87`\x02\x01T\x85\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aM\xF2V[`\x01\x88\x01\x95\x90\x95UPP`\x02\x90\x94\x01U\x91\x92\x90PV[`\x02\x0B_`\xFF\x82\x90\x1D\x80\x83\x01\x18b\r\x89\xE8\x81\x11\x15a=\xBFWa=\xBF\x7F\x8B\x862z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a,\xEDV[p\x01\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01`\x01\x82\x16\x02p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18`\x02\x82\x16\x15a>\x08Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a>'Wo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a>FWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a>eWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a>\x84Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a>\xA3Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a>\xC2Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a>\xE2Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a?\x02Wo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a?\"Wo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a?BWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a?bWo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a?\x82Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a?\xA2Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a?\xC2Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a?\xE3Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a@\x03Wn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a@\"Wm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a@?Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[_\x84\x13\x15a@jW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04[c\xFF\xFF\xFF\xFF\x01` \x1C\x93\x92PPPV[_\x80\x82`\x0F\x0B\x12a@\xA1Wa@\x9Aa@\x95\x85\x85\x85`\x01aN\xADV[aO\xEAV[_\x03a\x1D\x01V[a\x1D\x01a@\x95\x85\x85\x85_\x03_aN\xADV[_\x80\x82`\x0F\x0B\x12a@\xCDWa@\x9Aa@\x95\x85\x85\x85`\x01aP\x1CV[a\x1D\x01a@\x95\x85\x85\x85_\x03_aP\x1CV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x0F\x82\x90\x0B\x01`\x80\x81\x90\x1C\x15a\x07eWc\x93\xDA\xFD\xF1_R`\x04`\x1C\xFD[_\x80aA\x1A\x85\x85a8\xA3V[\x90P\x82aA*W_\x91PPa\x088V[\x80Q`@\x14aA\\WaA\\\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[`@\x01Q\x94\x93PPPPV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x03`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x03a+\ra+\x04\x83a&iV[_s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DQc\x96\x16\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xFF\xFD\x89]\x83\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aB\x08WaB\x08\x7FaHu$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a.\rV[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0` \x83\x90\x1B\x16\x80_aB2\x82aP\x87V[`\xFF\x16\x90P`\x80\x81\x10aBMW`\x7F\x81\x03\x83\x90\x1C\x91PaBWV[\x80`\x7F\x03\x83\x90\x1B\x91P[\x90\x80\x02`\x7F\x81\x81\x1C`\xFF\x83\x81\x1C\x91\x90\x91\x1C\x80\x02\x80\x83\x1C\x81\x83\x1C\x1C\x80\x02\x80\x84\x1C\x81\x84\x1C\x1C\x80\x02\x80\x85\x1C\x81\x85\x1C\x1C\x80\x02\x80\x86\x1C\x81\x86\x1C\x1C\x80\x02\x80\x87\x1C\x81\x87\x1C\x1C\x80\x02\x80\x88\x1C\x81\x88\x1C\x1C\x80\x02\x80\x89\x1C\x81\x89\x1C\x1C\x80\x02\x80\x8A\x1C\x81\x8A\x1C\x1C\x80\x02\x80\x8B\x1C\x81\x8B\x1C\x1C\x80\x02\x80\x8C\x1C\x81\x8C\x1C\x1C\x80\x02\x80\x8D\x1C\x81\x8D\x1C\x1C\x80\x02\x80\x8E\x1C\x9C\x81\x90\x1C\x9C\x90\x9C\x1C\x80\x02\x9C\x8D\x90\x1C\x9E\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8F\x01`@\x1B`\xC0\x91\x90\x91\x1Cg\x80\0\0\0\0\0\0\0\x16\x17`\xC1\x9B\x90\x9B\x1Cg@\0\0\0\0\0\0\0\x16\x9A\x90\x9A\x17`\xC2\x99\x90\x99\x1Cg \0\0\0\0\0\0\0\x16\x98\x90\x98\x17`\xC3\x97\x90\x97\x1Cg\x10\0\0\0\0\0\0\0\x16\x96\x90\x96\x17`\xC4\x95\x90\x95\x1Cg\x08\0\0\0\0\0\0\0\x16\x94\x90\x94\x17`\xC5\x93\x90\x93\x1Cg\x04\0\0\0\0\0\0\0\x16\x92\x90\x92\x17`\xC6\x91\x90\x91\x1Cg\x02\0\0\0\0\0\0\0\x16\x17`\xC7\x91\x90\x91\x1Cg\x01\0\0\0\0\0\0\0\x16\x17`\xC8\x91\x90\x91\x1Cf\x80\0\0\0\0\0\0\x16\x17`\xC9\x91\x90\x91\x1Cf@\0\0\0\0\0\0\x16\x17`\xCA\x91\x90\x91\x1Cf \0\0\0\0\0\0\x16\x17`\xCB\x91\x90\x91\x1Cf\x10\0\0\0\0\0\0\x16\x17`\xCC\x91\x90\x91\x1Cf\x08\0\0\0\0\0\0\x16\x17`\xCD\x91\x90\x91\x1Cf\x04\0\0\0\0\0\0\x16\x17i6'\xA3\x01\xD7\x10UwL\x85\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFDp\x9B~T\x80\xFB\xA5\xA5\x0F\xED^b\xFF\xC5V\x81\x01`\x80\x90\x81\x1D\x90o\xDB-\xF0\x9E\x81\x95\x9A\x81E^&\x07\x99\xA0c/\x83\x01\x90\x1D`\x02\x81\x81\x0B\x90\x83\x90\x0B\x14aD\x9EW\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aDv\x82a=}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aD\x98W\x81aD\xA0V[\x80aD\xA0V[\x81[\x99\x98PPPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90\x81\x90\x85T`@\x86\x01Q_\x81aD\xE7Wa\x0F\xFF`\xC4\x84\x90\x1C\x16aD\xF1V[a\x0F\xFF`\xB8\x84\x90\x1C\x16[\x88Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86Ra\xFF\xFF\x91\x90\x91\x16\x91P_`\xA0\x85\x90\x1C`\x02\x0B`\x02\x0B` \x87\x01R`\x03\x8B\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x87\x01R`\x80\x8A\x01Q_\x90b@\0\0\x16aEaW`\xD0\x86\x90\x1Cb\xFF\xFF\xFF\x16aEsV[aEs\x8B`\x80\x01Qb\xFF\xFF\xFF\x16aQ\x1BV[\x90P\x83\x15aE\xA1Wb\x0FB@a\x0F\xFF\x85\x16b\xFF\xFF\xFF\x83\x16\x81\x81\x02\x83\x81\x06\x15\x15\x93\x90\x04\x92\x90\x92\x01\x91\x01\x03aE\xA3V[\x80[\x97PPb\x0FB@\x87b\xFF\xFF\xFF\x16\x10aE\xE7W\x89Q_\x12\x15aE\xE7WaE\xE7\x7F\x96 bF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x89Q_\x03aE\xFFW_\x80\x98P\x98PPPPPPaM6V[\x83\x15aF\xE2W``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x10aFqWaFqs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16[``\x8C\x01Q\x7F|\x9Cn\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90a,\xFCV[d\x01\0\x02v\xA3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aF\xDDW``\x8A\x01QaF\xDD\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\rV[aG\xA0V[``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x11aG%WaG%s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16aFDV[s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10aG\xA0W``\x8A\x01QaG\xA0\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\rV[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x84aG\xEFW\x8B`\x02\x01TaG\xF5V[\x8B`\x01\x01T[`\xE0\x82\x01R[\x82\x15\x80aH:WP\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[aK\xC2W\x86Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x88\x01Q\x90\x8C\x01QaHq\x91`\x05\x8F\x01\x91\x88aQ*V[\x15\x15`@\x83\x01R`\x02\x0B` \x82\x01\x81\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x12aH\xCFW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18` \x82\x01R[b\r\x89\xE8`\x02\x0B\x81` \x01Q`\x02\x0B\x12aH\xEDWb\r\x89\xE8` \x82\x01R[aH\xFA\x81` \x01Qa=}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x82\x90R\x89Q\x90\x8E\x01QaID\x93\x91\x92\x91\x16\x80\x82\x18\x91\x81\x11`\x01\x8A\x16\x18\x91\x90\x91\x02\x18\x89`@\x01Q\x86\x8CaRUV[`\xC0\x85\x01R`\xA0\x84\x01R`\x80\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x8AQ_\x12\x15aI\xADWaI\x81\x81`\xA0\x01QaO\xEAV[\x83\x03\x92PaI\x9C\x81`\xC0\x01Q\x82`\x80\x01Qa@\x95\x91\x90a_\x92V[aI\xA6\x90\x83ag\x91V[\x91PaI\xDEV[aI\xC0\x81`\xC0\x01Q\x82`\x80\x01Q\x01aO\xEAV[\x83\x01\x92PaI\xD1\x81`\xA0\x01QaO\xEAV[aI\xDB\x90\x83ae\xDFV[\x91P[\x83\x15aJ\x1AW_b\x0FB@\x85\x83`\xC0\x01Q\x84`\x80\x01Q\x01\x02\x81aJ\x03WaJ\x03ag\xB0V[`\xC0\x84\x01\x80Q\x92\x90\x91\x04\x91\x82\x90\x03\x90R\x99\x90\x99\x01\x98P[`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15aJyWaJm\x81`\xC0\x01Qp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89`@\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x02\x04\x90V[`\xE0\x82\x01\x80Q\x90\x91\x01\x90R[\x80``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aK\x8FW\x80`@\x01Q\x15aKjW_\x80\x86aJ\xD3W\x8D`\x01\x01T\x83`\xE0\x01QaJ\xDEV[\x82`\xE0\x01Q\x8E`\x02\x01T[\x91P\x91P_aK6\x8F\x85` \x01Q\x85\x85`\x02\x92\x83\x0B_\x90\x81R`\x04\x90\x94\x01` R`@\x90\x93 `\x01\x81\x01\x80T\x90\x92\x03\x90\x91U\x90\x81\x01\x80T\x90\x92\x03\x90\x91UTp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0B\x90V[\x90P\x87\x15aKAW_\x03[aKO\x8A`@\x01Q\x82a@\xDEV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x8B\x01RPPP[\x84aKyW\x80` \x01QaK\x82V[`\x01\x81` \x01Q\x03[`\x02\x0B` \x88\x01RaG\xFBV[\x80Q\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aK\xBDW\x86QaK\x82\x90aA\x89V[aG\xFBV[\x86Q` \x88\x01QaLW\x91\x90aL\x19\x90\x89\x90`\xA0\x1Bv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x8CU`@\x87\x01Q`\x03\x8D\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aL\xC6W`@\x87\x01Q`\x03\x8D\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x84aL\xDAW`\xE0\x81\x01Q`\x02\x8D\x01UaL\xE5V[`\xE0\x81\x01Q`\x01\x8D\x01U[\x8AQ_\x13\x85\x15\x15\x14aM\x12WaM\x0BaL\xFD\x83a&iV[a)\x10\x85\x8E_\x01Q\x03a&iV[\x99PaM/V[aM,aM#\x84\x8D_\x01Q\x03a&iV[a)\x10\x84a&iV[\x99P[PPPPPP[\x92\x95\x91\x94P\x92PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90aM~\x90\x84\x90a_\x7FV[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01a ,V[`@Q\x83\x81R\x82`\x02\x0B`\x04\x82\x01R\x81`\x02\x0B`$\x82\x01R`D\x81\xFD[_\x83\x83\x02\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80\x84\x11aN0W_\x80\xFD[\x80_\x03aNBWP\x82\x90\x04\x90Pa\x088V[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aN\xE6W\x92\x93\x92[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aO\rWb\xBF\xC9!_R`\x04`\x1C\xFD[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x86\x03\x16\x83aO\x99W\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aO\x86\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aM\xF2V[\x81aO\x93WaO\x93ag\xB0V[\x04aO\xDFV[aO\xDFaO\xBD\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aS\xC5V[\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x97\x96PPPPPPPV[\x80_\x81\x12\x15a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x90\x86\x16\x03`\xFF\x81\x90\x1D\x90\x81\x01\x18l\x01\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aPn\x81\x84\x84aM\xF2V[\x93P\x84_\x83\x85\x84\t\x11\x16\x84\x01\x93PPPP\x94\x93PPPPV[_\x80\x82\x11aP\x93W_\x80\xFD[P\x7F\x07\x06\x06\x05\x06\x02\x05\0\x06\x02\x03\x02\x05\x04\0\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBEo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11`\x07\x1B\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[b\xBF\xFF\xFF\x81\x16a\x0Cb\x81a$\x10V[_\x80`\x02\x84\x81\x0B\x90\x86\x90\x0B\x81\x81\x07\x83\x13\x91\x90\x05\x03\x83\x15aQ\xC8W`\x02\x81\x90\x0B`\x08\x1D`\x01\x81\x90\x0B_\x90\x81R` \x89\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x80\x85\x16\x90\x81\x90\x03\x91\x90\x91\x1C\x91\x82\x16\x80\x15\x15\x95P\x90\x91\x90\x85aQ\xAAW\x88\x83`\xFF\x16\x86\x03\x02aQ\xBDV[\x88aQ\xB4\x82aP\x87V[\x84\x03`\xFF\x16\x86\x03\x02[\x96PPPPPaRKV[`\x01\x90\x81\x01`\x02\x81\x90\x0B`\x08\x1D\x80\x83\x0B_\x90\x81R` \x8A\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x84\x16\x94\x85\x1B\x01\x19\x90\x81\x16\x80\x15\x15\x95P\x92\x93\x91\x92\x85aR1W\x88\x83`\xFF\x03`\xFF\x16\x86\x01\x02aRDV[\x88\x83aR<\x83aS\xF5V[\x03`\xFF\x16\x86\x01\x02[\x96PPPPP[P\x94P\x94\x92PPPV[_\x80\x80\x80b\xFF\xFF\xFF\x85\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x90\x8B\x16\x10\x15\x82\x88\x12\x80\x15aS8W_aR\x9B\x8A_\x03\x85b\x0FB@\x03b\x0FB@aM\xF2V[\x90P\x82aR\xB4WaR\xAF\x8D\x8D\x8D`\x01aP\x1CV[aR\xC1V[aR\xC1\x8C\x8E\x8D`\x01aN\xADV[\x96P\x86\x81\x10aR\xF5W\x8B\x97Pb\x0FB@\x84\x14aR\xECWaR\xE7\x87\x85\x86b\x0FB@\x03aS\xC5V[aR\xEEV[\x86[\x94PaS\x0EV[\x80\x96PaS\x04\x8D\x8C\x83\x86aT\x8FV[\x97P\x86\x8A_\x03\x03\x94P[\x82aS$WaS\x1F\x8D\x89\x8D_aN\xADV[aS0V[aS0\x88\x8E\x8D_aP\x1CV[\x95PPaS\xB6V[\x81aSNWaSI\x8C\x8C\x8C_aN\xADV[aSZV[aSZ\x8B\x8D\x8C_aP\x1CV[\x94P\x84\x89\x10aSkW\x8A\x96PaS}V[\x88\x94PaSz\x8C\x8B\x87\x85aT\xF3V[\x96P[\x81aS\x94WaS\x8F\x8C\x88\x8C`\x01aP\x1CV[aS\xA1V[aS\xA1\x87\x8D\x8C`\x01aN\xADV[\x95PaS\xB3\x86\x84\x85b\x0FB@\x03aS\xC5V[\x93P[PPP\x95P\x95P\x95P\x95\x91PPV[_aS\xD1\x84\x84\x84aM\xF2V[\x90P\x81\x80aS\xE1WaS\xE1ag\xB0V[\x83\x85\t\x15a\x088W`\x01\x01\x80a\x088W_\x80\xFD[_\x80\x82\x11aT\x01W_\x80\xFD[P~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05_\x82\x90\x03\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aT\xCFWcO$a\xB8_R`\x04`\x1C\xFD[\x81aT\xE6WaT\xE1\x85\x85\x85`\x01aULV[a+\rV[a+\r\x85\x85\x85`\x01aV\xAEV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aU3WcO$a\xB8_R`\x04`\x1C\xFD[\x81aUDWaT\xE1\x85\x85\x85_aV\xAEV[a+\r\x85\x85\x85_[_\x81\x15aU\xF1W_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aU\x9FWaU\x9A\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aM\xF2V[aU\xBFV[aU\xBFo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16``\x86\x90\x1Bag\xDDV[\x90PaU\xE9aU\xE4\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16a_\x92V[aW\xE3V[\x91PPa\x1D\x01V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aV=WaV8\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aS\xC5V[aVcV[aVc``\x85\x90\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x11aV\x8FWcC#\xA5U_R`\x04`\x1C\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x03\x90Pa\x1D\x01V[_\x82_\x03aV\xBDWP\x83a\x1D\x01V[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x85\x90\x1B\x16\x82\x15aW\x88Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x81aW\x10WaW\x10ag\xB0V[\x04\x03aWMW\x81\x81\x01\x82\x81\x10aWKWaWA\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83aS\xC5V[\x93PPPPa\x1D\x01V[P[PaU\xE9\x81\x85aWss\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x83ag\xDDV[aW}\x91\x90a_\x92V[\x80\x82\x04\x91\x06\x15\x15\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x04\x14\x81\x83\x11\x16aW\xBCWc\xF5\xC7\x87\xF1_R`\x04`\x1C\xFD[\x80\x82\x03aWAaU\xE4\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x84aS\xC5V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17\x84W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15aX\\W_\x80\xFD[\x825aXg\x81aX*V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aX\x85W_\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x088W_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15aX\xC6W_\x80\xFD[\x835aX\xD1\x81aX*V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_\x80_``\x84\x86\x03\x12\x15aX\xF8W_\x80\xFD[\x835aY\x03\x81aX*V[\x92P` \x84\x015aY\x13\x81aX*V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15aY4W_\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aY\x8BWaY\x8BaY;V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aY\xD8WaY\xD8aY;V[`@R\x91\x90PV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbW_\x80\xFD[\x805`\x02\x81\x90\x0B\x81\x14a\x0CbW_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15aZ\x13W_\x80\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aZ6WaZ6aY;V[`@R\x90P\x80\x825aZG\x81aX*V[\x81R` \x83\x015aZW\x81aX*V[` \x82\x01RaZh`@\x84\x01aY\xE0V[`@\x82\x01RaZy``\x84\x01aY\xF2V[``\x82\x01R`\x80\x83\x015aZ\x8C\x81aX*V[`\x80\x91\x90\x91\x01R\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aZ\xA9W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xC0W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aZ\xD7W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15aZ\xF3W_\x80\xFD[aZ\xFD\x87\x87aZ\x03V[\x94P`\xA0\x86\x015\x93P`\xC0\x86\x015\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[&W_\x80\xFD[a[2\x88\x82\x89\x01aZ\x99V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a[SW_\x80\xFD[\x815a\x088\x81aX*V[_\x80`@\x83\x85\x03\x12\x15a[oW_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a[\xB5W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a[\x97V[P\x90\x95\x94PPPPPV[_\x80` \x83\x85\x03\x12\x15a[\xD1W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\xE7W_\x80\xFD[a[\xF3\x85\x82\x86\x01aZ\x99V[\x90\x96\x90\x95P\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_\x80`\xC0\x83\x85\x03\x12\x15a\\cW_\x80\xFD[a\\m\x84\x84aZ\x03V[\x91Pa\\{`\xA0\x84\x01aY\xE0V[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x0CbW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\\\xA4W_\x80\xFD[\x825a\\\xAF\x81aX*V[\x91Pa\\{` \x84\x01a\\\x84V[_\x80_\x80\x84\x86\x03a\x01@\x81\x12\x15a\\\xD2W_\x80\xFD[a\\\xDC\x87\x87aZ\x03V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a]\rW_\x80\xFD[Pa]\x16aYhV[a]\"`\xA0\x87\x01aY\xF2V[\x81Ra]0`\xC0\x87\x01aY\xF2V[` \x82\x01R`\xE0\x86\x015`@\x82\x01Ra\x01\0\x86\x015``\x82\x01R\x92Pa\x01 \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]fW_\x80\xFD[a]r\x87\x82\x88\x01aZ\x99V[\x95\x98\x94\x97P\x95PPPPV[_\x80`\xC0\x83\x85\x03\x12\x15a]\x8FW_\x80\xFD[a]\x99\x84\x84aZ\x03V[\x91P`\xA0\x83\x015a]\xA9\x81aX*V[\x80\x91PP\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15a]\xC5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xDBW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a]\xEBW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x01W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a^\x15W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a^6W_\x80\xFD[\x825a^A\x81aX*V[\x91P` \x83\x015a]\xA9\x81aX*V[_\x80_\x80\x84\x86\x03a\x01 \x81\x12\x15a^fW_\x80\xFD[a^p\x87\x87aZ\x03V[\x94P``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a^\xA1W_\x80\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a^\xC5Wa^\xC5aY;V[`@Ra^\xD4`\xA0\x87\x01a\\\x84V[\x81R`\xC0\x86\x015` \x82\x01R`\xE0\x86\x015a^\xEE\x81aX*V[`@\x82\x01R\x92Pa\x01\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]fW_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a_\"W_\x80\xFD[\x845a_-\x81aX*V[\x93P` \x85\x015a_=\x81aX*V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07eWa\x07ea_RV[\x80\x82\x01\x80\x82\x11\x15a\x07eWa\x07ea_RV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R_a\x1D\x01` \x83\x01\x84\x86a_\xA5V[_` \x82\x84\x03\x12\x15a`\x0FW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`%W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a`5W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`OWa`OaY;V[a`\x80` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01aY\x91V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a`\x94W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RaaL` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84`\xC0\x82\x01R\x83`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_aaqa\x01 \x83\x01\x84\x86a_\xA5V[\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rab\x18` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x85\x01Q\x90\x0B`\xE0\x82\x01R`@\x84\x01Qa\x01\0\x82\x01R``\x84\x01Qa\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_aO\xDFa\x01`\x83\x01\x84\x86a_\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x81Rab\xFA` \x82\x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x85Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x87\x01Q\x90\x0B`\xE0\x82\x01R`@\x86\x01Qa\x01\0\x82\x01R``\x86\x01Qa\x01 \x82\x01R\x84a\x01@\x82\x01R\x83a\x01`\x82\x01Ra\x01\xA0a\x01\x80\x82\x01R_aD\xA0a\x01\xA0\x83\x01\x84\x86a_\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R`\xE0\x81\x01ac\xEE` \x83\x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\xC0\x83\x01R\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81Ra\x01\0\x81\x01ad\xB2` \x83\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\xC0\x83\x01R\x82`\x02\x0B`\xE0\x83\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15ad\xF0W_\x80\xFD[PQ\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rae\x92` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q\x15\x15`\xC0\x82\x01R` \x84\x01Q`\xE0\x82\x01R`@\x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01R_aO\xDFa\x01@\x83\x01\x84\x86a_\xA5V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15ae\xFEWae\xFEa_RV[PP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81Raf\xA1` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84Q\x15\x15`\xC0\x82\x01R` \x85\x01Q`\xE0\x82\x01R`@\x85\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01R\x83a\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_aaqa\x01`\x83\x01\x84\x86a_\xA5V[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12\x17\x15a\x07eWa\x07ea_RV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x07eWa\x07ea_RV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a9\x8FWa9\x8Fa_RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82ah\x10W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101f4575f3560e01c80635a6bcfda11610117578063a5841194116100ac578063f135baaa1161007c578063f3cd914c11610062578063f3cd914c14610676578063f5298aca14610695578063fe99049a146106b4575f80fd5b8063f135baaa14610638578063f2fde38b14610657575f80fd5b8063a584119414610595578063b6363cf2146105b4578063dbd035ff146105ed578063f02de3b21461060c575f80fd5b80638161b874116100e75780638161b874146104dc5780638da5cb5b146104fb57806397e8cd4e1461054b5780639bf6645f14610576575f80fd5b80635a6bcfda146104385780636276cbbe1461046c5780637e87ce7d1461049e57806380f0b44c146104bd575f80fd5b80632d7713891161018d57806348c894911161015d57806348c894911461039257806352759651146103be578063558a7297146103dd578063598af9e7146103fc575f80fd5b80632d7713891461031557806335fd631a146103345780633dd45adb14610360578063426a849314610373575f80fd5b806311da60b4116101c857806311da60b4146102b0578063156e29f6146102b85780631e2eaeaf146102d7578063234266d7146102f6575f80fd5b8062fdd58e146101f857806301ffc9a714610241578063095bcdb6146102705780630b0d9c091461028f575b5f80fd5b348015610203575f80fd5b5061022e61021236600461584b565b600460209081525f928352604080842090915290825290205481565b6040519081526020015b60405180910390f35b34801561024c575f80fd5b5061026061025b366004615875565b6106d3565b6040519015158152602001610238565b34801561027b575f80fd5b5061026061028a3660046158b4565b61076b565b34801561029a575f80fd5b506102ae6102a93660046158e6565b61083f565b005b61022e6108c9565b3480156102c3575f80fd5b506102ae6102d23660046158b4565b610927565b3480156102e2575f80fd5b5061022e6102f1366004615924565b6109ab565b348015610301575f80fd5b5061022e610310366004615ade565b6109b5565b348015610320575f80fd5b506102ae61032f366004615b43565b610ad9565b34801561033f575f80fd5b5061035361034e366004615b5e565b610bcc565b6040516102389190615b7e565b61022e61036e366004615b43565b610c09565b34801561037e575f80fd5b5061026061038d3660046158b4565b610c67565b34801561039d575f80fd5b506103b16103ac366004615bc0565b610cd8565b6040516102389190615bff565b3480156103c9575f80fd5b506102ae6103d8366004615c52565b610e2a565b3480156103e8575f80fd5b506102606103f7366004615c93565b610ecc565b348015610407575f80fd5b5061022e6104163660046158e6565b600560209081525f938452604080852082529284528284209052825290205481565b348015610443575f80fd5b50610457610452366004615cbd565b610f66565b60408051928352602083019190915201610238565b348015610477575f80fd5b5061048b610486366004615d7e565b611165565b60405160029190910b8152602001610238565b3480156104a9575f80fd5b506102ae6104b8366004615c52565b6113fd565b3480156104c8575f80fd5b506102ae6104d736600461584b565b6114ee565b3480156104e7575f80fd5b5061022e6104f63660046158e6565b6115ad565b348015610506575f80fd5b505f546105269073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610238565b348015610556575f80fd5b5061022e610565366004615b43565b60016020525f908152604090205481565b348015610581575f80fd5b50610353610590366004615db4565b6116d9565b3480156105a0575f80fd5b506102ae6105af366004615b43565b611712565b3480156105bf575f80fd5b506102606105ce366004615e25565b600360209081525f928352604080842090915290825290205460ff1681565b3480156105f8575f80fd5b50610353610607366004615db4565b6117b7565b348015610617575f80fd5b506002546105269073ffffffffffffffffffffffffffffffffffffffff1681565b348015610643575f80fd5b5061022e610652366004615924565b6117ee565b348015610662575f80fd5b506102ae610671366004615b43565b6117f8565b348015610681575f80fd5b5061022e610690366004615e51565b6118e7565b3480156106a0575f80fd5b506102ae6106af3660046158b4565b611a99565b3480156106bf575f80fd5b506102606106ce366004615f0f565b611b1d565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316148061076557507f0f632fb3000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b335f908152600460209081526040808320858452909152812080548391908390610796908490615f7f565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f908152600460209081526040808320868452909152812080548492906107da908490615f92565b9091555050604080513380825260208201859052859273ffffffffffffffffffffffffffffffffffffffff8816927f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a45060015b9392505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61088e5761088e7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b6108a38361089b83611d11565b5f0333611d56565b6108c473ffffffffffffffffffffffffffffffffffffffff84168383611db6565b505050565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610919576109197f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b61092233611eb1565b905090565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610976576109767f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b816109848161089b84611d11565b6109a58473ffffffffffffffffffffffffffffffffffffffff831684611f9a565b50505050565b5f81545f5260205ff35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610a0557610a057f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b610a0d612039565b60a086205f818152600660205260409020610a27816120a1565b6080880151610a509073ffffffffffffffffffffffffffffffffffffffff1689898989896120e8565b610a5b8188886121cb565b9250610a688884336122c5565b6040805188815260208101889052339184917f29ef05caaff9404b7cb6d1c0e9bbae9eaa7ab2541feba1a9c4248594c08156cb910160405180910390a36080880151610ace9073ffffffffffffffffffffffffffffffffffffffff1689898989896122ed565b505095945050505050565b5f5473ffffffffffffffffffffffffffffffffffffffff163314610b5e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a4544000000000000000000000000000000000000000060448201526064015b60405180910390fd5b600280547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081179091556040517fb4bd8ef53df690b9943d3318996006dbb82a25f54719d8c8035b516a2a5b8acc905f90a250565b6060604051808360051b6020835284602084015260408301925080830190505b85548352602083019250600186019550808310610bec5781810382f35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610c5957610c597f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b61076582611eb1565b919050565b335f81815260056020908152604080832073ffffffffffffffffffffffffffffffffffffffff881680855290835281842087855290925280832085905551919285927fb3fd5071835887567a0671151121894ddccc2842f1d10bedad13e0d17cace9a79061082c9087815260200190565b60607fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c15610d2a57610d2a7f5090d6c600000000000000000000000000000000000000000000000000000000611d09565b610d326123c5565b6040517f91dd734600000000000000000000000000000000000000000000000000000000815233906391dd734690610d709086908690600401615fec565b5f604051808303815f875af1158015610d8b573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610dd09190810190615fff565b90507f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c15610e2257610e227f5212cba100000000000000000000000000000000000000000000000000000000611d09565b6107656123eb565b604082015162ffffff1662800000141580610e755750816080015173ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b15610ea357610ea37f30d2164100000000000000000000000000000000000000000000000000000000611d09565b610eb18162ffffff16612410565b60a082205f8181526006602052604090206108c4908361244f565b335f81815260036020908152604080832073ffffffffffffffffffffffffffffffffffffffff871680855290835281842080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001687151590811790915591519182529293917fceb576d9f15e4e200fdb5096d64d5dfd667e16def20c1eefd14256d8e3faa267910160405180910390a350600192915050565b5f807fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610fb757610fb77f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b610fbf612039565b60a086205f818152600660205260409020610fd9816120a1565b60808801516110019073ffffffffffffffffffffffffffffffffffffffff16898989896124a8565b5f6110756040518060c001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020018a5f015160020b81526020018a6020015160020b81526020016110528b60400151612669565b600f0b81526060808d015160020b60208301528b0151604090910152839061269e565b945090506110838185612ae3565b945050503373ffffffffffffffffffffffffffffffffffffffff16817ff208f4912782fd25c7f114ca3723a2d5dd6f3bcc3ac8db5af63baa85f711d5ec885f015189602001518a604001518b606001516040516111019493929190600294850b81529290930b60208301526040820152606081019190915260800190565b60405180910390a360808701515f906111359073ffffffffffffffffffffffffffffffffffffffff16898987878b8b612b16565b9094509050801561114f5761114f88828a608001516122c5565b61115a8885336122c5565b505094509492505050565b5f61116e612039565b6060830151617fff60029190910b13156111b25760608301516111b2907fb70024f80000000000000000000000000000000000000000000000000000000090612ced565b600160020b836060015160020b12156111f55760608301516111f5907fe9e905880000000000000000000000000000000000000000000000000000000090612ced565b8251602084015173ffffffffffffffffffffffffffffffffffffffff90811691161061124d578251602084015161124d917f6e6c98300000000000000000000000000000000000000000000000000000000091612cfc565b61127e8360400151846080015173ffffffffffffffffffffffffffffffffffffffff16612d3f90919063ffffffff16565b6112b25760808301516112b2907fe65af6a00000000000000000000000000000000000000000000000000000000090612e0d565b5f6112c5846040015162ffffff16612e2f565b60808501519091506112ee9073ffffffffffffffffffffffffffffffffffffffff168585612e54565b60a084205f81815260066020526040902061130a908584612f27565b60808601519093506113349073ffffffffffffffffffffffffffffffffffffffff16868686612fdf565b846020015173ffffffffffffffffffffffffffffffffffffffff16855f015173ffffffffffffffffffffffffffffffffffffffff16827fdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d6438886040015189606001518a608001518a8a6040516113ed95949392919062ffffff959095168552600293840b602086015273ffffffffffffffffffffffffffffffffffffffff928316604086015291166060840152900b608082015260a00190565b60405180910390a4505092915050565b60025473ffffffffffffffffffffffffffffffffffffffff163314611445576114457f48f5c3ed00000000000000000000000000000000000000000000000000000000611d09565b6103e9610fff821610623e900062fff0008316101661148d5761148d7fa7abe2f70000000000000000000000000000000000000000000000000000000062ffffff8316612e0d565b60a082206114af826114a9835f90815260066020526040902090565b906130b4565b60405162ffffff8316815281907fe9c42593e71f84403b84352cd168d693e2c9fcd1fdbcc3feb21d92b43e6696f99060200160405180910390a2505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61153d5761153d7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b335f90815273ffffffffffffffffffffffffffffffffffffffff8316602052604081205c9061156b83611d11565b90508181600f0b146115a0576115a07fbda73abf00000000000000000000000000000000000000000000000000000000611d09565b6109a584825f0333611d56565b6002545f9073ffffffffffffffffffffffffffffffffffffffff1633146115f7576115f77f48f5c3ed00000000000000000000000000000000000000000000000000000000611d09565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c15611647576116477f3e5f4fd600000000000000000000000000000000000000000000000000000000611d09565b81156116535781611679565b73ffffffffffffffffffffffffffffffffffffffff83165f908152600160205260409020545b73ffffffffffffffffffffffffffffffffffffffff84165f908152600160205260408120805492935083929091906116b2908490615f7f565b90915550610838905073ffffffffffffffffffffffffffffffffffffffff84168583611db6565b606060405180602082528360208301526040820191508360051b8201855b80355c8452602093840193018184106116f7575b5081810382f35b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611761576117617f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b73ffffffffffffffffffffffffffffffffffffffff811661178757611784613108565b50565b5f6117a78273ffffffffffffffffffffffffffffffffffffffff1661312d565b90506117b382826131dc565b5050565b606060405180602082528360208301526040820191508360051b8201855b8035548452602093840193018184101561170b576117d5565b5f815c5f5260205ff35b5f5473ffffffffffffffffffffffffffffffffffffffff163314611878576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a454400000000000000000000000000000000000000006044820152606401610b55565b5f80547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081178255604051909133917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a350565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611937576119377f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b61193f612039565b83602001515f03611973576119737fbe8b850700000000000000000000000000000000000000000000000000000000611d09565b60a085205f81815260066020526040902061198d816120a1565b60808701515f90819081906119bb9073ffffffffffffffffffffffffffffffffffffffff168b8b8b8b61323c565b809350819550829450505050611a3784866040518060a001604052808681526020018e6060015160020b81526020018d5f0151151581526020018d6040015173ffffffffffffffffffffffffffffffffffffffff1681526020018562ffffff168152508c5f0151611a30578d602001516133e3565b8d516133e3565b60808b01519096505f9250611a68915073ffffffffffffffffffffffffffffffffffffffff168a8a888b8b886134e4565b90955090508015611a8257611a8289828b608001516122c5565b611a8d8986336122c5565b50505050949350505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611ae857611ae87f54e3ca0d00000000000000000000000000000000000000000000000000000000611d09565b81611afc81611af684611d11565b33611d56565b6109a58473ffffffffffffffffffffffffffffffffffffffff83168461367d565b5f3373ffffffffffffffffffffffffffffffffffffffff861614801590611b74575073ffffffffffffffffffffffffffffffffffffffff85165f90815260036020908152604080832033845290915290205460ff16155b15611c1d5773ffffffffffffffffffffffffffffffffffffffff85165f90815260056020908152604080832033845282528083208684529091529020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114611c1b57611be28382615f7f565b73ffffffffffffffffffffffffffffffffffffffff87165f90815260056020908152604080832033845282528083208884529091529020555b505b73ffffffffffffffffffffffffffffffffffffffff85165f90815260046020908152604080832086845290915281208054849290611c5c908490615f7f565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f90815260046020908152604080832086845290915281208054849290611ca0908490615f92565b90915550506040805133815260208101849052849173ffffffffffffffffffffffffffffffffffffffff80881692908916917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859910160405180910390a45060015b949350505050565b805f5260045ffd5b5f6f800000000000000000000000000000008210611d5257611d527f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b5090565b81600f0b5f03611d6557505050565b5f80611d8873ffffffffffffffffffffffffffffffffffffffff8616848661378e565b91509150805f03611da057611d9b6137d4565b611daf565b815f03611daf57611daf613822565b5050505050565b5f73ffffffffffffffffffffffffffffffffffffffff8416611e10575f805f8085875af1905080611e0b57611e0b7f8549db590000000000000000000000000000000000000000000000000000000084613870565b6109a5565b6040517fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8416600482015282602482015260205f6044835f895af13d15601f3d1160015f511416171691505f81525f60208201525f604082015250806109a5576109a57fb12c5f9c0000000000000000000000000000000000000000000000000000000085613870565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95c73ffffffffffffffffffffffffffffffffffffffff8116611ef757349150611f81565b3415611f2657611f267fb0ec849e00000000000000000000000000000000000000000000000000000000611d09565b7f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955c5f611f6873ffffffffffffffffffffffffffffffffffffffff841661312d565b9050611f748282615f7f565b9350611f7e613108565b50505b611f9481611f8e84611d11565b85611d56565b50919050565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290611fd9908490615f92565b90915550506040805133815260208101839052839173ffffffffffffffffffffffffffffffffffffffff8616915f917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a4505050565b3073ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000161461209f5761209f7f0d89438e00000000000000000000000000000000000000000000000000000000611d09565b565b805473ffffffffffffffffffffffffffffffffffffffff165f03611784576117847f486aa30700000000000000000000000000000000000000000000000000000000611d09565b853373ffffffffffffffffffffffffffffffffffffffff8216146121c25760208716156121c2576121c033878787878760405160240161212d969594939291906160b1565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb6a8b0fa0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8916906138a3565b505b50505050505050565b60038301545f906fffffffffffffffffffffffffffffffff16808203612214576122147fa74f97ab00000000000000000000000000000000000000000000000000000000611d09565b61224b61222085611d11565b5f0361222b85611d11565b5f0360809190911b6fffffffffffffffffffffffffffffffff9091161790565b91508315612285576001850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008702040190555b82156122bd576002850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008602040190555b509392505050565b82516122db906122d58460801d90565b83611d56565b6108c483602001516122d584600f0b90565b853373ffffffffffffffffffffffffffffffffffffffff8216146121c25760108716156121c2576121c0338787878787604051602401612332969594939291906160b1565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe1b4af690000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8916906138a3565b60017fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b620f424062ffffff82161115611784576117847f140021130000000000000000000000000000000000000000000000000000000062ffffff8316612e0d565b612458826120a1565b81547fffffff000000ffffffffffffffffffffffffffffffffffffffffffffffffffff167cffffff000000000000000000000000000000000000000000000000000060d083901b16175b90915550565b843373ffffffffffffffffffffffffffffffffffffffff821614612661575f84604001511380156124dc5750610800861615155b156125965761259033868686866040516024016124fd95949392919061617d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f259982e50000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8816906138a3565b50612661565b5f8460400151131580156125ad5750610200861615155b15612661576121c233868686866040516024016125ce95949392919061617d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f21d0ee700000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8816906138a3565b505050505050565b80600f81900b8114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b6060810151602082015160408301515f92839290916126bd8282613996565b604080516080810182525f80825260208201819052918101829052606081019190915283600f0b5f1461288d576126f68884865f613a5d565b6fffffffffffffffffffffffffffffffff1660208301521515815261271e8883866001613a5d565b6fffffffffffffffffffffffffffffffff166060830152151560408201525f600f85900b126128525760808701515f9060020b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761881810783139082900503620d89e891909105036001016fffffffffffffffffffffffffffffffff049050806fffffffffffffffffffffffffffffffff1682602001516fffffffffffffffffffffffffffffffff1611156127f6576127f67fb8e3c3850000000000000000000000000000000000000000000000000000000085612ced565b806fffffffffffffffffffffffffffffffff1682606001516fffffffffffffffffffffffffffffffff161115612850576128507fb8e3c3850000000000000000000000000000000000000000000000000000000084612ced565b505b80511561286e57608087015161286e9060058a01908590613b46565b80604001511561288d57608087015161288d9060058a01908490613b46565b5f8061289a8a8686613b98565b8a5160a08c015160408051602681019290925260068083018a9052600383018b9052928252603a600c8301205f838301819052602080850182905293819052908152928f019091528120929450909250806128f7838a8787613c4c565b9150915061292c61290783611d11565b61291083611d11565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b995050505050505f84600f0b12156129955780511561296857600283810b5f90815260048a016020526040812081815560018101829055909101555b80604001511561299557600282810b5f90815260048a016020526040812081815560018101829055909101555b5082600f0b5f14612ad95786545f806129b18360a01c60020b90565b73ffffffffffffffffffffffffffffffffffffffff8416915091508460020b8260020b1215612a0d57612a06612a006129fb6129ec88613d7d565b6129f588613d7d565b8a61407a565b612669565b60801b90565b9750612ad5565b8360020b8260020b1215612ab057612a44612a2e6129fb836129f588613d7d565b6129106129fb612a3d89613d7d565b858b6140b2565b60038b0154909850612a68906fffffffffffffffffffffffffffffffff16876140de565b60038b0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff92909216919091179055612ad5565b612ad25f6129106129fb612ac389613d7d565b612acc89613d7d565b8b6140b2565b97505b5050505b5050509250929050565b5f608082811d9084901d01600f83810b9085900b01612b0d612b0483612669565b61291083612669565b95945050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff89163303612b3f57508490505f612ce1565b8591505f87604001511315612c2757610400891615612c2257612c1333898989898989604051602401612b78979695949392919061625f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9f063efc0000000000000000000000000000000000000000000000000000000017905260028b1615155b73ffffffffffffffffffffffffffffffffffffffff8c16919061410e565b9050612c1f8282614168565b91505b612ce1565b610100891615612ce157612cd233898989898989604051602401612c51979695949392919061625f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f6c2bbe7e0000000000000000000000000000000000000000000000000000000017905260018b161515612bf5565b9050612cde8282614168565b91505b97509795505050505050565b815f528060020b60045260245ffd5b60405183815273ffffffffffffffffffffffffffffffffffffffff8316600482015273ffffffffffffffffffffffffffffffffffffffff82166024820152604481fd5b5f60808316158015612d5357506008831615155b15612d5f57505f610765565b60408316158015612d7257506004831615155b15612d7e57505f610765565b6104008316158015612d9257506002831615155b15612d9e57505f610765565b6101008316158015612db257506001831615155b15612dbe57505f610765565b73ffffffffffffffffffffffffffffffffffffffff831615612dfc57613fff8316151580612df757506280000062ffffff831614610838565b610838565b5062ffffff16628000001415919050565b815f5273ffffffffffffffffffffffffffffffffffffffff811660045260245ffd5b5f6280000062ffffff831603612e4657505f919050565b611d528262ffffff16612410565b823373ffffffffffffffffffffffffffffffffffffffff8216146109a5576120008416156109a557611daf338484604051602401612e949392919061634f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fdc98354e0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8616906138a3565b82545f9073ffffffffffffffffffffffffffffffffffffffff1615612f6f57612f6f7f7983c05100000000000000000000000000000000000000000000000000000000611d09565b612f7883614189565b90507cffffff000000000000000000000000000000000000000000000000000060d083901b1673ffffffffffffffffffffffffffffffffffffffff841660a083901b76ffffff00000000000000000000000000000000000000001617179093555090919050565b833373ffffffffffffffffffffffffffffffffffffffff821614611daf57611000851615611daf57612661338585856040516024016130219493929190616412565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f6fe7e6eb0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff8716906138a3565b6130bd826120a1565b81547fffffffffffff000000ffffffffffffffffffffffffffffffffffffffffffffff1679ffffff000000000000000000000000000000000000000000000060b883901b16176124a2565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d565b5f73ffffffffffffffffffffffffffffffffffffffff8216613150575047919050565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff8316906370a0823190602401602060405180830381865afa1580156131b8573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061076591906164e0565b73ffffffffffffffffffffffffffffffffffffffff82167f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d807f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955d5050565b60208301515f8073ffffffffffffffffffffffffffffffffffffffff88163303613268575f91506133d8565b60808816156133d8575f61330a89338a8a8a8a60405160240161328f9594939291906164f7565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f575e24b4000000000000000000000000000000000000000000000000000000001790526138a3565b9050805160601461333e5761333e7f1e048e1d00000000000000000000000000000000000000000000000000000000611d09565b604088015162ffffff16628000000361335957606081015191505b60088916156133d657604081015192505f6133748460801d90565b905080600f0b5f146133d4575f8512613391600f83900b876165df565b9550806133a0575f86126133a4565b5f86135b156133d2576133d27ffa0b71d600000000000000000000000000000000000000000000000000000000611d09565b505b505b505b955095509592505050565b5f808080806133f289886144ad565b93509350935093505f83111561342d5773ffffffffffffffffffffffffffffffffffffffff86165f9081526001602052604090208054840190555b33887f40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f61345a8760801d90565b61346488600f0b90565b85516040808801516020808a01518351600f97880b81529590960b9085015273ffffffffffffffffffffffffffffffffffffffff909216908301526fffffffffffffffffffffffffffffffff16606082015260029190910b608082015262ffffff861660a082015260c00160405180910390a35091979650505050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff8916330361350d57508490505f612ce1565b5f6135188460801d90565b90505f61352585600f0b90565b905060408b16156135f8576135eb6129fb338c8c8c8c8c60405160240161355196959493929190616606565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb47b2fb10000000000000000000000000000000000000000000000000000000017905260048e16151573ffffffffffffffffffffffffffffffffffffffff8f16919061410e565b6135f590826166f5565b90505b5f81600f0b5f14158061360e575082600f0b5f14155b1561366b57895160208b01515f1390151514613642576fffffffffffffffffffffffffffffffff8316608083901b1761365c565b6fffffffffffffffffffffffffffffffff8216608084901b175b90506136688982614168565b98505b979b979a509698505050505050505050565b3373ffffffffffffffffffffffffffffffffffffffff841681148015906136d6575073ffffffffffffffffffffffffffffffffffffffff8085165f9081526003602090815260408083209385168352929052205460ff16155b156137835773ffffffffffffffffffffffffffffffffffffffff8085165f9081526005602090815260408083209385168352928152828220868352905220547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114613781576137468382615f7f565b73ffffffffffffffffffffffffffffffffffffffff8087165f9081526005602090815260408083209387168352928152828220888352905220555b505b6109a5848484614d3f565b73ffffffffffffffffffffffffffffffffffffffff8281165f90815290841660205260408120805c91906137c6600f85900b846165df565b915081815d50935093915050565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181039050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181019050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b3d60405183815282600482015260406024820152816044820152815f606483013e602080601f8401040260640191508181fd5b60605f805f8451602086015f885af19050806138e3576138e37f319d54c30000000000000000000000000000000000000000000000000000000085613870565b6040519150601f19603f3d011682016040523d82523d5f602084013e602082511080613961575060208301517fffffffff000000000000000000000000000000000000000000000000000000001661393c836020015190565b7fffffffff000000000000000000000000000000000000000000000000000000001614155b1561398f5761398f7f1e048e1d00000000000000000000000000000000000000000000000000000000611d09565b5092915050565b8060020b8260020b126139ce576139ce7fc4433ed5000000000000000000000000000000000000000000000000000000008383614dd5565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618600283900b1215613a2457613a247fd5e2f7ab0000000000000000000000000000000000000000000000000000000083612ced565b620d89e8600282900b13156117b3576117b37f1ad777f80000000000000000000000000000000000000000000000000000000082612ced565b600283900b5f908152600485016020526040812080548291906fffffffffffffffffffffffffffffffff8116907001000000000000000000000000000000009004600f0b613aab82886140de565b6fffffffffffffffffffffffffffffffff808216159084168015919091141596509094505f03613afe57885460a01c60020b60020b8860020b13613afe576001808a0154908401556002808a0154908401555b5f86613b1357613b0e88836166f5565b613b1d565b613b1d8883616743565b90508060801b6fffffffffffffffffffffffffffffffff86161784555050505094509492505050565b600291820b910b80820715613b735760405163d4d8f3e681528260208201528160408201526044601c8201fd5b80820591508160081d5f528260205260405f20600160ff84161b815418815550505050565b600282810b5f81815260048601602052604080822085850b83529082208754929485949293919260a09290921c900b90811215613bee578160010154836001015403945081600201548360020154039350613c41565b8560020b8160020b12613c1a578260010154826001015403945082600201548260020154039350613c41565b81600101548360010154896001015403039450816002015483600201548960020154030393505b505050935093915050565b83545f9081906fffffffffffffffffffffffffffffffff16600f86900b8203613cb657806fffffffffffffffffffffffffffffffff165f03613cb157613cb17faefeb92400000000000000000000000000000000000000000000000000000000611d09565b613cfd565b613cc081876140de565b87547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff919091161787555b613d3187600101548603826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614df2565b9250613d6787600201548503826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614df2565b6001880195909555505060029094015591929050565b60020b5f60ff82901d80830118620d89e8811115613dbf57613dbf7f8b86327a0000000000000000000000000000000000000000000000000000000084612ced565b7001fffcb933bd6fad37aa2d162d1a5940016001821602700100000000000000000000000000000000186002821615613e08576ffff97272373d413259a46990580e213a0260801c5b6004821615613e27576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b6008821615613e46576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b6010821615613e65576fffcb9843d60f6159c9db58835c9266440260801c5b6020821615613e84576fff973b41fa98c081472e6896dfb254c00260801c5b6040821615613ea3576fff2ea16466c96a3843ec78b326b528610260801c5b6080821615613ec2576ffe5dee046a99a2a811c461f1969c30530260801c5b610100821615613ee2576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b610200821615613f02576ff987a7253ac413176f2b074cf7815e540260801c5b610400821615613f22576ff3392b0822b70005940c7a398e4b70f30260801c5b610800821615613f42576fe7159475a2c29b7443b29c7fa6e889d90260801c5b611000821615613f62576fd097f3bdfd2022b8845ad8f792aa58250260801c5b612000821615613f82576fa9f746462d870fdf8a65dc1f90e061e50260801c5b614000821615613fa2576f70d869a156d2a1b890bb3df62baf32f70260801c5b618000821615613fc2576f31be135f97d08fd981231505542fcfa60260801c5b62010000821615613fe3576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b62020000821615614003576e5d6af8dedb81196699c329225ee6040260801c5b62040000821615614022576d2216e584f5fa1ea926041bedfe980260801c5b6208000082161561403f576b048a170391f7dc42444e8fa20260801c5b5f84131561406a577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff045b63ffffffff0160201c9392505050565b5f8082600f0b126140a15761409a6140958585856001614ead565b614fea565b5f03611d01565b611d016140958585855f035f614ead565b5f8082600f0b126140cd5761409a614095858585600161501c565b611d016140958585855f035f61501c565b6fffffffffffffffffffffffffffffffff8216600f82900b01608081901c15610765576393dafdf15f526004601cfd5b5f8061411a85856138a3565b90508261412a575f915050610838565b805160401461415c5761415c7f1e048e1d00000000000000000000000000000000000000000000000000000000611d09565b60400151949350505050565b5f608082811d9084901d03600f83810b9085900b03612b0d612b0483612669565b5f73fffd8963efd1fc6a506488495d951d51639616827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffefffd895d830173ffffffffffffffffffffffffffffffffffffffff161115614208576142087f614875240000000000000000000000000000000000000000000000000000000083612e0d565b77ffffffffffffffffffffffffffffffffffffffff00000000602083901b16805f61423282615087565b60ff1690506080811061424d57607f810383901c9150614257565b80607f0383901b91505b908002607f81811c60ff83811c9190911c800280831c81831c1c800280841c81841c1c800280851c81851c1c800280861c81861c1c800280871c81871c1c800280881c81881c1c800280891c81891c1c8002808a1c818a1c1c8002808b1c818b1c1c8002808c1c818c1c1c8002808d1c818d1c1c8002808e1c9c81901c9c909c1c80029c8d901c9e9d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808f0160401b60c09190911c678000000000000000161760c19b909b1c674000000000000000169a909a1760c29990991c672000000000000000169890981760c39790971c671000000000000000169690961760c49590951c670800000000000000169490941760c59390931c670400000000000000169290921760c69190911c670200000000000000161760c79190911c670100000000000000161760c89190911c6680000000000000161760c99190911c6640000000000000161760ca9190911c6620000000000000161760cb9190911c6610000000000000161760cc9190911c6608000000000000161760cd9190911c66040000000000001617693627a301d71055774c8581027ffffffffffffffffffffffffffffffffffd709b7e5480fba5a50fed5e62ffc5568101608090811d906fdb2df09e81959a81455e260799a0632f8301901d600281810b9083900b1461449e578873ffffffffffffffffffffffffffffffffffffffff1661447682613d7d565b73ffffffffffffffffffffffffffffffffffffffff16111561449857816144a0565b806144a0565b815b9998505050505050505050565b604080516060810182525f8082526020820181905291810182905281908190855460408601515f816144e757610fff60c484901c166144f1565b610fff60b884901c165b885173ffffffffffffffffffffffffffffffffffffffff8516865261ffff9190911691505f60a085901c60020b60020b602087015260038b01546fffffffffffffffffffffffffffffffff16604087015260808a01515f9062400000166145615760d086901c62ffffff16614573565b6145738b6080015162ffffff1661511b565b905083156145a157620f4240610fff851662ffffff83168181028381061515939004929092019101036145a3565b805b975050620f42408762ffffff16106145e75789515f12156145e7576145e77f9620624600000000000000000000000000000000000000000000000000000000611d09565b89515f036145ff575f80985098505050505050614d36565b83156146e25760608a015173ffffffffffffffffffffffffffffffffffffffff8681169116106146715761467173ffffffffffffffffffffffffffffffffffffffff86165b60608c01517f7c9c6e8f000000000000000000000000000000000000000000000000000000009190612cfc565b6401000276a373ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff16116146dd5760608a01516146dd907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e0d565b6147a0565b60608a015173ffffffffffffffffffffffffffffffffffffffff8681169116116147255761472573ffffffffffffffffffffffffffffffffffffffff8616614644565b73fffd8963efd1fc6a506488495d951d5263988d2673ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff16106147a05760608a01516147a0907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e0d565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152846147ef578b600201546147f5565b8b600101545b60e08201525b82158061483a57508a6060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff16145b614bc257865173ffffffffffffffffffffffffffffffffffffffff168152602080880151908c01516148719160058f01918861512a565b1515604083015260020b602082018190527ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618126148cf577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761860208201525b620d89e860020b816020015160020b126148ed57620d89e860208201525b6148fa8160200151613d7d565b73ffffffffffffffffffffffffffffffffffffffff90811660608381018290528951908e0151614944939192911680821891811160018a161891909102188960400151868c615255565b60c085015260a0840152608083015273ffffffffffffffffffffffffffffffffffffffff1687528a515f12156149ad576149818160a00151614fea565b8303925061499c8160c0015182608001516140959190615f92565b6149a69083616791565b91506149de565b6149c08160c00151826080015101614fea565b830192506149d18160a00151614fea565b6149db90836165df565b91505b8315614a1a575f620f4240858360c001518460800151010281614a0357614a036167b0565b60c084018051929091049182900390529990990198505b60408701516fffffffffffffffffffffffffffffffff1615614a7957614a6d8160c0015170010000000000000000000000000000000089604001516fffffffffffffffffffffffffffffffff1691020490565b60e08201805190910190525b806060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff1603614b8f57806040015115614b6a575f8086614ad3578d600101548360e00151614ade565b8260e001518e600201545b915091505f614b368f85602001518585600292830b5f908152600490940160205260409093206001810180549092039091559081018054909203909155547001000000000000000000000000000000009004600f0b90565b90508715614b41575f035b614b4f8a60400151826140de565b6fffffffffffffffffffffffffffffffff1660408b01525050505b84614b79578060200151614b82565b60018160200151035b60020b60208801526147fb565b8051875173ffffffffffffffffffffffffffffffffffffffff908116911614614bbd578651614b8290614189565b6147fb565b86516020880151614c579190614c1990899060a01b76ffffff0000000000000000000000000000000000000000167fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff919091161790565b7fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9091161790565b8c55604087015160038d01546fffffffffffffffffffffffffffffffff908116911614614cc657604087015160038d0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff9092169190911790555b84614cda5760e081015160028d0155614ce5565b60e081015160018d01555b8a515f1385151514614d1257614d0b614cfd83612669565b612910858e5f015103612669565b9950614d2f565b614d2c614d23848d5f015103612669565b61291084612669565b99505b5050505050505b92959194509250565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290614d7e908490615f7f565b9091555050604080513381526020810183905283915f9173ffffffffffffffffffffffffffffffffffffffff8716917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859910161202c565b6040518381528260020b60048201528160020b6024820152604481fd5b5f838302817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff85870982811083820303915050808411614e30575f80fd5b805f03614e4257508290049050610838565b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff161115614ee6579293925b73ffffffffffffffffffffffffffffffffffffffff8516614f0d5762bfc9215f526004601cfd5b7bffffffffffffffffffffffffffffffff000000000000000000000000606084901b1673ffffffffffffffffffffffffffffffffffffffff8686031683614f99578673ffffffffffffffffffffffffffffffffffffffff16614f8683838973ffffffffffffffffffffffffffffffffffffffff16614df2565b81614f9357614f936167b0565b04614fdf565b614fdf614fbd83838973ffffffffffffffffffffffffffffffffffffffff166153c5565b8873ffffffffffffffffffffffffffffffffffffffff16808204910615150190565b979650505050505050565b805f811215610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b5f73ffffffffffffffffffffffffffffffffffffffff8481169086160360ff81901d908101186c010000000000000000000000006fffffffffffffffffffffffffffffffff851661506e818484614df2565b9350845f83858409111684019350505050949350505050565b5f808211615093575f80fd5b507f0706060506020500060203020504000106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be6fffffffffffffffffffffffffffffffff841160071b84811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b62bfffff8116610c6281612410565b5f80600284810b9086900b81810783139190050383156151c857600281900b60081d600181900b5f908152602089905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff808516908190039190911c9182168015159550909190856151aa57888360ff168603026151bd565b886151b482615087565b840360ff168603025b96505050505061524b565b6001908101600281900b60081d80830b5f90815260208a905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff841694851b01199081168015159550929391928561523157888360ff0360ff16860102615244565b888361523c836153f5565b0360ff168601025b9650505050505b5094509492505050565b5f80808062ffffff851673ffffffffffffffffffffffffffffffffffffffff808a16908b1610158288128015615338575f61529b8a5f0385620f424003620f4240614df2565b9050826152b4576152af8d8d8d600161501c565b6152c1565b6152c18c8e8d6001614ead565b96508681106152f5578b9750620f424084146152ec576152e7878586620f4240036153c5565b6152ee565b865b945061530e565b8096506153048d8c838661548f565b9750868a5f030394505b826153245761531f8d898d5f614ead565b615330565b615330888e8d5f61501c565b9550506153b6565b8161534e576153498c8c8c5f614ead565b61535a565b61535a8b8d8c5f61501c565b945084891061536b578a965061537d565b88945061537a8c8b87856154f3565b96505b816153945761538f8c888c600161501c565b6153a1565b6153a1878d8c6001614ead565b95506153b3868485620f4240036153c5565b93505b50505095509550955095915050565b5f6153d1848484614df2565b905081806153e1576153e16167b0565b838509156108385760010180610838575f80fd5b5f808211615401575f80fd5b507e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b15031604055f8290039091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff86161517156154cf57634f2461b85f526004601cfd5b816154e6576154e1858585600161554c565b612b0d565b612b0d85858560016156ae565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff861615171561553357634f2461b85f526004601cfd5b81615544576154e18585855f6156ae565b612b0d8585855f5b5f81156155f1575f73ffffffffffffffffffffffffffffffffffffffff84111561559f5761559a846c01000000000000000000000000876fffffffffffffffffffffffffffffffff16614df2565b6155bf565b6155bf6fffffffffffffffffffffffffffffffff8616606086901b6167dd565b90506155e96155e48273ffffffffffffffffffffffffffffffffffffffff8916615f92565b6157e3565b915050611d01565b5f73ffffffffffffffffffffffffffffffffffffffff84111561563d57615638846c01000000000000000000000000876fffffffffffffffffffffffffffffffff166153c5565b615663565b615663606085901b6fffffffffffffffffffffffffffffffff8716808204910615150190565b90508073ffffffffffffffffffffffffffffffffffffffff87161161568f57634323a5555f526004601cfd5b73ffffffffffffffffffffffffffffffffffffffff8616039050611d01565b5f825f036156bd575083611d01565b7bffffffffffffffffffffffffffffffff000000000000000000000000606085901b1682156157885773ffffffffffffffffffffffffffffffffffffffff861684810290858281615710576157106167b0565b040361574d5781810182811061574b57615741838973ffffffffffffffffffffffffffffffffffffffff16836153c5565b9350505050611d01565b505b506155e9818561577373ffffffffffffffffffffffffffffffffffffffff8a16836167dd565b61577d9190615f92565b808204910615150190565b73ffffffffffffffffffffffffffffffffffffffff86168481029085820414818311166157bc5763f5c787f15f526004601cfd5b8082036157416155e48473ffffffffffffffffffffffffffffffffffffffff8b16846153c5565b8073ffffffffffffffffffffffffffffffffffffffff81168114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d09565b73ffffffffffffffffffffffffffffffffffffffff81168114611784575f80fd5b5f806040838503121561585c575f80fd5b82356158678161582a565b946020939093013593505050565b5f60208284031215615885575f80fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114610838575f80fd5b5f805f606084860312156158c6575f80fd5b83356158d18161582a565b95602085013595506040909401359392505050565b5f805f606084860312156158f8575f80fd5b83356159038161582a565b925060208401356159138161582a565b929592945050506040919091013590565b5f60208284031215615934575f80fd5b5035919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040516080810167ffffffffffffffff8111828210171561598b5761598b61593b565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff811182821017156159d8576159d861593b565b604052919050565b803562ffffff81168114610c62575f80fd5b8035600281900b8114610c62575f80fd5b5f60a08284031215615a13575f80fd5b60405160a0810167ffffffffffffffff81118282101715615a3657615a3661593b565b6040529050808235615a478161582a565b81526020830135615a578161582a565b6020820152615a68604084016159e0565b6040820152615a79606084016159f2565b60608201526080830135615a8c8161582a565b6080919091015292915050565b5f8083601f840112615aa9575f80fd5b50813567ffffffffffffffff811115615ac0575f80fd5b602083019150836020828501011115615ad7575f80fd5b9250929050565b5f805f805f6101008688031215615af3575f80fd5b615afd8787615a03565b945060a0860135935060c0860135925060e086013567ffffffffffffffff811115615b26575f80fd5b615b3288828901615a99565b969995985093965092949392505050565b5f60208284031215615b53575f80fd5b81356108388161582a565b5f8060408385031215615b6f575f80fd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b81811015615bb5578351835260209384019390920191600101615b97565b509095945050505050565b5f8060208385031215615bd1575f80fd5b823567ffffffffffffffff811115615be7575f80fd5b615bf385828601615a99565b90969095509350505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f8060c08385031215615c63575f80fd5b615c6d8484615a03565b9150615c7b60a084016159e0565b90509250929050565b80358015158114610c62575f80fd5b5f8060408385031215615ca4575f80fd5b8235615caf8161582a565b9150615c7b60208401615c84565b5f805f80848603610140811215615cd2575f80fd5b615cdc8787615a03565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215615d0d575f80fd5b50615d16615968565b615d2260a087016159f2565b8152615d3060c087016159f2565b602082015260e086013560408201526101008601356060820152925061012085013567ffffffffffffffff811115615d66575f80fd5b615d7287828801615a99565b95989497509550505050565b5f8060c08385031215615d8f575f80fd5b615d998484615a03565b915060a0830135615da98161582a565b809150509250929050565b5f8060208385031215615dc5575f80fd5b823567ffffffffffffffff811115615ddb575f80fd5b8301601f81018513615deb575f80fd5b803567ffffffffffffffff811115615e01575f80fd5b8560208260051b8401011115615e15575f80fd5b6020919091019590945092505050565b5f8060408385031215615e36575f80fd5b8235615e418161582a565b91506020830135615da98161582a565b5f805f80848603610120811215615e66575f80fd5b615e708787615a03565b945060607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215615ea1575f80fd5b506040516060810167ffffffffffffffff81118282101715615ec557615ec561593b565b604052615ed460a08701615c84565b815260c0860135602082015260e0860135615eee8161582a565b6040820152925061010085013567ffffffffffffffff811115615d66575f80fd5b5f805f8060808587031215615f22575f80fd5b8435615f2d8161582a565b93506020850135615f3d8161582a565b93969395505050506040820135916060013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8181038181111561076557610765615f52565b8082018082111561076557610765615f52565b81835281816020850137505f602082840101525f60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b602081525f611d01602083018486615fa5565b5f6020828403121561600f575f80fd5b815167ffffffffffffffff811115616025575f80fd5b8201601f81018413616035575f80fd5b805167ffffffffffffffff81111561604f5761604f61593b565b61608060207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601615991565b818152856020838501011115616094575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b73ffffffffffffffffffffffffffffffffffffffff8716815261614c602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8460c08201528360e08201526101206101008201525f61617161012083018486615fa5565b98975050505050505050565b73ffffffffffffffffffffffffffffffffffffffff86168152616218602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351600290810b60c08301526020850151900b60e0820152604084015161010082015260608401516101208201526101606101408201525f614fdf61016083018486615fa5565b73ffffffffffffffffffffffffffffffffffffffff881681526162fa602082018873ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8551600290810b60c08301526020870151900b60e08201526040860151610100820152606086015161012082015284610140820152836101608201526101a06101808201525f6144a06101a083018486615fa5565b73ffffffffffffffffffffffffffffffffffffffff8416815260e081016163ee602083018573ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff831660c0830152949350505050565b73ffffffffffffffffffffffffffffffffffffffff8516815261010081016164b2602083018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff841660c08301528260020b60e083015295945050505050565b5f602082840312156164f0575f80fd5b5051919050565b73ffffffffffffffffffffffffffffffffffffffff86168152616592602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351151560c0820152602084015160e0820152604084015173ffffffffffffffffffffffffffffffffffffffff166101008201526101406101208201525f614fdf61014083018486615fa5565b8082018281125f8312801582168215821617156165fe576165fe615f52565b505092915050565b73ffffffffffffffffffffffffffffffffffffffff871681526166a1602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8451151560c0820152602085015160e0820152604085015173ffffffffffffffffffffffffffffffffffffffff16610100820152836101208201526101606101408201525f61617161016083018486615fa5565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81137fffffffffffffffffffffffffffffffff800000000000000000000000000000008212171561076557610765615f52565b600f82810b9082900b037fffffffffffffffffffffffffffffffff8000000000000000000000000000000081126f7fffffffffffffffffffffffffffffff8213171561076557610765615f52565b8181035f83128015838313168383128216171561398f5761398f615f52565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82616810577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b50049056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xF4W_5`\xE0\x1C\x80cZk\xCF\xDA\x11a\x01\x17W\x80c\xA5\x84\x11\x94\x11a\0\xACW\x80c\xF15\xBA\xAA\x11a\0|W\x80c\xF3\xCD\x91L\x11a\0bW\x80c\xF3\xCD\x91L\x14a\x06vW\x80c\xF5)\x8A\xCA\x14a\x06\x95W\x80c\xFE\x99\x04\x9A\x14a\x06\xB4W_\x80\xFD[\x80c\xF15\xBA\xAA\x14a\x068W\x80c\xF2\xFD\xE3\x8B\x14a\x06WW_\x80\xFD[\x80c\xA5\x84\x11\x94\x14a\x05\x95W\x80c\xB66<\xF2\x14a\x05\xB4W\x80c\xDB\xD05\xFF\x14a\x05\xEDW\x80c\xF0-\xE3\xB2\x14a\x06\x0CW_\x80\xFD[\x80c\x81a\xB8t\x11a\0\xE7W\x80c\x81a\xB8t\x14a\x04\xDCW\x80c\x8D\xA5\xCB[\x14a\x04\xFBW\x80c\x97\xE8\xCDN\x14a\x05KW\x80c\x9B\xF6d_\x14a\x05vW_\x80\xFD[\x80cZk\xCF\xDA\x14a\x048W\x80cbv\xCB\xBE\x14a\x04lW\x80c~\x87\xCE}\x14a\x04\x9EW\x80c\x80\xF0\xB4L\x14a\x04\xBDW_\x80\xFD[\x80c-w\x13\x89\x11a\x01\x8DW\x80cH\xC8\x94\x91\x11a\x01]W\x80cH\xC8\x94\x91\x14a\x03\x92W\x80cRu\x96Q\x14a\x03\xBEW\x80cU\x8Ar\x97\x14a\x03\xDDW\x80cY\x8A\xF9\xE7\x14a\x03\xFCW_\x80\xFD[\x80c-w\x13\x89\x14a\x03\x15W\x80c5\xFDc\x1A\x14a\x034W\x80c=\xD4Z\xDB\x14a\x03`W\x80cBj\x84\x93\x14a\x03sW_\x80\xFD[\x80c\x11\xDA`\xB4\x11a\x01\xC8W\x80c\x11\xDA`\xB4\x14a\x02\xB0W\x80c\x15n)\xF6\x14a\x02\xB8W\x80c\x1E.\xAE\xAF\x14a\x02\xD7W\x80c#Bf\xD7\x14a\x02\xF6W_\x80\xFD[\x80b\xFD\xD5\x8E\x14a\x01\xF8W\x80c\x01\xFF\xC9\xA7\x14a\x02AW\x80c\t[\xCD\xB6\x14a\x02pW\x80c\x0B\r\x9C\t\x14a\x02\x8FW[_\x80\xFD[4\x80\x15a\x02\x03W_\x80\xFD[Pa\x02.a\x02\x126`\x04aXKV[`\x04` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02LW_\x80\xFD[Pa\x02`a\x02[6`\x04aXuV[a\x06\xD3V[`@Q\x90\x15\x15\x81R` \x01a\x028V[4\x80\x15a\x02{W_\x80\xFD[Pa\x02`a\x02\x8A6`\x04aX\xB4V[a\x07kV[4\x80\x15a\x02\x9AW_\x80\xFD[Pa\x02\xAEa\x02\xA96`\x04aX\xE6V[a\x08?V[\0[a\x02.a\x08\xC9V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02\xAEa\x02\xD26`\x04aX\xB4V[a\t'V[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x02.a\x02\xF16`\x04aY$V[a\t\xABV[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x02.a\x03\x106`\x04aZ\xDEV[a\t\xB5V[4\x80\x15a\x03 W_\x80\xFD[Pa\x02\xAEa\x03/6`\x04a[CV[a\n\xD9V[4\x80\x15a\x03?W_\x80\xFD[Pa\x03Sa\x03N6`\x04a[^V[a\x0B\xCCV[`@Qa\x028\x91\x90a[~V[a\x02.a\x03n6`\x04a[CV[a\x0C\tV[4\x80\x15a\x03~W_\x80\xFD[Pa\x02`a\x03\x8D6`\x04aX\xB4V[a\x0CgV[4\x80\x15a\x03\x9DW_\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04a[\xC0V[a\x0C\xD8V[`@Qa\x028\x91\x90a[\xFFV[4\x80\x15a\x03\xC9W_\x80\xFD[Pa\x02\xAEa\x03\xD86`\x04a\\RV[a\x0E*V[4\x80\x15a\x03\xE8W_\x80\xFD[Pa\x02`a\x03\xF76`\x04a\\\x93V[a\x0E\xCCV[4\x80\x15a\x04\x07W_\x80\xFD[Pa\x02.a\x04\x166`\x04aX\xE6V[`\x05` \x90\x81R_\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T\x81V[4\x80\x15a\x04CW_\x80\xFD[Pa\x04Wa\x04R6`\x04a\\\xBDV[a\x0FfV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x028V[4\x80\x15a\x04wW_\x80\xFD[Pa\x04\x8Ba\x04\x866`\x04a]~V[a\x11eV[`@Q`\x02\x91\x90\x91\x0B\x81R` \x01a\x028V[4\x80\x15a\x04\xA9W_\x80\xFD[Pa\x02\xAEa\x04\xB86`\x04a\\RV[a\x13\xFDV[4\x80\x15a\x04\xC8W_\x80\xFD[Pa\x02\xAEa\x04\xD76`\x04aXKV[a\x14\xEEV[4\x80\x15a\x04\xE7W_\x80\xFD[Pa\x02.a\x04\xF66`\x04aX\xE6V[a\x15\xADV[4\x80\x15a\x05\x06W_\x80\xFD[P_Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x028V[4\x80\x15a\x05VW_\x80\xFD[Pa\x02.a\x05e6`\x04a[CV[`\x01` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\x81W_\x80\xFD[Pa\x03Sa\x05\x906`\x04a]\xB4V[a\x16\xD9V[4\x80\x15a\x05\xA0W_\x80\xFD[Pa\x02\xAEa\x05\xAF6`\x04a[CV[a\x17\x12V[4\x80\x15a\x05\xBFW_\x80\xFD[Pa\x02`a\x05\xCE6`\x04a^%V[`\x03` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xF8W_\x80\xFD[Pa\x03Sa\x06\x076`\x04a]\xB4V[a\x17\xB7V[4\x80\x15a\x06\x17W_\x80\xFD[P`\x02Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x06CW_\x80\xFD[Pa\x02.a\x06R6`\x04aY$V[a\x17\xEEV[4\x80\x15a\x06bW_\x80\xFD[Pa\x02\xAEa\x06q6`\x04a[CV[a\x17\xF8V[4\x80\x15a\x06\x81W_\x80\xFD[Pa\x02.a\x06\x906`\x04a^QV[a\x18\xE7V[4\x80\x15a\x06\xA0W_\x80\xFD[Pa\x02\xAEa\x06\xAF6`\x04aX\xB4V[a\x1A\x99V[4\x80\x15a\x06\xBFW_\x80\xFD[Pa\x02`a\x06\xCE6`\x04a_\x0FV[a\x1B\x1DV[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14\x80a\x07eWP\x7F\x0Fc/\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x91\x90\x83\x90a\x07\x96\x90\x84\x90a_\x7FV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x07\xDA\x90\x84\x90a_\x92V[\x90\x91UPP`@\x80Q3\x80\x82R` \x82\x01\x85\x90R\x85\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x92\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4P`\x01[\x93\x92PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x08\x8EWa\x08\x8E\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x08\xA3\x83a\x08\x9B\x83a\x1D\x11V[_\x033a\x1DVV[a\x08\xC4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x1D\xB6V[PPPV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\t\x19Wa\t\x19\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\t\"3a\x1E\xB1V[\x90P\x90V[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\tvWa\tv\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x81a\t\x84\x81a\x08\x9B\x84a\x1D\x11V[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a\x1F\x9AV[PPPPV[_\x81T_R` _\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\n\x05Wa\n\x05\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\n\ra 9V[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\n'\x81a \xA1V[`\x80\x88\x01Qa\nP\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a \xE8V[a\n[\x81\x88\x88a!\xCBV[\x92Pa\nh\x88\x843a\"\xC5V[`@\x80Q\x88\x81R` \x81\x01\x88\x90R3\x91\x84\x91\x7F)\xEF\x05\xCA\xAF\xF9@K|\xB6\xD1\xC0\xE9\xBB\xAE\x9E\xAAz\xB2T\x1F\xEB\xA1\xA9\xC4$\x85\x94\xC0\x81V\xCB\x91\x01`@Q\x80\x91\x03\x90\xA3`\x80\x88\x01Qa\n\xCE\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a\"\xEDV[PP\x95\x94PPPPPV[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0B^W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB4\xBD\x8E\xF5=\xF6\x90\xB9\x94=3\x18\x99`\x06\xDB\xB8*%\xF5G\x19\xD8\xC8\x03[Qj*[\x8A\xCC\x90_\x90\xA2PV[```@Q\x80\x83`\x05\x1B` \x83R\x84` \x84\x01R`@\x83\x01\x92P\x80\x83\x01\x90P[\x85T\x83R` \x83\x01\x92P`\x01\x86\x01\x95P\x80\x83\x10a\x0B\xECW\x81\x81\x03\x82\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0CYWa\x0CY\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x07e\x82a\x1E\xB1V[\x91\x90PV[3_\x81\x81R`\x05` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x80\x85R\x90\x83R\x81\x84 \x87\x85R\x90\x92R\x80\x83 \x85\x90UQ\x91\x92\x85\x92\x7F\xB3\xFDPq\x83X\x87Vz\x06q\x15\x11!\x89M\xDC\xCC(B\xF1\xD1\x0B\xED\xAD\x13\xE0\xD1|\xAC\xE9\xA7\x90a\x08,\x90\x87\x81R` \x01\x90V[``\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\r*Wa\r*\x7FP\x90\xD6\xC6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\r2a#\xC5V[`@Q\x7F\x91\xDDsF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90c\x91\xDDsF\x90a\rp\x90\x86\x90\x86\x90`\x04\x01a_\xECV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\r\xD0\x91\x90\x81\x01\x90a_\xFFV[\x90P\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\\x15a\x0E\"Wa\x0E\"\x7FR\x12\xCB\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x07ea#\xEBV[`@\x82\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x80a\x0EuWP\x81`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x0E\xA3Wa\x0E\xA3\x7F0\xD2\x16A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x0E\xB1\x81b\xFF\xFF\xFF\x16a$\x10V[`\xA0\x82 _\x81\x81R`\x06` R`@\x90 a\x08\xC4\x90\x83a$OV[3_\x81\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x85R\x90\x83R\x81\x84 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x87\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x92\x93\x91\x7F\xCE\xB5v\xD9\xF1^N \x0F\xDBP\x96\xD6M]\xFDf~\x16\xDE\xF2\x0C\x1E\xEF\xD1BV\xD8\xE3\xFA\xA2g\x91\x01`@Q\x80\x91\x03\x90\xA3P`\x01\x92\x91PPV[_\x80\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0F\xB7Wa\x0F\xB7\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x0F\xBFa 9V[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\x0F\xD9\x81a \xA1V[`\x80\x88\x01Qa\x10\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89a$\xA8V[_a\x10u`@Q\x80`\xC0\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A_\x01Q`\x02\x0B\x81R` \x01\x8A` \x01Q`\x02\x0B\x81R` \x01a\x10R\x8B`@\x01Qa&iV[`\x0F\x0B\x81R``\x80\x8D\x01Q`\x02\x0B` \x83\x01R\x8B\x01Q`@\x90\x91\x01R\x83\x90a&\x9EV[\x94P\x90Pa\x10\x83\x81\x85a*\xE3V[\x94PPP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xF2\x08\xF4\x91'\x82\xFD%\xC7\xF1\x14\xCA7#\xA2\xD5\xDDo;\xCC:\xC8\xDBZ\xF6;\xAA\x85\xF7\x11\xD5\xEC\x88_\x01Q\x89` \x01Q\x8A`@\x01Q\x8B``\x01Q`@Qa\x11\x01\x94\x93\x92\x91\x90`\x02\x94\x85\x0B\x81R\x92\x90\x93\x0B` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x80\x87\x01Q_\x90a\x115\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x87\x87\x8B\x8Ba+\x16V[\x90\x94P\x90P\x80\x15a\x11OWa\x11O\x88\x82\x8A`\x80\x01Qa\"\xC5V[a\x11Z\x88\x853a\"\xC5V[PP\x94P\x94\x92PPPV[_a\x11na 9V[``\x83\x01Qa\x7F\xFF`\x02\x91\x90\x91\x0B\x13\x15a\x11\xB2W``\x83\x01Qa\x11\xB2\x90\x7F\xB7\0$\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a,\xEDV[`\x01`\x02\x0B\x83``\x01Q`\x02\x0B\x12\x15a\x11\xF5W``\x83\x01Qa\x11\xF5\x90\x7F\xE9\xE9\x05\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a,\xEDV[\x82Q` \x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x10a\x12MW\x82Q` \x84\x01Qa\x12M\x91\x7Fnl\x980\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a,\xFCV[a\x12~\x83`@\x01Q\x84`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a-?\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x12\xB2W`\x80\x83\x01Qa\x12\xB2\x90\x7F\xE6Z\xF6\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\rV[_a\x12\xC5\x84`@\x01Qb\xFF\xFF\xFF\x16a./V[`\x80\x85\x01Q\x90\x91Pa\x12\xEE\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85\x85a.TV[`\xA0\x84 _\x81\x81R`\x06` R`@\x90 a\x13\n\x90\x85\x84a/'V[`\x80\x86\x01Q\x90\x93Pa\x134\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86\x86\x86a/\xDFV[\x84` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x7F\xDDFngN\xA5W\xF5b\x95\xE2\xD0!\x8A\x12^\xA4\xB4\xF0\xF6\xF30{\x95\xF8^a\x10\x83\x8Dd8\x88`@\x01Q\x89``\x01Q\x8A`\x80\x01Q\x8A\x8A`@Qa\x13\xED\x95\x94\x93\x92\x91\x90b\xFF\xFF\xFF\x95\x90\x95\x16\x85R`\x02\x93\x84\x0B` \x86\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`@\x86\x01R\x91\x16``\x84\x01R\x90\x0B`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA4PP\x92\x91PPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x14EWa\x14E\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x03\xE9a\x0F\xFF\x82\x16\x10b>\x90\0b\xFF\xF0\0\x83\x16\x10\x16a\x14\x8DWa\x14\x8D\x7F\xA7\xAB\xE2\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\rV[`\xA0\x82 a\x14\xAF\x82a\x14\xA9\x83_\x90\x81R`\x06` R`@\x90 \x90V[\x90a0\xB4V[`@Qb\xFF\xFF\xFF\x83\x16\x81R\x81\x90\x7F\xE9\xC4%\x93\xE7\x1F\x84@;\x845,\xD1h\xD6\x93\xE2\xC9\xFC\xD1\xFD\xBC\xC3\xFE\xB2\x1D\x92\xB4>f\x96\xF9\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x15=Wa\x15=\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[3_\x90\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` R`@\x81 \\\x90a\x15k\x83a\x1D\x11V[\x90P\x81\x81`\x0F\x0B\x14a\x15\xA0Wa\x15\xA0\x7F\xBD\xA7:\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\t\xA5\x84\x82_\x033a\x1DVV[`\x02T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x15\xF7Wa\x15\xF7\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\x16GWa\x16G\x7F>_O\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x81\x15a\x16SW\x81a\x16yV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x01` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x16\xB2\x90\x84\x90a_\x7FV[\x90\x91UPa\x088\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x85\x83a\x1D\xB6V[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805\\\x84R` \x93\x84\x01\x93\x01\x81\x84\x10a\x16\xF7W[P\x81\x81\x03\x82\xF3[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x17aWa\x17a\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x17\x87Wa\x17\x84a1\x08V[PV[_a\x17\xA7\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a1-V[\x90Pa\x17\xB3\x82\x82a1\xDCV[PPV[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805T\x84R` \x93\x84\x01\x93\x01\x81\x84\x10\x15a\x17\x0BWa\x17\xD5V[_\x81\\_R` _\xF3[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x18xW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0BUV[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x197Wa\x197\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\x19?a 9V[\x83` \x01Q_\x03a\x19sWa\x19s\x7F\xBE\x8B\x85\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[`\xA0\x85 _\x81\x81R`\x06` R`@\x90 a\x19\x8D\x81a \xA1V[`\x80\x87\x01Q_\x90\x81\x90\x81\x90a\x19\xBB\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8B\x8B\x8B\x8Ba2<V[\x80\x93P\x81\x95P\x82\x94PPPPa\x1A7\x84\x86`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x8E``\x01Q`\x02\x0B\x81R` \x01\x8D_\x01Q\x15\x15\x81R` \x01\x8D`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85b\xFF\xFF\xFF\x16\x81RP\x8C_\x01Qa\x1A0W\x8D` \x01Qa3\xE3V[\x8DQa3\xE3V[`\x80\x8B\x01Q\x90\x96P_\x92Pa\x1Ah\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A\x8A\x88\x8B\x8B\x88a4\xE4V[\x90\x95P\x90P\x80\x15a\x1A\x82Wa\x1A\x82\x89\x82\x8B`\x80\x01Qa\"\xC5V[a\x1A\x8D\x89\x863a\"\xC5V[PPPP\x94\x93PPPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x1A\xE8Wa\x1A\xE8\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x81a\x1A\xFC\x81a\x1A\xF6\x84a\x1D\x11V[3a\x1DVV[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a6}V[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x14\x80\x15\x90a\x1BtWPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15[\x15a\x1C\x1DWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x86\x84R\x90\x91R\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x1C\x1BWa\x1B\xE2\x83\x82a_\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x88\x84R\x90\x91R\x90 U[P[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1C\\\x90\x84\x90a_\x7FV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1C\xA0\x90\x84\x90a_\x92V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x84\x90R\x84\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x92\x90\x89\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01`@Q\x80\x91\x03\x90\xA4P`\x01[\x94\x93PPPPV[\x80_R`\x04_\xFD[_o\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x1DRWa\x1DR\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[P\x90V[\x81`\x0F\x0B_\x03a\x1DeWPPPV[_\x80a\x1D\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x86a7\x8EV[\x91P\x91P\x80_\x03a\x1D\xA0Wa\x1D\x9Ba7\xD4V[a\x1D\xAFV[\x81_\x03a\x1D\xAFWa\x1D\xAFa8\"V[PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x1E\x10W_\x80_\x80\x85\x87Z\xF1\x90P\x80a\x1E\x0BWa\x1E\x0B\x7F\x85I\xDBY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a8pV[a\t\xA5V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91P_\x81R_` \x82\x01R_`@\x82\x01RP\x80a\t\xA5Wa\t\xA5\x7F\xB1,_\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a8pV[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1E\xF7W4\x91Pa\x1F\x81V[4\x15a\x1F&Wa\x1F&\x7F\xB0\xEC\x84\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95\\_a\x1Fhs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a1-V[\x90Pa\x1Ft\x82\x82a_\x7FV[\x93Pa\x1F~a1\x08V[PP[a\x1F\x94\x81a\x1F\x8E\x84a\x1D\x11V[\x85a\x1DVV[P\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x1F\xD9\x90\x84\x90a_\x92V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91_\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a \x9FWa \x9F\x7F\r\x89C\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x17\x84Wa\x17\x84\x7FHj\xA3\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xC2W` \x87\x16\x15a!\xC2Wa!\xC03\x87\x87\x87\x87\x87`@Q`$\x01a!-\x96\x95\x94\x93\x92\x91\x90a`\xB1V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\xA8\xB0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a8\xA3V[P[PPPPPPPV[`\x03\x83\x01T_\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x03a\"\x14Wa\"\x14\x7F\xA7O\x97\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a\"Ka\" \x85a\x1D\x11V[_\x03a\"+\x85a\x1D\x11V[_\x03`\x80\x91\x90\x91\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x91P\x83\x15a\"\x85W`\x01\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x02\x04\x01\x90U[\x82\x15a\"\xBDW`\x02\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x02\x04\x01\x90U[P\x93\x92PPPV[\x82Qa\"\xDB\x90a\"\xD5\x84`\x80\x1D\x90V[\x83a\x1DVV[a\x08\xC4\x83` \x01Qa\"\xD5\x84`\x0F\x0B\x90V[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xC2W`\x10\x87\x16\x15a!\xC2Wa!\xC03\x87\x87\x87\x87\x87`@Q`$\x01a#2\x96\x95\x94\x93\x92\x91\x90a`\xB1V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE1\xB4\xAFi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a8\xA3V[`\x01\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[b\x0FB@b\xFF\xFF\xFF\x82\x16\x11\x15a\x17\x84Wa\x17\x84\x7F\x14\0!\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\rV[a$X\x82a \xA1V[\x81T\x7F\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16\x17[\x90\x91UPV[\x843s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a&aW_\x84`@\x01Q\x13\x80\x15a$\xDCWPa\x08\0\x86\x16\x15\x15[\x15a%\x96Wa%\x903\x86\x86\x86\x86`@Q`$\x01a$\xFD\x95\x94\x93\x92\x91\x90aa}V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a8\xA3V[Pa&aV[_\x84`@\x01Q\x13\x15\x80\x15a%\xADWPa\x02\0\x86\x16\x15\x15[\x15a&aWa!\xC23\x86\x86\x86\x86`@Q`$\x01a%\xCE\x95\x94\x93\x92\x91\x90aa}V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a8\xA3V[PPPPPPV[\x80`\x0F\x81\x90\x0B\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[``\x81\x01Q` \x82\x01Q`@\x83\x01Q_\x92\x83\x92\x90\x91a&\xBD\x82\x82a9\x96V[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x83`\x0F\x0B_\x14a(\x8DWa&\xF6\x88\x84\x86_a:]V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01R\x15\x15\x81Ra'\x1E\x88\x83\x86`\x01a:]V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x83\x01R\x15\x15`@\x82\x01R_`\x0F\x85\x90\x0B\x12a(RW`\x80\x87\x01Q_\x90`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x81\x81\x07\x83\x13\x90\x82\x90\x05\x03b\r\x89\xE8\x91\x90\x91\x05\x03`\x01\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a'\xF6Wa'\xF6\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a,\xEDV[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82``\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a(PWa(P\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a,\xEDV[P[\x80Q\x15a(nW`\x80\x87\x01Qa(n\x90`\x05\x8A\x01\x90\x85\x90a;FV[\x80`@\x01Q\x15a(\x8DW`\x80\x87\x01Qa(\x8D\x90`\x05\x8A\x01\x90\x84\x90a;FV[_\x80a(\x9A\x8A\x86\x86a;\x98V[\x8AQ`\xA0\x8C\x01Q`@\x80Q`&\x81\x01\x92\x90\x92R`\x06\x80\x83\x01\x8A\x90R`\x03\x83\x01\x8B\x90R\x92\x82R`:`\x0C\x83\x01 _\x83\x83\x01\x81\x90R` \x80\x85\x01\x82\x90R\x93\x81\x90R\x90\x81R\x92\x8F\x01\x90\x91R\x81 \x92\x94P\x90\x92P\x80a(\xF7\x83\x8A\x87\x87a<LV[\x91P\x91Pa),a)\x07\x83a\x1D\x11V[a)\x10\x83a\x1D\x11V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x99PPPPPP_\x84`\x0F\x0B\x12\x15a)\x95W\x80Q\x15a)hW`\x02\x83\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[\x80`@\x01Q\x15a)\x95W`\x02\x82\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[P\x82`\x0F\x0B_\x14a*\xD9W\x86T_\x80a)\xB1\x83`\xA0\x1C`\x02\x0B\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x91P\x91P\x84`\x02\x0B\x82`\x02\x0B\x12\x15a*\rWa*\x06a*\0a)\xFBa)\xEC\x88a=}V[a)\xF5\x88a=}V[\x8Aa@zV[a&iV[`\x80\x1B\x90V[\x97Pa*\xD5V[\x83`\x02\x0B\x82`\x02\x0B\x12\x15a*\xB0Wa*Da*.a)\xFB\x83a)\xF5\x88a=}V[a)\x10a)\xFBa*=\x89a=}V[\x85\x8Ba@\xB2V[`\x03\x8B\x01T\x90\x98Pa*h\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87a@\xDEV[`\x03\x8B\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua*\xD5V[a*\xD2_a)\x10a)\xFBa*\xC3\x89a=}V[a*\xCC\x89a=}V[\x8Ba@\xB2V[\x97P[PPP[PPP\x92P\x92\x90PV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a+\ra+\x04\x83a&iV[a)\x10\x83a&iV[\x95\x94PPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a+?WP\x84\x90P_a,\xE1V[\x85\x91P_\x87`@\x01Q\x13\x15a,'Wa\x04\0\x89\x16\x15a,\"Wa,\x133\x89\x89\x89\x89\x89\x89`@Q`$\x01a+x\x97\x96\x95\x94\x93\x92\x91\x90ab_V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x9F\x06>\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x02\x8B\x16\x15\x15[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x91\x90aA\x0EV[\x90Pa,\x1F\x82\x82aAhV[\x91P[a,\xE1V[a\x01\0\x89\x16\x15a,\xE1Wa,\xD23\x89\x89\x89\x89\x89\x89`@Q`$\x01a,Q\x97\x96\x95\x94\x93\x92\x91\x90ab_V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fl+\xBE~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x01\x8B\x16\x15\x15a+\xF5V[\x90Pa,\xDE\x82\x82aAhV[\x91P[\x97P\x97\x95PPPPPPV[\x81_R\x80`\x02\x0B`\x04R`$_\xFD[`@Q\x83\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x81\xFD[_`\x80\x83\x16\x15\x80\x15a-SWP`\x08\x83\x16\x15\x15[\x15a-_WP_a\x07eV[`@\x83\x16\x15\x80\x15a-rWP`\x04\x83\x16\x15\x15[\x15a-~WP_a\x07eV[a\x04\0\x83\x16\x15\x80\x15a-\x92WP`\x02\x83\x16\x15\x15[\x15a-\x9EWP_a\x07eV[a\x01\0\x83\x16\x15\x80\x15a-\xB2WP`\x01\x83\x16\x15\x15[\x15a-\xBEWP_a\x07eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15a-\xFCWa?\xFF\x83\x16\x15\x15\x80a-\xF7WPb\x80\0\0b\xFF\xFF\xFF\x83\x16\x14a\x088V[a\x088V[Pb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x91\x90PV[\x81_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x04R`$_\xFD[_b\x80\0\0b\xFF\xFF\xFF\x83\x16\x03a.FWP_\x91\x90PV[a\x1DR\x82b\xFF\xFF\xFF\x16a$\x10V[\x823s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a\t\xA5Wa \0\x84\x16\x15a\t\xA5Wa\x1D\xAF3\x84\x84`@Q`$\x01a.\x94\x93\x92\x91\x90acOV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xDC\x985N\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x90a8\xA3V[\x82T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a/oWa/o\x7Fy\x83\xC0Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a/x\x83aA\x89V[\x90P|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\xA0\x83\x90\x1Bv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x17\x90\x93UP\x90\x91\x90PV[\x833s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a\x1D\xAFWa\x10\0\x85\x16\x15a\x1D\xAFWa&a3\x85\x85\x85`@Q`$\x01a0!\x94\x93\x92\x91\x90ad\x12V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fo\xE7\xE6\xEB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x90a8\xA3V[a0\xBD\x82a \xA1V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16y\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xB8\x83\x90\x1B\x16\x17a$\xA2V[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a1PWPG\x91\x90PV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a1\xB8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07e\x91\x90ad\xE0V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]\x80\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95]PPV[` \x83\x01Q_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x163\x03a2hW_\x91Pa3\xD8V[`\x80\x88\x16\x15a3\xD8W_a3\n\x893\x8A\x8A\x8A\x8A`@Q`$\x01a2\x8F\x95\x94\x93\x92\x91\x90ad\xF7V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FW^$\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra8\xA3V[\x90P\x80Q``\x14a3>Wa3>\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[`@\x88\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x03a3YW``\x81\x01Q\x91P[`\x08\x89\x16\x15a3\xD6W`@\x81\x01Q\x92P_a3t\x84`\x80\x1D\x90V[\x90P\x80`\x0F\x0B_\x14a3\xD4W_\x85\x12a3\x91`\x0F\x83\x90\x0B\x87ae\xDFV[\x95P\x80a3\xA0W_\x86\x12a3\xA4V[_\x86\x13[\x15a3\xD2Wa3\xD2\x7F\xFA\x0Bq\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[P[P[P[\x95P\x95P\x95\x92PPPV[_\x80\x80\x80\x80a3\xF2\x89\x88aD\xADV[\x93P\x93P\x93P\x93P_\x83\x11\x15a4-Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16_\x90\x81R`\x01` R`@\x90 \x80T\x84\x01\x90U[3\x88\x7F@\xE9\xCE\xCB\x9F_\x1F\x1C[\x9C\x97\xDE\xC2\x91{~\xE9.W\xBAUcp\x8D\xAC\xA9M\xD8J\xD7\x11/a4Z\x87`\x80\x1D\x90V[a4d\x88`\x0F\x0B\x90V[\x85Q`@\x80\x88\x01Q` \x80\x8A\x01Q\x83Q`\x0F\x97\x88\x0B\x81R\x95\x90\x96\x0B\x90\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x90\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x02\x91\x90\x91\x0B`\x80\x82\x01Rb\xFF\xFF\xFF\x86\x16`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90\xA3P\x91\x97\x96PPPPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a5\rWP\x84\x90P_a,\xE1V[_a5\x18\x84`\x80\x1D\x90V[\x90P_a5%\x85`\x0F\x0B\x90V[\x90P`@\x8B\x16\x15a5\xF8Wa5\xEBa)\xFB3\x8C\x8C\x8C\x8C\x8C`@Q`$\x01a5Q\x96\x95\x94\x93\x92\x91\x90af\x06V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB4{/\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x04\x8E\x16\x15\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x16\x91\x90aA\x0EV[a5\xF5\x90\x82af\xF5V[\x90P[_\x81`\x0F\x0B_\x14\x15\x80a6\x0EWP\x82`\x0F\x0B_\x14\x15[\x15a6kW\x89Q` \x8B\x01Q_\x13\x90\x15\x15\x14a6BWo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x90\x1B\x17a6\\V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x80\x84\x90\x1B\x17[\x90Pa6h\x89\x82aAhV[\x98P[\x97\x9B\x97\x9AP\x96\x98PPPPPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x14\x80\x15\x90a6\xD6WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T`\xFF\x16\x15[\x15a7\x83Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x86\x83R\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a7\x81Wa7F\x83\x82a_\x7FV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x81R\x82\x82 \x88\x83R\x90R U[P[a\t\xA5\x84\x84\x84aM?V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16_\x90\x81R\x90\x84\x16` R`@\x81 \x80\\\x91\x90a7\xC6`\x0F\x85\x90\x0B\x84ae\xDFV[\x91P\x81\x81]P\x93P\x93\x91PPV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x03\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x01\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[=`@Q\x83\x81R\x82`\x04\x82\x01R`@`$\x82\x01R\x81`D\x82\x01R\x81_`d\x83\x01>` \x80`\x1F\x84\x01\x04\x02`d\x01\x91P\x81\x81\xFD[``_\x80_\x84Q` \x86\x01_\x88Z\xF1\x90P\x80a8\xE3Wa8\xE3\x7F1\x9DT\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a8pV[`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>` \x82Q\x10\x80a9aWP` \x83\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a9<\x83` \x01Q\x90V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15[\x15a9\x8FWa9\x8F\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[P\x92\x91PPV[\x80`\x02\x0B\x82`\x02\x0B\x12a9\xCEWa9\xCE\x7F\xC4C>\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x83aM\xD5V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18`\x02\x83\x90\x0B\x12\x15a:$Wa:$\x7F\xD5\xE2\xF7\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a,\xEDV[b\r\x89\xE8`\x02\x82\x90\x0B\x13\x15a\x17\xB3Wa\x17\xB3\x7F\x1A\xD7w\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a,\xEDV[`\x02\x83\x90\x0B_\x90\x81R`\x04\x85\x01` R`@\x81 \x80T\x82\x91\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0Ba:\xAB\x82\x88a@\xDEV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x15\x90\x84\x16\x80\x15\x91\x90\x91\x14\x15\x96P\x90\x94P_\x03a:\xFEW\x88T`\xA0\x1C`\x02\x0B`\x02\x0B\x88`\x02\x0B\x13a:\xFEW`\x01\x80\x8A\x01T\x90\x84\x01U`\x02\x80\x8A\x01T\x90\x84\x01U[_\x86a;\x13Wa;\x0E\x88\x83af\xF5V[a;\x1DV[a;\x1D\x88\x83agCV[\x90P\x80`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x17\x84UPPPP\x94P\x94\x92PPPV[`\x02\x91\x82\x0B\x91\x0B\x80\x82\x07\x15a;sW`@Qc\xD4\xD8\xF3\xE6\x81R\x82` \x82\x01R\x81`@\x82\x01R`D`\x1C\x82\x01\xFD[\x80\x82\x05\x91P\x81`\x08\x1D_R\x82` R`@_ `\x01`\xFF\x84\x16\x1B\x81T\x18\x81UPPPPV[`\x02\x82\x81\x0B_\x81\x81R`\x04\x86\x01` R`@\x80\x82 \x85\x85\x0B\x83R\x90\x82 \x87T\x92\x94\x85\x94\x92\x93\x91\x92`\xA0\x92\x90\x92\x1C\x90\x0B\x90\x81\x12\x15a;\xEEW\x81`\x01\x01T\x83`\x01\x01T\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x03\x93Pa<AV[\x85`\x02\x0B\x81`\x02\x0B\x12a<\x1AW\x82`\x01\x01T\x82`\x01\x01T\x03\x94P\x82`\x02\x01T\x82`\x02\x01T\x03\x93Pa<AV[\x81`\x01\x01T\x83`\x01\x01T\x89`\x01\x01T\x03\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x89`\x02\x01T\x03\x03\x93P[PPP\x93P\x93\x91PPV[\x83T_\x90\x81\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0F\x86\x90\x0B\x82\x03a<\xB6W\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a<\xB1Wa<\xB1\x7F\xAE\xFE\xB9$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[a<\xFDV[a<\xC0\x81\x87a@\xDEV[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x87U[a=1\x87`\x01\x01T\x86\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aM\xF2V[\x92Pa=g\x87`\x02\x01T\x85\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aM\xF2V[`\x01\x88\x01\x95\x90\x95UPP`\x02\x90\x94\x01U\x91\x92\x90PV[`\x02\x0B_`\xFF\x82\x90\x1D\x80\x83\x01\x18b\r\x89\xE8\x81\x11\x15a=\xBFWa=\xBF\x7F\x8B\x862z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a,\xEDV[p\x01\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01`\x01\x82\x16\x02p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18`\x02\x82\x16\x15a>\x08Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a>'Wo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a>FWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a>eWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a>\x84Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a>\xA3Wo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a>\xC2Wo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a>\xE2Wo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a?\x02Wo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a?\"Wo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a?BWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15a?bWo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15a?\x82Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15a?\xA2Wop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15a?\xC2Wo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15a?\xE3Wo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15a@\x03Wn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15a@\"Wm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15a@?Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[_\x84\x13\x15a@jW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04[c\xFF\xFF\xFF\xFF\x01` \x1C\x93\x92PPPV[_\x80\x82`\x0F\x0B\x12a@\xA1Wa@\x9Aa@\x95\x85\x85\x85`\x01aN\xADV[aO\xEAV[_\x03a\x1D\x01V[a\x1D\x01a@\x95\x85\x85\x85_\x03_aN\xADV[_\x80\x82`\x0F\x0B\x12a@\xCDWa@\x9Aa@\x95\x85\x85\x85`\x01aP\x1CV[a\x1D\x01a@\x95\x85\x85\x85_\x03_aP\x1CV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x0F\x82\x90\x0B\x01`\x80\x81\x90\x1C\x15a\x07eWc\x93\xDA\xFD\xF1_R`\x04`\x1C\xFD[_\x80aA\x1A\x85\x85a8\xA3V[\x90P\x82aA*W_\x91PPa\x088V[\x80Q`@\x14aA\\WaA\\\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[`@\x01Q\x94\x93PPPPV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x03`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x03a+\ra+\x04\x83a&iV[_s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DQc\x96\x16\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xFF\xFD\x89]\x83\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aB\x08WaB\x08\x7FaHu$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a.\rV[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0` \x83\x90\x1B\x16\x80_aB2\x82aP\x87V[`\xFF\x16\x90P`\x80\x81\x10aBMW`\x7F\x81\x03\x83\x90\x1C\x91PaBWV[\x80`\x7F\x03\x83\x90\x1B\x91P[\x90\x80\x02`\x7F\x81\x81\x1C`\xFF\x83\x81\x1C\x91\x90\x91\x1C\x80\x02\x80\x83\x1C\x81\x83\x1C\x1C\x80\x02\x80\x84\x1C\x81\x84\x1C\x1C\x80\x02\x80\x85\x1C\x81\x85\x1C\x1C\x80\x02\x80\x86\x1C\x81\x86\x1C\x1C\x80\x02\x80\x87\x1C\x81\x87\x1C\x1C\x80\x02\x80\x88\x1C\x81\x88\x1C\x1C\x80\x02\x80\x89\x1C\x81\x89\x1C\x1C\x80\x02\x80\x8A\x1C\x81\x8A\x1C\x1C\x80\x02\x80\x8B\x1C\x81\x8B\x1C\x1C\x80\x02\x80\x8C\x1C\x81\x8C\x1C\x1C\x80\x02\x80\x8D\x1C\x81\x8D\x1C\x1C\x80\x02\x80\x8E\x1C\x9C\x81\x90\x1C\x9C\x90\x9C\x1C\x80\x02\x9C\x8D\x90\x1C\x9E\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8F\x01`@\x1B`\xC0\x91\x90\x91\x1Cg\x80\0\0\0\0\0\0\0\x16\x17`\xC1\x9B\x90\x9B\x1Cg@\0\0\0\0\0\0\0\x16\x9A\x90\x9A\x17`\xC2\x99\x90\x99\x1Cg \0\0\0\0\0\0\0\x16\x98\x90\x98\x17`\xC3\x97\x90\x97\x1Cg\x10\0\0\0\0\0\0\0\x16\x96\x90\x96\x17`\xC4\x95\x90\x95\x1Cg\x08\0\0\0\0\0\0\0\x16\x94\x90\x94\x17`\xC5\x93\x90\x93\x1Cg\x04\0\0\0\0\0\0\0\x16\x92\x90\x92\x17`\xC6\x91\x90\x91\x1Cg\x02\0\0\0\0\0\0\0\x16\x17`\xC7\x91\x90\x91\x1Cg\x01\0\0\0\0\0\0\0\x16\x17`\xC8\x91\x90\x91\x1Cf\x80\0\0\0\0\0\0\x16\x17`\xC9\x91\x90\x91\x1Cf@\0\0\0\0\0\0\x16\x17`\xCA\x91\x90\x91\x1Cf \0\0\0\0\0\0\x16\x17`\xCB\x91\x90\x91\x1Cf\x10\0\0\0\0\0\0\x16\x17`\xCC\x91\x90\x91\x1Cf\x08\0\0\0\0\0\0\x16\x17`\xCD\x91\x90\x91\x1Cf\x04\0\0\0\0\0\0\x16\x17i6'\xA3\x01\xD7\x10UwL\x85\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFDp\x9B~T\x80\xFB\xA5\xA5\x0F\xED^b\xFF\xC5V\x81\x01`\x80\x90\x81\x1D\x90o\xDB-\xF0\x9E\x81\x95\x9A\x81E^&\x07\x99\xA0c/\x83\x01\x90\x1D`\x02\x81\x81\x0B\x90\x83\x90\x0B\x14aD\x9EW\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aDv\x82a=}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aD\x98W\x81aD\xA0V[\x80aD\xA0V[\x81[\x99\x98PPPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90\x81\x90\x85T`@\x86\x01Q_\x81aD\xE7Wa\x0F\xFF`\xC4\x84\x90\x1C\x16aD\xF1V[a\x0F\xFF`\xB8\x84\x90\x1C\x16[\x88Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86Ra\xFF\xFF\x91\x90\x91\x16\x91P_`\xA0\x85\x90\x1C`\x02\x0B`\x02\x0B` \x87\x01R`\x03\x8B\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x87\x01R`\x80\x8A\x01Q_\x90b@\0\0\x16aEaW`\xD0\x86\x90\x1Cb\xFF\xFF\xFF\x16aEsV[aEs\x8B`\x80\x01Qb\xFF\xFF\xFF\x16aQ\x1BV[\x90P\x83\x15aE\xA1Wb\x0FB@a\x0F\xFF\x85\x16b\xFF\xFF\xFF\x83\x16\x81\x81\x02\x83\x81\x06\x15\x15\x93\x90\x04\x92\x90\x92\x01\x91\x01\x03aE\xA3V[\x80[\x97PPb\x0FB@\x87b\xFF\xFF\xFF\x16\x10aE\xE7W\x89Q_\x12\x15aE\xE7WaE\xE7\x7F\x96 bF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[\x89Q_\x03aE\xFFW_\x80\x98P\x98PPPPPPaM6V[\x83\x15aF\xE2W``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x10aFqWaFqs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16[``\x8C\x01Q\x7F|\x9Cn\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90a,\xFCV[d\x01\0\x02v\xA3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aF\xDDW``\x8A\x01QaF\xDD\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\rV[aG\xA0V[``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x11aG%WaG%s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16aFDV[s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10aG\xA0W``\x8A\x01QaG\xA0\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\rV[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x84aG\xEFW\x8B`\x02\x01TaG\xF5V[\x8B`\x01\x01T[`\xE0\x82\x01R[\x82\x15\x80aH:WP\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[aK\xC2W\x86Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x88\x01Q\x90\x8C\x01QaHq\x91`\x05\x8F\x01\x91\x88aQ*V[\x15\x15`@\x83\x01R`\x02\x0B` \x82\x01\x81\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x12aH\xCFW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18` \x82\x01R[b\r\x89\xE8`\x02\x0B\x81` \x01Q`\x02\x0B\x12aH\xEDWb\r\x89\xE8` \x82\x01R[aH\xFA\x81` \x01Qa=}V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x82\x90R\x89Q\x90\x8E\x01QaID\x93\x91\x92\x91\x16\x80\x82\x18\x91\x81\x11`\x01\x8A\x16\x18\x91\x90\x91\x02\x18\x89`@\x01Q\x86\x8CaRUV[`\xC0\x85\x01R`\xA0\x84\x01R`\x80\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x8AQ_\x12\x15aI\xADWaI\x81\x81`\xA0\x01QaO\xEAV[\x83\x03\x92PaI\x9C\x81`\xC0\x01Q\x82`\x80\x01Qa@\x95\x91\x90a_\x92V[aI\xA6\x90\x83ag\x91V[\x91PaI\xDEV[aI\xC0\x81`\xC0\x01Q\x82`\x80\x01Q\x01aO\xEAV[\x83\x01\x92PaI\xD1\x81`\xA0\x01QaO\xEAV[aI\xDB\x90\x83ae\xDFV[\x91P[\x83\x15aJ\x1AW_b\x0FB@\x85\x83`\xC0\x01Q\x84`\x80\x01Q\x01\x02\x81aJ\x03WaJ\x03ag\xB0V[`\xC0\x84\x01\x80Q\x92\x90\x91\x04\x91\x82\x90\x03\x90R\x99\x90\x99\x01\x98P[`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15aJyWaJm\x81`\xC0\x01Qp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89`@\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x02\x04\x90V[`\xE0\x82\x01\x80Q\x90\x91\x01\x90R[\x80``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aK\x8FW\x80`@\x01Q\x15aKjW_\x80\x86aJ\xD3W\x8D`\x01\x01T\x83`\xE0\x01QaJ\xDEV[\x82`\xE0\x01Q\x8E`\x02\x01T[\x91P\x91P_aK6\x8F\x85` \x01Q\x85\x85`\x02\x92\x83\x0B_\x90\x81R`\x04\x90\x94\x01` R`@\x90\x93 `\x01\x81\x01\x80T\x90\x92\x03\x90\x91U\x90\x81\x01\x80T\x90\x92\x03\x90\x91UTp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0B\x90V[\x90P\x87\x15aKAW_\x03[aKO\x8A`@\x01Q\x82a@\xDEV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x8B\x01RPPP[\x84aKyW\x80` \x01QaK\x82V[`\x01\x81` \x01Q\x03[`\x02\x0B` \x88\x01RaG\xFBV[\x80Q\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aK\xBDW\x86QaK\x82\x90aA\x89V[aG\xFBV[\x86Q` \x88\x01QaLW\x91\x90aL\x19\x90\x89\x90`\xA0\x1Bv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x8CU`@\x87\x01Q`\x03\x8D\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aL\xC6W`@\x87\x01Q`\x03\x8D\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x84aL\xDAW`\xE0\x81\x01Q`\x02\x8D\x01UaL\xE5V[`\xE0\x81\x01Q`\x01\x8D\x01U[\x8AQ_\x13\x85\x15\x15\x14aM\x12WaM\x0BaL\xFD\x83a&iV[a)\x10\x85\x8E_\x01Q\x03a&iV[\x99PaM/V[aM,aM#\x84\x8D_\x01Q\x03a&iV[a)\x10\x84a&iV[\x99P[PPPPPP[\x92\x95\x91\x94P\x92PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90aM~\x90\x84\x90a_\x7FV[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01a ,V[`@Q\x83\x81R\x82`\x02\x0B`\x04\x82\x01R\x81`\x02\x0B`$\x82\x01R`D\x81\xFD[_\x83\x83\x02\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80\x84\x11aN0W_\x80\xFD[\x80_\x03aNBWP\x82\x90\x04\x90Pa\x088V[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aN\xE6W\x92\x93\x92[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aO\rWb\xBF\xC9!_R`\x04`\x1C\xFD[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x86\x03\x16\x83aO\x99W\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aO\x86\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aM\xF2V[\x81aO\x93WaO\x93ag\xB0V[\x04aO\xDFV[aO\xDFaO\xBD\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aS\xC5V[\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x97\x96PPPPPPPV[\x80_\x81\x12\x15a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x90\x86\x16\x03`\xFF\x81\x90\x1D\x90\x81\x01\x18l\x01\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aPn\x81\x84\x84aM\xF2V[\x93P\x84_\x83\x85\x84\t\x11\x16\x84\x01\x93PPPP\x94\x93PPPPV[_\x80\x82\x11aP\x93W_\x80\xFD[P\x7F\x07\x06\x06\x05\x06\x02\x05\0\x06\x02\x03\x02\x05\x04\0\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBEo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11`\x07\x1B\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[b\xBF\xFF\xFF\x81\x16a\x0Cb\x81a$\x10V[_\x80`\x02\x84\x81\x0B\x90\x86\x90\x0B\x81\x81\x07\x83\x13\x91\x90\x05\x03\x83\x15aQ\xC8W`\x02\x81\x90\x0B`\x08\x1D`\x01\x81\x90\x0B_\x90\x81R` \x89\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x80\x85\x16\x90\x81\x90\x03\x91\x90\x91\x1C\x91\x82\x16\x80\x15\x15\x95P\x90\x91\x90\x85aQ\xAAW\x88\x83`\xFF\x16\x86\x03\x02aQ\xBDV[\x88aQ\xB4\x82aP\x87V[\x84\x03`\xFF\x16\x86\x03\x02[\x96PPPPPaRKV[`\x01\x90\x81\x01`\x02\x81\x90\x0B`\x08\x1D\x80\x83\x0B_\x90\x81R` \x8A\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x84\x16\x94\x85\x1B\x01\x19\x90\x81\x16\x80\x15\x15\x95P\x92\x93\x91\x92\x85aR1W\x88\x83`\xFF\x03`\xFF\x16\x86\x01\x02aRDV[\x88\x83aR<\x83aS\xF5V[\x03`\xFF\x16\x86\x01\x02[\x96PPPPP[P\x94P\x94\x92PPPV[_\x80\x80\x80b\xFF\xFF\xFF\x85\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x90\x8B\x16\x10\x15\x82\x88\x12\x80\x15aS8W_aR\x9B\x8A_\x03\x85b\x0FB@\x03b\x0FB@aM\xF2V[\x90P\x82aR\xB4WaR\xAF\x8D\x8D\x8D`\x01aP\x1CV[aR\xC1V[aR\xC1\x8C\x8E\x8D`\x01aN\xADV[\x96P\x86\x81\x10aR\xF5W\x8B\x97Pb\x0FB@\x84\x14aR\xECWaR\xE7\x87\x85\x86b\x0FB@\x03aS\xC5V[aR\xEEV[\x86[\x94PaS\x0EV[\x80\x96PaS\x04\x8D\x8C\x83\x86aT\x8FV[\x97P\x86\x8A_\x03\x03\x94P[\x82aS$WaS\x1F\x8D\x89\x8D_aN\xADV[aS0V[aS0\x88\x8E\x8D_aP\x1CV[\x95PPaS\xB6V[\x81aSNWaSI\x8C\x8C\x8C_aN\xADV[aSZV[aSZ\x8B\x8D\x8C_aP\x1CV[\x94P\x84\x89\x10aSkW\x8A\x96PaS}V[\x88\x94PaSz\x8C\x8B\x87\x85aT\xF3V[\x96P[\x81aS\x94WaS\x8F\x8C\x88\x8C`\x01aP\x1CV[aS\xA1V[aS\xA1\x87\x8D\x8C`\x01aN\xADV[\x95PaS\xB3\x86\x84\x85b\x0FB@\x03aS\xC5V[\x93P[PPP\x95P\x95P\x95P\x95\x91PPV[_aS\xD1\x84\x84\x84aM\xF2V[\x90P\x81\x80aS\xE1WaS\xE1ag\xB0V[\x83\x85\t\x15a\x088W`\x01\x01\x80a\x088W_\x80\xFD[_\x80\x82\x11aT\x01W_\x80\xFD[P~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05_\x82\x90\x03\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aT\xCFWcO$a\xB8_R`\x04`\x1C\xFD[\x81aT\xE6WaT\xE1\x85\x85\x85`\x01aULV[a+\rV[a+\r\x85\x85\x85`\x01aV\xAEV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aU3WcO$a\xB8_R`\x04`\x1C\xFD[\x81aUDWaT\xE1\x85\x85\x85_aV\xAEV[a+\r\x85\x85\x85_[_\x81\x15aU\xF1W_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aU\x9FWaU\x9A\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aM\xF2V[aU\xBFV[aU\xBFo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16``\x86\x90\x1Bag\xDDV[\x90PaU\xE9aU\xE4\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16a_\x92V[aW\xE3V[\x91PPa\x1D\x01V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aV=WaV8\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aS\xC5V[aVcV[aVc``\x85\x90\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x11aV\x8FWcC#\xA5U_R`\x04`\x1C\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x03\x90Pa\x1D\x01V[_\x82_\x03aV\xBDWP\x83a\x1D\x01V[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x85\x90\x1B\x16\x82\x15aW\x88Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x81aW\x10WaW\x10ag\xB0V[\x04\x03aWMW\x81\x81\x01\x82\x81\x10aWKWaWA\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83aS\xC5V[\x93PPPPa\x1D\x01V[P[PaU\xE9\x81\x85aWss\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x83ag\xDDV[aW}\x91\x90a_\x92V[\x80\x82\x04\x91\x06\x15\x15\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x04\x14\x81\x83\x11\x16aW\xBCWc\xF5\xC7\x87\xF1_R`\x04`\x1C\xFD[\x80\x82\x03aWAaU\xE4\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x84aS\xC5V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\tV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17\x84W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15aX\\W_\x80\xFD[\x825aXg\x81aX*V[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aX\x85W_\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x088W_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15aX\xC6W_\x80\xFD[\x835aX\xD1\x81aX*V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_\x80_``\x84\x86\x03\x12\x15aX\xF8W_\x80\xFD[\x835aY\x03\x81aX*V[\x92P` \x84\x015aY\x13\x81aX*V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15aY4W_\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aY\x8BWaY\x8BaY;V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aY\xD8WaY\xD8aY;V[`@R\x91\x90PV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbW_\x80\xFD[\x805`\x02\x81\x90\x0B\x81\x14a\x0CbW_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15aZ\x13W_\x80\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15aZ6WaZ6aY;V[`@R\x90P\x80\x825aZG\x81aX*V[\x81R` \x83\x015aZW\x81aX*V[` \x82\x01RaZh`@\x84\x01aY\xE0V[`@\x82\x01RaZy``\x84\x01aY\xF2V[``\x82\x01R`\x80\x83\x015aZ\x8C\x81aX*V[`\x80\x91\x90\x91\x01R\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12aZ\xA9W_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aZ\xC0W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aZ\xD7W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15aZ\xF3W_\x80\xFD[aZ\xFD\x87\x87aZ\x03V[\x94P`\xA0\x86\x015\x93P`\xC0\x86\x015\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[&W_\x80\xFD[a[2\x88\x82\x89\x01aZ\x99V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a[SW_\x80\xFD[\x815a\x088\x81aX*V[_\x80`@\x83\x85\x03\x12\x15a[oW_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a[\xB5W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a[\x97V[P\x90\x95\x94PPPPPV[_\x80` \x83\x85\x03\x12\x15a[\xD1W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\xE7W_\x80\xFD[a[\xF3\x85\x82\x86\x01aZ\x99V[\x90\x96\x90\x95P\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_\x80`\xC0\x83\x85\x03\x12\x15a\\cW_\x80\xFD[a\\m\x84\x84aZ\x03V[\x91Pa\\{`\xA0\x84\x01aY\xE0V[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x0CbW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a\\\xA4W_\x80\xFD[\x825a\\\xAF\x81aX*V[\x91Pa\\{` \x84\x01a\\\x84V[_\x80_\x80\x84\x86\x03a\x01@\x81\x12\x15a\\\xD2W_\x80\xFD[a\\\xDC\x87\x87aZ\x03V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a]\rW_\x80\xFD[Pa]\x16aYhV[a]\"`\xA0\x87\x01aY\xF2V[\x81Ra]0`\xC0\x87\x01aY\xF2V[` \x82\x01R`\xE0\x86\x015`@\x82\x01Ra\x01\0\x86\x015``\x82\x01R\x92Pa\x01 \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]fW_\x80\xFD[a]r\x87\x82\x88\x01aZ\x99V[\x95\x98\x94\x97P\x95PPPPV[_\x80`\xC0\x83\x85\x03\x12\x15a]\x8FW_\x80\xFD[a]\x99\x84\x84aZ\x03V[\x91P`\xA0\x83\x015a]\xA9\x81aX*V[\x80\x91PP\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15a]\xC5W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xDBW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a]\xEBW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x01W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a^\x15W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a^6W_\x80\xFD[\x825a^A\x81aX*V[\x91P` \x83\x015a]\xA9\x81aX*V[_\x80_\x80\x84\x86\x03a\x01 \x81\x12\x15a^fW_\x80\xFD[a^p\x87\x87aZ\x03V[\x94P``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a^\xA1W_\x80\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a^\xC5Wa^\xC5aY;V[`@Ra^\xD4`\xA0\x87\x01a\\\x84V[\x81R`\xC0\x86\x015` \x82\x01R`\xE0\x86\x015a^\xEE\x81aX*V[`@\x82\x01R\x92Pa\x01\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]fW_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15a_\"W_\x80\xFD[\x845a_-\x81aX*V[\x93P` \x85\x015a_=\x81aX*V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07eWa\x07ea_RV[\x80\x82\x01\x80\x82\x11\x15a\x07eWa\x07ea_RV[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R_a\x1D\x01` \x83\x01\x84\x86a_\xA5V[_` \x82\x84\x03\x12\x15a`\x0FW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`%W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a`5W_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`OWa`OaY;V[a`\x80` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01aY\x91V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a`\x94W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RaaL` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84`\xC0\x82\x01R\x83`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_aaqa\x01 \x83\x01\x84\x86a_\xA5V[\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rab\x18` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x85\x01Q\x90\x0B`\xE0\x82\x01R`@\x84\x01Qa\x01\0\x82\x01R``\x84\x01Qa\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_aO\xDFa\x01`\x83\x01\x84\x86a_\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x81Rab\xFA` \x82\x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x85Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x87\x01Q\x90\x0B`\xE0\x82\x01R`@\x86\x01Qa\x01\0\x82\x01R``\x86\x01Qa\x01 \x82\x01R\x84a\x01@\x82\x01R\x83a\x01`\x82\x01Ra\x01\xA0a\x01\x80\x82\x01R_aD\xA0a\x01\xA0\x83\x01\x84\x86a_\xA5V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81R`\xE0\x81\x01ac\xEE` \x83\x01\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\xC0\x83\x01R\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x81Ra\x01\0\x81\x01ad\xB2` \x83\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\xC0\x83\x01R\x82`\x02\x0B`\xE0\x83\x01R\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15ad\xF0W_\x80\xFD[PQ\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rae\x92` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q\x15\x15`\xC0\x82\x01R` \x84\x01Q`\xE0\x82\x01R`@\x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01R_aO\xDFa\x01@\x83\x01\x84\x86a_\xA5V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15ae\xFEWae\xFEa_RV[PP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81Raf\xA1` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84Q\x15\x15`\xC0\x82\x01R` \x85\x01Q`\xE0\x82\x01R`@\x85\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01R\x83a\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_aaqa\x01`\x83\x01\x84\x86a_\xA5V[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12\x17\x15a\x07eWa\x07ea_RV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x07eWa\x07ea_RV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a9\x8FWa9\x8Fa_RV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82ah\x10W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA1dsolcC\0\x08\x1A\0\n",
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
    /**Custom error with signature `AlreadyUnlocked()` and selector `0x5090d6c6`.
```solidity
error AlreadyUnlocked();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AlreadyUnlocked {}
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
        impl ::core::convert::From<AlreadyUnlocked> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyUnlocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyUnlocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyUnlocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyUnlocked()";
            const SELECTOR: [u8; 4] = [80u8, 144u8, 214u8, 198u8];
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
    /**Custom error with signature `ContractUnlocked()` and selector `0x3e5f4fd6`.
```solidity
error ContractUnlocked();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ContractUnlocked {}
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
        impl ::core::convert::From<ContractUnlocked> for UnderlyingRustTuple<'_> {
            fn from(value: ContractUnlocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContractUnlocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractUnlocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractUnlocked()";
            const SELECTOR: [u8; 4] = [62u8, 95u8, 79u8, 214u8];
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
    /**Custom error with signature `CurrenciesOutOfOrderOrEqual(address,address)` and selector `0x6e6c9830`.
```solidity
error CurrenciesOutOfOrderOrEqual(address currency0, address currency1);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct CurrenciesOutOfOrderOrEqual {
        pub currency0: alloy::sol_types::private::Address,
        pub currency1: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<CurrenciesOutOfOrderOrEqual>
        for UnderlyingRustTuple<'_> {
            fn from(value: CurrenciesOutOfOrderOrEqual) -> Self {
                (value.currency0, value.currency1)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CurrenciesOutOfOrderOrEqual {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currency0: tuple.0,
                    currency1: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrenciesOutOfOrderOrEqual {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrenciesOutOfOrderOrEqual(address,address)";
            const SELECTOR: [u8; 4] = [110u8, 108u8, 152u8, 48u8];
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
                        &self.currency0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.currency1,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `CurrencyNotSettled()` and selector `0x5212cba1`.
```solidity
error CurrencyNotSettled();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct CurrencyNotSettled {}
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
        impl ::core::convert::From<CurrencyNotSettled> for UnderlyingRustTuple<'_> {
            fn from(value: CurrencyNotSettled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrencyNotSettled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrencyNotSettled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrencyNotSettled()";
            const SELECTOR: [u8; 4] = [82u8, 18u8, 203u8, 161u8];
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
    /**Custom error with signature `DelegateCallNotAllowed()` and selector `0x0d89438e`.
```solidity
error DelegateCallNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct DelegateCallNotAllowed {}
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
        impl ::core::convert::From<DelegateCallNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: DelegateCallNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegateCallNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelegateCallNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelegateCallNotAllowed()";
            const SELECTOR: [u8; 4] = [13u8, 137u8, 67u8, 142u8];
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
    /**Custom error with signature `InvalidCaller()` and selector `0x48f5c3ed`.
```solidity
error InvalidCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidCaller {}
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
        impl ::core::convert::From<InvalidCaller> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCaller()";
            const SELECTOR: [u8; 4] = [72u8, 245u8, 195u8, 237u8];
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
    /**Custom error with signature `ManagerLocked()` and selector `0x54e3ca0d`.
```solidity
error ManagerLocked();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ManagerLocked {}
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
        impl ::core::convert::From<ManagerLocked> for UnderlyingRustTuple<'_> {
            fn from(value: ManagerLocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ManagerLocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ManagerLocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ManagerLocked()";
            const SELECTOR: [u8; 4] = [84u8, 227u8, 202u8, 13u8];
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
    /**Custom error with signature `MustClearExactPositiveDelta()` and selector `0xbda73abf`.
```solidity
error MustClearExactPositiveDelta();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MustClearExactPositiveDelta {}
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
        impl ::core::convert::From<MustClearExactPositiveDelta>
        for UnderlyingRustTuple<'_> {
            fn from(value: MustClearExactPositiveDelta) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MustClearExactPositiveDelta {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustClearExactPositiveDelta {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustClearExactPositiveDelta()";
            const SELECTOR: [u8; 4] = [189u8, 167u8, 58u8, 191u8];
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
    /**Custom error with signature `NonzeroNativeValue()` and selector `0xb0ec849e`.
```solidity
error NonzeroNativeValue();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NonzeroNativeValue {}
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
        impl ::core::convert::From<NonzeroNativeValue> for UnderlyingRustTuple<'_> {
            fn from(value: NonzeroNativeValue) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonzeroNativeValue {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonzeroNativeValue {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonzeroNativeValue()";
            const SELECTOR: [u8; 4] = [176u8, 236u8, 132u8, 158u8];
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
    /**Custom error with signature `PoolNotInitialized()` and selector `0x486aa307`.
```solidity
error PoolNotInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoolNotInitialized {}
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
        impl ::core::convert::From<PoolNotInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: PoolNotInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolNotInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PoolNotInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PoolNotInitialized()";
            const SELECTOR: [u8; 4] = [72u8, 106u8, 163u8, 7u8];
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
    /**Custom error with signature `ProtocolFeeTooLarge(uint24)` and selector `0xa7abe2f7`.
```solidity
error ProtocolFeeTooLarge(uint24 fee);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ProtocolFeeTooLarge {
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<ProtocolFeeTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ProtocolFeeTooLarge) -> Self {
                (value.fee,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProtocolFeeTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { fee: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProtocolFeeTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProtocolFeeTooLarge(uint24)";
            const SELECTOR: [u8; 4] = [167u8, 171u8, 226u8, 247u8];
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
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
        }
    };
    /**Custom error with signature `SwapAmountCannotBeZero()` and selector `0xbe8b8507`.
```solidity
error SwapAmountCannotBeZero();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SwapAmountCannotBeZero {}
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
        impl ::core::convert::From<SwapAmountCannotBeZero> for UnderlyingRustTuple<'_> {
            fn from(value: SwapAmountCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapAmountCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SwapAmountCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SwapAmountCannotBeZero()";
            const SELECTOR: [u8; 4] = [190u8, 139u8, 133u8, 7u8];
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
    /**Custom error with signature `TickSpacingTooLarge(int24)` and selector `0xb70024f8`.
```solidity
error TickSpacingTooLarge(int24 tickSpacing);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct TickSpacingTooLarge {
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<TickSpacingTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: TickSpacingTooLarge) -> Self {
                (value.tickSpacing,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TickSpacingTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tickSpacing: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TickSpacingTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TickSpacingTooLarge(int24)";
            const SELECTOR: [u8; 4] = [183u8, 0u8, 36u8, 248u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                )
            }
        }
    };
    /**Custom error with signature `TickSpacingTooSmall(int24)` and selector `0xe9e90588`.
```solidity
error TickSpacingTooSmall(int24 tickSpacing);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct TickSpacingTooSmall {
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
        impl ::core::convert::From<TickSpacingTooSmall> for UnderlyingRustTuple<'_> {
            fn from(value: TickSpacingTooSmall) -> Self {
                (value.tickSpacing,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TickSpacingTooSmall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tickSpacing: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TickSpacingTooSmall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TickSpacingTooSmall(int24)";
            const SELECTOR: [u8; 4] = [233u8, 233u8, 5u8, 136u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                )
            }
        }
    };
    /**Custom error with signature `UnauthorizedDynamicLPFeeUpdate()` and selector `0x30d21641`.
```solidity
error UnauthorizedDynamicLPFeeUpdate();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct UnauthorizedDynamicLPFeeUpdate {}
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
        impl ::core::convert::From<UnauthorizedDynamicLPFeeUpdate>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnauthorizedDynamicLPFeeUpdate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnauthorizedDynamicLPFeeUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnauthorizedDynamicLPFeeUpdate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnauthorizedDynamicLPFeeUpdate()";
            const SELECTOR: [u8; 4] = [48u8, 210u8, 22u8, 65u8];
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
    /**Event with signature `Approval(address,address,uint256,uint256)` and selector `0xb3fd5071835887567a0671151121894ddccc2842f1d10bedad13e0d17cace9a7`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 indexed id, uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                179u8,
                253u8,
                80u8,
                113u8,
                131u8,
                88u8,
                135u8,
                86u8,
                122u8,
                6u8,
                113u8,
                21u8,
                17u8,
                33u8,
                137u8,
                77u8,
                220u8,
                204u8,
                40u8,
                66u8,
                241u8,
                209u8,
                11u8,
                237u8,
                173u8,
                19u8,
                224u8,
                209u8,
                124u8,
                172u8,
                233u8,
                167u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    id: topics.3,
                    amount: data.0,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.owner.clone(),
                    self.spender.clone(),
                    self.id.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Donate(bytes32,address,uint256,uint256)` and selector `0x29ef05caaff9404b7cb6d1c0e9bbae9eaa7ab2541feba1a9c4248594c08156cb`.
```solidity
event Donate(PoolId indexed id, address indexed sender, uint256 amount0, uint256 amount1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Donate {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Donate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Donate(bytes32,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                41u8,
                239u8,
                5u8,
                202u8,
                175u8,
                249u8,
                64u8,
                75u8,
                124u8,
                182u8,
                209u8,
                192u8,
                233u8,
                187u8,
                174u8,
                158u8,
                170u8,
                122u8,
                178u8,
                84u8,
                31u8,
                235u8,
                161u8,
                169u8,
                196u8,
                36u8,
                133u8,
                148u8,
                192u8,
                129u8,
                86u8,
                203u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    sender: topics.2,
                    amount0: data.0,
                    amount1: data.1,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Donate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Donate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Donate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialize(bytes32,address,address,uint24,int24,address,uint160,int24)` and selector `0xdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d6438`.
```solidity
event Initialize(PoolId indexed id, Currency indexed currency0, Currency indexed currency1, uint24 fee, int24 tickSpacing, address hooks, uint160 sqrtPriceX96, int24 tick);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Initialize {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub currency0: <Currency as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub currency1: <Currency as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub hooks: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialize {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                Currency,
                Currency,
            );
            const SIGNATURE: &'static str = "Initialize(bytes32,address,address,uint24,int24,address,uint160,int24)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                70u8,
                110u8,
                103u8,
                78u8,
                165u8,
                87u8,
                245u8,
                98u8,
                149u8,
                226u8,
                208u8,
                33u8,
                138u8,
                18u8,
                94u8,
                164u8,
                180u8,
                240u8,
                246u8,
                243u8,
                48u8,
                123u8,
                149u8,
                248u8,
                94u8,
                97u8,
                16u8,
                131u8,
                141u8,
                100u8,
                56u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    currency0: topics.2,
                    currency1: topics.3,
                    fee: data.0,
                    tickSpacing: data.1,
                    hooks: data.2,
                    sqrtPriceX96: data.3,
                    tick: data.4,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hooks,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.id.clone(),
                    self.currency0.clone(),
                    self.currency1.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <Currency as alloy_sol_types::EventTopic>::encode_topic(
                    &self.currency0,
                );
                out[3usize] = <Currency as alloy_sol_types::EventTopic>::encode_topic(
                    &self.currency1,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialize {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialize> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialize) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ModifyLiquidity(bytes32,address,int24,int24,int256,bytes32)` and selector `0xf208f4912782fd25c7f114ca3723a2d5dd6f3bcc3ac8db5af63baa85f711d5ec`.
```solidity
event ModifyLiquidity(PoolId indexed id, address indexed sender, int24 tickLower, int24 tickUpper, int256 liquidityDelta, bytes32 salt);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ModifyLiquidity {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub liquidityDelta: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ModifyLiquidity {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ModifyLiquidity(bytes32,address,int24,int24,int256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8,
                8u8,
                244u8,
                145u8,
                39u8,
                130u8,
                253u8,
                37u8,
                199u8,
                241u8,
                20u8,
                202u8,
                55u8,
                35u8,
                162u8,
                213u8,
                221u8,
                111u8,
                59u8,
                204u8,
                58u8,
                200u8,
                219u8,
                90u8,
                246u8,
                59u8,
                170u8,
                133u8,
                247u8,
                17u8,
                213u8,
                236u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    sender: topics.2,
                    tickLower: data.0,
                    tickUpper: data.1,
                    liquidityDelta: data.2,
                    salt: data.3,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
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
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ModifyLiquidity {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ModifyLiquidity> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ModifyLiquidity) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSet(address,address,bool)` and selector `0xceb576d9f15e4e200fdb5096d64d5dfd667e16def20c1eefd14256d8e3faa267`.
```solidity
event OperatorSet(address indexed owner, address indexed operator, bool approved);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorSet {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approved: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSet(address,address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                206u8,
                181u8,
                118u8,
                217u8,
                241u8,
                94u8,
                78u8,
                32u8,
                15u8,
                219u8,
                80u8,
                150u8,
                214u8,
                77u8,
                93u8,
                253u8,
                102u8,
                126u8,
                22u8,
                222u8,
                242u8,
                12u8,
                30u8,
                239u8,
                209u8,
                66u8,
                86u8,
                216u8,
                227u8,
                250u8,
                162u8,
                103u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    operator: topics.2,
                    approved: data.0,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.approved,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed user, address indexed newOwner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    user: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.user.clone(), self.newOwner.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProtocolFeeControllerUpdated(address)` and selector `0xb4bd8ef53df690b9943d3318996006dbb82a25f54719d8c8035b516a2a5b8acc`.
```solidity
event ProtocolFeeControllerUpdated(address indexed protocolFeeController);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ProtocolFeeControllerUpdated {
        #[allow(missing_docs)]
        pub protocolFeeController: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolFeeControllerUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ProtocolFeeControllerUpdated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                180u8,
                189u8,
                142u8,
                245u8,
                61u8,
                246u8,
                144u8,
                185u8,
                148u8,
                61u8,
                51u8,
                24u8,
                153u8,
                96u8,
                6u8,
                219u8,
                184u8,
                42u8,
                37u8,
                245u8,
                71u8,
                25u8,
                216u8,
                200u8,
                3u8,
                91u8,
                81u8,
                106u8,
                42u8,
                91u8,
                138u8,
                204u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    protocolFeeController: topics.1,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.protocolFeeController.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.protocolFeeController,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolFeeControllerUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolFeeControllerUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ProtocolFeeControllerUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProtocolFeeUpdated(bytes32,uint24)` and selector `0xe9c42593e71f84403b84352cd168d693e2c9fcd1fdbcc3feb21d92b43e6696f9`.
```solidity
event ProtocolFeeUpdated(PoolId indexed id, uint24 protocolFee);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ProtocolFeeUpdated {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub protocolFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolFeeUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<24>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, PoolId);
            const SIGNATURE: &'static str = "ProtocolFeeUpdated(bytes32,uint24)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                233u8,
                196u8,
                37u8,
                147u8,
                231u8,
                31u8,
                132u8,
                64u8,
                59u8,
                132u8,
                53u8,
                44u8,
                209u8,
                104u8,
                214u8,
                147u8,
                226u8,
                201u8,
                252u8,
                209u8,
                253u8,
                188u8,
                195u8,
                254u8,
                178u8,
                29u8,
                146u8,
                180u8,
                62u8,
                102u8,
                150u8,
                249u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    protocolFee: data.0,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.protocolFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolFeeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolFeeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProtocolFeeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)` and selector `0x40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f`.
```solidity
event Swap(PoolId indexed id, address indexed sender, int128 amount0, int128 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick, uint24 fee);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Swap {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount0: i128,
        #[allow(missing_docs)]
        pub amount1: i128,
        #[allow(missing_docs)]
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub liquidity: u128,
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Swap {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Int<128>,
                alloy::sol_types::sol_data::Int<128>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                233u8,
                206u8,
                203u8,
                159u8,
                95u8,
                31u8,
                28u8,
                91u8,
                156u8,
                151u8,
                222u8,
                194u8,
                145u8,
                123u8,
                126u8,
                233u8,
                46u8,
                87u8,
                186u8,
                85u8,
                99u8,
                112u8,
                141u8,
                172u8,
                169u8,
                77u8,
                216u8,
                74u8,
                215u8,
                17u8,
                47u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    sender: topics.2,
                    amount0: data.0,
                    amount1: data.1,
                    sqrtPriceX96: data.2,
                    liquidity: data.3,
                    tick: data.4,
                    fee: data.5,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Swap {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Swap> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Swap) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Transfer(address,address,address,uint256,uint256)` and selector `0x1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859`.
```solidity
event Transfer(address caller, address indexed from, address indexed to, uint256 indexed id, uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                27u8,
                61u8,
                126u8,
                219u8,
                46u8,
                156u8,
                11u8,
                14u8,
                124u8,
                82u8,
                91u8,
                32u8,
                170u8,
                174u8,
                240u8,
                245u8,
                148u8,
                13u8,
                46u8,
                215u8,
                22u8,
                99u8,
                199u8,
                211u8,
                146u8,
                102u8,
                236u8,
                175u8,
                172u8,
                114u8,
                136u8,
                89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    caller: data.0,
                    from: topics.1,
                    to: topics.2,
                    id: topics.3,
                    amount: data.1,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.caller,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.from.clone(),
                    self.to.clone(),
                    self.id.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `allowance(address,address,uint256)` and selector `0x598af9e7`.
```solidity
function allowance(address owner, address spender, uint256 id) external view returns (uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct allowanceCall {
        pub owner: alloy::sol_types::private::Address,
        pub spender: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`allowance(address,address,uint256)`](allowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct allowanceReturn {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
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
            impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceCall) -> Self {
                    (value.owner, value.spender, value.id)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                        id: tuple.2,
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
            impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceReturn) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowance(address,address,uint256)";
            const SELECTOR: [u8; 4] = [89u8, 138u8, 249u8, 231u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `approve(address,uint256,uint256)` and selector `0x426a8493`.
```solidity
function approve(address spender, uint256 id, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct approveCall {
        pub spender: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`approve(address,uint256,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct approveReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.spender, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        id: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [66u8, 106u8, 132u8, 147u8];
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
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`.
```solidity
function balanceOf(address owner, uint256 id) external view returns (uint256 balance);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        pub owner: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`balanceOf(address,uint256)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
    }
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
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.owner, value.id)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        id: tuple.1,
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
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value.balance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { balance: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address,uint256)";
            const SELECTOR: [u8; 4] = [0u8, 253u8, 213u8, 142u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `burn(address,uint256,uint256)` and selector `0xf5298aca`.
```solidity
function burn(address from, uint256 id, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct burnCall {
        pub from: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burn(address,uint256,uint256)`](burnCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct burnReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<burnCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnCall) -> Self {
                    (value.from, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        id: tuple.1,
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
            impl ::core::convert::From<burnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burn(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [245u8, 41u8, 138u8, 202u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `clear(address,uint256)` and selector `0x80f0b44c`.
```solidity
function clear(Currency currency, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct clearCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`clear(address,uint256)`](clearCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct clearReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                Currency,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<clearCall> for UnderlyingRustTuple<'_> {
                fn from(value: clearCall) -> Self {
                    (value.currency, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        currency: tuple.0,
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
            impl ::core::convert::From<clearReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clearReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearCall {
            type Parameters<'a> = (Currency, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clear(address,uint256)";
            const SELECTOR: [u8; 4] = [128u8, 240u8, 180u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency),
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
    /**Function with signature `collectProtocolFees(address,address,uint256)` and selector `0x8161b874`.
```solidity
function collectProtocolFees(address recipient, Currency currency, uint256 amount) external returns (uint256 amountCollected);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct collectProtocolFeesCall {
        pub recipient: alloy::sol_types::private::Address,
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`collectProtocolFees(address,address,uint256)`](collectProtocolFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct collectProtocolFeesReturn {
        pub amountCollected: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                Currency,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <Currency as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<collectProtocolFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: collectProtocolFeesCall) -> Self {
                    (value.recipient, value.currency, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for collectProtocolFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        currency: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<collectProtocolFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: collectProtocolFeesReturn) -> Self {
                    (value.amountCollected,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for collectProtocolFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amountCollected: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for collectProtocolFeesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                Currency,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = collectProtocolFeesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "collectProtocolFees(address,address,uint256)";
            const SELECTOR: [u8; 4] = [129u8, 97u8, 184u8, 116u8];
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
                        &self.recipient,
                    ),
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency),
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
    /**Function with signature `donate((address,address,uint24,int24,address),uint256,uint256,bytes)` and selector `0x234266d7`.
```solidity
function donate(PoolKey memory key, uint256 amount0, uint256 amount1, bytes memory hookData) external returns (BalanceDelta delta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct donateCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`donate((address,address,uint24,int24,address),uint256,uint256,bytes)`](donateCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct donateReturn {
        pub delta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<donateCall> for UnderlyingRustTuple<'_> {
                fn from(value: donateCall) -> Self {
                    (value.key, value.amount0, value.amount1, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for donateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        amount0: tuple.1,
                        amount1: tuple.2,
                        hookData: tuple.3,
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
            impl ::core::convert::From<donateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: donateReturn) -> Self {
                    (value.delta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for donateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for donateCall {
            type Parameters<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = donateReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "donate((address,address,uint24,int24,address),uint256,uint256,bytes)";
            const SELECTOR: [u8; 4] = [35u8, 66u8, 102u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
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
    pub struct extsload_0Call {
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`extsload(bytes32)`](extsload_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_0Return {
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
            impl ::core::convert::From<extsload_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_0Call) -> Self {
                    (value.slot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_0Call {
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
            impl ::core::convert::From<extsload_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsload_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsload_0Return;
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
    /**Function with signature `extsload(bytes32,uint256)` and selector `0x35fd631a`.
```solidity
function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_1Call {
        pub startSlot: alloy::sol_types::private::FixedBytes<32>,
        pub nSlots: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`extsload(bytes32,uint256)`](extsload_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_1Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<extsload_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_1Call) -> Self {
                    (value.startSlot, value.nSlots)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startSlot: tuple.0,
                        nSlots: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<extsload_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsload_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsload_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [53u8, 253u8, 99u8, 26u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.startSlot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nSlots),
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
    /**Function with signature `extsload(bytes32[])` and selector `0xdbd035ff`.
```solidity
function extsload(bytes32[] memory slots) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_2Call {
        pub slots: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`extsload(bytes32[])`](extsload_2Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_2Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<extsload_2Call> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_2Call) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<extsload_2Return> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsload_2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsload_2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(bytes32[])";
            const SELECTOR: [u8; 4] = [219u8, 208u8, 53u8, 255u8];
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.slots),
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
    /**Function with signature `exttload(bytes32[])` and selector `0x9bf6645f`.
```solidity
function exttload(bytes32[] memory slots) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_0Call {
        pub slots: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`exttload(bytes32[])`](exttload_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_0Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<exttload_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_0Call) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<exttload_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exttload_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exttload_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exttload(bytes32[])";
            const SELECTOR: [u8; 4] = [155u8, 246u8, 100u8, 95u8];
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
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.slots),
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
    /**Function with signature `exttload(bytes32)` and selector `0xf135baaa`.
```solidity
function exttload(bytes32 slot) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_1Call {
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`exttload(bytes32)`](exttload_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_1Return {
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
            impl ::core::convert::From<exttload_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_1Call) -> Self {
                    (value.slot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_1Call {
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
            impl ::core::convert::From<exttload_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exttload_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exttload_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exttload(bytes32)";
            const SELECTOR: [u8; 4] = [241u8, 53u8, 186u8, 170u8];
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
    /**Function with signature `initialize((address,address,uint24,int24,address),uint160)` and selector `0x6276cbbe`.
```solidity
function initialize(PoolKey memory key, uint160 sqrtPriceX96) external returns (int24 tick);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    ///Container type for the return parameters of the [`initialize((address,address,uint24,int24,address),uint160)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeReturn {
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<160>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.key, value.sqrtPriceX96)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        sqrtPriceX96: tuple.1,
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    (value.tick,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tick: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (PoolKey, alloy::sol_types::sol_data::Uint<160>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize((address,address,uint24,int24,address),uint160)";
            const SELECTOR: [u8; 4] = [98u8, 118u8, 203u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
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
    /**Function with signature `isOperator(address,address)` and selector `0xb6363cf2`.
```solidity
function isOperator(address owner, address operator) external view returns (bool isOperator);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isOperatorCall {
        pub owner: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperator(address,address)`](isOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isOperatorReturn {
        pub isOperator: bool,
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
            impl ::core::convert::From<isOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorCall) -> Self {
                    (value.owner, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        operator: tuple.1,
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
            impl ::core::convert::From<isOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorReturn) -> Self {
                    (value.isOperator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isOperator: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperator(address,address)";
            const SELECTOR: [u8; 4] = [182u8, 54u8, 60u8, 242u8];
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
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
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
    /**Function with signature `mint(address,uint256,uint256)` and selector `0x156e29f6`.
```solidity
function mint(address to, uint256 id, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintCall {
        pub to: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,uint256,uint256)`](mintCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<mintCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintCall) -> Self {
                    (value.to, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        id: tuple.1,
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
            impl ::core::convert::From<mintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [21u8, 110u8, 41u8, 246u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `modifyLiquidity((address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x5a6bcfda`.
```solidity
function modifyLiquidity(PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory hookData) external returns (BalanceDelta callerDelta, BalanceDelta feesAccrued);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct modifyLiquidityCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`modifyLiquidity((address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)`](modifyLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct modifyLiquidityReturn {
        pub callerDelta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
        pub feesAccrued: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<modifyLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyLiquidityCall) -> Self {
                    (value.key, value.params, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        params: tuple.1,
                        hookData: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta, BalanceDelta);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<modifyLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyLiquidityReturn) -> Self {
                    (value.callerDelta, value.feesAccrued)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        callerDelta: tuple.0,
                        feesAccrued: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyLiquidityCall {
            type Parameters<'a> = (
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyLiquidityReturn;
            type ReturnTuple<'a> = (BalanceDelta, BalanceDelta);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyLiquidity((address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [90u8, 107u8, 207u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
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
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerReturn {
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
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
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
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
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
    /**Function with signature `protocolFeeController()` and selector `0xf02de3b2`.
```solidity
function protocolFeeController() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeeControllerCall {}
    ///Container type for the return parameters of the [`protocolFeeController()`](protocolFeeControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeeControllerReturn {
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
            impl ::core::convert::From<protocolFeeControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeeControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeeControllerCall {
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
            impl ::core::convert::From<protocolFeeControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeeControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeeControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for protocolFeeControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = protocolFeeControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "protocolFeeController()";
            const SELECTOR: [u8; 4] = [240u8, 45u8, 227u8, 178u8];
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
    /**Function with signature `protocolFeesAccrued(address)` and selector `0x97e8cd4e`.
```solidity
function protocolFeesAccrued(Currency currency) external view returns (uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeesAccruedCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`protocolFeesAccrued(address)`](protocolFeesAccruedCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeesAccruedReturn {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Currency,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<protocolFeesAccruedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeesAccruedCall) -> Self {
                    (value.currency,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeesAccruedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { currency: tuple.0 }
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
            impl ::core::convert::From<protocolFeesAccruedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeesAccruedReturn) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeesAccruedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for protocolFeesAccruedCall {
            type Parameters<'a> = (Currency,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = protocolFeesAccruedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "protocolFeesAccrued(address)";
            const SELECTOR: [u8; 4] = [151u8, 232u8, 205u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Currency as alloy_sol_types::SolType>::tokenize(&self.currency),)
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
    /**Function with signature `setOperator(address,bool)` and selector `0x558a7297`.
```solidity
function setOperator(address operator, bool approved) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub approved: bool,
    }
    ///Container type for the return parameters of the [`setOperator(address,bool)`](setOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setOperatorReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorCall) -> Self {
                    (value.operator, value.approved)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        approved: tuple.1,
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
            impl ::core::convert::From<setOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperator(address,bool)";
            const SELECTOR: [u8; 4] = [85u8, 138u8, 114u8, 151u8];
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.approved,
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
    /**Function with signature `setProtocolFee((address,address,uint24,int24,address),uint24)` and selector `0x7e87ce7d`.
```solidity
function setProtocolFee(PoolKey memory key, uint24 newProtocolFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub newProtocolFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    ///Container type for the return parameters of the [`setProtocolFee((address,address,uint24,int24,address),uint24)`](setProtocolFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setProtocolFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeCall) -> Self {
                    (value.key, value.newProtocolFee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProtocolFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        newProtocolFee: tuple.1,
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
            impl ::core::convert::From<setProtocolFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProtocolFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProtocolFeeCall {
            type Parameters<'a> = (PoolKey, alloy::sol_types::sol_data::Uint<24>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProtocolFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProtocolFee((address,address,uint24,int24,address),uint24)";
            const SELECTOR: [u8; 4] = [126u8, 135u8, 206u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.newProtocolFee),
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
    /**Function with signature `setProtocolFeeController(address)` and selector `0x2d771389`.
```solidity
function setProtocolFeeController(address controller) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeControllerCall {
        pub controller: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setProtocolFeeController(address)`](setProtocolFeeControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeControllerReturn {}
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
            impl ::core::convert::From<setProtocolFeeControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeControllerCall) -> Self {
                    (value.controller,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProtocolFeeControllerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { controller: tuple.0 }
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
            impl ::core::convert::From<setProtocolFeeControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeControllerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProtocolFeeControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProtocolFeeControllerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProtocolFeeControllerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProtocolFeeController(address)";
            const SELECTOR: [u8; 4] = [45u8, 119u8, 19u8, 137u8];
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
                        &self.controller,
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
    /**Function with signature `settle()` and selector `0x11da60b4`.
```solidity
function settle() external payable returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleCall {}
    ///Container type for the return parameters of the [`settle()`](settleCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<settleCall> for UnderlyingRustTuple<'_> {
                fn from(value: settleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<settleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: settleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for settleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = settleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "settle()";
            const SELECTOR: [u8; 4] = [17u8, 218u8, 96u8, 180u8];
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
    /**Function with signature `settleFor(address)` and selector `0x3dd45adb`.
```solidity
function settleFor(address recipient) external payable returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleForCall {
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`settleFor(address)`](settleForCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleForReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
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
            impl ::core::convert::From<settleForCall> for UnderlyingRustTuple<'_> {
                fn from(value: settleForCall) -> Self {
                    (value.recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { recipient: tuple.0 }
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
            impl ::core::convert::From<settleForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: settleForReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for settleForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = settleForReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "settleFor(address)";
            const SELECTOR: [u8; 4] = [61u8, 212u8, 90u8, 219u8];
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
                        &self.recipient,
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
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
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
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
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
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
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
    /**Function with signature `swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)` and selector `0xf3cd914c`.
```solidity
function swap(PoolKey memory key, IPoolManager.SwapParams memory params, bytes memory hookData) external returns (BalanceDelta swapDelta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub params: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)`](swapCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapReturn {
        pub swapDelta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                IPoolManager::SwapParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<swapCall> for UnderlyingRustTuple<'_> {
                fn from(value: swapCall) -> Self {
                    (value.key, value.params, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        params: tuple.1,
                        hookData: tuple.2,
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
                    (value.swapDelta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { swapDelta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapCall {
            type Parameters<'a> = (
                PoolKey,
                IPoolManager::SwapParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)";
            const SELECTOR: [u8; 4] = [243u8, 205u8, 145u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::SwapParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
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
    /**Function with signature `sync(address)` and selector `0xa5841194`.
```solidity
function sync(Currency currency) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct syncCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`sync(address)`](syncCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct syncReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Currency,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<syncCall> for UnderlyingRustTuple<'_> {
                fn from(value: syncCall) -> Self {
                    (value.currency,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syncCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { currency: tuple.0 }
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
            impl ::core::convert::From<syncReturn> for UnderlyingRustTuple<'_> {
                fn from(value: syncReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syncReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for syncCall {
            type Parameters<'a> = (Currency,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = syncReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sync(address)";
            const SELECTOR: [u8; 4] = [165u8, 132u8, 17u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Currency as alloy_sol_types::SolType>::tokenize(&self.currency),)
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
    /**Function with signature `take(address,address,uint256)` and selector `0x0b0d9c09`.
```solidity
function take(Currency currency, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct takeCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`take(address,address,uint256)`](takeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct takeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                Currency,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<takeCall> for UnderlyingRustTuple<'_> {
                fn from(value: takeCall) -> Self {
                    (value.currency, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for takeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        currency: tuple.0,
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
            impl ::core::convert::From<takeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: takeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for takeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for takeCall {
            type Parameters<'a> = (
                Currency,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = takeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "take(address,address,uint256)";
            const SELECTOR: [u8; 4] = [11u8, 13u8, 156u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency),
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
    /**Function with signature `transfer(address,uint256,uint256)` and selector `0x095bcdb6`.
```solidity
function transfer(address receiver, uint256 id, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferCall {
        pub receiver: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transfer(address,uint256,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.receiver, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receiver: tuple.0,
                        id: tuple.1,
                        amount: tuple.2,
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
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 91u8, 205u8, 182u8];
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
                        &self.receiver,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `transferFrom(address,address,uint256,uint256)` and selector `0xfe99049a`.
```solidity
function transferFrom(address sender, address receiver, uint256 id, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferFromCall {
        pub sender: alloy::sol_types::private::Address,
        pub receiver: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferFromReturn {
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.sender, value.receiver, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        receiver: tuple.1,
                        id: tuple.2,
                        amount: tuple.3,
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
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferFromReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [254u8, 153u8, 4u8, 154u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiver,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
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
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
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
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
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
                        &self.newOwner,
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
    /**Function with signature `unlock(bytes)` and selector `0x48c89491`.
```solidity
function unlock(bytes memory data) external returns (bytes memory result);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct unlockCall {
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`unlock(bytes)`](unlockCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct unlockReturn {
        pub result: alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<unlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockCall {
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
            impl ::core::convert::From<unlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unlockReturn) -> Self {
                    (value.result,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { result: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlock(bytes)";
            const SELECTOR: [u8; 4] = [72u8, 200u8, 148u8, 145u8];
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
    /**Function with signature `updateDynamicLPFee((address,address,uint24,int24,address),uint24)` and selector `0x52759651`.
```solidity
function updateDynamicLPFee(PoolKey memory key, uint24 newDynamicLPFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateDynamicLPFeeCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub newDynamicLPFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    ///Container type for the return parameters of the [`updateDynamicLPFee((address,address,uint24,int24,address),uint24)`](updateDynamicLPFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateDynamicLPFeeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateDynamicLPFeeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateDynamicLPFeeCall) -> Self {
                    (value.key, value.newDynamicLPFee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateDynamicLPFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        newDynamicLPFee: tuple.1,
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
            impl ::core::convert::From<updateDynamicLPFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateDynamicLPFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateDynamicLPFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateDynamicLPFeeCall {
            type Parameters<'a> = (PoolKey, alloy::sol_types::sol_data::Uint<24>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateDynamicLPFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateDynamicLPFee((address,address,uint24,int24,address),uint24)";
            const SELECTOR: [u8; 4] = [82u8, 117u8, 150u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.newDynamicLPFee),
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
    ///Container for all the [`PoolManager`](self) function calls.
    pub enum PoolManagerCalls {
        allowance(allowanceCall),
        approve(approveCall),
        balanceOf(balanceOfCall),
        burn(burnCall),
        clear(clearCall),
        collectProtocolFees(collectProtocolFeesCall),
        donate(donateCall),
        extsload_0(extsload_0Call),
        extsload_1(extsload_1Call),
        extsload_2(extsload_2Call),
        exttload_0(exttload_0Call),
        exttload_1(exttload_1Call),
        initialize(initializeCall),
        isOperator(isOperatorCall),
        mint(mintCall),
        modifyLiquidity(modifyLiquidityCall),
        owner(ownerCall),
        protocolFeeController(protocolFeeControllerCall),
        protocolFeesAccrued(protocolFeesAccruedCall),
        setOperator(setOperatorCall),
        setProtocolFee(setProtocolFeeCall),
        setProtocolFeeController(setProtocolFeeControllerCall),
        settle(settleCall),
        settleFor(settleForCall),
        supportsInterface(supportsInterfaceCall),
        swap(swapCall),
        sync(syncCall),
        take(takeCall),
        transfer(transferCall),
        transferFrom(transferFromCall),
        transferOwnership(transferOwnershipCall),
        unlock(unlockCall),
        updateDynamicLPFee(updateDynamicLPFeeCall),
    }
    #[automatically_derived]
    impl PoolManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 253u8, 213u8, 142u8],
            [1u8, 255u8, 201u8, 167u8],
            [9u8, 91u8, 205u8, 182u8],
            [11u8, 13u8, 156u8, 9u8],
            [17u8, 218u8, 96u8, 180u8],
            [21u8, 110u8, 41u8, 246u8],
            [30u8, 46u8, 174u8, 175u8],
            [35u8, 66u8, 102u8, 215u8],
            [45u8, 119u8, 19u8, 137u8],
            [53u8, 253u8, 99u8, 26u8],
            [61u8, 212u8, 90u8, 219u8],
            [66u8, 106u8, 132u8, 147u8],
            [72u8, 200u8, 148u8, 145u8],
            [82u8, 117u8, 150u8, 81u8],
            [85u8, 138u8, 114u8, 151u8],
            [89u8, 138u8, 249u8, 231u8],
            [90u8, 107u8, 207u8, 218u8],
            [98u8, 118u8, 203u8, 190u8],
            [126u8, 135u8, 206u8, 125u8],
            [128u8, 240u8, 180u8, 76u8],
            [129u8, 97u8, 184u8, 116u8],
            [141u8, 165u8, 203u8, 91u8],
            [151u8, 232u8, 205u8, 78u8],
            [155u8, 246u8, 100u8, 95u8],
            [165u8, 132u8, 17u8, 148u8],
            [182u8, 54u8, 60u8, 242u8],
            [219u8, 208u8, 53u8, 255u8],
            [240u8, 45u8, 227u8, 178u8],
            [241u8, 53u8, 186u8, 170u8],
            [242u8, 253u8, 227u8, 139u8],
            [243u8, 205u8, 145u8, 76u8],
            [245u8, 41u8, 138u8, 202u8],
            [254u8, 153u8, 4u8, 154u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolManagerCalls {
        const NAME: &'static str = "PoolManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::allowance(_) => {
                    <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::burn(_) => <burnCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::clear(_) => <clearCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::collectProtocolFees(_) => {
                    <collectProtocolFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::donate(_) => <donateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extsload_0(_) => {
                    <extsload_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::extsload_1(_) => {
                    <extsload_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::extsload_2(_) => {
                    <extsload_2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exttload_0(_) => {
                    <exttload_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exttload_1(_) => {
                    <exttload_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperator(_) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mint(_) => <mintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::modifyLiquidity(_) => {
                    <modifyLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::protocolFeeController(_) => {
                    <protocolFeeControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::protocolFeesAccrued(_) => {
                    <protocolFeesAccruedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperator(_) => {
                    <setOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProtocolFee(_) => {
                    <setProtocolFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProtocolFeeController(_) => {
                    <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::settle(_) => <settleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::settleFor(_) => {
                    <settleForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swap(_) => <swapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::sync(_) => <syncCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::take(_) => <takeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => {
                    <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlock(_) => <unlockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateDynamicLPFee(_) => {
                    <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<PoolManagerCalls>] = &[
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::transfer)
                    }
                    transfer
                },
                {
                    fn take(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <takeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::take)
                    }
                    take
                },
                {
                    fn settle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <settleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::settle)
                    }
                    settle
                },
                {
                    fn mint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::mint)
                    }
                    mint
                },
                {
                    fn extsload_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <extsload_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::extsload_0)
                    }
                    extsload_0
                },
                {
                    fn donate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <donateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::donate)
                    }
                    donate
                },
                {
                    fn setProtocolFeeController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::setProtocolFeeController)
                    }
                    setProtocolFeeController
                },
                {
                    fn extsload_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <extsload_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::extsload_1)
                    }
                    extsload_1
                },
                {
                    fn settleFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <settleForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::settleFor)
                    }
                    settleFor
                },
                {
                    fn approve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::approve)
                    }
                    approve
                },
                {
                    fn unlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <unlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::unlock)
                    }
                    unlock
                },
                {
                    fn updateDynamicLPFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::updateDynamicLPFee)
                    }
                    updateDynamicLPFee
                },
                {
                    fn setOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <setOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::setOperator)
                    }
                    setOperator
                },
                {
                    fn allowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::allowance)
                    }
                    allowance
                },
                {
                    fn modifyLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <modifyLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::modifyLiquidity)
                    }
                    modifyLiquidity
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setProtocolFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <setProtocolFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::setProtocolFee)
                    }
                    setProtocolFee
                },
                {
                    fn clear(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <clearCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::clear)
                    }
                    clear
                },
                {
                    fn collectProtocolFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <collectProtocolFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::collectProtocolFees)
                    }
                    collectProtocolFees
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn protocolFeesAccrued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <protocolFeesAccruedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::protocolFeesAccrued)
                    }
                    protocolFeesAccrued
                },
                {
                    fn exttload_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <exttload_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::exttload_0)
                    }
                    exttload_0
                },
                {
                    fn sync(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <syncCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::sync)
                    }
                    sync
                },
                {
                    fn isOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <isOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::isOperator)
                    }
                    isOperator
                },
                {
                    fn extsload_2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <extsload_2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::extsload_2)
                    }
                    extsload_2
                },
                {
                    fn protocolFeeController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <protocolFeeControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::protocolFeeController)
                    }
                    protocolFeeController
                },
                {
                    fn exttload_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <exttload_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::exttload_1)
                    }
                    exttload_1
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn swap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <swapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::swap)
                    }
                    swap
                },
                {
                    fn burn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::burn)
                    }
                    burn
                },
                {
                    fn transferFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::transferFrom)
                    }
                    transferFrom
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
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::clear(inner) => {
                    <clearCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::collectProtocolFees(inner) => {
                    <collectProtocolFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::donate(inner) => {
                    <donateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload_0(inner) => {
                    <extsload_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload_1(inner) => {
                    <extsload_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload_2(inner) => {
                    <extsload_2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exttload_0(inner) => {
                    <exttload_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exttload_1(inner) => {
                    <exttload_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::modifyLiquidity(inner) => {
                    <modifyLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::protocolFeeController(inner) => {
                    <protocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::protocolFeesAccrued(inner) => {
                    <protocolFeesAccruedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperator(inner) => {
                    <setOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProtocolFee(inner) => {
                    <setProtocolFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProtocolFeeController(inner) => {
                    <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::settle(inner) => {
                    <settleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::settleFor(inner) => {
                    <settleForCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swap(inner) => {
                    <swapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sync(inner) => {
                    <syncCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::take(inner) => {
                    <takeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlock(inner) => {
                    <unlockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateDynamicLPFee(inner) => {
                    <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::clear(inner) => {
                    <clearCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::collectProtocolFees(inner) => {
                    <collectProtocolFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::donate(inner) => {
                    <donateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::extsload_0(inner) => {
                    <extsload_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::extsload_1(inner) => {
                    <extsload_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::extsload_2(inner) => {
                    <extsload_2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::exttload_0(inner) => {
                    <exttload_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::exttload_1(inner) => {
                    <exttload_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::modifyLiquidity(inner) => {
                    <modifyLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::protocolFeeController(inner) => {
                    <protocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::protocolFeesAccrued(inner) => {
                    <protocolFeesAccruedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperator(inner) => {
                    <setOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProtocolFee(inner) => {
                    <setProtocolFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProtocolFeeController(inner) => {
                    <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::settle(inner) => {
                    <settleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::settleFor(inner) => {
                    <settleForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swap(inner) => {
                    <swapCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::sync(inner) => {
                    <syncCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::take(inner) => {
                    <takeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlock(inner) => {
                    <unlockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateDynamicLPFee(inner) => {
                    <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PoolManager`](self) custom errors.
    pub enum PoolManagerErrors {
        AlreadyUnlocked(AlreadyUnlocked),
        ContractUnlocked(ContractUnlocked),
        CurrenciesOutOfOrderOrEqual(CurrenciesOutOfOrderOrEqual),
        CurrencyNotSettled(CurrencyNotSettled),
        DelegateCallNotAllowed(DelegateCallNotAllowed),
        InvalidCaller(InvalidCaller),
        ManagerLocked(ManagerLocked),
        MustClearExactPositiveDelta(MustClearExactPositiveDelta),
        NonzeroNativeValue(NonzeroNativeValue),
        PoolNotInitialized(PoolNotInitialized),
        ProtocolFeeTooLarge(ProtocolFeeTooLarge),
        SwapAmountCannotBeZero(SwapAmountCannotBeZero),
        TickSpacingTooLarge(TickSpacingTooLarge),
        TickSpacingTooSmall(TickSpacingTooSmall),
        UnauthorizedDynamicLPFeeUpdate(UnauthorizedDynamicLPFeeUpdate),
    }
    #[automatically_derived]
    impl PoolManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 137u8, 67u8, 142u8],
            [48u8, 210u8, 22u8, 65u8],
            [62u8, 95u8, 79u8, 214u8],
            [72u8, 106u8, 163u8, 7u8],
            [72u8, 245u8, 195u8, 237u8],
            [80u8, 144u8, 214u8, 198u8],
            [82u8, 18u8, 203u8, 161u8],
            [84u8, 227u8, 202u8, 13u8],
            [110u8, 108u8, 152u8, 48u8],
            [167u8, 171u8, 226u8, 247u8],
            [176u8, 236u8, 132u8, 158u8],
            [183u8, 0u8, 36u8, 248u8],
            [189u8, 167u8, 58u8, 191u8],
            [190u8, 139u8, 133u8, 7u8],
            [233u8, 233u8, 5u8, 136u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolManagerErrors {
        const NAME: &'static str = "PoolManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 15usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyUnlocked(_) => {
                    <AlreadyUnlocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractUnlocked(_) => {
                    <ContractUnlocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrenciesOutOfOrderOrEqual(_) => {
                    <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrencyNotSettled(_) => {
                    <CurrencyNotSettled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelegateCallNotAllowed(_) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCaller(_) => {
                    <InvalidCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ManagerLocked(_) => {
                    <ManagerLocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustClearExactPositiveDelta(_) => {
                    <MustClearExactPositiveDelta as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonzeroNativeValue(_) => {
                    <NonzeroNativeValue as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PoolNotInitialized(_) => {
                    <PoolNotInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProtocolFeeTooLarge(_) => {
                    <ProtocolFeeTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SwapAmountCannotBeZero(_) => {
                    <SwapAmountCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TickSpacingTooLarge(_) => {
                    <TickSpacingTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TickSpacingTooSmall(_) => {
                    <TickSpacingTooSmall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnauthorizedDynamicLPFeeUpdate(_) => {
                    <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<PoolManagerErrors>] = &[
                {
                    fn DelegateCallNotAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::DelegateCallNotAllowed)
                    }
                    DelegateCallNotAllowed
                },
                {
                    fn UnauthorizedDynamicLPFeeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::UnauthorizedDynamicLPFeeUpdate)
                    }
                    UnauthorizedDynamicLPFeeUpdate
                },
                {
                    fn ContractUnlocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ContractUnlocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ContractUnlocked)
                    }
                    ContractUnlocked
                },
                {
                    fn PoolNotInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <PoolNotInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::PoolNotInitialized)
                    }
                    PoolNotInitialized
                },
                {
                    fn InvalidCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <InvalidCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::InvalidCaller)
                    }
                    InvalidCaller
                },
                {
                    fn AlreadyUnlocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <AlreadyUnlocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::AlreadyUnlocked)
                    }
                    AlreadyUnlocked
                },
                {
                    fn CurrencyNotSettled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <CurrencyNotSettled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::CurrencyNotSettled)
                    }
                    CurrencyNotSettled
                },
                {
                    fn ManagerLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ManagerLocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ManagerLocked)
                    }
                    ManagerLocked
                },
                {
                    fn CurrenciesOutOfOrderOrEqual(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::CurrenciesOutOfOrderOrEqual)
                    }
                    CurrenciesOutOfOrderOrEqual
                },
                {
                    fn ProtocolFeeTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ProtocolFeeTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ProtocolFeeTooLarge)
                    }
                    ProtocolFeeTooLarge
                },
                {
                    fn NonzeroNativeValue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <NonzeroNativeValue as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::NonzeroNativeValue)
                    }
                    NonzeroNativeValue
                },
                {
                    fn TickSpacingTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <TickSpacingTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::TickSpacingTooLarge)
                    }
                    TickSpacingTooLarge
                },
                {
                    fn MustClearExactPositiveDelta(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <MustClearExactPositiveDelta as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::MustClearExactPositiveDelta)
                    }
                    MustClearExactPositiveDelta
                },
                {
                    fn SwapAmountCannotBeZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <SwapAmountCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::SwapAmountCannotBeZero)
                    }
                    SwapAmountCannotBeZero
                },
                {
                    fn TickSpacingTooSmall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <TickSpacingTooSmall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::TickSpacingTooSmall)
                    }
                    TickSpacingTooSmall
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
                Self::AlreadyUnlocked(inner) => {
                    <AlreadyUnlocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractUnlocked(inner) => {
                    <ContractUnlocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrenciesOutOfOrderOrEqual(inner) => {
                    <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrencyNotSettled(inner) => {
                    <CurrencyNotSettled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelegateCallNotAllowed(inner) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ManagerLocked(inner) => {
                    <ManagerLocked as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MustClearExactPositiveDelta(inner) => {
                    <MustClearExactPositiveDelta as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonzeroNativeValue(inner) => {
                    <NonzeroNativeValue as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PoolNotInitialized(inner) => {
                    <PoolNotInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProtocolFeeTooLarge(inner) => {
                    <ProtocolFeeTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SwapAmountCannotBeZero(inner) => {
                    <SwapAmountCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TickSpacingTooLarge(inner) => {
                    <TickSpacingTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TickSpacingTooSmall(inner) => {
                    <TickSpacingTooSmall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnauthorizedDynamicLPFeeUpdate(inner) => {
                    <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyUnlocked(inner) => {
                    <AlreadyUnlocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContractUnlocked(inner) => {
                    <ContractUnlocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrenciesOutOfOrderOrEqual(inner) => {
                    <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrencyNotSettled(inner) => {
                    <CurrencyNotSettled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelegateCallNotAllowed(inner) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ManagerLocked(inner) => {
                    <ManagerLocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustClearExactPositiveDelta(inner) => {
                    <MustClearExactPositiveDelta as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonzeroNativeValue(inner) => {
                    <NonzeroNativeValue as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PoolNotInitialized(inner) => {
                    <PoolNotInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProtocolFeeTooLarge(inner) => {
                    <ProtocolFeeTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SwapAmountCannotBeZero(inner) => {
                    <SwapAmountCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TickSpacingTooLarge(inner) => {
                    <TickSpacingTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TickSpacingTooSmall(inner) => {
                    <TickSpacingTooSmall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnauthorizedDynamicLPFeeUpdate(inner) => {
                    <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PoolManager`](self) events.
    pub enum PoolManagerEvents {
        Approval(Approval),
        Donate(Donate),
        Initialize(Initialize),
        ModifyLiquidity(ModifyLiquidity),
        OperatorSet(OperatorSet),
        OwnershipTransferred(OwnershipTransferred),
        ProtocolFeeControllerUpdated(ProtocolFeeControllerUpdated),
        ProtocolFeeUpdated(ProtocolFeeUpdated),
        Swap(Swap),
        Transfer(Transfer),
    }
    #[automatically_derived]
    impl PoolManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                27u8,
                61u8,
                126u8,
                219u8,
                46u8,
                156u8,
                11u8,
                14u8,
                124u8,
                82u8,
                91u8,
                32u8,
                170u8,
                174u8,
                240u8,
                245u8,
                148u8,
                13u8,
                46u8,
                215u8,
                22u8,
                99u8,
                199u8,
                211u8,
                146u8,
                102u8,
                236u8,
                175u8,
                172u8,
                114u8,
                136u8,
                89u8,
            ],
            [
                41u8,
                239u8,
                5u8,
                202u8,
                175u8,
                249u8,
                64u8,
                75u8,
                124u8,
                182u8,
                209u8,
                192u8,
                233u8,
                187u8,
                174u8,
                158u8,
                170u8,
                122u8,
                178u8,
                84u8,
                31u8,
                235u8,
                161u8,
                169u8,
                196u8,
                36u8,
                133u8,
                148u8,
                192u8,
                129u8,
                86u8,
                203u8,
            ],
            [
                64u8,
                233u8,
                206u8,
                203u8,
                159u8,
                95u8,
                31u8,
                28u8,
                91u8,
                156u8,
                151u8,
                222u8,
                194u8,
                145u8,
                123u8,
                126u8,
                233u8,
                46u8,
                87u8,
                186u8,
                85u8,
                99u8,
                112u8,
                141u8,
                172u8,
                169u8,
                77u8,
                216u8,
                74u8,
                215u8,
                17u8,
                47u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                179u8,
                253u8,
                80u8,
                113u8,
                131u8,
                88u8,
                135u8,
                86u8,
                122u8,
                6u8,
                113u8,
                21u8,
                17u8,
                33u8,
                137u8,
                77u8,
                220u8,
                204u8,
                40u8,
                66u8,
                241u8,
                209u8,
                11u8,
                237u8,
                173u8,
                19u8,
                224u8,
                209u8,
                124u8,
                172u8,
                233u8,
                167u8,
            ],
            [
                180u8,
                189u8,
                142u8,
                245u8,
                61u8,
                246u8,
                144u8,
                185u8,
                148u8,
                61u8,
                51u8,
                24u8,
                153u8,
                96u8,
                6u8,
                219u8,
                184u8,
                42u8,
                37u8,
                245u8,
                71u8,
                25u8,
                216u8,
                200u8,
                3u8,
                91u8,
                81u8,
                106u8,
                42u8,
                91u8,
                138u8,
                204u8,
            ],
            [
                206u8,
                181u8,
                118u8,
                217u8,
                241u8,
                94u8,
                78u8,
                32u8,
                15u8,
                219u8,
                80u8,
                150u8,
                214u8,
                77u8,
                93u8,
                253u8,
                102u8,
                126u8,
                22u8,
                222u8,
                242u8,
                12u8,
                30u8,
                239u8,
                209u8,
                66u8,
                86u8,
                216u8,
                227u8,
                250u8,
                162u8,
                103u8,
            ],
            [
                221u8,
                70u8,
                110u8,
                103u8,
                78u8,
                165u8,
                87u8,
                245u8,
                98u8,
                149u8,
                226u8,
                208u8,
                33u8,
                138u8,
                18u8,
                94u8,
                164u8,
                180u8,
                240u8,
                246u8,
                243u8,
                48u8,
                123u8,
                149u8,
                248u8,
                94u8,
                97u8,
                16u8,
                131u8,
                141u8,
                100u8,
                56u8,
            ],
            [
                233u8,
                196u8,
                37u8,
                147u8,
                231u8,
                31u8,
                132u8,
                64u8,
                59u8,
                132u8,
                53u8,
                44u8,
                209u8,
                104u8,
                214u8,
                147u8,
                226u8,
                201u8,
                252u8,
                209u8,
                253u8,
                188u8,
                195u8,
                254u8,
                178u8,
                29u8,
                146u8,
                180u8,
                62u8,
                102u8,
                150u8,
                249u8,
            ],
            [
                242u8,
                8u8,
                244u8,
                145u8,
                39u8,
                130u8,
                253u8,
                37u8,
                199u8,
                241u8,
                20u8,
                202u8,
                55u8,
                35u8,
                162u8,
                213u8,
                221u8,
                111u8,
                59u8,
                204u8,
                58u8,
                200u8,
                219u8,
                90u8,
                246u8,
                59u8,
                170u8,
                133u8,
                247u8,
                17u8,
                213u8,
                236u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for PoolManagerEvents {
        const NAME: &'static str = "PoolManagerEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Approval)
                }
                Some(<Donate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Donate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Donate)
                }
                Some(<Initialize as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialize as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialize)
                }
                Some(<ModifyLiquidity as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ModifyLiquidity as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ModifyLiquidity)
                }
                Some(<OperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSet)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <ProtocolFeeControllerUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolFeeControllerUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProtocolFeeControllerUpdated)
                }
                Some(
                    <ProtocolFeeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolFeeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProtocolFeeUpdated)
                }
                Some(<Swap as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Swap as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Swap)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for PoolManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Donate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialize(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ModifyLiquidity(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolFeeControllerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolFeeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Swap(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Donate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialize(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ModifyLiquidity(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolFeeControllerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolFeeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Swap(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PoolManager`](self) contract instance.

See the [wrapper's documentation](`PoolManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolManagerInstance<T, P, N> {
        PoolManagerInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PoolManagerInstance<T, P, N>>,
    > {
        PoolManagerInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        PoolManagerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`PoolManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolManager`](self) contract instance.

See the [wrapper's documentation](`PoolManagerInstance`) for more details.*/
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
        ) -> alloy_contract::Result<PoolManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> PoolManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolManagerInstance<T, P, N> {
            PoolManagerInstance {
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
    > PoolManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowanceCall, N> {
            self.call_builder(
                &allowanceCall {
                    owner,
                    spender,
                    id,
                },
            )
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveCall, N> {
            self.call_builder(&approveCall { spender, id, amount })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            owner: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { owner, id })
        }
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            from: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnCall, N> {
            self.call_builder(&burnCall { from, id, amount })
        }
        ///Creates a new call builder for the [`clear`] function.
        pub fn clear(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, clearCall, N> {
            self.call_builder(&clearCall { currency, amount })
        }
        ///Creates a new call builder for the [`collectProtocolFees`] function.
        pub fn collectProtocolFees(
            &self,
            recipient: alloy::sol_types::private::Address,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, collectProtocolFeesCall, N> {
            self.call_builder(
                &collectProtocolFeesCall {
                    recipient,
                    currency,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`donate`] function.
        pub fn donate(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            amount0: alloy::sol_types::private::primitives::aliases::U256,
            amount1: alloy::sol_types::private::primitives::aliases::U256,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, donateCall, N> {
            self.call_builder(
                &donateCall {
                    key,
                    amount0,
                    amount1,
                    hookData,
                },
            )
        }
        ///Creates a new call builder for the [`extsload_0`] function.
        pub fn extsload_0(
            &self,
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsload_0Call, N> {
            self.call_builder(&extsload_0Call { slot })
        }
        ///Creates a new call builder for the [`extsload_1`] function.
        pub fn extsload_1(
            &self,
            startSlot: alloy::sol_types::private::FixedBytes<32>,
            nSlots: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsload_1Call, N> {
            self.call_builder(
                &extsload_1Call {
                    startSlot,
                    nSlots,
                },
            )
        }
        ///Creates a new call builder for the [`extsload_2`] function.
        pub fn extsload_2(
            &self,
            slots: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsload_2Call, N> {
            self.call_builder(&extsload_2Call { slots })
        }
        ///Creates a new call builder for the [`exttload_0`] function.
        pub fn exttload_0(
            &self,
            slots: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, exttload_0Call, N> {
            self.call_builder(&exttload_0Call { slots })
        }
        ///Creates a new call builder for the [`exttload_1`] function.
        pub fn exttload_1(
            &self,
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, exttload_1Call, N> {
            self.call_builder(&exttload_1Call { slot })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    key,
                    sqrtPriceX96,
                },
            )
        }
        ///Creates a new call builder for the [`isOperator`] function.
        pub fn isOperator(
            &self,
            owner: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorCall, N> {
            self.call_builder(&isOperatorCall { owner, operator })
        }
        ///Creates a new call builder for the [`mint`] function.
        pub fn mint(
            &self,
            to: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintCall, N> {
            self.call_builder(&mintCall { to, id, amount })
        }
        ///Creates a new call builder for the [`modifyLiquidity`] function.
        pub fn modifyLiquidity(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyLiquidityCall, N> {
            self.call_builder(
                &modifyLiquidityCall {
                    key,
                    params,
                    hookData,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`protocolFeeController`] function.
        pub fn protocolFeeController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, protocolFeeControllerCall, N> {
            self.call_builder(&protocolFeeControllerCall {})
        }
        ///Creates a new call builder for the [`protocolFeesAccrued`] function.
        pub fn protocolFeesAccrued(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, protocolFeesAccruedCall, N> {
            self.call_builder(
                &protocolFeesAccruedCall {
                    currency,
                },
            )
        }
        ///Creates a new call builder for the [`setOperator`] function.
        pub fn setOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            approved: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorCall, N> {
            self.call_builder(
                &setOperatorCall {
                    operator,
                    approved,
                },
            )
        }
        ///Creates a new call builder for the [`setProtocolFee`] function.
        pub fn setProtocolFee(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            newProtocolFee: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProtocolFeeCall, N> {
            self.call_builder(
                &setProtocolFeeCall {
                    key,
                    newProtocolFee,
                },
            )
        }
        ///Creates a new call builder for the [`setProtocolFeeController`] function.
        pub fn setProtocolFeeController(
            &self,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProtocolFeeControllerCall, N> {
            self.call_builder(
                &setProtocolFeeControllerCall {
                    controller,
                },
            )
        }
        ///Creates a new call builder for the [`settle`] function.
        pub fn settle(&self) -> alloy_contract::SolCallBuilder<T, &P, settleCall, N> {
            self.call_builder(&settleCall {})
        }
        ///Creates a new call builder for the [`settleFor`] function.
        pub fn settleFor(
            &self,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, settleForCall, N> {
            self.call_builder(&settleForCall { recipient })
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`swap`] function.
        pub fn swap(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapCall, N> {
            self.call_builder(&swapCall { key, params, hookData })
        }
        ///Creates a new call builder for the [`sync`] function.
        pub fn sync(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, syncCall, N> {
            self.call_builder(&syncCall { currency })
        }
        ///Creates a new call builder for the [`take`] function.
        pub fn take(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, takeCall, N> {
            self.call_builder(&takeCall { currency, to, amount })
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            receiver: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferCall, N> {
            self.call_builder(
                &transferCall {
                    receiver,
                    id,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            sender: alloy::sol_types::private::Address,
            receiver: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferFromCall, N> {
            self.call_builder(
                &transferFromCall {
                    sender,
                    receiver,
                    id,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unlock`] function.
        pub fn unlock(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockCall, N> {
            self.call_builder(&unlockCall { data })
        }
        ///Creates a new call builder for the [`updateDynamicLPFee`] function.
        pub fn updateDynamicLPFee(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            newDynamicLPFee: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateDynamicLPFeeCall, N> {
            self.call_builder(
                &updateDynamicLPFeeCall {
                    key,
                    newDynamicLPFee,
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
    > PoolManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<T, &P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`Donate`] event.
        pub fn Donate_filter(&self) -> alloy_contract::Event<T, &P, Donate, N> {
            self.event_filter::<Donate>()
        }
        ///Creates a new event filter for the [`Initialize`] event.
        pub fn Initialize_filter(&self) -> alloy_contract::Event<T, &P, Initialize, N> {
            self.event_filter::<Initialize>()
        }
        ///Creates a new event filter for the [`ModifyLiquidity`] event.
        pub fn ModifyLiquidity_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ModifyLiquidity, N> {
            self.event_filter::<ModifyLiquidity>()
        }
        ///Creates a new event filter for the [`OperatorSet`] event.
        pub fn OperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSet, N> {
            self.event_filter::<OperatorSet>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`ProtocolFeeControllerUpdated`] event.
        pub fn ProtocolFeeControllerUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProtocolFeeControllerUpdated, N> {
            self.event_filter::<ProtocolFeeControllerUpdated>()
        }
        ///Creates a new event filter for the [`ProtocolFeeUpdated`] event.
        pub fn ProtocolFeeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProtocolFeeUpdated, N> {
            self.event_filter::<ProtocolFeeUpdated>()
        }
        ///Creates a new event filter for the [`Swap`] event.
        pub fn Swap_filter(&self) -> alloy_contract::Event<T, &P, Swap, N> {
            self.event_filter::<Swap>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
    }
}
