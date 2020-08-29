#[path="posts.rs"]
mod posts;

use std::string::String;
use posts::*;


pub fn create_db(){
    let db_host = "127.0.0.1".to_string();
    let db_path = "query?q=CREATE+DATABASE+responses".to_string();
    let _ = post_req(&db_path,&db_host,&"8086".to_string(),&"".to_string(),&"".to_string());
}

pub fn send_data(status: &str, resp_time: &String){
    let body = format!("resp_data,resp_status={} resp_time={}",status,resp_time);
    let cont_l = format!("Content-length: {}\n",body.len()+2);
    let db_host = "127.0.0.1".to_string();
    let db_path = "write?db=responses".to_string();
    let _ = post_req(&db_path,&db_host,&"8086".to_string(),&body,&cont_l);
}

