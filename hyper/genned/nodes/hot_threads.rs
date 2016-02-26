//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 26);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cluster/nodes/hotthreads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 27);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cluster/nodes/hot_threads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id(client: &'a mut hyper::Client, base: String, node_id: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + 11 + node_id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cluster/nodes/");
    url_fmtd.push_str(&node_id);
    url_fmtd.push_str("/hotthreads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id(client: &'a mut hyper::Client, base: String, node_id: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + 12 + node_id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_cluster/nodes/");
    url_fmtd.push_str(&node_id);
    url_fmtd.push_str("/hot_threads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 18);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_nodes/hotthreads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 19);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_nodes/hot_threads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id(client: &'a mut hyper::Client, base: String, node_id: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + 11 + node_id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(&node_id);
    url_fmtd.push_str("/hotthreads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_node_id(client: &'a mut hyper::Client, base: String, node_id: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 8 + 12 + node_id.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_nodes/");
    url_fmtd.push_str(&node_id);
    url_fmtd.push_str("/hot_threads");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
