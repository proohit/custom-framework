use crate::server::handle_request_external::handle_request_external;
use wasmedge_sdk;
use wasmedge_sdk::FuncType;
use wasmedge_sdk::ImportObjectBuilder;
use wasmedge_sdk::ValType;

use super::constants::Constants;

pub(crate) fn get_env_module() -> wasmedge_sdk::ImportObject {
    let module = ImportObjectBuilder::new()
        .with_func_by_type(
            Constants::HandleRequestExternalFunctionName.as_str(),
            FuncType::new(
                Some(vec![ValType::I32, ValType::I32, ValType::I32]),
                Some(vec![ValType::I32]),
            ),
            handle_request_external,
        )
        .unwrap()
        .build(Constants::EnvModuleName.as_str())
        .unwrap();
    module
}
