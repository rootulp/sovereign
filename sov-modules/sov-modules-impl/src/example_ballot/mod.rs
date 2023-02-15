mod call;
mod genesis;

use sov_modules_api::Error;
use sov_modules_macros::ModuleInfo;

#[derive(ModuleInfo)]
pub struct Ballot<C: sov_modules_api::Context> {
    #[state]
    pub chairperson: sov_state::StateValue<C::PublicKey, C::Storage>,

    #[state]
    pub proposals: sov_state::StateValue<call::Proposals, C::Storage>,

    #[state]
    pub voters: sov_state::StateMap<C::PublicKey, call::Voter, C::Storage>,
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
            call::CallMessage::SetProposals(proposals) => Ok(self.set_proposals(proposals)?),
        }
    }
}
