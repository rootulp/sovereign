use anyhow::{bail, Result};

use super::Ballot;

impl<C: sov_modules_api::Context> Ballot<C> {
    /// Initializes module with the `admin` role.
    pub(crate) fn init_module(&mut self) -> Result<()> {
        let maybe_chairperson = C::PublicKey::try_from("admin");

        let chairperson = match maybe_chairperson {
            Ok(chairperson) => chairperson,
            Err(_) => bail!("Chairperson initialization failed"),
        };

        self.chairperson.set(chairperson);
        Ok(())
    }
}
