use super::Ballot;
use anyhow::{bail, Result};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Voter {}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Proposals {}

#[derive(BorshDeserialize)]
pub enum CallMessage {
    SetProposals(Proposals),
}

impl<C: sov_modules_api::Context> Ballot<C> {
    pub(crate) fn set_proposals(
        &self,
        proposals: Proposals,
    ) -> Result<sov_modules_api::CallResponse> {
        todo!()
    }
}
