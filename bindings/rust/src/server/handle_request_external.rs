use super::constants::Constants;
use super::routes::ROUTES;
use crate::module_wrapper::{get_string_from_pointer, get_string_pointer};
use wasmedge_sdk::error::HostFuncError;
use wasmedge_sdk::{CallingFrame, WasmValue};

pub(crate) fn handle_request_external(
    _calling_frame: CallingFrame,
    params: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, HostFuncError> {
    let request_handler_index = params[0].to_i32();
    let request_pointer = params[1].to_i32();
    let request_length = params[2].to_i32();

    if request_pointer == Constants::NoRequestError.get() {
        return Ok(vec![WasmValue::from_i32(
            Constants::EmptyResponseError.get(),
        )]);
    }

    let request_body_string = get_string_from_pointer(
        request_pointer.try_into().unwrap(),
        request_length.try_into().unwrap(),
    );

    let response_body_string = unsafe {
        let request_handler = ROUTES.get(&request_handler_index).unwrap();
        (request_handler.handler)(request_body_string)
    };

    let response_body_pointer = get_string_pointer(&response_body_string);
    Ok(vec![WasmValue::from_i32(response_body_pointer)])
}
