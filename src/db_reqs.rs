#[path="posts.rs"]
mod posts;

use std::string::String;
use posts::*;


pub fn create_db(db_host_ip: &String,db_port:&String){
    let db_path = "query?q=CREATE+DATABASE+responses".to_string();
    let _ = post_req(&db_path,&db_host_ip,&db_port,&"".to_string(),&"".to_string());
}

pub fn send_data_to_db(db_host_ip: &String,db_port:&String, status: &str, resp_time: &String,db_meas_name: &String){
    let body = format!("{},tag=tag resp_status={},resp_time(s)={}",db_meas_name,status,resp_time);
    let cont_l = format!("Content-length: {}\n",body.len()+2);
    let db_path = "write?db=responses".to_string();
    let _ = post_req(&db_path,&db_host_ip,&db_port,&body,&cont_l);
}

