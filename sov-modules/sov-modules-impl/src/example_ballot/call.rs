use std::io::Cursor;

use super::Ballot;
use anyhow::{bail, Result};
use borsh::{BorshDeserialize, BorshSerialize};
use sov_modules_api::{CallResponse, Context};
use sovereign_sdk::serial::{Decode, DecodeBorrowed, Encode};

pub struct Voter<C: sov_modules_api::Context> {
    weight: u32,
    voted: bool,
    pub_key: C::PublicKey,
    vote: u32,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProposalsMsg {
    names: Vec<String>,
}

#[derive(BorshDeserialize)]
pub enum CallMessage {
    SetProposals(ProposalsMsg),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub(crate) struct Proposal {
    name: String,
    count: u32,
}

impl<C: sov_modules_api::Context> Ballot<C> {
    pub(crate) fn set_proposals(
        &mut self,
        proposals_msg: ProposalsMsg,
        context: C,
    ) -> Result<CallResponse> {
        if self.chairperson.get().as_ref() != Some(context.sender()) {
            // chairperson has to be set and equal to sender
            bail!("todo")
        }

        if self.proposals.get().is_some() {
            bail!("todo")
        }

        let proposals = proposals_msg
            .names
            .into_iter()
            .map(|name| Proposal { name, count: 0 })
            .collect();

        self.proposals.set(proposals);
        Ok(CallResponse::default())
    }

    pub(crate) fn give_right_to_vote(&mut self, context: C) {
        if self.chairperson.get().as_ref() != Some(context.sender()) {
            // chairperson has to be set and equal to sender
            // bail!("todo");
        }
    }
}

// Generated code
impl<'de, C: Context> DecodeBorrowed<'de> for Voter<C> {
    type Error = anyhow::Error;

    fn decode_from_slice(target: &'de [u8]) -> std::result::Result<Self, Self::Error> {
        let mut target = Cursor::new(target.to_vec());

        // Self::decode(&mut target);

        todo!()
    }
}

impl<C: Context> Decode for Voter<C> {
    type Error = anyhow::Error;

    fn decode<R: std::io::Read>(
        target: &mut R,
    ) -> std::result::Result<Self, <Self as Decode>::Error> {
        let weight = u32::decode(target)?;
        let voted = bool::decode(target)?;
        let pub_key = C::PublicKey::decode(target)?;
        let vote = u32::decode(target)?;
        Ok(Self {
            weight,
            voted,
            pub_key,
            vote,
        })
    }
}

impl<C: Context> Encode for Voter<C> {
    fn encode(&self, target: &mut impl std::io::Write) {
        self.weight.encode(target);
        self.voted.encode(target);
        self.pub_key.encode(target);
        self.vote.encode(target);
    }
}
