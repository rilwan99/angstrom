pub use pool_manager::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod pool_manager {
    /*
    const _: () = {
        ::core::include_bytes!(
            "/home/will/ghq/github.com/SorellaLabs/Angstrom/anvil/abi/PoolManager.json",
        );
    };
    */
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("controllerGasLimit"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "uint256"
                    ),)
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("MAX_TICK_SPACING"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_TICK_SPACING"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int24"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MIN_PROTOCOL_FEE_DENOMINATOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MIN_PROTOCOL_FEE_DENOMINATOR",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MIN_TICK_SPACING"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MIN_TICK_SPACING"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int24"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("accounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectHookFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collectHookFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currency"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Currency"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountCollected"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("collectProtocolFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("collectProtocolFees",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currency"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Currency"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amountCollected"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currencyDelta"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("currencyDelta"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("locker"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currency"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Currency"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("currencyDelta"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("donate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("donate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("delta"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("BalanceDelta"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("extsload"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("extsload"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slot"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                )
                            },],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                )
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("extsload"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("startSlot"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    )
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nSlots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    )
                                },
                            ],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                )
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("PoolId"),
                                    )
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_owner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    )
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    )
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    )
                                },
                            ],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                )
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getLiquidity"),
                            inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("PoolId"),
                                )
                            },],
                            outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                )
                            },],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLock"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("i"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("locker"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPosition"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPosition"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("PoolId"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("position"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Position.Info"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSlot0"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSlot0"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("PoolId"),
                            )
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tick"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("protocolSwapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("protocolWithdrawFee",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hookSwapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hookWithdrawFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                )
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hookFeesAccrued"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("hookFeesAccrued"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hookAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currency"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Currency"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tick"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int24"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lock"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lock"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("result"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lockData"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("length"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nonzeroDeltaCount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                )
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mint"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mint"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currency"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Currency"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("modifyPosition"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("modifyPosition"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPoolManager.ModifyPositionParams",
                                    ),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("delta"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("BalanceDelta"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onERC1155BatchReceived",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("values"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("onERC1155Received"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("owner"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pools"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("PoolId"),
                            )
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slot0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Pool.Slot0"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeGrowthGlobal0X128",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeGrowthGlobal1X128",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                )
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFeeController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("protocolFeeController",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IProtocolFeeController",),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFeesAccrued"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("protocolFeesAccrued",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("currency"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Currency"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("reservesOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("reservesOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("currency"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Currency"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("values"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                )
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                )
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                )
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setHookFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setHookFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                            )
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOwner"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOwner"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("_owner"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFeeController"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProtocolFeeController",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("controller"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract IProtocolFeeController",),
                            )
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProtocolFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("key"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                            )
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("settle"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("settle"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("currency"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("Currency"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("paid"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("key"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct IPoolManager.SwapParams",
                                    ),
                                )
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("delta"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("BalanceDelta"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("take"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("take"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("currency"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("Currency"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uri"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uri"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            )
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            )
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HookFeeUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("HookFeeUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hookSwapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hookWithdrawFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("currency0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("currency1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tickSpacing"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hooks"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ModifyPosition"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ModifyPosition"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnerChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OwnerChanged"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeControllerUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProtocolFeeControllerUpdated",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("protocolFeeController",),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false
                        },],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ProtocolFeeUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("protocolSwapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("protocolWithdrawFee",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Swap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sqrtPriceX96"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tick"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("values"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferSingle"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferSingle"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false
                            },
                        ],
                        anonymous: false
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("URI"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("URI"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true
                            },
                        ],
                        anonymous: false
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("CannotUpdateEmptyPosition"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CannotUpdateEmptyPosition",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CurrencyNotSettled"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("CurrencyNotSettled"),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DelegateCallNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DelegateCallNotAllowed",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InsufficientBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InsufficientBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("needed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidApprover"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidApprover",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("approver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidArrayLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidArrayLength",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("idsLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("valuesLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                )
                            },
                        ]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidOperator",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("operator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidReceiver"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidReceiver",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("receiver"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155InvalidSender",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sender"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC1155MissingApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1155MissingApprovalForAll",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                )
                            },
                        ]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ERC20TransferFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC20TransferFailed",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("FeeTooLarge"),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HookAddressNotValid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("HookAddressNotValid",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("hooks"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCaller"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidCaller"),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidHookResponse"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidHookResponse",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSqrtRatio"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSqrtRatio"),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTick"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTick"),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockedBy"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LockedBy"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("locker"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MaxCurrenciesTouched"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MaxCurrenciesTouched",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeTransferFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NativeTransferFailed",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoLiquidityToReceiveFees"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoLiquidityToReceiveFees",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotPoolManagerToken"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotPoolManagerToken",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolAlreadyInitialized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolAlreadyInitialized",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolNotInitialized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolNotInitialized"),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PriceLimitAlreadyExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PriceLimitAlreadyExceeded",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sqrtPriceCurrentX96",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint160"),
                                )
                            },
                        ]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PriceLimitOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PriceLimitOutOfBounds",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("sqrtPriceLimitX96"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(160usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint160"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProtocolFeeCannotBeFetched"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ProtocolFeeCannotBeFetched",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapAmountCannotBeZero"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SwapAmountCannotBeZero",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TickLiquidityOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TickLiquidityOverflow",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tick"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int24"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TickLowerOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TickLowerOutOfBounds",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tickLower"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int24"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TickMisaligned"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TickMisaligned"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tick"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickSpacing"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                        ]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TickSpacingTooLarge"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TickSpacingTooLarge",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TickSpacingTooSmall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TickSpacingTooSmall",),
                        inputs: ::std::vec![]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TickUpperOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TickUpperOutOfBounds",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int24"),
                            )
                        },]
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TicksMisordered"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TicksMisordered"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickLower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tickUpper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int24"),
                                )
                            },
                        ]
                    },],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static POOLMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct PoolManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PoolManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PoolManager<M> {
        type Target = ::ethers::contract::Contract<M>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PoolManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PoolManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PoolManager))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PoolManager<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), POOLMANAGER_ABI.clone(), client))
        }

        ///Calls the contract's `MAX_TICK_SPACING` (0x60460f06) function
        pub fn max_tick_spacing(&self) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([96, 70, 15, 6], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `MIN_PROTOCOL_FEE_DENOMINATOR` (0x84e41c5e)
        /// function
        pub fn min_protocol_fee_denominator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([132, 228, 28, 94], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `MIN_TICK_SPACING` (0x07eff0dd) function
        pub fn min_tick_spacing(&self) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([7, 239, 240, 221], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `balanceOf` (0x00fdd58e) function
        pub fn balance_of(
            &self,
            account: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (account, id))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `balanceOfBatch` (0x4e1273f4) function
        pub fn balance_of_batch(
            &self,
            accounts: ::std::vec::Vec<::ethers::core::types::Address>,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([78, 18, 115, 244], (accounts, ids))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `collectHookFees` (0xeffd18c0) function
        pub fn collect_hook_fees(
            &self,
            recipient: ::ethers::core::types::Address,
            currency: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([239, 253, 24, 192], (recipient, currency, amount))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `collectProtocolFees` (0x8161b874) function
        pub fn collect_protocol_fees(
            &self,
            recipient: ::ethers::core::types::Address,
            currency: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([129, 97, 184, 116], (recipient, currency, amount))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `currencyDelta` (0xa54b2831) function
        pub fn currency_delta(
            &self,
            locker: ::ethers::core::types::Address,
            currency: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([165, 75, 40, 49], (locker, currency))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `donate` (0xa67dd8f3) function
        pub fn donate(
            &self,
            key: PoolKey,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([166, 125, 216, 243], (key, amount_0, amount_1))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `extsload` (0x1e2eaeaf) function
        pub fn extsload(
            &self,
            slot: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([30, 46, 174, 175], slot)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `extsload` (0x35fd631a) function
        pub fn extsload_with_start_slot(
            &self,
            start_slot: [u8; 32],
            n_slots: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([53, 253, 99, 26], (start_slot, n_slots))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `getLiquidity` (0x33aa955b) function
        pub fn get_liquidity_with_owner(
            &self,
            id: [u8; 32],
            owner: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([51, 170, 149, 91], (id, owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `getLiquidity` (0xfa6793d5) function
        pub fn get_liquidity(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([250, 103, 147, 213], id)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `getLock` (0xd68f4dd1) function
        pub fn get_lock(
            &self,
            i: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([214, 143, 77, 209], i)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `getPosition` (0x048d9c70) function
        pub fn get_position(
            &self,
            id: [u8; 32],
            owner: ::ethers::core::types::Address,
            tick_lower: i32,
            tick_upper: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, Info> {
            self.0
                .method_hash([4, 141, 156, 112], (id, owner, tick_lower, tick_upper))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `getSlot0` (0xc815641c) function
        pub fn get_slot_0(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, i32, u8, u8, u8, u8),
        > {
            self.0
                .method_hash([200, 21, 100, 28], id)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `hookFeesAccrued` (0xb4c41939) function
        pub fn hook_fees_accrued(
            &self,
            hook_address: ::ethers::core::types::Address,
            currency: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([180, 196, 25, 57], (hook_address, currency))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `initialize` (0x6276cbbe) function
        pub fn initialize(
            &self,
            key: PoolKey,
            sqrt_price_x96: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, i32> {
            self.0
                .method_hash([98, 118, 203, 190], (key, sqrt_price_x96))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            account: ::ethers::core::types::Address,
            operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (account, operator))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `lock` (0x81548319) function
        pub fn lock(
            &self,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([129, 84, 131, 25], data)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `lockData` (0xf8fcd156) function
        pub fn lock_data(&self) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([248, 252, 209, 86], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `mint` (0xc6c3bbe6) function
        pub fn mint(
            &self,
            currency: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 195, 187, 230], (currency, to, amount))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `modifyPosition` (0x555a4733) function
        pub fn modify_position(
            &self,
            key: PoolKey,
            params: ModifyPositionParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([85, 90, 71, 51], (key, params))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, ids, values, p4))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, id, value, p4))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `pools` (0xb5217bb4) function
        pub fn pools(
            &self,
            id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (Slot0, ::ethers::core::types::U256, ::ethers::core::types::U256, u128),
        > {
            self.0
                .method_hash([181, 33, 123, 180], id)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `protocolFeeController` (0xf02de3b2) function
        pub fn protocol_fee_controller(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([240, 45, 227, 178], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `protocolFeesAccrued` (0x97e8cd4e) function
        pub fn protocol_fees_accrued(
            &self,
            currency: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([151, 232, 205, 78], currency)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `reservesOf` (0x93c85a21) function
        pub fn reserves_of(
            &self,
            currency: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 200, 90, 33], currency)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function
        pub fn safe_batch_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            values: ::std::vec::Vec<::ethers::core::types::U256>,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, values, data))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `safeTransferFrom` (0xf242432a) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            value: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, value, data))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `setHookFees` (0xaf440f82) function
        pub fn set_hook_fees(
            &self,
            key: PoolKey,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([175, 68, 15, 130], (key,))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `setOwner` (0x13af4035) function
        pub fn set_owner(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 175, 64, 53], owner)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `setProtocolFeeController` (0x2d771389)
        /// function
        pub fn set_protocol_fee_controller(
            &self,
            controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 119, 19, 137], controller)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `setProtocolFees` (0x282076cc) function
        pub fn set_protocol_fees(
            &self,
            key: PoolKey,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 32, 118, 204], (key,))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `settle` (0x6a256b29) function
        pub fn settle(
            &self,
            currency: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([106, 37, 107, 41], currency)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `swap` (0x1e2817de) function
        pub fn swap(
            &self,
            key: PoolKey,
            params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([30, 40, 23, 222], (key, params))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `take` (0x0b0d9c09) function
        pub fn take(
            &self,
            currency: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 13, 156, 9], (currency, to, amount))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `uri` (0x0e89341c) function
        pub fn uri(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([14, 137, 52, 28], p0)
                .expect("method not found (this should never happen)")
        }

        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalForAllFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `HookFeeUpdated` event
        pub fn hook_fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, HookFeeUpdatedFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `Initialize` event
        pub fn initialize_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializeFilter> {
            self.0.event()
        }

        ///Gets the contract's `ModifyPosition` event
        pub fn modify_position_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ModifyPositionFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `OwnerChanged` event
        pub fn owner_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnerChangedFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `ProtocolFeeControllerUpdated` event
        pub fn protocol_fee_controller_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ProtocolFeeControllerUpdatedFilter,
        > {
            self.0.event()
        }

        ///Gets the contract's `ProtocolFeeUpdated` event
        pub fn protocol_fee_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProtocolFeeUpdatedFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }

        ///Gets the contract's `TransferBatch` event
        pub fn transfer_batch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferBatchFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `TransferSingle` event
        pub fn transfer_single_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferSingleFilter>
        {
            self.0.event()
        }

        ///Gets the contract's `URI` event
        pub fn uri_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UriFilter> {
            self.0.event()
        }

        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PoolManagerEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for PoolManager<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `CannotUpdateEmptyPosition` with signature
    /// `CannotUpdateEmptyPosition()` and selector `0xaefeb924`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CannotUpdateEmptyPosition", abi = "CannotUpdateEmptyPosition()")]
    pub struct CannotUpdateEmptyPosition;
    ///Custom Error type `CurrencyNotSettled` with signature
    /// `CurrencyNotSettled()` and selector `0x5212cba1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "CurrencyNotSettled", abi = "CurrencyNotSettled()")]
    pub struct CurrencyNotSettled;
    ///Custom Error type `DelegateCallNotAllowed` with signature
    /// `DelegateCallNotAllowed()` and selector `0x0d89438e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DelegateCallNotAllowed", abi = "DelegateCallNotAllowed()")]
    pub struct DelegateCallNotAllowed;
    ///Custom Error type `ERC1155InsufficientBalance` with signature
    /// `ERC1155InsufficientBalance(address,uint256,uint256,uint256)` and
    /// selector `0x03dee4c5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ERC1155InsufficientBalance",
        abi = "ERC1155InsufficientBalance(address,uint256,uint256,uint256)"
    )]
    pub struct ERC1155InsufficientBalance {
        pub sender: ::ethers::core::types::Address,
        pub balance: ::ethers::core::types::U256,
        pub needed: ::ethers::core::types::U256,
        pub token_id: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155InvalidApprover` with signature
    /// `ERC1155InvalidApprover(address)` and selector `0x3e31884e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC1155InvalidApprover", abi = "ERC1155InvalidApprover(address)")]
    pub struct ERC1155InvalidApprover {
        pub approver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidArrayLength` with signature
    /// `ERC1155InvalidArrayLength(uint256,uint256)` and selector `0x5b059991`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ERC1155InvalidArrayLength",
        abi = "ERC1155InvalidArrayLength(uint256,uint256)"
    )]
    pub struct ERC1155InvalidArrayLength {
        pub ids_length: ::ethers::core::types::U256,
        pub values_length: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155InvalidOperator` with signature
    /// `ERC1155InvalidOperator(address)` and selector `0xced3e100`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC1155InvalidOperator", abi = "ERC1155InvalidOperator(address)")]
    pub struct ERC1155InvalidOperator {
        pub operator: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidReceiver` with signature
    /// `ERC1155InvalidReceiver(address)` and selector `0x57f447ce`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC1155InvalidReceiver", abi = "ERC1155InvalidReceiver(address)")]
    pub struct ERC1155InvalidReceiver {
        pub receiver: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155InvalidSender` with signature
    /// `ERC1155InvalidSender(address)` and selector `0x01a83514`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC1155InvalidSender", abi = "ERC1155InvalidSender(address)")]
    pub struct ERC1155InvalidSender {
        pub sender: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC1155MissingApprovalForAll` with signature
    /// `ERC1155MissingApprovalForAll(address,address)` and selector
    /// `0xe237d922`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "ERC1155MissingApprovalForAll",
        abi = "ERC1155MissingApprovalForAll(address,address)"
    )]
    pub struct ERC1155MissingApprovalForAll {
        pub operator: ::ethers::core::types::Address,
        pub owner: ::ethers::core::types::Address,
    }
    ///Custom Error type `ERC20TransferFailed` with signature
    /// `ERC20TransferFailed()` and selector `0xf27f64e4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC20TransferFailed", abi = "ERC20TransferFailed()")]
    pub struct ERC20TransferFailed;
    ///Custom Error type `FeeTooLarge` with signature `FeeTooLarge()` and
    /// selector `0xfc5bee12`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "FeeTooLarge", abi = "FeeTooLarge()")]
    pub struct FeeTooLarge;
    ///Custom Error type `HookAddressNotValid` with signature
    /// `HookAddressNotValid(address)` and selector `0xe65af6a0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "HookAddressNotValid", abi = "HookAddressNotValid(address)")]
    pub struct HookAddressNotValid {
        pub hooks: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidCaller` with signature `InvalidCaller()` and
    /// selector `0x48f5c3ed`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidCaller", abi = "InvalidCaller()")]
    pub struct InvalidCaller;
    ///Custom Error type `InvalidHookResponse` with signature
    /// `InvalidHookResponse()` and selector `0x1e048e1d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidHookResponse", abi = "InvalidHookResponse()")]
    pub struct InvalidHookResponse;
    ///Custom Error type `InvalidSqrtRatio` with signature `InvalidSqrtRatio()`
    /// and selector `0x02ad01b6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidSqrtRatio", abi = "InvalidSqrtRatio()")]
    pub struct InvalidSqrtRatio;
    ///Custom Error type `InvalidTick` with signature `InvalidTick()` and
    /// selector `0xce8ef7fc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidTick", abi = "InvalidTick()")]
    pub struct InvalidTick;
    ///Custom Error type `LockedBy` with signature `LockedBy(address)` and
    /// selector `0x4f4b04cc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LockedBy", abi = "LockedBy(address)")]
    pub struct LockedBy {
        pub locker: ::ethers::core::types::Address,
    }
    ///Custom Error type `MaxCurrenciesTouched` with signature
    /// `MaxCurrenciesTouched()` and selector `0x543f71a6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MaxCurrenciesTouched", abi = "MaxCurrenciesTouched()")]
    pub struct MaxCurrenciesTouched;
    ///Custom Error type `NativeTransferFailed` with signature
    /// `NativeTransferFailed()` and selector `0xf4b3b1bc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NativeTransferFailed", abi = "NativeTransferFailed()")]
    pub struct NativeTransferFailed;
    ///Custom Error type `NoLiquidityToReceiveFees` with signature
    /// `NoLiquidityToReceiveFees()` and selector `0xa74f97ab`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoLiquidityToReceiveFees", abi = "NoLiquidityToReceiveFees()")]
    pub struct NoLiquidityToReceiveFees;
    ///Custom Error type `NotPoolManagerToken` with signature
    /// `NotPoolManagerToken()` and selector `0x53fa5ae4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotPoolManagerToken", abi = "NotPoolManagerToken()")]
    pub struct NotPoolManagerToken;
    ///Custom Error type `PoolAlreadyInitialized` with signature
    /// `PoolAlreadyInitialized()` and selector `0x7983c051`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolAlreadyInitialized", abi = "PoolAlreadyInitialized()")]
    pub struct PoolAlreadyInitialized;
    ///Custom Error type `PoolNotInitialized` with signature
    /// `PoolNotInitialized()` and selector `0x486aa307`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolNotInitialized", abi = "PoolNotInitialized()")]
    pub struct PoolNotInitialized;
    ///Custom Error type `PriceLimitAlreadyExceeded` with signature
    /// `PriceLimitAlreadyExceeded(uint160,uint160)` and selector `0x7c9c6e8f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PriceLimitAlreadyExceeded",
        abi = "PriceLimitAlreadyExceeded(uint160,uint160)"
    )]
    pub struct PriceLimitAlreadyExceeded {
        pub sqrt_price_current_x96: ::ethers::core::types::U256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///Custom Error type `PriceLimitOutOfBounds` with signature
    /// `PriceLimitOutOfBounds(uint160)` and selector `0x9e4d7cc7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PriceLimitOutOfBounds", abi = "PriceLimitOutOfBounds(uint160)")]
    pub struct PriceLimitOutOfBounds {
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///Custom Error type `ProtocolFeeCannotBeFetched` with signature
    /// `ProtocolFeeCannotBeFetched()` and selector `0x1ee49702`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ProtocolFeeCannotBeFetched", abi = "ProtocolFeeCannotBeFetched()")]
    pub struct ProtocolFeeCannotBeFetched;
    ///Custom Error type `SwapAmountCannotBeZero` with signature
    /// `SwapAmountCannotBeZero()` and selector `0xbe8b8507`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SwapAmountCannotBeZero", abi = "SwapAmountCannotBeZero()")]
    pub struct SwapAmountCannotBeZero;
    ///Custom Error type `TickLiquidityOverflow` with signature
    /// `TickLiquidityOverflow(int24)` and selector `0xb8e3c385`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TickLiquidityOverflow", abi = "TickLiquidityOverflow(int24)")]
    pub struct TickLiquidityOverflow {
        pub tick: i32,
    }
    ///Custom Error type `TickLowerOutOfBounds` with signature
    /// `TickLowerOutOfBounds(int24)` and selector `0xd5e2f7ab`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TickLowerOutOfBounds", abi = "TickLowerOutOfBounds(int24)")]
    pub struct TickLowerOutOfBounds {
        pub tick_lower: i32,
    }
    ///Custom Error type `TickMisaligned` with signature
    /// `TickMisaligned(int24,int24)` and selector `0xd4d8f3e6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TickMisaligned", abi = "TickMisaligned(int24,int24)")]
    pub struct TickMisaligned {
        pub tick: i32,
        pub tick_spacing: i32,
    }
    ///Custom Error type `TickSpacingTooLarge` with signature
    /// `TickSpacingTooLarge()` and selector `0xb02b5dc2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TickSpacingTooLarge", abi = "TickSpacingTooLarge()")]
    pub struct TickSpacingTooLarge;
    ///Custom Error type `TickSpacingTooSmall` with signature
    /// `TickSpacingTooSmall()` and selector `0x16fe7696`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TickSpacingTooSmall", abi = "TickSpacingTooSmall()")]
    pub struct TickSpacingTooSmall;
    ///Custom Error type `TickUpperOutOfBounds` with signature
    /// `TickUpperOutOfBounds(int24)` and selector `0x1ad777f8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TickUpperOutOfBounds", abi = "TickUpperOutOfBounds(int24)")]
    pub struct TickUpperOutOfBounds {
        pub tick_upper: i32,
    }
    ///Custom Error type `TicksMisordered` with signature
    /// `TicksMisordered(int24,int24)` and selector `0xc4433ed5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TicksMisordered", abi = "TicksMisordered(int24,int24)")]
    pub struct TicksMisordered {
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolManagerErrors {
        CannotUpdateEmptyPosition(CannotUpdateEmptyPosition),
        CurrencyNotSettled(CurrencyNotSettled),
        DelegateCallNotAllowed(DelegateCallNotAllowed),
        ERC1155InsufficientBalance(ERC1155InsufficientBalance),
        ERC1155InvalidApprover(ERC1155InvalidApprover),
        ERC1155InvalidArrayLength(ERC1155InvalidArrayLength),
        ERC1155InvalidOperator(ERC1155InvalidOperator),
        ERC1155InvalidReceiver(ERC1155InvalidReceiver),
        ERC1155InvalidSender(ERC1155InvalidSender),
        ERC1155MissingApprovalForAll(ERC1155MissingApprovalForAll),
        ERC20TransferFailed(ERC20TransferFailed),
        FeeTooLarge(FeeTooLarge),
        HookAddressNotValid(HookAddressNotValid),
        InvalidCaller(InvalidCaller),
        InvalidHookResponse(InvalidHookResponse),
        InvalidSqrtRatio(InvalidSqrtRatio),
        InvalidTick(InvalidTick),
        LockedBy(LockedBy),
        MaxCurrenciesTouched(MaxCurrenciesTouched),
        NativeTransferFailed(NativeTransferFailed),
        NoLiquidityToReceiveFees(NoLiquidityToReceiveFees),
        NotPoolManagerToken(NotPoolManagerToken),
        PoolAlreadyInitialized(PoolAlreadyInitialized),
        PoolNotInitialized(PoolNotInitialized),
        PriceLimitAlreadyExceeded(PriceLimitAlreadyExceeded),
        PriceLimitOutOfBounds(PriceLimitOutOfBounds),
        ProtocolFeeCannotBeFetched(ProtocolFeeCannotBeFetched),
        SwapAmountCannotBeZero(SwapAmountCannotBeZero),
        TickLiquidityOverflow(TickLiquidityOverflow),
        TickLowerOutOfBounds(TickLowerOutOfBounds),
        TickMisaligned(TickMisaligned),
        TickSpacingTooLarge(TickSpacingTooLarge),
        TickSpacingTooSmall(TickSpacingTooSmall),
        TickUpperOutOfBounds(TickUpperOutOfBounds),
        TicksMisordered(TicksMisordered),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PoolManagerErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <CannotUpdateEmptyPosition as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CannotUpdateEmptyPosition(decoded));
            }
            if let Ok(decoded) =
                <CurrencyNotSettled as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CurrencyNotSettled(decoded));
            }
            if let Ok(decoded) =
                <DelegateCallNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DelegateCallNotAllowed(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InsufficientBalance as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InsufficientBalance(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidApprover as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidApprover(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidArrayLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidArrayLength(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidOperator as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidOperator(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidReceiver as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidReceiver(decoded));
            }
            if let Ok(decoded) =
                <ERC1155InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155InvalidSender(decoded));
            }
            if let Ok(decoded) =
                <ERC1155MissingApprovalForAll as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1155MissingApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <ERC20TransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20TransferFailed(decoded));
            }
            if let Ok(decoded) = <FeeTooLarge as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FeeTooLarge(decoded));
            }
            if let Ok(decoded) =
                <HookAddressNotValid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HookAddressNotValid(decoded));
            }
            if let Ok(decoded) = <InvalidCaller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidCaller(decoded));
            }
            if let Ok(decoded) =
                <InvalidHookResponse as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidHookResponse(decoded));
            }
            if let Ok(decoded) = <InvalidSqrtRatio as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidSqrtRatio(decoded));
            }
            if let Ok(decoded) = <InvalidTick as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTick(decoded));
            }
            if let Ok(decoded) = <LockedBy as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockedBy(decoded));
            }
            if let Ok(decoded) =
                <MaxCurrenciesTouched as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxCurrenciesTouched(decoded));
            }
            if let Ok(decoded) =
                <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            if let Ok(decoded) =
                <NoLiquidityToReceiveFees as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NoLiquidityToReceiveFees(decoded));
            }
            if let Ok(decoded) =
                <NotPoolManagerToken as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NotPoolManagerToken(decoded));
            }
            if let Ok(decoded) =
                <PoolAlreadyInitialized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolAlreadyInitialized(decoded));
            }
            if let Ok(decoded) =
                <PoolNotInitialized as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolNotInitialized(decoded));
            }
            if let Ok(decoded) =
                <PriceLimitAlreadyExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PriceLimitAlreadyExceeded(decoded));
            }
            if let Ok(decoded) =
                <PriceLimitOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PriceLimitOutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeeCannotBeFetched as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProtocolFeeCannotBeFetched(decoded));
            }
            if let Ok(decoded) =
                <SwapAmountCannotBeZero as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapAmountCannotBeZero(decoded));
            }
            if let Ok(decoded) =
                <TickLiquidityOverflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TickLiquidityOverflow(decoded));
            }
            if let Ok(decoded) =
                <TickLowerOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TickLowerOutOfBounds(decoded));
            }
            if let Ok(decoded) = <TickMisaligned as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TickMisaligned(decoded));
            }
            if let Ok(decoded) =
                <TickSpacingTooLarge as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TickSpacingTooLarge(decoded));
            }
            if let Ok(decoded) =
                <TickSpacingTooSmall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TickSpacingTooSmall(decoded));
            }
            if let Ok(decoded) =
                <TickUpperOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TickUpperOutOfBounds(decoded));
            }
            if let Ok(decoded) = <TicksMisordered as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TicksMisordered(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolManagerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::CannotUpdateEmptyPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrencyNotSettled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DelegateCallNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InsufficientBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidApprover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidArrayLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidOperator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidReceiver(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155MissingApprovalForAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC20TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FeeTooLarge(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HookAddressNotValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCaller(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidHookResponse(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSqrtRatio(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidTick(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockedBy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxCurrenciesTouched(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoLiquidityToReceiveFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotPoolManagerToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolAlreadyInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolNotInitialized(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceLimitAlreadyExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PriceLimitOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFeeCannotBeFetched(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapAmountCannotBeZero(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickLiquidityOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickLowerOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickMisaligned(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TickSpacingTooLarge(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickSpacingTooSmall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TickUpperOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TicksMisordered(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PoolManagerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <CannotUpdateEmptyPosition as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <CurrencyNotSettled as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <DelegateCallNotAllowed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InsufficientBalance as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidApprover as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidArrayLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidOperator as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidReceiver as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155InvalidSender as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ERC1155MissingApprovalForAll as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector
                    == <ERC20TransferFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <FeeTooLarge as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <HookAddressNotValid as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidCaller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidHookResponse as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidSqrtRatio as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidTick as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <LockedBy as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <MaxCurrenciesTouched as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NoLiquidityToReceiveFees as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NotPoolManagerToken as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <PoolAlreadyInitialized as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <PoolNotInitialized as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <PriceLimitAlreadyExceeded as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <PriceLimitOutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ProtocolFeeCannotBeFetched as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <SwapAmountCannotBeZero as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TickLiquidityOverflow as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TickLowerOutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <TickMisaligned as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TickSpacingTooLarge as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TickSpacingTooSmall as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TickUpperOutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <TicksMisordered as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PoolManagerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CannotUpdateEmptyPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrencyNotSettled(element) => ::core::fmt::Display::fmt(element, f),
                Self::DelegateCallNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InsufficientBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidApprover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidArrayLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidReceiver(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::ERC1155MissingApprovalForAll(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC20TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::HookAddressNotValid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCaller(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidHookResponse(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSqrtRatio(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTick(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockedBy(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxCurrenciesTouched(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoLiquidityToReceiveFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotPoolManagerToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolAlreadyInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolNotInitialized(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceLimitAlreadyExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceLimitOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeCannotBeFetched(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapAmountCannotBeZero(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickLiquidityOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickLowerOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickMisaligned(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickSpacingTooLarge(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickSpacingTooSmall(element) => ::core::fmt::Display::fmt(element, f),
                Self::TickUpperOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::TicksMisordered(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PoolManagerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<CannotUpdateEmptyPosition> for PoolManagerErrors {
        fn from(value: CannotUpdateEmptyPosition) -> Self {
            Self::CannotUpdateEmptyPosition(value)
        }
    }
    impl ::core::convert::From<CurrencyNotSettled> for PoolManagerErrors {
        fn from(value: CurrencyNotSettled) -> Self {
            Self::CurrencyNotSettled(value)
        }
    }
    impl ::core::convert::From<DelegateCallNotAllowed> for PoolManagerErrors {
        fn from(value: DelegateCallNotAllowed) -> Self {
            Self::DelegateCallNotAllowed(value)
        }
    }
    impl ::core::convert::From<ERC1155InsufficientBalance> for PoolManagerErrors {
        fn from(value: ERC1155InsufficientBalance) -> Self {
            Self::ERC1155InsufficientBalance(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidApprover> for PoolManagerErrors {
        fn from(value: ERC1155InvalidApprover) -> Self {
            Self::ERC1155InvalidApprover(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidArrayLength> for PoolManagerErrors {
        fn from(value: ERC1155InvalidArrayLength) -> Self {
            Self::ERC1155InvalidArrayLength(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidOperator> for PoolManagerErrors {
        fn from(value: ERC1155InvalidOperator) -> Self {
            Self::ERC1155InvalidOperator(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidReceiver> for PoolManagerErrors {
        fn from(value: ERC1155InvalidReceiver) -> Self {
            Self::ERC1155InvalidReceiver(value)
        }
    }
    impl ::core::convert::From<ERC1155InvalidSender> for PoolManagerErrors {
        fn from(value: ERC1155InvalidSender) -> Self {
            Self::ERC1155InvalidSender(value)
        }
    }
    impl ::core::convert::From<ERC1155MissingApprovalForAll> for PoolManagerErrors {
        fn from(value: ERC1155MissingApprovalForAll) -> Self {
            Self::ERC1155MissingApprovalForAll(value)
        }
    }
    impl ::core::convert::From<ERC20TransferFailed> for PoolManagerErrors {
        fn from(value: ERC20TransferFailed) -> Self {
            Self::ERC20TransferFailed(value)
        }
    }
    impl ::core::convert::From<FeeTooLarge> for PoolManagerErrors {
        fn from(value: FeeTooLarge) -> Self {
            Self::FeeTooLarge(value)
        }
    }
    impl ::core::convert::From<HookAddressNotValid> for PoolManagerErrors {
        fn from(value: HookAddressNotValid) -> Self {
            Self::HookAddressNotValid(value)
        }
    }
    impl ::core::convert::From<InvalidCaller> for PoolManagerErrors {
        fn from(value: InvalidCaller) -> Self {
            Self::InvalidCaller(value)
        }
    }
    impl ::core::convert::From<InvalidHookResponse> for PoolManagerErrors {
        fn from(value: InvalidHookResponse) -> Self {
            Self::InvalidHookResponse(value)
        }
    }
    impl ::core::convert::From<InvalidSqrtRatio> for PoolManagerErrors {
        fn from(value: InvalidSqrtRatio) -> Self {
            Self::InvalidSqrtRatio(value)
        }
    }
    impl ::core::convert::From<InvalidTick> for PoolManagerErrors {
        fn from(value: InvalidTick) -> Self {
            Self::InvalidTick(value)
        }
    }
    impl ::core::convert::From<LockedBy> for PoolManagerErrors {
        fn from(value: LockedBy) -> Self {
            Self::LockedBy(value)
        }
    }
    impl ::core::convert::From<MaxCurrenciesTouched> for PoolManagerErrors {
        fn from(value: MaxCurrenciesTouched) -> Self {
            Self::MaxCurrenciesTouched(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for PoolManagerErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
        }
    }
    impl ::core::convert::From<NoLiquidityToReceiveFees> for PoolManagerErrors {
        fn from(value: NoLiquidityToReceiveFees) -> Self {
            Self::NoLiquidityToReceiveFees(value)
        }
    }
    impl ::core::convert::From<NotPoolManagerToken> for PoolManagerErrors {
        fn from(value: NotPoolManagerToken) -> Self {
            Self::NotPoolManagerToken(value)
        }
    }
    impl ::core::convert::From<PoolAlreadyInitialized> for PoolManagerErrors {
        fn from(value: PoolAlreadyInitialized) -> Self {
            Self::PoolAlreadyInitialized(value)
        }
    }
    impl ::core::convert::From<PoolNotInitialized> for PoolManagerErrors {
        fn from(value: PoolNotInitialized) -> Self {
            Self::PoolNotInitialized(value)
        }
    }
    impl ::core::convert::From<PriceLimitAlreadyExceeded> for PoolManagerErrors {
        fn from(value: PriceLimitAlreadyExceeded) -> Self {
            Self::PriceLimitAlreadyExceeded(value)
        }
    }
    impl ::core::convert::From<PriceLimitOutOfBounds> for PoolManagerErrors {
        fn from(value: PriceLimitOutOfBounds) -> Self {
            Self::PriceLimitOutOfBounds(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeCannotBeFetched> for PoolManagerErrors {
        fn from(value: ProtocolFeeCannotBeFetched) -> Self {
            Self::ProtocolFeeCannotBeFetched(value)
        }
    }
    impl ::core::convert::From<SwapAmountCannotBeZero> for PoolManagerErrors {
        fn from(value: SwapAmountCannotBeZero) -> Self {
            Self::SwapAmountCannotBeZero(value)
        }
    }
    impl ::core::convert::From<TickLiquidityOverflow> for PoolManagerErrors {
        fn from(value: TickLiquidityOverflow) -> Self {
            Self::TickLiquidityOverflow(value)
        }
    }
    impl ::core::convert::From<TickLowerOutOfBounds> for PoolManagerErrors {
        fn from(value: TickLowerOutOfBounds) -> Self {
            Self::TickLowerOutOfBounds(value)
        }
    }
    impl ::core::convert::From<TickMisaligned> for PoolManagerErrors {
        fn from(value: TickMisaligned) -> Self {
            Self::TickMisaligned(value)
        }
    }
    impl ::core::convert::From<TickSpacingTooLarge> for PoolManagerErrors {
        fn from(value: TickSpacingTooLarge) -> Self {
            Self::TickSpacingTooLarge(value)
        }
    }
    impl ::core::convert::From<TickSpacingTooSmall> for PoolManagerErrors {
        fn from(value: TickSpacingTooSmall) -> Self {
            Self::TickSpacingTooSmall(value)
        }
    }
    impl ::core::convert::From<TickUpperOutOfBounds> for PoolManagerErrors {
        fn from(value: TickUpperOutOfBounds) -> Self {
            Self::TickUpperOutOfBounds(value)
        }
    }
    impl ::core::convert::From<TicksMisordered> for PoolManagerErrors {
        fn from(value: TicksMisordered) -> Self {
            Self::TicksMisordered(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "HookFeeUpdated", abi = "HookFeeUpdated(bytes32,uint8,uint8)")]
    pub struct HookFeeUpdatedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub hook_swap_fee: u8,
        pub hook_withdraw_fee: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Initialize",
        abi = "Initialize(bytes32,address,address,uint24,int24,address)"
    )]
    pub struct InitializeFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub currency_0: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub currency_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_spacing: i32,
        pub hooks: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ModifyPosition", abi = "ModifyPosition(bytes32,address,int24,int24,int256)")]
    pub struct ModifyPositionFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub liquidity_delta: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OwnerChanged", abi = "OwnerChanged(address,address)")]
    pub struct OwnerChangedFilter {
        #[ethevent(indexed)]
        pub old_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ProtocolFeeControllerUpdated",
        abi = "ProtocolFeeControllerUpdated(address)"
    )]
    pub struct ProtocolFeeControllerUpdatedFilter {
        pub protocol_fee_controller: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ProtocolFeeUpdated", abi = "ProtocolFeeUpdated(bytes32,uint8,uint8)")]
    pub struct ProtocolFeeUpdatedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        pub protocol_swap_fee: u8,
        pub protocol_withdraw_fee: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub amount_0: i128,
        pub amount_1: i128,
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub liquidity: u128,
        pub tick: i32,
        pub fee: u32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: ::std::string::String,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolManagerEvents {
        ApprovalForAllFilter(ApprovalForAllFilter),
        HookFeeUpdatedFilter(HookFeeUpdatedFilter),
        InitializeFilter(InitializeFilter),
        ModifyPositionFilter(ModifyPositionFilter),
        OwnerChangedFilter(OwnerChangedFilter),
        ProtocolFeeControllerUpdatedFilter(ProtocolFeeControllerUpdatedFilter),
        ProtocolFeeUpdatedFilter(ProtocolFeeUpdatedFilter),
        SwapFilter(SwapFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
    }
    impl ::ethers::contract::EthLogDecode for PoolManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(PoolManagerEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = HookFeeUpdatedFilter::decode_log(log) {
                return Ok(PoolManagerEvents::HookFeeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = InitializeFilter::decode_log(log) {
                return Ok(PoolManagerEvents::InitializeFilter(decoded));
            }
            if let Ok(decoded) = ModifyPositionFilter::decode_log(log) {
                return Ok(PoolManagerEvents::ModifyPositionFilter(decoded));
            }
            if let Ok(decoded) = OwnerChangedFilter::decode_log(log) {
                return Ok(PoolManagerEvents::OwnerChangedFilter(decoded));
            }
            if let Ok(decoded) = ProtocolFeeControllerUpdatedFilter::decode_log(log) {
                return Ok(PoolManagerEvents::ProtocolFeeControllerUpdatedFilter(decoded));
            }
            if let Ok(decoded) = ProtocolFeeUpdatedFilter::decode_log(log) {
                return Ok(PoolManagerEvents::ProtocolFeeUpdatedFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(PoolManagerEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(PoolManagerEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(PoolManagerEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(PoolManagerEvents::UriFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PoolManagerEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::HookFeeUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyPositionFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeControllerUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProtocolFeeUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferBatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferSingleFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UriFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for PoolManagerEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<HookFeeUpdatedFilter> for PoolManagerEvents {
        fn from(value: HookFeeUpdatedFilter) -> Self {
            Self::HookFeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<InitializeFilter> for PoolManagerEvents {
        fn from(value: InitializeFilter) -> Self {
            Self::InitializeFilter(value)
        }
    }
    impl ::core::convert::From<ModifyPositionFilter> for PoolManagerEvents {
        fn from(value: ModifyPositionFilter) -> Self {
            Self::ModifyPositionFilter(value)
        }
    }
    impl ::core::convert::From<OwnerChangedFilter> for PoolManagerEvents {
        fn from(value: OwnerChangedFilter) -> Self {
            Self::OwnerChangedFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerUpdatedFilter> for PoolManagerEvents {
        fn from(value: ProtocolFeeControllerUpdatedFilter) -> Self {
            Self::ProtocolFeeControllerUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeUpdatedFilter> for PoolManagerEvents {
        fn from(value: ProtocolFeeUpdatedFilter) -> Self {
            Self::ProtocolFeeUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for PoolManagerEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<TransferBatchFilter> for PoolManagerEvents {
        fn from(value: TransferBatchFilter) -> Self {
            Self::TransferBatchFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for PoolManagerEvents {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    impl ::core::convert::From<UriFilter> for PoolManagerEvents {
        fn from(value: UriFilter) -> Self {
            Self::UriFilter(value)
        }
    }
    ///Container type for all input parameters for the `MAX_TICK_SPACING`
    /// function with signature `MAX_TICK_SPACING()` and selector `0x60460f06`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MAX_TICK_SPACING", abi = "MAX_TICK_SPACING()")]
    pub struct MaxTickSpacingCall;
    ///Container type for all input parameters for the
    /// `MIN_PROTOCOL_FEE_DENOMINATOR` function with signature
    /// `MIN_PROTOCOL_FEE_DENOMINATOR()` and selector `0x84e41c5e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MIN_PROTOCOL_FEE_DENOMINATOR", abi = "MIN_PROTOCOL_FEE_DENOMINATOR()")]
    pub struct MinProtocolFeeDenominatorCall;
    ///Container type for all input parameters for the `MIN_TICK_SPACING`
    /// function with signature `MIN_TICK_SPACING()` and selector `0x07eff0dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MIN_TICK_SPACING", abi = "MIN_TICK_SPACING()")]
    pub struct MinTickSpacingCall;
    ///Container type for all input parameters for the `balanceOf` function
    /// with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall {
        pub account: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `balanceOfBatch`
    /// function with signature `balanceOfBatch(address[],uint256[])` and
    /// selector `0x4e1273f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub accounts: ::std::vec::Vec<::ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `collectHookFees`
    /// function with signature `collectHookFees(address,address,uint256)` and
    /// selector `0xeffd18c0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "collectHookFees", abi = "collectHookFees(address,address,uint256)")]
    pub struct CollectHookFeesCall {
        pub recipient: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `collectProtocolFees`
    /// function with signature `collectProtocolFees(address,address,uint256)`
    /// and selector `0x8161b874`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "collectProtocolFees", abi = "collectProtocolFees(address,address,uint256)")]
    pub struct CollectProtocolFeesCall {
        pub recipient: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `currencyDelta` function
    /// with signature `currencyDelta(address,address)` and selector
    /// `0xa54b2831`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "currencyDelta", abi = "currencyDelta(address,address)")]
    pub struct CurrencyDeltaCall {
        pub locker: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `donate` function with
    /// signature `donate((address,address,uint24,int24,address),uint256,
    /// uint256)` and selector `0xa67dd8f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "donate",
        abi = "donate((address,address,uint24,int24,address),uint256,uint256)"
    )]
    pub struct DonateCall {
        pub key: PoolKey,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `extsload` function with
    /// signature `extsload(bytes32)` and selector `0x1e2eaeaf`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "extsload", abi = "extsload(bytes32)")]
    pub struct ExtsloadCall {
        pub slot: [u8; 32],
    }
    ///Container type for all input parameters for the `extsload` function with
    /// signature `extsload(bytes32,uint256)` and selector `0x35fd631a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "extsload", abi = "extsload(bytes32,uint256)")]
    pub struct ExtsloadWithStartSlotCall {
        pub start_slot: [u8; 32],
        pub n_slots: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getLiquidity` function
    /// with signature `getLiquidity(bytes32,address,int24,int24)` and selector
    /// `0x33aa955b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLiquidity", abi = "getLiquidity(bytes32,address,int24,int24)")]
    pub struct GetLiquidityWithOwnerCall {
        pub id: [u8; 32],
        pub owner: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all input parameters for the `getLiquidity` function
    /// with signature `getLiquidity(bytes32)` and selector `0xfa6793d5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLiquidity", abi = "getLiquidity(bytes32)")]
    pub struct GetLiquidityCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `getLock` function with
    /// signature `getLock(uint256)` and selector `0xd68f4dd1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLock", abi = "getLock(uint256)")]
    pub struct GetLockCall {
        pub i: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getPosition` function
    /// with signature `getPosition(bytes32,address,int24,int24)` and selector
    /// `0x048d9c70`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPosition", abi = "getPosition(bytes32,address,int24,int24)")]
    pub struct GetPositionCall {
        pub id: [u8; 32],
        pub owner: ::ethers::core::types::Address,
        pub tick_lower: i32,
        pub tick_upper: i32,
    }
    ///Container type for all input parameters for the `getSlot0` function with
    /// signature `getSlot0(bytes32)` and selector `0xc815641c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSlot0", abi = "getSlot0(bytes32)")]
    pub struct GetSlot0Call {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `hookFeesAccrued`
    /// function with signature `hookFeesAccrued(address,address)` and selector
    /// `0xb4c41939`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "hookFeesAccrued", abi = "hookFeesAccrued(address,address)")]
    pub struct HookFeesAccruedCall {
        pub hook_address: ::ethers::core::types::Address,
        pub currency: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `initialize` function
    /// with signature
    /// `initialize((address,address,uint24,int24,address),uint160)` and
    /// selector `0x6276cbbe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize((address,address,uint24,int24,address),uint160)"
    )]
    pub struct InitializeCall {
        pub key: PoolKey,
        pub sqrt_price_x96: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isApprovedForAll`
    /// function with signature `isApprovedForAll(address,address)` and selector
    /// `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall {
        pub account: ::ethers::core::types::Address,
        pub operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lock` function with
    /// signature `lock(bytes)` and selector `0x81548319`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lock", abi = "lock(bytes)")]
    pub struct LockCall {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `lockData` function with
    /// signature `lockData()` and selector `0xf8fcd156`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lockData", abi = "lockData()")]
    pub struct LockDataCall;
    ///Container type for all input parameters for the `mint` function with
    /// signature `mint(address,address,uint256)` and selector `0xc6c3bbe6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "mint", abi = "mint(address,address,uint256)")]
    pub struct MintCall {
        pub currency: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `modifyPosition`
    /// function with signature
    /// `modifyPosition((address,address,uint24,int24,address),(int24,int24,
    /// int256))` and selector `0x555a4733`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "modifyPosition",
        abi = "modifyPosition((address,address,uint24,int24,address),(int24,int24,int256))"
    )]
    pub struct ModifyPositionCall {
        pub key: PoolKey,
        pub params: ModifyPositionParams,
    }
    ///Container type for all input parameters for the `onERC1155BatchReceived`
    /// function with signature
    /// `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and
    /// selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall {
        pub p0: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `onERC1155Received`
    /// function with signature
    /// `onERC1155Received(address,address,uint256,uint256,bytes)` and selector
    /// `0xf23a6e61`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall {
        pub p0: ::ethers::core::types::Address,
        pub p1: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub p4: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `owner` function with
    /// signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pools` function with
    /// signature `pools(bytes32)` and selector `0xb5217bb4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pools", abi = "pools(bytes32)")]
    pub struct PoolsCall {
        pub id: [u8; 32],
    }
    ///Container type for all input parameters for the `protocolFeeController`
    /// function with signature `protocolFeeController()` and selector
    /// `0xf02de3b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "protocolFeeController", abi = "protocolFeeController()")]
    pub struct ProtocolFeeControllerCall;
    ///Container type for all input parameters for the `protocolFeesAccrued`
    /// function with signature `protocolFeesAccrued(address)` and selector
    /// `0x97e8cd4e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "protocolFeesAccrued", abi = "protocolFeesAccrued(address)")]
    pub struct ProtocolFeesAccruedCall {
        pub currency: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `reservesOf` function
    /// with signature `reservesOf(address)` and selector `0x93c85a21`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "reservesOf", abi = "reservesOf(address)")]
    pub struct ReservesOfCall {
        pub currency: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `safeBatchTransferFrom`
    /// function with signature
    /// `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and
    /// selector `0x2eb2c2d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub values: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `safeTransferFrom`
    /// function with signature
    /// `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector
    /// `0xf242432a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub value: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll`
    /// function with signature `setApprovalForAll(address,bool)` and selector
    /// `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setHookFees` function
    /// with signature `setHookFees((address,address,uint24,int24,address))` and
    /// selector `0xaf440f82`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setHookFees", abi = "setHookFees((address,address,uint24,int24,address))")]
    pub struct SetHookFeesCall {
        pub key: PoolKey,
    }
    ///Container type for all input parameters for the `setOwner` function with
    /// signature `setOwner(address)` and selector `0x13af4035`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setOwner", abi = "setOwner(address)")]
    pub struct SetOwnerCall {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `setProtocolFeeController` function with signature
    /// `setProtocolFeeController(address)` and selector `0x2d771389`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setProtocolFeeController", abi = "setProtocolFeeController(address)")]
    pub struct SetProtocolFeeControllerCall {
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setProtocolFees`
    /// function with signature
    /// `setProtocolFees((address,address,uint24,int24,address))` and selector
    /// `0x282076cc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "setProtocolFees",
        abi = "setProtocolFees((address,address,uint24,int24,address))"
    )]
    pub struct SetProtocolFeesCall {
        pub key: PoolKey,
    }
    ///Container type for all input parameters for the `settle` function with
    /// signature `settle(address)` and selector `0x6a256b29`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "settle", abi = "settle(address)")]
    pub struct SettleCall {
        pub currency: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface`
    /// function with signature `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `swap` function with
    /// signature `swap((address,address,uint24,int24,address),(bool,int256,
    /// uint160))` and selector `0x1e2817de`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "swap",
        abi = "swap((address,address,uint24,int24,address),(bool,int256,uint160))"
    )]
    pub struct SwapCall {
        pub key: PoolKey,
        pub params: SwapParams,
    }
    ///Container type for all input parameters for the `take` function with
    /// signature `take(address,address,uint256)` and selector `0x0b0d9c09`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "take", abi = "take(address,address,uint256)")]
    pub struct TakeCall {
        pub currency: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uri` function with
    /// signature `uri(uint256)` and selector `0x0e89341c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall(pub ::ethers::core::types::U256);
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PoolManagerCalls {
        MaxTickSpacing(MaxTickSpacingCall),
        MinProtocolFeeDenominator(MinProtocolFeeDenominatorCall),
        MinTickSpacing(MinTickSpacingCall),
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        CollectHookFees(CollectHookFeesCall),
        CollectProtocolFees(CollectProtocolFeesCall),
        CurrencyDelta(CurrencyDeltaCall),
        Donate(DonateCall),
        Extsload(ExtsloadCall),
        ExtsloadWithStartSlot(ExtsloadWithStartSlotCall),
        GetLiquidityWithOwner(GetLiquidityWithOwnerCall),
        GetLiquidity(GetLiquidityCall),
        GetLock(GetLockCall),
        GetPosition(GetPositionCall),
        GetSlot0(GetSlot0Call),
        HookFeesAccrued(HookFeesAccruedCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Lock(LockCall),
        LockData(LockDataCall),
        Mint(MintCall),
        ModifyPosition(ModifyPositionCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        Owner(OwnerCall),
        Pools(PoolsCall),
        ProtocolFeeController(ProtocolFeeControllerCall),
        ProtocolFeesAccrued(ProtocolFeesAccruedCall),
        ReservesOf(ReservesOfCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetHookFees(SetHookFeesCall),
        SetOwner(SetOwnerCall),
        SetProtocolFeeController(SetProtocolFeeControllerCall),
        SetProtocolFees(SetProtocolFeesCall),
        Settle(SettleCall),
        SupportsInterface(SupportsInterfaceCall),
        Swap(SwapCall),
        Take(TakeCall),
        Uri(UriCall),
    }
    impl ::ethers::core::abi::AbiDecode for PoolManagerCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <MaxTickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxTickSpacing(decoded));
            }
            if let Ok(decoded) =
                <MinProtocolFeeDenominatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinProtocolFeeDenominator(decoded));
            }
            if let Ok(decoded) =
                <MinTickSpacingCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinTickSpacing(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) =
                <CollectHookFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CollectHookFees(decoded));
            }
            if let Ok(decoded) =
                <CollectProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CollectProtocolFees(decoded));
            }
            if let Ok(decoded) = <CurrencyDeltaCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CurrencyDelta(decoded));
            }
            if let Ok(decoded) = <DonateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Donate(decoded));
            }
            if let Ok(decoded) = <ExtsloadCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Extsload(decoded));
            }
            if let Ok(decoded) =
                <ExtsloadWithStartSlotCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExtsloadWithStartSlot(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidityWithOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidityWithOwner(decoded));
            }
            if let Ok(decoded) = <GetLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidity(decoded));
            }
            if let Ok(decoded) = <GetLockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetLock(decoded));
            }
            if let Ok(decoded) = <GetPositionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPosition(decoded));
            }
            if let Ok(decoded) = <GetSlot0Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSlot0(decoded));
            }
            if let Ok(decoded) =
                <HookFeesAccruedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HookFeesAccrued(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <LockCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Lock(decoded));
            }
            if let Ok(decoded) = <LockDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockData(decoded));
            }
            if let Ok(decoded) = <MintCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Mint(decoded));
            }
            if let Ok(decoded) =
                <ModifyPositionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ModifyPosition(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded) =
                <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProtocolFeeController(decoded));
            }
            if let Ok(decoded) =
                <ProtocolFeesAccruedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProtocolFeesAccrued(decoded));
            }
            if let Ok(decoded) = <ReservesOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ReservesOf(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) = <SetHookFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetHookFees(decoded));
            }
            if let Ok(decoded) = <SetOwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOwner(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeControllerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProtocolFeeController(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProtocolFees(decoded));
            }
            if let Ok(decoded) = <SettleCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Settle(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <TakeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Take(decoded));
            }
            if let Ok(decoded) = <UriCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Uri(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PoolManagerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MaxTickSpacing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinProtocolFeeDenominator(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MinTickSpacing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOfBatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollectHookFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CollectProtocolFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrencyDelta(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Donate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Extsload(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExtsloadWithStartSlot(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidityWithOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPosition(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSlot0(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HookFeesAccrued(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Lock(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Mint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ModifyPosition(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFeesAccrued(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ReservesOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeBatchTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetHookFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOwner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFeeController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetProtocolFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Settle(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Take(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Uri(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PoolManagerCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MaxTickSpacing(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinProtocolFeeDenominator(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinTickSpacing(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectHookFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::CollectProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrencyDelta(element) => ::core::fmt::Display::fmt(element, f),
                Self::Donate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Extsload(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExtsloadWithStartSlot(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidityWithOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLock(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSlot0(element) => ::core::fmt::Display::fmt(element, f),
                Self::HookFeesAccrued(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Lock(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Mint(element) => ::core::fmt::Display::fmt(element, f),
                Self::ModifyPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeesAccrued(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReservesOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeBatchTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetHookFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFeeController(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::Settle(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Take(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uri(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<MaxTickSpacingCall> for PoolManagerCalls {
        fn from(value: MaxTickSpacingCall) -> Self {
            Self::MaxTickSpacing(value)
        }
    }
    impl ::core::convert::From<MinProtocolFeeDenominatorCall> for PoolManagerCalls {
        fn from(value: MinProtocolFeeDenominatorCall) -> Self {
            Self::MinProtocolFeeDenominator(value)
        }
    }
    impl ::core::convert::From<MinTickSpacingCall> for PoolManagerCalls {
        fn from(value: MinTickSpacingCall) -> Self {
            Self::MinTickSpacing(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for PoolManagerCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfBatchCall> for PoolManagerCalls {
        fn from(value: BalanceOfBatchCall) -> Self {
            Self::BalanceOfBatch(value)
        }
    }
    impl ::core::convert::From<CollectHookFeesCall> for PoolManagerCalls {
        fn from(value: CollectHookFeesCall) -> Self {
            Self::CollectHookFees(value)
        }
    }
    impl ::core::convert::From<CollectProtocolFeesCall> for PoolManagerCalls {
        fn from(value: CollectProtocolFeesCall) -> Self {
            Self::CollectProtocolFees(value)
        }
    }
    impl ::core::convert::From<CurrencyDeltaCall> for PoolManagerCalls {
        fn from(value: CurrencyDeltaCall) -> Self {
            Self::CurrencyDelta(value)
        }
    }
    impl ::core::convert::From<DonateCall> for PoolManagerCalls {
        fn from(value: DonateCall) -> Self {
            Self::Donate(value)
        }
    }
    impl ::core::convert::From<ExtsloadCall> for PoolManagerCalls {
        fn from(value: ExtsloadCall) -> Self {
            Self::Extsload(value)
        }
    }
    impl ::core::convert::From<ExtsloadWithStartSlotCall> for PoolManagerCalls {
        fn from(value: ExtsloadWithStartSlotCall) -> Self {
            Self::ExtsloadWithStartSlot(value)
        }
    }
    impl ::core::convert::From<GetLiquidityWithOwnerCall> for PoolManagerCalls {
        fn from(value: GetLiquidityWithOwnerCall) -> Self {
            Self::GetLiquidityWithOwner(value)
        }
    }
    impl ::core::convert::From<GetLiquidityCall> for PoolManagerCalls {
        fn from(value: GetLiquidityCall) -> Self {
            Self::GetLiquidity(value)
        }
    }
    impl ::core::convert::From<GetLockCall> for PoolManagerCalls {
        fn from(value: GetLockCall) -> Self {
            Self::GetLock(value)
        }
    }
    impl ::core::convert::From<GetPositionCall> for PoolManagerCalls {
        fn from(value: GetPositionCall) -> Self {
            Self::GetPosition(value)
        }
    }
    impl ::core::convert::From<GetSlot0Call> for PoolManagerCalls {
        fn from(value: GetSlot0Call) -> Self {
            Self::GetSlot0(value)
        }
    }
    impl ::core::convert::From<HookFeesAccruedCall> for PoolManagerCalls {
        fn from(value: HookFeesAccruedCall) -> Self {
            Self::HookFeesAccrued(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for PoolManagerCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for PoolManagerCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<LockCall> for PoolManagerCalls {
        fn from(value: LockCall) -> Self {
            Self::Lock(value)
        }
    }
    impl ::core::convert::From<LockDataCall> for PoolManagerCalls {
        fn from(value: LockDataCall) -> Self {
            Self::LockData(value)
        }
    }
    impl ::core::convert::From<MintCall> for PoolManagerCalls {
        fn from(value: MintCall) -> Self {
            Self::Mint(value)
        }
    }
    impl ::core::convert::From<ModifyPositionCall> for PoolManagerCalls {
        fn from(value: ModifyPositionCall) -> Self {
            Self::ModifyPosition(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for PoolManagerCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for PoolManagerCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for PoolManagerCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for PoolManagerCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeControllerCall> for PoolManagerCalls {
        fn from(value: ProtocolFeeControllerCall) -> Self {
            Self::ProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<ProtocolFeesAccruedCall> for PoolManagerCalls {
        fn from(value: ProtocolFeesAccruedCall) -> Self {
            Self::ProtocolFeesAccrued(value)
        }
    }
    impl ::core::convert::From<ReservesOfCall> for PoolManagerCalls {
        fn from(value: ReservesOfCall) -> Self {
            Self::ReservesOf(value)
        }
    }
    impl ::core::convert::From<SafeBatchTransferFromCall> for PoolManagerCalls {
        fn from(value: SafeBatchTransferFromCall) -> Self {
            Self::SafeBatchTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for PoolManagerCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for PoolManagerCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetHookFeesCall> for PoolManagerCalls {
        fn from(value: SetHookFeesCall) -> Self {
            Self::SetHookFees(value)
        }
    }
    impl ::core::convert::From<SetOwnerCall> for PoolManagerCalls {
        fn from(value: SetOwnerCall) -> Self {
            Self::SetOwner(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeControllerCall> for PoolManagerCalls {
        fn from(value: SetProtocolFeeControllerCall) -> Self {
            Self::SetProtocolFeeController(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeesCall> for PoolManagerCalls {
        fn from(value: SetProtocolFeesCall) -> Self {
            Self::SetProtocolFees(value)
        }
    }
    impl ::core::convert::From<SettleCall> for PoolManagerCalls {
        fn from(value: SettleCall) -> Self {
            Self::Settle(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for PoolManagerCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SwapCall> for PoolManagerCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TakeCall> for PoolManagerCalls {
        fn from(value: TakeCall) -> Self {
            Self::Take(value)
        }
    }
    impl ::core::convert::From<UriCall> for PoolManagerCalls {
        fn from(value: UriCall) -> Self {
            Self::Uri(value)
        }
    }
    ///Container type for all return fields from the `MAX_TICK_SPACING`
    /// function with signature `MAX_TICK_SPACING()` and selector `0x60460f06`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaxTickSpacingReturn(pub i32);
    ///Container type for all return fields from the
    /// `MIN_PROTOCOL_FEE_DENOMINATOR` function with signature
    /// `MIN_PROTOCOL_FEE_DENOMINATOR()` and selector `0x84e41c5e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinProtocolFeeDenominatorReturn(pub u8);
    ///Container type for all return fields from the `MIN_TICK_SPACING`
    /// function with signature `MIN_TICK_SPACING()` and selector `0x07eff0dd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinTickSpacingReturn(pub i32);
    ///Container type for all return fields from the `balanceOf` function with
    /// signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOfBatch` function
    /// with signature `balanceOfBatch(address[],uint256[])` and selector
    /// `0x4e1273f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfBatchReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `collectHookFees` function
    /// with signature `collectHookFees(address,address,uint256)` and selector
    /// `0xeffd18c0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CollectHookFeesReturn {
        pub amount_collected: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `collectProtocolFees`
    /// function with signature `collectProtocolFees(address,address,uint256)`
    /// and selector `0x8161b874`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CollectProtocolFeesReturn {
        pub amount_collected: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `currencyDelta` function
    /// with signature `currencyDelta(address,address)` and selector
    /// `0xa54b2831`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CurrencyDeltaReturn {
        pub currency_delta: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `donate` function with
    /// signature `donate((address,address,uint24,int24,address),uint256,
    /// uint256)` and selector `0xa67dd8f3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DonateReturn {
        pub delta: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `extsload` function with
    /// signature `extsload(bytes32)` and selector `0x1e2eaeaf`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExtsloadReturn {
        pub value: [u8; 32],
    }
    ///Container type for all return fields from the `extsload` function with
    /// signature `extsload(bytes32,uint256)` and selector `0x35fd631a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExtsloadWithStartSlotReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getLiquidity` function
    /// with signature `getLiquidity(bytes32,address,int24,int24)` and selector
    /// `0x33aa955b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidityWithOwnerReturn {
        pub liquidity: u128,
    }
    ///Container type for all return fields from the `getLiquidity` function
    /// with signature `getLiquidity(bytes32)` and selector `0xfa6793d5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidityReturn {
        pub liquidity: u128,
    }
    ///Container type for all return fields from the `getLock` function with
    /// signature `getLock(uint256)` and selector `0xd68f4dd1`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLockReturn {
        pub locker: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getPosition` function
    /// with signature `getPosition(bytes32,address,int24,int24)` and selector
    /// `0x048d9c70`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPositionReturn {
        pub position: Info,
    }
    ///Container type for all return fields from the `getSlot0` function with
    /// signature `getSlot0(bytes32)` and selector `0xc815641c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSlot0Return {
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub tick: i32,
        pub protocol_swap_fee: u8,
        pub protocol_withdraw_fee: u8,
        pub hook_swap_fee: u8,
        pub hook_withdraw_fee: u8,
    }
    ///Container type for all return fields from the `hookFeesAccrued` function
    /// with signature `hookFeesAccrued(address,address)` and selector
    /// `0xb4c41939`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct HookFeesAccruedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `initialize` function with
    /// signature `initialize((address,address,uint24,int24,address),uint160)`
    /// and selector `0x6276cbbe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct InitializeReturn {
        pub tick: i32,
    }
    ///Container type for all return fields from the `isApprovedForAll`
    /// function with signature `isApprovedForAll(address,address)` and selector
    /// `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `lock` function with
    /// signature `lock(bytes)` and selector `0x81548319`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LockReturn {
        pub result: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `lockData` function with
    /// signature `lockData()` and selector `0xf8fcd156`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LockDataReturn {
        pub length: u128,
        pub nonzero_delta_count: u128,
    }
    ///Container type for all return fields from the `modifyPosition` function
    /// with signature
    /// `modifyPosition((address,address,uint24,int24,address),(int24,int24,
    /// int256))` and selector `0x555a4733`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ModifyPositionReturn {
        pub delta: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `onERC1155BatchReceived`
    /// function with signature
    /// `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and
    /// selector `0xbc197c81`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received`
    /// function with signature
    /// `onERC1155Received(address,address,uint256,uint256,bytes)` and selector
    /// `0xf23a6e61`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with
    /// signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pools` function with
    /// signature `pools(bytes32)` and selector `0xb5217bb4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolsReturn {
        pub slot_0: Slot0,
        pub fee_growth_global_0x128: ::ethers::core::types::U256,
        pub fee_growth_global_1x128: ::ethers::core::types::U256,
        pub liquidity: u128,
    }
    ///Container type for all return fields from the `protocolFeeController`
    /// function with signature `protocolFeeController()` and selector
    /// `0xf02de3b2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProtocolFeeControllerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `protocolFeesAccrued`
    /// function with signature `protocolFeesAccrued(address)` and selector
    /// `0x97e8cd4e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProtocolFeesAccruedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `reservesOf` function with
    /// signature `reservesOf(address)` and selector `0x93c85a21`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ReservesOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `settle` function with
    /// signature `settle(address)` and selector `0x6a256b29`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SettleReturn {
        pub paid: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `supportsInterface`
    /// function with signature `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `swap` function with
    /// signature `swap((address,address,uint24,int24,address),(bool,int256,
    /// uint160))` and selector `0x1e2817de`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapReturn {
        pub delta: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `uri` function with
    /// signature `uri(uint256)` and selector `0x0e89341c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UriReturn(pub ::std::string::String);
    ///`ModifyPositionParams(int24,int24,int256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ModifyPositionParams {
        pub tick_lower: i32,
        pub tick_upper: i32,
        pub liquidity_delta: ::ethers::core::types::I256,
    }
    ///`SwapParams(bool,int256,uint160)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapParams {
        pub zero_for_one: bool,
        pub amount_specified: ::ethers::core::types::I256,
        pub sqrt_price_limit_x96: ::ethers::core::types::U256,
    }
    ///`Slot0(uint160,int24,uint8,uint8,uint8,uint8)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Slot0 {
        pub sqrt_price_x96: ::ethers::core::types::U256,
        pub tick: i32,
        pub protocol_swap_fee: u8,
        pub protocol_withdraw_fee: u8,
        pub hook_swap_fee: u8,
        pub hook_withdraw_fee: u8,
    }
    ///`PoolKey(address,address,uint24,int24,address)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolKey {
        pub currency_0: ::ethers::core::types::Address,
        pub currency_1: ::ethers::core::types::Address,
        pub fee: u32,
        pub tick_spacing: i32,
        pub hooks: ::ethers::core::types::Address,
    }
    ///`Info(uint128,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Info {
        pub liquidity: u128,
        pub fee_growth_inside_0_last_x128: ::ethers::core::types::U256,
        pub fee_growth_inside_1_last_x128: ::ethers::core::types::U256,
    }
}
