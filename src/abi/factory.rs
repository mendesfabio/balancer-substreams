    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's events.
    #[allow(dead_code)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct FactoryDisabled {}
        impl FactoryDisabled {
            const TOPIC_ID: [u8; 32] = [
                67u8,
                42u8,
                203u8,
                253u8,
                102u8,
                45u8,
                187u8,
                93u8,
                139u8,
                55u8,
                131u8,
                132u8,
                166u8,
                113u8,
                89u8,
                180u8,
                124u8,
                169u8,
                208u8,
                241u8,
                183u8,
                159u8,
                151u8,
                207u8,
                100u8,
                207u8,
                133u8,
                133u8,
                250u8,
                54u8,
                45u8,
                80u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
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
                Ok(Self {})
            }
        }
        impl substreams_ethereum::Event for FactoryDisabled {
            const NAME: &'static str = "FactoryDisabled";
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
        pub struct PoolCreated {
            pub pool: Vec<u8>,
        }
        impl PoolCreated {
            const TOPIC_ID: [u8; 32] = [
                131u8,
                164u8,
                143u8,
                188u8,
                252u8,
                153u8,
                19u8,
                53u8,
                49u8,
                78u8,
                116u8,
                208u8,
                73u8,
                106u8,
                171u8,
                106u8,
                25u8,
                135u8,
                233u8,
                146u8,
                221u8,
                200u8,
                93u8,
                221u8,
                188u8,
                196u8,
                214u8,
                221u8,
                110u8,
                242u8,
                233u8,
                252u8,
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
                    pool: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'pool' from topic of type 'address': {}",
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
        impl substreams_ethereum::Event for PoolCreated {
            const NAME: &'static str = "PoolCreated";
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