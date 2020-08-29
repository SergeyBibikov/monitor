#[path="posts.rs"]
mod posts;

use std::string::String;
use posts::*;


pub fn create_db(db_host_ip: &String){
    let db_path = "query?q=CREATE+DATABASE+responses".to_string();
    let _ = post_req(&db_path,&db_host_ip,&"8086".to_string(),&"".to_string(),&"".to_string());
}

pub fn send_data_to_db(db_host_ip: &String, status: &str, resp_time: &String){
    let body = format!("resp_data,resp_status={} resp_time(secs)={}",status,resp_time);
    let cont_l = format!("Content-length: {}\n",body.len()+2);
    let db_path = "write?db=responses".to_string();
    let _ = post_req(&db_path,&db_host_ip,&"8086".to_string(),&body,&cont_l);
}

