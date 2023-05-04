use bytecodec::{DecodeExt, Error, ErrorKind};
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr};
use std::io::{Read, Write};
use std::{mem, vec};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

extern "C" {
    fn handle_request_external(idx: i32, req_ptr: *mut c_void, req_len: usize) -> i32;
}
struct ResponseInfo {
    pointer: *mut c_void,
    size: usize,
}

static mut RESPONSES: Lazy<HashMap<i32, ResponseInfo>> = Lazy::new(|| HashMap::new());

#[no_mangle]
pub unsafe extern "C" fn allocate(size: i32) -> *const u8 {
    let buffer = Vec::with_capacity(size as usize);

    let mut buffer = mem::ManuallyDrop::new(buffer);
    let pointer = buffer.as_mut_ptr();

    pointer as *const u8
}

#[no_mangle]
pub unsafe extern "C" fn register_request_response(ptr: *mut c_void, size: i32) -> i32 {
    let idx = RESPONSES.len() as i32;
    RESPONSES.insert(
        idx,
        ResponseInfo {
            pointer: ptr,
            size: size as usize,
        },
    );
    idx
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(pointer: *mut u8, size: i32) {
    drop(Vec::from_raw_parts(pointer, size as usize, size as usize));
}

type Controllers = std::collections::HashMap<String, i32>;

#[no_mangle]
pub unsafe fn start(pointer: *mut c_char) {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false).unwrap();
    let raw_controllers = CStr::from_ptr(pointer).to_str().unwrap().to_string();
    let controllers = parse_controllers(raw_controllers);
    println!("controllers: {:?}", controllers);
    loop {
        let _ = handle_client(listener.accept(false).unwrap().0, &controllers);
    }
}

fn parse_controllers(raw_controllers: String) -> Controllers {
    let mut controllers = std::collections::HashMap::new();
    raw_controllers.split(",").for_each(|controller| {
        let mut separated = controller.split(":");
        let idx = separated.next().unwrap().parse::<i32>().unwrap();
        let name = separated.next().unwrap();
        controllers.insert(name.to_string(), idx);
    });
    controllers
}

fn handle_client(mut stream: TcpStream, controllers: &Controllers) -> std::io::Result<()> {
    let request_data = read_stream_data(&mut stream)?;

    let response = handle_request(request_data, controllers);

    let http_status_code = response.status_code().to_string();
    let http_reason_phrase = response.reason_phrase().to_string();
    let http_version = response.http_version().to_string();
    let http_headers = response.header().to_string();
    let http_body = response.body();
    // build full http response
    let full_response = format!(
        "{} {} {}\r\n{}",
        http_version, http_status_code, http_reason_phrase, http_headers
    );
    stream.write(full_response.as_bytes())?;
    stream.write(http_body)?;
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}

fn read_stream_data(stream: &mut TcpStream) -> std::io::Result<Vec<u8>> {
    let mut buff = [0u8; 1024];
    let mut data = Vec::new();

    loop {
        let n = stream.read(&mut buff)?;
        data.extend_from_slice(&buff[0..n]);
        if n < 1024 {
            break;
        }
    }
    Ok(data)
}

type BinaryResponse<'a> = Response<Vec<u8>>;

fn handle_request(request_data: Vec<u8>, controllers: &Controllers) -> BinaryResponse {
    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let raw_response = match decoder.decode_from_bytes(request_data.as_slice()) {
        Ok(req) => respond_request(req, controllers),
        Err(e) => Err(e),
    };

    match raw_response {
        Ok(r) => r,
        Err(e) => {
            let err = e.to_string();
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new("").unwrap(),
                err.as_bytes().to_vec(),
            )
        }
    }
}

fn respond_request(
    req: Request<String>,
    controllers: &Controllers,
) -> bytecodec::Result<BinaryResponse> {
    let path = req.request_target();
    let idx = controllers.get(path.as_str()).unwrap_or(&-1);
    if idx == &-1 {
        return Err(Error::from(ErrorKind::Other));
    }
    let req_body_ptr = req.body().as_ptr() as *mut c_void;
    let req_len = req.body().len();
    unsafe {
        let res_id = handle_request_external(idx.clone(), req_body_ptr, req_len);
        let req = RESPONSES.get(&res_id).unwrap();
        let res_ptr = req.pointer;
        let res_len = req.size;

        let bindings_response = vec::Vec::from_raw_parts(res_ptr as *mut u8, res_len, res_len);

        let res = Response::new(
            HttpVersion::V1_1,
            StatusCode::new(200)?,
            ReasonPhrase::new("OK")?,
            bindings_response,
        );

        return Ok(res);
    }
}
