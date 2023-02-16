use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::ffi::{c_char, c_void, CStr};
use std::io::{Read, Write};
use std::mem;
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

extern "C" {
    fn handle_request_external(idx: i32, req_ptr: *mut c_void, req_len: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn allocate(size: i32) -> *const u8 {
    let buffer = Vec::with_capacity(size as usize);

    let mut buffer = mem::ManuallyDrop::new(buffer);
    let pointer = buffer.as_mut_ptr();

    pointer as *const u8
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

    let write_buf = response.to_string();
    stream.write(write_buf.as_bytes())?;
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

fn handle_request(request_data: Vec<u8>, controllers: &Controllers) -> Response<String> {
    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let raw_response = match decoder.decode_from_bytes(request_data.as_slice()) {
        Ok(req) => respond_request(req, controllers),
        Err(e) => Err(e),
    };

    match raw_response {
        Ok(r) => r,
        Err(e) => {
            let err = format!("{:?}", e);
            Response::new(
                HttpVersion::V1_0,
                StatusCode::new(500).unwrap(),
                ReasonPhrase::new(err.as_str()).unwrap(),
                err.clone(),
            )
        }
    }
}

fn respond_request(
    req: Request<String>,
    controllers: &Controllers,
) -> bytecodec::Result<Response<String>> {
    let path = req.request_target();
    let idx = controllers.get(path.as_str()).unwrap_or(&-1);
    let req_json = serde_json::to_string(req.body()).unwrap();
    let req_body_ptr = req_json.as_ptr() as *mut c_void;
    let req_len = req_json.len();
    unsafe {
        let res_ptr = handle_request_external(idx.clone(), req_body_ptr, req_len);
        let raw_res = CStr::from_ptr(res_ptr as *mut i8)
            .to_str()
            .unwrap()
            .to_string();
        let res = Response::new(
            HttpVersion::V1_0,
            StatusCode::new(200)?,
            ReasonPhrase::new("")?,
            format!(
                "Hello world from WebAssembly!

Path: {},
Body: {}",
                path, raw_res,
            ),
        );
        return Ok(res);
    }
}
