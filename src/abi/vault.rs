    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct AuthorizerChanged {
            pub new_authorizer: Vec<u8>,
        }
        impl AuthorizerChanged {
            const TOPIC_ID: [u8; 32] = [
                148u8,
                185u8,
                121u8,
                182u8,
                131u8,
                26u8,
                81u8,
                41u8,
                62u8,
                38u8,
                65u8,
                66u8,
                111u8,
                151u8,
                116u8,
                127u8,
                238u8,
                212u8,
                111u8,
                23u8,
                119u8,
                159u8,
                237u8,
                156u8,
                209u8,
                141u8,
                30u8,
                206u8,
                252u8,
                254u8,
                146u8,
                239u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 0usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Ok(Self {
                    new_authorizer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'new_authorizer' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
        }
        impl substreams_ethereum::Event for AuthorizerChanged {
            const NAME: &'static str = "AuthorizerChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct ExternalBalanceTransfer {
            pub token: Vec<u8>,
            pub sender: Vec<u8>,
            pub recipient: Vec<u8>,
            pub amount: ethabi::Uint,
        }
        impl ExternalBalanceTransfer {
            const TOPIC_ID: [u8; 32] = [
                84u8,
                10u8,
                26u8,
                63u8,
                40u8,
                52u8,
                12u8,
                174u8,
                195u8,
                54u8,
                200u8,
                29u8,
                141u8,
                123u8,
                61u8,
                241u8,
                57u8,
                238u8,
                92u8,
                220u8,
                24u8,
                57u8,
                164u8,
                242u8,
                131u8,
                215u8,
                235u8,
                183u8,
                234u8,
                174u8,
                45u8,
                92u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    sender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'sender' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for ExternalBalanceTransfer {
            const NAME: &'static str = "ExternalBalanceTransfer";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FlashLoan {
            pub recipient: Vec<u8>,
            pub token: Vec<u8>,
            pub amount: ethabi::Uint,
            pub fee_amount: ethabi::Uint,
        }
        impl FlashLoan {
            const TOPIC_ID: [u8; 32] = [
                13u8,
                125u8,
                117u8,
                224u8,
                26u8,
                185u8,
                87u8,
                128u8,
                211u8,
                205u8,
                28u8,
                142u8,
                192u8,
                221u8,
                108u8,
                44u8,
                225u8,
                158u8,
                58u8,
                32u8,
                66u8,
                126u8,
                236u8,
                139u8,
                245u8,
                50u8,
                131u8,
                182u8,
                251u8,
                142u8,
                149u8,
                240u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    recipient: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'recipient' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    fee_amount: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for FlashLoan {
            const NAME: &'static str = "FlashLoan";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct InternalBalanceChanged {
            pub user: Vec<u8>,
            pub token: Vec<u8>,
            pub delta: num_bigint::BigInt,
        }
        impl InternalBalanceChanged {
            const TOPIC_ID: [u8; 32] = [
                24u8,
                225u8,
                234u8,
                65u8,
                57u8,
                230u8,
                132u8,
                19u8,
                215u8,
                208u8,
                138u8,
                167u8,
                82u8,
                231u8,
                21u8,
                104u8,
                227u8,
                107u8,
                44u8,
                91u8,
                249u8,
                64u8,
                137u8,
                51u8,
                20u8,
                194u8,
                197u8,
                176u8,
                30u8,
                170u8,
                12u8,
                66u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Int(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    user: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'user' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    delta: {
                        values.pop().expect(INTERNAL_ERR);
                        num_bigint::BigInt::from_signed_bytes_be(
                            log.data[0usize..32usize].as_ref(),
                        )
                    },
                })
            }
        }
        impl substreams_ethereum::Event for InternalBalanceChanged {
            const NAME: &'static str = "InternalBalanceChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PausedStateChanged {
            pub paused: bool,
        }
        impl PausedStateChanged {
            const TOPIC_ID: [u8; 32] = [
                158u8,
                58u8,
                94u8,
                55u8,
                34u8,
                69u8,
                50u8,
                222u8,
                166u8,
                123u8,
                137u8,
                250u8,
                206u8,
                24u8,
                87u8,
                3u8,
                115u8,
                138u8,
                34u8,
                138u8,
                110u8,
                138u8,
                35u8,
                222u8,
                229u8,
                70u8,
                150u8,
                1u8,
                128u8,
                211u8,
                190u8,
                100u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    paused: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for PausedStateChanged {
            const NAME: &'static str = "PausedStateChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PoolBalanceChanged {
            pub pool_id: [u8; 32usize],
            pub liquidity_provider: Vec<u8>,
            pub tokens: Vec<Vec<u8>>,
            pub deltas: Vec<ethabi::Uint>,
            pub protocol_fee_amounts: Vec<ethabi::Uint>,
        }
        impl PoolBalanceChanged {
            const TOPIC_ID: [u8; 32] = [
                129u8,
                189u8,
                44u8,
                131u8,
                246u8,
                179u8,
                167u8,
                76u8,
                142u8,
                206u8,
                255u8,
                109u8,
                156u8,
                208u8,
                74u8,
                212u8,
                28u8,
                154u8,
                103u8,
                179u8,
                80u8,
                195u8,
                188u8,
                187u8,
                119u8,
                175u8,
                60u8,
                59u8,
                185u8,
                87u8,
                49u8,
                6u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 192usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Uint(256usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    pool_id: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pool_id' from topic of type 'bytes32': {}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    liquidity_provider: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'liquidity_provider' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    deltas: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                    protocol_fee_amounts: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_uint().expect(INTERNAL_ERR))
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for PoolBalanceChanged {
            const NAME: &'static str = "PoolBalanceChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PoolBalanceManaged {
            pub pool_id: [u8; 32usize],
            pub asset_manager: Vec<u8>,
            pub token: Vec<u8>,
            pub cash_delta: num_bigint::BigInt,
            pub managed_delta: num_bigint::BigInt,
        }
        impl PoolBalanceManaged {
            const TOPIC_ID: [u8; 32] = [
                110u8,
                220u8,
                175u8,
                98u8,
                65u8,
                16u8,
                91u8,
                76u8,
                148u8,
                194u8,
                239u8,
                219u8,
                243u8,
                166u8,
                177u8,
                36u8,
                88u8,
                235u8,
                61u8,
                7u8,
                190u8,
                58u8,
                14u8,
                129u8,
                210u8,
                75u8,
                19u8,
                196u8,
                64u8,
                69u8,
                254u8,
                122u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Int(256usize),
                            ethabi::ParamType::Int(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    pool_id: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pool_id' from topic of type 'bytes32': {}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    asset_manager: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'asset_manager' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    cash_delta: {
                        values.pop().expect(INTERNAL_ERR);
                        num_bigint::BigInt::from_signed_bytes_be(
                            log.data[0usize..32usize].as_ref(),
                        )
                    },
                    managed_delta: {
                        values.pop().expect(INTERNAL_ERR);
                        num_bigint::BigInt::from_signed_bytes_be(
                            log.data[32usize..64usize].as_ref(),
                        )
                    },
                })
            }
        }
        impl substreams_ethereum::Event for PoolBalanceManaged {
            const NAME: &'static str = "PoolBalanceManaged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct PoolRegistered {
            pub pool_id: [u8; 32usize],
            pub pool_address: Vec<u8>,
            pub specialization: ethabi::Uint,
        }
        impl PoolRegistered {
            const TOPIC_ID: [u8; 32] = [
                60u8,
                19u8,
                188u8,
                48u8,
                184u8,
                232u8,
                120u8,
                197u8,
                63u8,
                210u8,
                163u8,
                107u8,
                103u8,
                148u8,
                9u8,
                192u8,
                115u8,
                175u8,
                215u8,
                89u8,
                80u8,
                190u8,
                67u8,
                216u8,
                133u8,
                135u8,
                104u8,
                233u8,
                86u8,
                251u8,
                194u8,
                14u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(8usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    pool_id: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pool_id' from topic of type 'bytes32': {}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    pool_address: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'pool_address' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    specialization: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for PoolRegistered {
            const NAME: &'static str = "PoolRegistered";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct RelayerApprovalChanged {
            pub relayer: Vec<u8>,
            pub sender: Vec<u8>,
            pub approved: bool,
        }
        impl RelayerApprovalChanged {
            const TOPIC_ID: [u8; 32] = [
                70u8,
                150u8,
                31u8,
                219u8,
                69u8,
                2u8,
                182u8,
                70u8,
                213u8,
                9u8,
                95u8,
                186u8,
                118u8,
                0u8,
                72u8,
                106u8,
                138u8,
                192u8,
                80u8,
                65u8,
                213u8,
                92u8,
                223u8,
                15u8,
                22u8,
                237u8,
                103u8,
                113u8,
                128u8,
                181u8,
                202u8,
                216u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    relayer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'relayer' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    sender: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'sender' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    approved: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_bool()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for RelayerApprovalChanged {
            const NAME: &'static str = "RelayerApprovalChanged";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Swap {
            pub pool_id: [u8; 32usize],
            pub token_in: Vec<u8>,
            pub token_out: Vec<u8>,
            pub amount_in: ethabi::Uint,
            pub amount_out: ethabi::Uint,
        }
        impl Swap {
            const TOPIC_ID: [u8; 32] = [
                33u8,
                112u8,
                199u8,
                65u8,
                196u8,
                21u8,
                49u8,
                174u8,
                194u8,
                14u8,
                124u8,
                16u8,
                124u8,
                36u8,
                238u8,
                207u8,
                221u8,
                21u8,
                230u8,
                156u8,
                155u8,
                176u8,
                168u8,
                221u8,
                55u8,
                177u8,
                132u8,
                11u8,
                158u8,
                11u8,
                32u8,
                123u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 4usize {
                    return false;
                }
                if log.data.len() != 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    pool_id: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pool_id' from topic of type 'bytes32': {}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    token_in: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_in' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    token_out: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[3usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'token_out' from topic of type 'address': {}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    amount_in: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                    amount_out: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR),
                })
            }
        }
        impl substreams_ethereum::Event for Swap {
            const NAME: &'static str = "Swap";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokensDeregistered {
            pub pool_id: [u8; 32usize],
            pub tokens: Vec<Vec<u8>>,
        }
        impl TokensDeregistered {
            const TOPIC_ID: [u8; 32] = [
                125u8,
                205u8,
                198u8,
                208u8,
                46u8,
                244u8,
                12u8,
                124u8,
                26u8,
                112u8,
                70u8,
                160u8,
                17u8,
                176u8,
                88u8,
                189u8,
                127u8,
                152u8,
                143u8,
                161u8,
                78u8,
                32u8,
                166u8,
                99u8,
                68u8,
                249u8,
                212u8,
                230u8,
                6u8,
                87u8,
                214u8,
                16u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    pool_id: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pool_id' from topic of type 'bytes32': {}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for TokensDeregistered {
            const NAME: &'static str = "TokensDeregistered";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct TokensRegistered {
            pub pool_id: [u8; 32usize],
            pub tokens: Vec<Vec<u8>>,
            pub asset_managers: Vec<Vec<u8>>,
        }
        impl TokensRegistered {
            const TOPIC_ID: [u8; 32] = [
                245u8,
                132u8,
                125u8,
                63u8,
                33u8,
                151u8,
                177u8,
                108u8,
                220u8,
                210u8,
                9u8,
                142u8,
                201u8,
                93u8,
                9u8,
                5u8,
                205u8,
                26u8,
                189u8,
                175u8,
                65u8,
                95u8,
                7u8,
                187u8,
                124u8,
                239u8,
                43u8,
                186u8,
                138u8,
                197u8,
                222u8,
                196u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() < 128usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::Address),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {}", e))?;
                values.reverse();
                Ok(Self {
                    pool_id: {
                        let mut result = [0u8; 32];
                        let v = ethabi::decode(
                                &[ethabi::ParamType::FixedBytes(32usize)],
                                log.topics[1usize].as_ref(),
                            )
                            .map_err(|e| {
                                format!(
                                    "unable to decode param 'pool_id' from topic of type 'bytes32': {}",
                                    e
                                )
                            })?
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    tokens: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                    asset_managers: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner.into_address().expect(INTERNAL_ERR).as_bytes().to_vec()
                        })
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for TokensRegistered {
            const NAME: &'static str = "TokensRegistered";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }