use actix_web::{
    web::{self, Json},
    HttpResponse, Responder, ResponseError,
};
use log::{error, info};
use redis::Commands;
use std::{
    num::{NonZero, NonZeroUsize},
    sync::Arc,
    time::Duration,
};
extern crate redis;
use tokio::{
    sync::{mpsc, Notify},
    time::Instant,
};

use serde::{Deserialize, Serialize};

use crate::{
    // Ensure your custom error wrappers are imported
    error::RedisErrorWrapper,
    services::product::model::{NewProduct, Product},
};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum Update {
//     NewProduct(NewProduct),
//     UpdateProduct(Product),
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum StreamChannel {
//     ProductUpdates,
// }

// impl StreamChannel {
//     fn from_update(update: &Json<Update>) -> Self {
//         match **update {
//             Update::NewProduct(_) => StreamChannel::ProductUpdates,
//             Update::UpdateProduct(_) => StreamChannel::ProductUpdates,
//         }
//     }
// }

// impl Into<String> for StreamChannel {
//     fn into(self) -> String {
//         match self {
//             StreamChannel::ProductUpdates => "product_updates".to_string(),
//         }
//     }
// }

// impl Into<&str> for StreamChannel {
//     fn into(self) -> &'static str {
//         match self {
//             StreamChannel::ProductUpdates => "product_updates",
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UpdateStreamEvent {
//     pub update: Update,
//     pub attempt_count: usize,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Client {
//     pub mode: StreamMode,
//     pub connection_addr: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum StreamMode {
//     OnEvent,
//     OnSchedule(std::time::Duration),
// }

use redis::AsyncCommands;

async fn get_redis_connection(
    redis_url: String,
) -> redis::RedisResult<redis::aio::MultiplexedConnection> {
    let client = redis::Client::open(redis_url)?;
    client.get_multiplexed_async_connection().await
}

pub fn create_add_update_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/add_update", web::post().to(add_update))
        .route("/pause", web::post().to(pause_update_processor))
        .route("/start", web::post().to(start_update_processor));
}
async fn pause_update_processor(processor: web::Data<Arc<UpdateProcessor>>) -> impl Responder {
    log::info!("Pausing the update processor.");
    processor.pause().await;
    HttpResponse::Ok().json("Update Processor paused")
}

async fn start_update_processor(processor: web::Data<Arc<UpdateProcessor>>) -> impl Responder {
    log::info!("Starting the update processor.");
    processor.start().await;
    HttpResponse::Ok().json("Update Processor started")
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Update {
    NewProduct(NewProduct),
    UpdateProduct(Product),
}
async fn get_cached_data(key: &str, redis_url: &str) -> redis::RedisResult<Option<String>> {
    let mut conn = get_redis_connection(redis_url.to_string()).await?;
    let cached_data: Option<String> = conn.get(key).await?;
    Ok(cached_data)
}

async fn delete_cached_data(key: &str, redis_url: &str) -> redis::RedisResult<Option<String>> {
    let mut conn = get_redis_connection(redis_url.to_string()).await?;
    let cached_data: Option<String> = conn.del(key).await?;
    Ok(cached_data)
}

async fn set_cached_data(key: &str, data: &str, redis_url: &str) -> redis::RedisResult<()> {
    let mut conn = get_redis_connection(redis_url.to_string()).await?;
    let _: () = conn.set_ex(key, data, 60).await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamChannel {
    ProductUpdates,
}

impl StreamChannel {
    fn from_update(update: &Json<Update>) -> Self {
        match **update {
            Update::NewProduct(_) => StreamChannel::ProductUpdates,
            Update::UpdateProduct(_) => StreamChannel::ProductUpdates,
        }
    }
}

impl Into<String> for StreamChannel {
    fn into(self) -> String {
        match self {
            StreamChannel::ProductUpdates => "product_updates".to_string(),
        }
    }
}

impl Into<&str> for StreamChannel {
    fn into(self) -> &'static str {
        match self {
            StreamChannel::ProductUpdates => "product_updates",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateStreamEvent {
    pub update: Update,
    pub attempt_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub mode: StreamMode,
    pub connection_addr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamMode {
    OnEvent,
    OnSchedule(std::time::Duration),
}
async fn add_update(
    notify: web::Data<Arc<Notify>>,
    redis_url: web::Data<Arc<String>>,
    update: web::Json<Update>,
) -> impl Responder {
    let notify = notify.get_ref().clone();
    let redis_url = redis_url.get_ref().clone();
    let stream_key: &str = StreamChannel::from_update(&update).into();
    let update = update.into_inner();

    let update_with_attempts = UpdateStreamEvent {
        update,
        attempt_count: 0,
    };

    match write_to_stream_async(&redis_url, stream_key, update_with_attempts).await {
        Ok(_) => (),
        Err(e) => return RedisErrorWrapper(e).error_response(),
    }

    notify.notify_one();

    HttpResponse::Ok().json("Update added")
}
// async fn add_update(
//     notify: web::Data<Arc<Notify>>,
//     redis_url: web::Data<Arc<String>>,
//     tx: web::Data<mpsc::Sender<()>>,
//     updates_counter: web::Data<Arc<AtomicUsize>>,
//     update: web::Json<Update>,
// ) -> impl Responder {
//     let notify = notify.get_ref().clone();
//     let redis_url = redis_url.get_ref().clone();
//     let tx = tx.get_ref().clone();
//     let updates_counter = updates_counter.get_ref().clone();
//     let stream_key: &str = StreamChannel::from_update(&update).into();
//     let update = update.into_inner();

//     let update_with_attempts = UpdateStreamEvent {
//         update,
//         attempt_count: 0,
//     };

//     match write_to_stream_async(&redis_url, stream_key, update_with_attempts).await {
//         Ok(_) => (),
//         Err(e) => return RedisErrorWrapper(e).error_response(),
//     }

//     if let Err(_) = tx.send(()).await {
//         return HttpResponse::InternalServerError().body("Failed to send message");
//     }

//     notify.notify_one();
//     updates_counter.fetch_add(1, Ordering::SeqCst);

//     HttpResponse::Ok().json("Update added")
// }

async fn write_to_stream_async(
    redis_url: &str,
    key: &str,
    update: UpdateStreamEvent,
) -> redis::RedisResult<()> {
    let mut con = get_redis_connection(redis_url.to_string()).await?;
    con.lpush(
        key,
        serde_json::to_string(&update).map_err(|err| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                format!("{}", err),
            ))
        })?,
    )
    .await?;
    Ok(())
}

async fn read_from_stream_async(
    redis_url: &str,
    key: &str,
    count: Option<NonZeroUsize>,
) -> redis::RedisResult<Vec<UpdateStreamEvent>> {
    let mut con = get_redis_connection(redis_url.to_string()).await?;
    let serialized_updates: Vec<String> = con.rpop(key, count).await.unwrap_or_else(|_| Vec::new());

    let mut updates = Vec::new();
    for update_str in serialized_updates {
        let update = serde_json::from_str(&update_str).map_err(|err| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                format!("{}", err),
            ))
        })?;
        updates.push(update);
    }

    Ok(updates)
}
pub fn get_connection(connection_addr: &str) -> redis::RedisResult<redis::Connection> {
    let client = redis::Client::open(connection_addr)?;
    client.get_connection()
}

pub fn remove_from_stream(
    con: &mut redis::Connection,
    key: &str,
    update: UpdateStreamEvent,
    count: isize,
) -> redis::RedisResult<()> {
    con.lrem(
        key,
        count,
        serde_json::to_string(&update).map_err(|err| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                format!("{}", err),
            ))
        })?,
    )
}
async fn remove_from_stream_async(
    redis_url: &str,
    key: &str,
    update: UpdateStreamEvent,
) -> redis::RedisResult<()> {
    let mut con = get_redis_connection(redis_url.to_string()).await?;
    con.lrem(
        key,
        1,
        serde_json::to_string(&update).map_err(|err| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                format!("{}", err),
            ))
        })?,
    )
    .await?;
    Ok(())
}
pub fn write_to_stream(
    con: &mut redis::Connection,
    key: &str,
    update: UpdateStreamEvent,
) -> redis::RedisResult<()> {
    con.lpush(
        key,
        serde_json::to_string(&update).map_err(|err| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                format!("{}", err),
            ))
        })?,
    )
}

pub fn read_from_stream(
    con: &mut redis::Connection,
    key: &str,
) -> redis::RedisResult<Option<UpdateStreamEvent>> {
    let serialized_update: Option<String> = con.rpop(key, None)?;
    if let Some(update_str) = serialized_update {
        Ok(Some(serde_json::from_str(&update_str).map_err(|err| {
            redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Serialization error",
                format!("{}", err),
            ))
        })?))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub struct UpdateProcessor {
    redis_url: Arc<String>,
    notify: Arc<Notify>,
}

impl UpdateProcessor {
    pub async fn new(redis_url: String) -> Self {
        let redis_url = Arc::new(redis_url);
        let mut conn = get_redis_connection(redis_url.to_string()).await.unwrap();
        let _: () = conn.set("update_processor_active", true).await.unwrap();

        UpdateProcessor {
            redis_url,

            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn pause(&self) {
        let mut conn = get_redis_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        let _: () = conn.set("update_processor_active", false).await.unwrap();
        self.notify.notify_one(); // Wake up the processor loop if it's waiting
    }

    pub async fn start(&self) {
        let mut conn = get_redis_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        let _: () = conn.set("update_processor_active", true).await.unwrap();
        self.notify.notify_one(); // Wake up the processor loop if it's waiting
    }

    async fn is_active(&self) -> bool {
        let mut conn = get_redis_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        conn.get("update_processor_active").await.unwrap_or(false)
    }

    async fn get_batch_size(&self, key: &str) -> usize {
        let mut conn = get_redis_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        let list_size = conn.llen(key).await.unwrap_or(0) as usize;
        println!("Received list size: {}", list_size);
        list_size
    }

    pub async fn process_updates(
        &self,
        stream_client: Client,
        key: String,
        mut rx: mpsc::Receiver<()>,
    ) {
        let redis_url = stream_client.connection_addr.clone();

        loop {
            tokio::select! {
                _ = self.notify.notified() => {
                    if !self.is_active().await {
                        continue;
                    }
                    let batch_size = self.get_batch_size(&key).await;
                    let updates = read_from_stream_async(&redis_url, & key, NonZero::new(batch_size)).await.unwrap();
                    let start = Instant::now();
                    for update in updates {
                        Self::handle_update(&redis_url, &key, update.clone()).await;
                    }
                    let duration = start.elapsed();
                    Self::log_update_info(duration).await;
                },
                Some(_) = rx.recv() => {
                    if !self.is_active().await {
                        continue;
                    }
                    let batch_size = self.get_batch_size(&key).await;
                    let updates = read_from_stream_async(&redis_url, &key, NonZero::new(batch_size)).await.unwrap();
                    let start = Instant::now();
                    for update in updates {
                        Self::handle_update(&redis_url, &key, update.clone()).await;
                    }
                    let duration = start.elapsed();
                    Self::log_update_info(duration).await;
                },
                else => {
                    if self.is_active().await {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        }
    }

    async fn log_update_info(duration: Duration) {
        info!("Processing took: {}", duration.as_millis());
    }

    async fn handle_update(
        redis_url: &str,
        key: &str,
        mut update_with_attempts: UpdateStreamEvent,
    ) {
        let success = match update_with_attempts.update {
            Update::NewProduct(ref new_product) => {
                match crate::services::product::query::insert_product_query_pg(
                    new_product.clone(),
                    &mut super::db::new_connection(),
                ) {
                    Ok(result) => {
                        info!("Create completed: {:?}", result);
                        true
                    } // Simulate success
                    Err(err) => {
                        error!("Create failed: {:?}", err);
                        false
                    } // Simulate failure
                }
            }
            Update::UpdateProduct(ref product) => {
                match crate::services::product::query::set_product_query_pg(
                    product.clone(),
                    &mut super::db::new_connection(),
                ) {
                    Ok(result) => {
                        info!("Update completed: {:?}", result);
                        true
                    } // Simulate success
                    Err(err) => {
                        error!("Update failed: {:?}", err);
                        false
                    } // Simulate failure
                }
            }
        };

        if success {
            // remove_from_stream_async(redis_url, key, update_with_attempts)
            //     .await
            //     .unwrap();
            return;
        } else {
            update_with_attempts.attempt_count += 1;
            if update_with_attempts.attempt_count < 3 {
                let _serialized_update = serde_json::to_string(&update_with_attempts).unwrap();
                write_to_stream_async(redis_url, key, update_with_attempts)
                    .await
                    .unwrap();
            } else {
                println!("Update failed after 3 attempts");
            }
        }
    }
}
