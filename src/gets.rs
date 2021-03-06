use std::io::prelude::*;
use native_tls::TlsConnector;
use std::net::TcpStream;
use std::string::String;


pub fn get_req(path: &String, domain: &String, port: &String, headers: &String)->[u8;16]{
    let temp = create_get_req(path, domain, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let mut response = [0u8;16];
    let mut connection = TcpStream::connect(&dom_port).unwrap();                        
    connection.write(request).unwrap();
    connection.read_exact(&mut response).unwrap();
    response
}   

pub fn tls_get_req(path: &String, domain: &String, port: &String, headers: &String)->[u8;16]{
    let connector = TlsConnector::new().unwrap();
    let temp = create_get_req(path, domain, headers);
    let request = temp.as_bytes();
    let dom_port = format!("{}:{}",domain,port);
    let mut response = [0u8;16];
    let tcp_stream = TcpStream::connect(&dom_port).unwrap();
    let mut tls_stream = connector.connect(&domain, tcp_stream).unwrap();    
    tls_stream.write(request).unwrap();
    tls_stream.read_exact(&mut response).unwrap();
    response
}

fn create_get_req(path: &String, domain: &String, headers: &String)-> String {
    let mut request = String::from("GET /");
    request.push_str(path);
    request.push_str(" HTTP/1.1\r\n");
    request.push_str("Host: ");
    request.push_str(domain);
    request.push_str("\r\n");
    for h in headers.lines(){
        request.push_str(h);
        request.push_str("\r\n");
    }
    request.push_str("\r\n\r\n");
    request
}