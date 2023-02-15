mod call;
mod genesis;

use borsh::{BorshDeserialize, BorshSerialize};
use sov_modules_api::Error;
use sov_modules_macros::ModuleInfo;

#[derive(ModuleInfo)]
pub struct Ballot<C: sov_modules_api::Context> {
    #[state]
    pub(crate) chairperson: sov_state::StateValue<C::PublicKey, C::Storage>,

    // TODO replace it with a Map once we have iterators
    #[state]
    pub(crate) proposals: sov_state::StateValue<Vec<call::Proposal>, C::Storage>,

    #[state]
    pub(crate) voters: sov_state::StateMap<C::PublicKey, call::Voter<C>, C::Storage>,
}

impl<C: sov_modules_api::Context> sov_modules_api::Module for Ballot<C> {
    type Context = C;

    type CallMessage = call::CallMessage;

    type QueryMessage = sov_modules_api::NonInstantiable;

    fn genesis(&mut self) -> Result<(), Error> {
        Ok(self.init_module()?)
    }

    fn call(
        &mut self,
        msg: Self::CallMessage,
        context: Self::Context,
    ) -> Result<sov_modules_api::CallResponse, Error> {
        match msg {
            call::CallMessage::SetProposals(proposals) => {
                Ok(self.set_proposals(proposals, context)?)
            }
        }
    }
}
