use std::{collections::HashMap, sync::Arc};
use anyhow::Error;
use futures::StreamExt;
use mongodb::{bson::doc, Client, Collection};
use reqwest::Client as ReqClient;
use crate::{dto::csv_dto::{OpenCookieCsvColumn, OpenTrackerCsvColumn}, model::open_list::{OpenCookie, OpenTracker}};
use super::{csv::get_list_from_csv, database::get_client};
use lazy_static::lazy_static;

lazy_static!{
    pub static ref OPEN_COOKIES_CSV_URL:String = std::env::var("OPEN_COOKIES_CSV_URL").expect("Must set OPEN_COOKIES_CSV_URL");
    pub static ref OPEN_TRACKERS_CSV_URL:String = std::env::var("OPEN_TRACKERS_CSV_URL").expect("Must set OPEN_TRACKERS_CSV_URL");
}

pub async fn initialize_db_client_and_cache(req_client:&Arc<ReqClient>) -> Result<(Client,Arc<HashMap<String,OpenCookie>>,Arc<HashMap<String,OpenTracker>>),Error> {
    let client = get_client().await?;
    let open_cookies_csv_list:Vec<OpenCookieCsvColumn> = get_list_from_csv(req_client,OPEN_COOKIES_CSV_URL.as_str()) .await?;
    let open_trackers_csv_list:Vec<OpenTrackerCsvColumn> = get_list_from_csv(req_client,OPEN_TRACKERS_CSV_URL.as_str()).await?;
    println!("Loaded .csv files Cookies -> {} Trackers {}",open_cookies_csv_list.len(),open_trackers_csv_list.len());
    let mut open_cookies_cache:HashMap<String,OpenCookie> = HashMap::new();
    let mut open_trackers_cache:HashMap<String,OpenTracker> = HashMap::new();
    let open_cookies_collection:Collection<OpenCookie> = client.database("cmp").collection("open_cookies_collection");
    let mut open_cookies_cursor = open_cookies_collection.find(doc!{}).await?;
    while let Some(open_cookie) = open_cookies_cursor.next().await{
         match open_cookie {
             Ok(cookie) => {open_cookies_cache.insert(cookie.cookie.clone(), cookie);},
             Err(_e) => continue,
        }
    }
    for open_cookie_csv in &open_cookies_csv_list{
        if open_cookies_cache.get(&open_cookie_csv.name).is_none(){
           let open_cookie = open_cookie_csv.to_cookie();
           open_cookies_collection.insert_one(&open_cookie).await?;
           open_cookies_cache.insert(open_cookie.cookie.clone(), open_cookie);
        }
    }
    let open_trackers_collection:Collection<OpenTracker> = client.database("cmp").collection("open_trackers_collection");
    let mut open_tracker_cursor = open_trackers_collection.find(doc!{}).await?;
    while let Some(open_tracker) = open_tracker_cursor.next().await{
        match open_tracker {
            Ok(tracker) => {open_trackers_cache.insert(tracker.domain.clone(), tracker);},
            Err(_e) => continue,
        }
    }
    for open_tracker_csv in &open_trackers_csv_list{
        if open_trackers_cache.get(&open_tracker_csv.domain).is_none(){
           let open_tracker = open_tracker_csv.to_tracker();
           open_trackers_collection.insert_one(&open_tracker).await?;
           open_trackers_cache.insert(open_tracker.domain.clone(), open_tracker);
        }
    }
  Ok((client,Arc::new(open_cookies_cache),Arc::new(open_trackers_cache))) 
}