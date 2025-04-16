const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
/// Contract's functions.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod functions {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct AddOrder2 {
        pub order_config: (
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
            [u8; 32usize],
            Vec<u8>,
        ),
        pub post: Vec<(
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        )>,
    }
    impl AddOrder2 {
        const METHOD_ID: [u8; 4] = [166u8, 22u8, 134u8, 77u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                        ethabi::ParamType::FixedBytes(32usize),
                        ethabi::ParamType::Bytes,
                    ]),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                            ethabi::ParamType::Bytes,
                        ]))),
                    ]))),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                order_config: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let tuple_elements = tuple_elements[0usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[1usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[3usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                        tuple_elements[5usize]
                            .clone()
                            .into_bytes()
                            .expect(INTERNAL_ERR),
                    )
                },
                post: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut v = [0 as u8; 32];
                                                inner
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Tuple(vec![
                    ethabi::Token::Tuple(vec![
                        ethabi::Token::Address(ethabi::Address::from_slice(
                            &self.order_config.0 .0,
                        )),
                        ethabi::Token::Address(ethabi::Address::from_slice(
                            &self.order_config.0 .1,
                        )),
                        ethabi::Token::Bytes(self.order_config.0 .2.clone()),
                    ]),
                    {
                        let v = self
                            .order_config
                            .1
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    {
                        let v = self
                            .order_config
                            .2
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    ethabi::Token::FixedBytes(self.order_config.3.as_ref().to_vec()),
                    ethabi::Token::FixedBytes(self.order_config.4.as_ref().to_vec()),
                    ethabi::Token::Bytes(self.order_config.5.clone()),
                ]),
                {
                    let v = self
                        .post
                        .iter()
                        .map(|inner| {
                            ethabi::Token::Tuple(vec![
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .0,
                                    )),
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .1,
                                    )),
                                    ethabi::Token::Bytes(inner.0 .2.clone()),
                                ]),
                                {
                                    let v = inner
                                        .1
                                        .iter()
                                        .map(|inner| {
                                            ethabi::Token::Tuple(vec![
                                                ethabi::Token::Address(
                                                    ethabi::Address::from_slice(&inner.0),
                                                ),
                                                {
                                                    let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                                    ethabi::Token::Array(v)
                                                },
                                                ethabi::Token::Bytes(inner.2.clone()),
                                            ])
                                        })
                                        .collect();
                                    ethabi::Token::Array(v)
                                },
                            ])
                        })
                        .collect();
                    ethabi::Token::Array(v)
                },
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for AddOrder2 {
        const NAME: &'static str = "addOrder2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for AddOrder2 {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Clear2 {
        pub alice_order: (
            Vec<u8>,
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
        ),
        pub bob_order: (
            Vec<u8>,
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
        ),
        pub clear_config: (
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
        ),
        pub alice_signed_context: Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        pub bob_signed_context: Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
    }
    impl Clear2 {
        const METHOD_ID: [u8; 4] = [160u8, 143u8, 93u8, 255u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                    ]),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                        ethabi::ParamType::Bytes,
                    ]))),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                        ethabi::ParamType::Bytes,
                    ]))),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                alice_order: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
                bob_order: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
                clear_config: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[0usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[1usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[2usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[3usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[4usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[5usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                    )
                },
                alice_signed_context: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let mut v = [0 as u8; 32];
                                    inner
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                })
                                .collect(),
                            tuple_elements[2usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    })
                    .collect(),
                bob_signed_context: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let mut v = [0 as u8; 32];
                                    inner
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                })
                                .collect(),
                            tuple_elements[2usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    })
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Tuple(vec![
                    ethabi::Token::Address(ethabi::Address::from_slice(&self.alice_order.0)),
                    ethabi::Token::Tuple(vec![
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.alice_order.1 .0)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.alice_order.1 .1)),
                        ethabi::Token::Bytes(self.alice_order.1 .2.clone()),
                    ]),
                    {
                        let v = self
                            .alice_order
                            .2
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    {
                        let v = self
                            .alice_order
                            .3
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    ethabi::Token::FixedBytes(self.alice_order.4.as_ref().to_vec()),
                ]),
                ethabi::Token::Tuple(vec![
                    ethabi::Token::Address(ethabi::Address::from_slice(&self.bob_order.0)),
                    ethabi::Token::Tuple(vec![
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.bob_order.1 .0)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.bob_order.1 .1)),
                        ethabi::Token::Bytes(self.bob_order.1 .2.clone()),
                    ]),
                    {
                        let v = self
                            .bob_order
                            .2
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    {
                        let v = self
                            .bob_order
                            .3
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    ethabi::Token::FixedBytes(self.bob_order.4.as_ref().to_vec()),
                ]),
                ethabi::Token::Tuple(vec![
                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                        match self
                            .clear_config
                            .0
                            .clone()
                            .to_bytes_be()
                        {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    )),
                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                        match self
                            .clear_config
                            .1
                            .clone()
                            .to_bytes_be()
                        {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    )),
                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                        match self
                            .clear_config
                            .2
                            .clone()
                            .to_bytes_be()
                        {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    )),
                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                        match self
                            .clear_config
                            .3
                            .clone()
                            .to_bytes_be()
                        {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    )),
                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                        match self
                            .clear_config
                            .4
                            .clone()
                            .to_bytes_be()
                        {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    )),
                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                        match self
                            .clear_config
                            .5
                            .clone()
                            .to_bytes_be()
                        {
                            (num_bigint::Sign::Plus, bytes) => bytes,
                            (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported")
                            }
                        }
                        .as_slice(),
                    )),
                ]),
                {
                    let v =
                        self.alice_signed_context
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    {
                                        let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                        ethabi::Token::Array(v)
                                    },
                                    ethabi::Token::Bytes(inner.2.clone()),
                                ])
                            })
                            .collect();
                    ethabi::Token::Array(v)
                },
                {
                    let v =
                        self.bob_signed_context
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    {
                                        let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                        ethabi::Token::Array(v)
                                    },
                                    ethabi::Token::Bytes(inner.2.clone()),
                                ])
                            })
                            .collect();
                    ethabi::Token::Array(v)
                },
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for Clear2 {
        const NAME: &'static str = "clear2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Deposit2 {
        pub token: Vec<u8>,
        pub vault_id: substreams::scalar::BigInt,
        pub deposit_amount: substreams::scalar::BigInt,
        pub post: Vec<(
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        )>,
    }
    impl Deposit2 {
        const METHOD_ID: [u8; 4] = [145u8, 51u8, 124u8, 10u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                            ethabi::ParamType::Bytes,
                        ]))),
                    ]))),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_id: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                deposit_amount: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                post: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut v = [0 as u8; 32];
                                                inner
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.token)),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.vault_id.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self
                        .deposit_amount
                        .clone()
                        .to_bytes_be()
                    {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                {
                    let v = self
                        .post
                        .iter()
                        .map(|inner| {
                            ethabi::Token::Tuple(vec![
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .0,
                                    )),
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .1,
                                    )),
                                    ethabi::Token::Bytes(inner.0 .2.clone()),
                                ]),
                                {
                                    let v = inner
                                        .1
                                        .iter()
                                        .map(|inner| {
                                            ethabi::Token::Tuple(vec![
                                                ethabi::Token::Address(
                                                    ethabi::Address::from_slice(&inner.0),
                                                ),
                                                {
                                                    let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                                    ethabi::Token::Array(v)
                                                },
                                                ethabi::Token::Bytes(inner.2.clone()),
                                            ])
                                        })
                                        .collect();
                                    ethabi::Token::Array(v)
                                },
                            ])
                        })
                        .collect();
                    ethabi::Token::Array(v)
                },
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for Deposit2 {
        const NAME: &'static str = "deposit2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Entask {
        pub post: Vec<(
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        )>,
    }
    impl Entask {
        const METHOD_ID: [u8; 4] = [13u8, 79u8, 127u8, 113u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Bytes,
                    ]),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                        ethabi::ParamType::Bytes,
                    ]))),
                ])))],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                post: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut v = [0 as u8; 32];
                                                inner
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[{
                let v = self
                    .post
                    .iter()
                    .map(|inner| {
                        ethabi::Token::Tuple(vec![
                            ethabi::Token::Tuple(vec![
                                ethabi::Token::Address(ethabi::Address::from_slice(&inner.0 .0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(&inner.0 .1)),
                                ethabi::Token::Bytes(inner.0 .2.clone()),
                            ]),
                            {
                                let v = inner
                                    .1
                                    .iter()
                                    .map(|inner| {
                                        ethabi::Token::Tuple(vec![
                                            ethabi::Token::Address(ethabi::Address::from_slice(
                                                &inner.0,
                                            )),
                                            {
                                                let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                                ethabi::Token::Array(v)
                                            },
                                            ethabi::Token::Bytes(inner.2.clone()),
                                        ])
                                    })
                                    .collect();
                                ethabi::Token::Array(v)
                            },
                        ])
                    })
                    .collect();
                ethabi::Token::Array(v)
            }]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for Entask {
        const NAME: &'static str = "entask";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct FlashFee {
        pub param0: Vec<u8>,
        pub param1: substreams::scalar::BigInt,
    }
    impl FlashFee {
        const METHOD_ID: [u8; 4] = [217u8, 217u8, 140u8, 228u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Address, ethabi::ParamType::Uint(256usize)],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                param0: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                param1: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.param0)),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.param1.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<substreams::scalar::BigInt, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Uint(256usize)], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok({
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            })
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for FlashFee {
        const NAME: &'static str = "flashFee";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt> for FlashFee {
        fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct FlashLoan {
        pub receiver: Vec<u8>,
        pub token: Vec<u8>,
        pub amount: substreams::scalar::BigInt,
        pub data: Vec<u8>,
    }
    impl FlashLoan {
        const METHOD_ID: [u8; 4] = [92u8, 255u8, 233u8, 222u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Bytes,
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                receiver: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                amount: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                data: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.receiver)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.token)),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.amount.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                ethabi::Token::Bytes(self.data.clone()),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for FlashLoan {
        const NAME: &'static str = "flashLoan";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for FlashLoan {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct MaxFlashLoan {
        pub token: Vec<u8>,
    }
    impl MaxFlashLoan {
        const METHOD_ID: [u8; 4] = [97u8, 50u8, 85u8, 171u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(&[ethabi::ParamType::Address], maybe_data.unwrap())
                .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data =
                ethabi::encode(&[ethabi::Token::Address(ethabi::Address::from_slice(&self.token))]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<substreams::scalar::BigInt, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Uint(256usize)], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok({
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            })
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for MaxFlashLoan {
        const NAME: &'static str = "maxFlashLoan";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt> for MaxFlashLoan {
        fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Multicall {
        pub data: Vec<Vec<u8>>,
    }
    impl Multicall {
        const METHOD_ID: [u8; 4] = [172u8, 150u8, 80u8, 216u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                data: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[{
                let v = self
                    .data
                    .iter()
                    .map(|inner| ethabi::Token::Bytes(inner.clone()))
                    .collect();
                ethabi::Token::Array(v)
            }]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<Vec<Vec<u8>>, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bytes))],
                data.as_ref(),
            )
            .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_array()
                .expect(INTERNAL_ERR)
                .into_iter()
                .map(|inner| inner.into_bytes().expect(INTERNAL_ERR))
                .collect())
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<Vec<Vec<u8>>> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Multicall {
        const NAME: &'static str = "multicall";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<Vec<Vec<u8>>> for Multicall {
        fn output(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderExists {
        pub order_hash: [u8; 32usize],
    }
    impl OrderExists {
        const METHOD_ID: [u8; 4] = [44u8, 183u8, 126u8, 159u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values =
                ethabi::decode(&[ethabi::ParamType::FixedBytes(32usize)], maybe_data.unwrap())
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                order_hash: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data =
                ethabi::encode(&[ethabi::Token::FixedBytes(self.order_hash.as_ref().to_vec())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for OrderExists {
        const NAME: &'static str = "orderExists";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for OrderExists {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Quote {
        pub quote_config: (
            (
                Vec<u8>,
                (Vec<u8>, Vec<u8>, Vec<u8>),
                Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                [u8; 32usize],
            ),
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        ),
    }
    impl Quote {
        const METHOD_ID: [u8; 4] = [224u8, 229u8, 48u8, 183u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Tuple(vec![
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                        ethabi::ParamType::Bytes,
                    ]))),
                ])],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                quote_config: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let tuple_elements = tuple_elements[0usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let tuple_elements = tuple_elements[1usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                },
                                tuple_elements[2usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements =
                                            inner.into_tuple().expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                        )
                                    })
                                    .collect(),
                                tuple_elements[3usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements =
                                            inner.into_tuple().expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                        )
                                    })
                                    .collect(),
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[4usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[1usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[2usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut v = [0 as u8; 32];
                                            inner
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        })
                                        .collect(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            })
                            .collect(),
                    )
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Tuple(vec![
                ethabi::Token::Tuple(vec![
                    ethabi::Token::Address(ethabi::Address::from_slice(&self.quote_config.0 .0)),
                    ethabi::Token::Tuple(vec![
                        ethabi::Token::Address(ethabi::Address::from_slice(
                            &self.quote_config.0 .1 .0,
                        )),
                        ethabi::Token::Address(ethabi::Address::from_slice(
                            &self.quote_config.0 .1 .1,
                        )),
                        ethabi::Token::Bytes(self.quote_config.0 .1 .2.clone()),
                    ]),
                    {
                        let v = self
                            .quote_config
                            .0
                             .2
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    {
                        let v = self
                            .quote_config
                            .0
                             .3
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    ethabi::Token::FixedBytes(self.quote_config.0 .4.as_ref().to_vec()),
                ]),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self
                        .quote_config
                        .1
                        .clone()
                        .to_bytes_be()
                    {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self
                        .quote_config
                        .2
                        .clone()
                        .to_bytes_be()
                    {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                {
                    let v =
                        self.quote_config
                            .3
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    {
                                        let v = inner.1.iter().map(| inner |
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),)).collect();
                                        ethabi::Token::Array(v)
                                    },
                                    ethabi::Token::Bytes(inner.2.clone()),
                                ])
                            })
                            .collect();
                    ethabi::Token::Array(v)
                },
            ])]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<(bool, substreams::scalar::BigInt, substreams::scalar::BigInt), String>
        {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(
            data: &[u8],
        ) -> Result<(bool, substreams::scalar::BigInt, substreams::scalar::BigInt), String>
        {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Bool,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                data.as_ref(),
            )
            .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            values.reverse();
            Ok((
                values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bool()
                    .expect(INTERNAL_ERR),
                {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            ))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(
            &self,
            address: Vec<u8>,
        ) -> Option<(bool, substreams::scalar::BigInt, substreams::scalar::BigInt)> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for Quote {
        const NAME: &'static str = "quote";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl
        substreams_ethereum::rpc::RPCDecodable<(
            bool,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
        )> for Quote
    {
        fn output(
            data: &[u8],
        ) -> Result<(bool, substreams::scalar::BigInt, substreams::scalar::BigInt), String>
        {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct RemoveOrder2 {
        pub order: (
            Vec<u8>,
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
        ),
        pub post: Vec<(
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        )>,
    }
    impl RemoveOrder2 {
        const METHOD_ID: [u8; 4] = [141u8, 123u8, 107u8, 235u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                            ethabi::ParamType::Bytes,
                        ]))),
                    ]))),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                order: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
                post: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut v = [0 as u8; 32];
                                                inner
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Tuple(vec![
                    ethabi::Token::Address(ethabi::Address::from_slice(&self.order.0)),
                    ethabi::Token::Tuple(vec![
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.order.1 .0)),
                        ethabi::Token::Address(ethabi::Address::from_slice(&self.order.1 .1)),
                        ethabi::Token::Bytes(self.order.1 .2.clone()),
                    ]),
                    {
                        let v = self
                            .order
                            .2
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    {
                        let v = self
                            .order
                            .3
                            .iter()
                            .map(|inner| {
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(&inner.0)),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.1.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                                        match inner.2.clone().to_bytes_be() {
                                            (num_bigint::Sign::Plus, bytes) => bytes,
                                            (num_bigint::Sign::NoSign, bytes) => bytes,
                                            (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported")
                                            }
                                        }
                                        .as_slice(),
                                    )),
                                ])
                            })
                            .collect();
                        ethabi::Token::Array(v)
                    },
                    ethabi::Token::FixedBytes(self.order.4.as_ref().to_vec()),
                ]),
                {
                    let v = self
                        .post
                        .iter()
                        .map(|inner| {
                            ethabi::Token::Tuple(vec![
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .0,
                                    )),
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .1,
                                    )),
                                    ethabi::Token::Bytes(inner.0 .2.clone()),
                                ]),
                                {
                                    let v = inner
                                        .1
                                        .iter()
                                        .map(|inner| {
                                            ethabi::Token::Tuple(vec![
                                                ethabi::Token::Address(
                                                    ethabi::Address::from_slice(&inner.0),
                                                ),
                                                {
                                                    let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                                    ethabi::Token::Array(v)
                                                },
                                                ethabi::Token::Bytes(inner.2.clone()),
                                            ])
                                        })
                                        .collect();
                                    ethabi::Token::Array(v)
                                },
                            ])
                        })
                        .collect();
                    ethabi::Token::Array(v)
                },
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for RemoveOrder2 {
        const NAME: &'static str = "removeOrder2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for RemoveOrder2 {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct SupportsInterface {
        pub interface_id: [u8; 4usize],
    }
    impl SupportsInterface {
        const METHOD_ID: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values =
                ethabi::decode(&[ethabi::ParamType::FixedBytes(4usize)], maybe_data.unwrap())
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                interface_id: {
                    let mut result = [0u8; 4];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data =
                ethabi::encode(&[ethabi::Token::FixedBytes(self.interface_id.as_ref().to_vec())]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<bool, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<bool, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Bool], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok(values
                .pop()
                .expect("one output data should have existed")
                .into_bool()
                .expect(INTERNAL_ERR))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<bool> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for SupportsInterface {
        const NAME: &'static str = "supportsInterface";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<bool> for SupportsInterface {
        fn output(data: &[u8]) -> Result<bool, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct TakeOrders2 {
        pub config: (
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            Vec<(
                (
                    Vec<u8>,
                    (Vec<u8>, Vec<u8>, Vec<u8>),
                    Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                    Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                    [u8; 32usize],
                ),
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
            )>,
            Vec<u8>,
        ),
    }
    impl TakeOrders2 {
        const METHOD_ID: [u8; 4] = [9u8, 151u8, 196u8, 160u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Tuple(vec![
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Tuple(vec![
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Bytes,
                            ]),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Uint(8usize),
                                ethabi::ParamType::Uint(256usize),
                            ]))),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Uint(8usize),
                                ethabi::ParamType::Uint(256usize),
                            ]))),
                            ethabi::ParamType::FixedBytes(32usize),
                        ]),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                            ethabi::ParamType::Bytes,
                        ]))),
                    ]))),
                    ethabi::ParamType::Bytes,
                ])],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                config: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[0usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[1usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[2usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    {
                                        let tuple_elements = tuple_elements[0usize]
                                            .clone()
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let tuple_elements = tuple_elements[1usize]
                                                    .clone()
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_bytes()
                                                        .expect(INTERNAL_ERR),
                                                )
                                            },
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_array()
                                                .expect(INTERNAL_ERR)
                                                .into_iter()
                                                .map(|inner| {
                                                    let tuple_elements = inner
                                                        .into_tuple()
                                                        .expect(INTERNAL_ERR);
                                                    (
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec(),
                                                        {
                                                            let mut v = [0 as u8; 32];
                                                            tuple_elements[1usize]
                                                                .clone()
                                                                .into_uint()
                                                                .expect(INTERNAL_ERR)
                                                                .to_big_endian(v.as_mut_slice());
                                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                        },
                                                        {
                                                            let mut v = [0 as u8; 32];
                                                            tuple_elements[2usize]
                                                                .clone()
                                                                .into_uint()
                                                                .expect(INTERNAL_ERR)
                                                                .to_big_endian(v.as_mut_slice());
                                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                        },
                                                    )
                                                })
                                                .collect(),
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_array()
                                                .expect(INTERNAL_ERR)
                                                .into_iter()
                                                .map(|inner| {
                                                    let tuple_elements = inner
                                                        .into_tuple()
                                                        .expect(INTERNAL_ERR);
                                                    (
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_address()
                                                            .expect(INTERNAL_ERR)
                                                            .as_bytes()
                                                            .to_vec(),
                                                        {
                                                            let mut v = [0 as u8; 32];
                                                            tuple_elements[1usize]
                                                                .clone()
                                                                .into_uint()
                                                                .expect(INTERNAL_ERR)
                                                                .to_big_endian(v.as_mut_slice());
                                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                        },
                                                        {
                                                            let mut v = [0 as u8; 32];
                                                            tuple_elements[2usize]
                                                                .clone()
                                                                .into_uint()
                                                                .expect(INTERNAL_ERR)
                                                                .to_big_endian(v.as_mut_slice());
                                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                        },
                                                    )
                                                })
                                                .collect(),
                                            {
                                                let mut result = [0u8; 32];
                                                let v = tuple_elements[4usize]
                                                    .clone()
                                                    .into_fixed_bytes()
                                                    .expect(INTERNAL_ERR);
                                                result.copy_from_slice(&v);
                                                result
                                            },
                                        )
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_array()
                                                    .expect(INTERNAL_ERR)
                                                    .into_iter()
                                                    .map(|inner| {
                                                        let mut v = [0 as u8; 32];
                                                        inner
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    })
                                                    .collect(),
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_bytes()
                                                    .expect(INTERNAL_ERR),
                                            )
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                        tuple_elements[4usize].clone().into_bytes().expect(INTERNAL_ERR),
                    )
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[ethabi::Token::Tuple(vec![
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.config.0.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.config.1.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.config.2.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                {
                    let v = self.config.3.iter().map(| inner |
                            ethabi::Token::Tuple(vec![ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                            inner.0.0)),
                            ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                            inner.0.1.0)),
                            ethabi::Token::Address(ethabi::Address::from_slice(& inner.0
                            .1.1)), ethabi::Token::Bytes(inner.0.1.2.clone())]), { let v
                            = inner.0.2.iter().map(| inner |
                            ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                            inner.0)),
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .1.clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),),
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .2.clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                            let v = inner.0.3.iter().map(| inner |
                            ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                            inner.0)),
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .1.clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),),
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .2.clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),)])).collect(); ethabi::Token::Array(v) },
                            ethabi::Token::FixedBytes(inner.0.4.as_ref().to_vec())]),
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .1.clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),),
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .2.clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),), { let v = inner.3.iter().map(| inner |
                            ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                            inner.0)), { let v = inner.1.iter().map(| inner |
                            ethabi::Token::Uint(ethabi::Uint::from_big_endian(match inner
                            .clone().to_bytes_be() { (num_bigint::Sign::Plus, bytes) =>
                            bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                            (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported") }, }
                            .as_slice(),),)).collect(); ethabi::Token::Array(v) },
                            ethabi::Token::Bytes(inner.2.clone())])).collect();
                            ethabi::Token::Array(v) }])).collect();
                    ethabi::Token::Array(v)
                },
                ethabi::Token::Bytes(self.config.4.clone()),
            ])]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<(substreams::scalar::BigInt, substreams::scalar::BigInt), String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(
            data: &[u8],
        ) -> Result<(substreams::scalar::BigInt, substreams::scalar::BigInt), String> {
            let mut values = ethabi::decode(
                &[ethabi::ParamType::Uint(256usize), ethabi::ParamType::Uint(256usize)],
                data.as_ref(),
            )
            .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            values.reverse();
            Ok((
                {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            ))
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(
            &self,
            address: Vec<u8>,
        ) -> Option<(substreams::scalar::BigInt, substreams::scalar::BigInt)> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for TakeOrders2 {
        const NAME: &'static str = "takeOrders2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl
        substreams_ethereum::rpc::RPCDecodable<(
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
        )> for TakeOrders2
    {
        fn output(
            data: &[u8],
        ) -> Result<(substreams::scalar::BigInt, substreams::scalar::BigInt), String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct VaultBalance {
        pub owner: Vec<u8>,
        pub token: Vec<u8>,
        pub vault_id: substreams::scalar::BigInt,
    }
    impl VaultBalance {
        const METHOD_ID: [u8; 4] = [217u8, 123u8, 46u8, 72u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_id: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.owner)),
                ethabi::Token::Address(ethabi::Address::from_slice(&self.token)),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.vault_id.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn output_call(
            call: &substreams_ethereum::pb::eth::v2::Call,
        ) -> Result<substreams::scalar::BigInt, String> {
            Self::output(call.return_data.as_ref())
        }
        pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            let mut values = ethabi::decode(&[ethabi::ParamType::Uint(256usize)], data.as_ref())
                .map_err(|e| format!("unable to decode output data: {:?}", e))?;
            Ok({
                let mut v = [0 as u8; 32];
                values
                    .pop()
                    .expect("one output data should have existed")
                    .into_uint()
                    .expect(INTERNAL_ERR)
                    .to_big_endian(v.as_mut_slice());
                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
            })
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
        pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
            use substreams_ethereum::pb::eth::rpc;
            let rpc_calls = rpc::RpcCalls {
                calls: vec![rpc::RpcCall { to_addr: address, data: self.encode() }],
            };
            let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
            let response = responses
                .get(0)
                .expect("one response should have existed");
            if response.failed {
                return None;
            }
            match Self::output(response.raw.as_ref()) {
                Ok(data) => Some(data),
                Err(err) => {
                    use substreams_ethereum::Function;
                    substreams::log::info!(
                        "Call output for function `{}` failed to decode with error: {}",
                        Self::NAME,
                        err
                    );
                    None
                }
            }
        }
    }
    impl substreams_ethereum::Function for VaultBalance {
        const NAME: &'static str = "vaultBalance";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
    impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt> for VaultBalance {
        fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
            Self::output(data)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Withdraw2 {
        pub token: Vec<u8>,
        pub vault_id: substreams::scalar::BigInt,
        pub target_amount: substreams::scalar::BigInt,
        pub post: Vec<(
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        )>,
    }
    impl Withdraw2 {
        const METHOD_ID: [u8; 4] = [245u8, 19u8, 196u8, 45u8];
        pub fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            let maybe_data = call.input.get(4..);
            if maybe_data.is_none() {
                return Err("no data to decode".to_string());
            }
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                            ethabi::ParamType::Bytes,
                        ]))),
                    ]))),
                ],
                maybe_data.unwrap(),
            )
            .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
            values.reverse();
            Ok(Self {
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_id: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                target_amount: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                post: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            },
                            tuple_elements[1usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let mut v = [0 as u8; 32];
                                                inner
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                })
                                .collect(),
                        )
                    })
                    .collect(),
            })
        }
        pub fn encode(&self) -> Vec<u8> {
            let data = ethabi::encode(&[
                ethabi::Token::Address(ethabi::Address::from_slice(&self.token)),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.vault_id.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                ethabi::Token::Uint(ethabi::Uint::from_big_endian(
                    match self.target_amount.clone().to_bytes_be() {
                        (num_bigint::Sign::Plus, bytes) => bytes,
                        (num_bigint::Sign::NoSign, bytes) => bytes,
                        (num_bigint::Sign::Minus, _) => {
                            panic!("negative numbers are not supported")
                        }
                    }
                    .as_slice(),
                )),
                {
                    let v = self
                        .post
                        .iter()
                        .map(|inner| {
                            ethabi::Token::Tuple(vec![
                                ethabi::Token::Tuple(vec![
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .0,
                                    )),
                                    ethabi::Token::Address(ethabi::Address::from_slice(
                                        &inner.0 .1,
                                    )),
                                    ethabi::Token::Bytes(inner.0 .2.clone()),
                                ]),
                                {
                                    let v = inner
                                        .1
                                        .iter()
                                        .map(|inner| {
                                            ethabi::Token::Tuple(vec![
                                                ethabi::Token::Address(
                                                    ethabi::Address::from_slice(&inner.0),
                                                ),
                                                {
                                                    let v = inner.1.iter().map(| inner |
                                    ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                    inner.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                    bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                    panic!("negative numbers are not supported") }, }
                                    .as_slice(),),)).collect();
                                                    ethabi::Token::Array(v)
                                                },
                                                ethabi::Token::Bytes(inner.2.clone()),
                                            ])
                                        })
                                        .collect();
                                    ethabi::Token::Array(v)
                                },
                            ])
                        })
                        .collect();
                    ethabi::Token::Array(v)
                },
            ]);
            let mut encoded = Vec::with_capacity(4 + data.len());
            encoded.extend(Self::METHOD_ID);
            encoded.extend(data);
            encoded
        }
        pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            match call.input.get(0..4) {
                Some(signature) => Self::METHOD_ID == signature,
                None => false,
            }
        }
    }
    impl substreams_ethereum::Function for Withdraw2 {
        const NAME: &'static str = "withdraw2";
        fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
            Self::match_call(call)
        }
        fn decode(call: &substreams_ethereum::pb::eth::v2::Call) -> Result<Self, String> {
            Self::decode(call)
        }
        fn encode(&self) -> Vec<u8> {
            self.encode()
        }
    }
}
/// Contract's events.
#[allow(dead_code, unused_imports, unused_variables)]
pub mod events {
    use super::INTERNAL_ERR;
    #[derive(Debug, Clone, PartialEq)]
    pub struct AddOrderV2 {
        pub sender: Vec<u8>,
        pub order_hash: [u8; 32usize],
        pub order: (
            // `address owner`
            Vec<u8>,

            // `EvaluableV3 evaluable`
            (
                // - `IInterpreterV3 interpreter`
                Vec<u8>,
                // `IInterpreterStoreV2 store`
                Vec<u8>,
                // `bytes bytecode`
                Vec<u8>,
            ),

            // `IO[] validInputs`
            Vec<(
                // `address token`
                Vec<u8>,
                // `uint8 decimals`
                substreams::scalar::BigInt,
                // `uint256 vaultId`
                substreams::scalar::BigInt,
            )>,

            // `IO[] validOutputs`
            Vec<(
                // `address token`
                Vec<u8>,
                // `uint8 decimals`
                substreams::scalar::BigInt,
                // `uint256 vaultId`
                substreams::scalar::BigInt,
            )>,
            // `bytes32 nonce`
            [u8; 32usize],
        ),
    }

    impl AddOrderV2 {
        const TOPIC_ID: [u8; 32] = [
            60u8, 232u8, 187u8, 230u8, 82u8, 216u8, 119u8, 139u8, 103u8, 221u8, 86u8, 92u8, 71u8,
            156u8, 59u8, 186u8, 168u8, 67u8, 229u8, 94u8, 71u8, 125u8, 7u8, 95u8, 117u8, 111u8,
            136u8, 158u8, 196u8, 187u8, 218u8, 238u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 384usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                order_hash: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                order: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
            })
        }
    }
    impl substreams_ethereum::Event for AddOrderV2 {
        const NAME: &'static str = "AddOrderV2";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct AfterClear {
        pub sender: Vec<u8>,
        pub clear_state_change: (
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
        ),
    }
    impl AfterClear {
        const TOPIC_ID: [u8; 32] = [
            63u8, 32u8, 229u8, 89u8, 25u8, 204u8, 167u8, 1u8, 171u8, 178u8, 164u8, 10u8, 183u8,
            37u8, 66u8, 178u8, 94u8, 167u8, 238u8, 214u8, 58u8, 80u8, 249u8, 121u8, 221u8, 44u8,
            211u8, 35u8, 30u8, 95u8, 72u8, 141u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 160usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                    ]),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                clear_state_change: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[0usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[1usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[2usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[3usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                    )
                },
            })
        }
    }
    impl substreams_ethereum::Event for AfterClear {
        const NAME: &'static str = "AfterClear";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct ClearV2 {
        pub sender: Vec<u8>,
        pub alice: (
            Vec<u8>,
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
        ),
        pub bob: (
            Vec<u8>,
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
        ),
        pub clear_config: (
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
        ),
    }
    impl ClearV2 {
        const TOPIC_ID: [u8; 32] = [
            105u8, 46u8, 230u8, 154u8, 242u8, 132u8, 60u8, 112u8, 119u8, 43u8, 233u8, 52u8, 112u8,
            102u8, 159u8, 60u8, 115u8, 229u8, 131u8, 49u8, 106u8, 4u8, 123u8, 107u8, 254u8, 253u8,
            174u8, 86u8, 41u8, 222u8, 55u8, 206u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 864usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                    ]),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                alice: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
                bob: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
                clear_config: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[0usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[1usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[2usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[3usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[4usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[5usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                    )
                },
            })
        }
    }
    impl substreams_ethereum::Event for ClearV2 {
        const NAME: &'static str = "ClearV2";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Context {
        pub sender: Vec<u8>,
        pub context: Vec<Vec<substreams::scalar::BigInt>>,
    }
    impl Context {
        const TOPIC_ID: [u8; 32] = [
            23u8, 165u8, 192u8, 243u8, 120u8, 81u8, 50u8, 165u8, 119u8, 3u8, 147u8, 32u8, 50u8,
            246u8, 134u8, 62u8, 121u8, 32u8, 67u8, 65u8, 80u8, 170u8, 29u8, 201u8, 64u8, 229u8,
            103u8, 180u8, 64u8, 253u8, 206u8, 31u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 96usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Array(Box::new(
                        ethabi::ParamType::Uint(256usize),
                    )))),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                context: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_array()
                    .expect(INTERNAL_ERR)
                    .into_iter()
                    .map(|inner| {
                        inner
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let mut v = [0 as u8; 32];
                                inner
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            })
                            .collect()
                    })
                    .collect(),
            })
        }
    }
    impl substreams_ethereum::Event for Context {
        const NAME: &'static str = "Context";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Deposit {
        pub sender: Vec<u8>,
        pub token: Vec<u8>,
        pub vault_id: substreams::scalar::BigInt,
        pub amount: substreams::scalar::BigInt,
    }
    impl Deposit {
        const TOPIC_ID: [u8; 32] = [
            220u8, 188u8, 28u8, 5u8, 36u8, 15u8, 49u8, 255u8, 58u8, 208u8, 103u8, 239u8, 30u8,
            227u8, 92u8, 228u8, 153u8, 119u8, 98u8, 117u8, 46u8, 58u8, 9u8, 82u8, 132u8, 117u8,
            69u8, 68u8, 244u8, 199u8, 9u8, 215u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 128usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_id: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                amount: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }
    impl substreams_ethereum::Event for Deposit {
        const NAME: &'static str = "Deposit";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct MetaV12 {
        pub sender: Vec<u8>,
        pub subject: [u8; 32usize],
        pub meta: Vec<u8>,
    }
    impl MetaV12 {
        const TOPIC_ID: [u8; 32] = [
            212u8, 108u8, 44u8, 86u8, 179u8, 92u8, 130u8, 16u8, 233u8, 231u8, 18u8, 236u8, 63u8,
            2u8, 36u8, 45u8, 95u8, 201u8, 1u8, 135u8, 192u8, 172u8, 142u8, 216u8, 12u8, 195u8,
            54u8, 38u8, 132u8, 110u8, 195u8, 105u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 128usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Bytes,
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                subject: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                meta: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_bytes()
                    .expect(INTERNAL_ERR),
            })
        }
    }
    impl substreams_ethereum::Event for MetaV12 {
        const NAME: &'static str = "MetaV1_2";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderExceedsMaxRatio {
        pub sender: Vec<u8>,
        pub owner: Vec<u8>,
        pub order_hash: [u8; 32usize],
    }
    impl OrderExceedsMaxRatio {
        const TOPIC_ID: [u8; 32] = [
            227u8, 21u8, 29u8, 200u8, 203u8, 122u8, 84u8, 255u8, 196u8, 186u8, 171u8, 210u8, 140u8,
            31u8, 36u8, 28u8, 148u8, 213u8, 16u8, 181u8, 229u8, 181u8, 2u8, 73u8, 26u8, 195u8,
            202u8, 214u8, 193u8, 99u8, 22u8, 213u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 96usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::FixedBytes(32usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                order_hash: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
    }
    impl substreams_ethereum::Event for OrderExceedsMaxRatio {
        const NAME: &'static str = "OrderExceedsMaxRatio";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderNotFound {
        pub sender: Vec<u8>,
        pub owner: Vec<u8>,
        pub order_hash: [u8; 32usize],
    }
    impl OrderNotFound {
        const TOPIC_ID: [u8; 32] = [
            183u8, 12u8, 18u8, 250u8, 69u8, 55u8, 147u8, 250u8, 104u8, 24u8, 236u8, 7u8, 201u8,
            30u8, 116u8, 54u8, 58u8, 71u8, 170u8, 106u8, 104u8, 41u8, 220u8, 217u8, 83u8, 57u8,
            55u8, 253u8, 243u8, 3u8, 20u8, 243u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 96usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::FixedBytes(32usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                order_hash: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
    }
    impl substreams_ethereum::Event for OrderNotFound {
        const NAME: &'static str = "OrderNotFound";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct OrderZeroAmount {
        pub sender: Vec<u8>,
        pub owner: Vec<u8>,
        pub order_hash: [u8; 32usize],
    }
    impl OrderZeroAmount {
        const TOPIC_ID: [u8; 32] = [
            80u8, 11u8, 113u8, 56u8, 87u8, 50u8, 95u8, 158u8, 109u8, 203u8, 82u8, 174u8, 131u8,
            46u8, 202u8, 145u8, 9u8, 209u8, 7u8, 237u8, 26u8, 174u8, 156u8, 180u8, 146u8, 139u8,
            76u8, 30u8, 19u8, 240u8, 81u8, 170u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 96usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::FixedBytes(32usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                owner: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                order_hash: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
            })
        }
    }
    impl substreams_ethereum::Event for OrderZeroAmount {
        const NAME: &'static str = "OrderZeroAmount";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct RemoveOrderV2 {
        pub sender: Vec<u8>,
        pub order_hash: [u8; 32usize],
        pub order: (
            Vec<u8>,
            (Vec<u8>, Vec<u8>, Vec<u8>),
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            [u8; 32usize],
        ),
    }
    impl RemoveOrderV2 {
        const TOPIC_ID: [u8; 32] = [
            162u8, 215u8, 169u8, 106u8, 254u8, 119u8, 201u8, 155u8, 106u8, 45u8, 114u8, 202u8,
            139u8, 77u8, 60u8, 92u8, 136u8, 182u8, 70u8, 110u8, 227u8, 76u8, 167u8, 78u8, 9u8,
            85u8, 149u8, 31u8, 68u8, 148u8, 37u8, 200u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 384usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::FixedBytes(32usize),
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Address,
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Bytes,
                        ]),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(8usize),
                            ethabi::ParamType::Uint(256usize),
                        ]))),
                        ethabi::ParamType::FixedBytes(32usize),
                    ]),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                order_hash: {
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                },
                order: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        tuple_elements[0usize]
                            .clone()
                            .into_address()
                            .expect(INTERNAL_ERR)
                            .as_bytes()
                            .to_vec(),
                        {
                            let tuple_elements = tuple_elements[1usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        },
                        tuple_elements[2usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            })
                            .collect(),
                        {
                            let mut result = [0u8; 32];
                            let v = tuple_elements[4usize]
                                .clone()
                                .into_fixed_bytes()
                                .expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        },
                    )
                },
            })
        }
    }
    impl substreams_ethereum::Event for RemoveOrderV2 {
        const NAME: &'static str = "RemoveOrderV2";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct TakeOrderV2 {
        pub sender: Vec<u8>,
        pub config: (
            (
                Vec<u8>,
                (Vec<u8>, Vec<u8>, Vec<u8>),
                Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                Vec<(Vec<u8>, substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                [u8; 32usize],
            ),
            substreams::scalar::BigInt,
            substreams::scalar::BigInt,
            Vec<(Vec<u8>, Vec<substreams::scalar::BigInt>, Vec<u8>)>,
        ),
        pub input: substreams::scalar::BigInt,
        pub output: substreams::scalar::BigInt,
    }
    impl TakeOrderV2 {
        const TOPIC_ID: [u8; 32] = [
            16u8, 222u8, 153u8, 185u8, 3u8, 33u8, 132u8, 88u8, 117u8, 64u8, 192u8, 78u8, 24u8,
            80u8, 223u8, 249u8, 74u8, 118u8, 131u8, 174u8, 31u8, 204u8, 158u8, 176u8, 96u8, 152u8,
            98u8, 28u8, 115u8, 156u8, 2u8, 117u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() < 544usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Tuple(vec![
                        ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Tuple(vec![
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Bytes,
                            ]),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Uint(8usize),
                                ethabi::ParamType::Uint(256usize),
                            ]))),
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                                ethabi::ParamType::Address,
                                ethabi::ParamType::Uint(8usize),
                                ethabi::ParamType::Uint(256usize),
                            ]))),
                            ethabi::ParamType::FixedBytes(32usize),
                        ]),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Uint(256usize),
                        ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Uint(256usize))),
                            ethabi::ParamType::Bytes,
                        ]))),
                    ]),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                config: {
                    let tuple_elements = values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_tuple()
                        .expect(INTERNAL_ERR);
                    (
                        {
                            let tuple_elements = tuple_elements[0usize]
                                .clone()
                                .into_tuple()
                                .expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let tuple_elements = tuple_elements[1usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_bytes()
                                            .expect(INTERNAL_ERR),
                                    )
                                },
                                tuple_elements[2usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements =
                                            inner.into_tuple().expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                        )
                                    })
                                    .collect(),
                                tuple_elements[3usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements =
                                            inner.into_tuple().expect(INTERNAL_ERR);
                                        (
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(
                                                    &v,
                                                )
                                            },
                                        )
                                    })
                                    .collect(),
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[4usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[1usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        {
                            let mut v = [0 as u8; 32];
                            tuple_elements[2usize]
                                .clone()
                                .into_uint()
                                .expect(INTERNAL_ERR)
                                .to_big_endian(v.as_mut_slice());
                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                        },
                        tuple_elements[3usize]
                            .clone()
                            .into_array()
                            .expect(INTERNAL_ERR)
                            .into_iter()
                            .map(|inner| {
                                let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let mut v = [0 as u8; 32];
                                            inner
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        })
                                        .collect(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_bytes()
                                        .expect(INTERNAL_ERR),
                                )
                            })
                            .collect(),
                    )
                },
                input: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                output: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }
    impl substreams_ethereum::Event for TakeOrderV2 {
        const NAME: &'static str = "TakeOrderV2";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
    #[derive(Debug, Clone, PartialEq)]
    pub struct Withdraw {
        pub sender: Vec<u8>,
        pub token: Vec<u8>,
        pub vault_id: substreams::scalar::BigInt,
        pub target_amount: substreams::scalar::BigInt,
        pub amount: substreams::scalar::BigInt,
    }
    impl Withdraw {
        const TOPIC_ID: [u8; 32] = [
            235u8, 255u8, 38u8, 2u8, 179u8, 244u8, 104u8, 37u8, 158u8, 30u8, 153u8, 246u8, 19u8,
            254u8, 214u8, 105u8, 31u8, 58u8, 101u8, 38u8, 239u8, 254u8, 110u8, 243u8, 231u8, 104u8,
            186u8, 122u8, 231u8, 163u8, 108u8, 79u8,
        ];
        pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            if log.topics.len() != 1usize {
                return false;
            }
            if log.data.len() != 160usize {
                return false;
            }
            return log
                .topics
                .get(0)
                .expect("bounds already checked")
                .as_ref()
                == Self::TOPIC_ID;
        }
        pub fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            let mut values = ethabi::decode(
                &[
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Address,
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                    ethabi::ParamType::Uint(256usize),
                ],
                log.data.as_ref(),
            )
            .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
            values.reverse();
            Ok(Self {
                sender: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                token: values
                    .pop()
                    .expect(INTERNAL_ERR)
                    .into_address()
                    .expect(INTERNAL_ERR)
                    .as_bytes()
                    .to_vec(),
                vault_id: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                target_amount: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
                amount: {
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                },
            })
        }
    }
    impl substreams_ethereum::Event for Withdraw {
        const NAME: &'static str = "Withdraw";
        fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
            Self::match_log(log)
        }
        fn decode(log: &substreams_ethereum::pb::eth::v2::Log) -> Result<Self, String> {
            Self::decode(log)
        }
    }
}
