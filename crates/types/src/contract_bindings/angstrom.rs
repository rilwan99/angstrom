pub use angstrom::*;
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
pub mod angstrom {
    /*
    const _: () = {
        ::core::include_bytes!(
            "/home/will/ghq/github.com/SorellaLabs/Angstrom/anvil/abi/Angstrom.json",
        );
    };
    */
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("aPoolManager"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("contract IPoolManager"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("aOwner"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("afterDonate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("afterDonate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("afterInitialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("afterInitialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int24"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("afterModifyPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "afterModifyPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPoolManager.ModifyPositionParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("BalanceDelta"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("afterSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("afterSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPoolManager.SwapParams",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("BalanceDelta"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeDonate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beforeDonate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeInitialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beforeInitialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        160usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint160"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeModifyPosition"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "beforeModifyPosition",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPoolManager.ModifyPositionParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("beforeSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aCaller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aPool"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct PoolKey"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aParams"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(160usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IPoolManager.SwapParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rSelector"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        4usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes4"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cancelOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "cancelOwnershipHandover",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimFees"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("claimFees"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aCurrency"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("Currency"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aRecipient"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("completeOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "completeOwnershipHandover",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("eip712Domain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("eip712Domain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("fields"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        1usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes1"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("name"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("verifyingContract"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("salt"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("extensions"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getHooksCalls"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getHooksCalls"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Hooks.Calls"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lockAcquired"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("lockAcquired"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("aBatch"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ownershipHandoverExpiresAt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ownershipHandoverExpiresAt",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("result"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("poolManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("poolManager"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPoolManager"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("process"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("process"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                        ::std::vec![
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                            ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                                        ],
                                                                    ),
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Int(24usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                                                ],
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Batch"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("requestOwnershipHandover"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "requestOwnershipHandover",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipHandoverCanceled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipHandoverCanceled",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipHandoverRequested"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipHandoverRequested",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("oldOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC20TransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC20TransferFailed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HookAddressNotValid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "HookAddressNotValid",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("hooks"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HookNotImplemented"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("HookNotImplemented"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidPool"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LockFailure"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("LockFailure"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NativeTransferFailed",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NewOwnerIsZeroAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NewOwnerIsZeroAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoHandoverRequest"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NoHandoverRequest"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotPoolManager"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotPoolManager"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotSelf"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ANGSTROM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct Angstrom<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Angstrom<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Angstrom<M> {
        type Target = ::ethers::contract::Contract<M>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Angstrom<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Angstrom<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Angstrom))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Angstrom<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(address.into(), ANGSTROM_ABI.clone(), client))
        }

        ///Calls the contract's `afterDonate` (0x43c4407e) function
        pub fn after_donate(
            &self,
            p0: ::ethers::core::types::Address,
            p1: PoolKey,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([67, 196, 64, 126], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `afterInitialize` (0x6fe7e6eb) function
        pub fn after_initialize(
            &self,
            p0: ::ethers::core::types::Address,
            p1: PoolKey,
            p2: ::ethers::core::types::U256,
            p3: i32,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([111, 231, 230, 235], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `afterModifyPosition` (0x0e2059f5) function
        pub fn after_modify_position(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ModifyPositionParams,
            p2: ModifyPositionParams,
            p3: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([14, 32, 89, 245], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `afterSwap` (0xa5aa370a) function
        pub fn after_swap(
            &self,
            p0: ::ethers::core::types::Address,
            p1: SwapParams,
            p2: SwapParams,
            p3: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([165, 170, 55, 10], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `beforeDonate` (0x4dbb99a6) function
        pub fn before_donate(
            &self,
            p0: ::ethers::core::types::Address,
            p1: PoolKey,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([77, 187, 153, 166], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `beforeInitialize` (0xdc98354e) function
        pub fn before_initialize(
            &self,
            p0: ::ethers::core::types::Address,
            p1: PoolKey,
            p2: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([220, 152, 53, 78], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `beforeModifyPosition` (0x0dbe5dbd) function
        pub fn before_modify_position(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ModifyPositionParams,
            p2: ModifyPositionParams,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([13, 190, 93, 189], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `beforeSwap` (0xb3f97f80) function
        pub fn before_swap(
            &self,
            a_caller: ::ethers::core::types::Address,
            a_pool: PoolKey,
            a_params: SwapParams,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([179, 249, 127, 128], (a_caller, a_pool, a_params))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `cancelOwnershipHandover` (0x54d1f13d) function
        pub fn cancel_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 209, 241, 61], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `claimFees` (0x2dbfa735) function
        pub fn claim_fees(
            &self,
            a_currency: ::ethers::core::types::Address,
            a_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 191, 167, 53], (a_currency, a_recipient))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `completeOwnershipHandover` (0xf04e283e)
        /// function
        pub fn complete_ownership_handover(
            &self,
            pending_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 78, 40, 62], pending_owner)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `eip712Domain` (0x84b0196e) function
        pub fn eip_712_domain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                [u8; 1],
                ::std::string::String,
                ::std::string::String,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
                [u8; 32],
                ::std::vec::Vec<::ethers::core::types::U256>,
            ),
        > {
            self.0
                .method_hash([132, 176, 25, 110], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `getHooksCalls` (0x612c39b7) function
        pub fn get_hooks_calls(&self) -> ::ethers::contract::builders::ContractCall<M, Calls> {
            self.0
                .method_hash([97, 44, 57, 183], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `lockAcquired` (0xab6291fe) function
        pub fn lock_acquired(
            &self,
            a_batch: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([171, 98, 145, 254], a_batch)
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

        ///Calls the contract's `ownershipHandoverExpiresAt` (0xfee81cf4)
        /// function
        pub fn ownership_handover_expires_at(
            &self,
            pending_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([254, 232, 28, 244], pending_owner)
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `poolManager` (0xdc4c90d3) function
        pub fn pool_manager(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([220, 76, 144, 211], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `process` (0xac8a9f85) function
        pub fn process(&self, p0: Batch) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 138, 159, 133], (p0,))
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `requestOwnershipHandover` (0x25692962)
        /// function
        pub fn request_ownership_handover(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 105, 41, 98], ())
                .expect("method not found (this should never happen)")
        }

        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }

        ///Gets the contract's `OwnershipHandoverCanceled` event
        pub fn ownership_handover_canceled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipHandoverCanceledFilter,
        > {
            self.0.event()
        }

        ///Gets the contract's `OwnershipHandoverRequested` event
        pub fn ownership_handover_requested_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipHandoverRequestedFilter,
        > {
            self.0.event()
        }

        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
            self.0.event()
        }

        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AngstromEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Angstrom<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    ///Custom Error type `HookNotImplemented` with signature
    /// `HookNotImplemented()` and selector `0x0a85dc29`
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
    #[etherror(name = "HookNotImplemented", abi = "HookNotImplemented()")]
    pub struct HookNotImplemented;
    ///Custom Error type `InvalidPool` with signature `InvalidPool()` and
    /// selector `0x2083cd40`
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
    #[etherror(name = "InvalidPool", abi = "InvalidPool()")]
    pub struct InvalidPool;
    ///Custom Error type `LockFailure` with signature `LockFailure()` and
    /// selector `0xa40afa38`
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
    #[etherror(name = "LockFailure", abi = "LockFailure()")]
    pub struct LockFailure;
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
    ///Custom Error type `NewOwnerIsZeroAddress` with signature
    /// `NewOwnerIsZeroAddress()` and selector `0x7448fbae`
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
    #[etherror(name = "NewOwnerIsZeroAddress", abi = "NewOwnerIsZeroAddress()")]
    pub struct NewOwnerIsZeroAddress;
    ///Custom Error type `NoHandoverRequest` with signature
    /// `NoHandoverRequest()` and selector `0x6f5e8818`
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
    #[etherror(name = "NoHandoverRequest", abi = "NoHandoverRequest()")]
    pub struct NoHandoverRequest;
    ///Custom Error type `NotPoolManager` with signature `NotPoolManager()` and
    /// selector `0xae18210a`
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
    #[etherror(name = "NotPoolManager", abi = "NotPoolManager()")]
    pub struct NotPoolManager;
    ///Custom Error type `NotSelf` with signature `NotSelf()` and selector
    /// `0x29c3b7ee`
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
    #[etherror(name = "NotSelf", abi = "NotSelf()")]
    pub struct NotSelf;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and
    /// selector `0x82b42900`
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
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AngstromErrors {
        ERC20TransferFailed(ERC20TransferFailed),
        HookAddressNotValid(HookAddressNotValid),
        HookNotImplemented(HookNotImplemented),
        InvalidPool(InvalidPool),
        LockFailure(LockFailure),
        NativeTransferFailed(NativeTransferFailed),
        NewOwnerIsZeroAddress(NewOwnerIsZeroAddress),
        NoHandoverRequest(NoHandoverRequest),
        NotPoolManager(NotPoolManager),
        NotSelf(NotSelf),
        Unauthorized(Unauthorized),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AngstromErrors {
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
                <ERC20TransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC20TransferFailed(decoded));
            }
            if let Ok(decoded) =
                <HookAddressNotValid as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HookAddressNotValid(decoded));
            }
            if let Ok(decoded) =
                <HookNotImplemented as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::HookNotImplemented(decoded));
            }
            if let Ok(decoded) = <InvalidPool as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidPool(decoded));
            }
            if let Ok(decoded) = <LockFailure as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LockFailure(decoded));
            }
            if let Ok(decoded) =
                <NativeTransferFailed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeTransferFailed(decoded));
            }
            if let Ok(decoded) =
                <NewOwnerIsZeroAddress as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NewOwnerIsZeroAddress(decoded));
            }
            if let Ok(decoded) = <NoHandoverRequest as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NoHandoverRequest(decoded));
            }
            if let Ok(decoded) = <NotPoolManager as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotPoolManager(decoded));
            }
            if let Ok(decoded) = <NotSelf as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotSelf(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AngstromErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC20TransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HookAddressNotValid(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HookNotImplemented(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockFailure(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NewOwnerIsZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoHandoverRequest(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotPoolManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotSelf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unauthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AngstromErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC20TransferFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <HookAddressNotValid as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <HookNotImplemented as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidPool as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <LockFailure as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NativeTransferFailed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NewOwnerIsZeroAddress as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NoHandoverRequest as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NotPoolManager as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotSelf as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AngstromErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC20TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::HookAddressNotValid(element) => ::core::fmt::Display::fmt(element, f),
                Self::HookNotImplemented(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeTransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NewOwnerIsZeroAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoHandoverRequest(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotPoolManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSelf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AngstromErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC20TransferFailed> for AngstromErrors {
        fn from(value: ERC20TransferFailed) -> Self {
            Self::ERC20TransferFailed(value)
        }
    }
    impl ::core::convert::From<HookAddressNotValid> for AngstromErrors {
        fn from(value: HookAddressNotValid) -> Self {
            Self::HookAddressNotValid(value)
        }
    }
    impl ::core::convert::From<HookNotImplemented> for AngstromErrors {
        fn from(value: HookNotImplemented) -> Self {
            Self::HookNotImplemented(value)
        }
    }
    impl ::core::convert::From<InvalidPool> for AngstromErrors {
        fn from(value: InvalidPool) -> Self {
            Self::InvalidPool(value)
        }
    }
    impl ::core::convert::From<LockFailure> for AngstromErrors {
        fn from(value: LockFailure) -> Self {
            Self::LockFailure(value)
        }
    }
    impl ::core::convert::From<NativeTransferFailed> for AngstromErrors {
        fn from(value: NativeTransferFailed) -> Self {
            Self::NativeTransferFailed(value)
        }
    }
    impl ::core::convert::From<NewOwnerIsZeroAddress> for AngstromErrors {
        fn from(value: NewOwnerIsZeroAddress) -> Self {
            Self::NewOwnerIsZeroAddress(value)
        }
    }
    impl ::core::convert::From<NoHandoverRequest> for AngstromErrors {
        fn from(value: NoHandoverRequest) -> Self {
            Self::NoHandoverRequest(value)
        }
    }
    impl ::core::convert::From<NotPoolManager> for AngstromErrors {
        fn from(value: NotPoolManager) -> Self {
            Self::NotPoolManager(value)
        }
    }
    impl ::core::convert::From<NotSelf> for AngstromErrors {
        fn from(value: NotSelf) -> Self {
            Self::NotSelf(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for AngstromErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
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
    #[ethevent(name = "OwnershipHandoverCanceled", abi = "OwnershipHandoverCanceled(address)")]
    pub struct OwnershipHandoverCanceledFilter {
        #[ethevent(indexed)]
        pub pending_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "OwnershipHandoverRequested", abi = "OwnershipHandoverRequested(address)")]
    pub struct OwnershipHandoverRequestedFilter {
        #[ethevent(indexed)]
        pub pending_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address,address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub old_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AngstromEvents {
        OwnershipHandoverCanceledFilter(OwnershipHandoverCanceledFilter),
        OwnershipHandoverRequestedFilter(OwnershipHandoverRequestedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for AngstromEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OwnershipHandoverCanceledFilter::decode_log(log) {
                return Ok(AngstromEvents::OwnershipHandoverCanceledFilter(decoded));
            }
            if let Ok(decoded) = OwnershipHandoverRequestedFilter::decode_log(log) {
                return Ok(AngstromEvents::OwnershipHandoverRequestedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AngstromEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AngstromEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OwnershipHandoverCanceledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipHandoverRequestedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<OwnershipHandoverCanceledFilter> for AngstromEvents {
        fn from(value: OwnershipHandoverCanceledFilter) -> Self {
            Self::OwnershipHandoverCanceledFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverRequestedFilter> for AngstromEvents {
        fn from(value: OwnershipHandoverRequestedFilter) -> Self {
            Self::OwnershipHandoverRequestedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AngstromEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `afterDonate` function
    /// with signature
    /// `afterDonate(address,(address,address,uint24,int24,address),uint256,
    /// uint256)` and selector `0x43c4407e`
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
        name = "afterDonate",
        abi = "afterDonate(address,(address,address,uint24,int24,address),uint256,uint256)"
    )]
    pub struct AfterDonateCall(
        pub ::ethers::core::types::Address,
        pub PoolKey,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `afterInitialize`
    /// function with signature
    /// `afterInitialize(address,(address,address,uint24,int24,address),uint160,
    /// int24)` and selector `0x6fe7e6eb`
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
        name = "afterInitialize",
        abi = "afterInitialize(address,(address,address,uint24,int24,address),uint160,int24)"
    )]
    pub struct AfterInitializeCall(
        pub ::ethers::core::types::Address,
        pub PoolKey,
        pub ::ethers::core::types::U256,
        pub i32,
    );
    ///Container type for all input parameters for the `afterModifyPosition`
    /// function with signature
    /// `afterModifyPosition(address,(address,address,uint24,int24,address),
    /// (int24,int24,int256),int256)` and selector `0x0e2059f5`
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
        name = "afterModifyPosition",
        abi = "afterModifyPosition(address,(address,address,uint24,int24,address),(int24,int24,\
               int256),int256)"
    )]
    pub struct AfterModifyPositionCall(
        pub ::ethers::core::types::Address,
        pub ModifyPositionParams,
        pub ModifyPositionParams,
        pub ::ethers::core::types::I256,
    );
    ///Container type for all input parameters for the `afterSwap` function
    /// with signature
    /// `afterSwap(address,(address,address,uint24,int24,address),(bool,int256,
    /// uint160),int256)` and selector `0xa5aa370a`
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
        name = "afterSwap",
        abi = "afterSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160),\
               int256)"
    )]
    pub struct AfterSwapCall(
        pub ::ethers::core::types::Address,
        pub SwapParams,
        pub SwapParams,
        pub ::ethers::core::types::I256,
    );
    ///Container type for all input parameters for the `beforeDonate` function
    /// with signature
    /// `beforeDonate(address,(address,address,uint24,int24,address),uint256,
    /// uint256)` and selector `0x4dbb99a6`
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
        name = "beforeDonate",
        abi = "beforeDonate(address,(address,address,uint24,int24,address),uint256,uint256)"
    )]
    pub struct BeforeDonateCall(
        pub ::ethers::core::types::Address,
        pub PoolKey,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `beforeInitialize`
    /// function with signature
    /// `beforeInitialize(address,(address,address,uint24,int24,address),
    /// uint160)` and selector `0xdc98354e`
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
        name = "beforeInitialize",
        abi = "beforeInitialize(address,(address,address,uint24,int24,address),uint160)"
    )]
    pub struct BeforeInitializeCall(
        pub ::ethers::core::types::Address,
        pub PoolKey,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `beforeModifyPosition`
    /// function with signature
    /// `beforeModifyPosition(address,(address,address,uint24,int24,address),
    /// (int24,int24,int256))` and selector `0x0dbe5dbd`
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
        name = "beforeModifyPosition",
        abi = "beforeModifyPosition(address,(address,address,uint24,int24,address),(int24,int24,\
               int256))"
    )]
    pub struct BeforeModifyPositionCall(
        pub ::ethers::core::types::Address,
        pub ModifyPositionParams,
        pub ModifyPositionParams,
    );
    ///Container type for all input parameters for the `beforeSwap` function
    /// with signature
    /// `beforeSwap(address,(address,address,uint24,int24,address),(bool,int256,
    /// uint160))` and selector `0xb3f97f80`
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
        name = "beforeSwap",
        abi = "beforeSwap(address,(address,address,uint24,int24,address),(bool,int256,uint160))"
    )]
    pub struct BeforeSwapCall {
        pub a_caller: ::ethers::core::types::Address,
        pub a_pool: PoolKey,
        pub a_params: SwapParams,
    }
    ///Container type for all input parameters for the
    /// `cancelOwnershipHandover` function with signature
    /// `cancelOwnershipHandover()` and selector `0x54d1f13d`
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
    #[ethcall(name = "cancelOwnershipHandover", abi = "cancelOwnershipHandover()")]
    pub struct CancelOwnershipHandoverCall;
    ///Container type for all input parameters for the `claimFees` function
    /// with signature `claimFees(address,address)` and selector `0x2dbfa735`
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
    #[ethcall(name = "claimFees", abi = "claimFees(address,address)")]
    pub struct ClaimFeesCall {
        pub a_currency: ::ethers::core::types::Address,
        pub a_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the
    /// `completeOwnershipHandover` function with signature
    /// `completeOwnershipHandover(address)` and selector `0xf04e283e`
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
    #[ethcall(name = "completeOwnershipHandover", abi = "completeOwnershipHandover(address)")]
    pub struct CompleteOwnershipHandoverCall {
        pub pending_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `eip712Domain` function
    /// with signature `eip712Domain()` and selector `0x84b0196e`
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
    #[ethcall(name = "eip712Domain", abi = "eip712Domain()")]
    pub struct Eip712DomainCall;
    ///Container type for all input parameters for the `getHooksCalls` function
    /// with signature `getHooksCalls()` and selector `0x612c39b7`
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
    #[ethcall(name = "getHooksCalls", abi = "getHooksCalls()")]
    pub struct GetHooksCallsCall;
    ///Container type for all input parameters for the `lockAcquired` function
    /// with signature `lockAcquired(bytes)` and selector `0xab6291fe`
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
    #[ethcall(name = "lockAcquired", abi = "lockAcquired(bytes)")]
    pub struct LockAcquiredCall {
        pub a_batch: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the
    /// `ownershipHandoverExpiresAt` function with signature
    /// `ownershipHandoverExpiresAt(address)` and selector `0xfee81cf4`
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
    #[ethcall(name = "ownershipHandoverExpiresAt", abi = "ownershipHandoverExpiresAt(address)")]
    pub struct OwnershipHandoverExpiresAtCall {
        pub pending_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `poolManager` function
    /// with signature `poolManager()` and selector `0xdc4c90d3`
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
    #[ethcall(name = "poolManager", abi = "poolManager()")]
    pub struct PoolManagerCall;
    ///Container type for all input parameters for the `process` function with
    /// signature `process((((address,address,uint24,int24,address),address,
    /// uint256)[],(((address,address,uint24,int24,address),address,address,
    /// uint128,uint128,uint256,uint256,uint256,bytes,bytes),bytes,uint256)[],
    /// ((address,address,uint128,uint128,uint256,uint256,bytes,bytes),bytes,
    /// uint256,uint256)[],(address,int256)[],((address,address,uint24,int24,
    /// address),uint256,uint256)[]))` and selector `0xac8a9f85`
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
        name = "process",
        abi = "process((((address,address,uint24,int24,address),address,uint256)[],(((address,\
               address,uint24,int24,address),address,address,uint128,uint128,uint256,uint256,\
               uint256,bytes,bytes),bytes,uint256)[],((address,address,uint128,uint128,uint256,\
               uint256,bytes,bytes),bytes,uint256,uint256)[],(address,int256)[],((address,address,\
               uint24,int24,address),uint256,uint256)[]))"
    )]
    pub struct ProcessCall(pub Batch);
    ///Container type for all input parameters for the `renounceOwnership`
    /// function with signature `renounceOwnership()` and selector `0x715018a6`
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
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the
    /// `requestOwnershipHandover` function with signature
    /// `requestOwnershipHandover()` and selector `0x25692962`
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
    #[ethcall(name = "requestOwnershipHandover", abi = "requestOwnershipHandover()")]
    pub struct RequestOwnershipHandoverCall;
    ///Container type for all input parameters for the `transferOwnership`
    /// function with signature `transferOwnership(address)` and selector
    /// `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AngstromCalls {
        AfterDonate(AfterDonateCall),
        AfterInitialize(AfterInitializeCall),
        AfterModifyPosition(AfterModifyPositionCall),
        AfterSwap(AfterSwapCall),
        BeforeDonate(BeforeDonateCall),
        BeforeInitialize(BeforeInitializeCall),
        BeforeModifyPosition(BeforeModifyPositionCall),
        BeforeSwap(BeforeSwapCall),
        CancelOwnershipHandover(CancelOwnershipHandoverCall),
        ClaimFees(ClaimFeesCall),
        CompleteOwnershipHandover(CompleteOwnershipHandoverCall),
        Eip712Domain(Eip712DomainCall),
        GetHooksCalls(GetHooksCallsCall),
        LockAcquired(LockAcquiredCall),
        Owner(OwnerCall),
        OwnershipHandoverExpiresAt(OwnershipHandoverExpiresAtCall),
        PoolManager(PoolManagerCall),
        Process(ProcessCall),
        RenounceOwnership(RenounceOwnershipCall),
        RequestOwnershipHandover(RequestOwnershipHandoverCall),
        TransferOwnership(TransferOwnershipCall),
    }
    impl ::ethers::core::abi::AbiDecode for AngstromCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AfterDonateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AfterDonate(decoded));
            }
            if let Ok(decoded) =
                <AfterInitializeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AfterInitialize(decoded));
            }
            if let Ok(decoded) =
                <AfterModifyPositionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AfterModifyPosition(decoded));
            }
            if let Ok(decoded) = <AfterSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AfterSwap(decoded));
            }
            if let Ok(decoded) = <BeforeDonateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BeforeDonate(decoded));
            }
            if let Ok(decoded) =
                <BeforeInitializeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BeforeInitialize(decoded));
            }
            if let Ok(decoded) =
                <BeforeModifyPositionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BeforeModifyPosition(decoded));
            }
            if let Ok(decoded) = <BeforeSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BeforeSwap(decoded));
            }
            if let Ok(decoded) =
                <CancelOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CancelOwnershipHandover(decoded));
            }
            if let Ok(decoded) = <ClaimFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimFees(decoded));
            }
            if let Ok(decoded) =
                <CompleteOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CompleteOwnershipHandover(decoded));
            }
            if let Ok(decoded) = <Eip712DomainCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Eip712Domain(decoded));
            }
            if let Ok(decoded) = <GetHooksCallsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetHooksCalls(decoded));
            }
            if let Ok(decoded) = <LockAcquiredCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LockAcquired(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) =
                <OwnershipHandoverExpiresAtCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OwnershipHandoverExpiresAt(decoded));
            }
            if let Ok(decoded) = <PoolManagerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolManager(decoded));
            }
            if let Ok(decoded) = <ProcessCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Process(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RequestOwnershipHandoverCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RequestOwnershipHandover(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AngstromCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AfterDonate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AfterInitialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AfterModifyPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AfterSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BeforeDonate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BeforeInitialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BeforeModifyPosition(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CancelOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ClaimFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CompleteOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Eip712Domain(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetHooksCalls(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LockAcquired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnershipHandoverExpiresAt(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolManager(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Process(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RequestOwnershipHandover(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AngstromCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterDonate(element) => ::core::fmt::Display::fmt(element, f),
                Self::AfterInitialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::AfterModifyPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::AfterSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeforeDonate(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeforeInitialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeforeModifyPosition(element) => ::core::fmt::Display::fmt(element, f),
                Self::BeforeSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::CancelOwnershipHandover(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::CompleteOwnershipHandover(element) => ::core::fmt::Display::fmt(element, f),
                Self::Eip712Domain(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetHooksCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::LockAcquired(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipHandoverExpiresAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolManager(element) => ::core::fmt::Display::fmt(element, f),
                Self::Process(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::RequestOwnershipHandover(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterDonateCall> for AngstromCalls {
        fn from(value: AfterDonateCall) -> Self {
            Self::AfterDonate(value)
        }
    }
    impl ::core::convert::From<AfterInitializeCall> for AngstromCalls {
        fn from(value: AfterInitializeCall) -> Self {
            Self::AfterInitialize(value)
        }
    }
    impl ::core::convert::From<AfterModifyPositionCall> for AngstromCalls {
        fn from(value: AfterModifyPositionCall) -> Self {
            Self::AfterModifyPosition(value)
        }
    }
    impl ::core::convert::From<AfterSwapCall> for AngstromCalls {
        fn from(value: AfterSwapCall) -> Self {
            Self::AfterSwap(value)
        }
    }
    impl ::core::convert::From<BeforeDonateCall> for AngstromCalls {
        fn from(value: BeforeDonateCall) -> Self {
            Self::BeforeDonate(value)
        }
    }
    impl ::core::convert::From<BeforeInitializeCall> for AngstromCalls {
        fn from(value: BeforeInitializeCall) -> Self {
            Self::BeforeInitialize(value)
        }
    }
    impl ::core::convert::From<BeforeModifyPositionCall> for AngstromCalls {
        fn from(value: BeforeModifyPositionCall) -> Self {
            Self::BeforeModifyPosition(value)
        }
    }
    impl ::core::convert::From<BeforeSwapCall> for AngstromCalls {
        fn from(value: BeforeSwapCall) -> Self {
            Self::BeforeSwap(value)
        }
    }
    impl ::core::convert::From<CancelOwnershipHandoverCall> for AngstromCalls {
        fn from(value: CancelOwnershipHandoverCall) -> Self {
            Self::CancelOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<ClaimFeesCall> for AngstromCalls {
        fn from(value: ClaimFeesCall) -> Self {
            Self::ClaimFees(value)
        }
    }
    impl ::core::convert::From<CompleteOwnershipHandoverCall> for AngstromCalls {
        fn from(value: CompleteOwnershipHandoverCall) -> Self {
            Self::CompleteOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<Eip712DomainCall> for AngstromCalls {
        fn from(value: Eip712DomainCall) -> Self {
            Self::Eip712Domain(value)
        }
    }
    impl ::core::convert::From<GetHooksCallsCall> for AngstromCalls {
        fn from(value: GetHooksCallsCall) -> Self {
            Self::GetHooksCalls(value)
        }
    }
    impl ::core::convert::From<LockAcquiredCall> for AngstromCalls {
        fn from(value: LockAcquiredCall) -> Self {
            Self::LockAcquired(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AngstromCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<OwnershipHandoverExpiresAtCall> for AngstromCalls {
        fn from(value: OwnershipHandoverExpiresAtCall) -> Self {
            Self::OwnershipHandoverExpiresAt(value)
        }
    }
    impl ::core::convert::From<PoolManagerCall> for AngstromCalls {
        fn from(value: PoolManagerCall) -> Self {
            Self::PoolManager(value)
        }
    }
    impl ::core::convert::From<ProcessCall> for AngstromCalls {
        fn from(value: ProcessCall) -> Self {
            Self::Process(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for AngstromCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RequestOwnershipHandoverCall> for AngstromCalls {
        fn from(value: RequestOwnershipHandoverCall) -> Self {
            Self::RequestOwnershipHandover(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AngstromCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    ///Container type for all return fields from the `afterDonate` function
    /// with signature
    /// `afterDonate(address,(address,address,uint24,int24,address),uint256,
    /// uint256)` and selector `0x43c4407e`
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
    pub struct AfterDonateReturn(pub [u8; 4]);
    ///Container type for all return fields from the `afterInitialize` function
    /// with signature
    /// `afterInitialize(address,(address,address,uint24,int24,address),uint160,
    /// int24)` and selector `0x6fe7e6eb`
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
    pub struct AfterInitializeReturn(pub [u8; 4]);
    ///Container type for all return fields from the `afterModifyPosition`
    /// function with signature
    /// `afterModifyPosition(address,(address,address,uint24,int24,address),
    /// (int24,int24,int256),int256)` and selector `0x0e2059f5`
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
    pub struct AfterModifyPositionReturn(pub [u8; 4]);
    ///Container type for all return fields from the `afterSwap` function with
    /// signature `afterSwap(address,(address,address,uint24,int24,address),
    /// (bool,int256,uint160),int256)` and selector `0xa5aa370a`
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
    pub struct AfterSwapReturn(pub [u8; 4]);
    ///Container type for all return fields from the `beforeDonate` function
    /// with signature
    /// `beforeDonate(address,(address,address,uint24,int24,address),uint256,
    /// uint256)` and selector `0x4dbb99a6`
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
    pub struct BeforeDonateReturn(pub [u8; 4]);
    ///Container type for all return fields from the `beforeInitialize`
    /// function with signature
    /// `beforeInitialize(address,(address,address,uint24,int24,address),
    /// uint160)` and selector `0xdc98354e`
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
    pub struct BeforeInitializeReturn(pub [u8; 4]);
    ///Container type for all return fields from the `beforeModifyPosition`
    /// function with signature
    /// `beforeModifyPosition(address,(address,address,uint24,int24,address),
    /// (int24,int24,int256))` and selector `0x0dbe5dbd`
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
    pub struct BeforeModifyPositionReturn(pub [u8; 4]);
    ///Container type for all return fields from the `beforeSwap` function with
    /// signature `beforeSwap(address,(address,address,uint24,int24,address),
    /// (bool,int256,uint160))` and selector `0xb3f97f80`
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
    pub struct BeforeSwapReturn {
        pub r_selector: [u8; 4],
    }
    ///Container type for all return fields from the `eip712Domain` function
    /// with signature `eip712Domain()` and selector `0x84b0196e`
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
    pub struct Eip712DomainReturn {
        pub fields: [u8; 1],
        pub name: ::std::string::String,
        pub version: ::std::string::String,
        pub chain_id: ::ethers::core::types::U256,
        pub verifying_contract: ::ethers::core::types::Address,
        pub salt: [u8; 32],
        pub extensions: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `getHooksCalls` function
    /// with signature `getHooksCalls()` and selector `0x612c39b7`
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
    pub struct GetHooksCallsReturn(pub Calls);
    ///Container type for all return fields from the `lockAcquired` function
    /// with signature `lockAcquired(bytes)` and selector `0xab6291fe`
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
    pub struct LockAcquiredReturn(pub ::ethers::core::types::Bytes);
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
    pub struct OwnerReturn {
        pub result: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the
    /// `ownershipHandoverExpiresAt` function with signature
    /// `ownershipHandoverExpiresAt(address)` and selector `0xfee81cf4`
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
    pub struct OwnershipHandoverExpiresAtReturn {
        pub result: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `poolManager` function
    /// with signature `poolManager()` and selector `0xdc4c90d3`
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
    pub struct PoolManagerReturn(pub ::ethers::core::types::Address);
    ///`Batch(((address,address,uint24,int24,address),address,uint256)[],
    /// (((address,address,uint24,int24,address),address,address,uint128,
    /// uint128,uint256,uint256,uint256,bytes,bytes),bytes,uint256)[],((address,
    /// address,uint128,uint128,uint256,uint256,bytes,bytes),bytes,uint256,
    /// uint256)[],(address,int256)[],((address,address,uint24,int24,address),
    /// uint256,uint256)[])`
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
    pub struct Batch {
        pub swaps: ::std::vec::Vec<PoolSwap>,
        pub lvr: ::std::vec::Vec<LvrSettlement>,
        pub users: ::std::vec::Vec<UserSettlement>,
        pub currencies: ::std::vec::Vec<CurrencySettlement>,
        pub pools: ::std::vec::Vec<PoolFees>,
    }
    ///`CurrencySettlement(address,int256)`
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
    pub struct CurrencySettlement {
        pub currency: ::ethers::core::types::Address,
        pub amount_net: ::ethers::core::types::I256,
    }
    ///`Calls(bool,bool,bool,bool,bool,bool,bool,bool)`
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
    pub struct Calls {
        pub before_initialize: bool,
        pub after_initialize: bool,
        pub before_modify_position: bool,
        pub after_modify_position: bool,
        pub before_swap: bool,
        pub after_swap: bool,
        pub before_donate: bool,
        pub after_donate: bool,
    }
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
    ///`LvrSettlement(((address,address,uint24,int24,address),address,address,
    /// uint128,uint128,uint256,uint256,uint256,bytes,bytes),bytes,uint256)`
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
    pub struct LvrSettlement {
        pub order: SearcherOrder,
        pub signature: ::ethers::core::types::Bytes,
        pub gas_actual: ::ethers::core::types::U256,
    }
    ///`PoolFees((address,address,uint24,int24,address),uint256,uint256)`
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
    pub struct PoolFees {
        pub pool: PoolKey,
        pub fees_0: ::ethers::core::types::U256,
        pub fees_1: ::ethers::core::types::U256,
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
    ///`PoolSwap((address,address,uint24,int24,address),address,uint256)`
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
    pub struct PoolSwap {
        pub pool: PoolKey,
        pub currency_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///`SearcherOrder((address,address,uint24,int24,address),address,address,
    /// uint128,uint128,uint256,uint256,uint256,bytes,bytes)`
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
    pub struct SearcherOrder {
        pub pool: PoolKey,
        pub currency_in: ::ethers::core::types::Address,
        pub currency_out: ::ethers::core::types::Address,
        pub amount_in: u128,
        pub amount_out: u128,
        pub deadline: ::ethers::core::types::U256,
        pub gas_cap: ::ethers::core::types::U256,
        pub bribe: ::ethers::core::types::U256,
        pub pre_hook: ::ethers::core::types::Bytes,
        pub post_hook: ::ethers::core::types::Bytes,
    }
    ///`UserOrder(address,address,uint128,uint128,uint256,uint256,bytes,bytes)`
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
    pub struct UserOrder {
        pub currency_in: ::ethers::core::types::Address,
        pub currency_out: ::ethers::core::types::Address,
        pub amount_in: u128,
        pub amount_out_min: u128,
        pub deadline: ::ethers::core::types::U256,
        pub gas_cap: ::ethers::core::types::U256,
        pub pre_hook: ::ethers::core::types::Bytes,
        pub post_hook: ::ethers::core::types::Bytes,
    }
    ///`UserSettlement((address,address,uint128,uint128,uint256,uint256,bytes,
    /// bytes),bytes,uint256,uint256)`
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
    pub struct UserSettlement {
        pub order: UserOrder,
        pub signature: ::ethers::core::types::Bytes,
        pub amount_out: ::ethers::core::types::U256,
        pub gas_actual: ::ethers::core::types::U256,
    }
}
