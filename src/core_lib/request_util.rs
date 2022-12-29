use bytes::Bytes;
use http::StatusCode;
use serde::Deserialize;
use spin_sdk::http::{Request, Response};
use anyhow::{Result, Error};
use super::surrealdb_connector::SurrealdbResponse;
pub fn extract_from_request_body<T: for<'a> Deserialize<'a>>(request: &Request) -> T {
    let body = request.body().clone().unwrap();
    let req_body = String::from_utf8(body.to_vec().clone()).unwrap();
    let user_data = serde_json::from_str::<T>(req_body.as_str()).unwrap();
    user_data
}

pub fn has_internal_headers(request: &Request) -> bool {
    request.headers().contains_key("x-api-internal")
}

pub fn extract_from_surrealdb_response<T: for<'a> Deserialize<'a>>(surreadl_response: &Response) -> Option<Bytes>  {    
    let body = surreadl_response.body().clone().unwrap();    
    let body_string = String::from_utf8(body.to_vec().clone());
    
    let surreal_response = match body_string {
        Ok(b_string) => {
                        
            let extract_response = serde_json::from_str::<Vec<SurrealdbResponse>>(b_string.as_str());
            
            let data = match extract_response{
                Ok(_responses) => {
                    let final_response = _responses.first().unwrap();
                    let mut final_data = None;
                    if final_response.status == "OK" {                        
                        
                        final_data = Some(body)
                    } else {
                        final_data = None;
                    }
                    final_data
                    
                },
                Err(_) => None,
            };
            data
        }
        Err(_) => {
            println!("error");
            None
        }
    };
    surreal_response
    
    
}

pub fn get_data_from_surreal_response<T: for<'a> Deserialize<'a> >(surreal_response: &SurrealdbResponse) -> Result<Vec<T>, Error> {
    let result = surreal_response.result.clone();
    let data = result.iter().map(|val| serde_json::from_value::<T>(val.clone()));
    let final_data = data.collect::<Result<Vec<T>, _>>()?;
    Ok(final_data)
}

pub fn send_response(status_code: &StatusCode) -> anyhow::Result<Response> {
    Ok(http::Response::builder()
    .status(status_code.as_u16())
    .body(Some(Bytes::from("Hello, world!")))
    .unwrap())
}
