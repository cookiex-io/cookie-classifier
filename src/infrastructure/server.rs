use std::{collections::HashMap, sync::Arc};
use futures::StreamExt;
use mongodb::{bson::doc, Client, Collection};
use crate::model::open_list::{OpenCookie, OpenTracker};
use super::database::get_client;

pub async fn initialize_db_client_and_cache() -> mongodb::error::Result<(Client,Arc<HashMap<String,OpenCookie>>,Arc<HashMap<String,OpenTracker>>)> {
    let client = get_client().await?;
    let mut open_cookies_cache:HashMap<String,OpenCookie> = HashMap::default();
    let mut open_trackers_cache:HashMap<String,OpenTracker> = HashMap::default();
    let open_cookies_collection:Collection<OpenCookie> = client.database("cmp").collection("open_cookies_collection");
    let mut open_cookies_cursor = open_cookies_collection.find(doc!{}).await?;
    while let Some(open_cookie) = open_cookies_cursor.next().await{
         match open_cookie {
             Ok(cookie) => {open_cookies_cache.insert(cookie.cookie.clone(), cookie);},
             Err(_e) => continue,
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
    Ok((client,Arc::new(open_cookies_cache),Arc::new(open_trackers_cache))) 
  }