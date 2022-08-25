// include!("message_protocol.rs");

use ink_prelude::{
    string::String,
};

use scale::{
    Encode,
    Decode,
};

use crate::message_protocol::MessagePayload;

/// Errors for cross-chain contract
#[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum IError {
    NotOwner,
    IdNotMatch,
    ChainMessageNotFound,
    IdOutOfBound,
    AlreadyExecuted,
    InterfaceNotFound,
    DecodeDataFailed,
    CrossContractCallFailed,
}

impl scale_info::TypeInfo for IError {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IError", module_path!()))
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant("NotOwner", |v| v.index(0))
                                .variant("IdNotMatch", |v| v.index(1))
                                .variant("ChainMessageNotFound", |v| v.index(2))
                                .variant("IdOutOfBound", |v| v.index(3))
                                .variant("AlreadyExecuted", |v| v.index(4))
                                .variant("InterfaceNotFound", |v| v.index(5))
                        )
    }
}

/// Content structure
#[derive(Decode, Encode, Clone)]
// #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct IContent {
    pub contract: ink_prelude::vec::Vec<u8>,
    pub action: ink_prelude::vec::Vec<u8>,
    pub data: ink_prelude::vec::Vec<u8>,
}

impl IContent {
    pub fn new(contract: ink_prelude::vec::Vec<u8>, action: ink_prelude::vec::Vec<u8>, data: ink_prelude::vec::Vec<u8>) -> Self {
        Self {
            contract,
            action,
            data,
        }
    }
}

impl scale_info::TypeInfo for IContent {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IContent", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("contract").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("action").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("data").type_name("ink_prelude::vec::Vec<u8>"))
                    )
    }
}

/// SQOS structure
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum ISQoSType{
    Reveal,
    Challenge,
    Threshold,
    Priority,
    ExceptionRollback,
    SelectionDelay,
    Anonymous,
    Identity,
    Isolation,
    CrossVerify,
}

impl ::scale_info::TypeInfo for ISQoSType {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("ISQoSType", module_path!()))
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant("Reveal", |v| v.index(0))
                                .variant("Challenge", |v| v.index(1))
                                .variant("Threshold", |v| v.index(2))
                                .variant("Priority", |v| v.index(3))
                                .variant("ExceptionRollback", |v| v.index(4))
                                .variant("SelectionDelay", |v| v.index(5))
                                .variant("Anonymous", |v| v.index(6))
                                .variant("Identity", |v| v.index(7))
                                .variant("Isolation", |v| v.index(8))
                                .variant("CrossVerify", |v| v.index(9))
                        )
    }
}

#[derive(Debug, Clone, Decode, Encode)]
// #[cfg_attr(feature = "std", derive(Debug, scale_info::TypeInfo))]
pub struct ISQoS {
    pub t: ISQoSType,
    pub v: Option<ink_prelude::vec::Vec<u8>>,
}

impl scale_info::TypeInfo for ISQoS {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("ISQoS", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<ISQoSType>().name("t").type_name("ISQoSType"))
                        .field(|f| f.ty::<Option<ink_prelude::vec::Vec<u8>>>().name("v").type_name("Option<ink_prelude::vec::Vec<u8>>"))
                    )
    }
}

impl ISQoS {
    pub fn new(t: ISQoSType, v: Option<ink_prelude::vec::Vec<u8>>) -> Self {
        Self {
            t,
            v,
        }
    }

    pub fn into_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
        let mut raw_buffer = ink_prelude::vec![];

        let t_u8: u8 = match self.t {
            ISQoSType::Reveal => 0,
            ISQoSType::Challenge => 1,
            ISQoSType::Threshold => 2,
            ISQoSType::Priority => 3,
            ISQoSType::ExceptionRollback => 4,
            ISQoSType::SelectionDelay => 5,
            ISQoSType::Anonymous => 6,
            ISQoSType::Identity => 7,
            ISQoSType::Isolation => 8,
            ISQoSType::CrossVerify => 9,
        };

        raw_buffer.append(&mut ink_prelude::vec::Vec::from(t_u8.to_be_bytes()));
        raw_buffer.append(&mut self.v.clone().unwrap_or(ink_prelude::vec![]));

        raw_buffer
    }
}

/// Session Structure
#[derive(Debug, Clone, Decode, Encode)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ISession {
    pub id: u128,
    pub callback: ink_prelude::vec::Vec<u8>,
}

impl ISession {
    pub fn new(id: u128, callback: ink_prelude::vec::Vec<u8>) -> Self {
        Self {
            id,
            callback,
        }
    }

    pub fn into_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
        let mut raw_buffer = ink_prelude::vec![];

        raw_buffer.append(&mut ink_prelude::vec::Vec::from(self.id.to_be_bytes()));
        raw_buffer.append(&mut self.callback.clone());

        raw_buffer
    }
}

impl scale_info::TypeInfo for ISession {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("ISession", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<u128>().name("id").type_name("u128"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("callback").type_name("ink_prelude::vec::Vec<u8>"))
                    )
    }
}

/// Received message structure
#[derive(Debug, Decode, Encode, Clone)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct IReceivedMessage {
    pub id: u128,
    pub from_chain: String,
    pub to_chain: String,
    pub sender: ink_prelude::vec::Vec<u8>,
    pub signer: ink_prelude::vec::Vec<u8>,
    pub sqos: ink_prelude::vec::Vec<ISQoS>,
    pub contract: [u8;32],
    pub action: [u8;4],
    pub data: ink_prelude::vec::Vec<u8>,
    pub session: ISession,
}

impl scale_info::TypeInfo for IReceivedMessage {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IReceivedMessage", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<u128>().name("id").type_name("u128"))
                        .field(|f| f.ty::<String>().name("from_chain").type_name("String"))
                        .field(|f| f.ty::<String>().name("to_chain").type_name("String"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("sender").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("signer").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<ISQoS>>().name("sqos").type_name("ink_prelude::vec::Vec<ISQoS>"))
                        .field(|f| f.ty::<[u8;32]>().name("contract").type_name("[u8;32]"))
                        .field(|f| f.ty::<[u8;4]>().name("action").type_name("[u8;4]"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("data").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ISession>().name("session").type_name("ISession"))
                    )
    }
}

impl IReceivedMessage {
    pub fn new(id: u128, from_chain: String, to_chain: String, sender: ink_prelude::vec::Vec<u8>, signer: ink_prelude::vec::Vec<u8>, sqos: ink_prelude::vec::Vec<ISQoS>,
        contract: [u8;32], action: [u8;4], data: ink_prelude::vec::Vec<u8>, session: ISession) -> Self {
        Self {
            id,
            from_chain,
            to_chain,
            sender,
            signer,
            sqos,
            contract,
            action,
            data,
            session,
        }
    }

    pub fn into_bytes(&self) -> ink_prelude::vec::Vec<u8> {
        let mut msg_code: ink_prelude::vec::Vec<u8> = ink_prelude::vec::Vec::<u8>::new();
        scale::Encode::encode_to(self, &mut msg_code);
        msg_code
    }

    pub fn into_hash<T: ink_env::hash::HashOutput+ink_env::hash::CryptoHash>(&self) -> <T as ink_env::hash::HashOutput>::Type {
        let mut output = <T as ink_env::hash::HashOutput>::Type::default();
        ink_env::hash_encoded::<T, _>(&self, &mut output);
        output
    }

    pub fn into_raw_data(&self) -> ink_prelude::vec::Vec<u8> {
        let mut raw_string_vec = ink_prelude::vec![];
        raw_string_vec.append(&mut ink_prelude::vec::Vec::from(self.id.to_be_bytes()));
        raw_string_vec.append(&mut ink_prelude::vec::Vec::from(self.from_chain.as_bytes()));
        raw_string_vec.append(&mut ink_prelude::vec::Vec::from(self.to_chain.as_bytes()));

        for s in self.sqos.iter() {
            raw_string_vec.append(&mut s.into_raw_data());
        }

        raw_string_vec.append(&mut ink_prelude::vec::Vec::from(self.contract));
        raw_string_vec.append(&mut ink_prelude::vec::Vec::from(self.action));
        let payload: MessagePayload = scale::Decode::decode(&mut self.data.as_slice()).unwrap();
        raw_string_vec.append(&mut payload.into_raw_data());
        raw_string_vec.append(&mut self.sender.clone());
        raw_string_vec.append(&mut self.signer.clone());

        raw_string_vec.append(&mut self.session.into_raw_data());

        raw_string_vec
    }
}

/// Sent message structure
#[derive(Decode, Encode, Clone)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ISentMessage {
    pub to_chain: String,
    pub sqos: ink_prelude::vec::Vec<ISQoS>,
    pub content: IContent,
    pub session: ISession,
}

impl scale_info::TypeInfo for ISentMessage {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("ISentMessage", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<String>().name("to_chain").type_name("String"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<ISQoS>>().name("sqos").type_name("ink_prelude::vec::Vec<ISQoS>"))
                        .field(|f| f.ty::<IContent>().name("content").type_name("IContent"))
                        .field(|f| f.ty::<ISession>().name("session").type_name("ISession"))
                    )
    }
}

impl ISentMessage {
    pub fn new(to_chain: String, sqos: ink_prelude::vec::Vec<ISQoS>, content: IContent, session: ISession) -> Self {
        Self {
            to_chain,
            sqos,
            content,
            session,
        }
    }
}

/// Request message structure
#[derive(Decode, Encode, Clone)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct IRequestMessage {
    pub to_chain: String,
    pub sqos: ink_prelude::vec::Vec<ISQoS>,
    pub content: IContent,
}

impl scale_info::TypeInfo for IRequestMessage {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IRequestMessage", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<String>().name("to_chain").type_name("String"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<ISQoS>>().name("sqos").type_name("ink_prelude::vec::Vec<ISQoS>"))
                        .field(|f| f.ty::<IContent>().name("content").type_name("IContent"))
                    )
    }
}

impl IRequestMessage {
    pub fn new(to_chain: String, sqos: ink_prelude::vec::Vec<ISQoS>, content: IContent) -> Self {
        Self {
            to_chain,
            sqos,
            content,
        }
    }
}

/// Response message structure
#[derive(Decode, Encode, Clone)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct IResponseMessage {
    pub sqos: ink_prelude::vec::Vec<ISQoS>,
    pub data: ink_prelude::vec::Vec<u8>,
}

impl scale_info::TypeInfo for IResponseMessage {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IResponseMessage", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<ink_prelude::vec::Vec<ISQoS>>().name("sqos").type_name("ink_prelude::vec::Vec<ISQoS>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("data").type_name("ink_prelude::vec::Vec<u8>"))
                    )
    }
}

impl IResponseMessage {
    pub fn new(sqos: ink_prelude::vec::Vec<ISQoS>, data: ink_prelude::vec::Vec<u8>) -> Self {
        Self {
            sqos,
            data,
        }
    }
}

/// Context structure
#[derive(Debug, Clone, Decode, Encode)]
// #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct IContext {
    pub id: u128,
    pub from_chain: String,
    pub sender: ink_prelude::vec::Vec<u8>,
    pub signer: ink_prelude::vec::Vec<u8>,
    pub sqos: ink_prelude::vec::Vec<ISQoS>,
    pub contract: [u8;32],
    pub action: [u8;4],
    pub session: ISession,
}

impl scale_info::TypeInfo for IContext {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("IContext", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<u128>().name("id").type_name("u128"))
                        .field(|f| f.ty::<String>().name("from_chain").type_name("String"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("sender").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("signer").type_name("ink_prelude::vec::Vec<u8>"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<ISQoS>>().name("sqos").type_name("ink_prelude::vec::Vec<ISQoS>"))
                        .field(|f| f.ty::<[u8;32]>().name("contract").type_name("[u8;32]"))
                        .field(|f| f.ty::<[u8;4]>().name("action").type_name("[u8;4]"))
                        .field(|f| f.ty::<ISession>().name("session").type_name("ISession"))
                    )
    }
}

impl IContext {
    pub fn new(id: u128, from_chain: String, sender: ink_prelude::vec::Vec<u8>, signer: ink_prelude::vec::Vec<u8>, sqos: ink_prelude::vec::Vec<ISQoS>,
            contract: [u8;32], action: [u8;4], session: ISession) -> Self {
        Self {
            id,
            from_chain,
            sender,
            signer,
            sqos,
            contract,
            action,
            session,
        }
    }
}
