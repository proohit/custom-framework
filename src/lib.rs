use bytecodec::DecodeExt;
use httpcodec::{HttpVersion, ReasonPhrase, Request, RequestDecoder, Response, StatusCode};
use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpListener, TcpStream};

#[no_mangle]
fn start() {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("new connection at {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port), false).unwrap();
    loop {
        let _ = handle_client(listener.accept(false).unwrap().0);
    }
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let request_data = read_stream_data(&mut stream)?;

    let response = handle_request(request_data);

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

fn handle_request(request_data: Vec<u8>) -> Response<String> {
    let mut decoder =
        RequestDecoder::<httpcodec::BodyDecoder<bytecodec::bytes::Utf8Decoder>>::default();

    let raw_response = match decoder.decode_from_bytes(request_data.as_slice()) {
        Ok(req) => respond_request(req),
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

fn respond_request(req: Request<String>) -> bytecodec::Result<Response<String>> {
    let path = req.request_target();
    Ok(Response::new(
        HttpVersion::V1_0,
        StatusCode::new(200)?,
        ReasonPhrase::new("")?,
        format!(
            "Hello world from WebAssembly!

Path: {},
Body: {}",
            path,
            req.body()
        ),
    ))
}
