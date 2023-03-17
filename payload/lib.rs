#![cfg_attr(not(feature = "std"), no_std)]

pub mod message_protocol;
pub mod message_define;

pub use self::payload::{
    Payload as Other,
    PayloadRef,
};

/// This is an example that shows how can user-defined struct be used for other contracts as parameter in `message` interface
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct TestData{
    pub n: u128,
    pub s: ink::prelude::string::String,
}

impl ::scale_info::TypeInfo for TestData{
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
            .path(::scale_info::Path::new("TestData", module_path!()))
            .composite(::scale_info::build::Fields::named()
                .field(|f| f.ty::<u128>().name("n").type_name("u128"))
                .field(|f| f.ty::<ink::prelude::string::String>().name("s").type_name("ink::prelude::string::String"))
            )
    }
}

#[ink::contract]
mod payload {

    // use ink::storage::traits::{SpreadAllocate};
    use super::message_protocol::{MsgDetail, InMsgType};

    /// for test
    #[derive(Debug, PartialEq, Clone, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub struct UserMessage{
        name: ink::prelude::string::String,
        age: u32,
        phones: ink::prelude::vec::Vec<ink::prelude::string::String>,
    }

    /// This is an example to impl `payload::message_protocol::InMsgType` for a user defined struct, 
    /// such that `MessageDetail` can be read directly through `payload::message_protocol::MessageItem::in_to::<MessageDetail>()`
    impl InMsgType for UserMessage {
        type MyType = UserMessage;
        fn get_value(type_value: & MsgDetail) -> Option<Self::MyType> {
            if let MsgDetail::UserData(val) = type_value.clone() {
                let mut v_ref = val.as_slice();
                Some(scale::Decode::decode(&mut v_ref).unwrap())
            } else {
                None
            }
        }

        /// items from traits can only be used if the trait is in scope
        fn create_message(msg_detail: Self::MyType) -> MsgDetail {
            let mut v = ink::prelude::vec::Vec::new();
            scale::Encode::encode_to(&msg_detail, &mut v);
            
            MsgDetail::UserData(v)
        }

        fn into_raw_data(self) -> ink::prelude::vec::Vec<u8> {
            ink::prelude::vec![]
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    // #[derive( ::scale_info::TypeInfo)]
    pub struct Payload {
        /// Stores a single `bool` value on the storage.
        value: bool,
        info: Option<ink::prelude::string::String>,
        // items: ink_storage::Mapping<ink::prelude::string::String, MessagePayload>,
        // mp: ink_storage::Mapping<u8, MessagePayload>,
        // msg: MessageDetail,
    }

    impl Payload {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self {
                value: init_value,
                info: None,
            }
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// encode `UserMessage`
        #[ink(message)]
        pub fn encode_um(&self, msg: UserMessage) -> ink::prelude::vec::Vec<u8> {
            let mut v = ink::prelude::vec::Vec::new();
            scale::Encode::encode_to(&msg, &mut v);

            v
        }

        #[ink(message)]
        pub fn get_inkaddressdata(&self, msg: super::message_protocol::InkAddressData) ->super::message_protocol::InkAddressData {
            msg
        }

        /// Test the message Type.
        #[ink(message)]
        pub fn get_message(&self, msg: super::message_protocol::MessagePayload) -> super::message_protocol::MessagePayload {
            msg
        }

        #[ink(message)]
        pub fn get_recv_message(&self, msg: super::message_define::IReceivedMessage) -> super::message_define::IReceivedMessage {
            msg
        }

        /// User defined behaviors when messages or invocations are received from other chains
        #[ink(message)]
        pub fn test_callee_received(&self, m_payload: super::message_protocol::MessagePayload) ->ink::prelude::string::String{
            let mut s = ink::prelude::string::String::new();
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("0")) {
                let ss = item.in_to::<ink::prelude::string::String>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("1")) {
                // This is for test, and use `if let` is better
                let ss = match item.tv.clone() {
                    super::message_protocol::MsgDetail::InkU8(val) => {
                        ink::prelude::format!("{:?}", val)
                    },
                    _ => {
                        ink::prelude::string::String::from("")
                    }
                };
                s = s + &ss;
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("2")) {
                let ss = item.in_to::<u16>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("3")) {
                let ss = item.in_to::<u32>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("4")) {
                let ss = item.in_to::<u64>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("5")) {
                let ss = item.in_to::<u128>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("6")) {
                let ss = item.in_to::<i8>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("7")) {
                let ss = item.in_to::<i16>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("8")) {
                let ss = item.in_to::<i32>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("9")) {
                let ss = item.in_to::<i64>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("10")) {
                let ss = item.in_to::<i128>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("11")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<ink::prelude::string::String>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("12")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<u8>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("13")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<u16>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("14")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<u32>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("15")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<u64>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("16")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<u128>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("17")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<i8>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("18")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<i16>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("19")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<i32>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("20")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<i64>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("21")) {
                let ss = item.in_to::<ink::prelude::vec::Vec<i128>>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("22")) {
                let ss = item.in_to::<super::message_protocol::InkAddressData>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }
            if let Some(item) = m_payload.get_item(ink::prelude::string::String::from("23")) {
                let ss = item.in_to::<UserMessage>();
                s = s + &ink::prelude::format!("{:?}", ss);
                s += "\n";
            }

            s
        }

        /// User defined behaviors when messages or invocations are received from other chains
        #[ink(message)]
        pub fn test_raw_data(&self, m_payload: super::message_protocol::MessagePayload) ->ink::prelude::vec::Vec<u8>{
            m_payload.into_raw_data()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {


        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let payload = Payload::new(false);
            assert_eq!(payload.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut m_hm: ink::prelude::vec::Vec<(u32, u32)> = ink::prelude::vec::Vec::new();
            m_hm.push((1, 1));
            m_hm.push((2, 2));

            // let m_hash_map: ink::prelude::collections::HashMap<u32, u32> = ink::prelude::collections::HashMap::from_iter(m_hm.clone());

            assert_eq!(*m_hm, [(1, 1), (2, 2)]);

        }

        /// test encode and decode
        #[ink::test]
        fn test_encode_decode() {
            let msg = UserMessage{
                name: "Nika".into(),
                age: 37,
                phones: ink::prelude::vec!["123".into(), "456".into()],
            };

            let mut v: ink::prelude::vec::Vec::<u8> = ink::prelude::vec::Vec::<u8>::new();
            scale::Encode::encode_to(&msg, &mut v);
            let mut vv = v.as_slice();
            let vout: UserMessage = scale::Decode::decode(&mut vv).unwrap();
            println!("{:?}", vout);
            assert_eq!(Some(msg), Some(vout));
        }

        /// test encode and decode of user defined contract interface
        // #[ink::test]
        // fn test_contract_en_de() {
        //     let msg = MessageDetail{
        //         name: "Nika".into(),
        //         age: 37,
        //         phones: ink::prelude::vec!["123".into(), "456".into()],
        //     };

        //     let rst_s = ink::prelude::format!("{:?}", msg) + "\n" + &ink::prelude::format!("{:?}", msg) + "\n" + &ink::prelude::format!("{:?}", msg) + "\n";

        //     let mut v: ink::prelude::vec::Vec::<u8> = ink::prelude::vec::Vec::<u8>::new();
        //     scale::Encode::encode_to(&msg, &mut v);

        //     let mut msg_payload = super::super::message_protocol::MessagePayload::new();
        //     let msg_item = super::super::message_protocol::MessageItem{
        //         n: ink::prelude::string::String::from("1"),
        //         t: super::super::message_protocol::MsgType::UserData,
        //         v: v.clone(),
        //     };

        //     let msg_item2 = super::super::message_protocol::MessageItem::from(ink::prelude::string::String::from("1"), 
        //                                                                         super::super::message_protocol::MsgType::UserData, 
        //                                                                         msg.clone());

        //     assert_eq!(msg_item, msg_item2);

        //     assert_eq!(msg_payload.push_item(ink::prelude::string::String::from("1"), 
        //                                         super::super::message_protocol::MsgType::UserData, 
        //                                         msg.clone()), 
        //                                         true);

        //     let mut vec_eles: ink::prelude::vec::Vec<MessageDetail> = ink::prelude::vec::Vec::new();
        //     vec_eles.push(msg.clone());
        //     vec_eles.push(msg.clone());

        //     // let msg_item_vec = super::super::message_protocol::MessageItem::from(ink::prelude::string::String::from("11"), 
        //     //                                                                         super::super::message_protocol::MsgType::UserData, 
        //     //                                                                         vec_eles.clone());

        //     assert_eq!(msg_payload.push_item(ink::prelude::string::String::from("11"), 
        //                                         super::super::message_protocol::MsgType::UserData, 
        //                                         vec_eles.clone()), 
        //                                         true);
            
        //     // simulate encode `MessagePayload` from routers(off-chain js)
        //     let mut pl_code: ink::prelude::vec::Vec::<u8> = ink::prelude::vec::Vec::<u8>::new();
        //     scale::Encode::encode_to(&msg_payload, &mut pl_code);

        //     // simulate decode `MessagePayload` implemented underlying
        //     let mut vv = pl_code.as_slice();
        //     let vout: super::super::message_protocol::MessagePayload = scale::Decode::decode(&mut vv).unwrap();

        //     // simulate contract call
        //     let payload = Payload::default();
        //     let return_s = payload.test_callee_received(vout);

        //     assert_eq!(return_s, rst_s);
        // }

        /// test vec compare
        #[ink::test]
        fn test_vec_compare() {
            let v1: ink::prelude::vec::Vec<u8> = ink::prelude::vec![1, 2, 3];
            let v2: ink::prelude::vec::Vec<u8> = ink::prelude::vec![1, 2, 3];

            assert_eq!(v1, v2);
        }

        /// test msg hash
        #[ink::test]
        fn test_msg_hash() {
            let default_str = ink::prelude::string::String::from("default str");
            let default_num: [u8;32] = [0; 32];
            let default_act: [u8;4] = [0;4];
            let recv_msg = super::super::message_define::IReceivedMessage::new(
                18,
                default_str.clone(),
                default_str.clone(),
                ink::prelude::vec![],
                ink::prelude::vec![],
                ink::prelude::vec![],
                default_num,
                default_act,
                ink::prelude::vec![],
                super::super::message_define::ISession {
                    id: 128,
                    session_type: 0,
                    callback: ink::prelude::vec![],
                    commitment: ink::prelude::vec![],
                    answer: ink::prelude::vec![],
                },
            );

            // use ink_env::hash::{Sha2x256, HashOutput};
            // let input: &[u8] = &[13, 14, 15];
            // `output1` is the type of `[u8;32]`
            // let mut output1 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            let output1  = recv_msg.into_hash::<ink::env::hash::Sha2x256>();

            // let mut output2 = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            let output2  = recv_msg.into_hash::<ink::env::hash::Sha2x256>();

            // let hash = ink_env::hash_bytes(input: &[u8], output: &mut <H as HashOutput>::Type)
            // assert_eq!(hash, ());
            assert_eq!(output1, output2);
        }

        #[ink::test]
        fn test_crypto_payload() {
            let address_here = super::super::message_protocol::InkAddressData {
                ink_address: ink::prelude::vec![1, 2, 3],
                address_type: 0
            };

            let raw1 = address_here.clone().into_raw_data();
            let raw2 = address_here.into_raw_data();

            assert_eq!(raw1, raw2);
        }

        /// test raw data of received message
        #[ink::test]
        fn test_raw_data_received() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut default_num: [u8;32] = [0; 32];
            default_num[31] = 0x34;
            default_num[30] = 0x12;
            let default_act: [u8;4] = [0x12, 0x34, 0x56, 0x78];
            let mut msg_payload = crate::message_protocol::MessagePayload::new();
            msg_payload.push_item(String::try_from("greeting").unwrap(), MsgDetail::InkString("asdf".to_string()));
            let data = msg_payload.to_bytes();
            let alice: [u8; 32] = *accounts.alice.as_ref();
            let mut sqos: Vec<crate::message_define::ISQoS> = ink::prelude::vec![];
            sqos.push(crate::message_define::ISQoS::new(crate::message_define::ISQoSType::SelectionDelay, Vec::<u8>::from([0x12, 0x34, 0x56, 0x78])));
            let recv_msg = super::super::message_define::IReceivedMessage::new(
                1,
                "POLKADOT".to_string(),
                "POLKADOT".to_string(),
                ink::prelude::vec::Vec::from(alice),
                ink::prelude::vec::Vec::from(alice),
                sqos,
                default_num,
                default_act,
                data,
                super::super::message_define::ISession {
                    id: 0,
                    session_type: 0,
                    callback: Vec::<u8>::from([0x11, 0x11, 0x11, 0x11]),
                    commitment: Vec::<u8>::from([0x22]),
                    answer: Vec::<u8>::from([0x33]),
                },
            );
            
            let b = recv_msg.into_raw_data();
            println!("test_raw_data_received{:?}", b);
        }
        
        /// test raw data of sent message
        #[ink::test]
        fn test_raw_data_sent() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut default_num: [u8;32] = [0; 32];
            default_num[31] = 0x34;
            default_num[30] = 0x12;
            let default_act: [u8;4] = [0x12, 0x34, 0x56, 0x78];
            let mut msg_payload = crate::message_protocol::MessagePayload::new();
            msg_payload.push_item(String::try_from("greeting").unwrap(), MsgDetail::InkString("asdf".to_string()));
            let data = msg_payload.to_bytes();
            let alice: [u8; 32] = *accounts.alice.as_ref();
            let mut sqos: Vec<crate::message_define::ISQoS> = ink::prelude::vec![];
            sqos.push(crate::message_define::ISQoS::new(crate::message_define::ISQoSType::SelectionDelay, Vec::<u8>::from([0x12, 0x34, 0x56, 0x78])));
            let recv_msg = super::super::message_define::IReceivedMessage::new(
                1,
                "POLKADOT".to_string(),
                "POLKADOT".to_string(),
                ink::prelude::vec::Vec::from(alice),
                ink::prelude::vec::Vec::from(alice),
                sqos,
                default_num,
                default_act,
                data,
                super::super::message_define::ISession {
                    id: 0,
                    session_type: 0,
                    callback: Vec::<u8>::from([0x11, 0x11, 0x11, 0x11]),
                    commitment: Vec::<u8>::from([0x22]),
                    answer: Vec::<u8>::from([0x33]),
                },
            );

            println!("{:?}", ink::prelude::vec::Vec::from(alice));
            
            let b = recv_msg.into_raw_data();
            println!("{:?}", b);
        }

        /// test raw data of SQoS
        #[ink::test]
        fn test_raw_data_sqos() {
            let sqos1 = super::super::message_define::ISQoS::new(super::super::message_define::ISQoSType::Isolation, ink::prelude::vec![1, 2]);
            let sqos2 = super::super::message_define::ISQoS::new(super::super::message_define::ISQoSType::Isolation, ink::prelude::vec![]);

            assert_eq!(sqos1.into_raw_data(), ink::prelude::vec![8, 1, 2], "sqos1 `into_raw_data error!`");
            assert_eq!(sqos2.into_raw_data(), ink::prelude::vec![8]);
        }
    }
}
