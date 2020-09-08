mod posts;
mod gets;
mod db_reqs;
use posts::*;
use gets::*;
use db_reqs::*;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{fs,str};
use std::string::String;


#[derive(Serialize, Deserialize)]
struct Request {
    protocol: String,
    path: String,
    domain: String,
    port: String,
    method: String,
    max_reqs_per_conn:usize,
    headers: String,    
    path_to_body: String,
    thread_num: usize,
    db_host_ip: String,
    db_port: String,
    db_meas_name: String,
    sleep_time: u64,
}

fn main() {
    let init_str = init().unwrap();
    create_db(&init_str.db_host_ip,&init_str.db_port);
    let mut start: std::time::Instant;
    let mut resp_time: f64;
    loop{       
    start = std::time::Instant::now();
    let resp = send_req(&init_str);
    resp_time = (start.elapsed().as_millis())as f64/1000.0f64;
    let temp_str: Vec<&str> = std::str::from_utf8(&resp).unwrap().split(" ").collect();
    let status_code = temp_str[1];
    send_data_to_db(&init_str.db_host_ip,&init_str.db_port, status_code, &resp_time.to_string(),&init_str.db_meas_name);
    std::thread::sleep(std::time::Duration::from_millis(init_str.sleep_time));   
}

fn init() -> Result<Request>{
    let mut file = std::env::args();
    file.next();
    //Request data serialization
    let temp = fs::read(file.next().unwrap()).unwrap();
    let data_to_serialize: &str = str::from_utf8(&temp).unwrap();
    let req: Request = serde_json::from_str(data_to_serialize)?;

    Ok(req)
}

fn send_req(req_d:&Request)->[u8;16]{
    if req_d.protocol == "http".to_string() && req_d.method=="GET"{
        get_req(&req_d.path, &req_d.domain, &req_d.port, &req_d.headers)
    }
    else if req_d.protocol == "https".to_string() && req_d.method=="POST"{
        let temp_req_body = fs::read(&req_d.path_to_body).unwrap();
        let req_body = String::from_utf8(temp_req_body).unwrap();
        tls_post_req(&req_d.path, &req_d.domain, &req_d.port, &req_body, &req_d.headers)        
    }
    else if req_d.protocol == "https".to_string() && req_d.method=="GET"{
        tls_get_req(&req_d.path, &req_d.domain, &req_d.port, &req_d.headers)
    }
    else {
        let temp_req_body = fs::read(&req_d.path_to_body).unwrap();
        let req_body = String::from_utf8(temp_req_body).unwrap();
        post_req(&req_d.path, &req_d.domain, &req_d.port, &req_body, &req_d.headers)
    } 
}
}
