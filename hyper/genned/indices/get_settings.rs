//Autogenerated

use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get(client: &'a mut hyper::Client, base: String) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 10);
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_settings");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index(client: &'a mut hyper::Client, base: String, index: String)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 10 + index.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_settings");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_name(client: &'a mut hyper::Client, base: String, index: String,
                  name: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 11 + index.len() + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(&index);
    url_fmtd.push_str("/_settings/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_name(client: &'a mut hyper::Client, base: String, name: String)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 11 + name.len());
    url_fmtd.push_str(&base);
    url_fmtd.push_str("/_settings/");
    url_fmtd.push_str(&name);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
