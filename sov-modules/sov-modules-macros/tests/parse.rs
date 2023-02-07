use sov_modules_api::mocks::{MockContext, MockStorage};
use sov_modules_api::Context;
use sov_modules_macros::ModuleInfo;
use sov_state::StateMap;

mod test_module {
    use super::*;

    #[derive(ModuleInfo)]
    pub(crate) struct TestStruct<C: Context> {
        #[state]
        pub test_state1: StateMap<C::PublicKey, u32, C::Storage>,

        #[state]
        pub test_state2: StateMap<String, String, C::Storage>,
    }
}

fn main() {
    let test_storage = MockStorage::default();
    let test_struct = test_module::TestStruct::<MockContext>::_new(test_storage);

    let prefix1 = test_struct.test_state1.prefix();

    assert_eq!(
        *prefix1,
        sov_modules_api::Prefix::new(
            // The tests compile inside trybuild.
            "trybuild000::test_module".to_owned(),
            "TestStruct".to_owned(),
            "test_state1".to_owned(),
        )
        .into()
    );

    let prefix2 = test_struct.test_state2.prefix();
    assert_eq!(
        *prefix2,
        sov_modules_api::Prefix::new(
            // The tests compile inside trybuild.
            "trybuild000::test_module".to_owned(),
            "TestStruct".to_owned(),
            "test_state2".to_owned()
        )
        .into()
    );
}
