use std::convert::Infallible;

use crate::Context;
use sov_state::{JmtStorage, ZkStorage};
use sovereign_sdk::serial::{Decode, DecodeBorrowed, Encode};

/// Mock for Context::PublicKey, useful for testing.
#[derive(PartialEq, Eq, Clone)]
pub struct MockPublicKey {
    pub_key: Vec<u8>,
}

impl Encode for MockPublicKey {
    fn encode(&self, target: &mut impl std::io::Write) {
        self.pub_key.encode(target)
    }
}

impl<'de> DecodeBorrowed<'de> for MockPublicKey {
    type Error = anyhow::Error;

    fn decode_from_slice(target: &'de [u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl Decode for MockPublicKey {
    type Error = anyhow::Error;

    fn decode<R: std::io::Read>(target: &mut R) -> Result<Self, <Self as Decode>::Error> {
        let pub_key = Vec::<u8>::decode(target)?;
        Ok(Self { pub_key })
    }
}

impl MockPublicKey {
    pub fn new(pub_key: Vec<u8>) -> Self {
        Self { pub_key }
    }
}

impl TryFrom<&'static str> for MockPublicKey {
    type Error = Infallible;

    fn try_from(key: &'static str) -> Result<Self, Self::Error> {
        let key = key.as_bytes().to_vec();
        Ok(Self { pub_key: key })
    }
}

/// Mock for Context::Signature, useful for testing.
#[derive(borsh::BorshDeserialize, PartialEq, Eq)]
pub struct MockSignature {
    sig: Vec<u8>,
}

impl MockSignature {
    pub fn new(sig: Vec<u8>) -> Self {
        Self { sig }
    }
}

/// Mock for Context, useful for testing.
pub struct MockContext {
    pub sender: MockPublicKey,
}

impl Context for MockContext {
    type Storage = JmtStorage;

    type PublicKey = MockPublicKey;

    fn sender(&self) -> &Self::PublicKey {
        &self.sender
    }
}

pub struct ZkMockContext {
    pub sender: MockPublicKey,
}

impl Context for ZkMockContext {
    type Storage = ZkStorage;

    type PublicKey = MockPublicKey;

    fn sender(&self) -> &Self::PublicKey {
        &self.sender
    }
}
