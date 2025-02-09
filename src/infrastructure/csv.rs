use std::sync::Arc;
use reqwest::Client as ReqClient;
use anyhow::Error;
use serde::Deserialize;

pub async fn get_list_from_csv<T:for<'a> Deserialize<'a>>(req_client:&Arc<ReqClient>,url:&str) -> Result<Vec<T>,Error>{
    let response = req_client
     .get(url)
     .send()
     .await?;    
    let txt = response
     .text()
     .await?;
    let mut rdr = csv::Reader::from_reader(txt.as_bytes());
    let mut records:Vec<T> = Vec::new();
    for result in rdr.deserialize() {
        let record:T = result?;
        records.push(record);
    }
 Ok(records)
}