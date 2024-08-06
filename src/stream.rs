use actix_web::{
    web::{self, Json},
    HttpResponse, Responder, ResponseError,
};
use diesel::PgConnection;
use log::{error, info};
use redis::{AsyncCommands, Commands};
use std::{
    num::{NonZero, NonZeroUsize},
    ops::Deref,
    sync::Arc,
    time::Duration,
};
extern crate redis;
use crate::redis::multiplexed_async_connection;
use serde::{Deserialize, Serialize};
use tokio::{
    sync::{mpsc, Notify},
    time::Instant,
};

use crate::{
    error::RedisErrorWrapper,
    services::product::model::{NewProduct, Product},
};

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
    pub data: Update,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamMode {
    OnEvent,
    OnSchedule(std::time::Duration),
}
async fn add_update(
    notify: web::Data<Arc<Notify>>,
    redis_url: web::Data<Arc<String>>,
    payload: web::Json<Update>,
) -> impl Responder {
    match write_to_stream_async(
        &redis_url.get_ref(),
        StreamChannel::from_update(&payload).into(),
        UpdateStreamEvent {
            data: payload.into_inner(),
        },
    )
    .await
    {
        Ok(_) => (),
        Err(e) => return RedisErrorWrapper(e).error_response(),
    }

    notify.notify_one();

    HttpResponse::Ok().json("Update added")
}

async fn write_to_stream_async(
    redis_url: &str,
    key: &str,
    update: UpdateStreamEvent,
) -> redis::RedisResult<()> {
    let mut con = multiplexed_async_connection(redis_url.to_string()).await?;
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
    let mut con = multiplexed_async_connection(redis_url.to_string()).await?;
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
pub async fn remove_from_stream_async(
    redis_url: &str,
    key: &str,
    update: UpdateStreamEvent,
) -> redis::RedisResult<()> {
    let mut con = multiplexed_async_connection(redis_url.to_string()).await?;
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
        let mut conn = multiplexed_async_connection(redis_url.to_string())
            .await
            .unwrap();
        let _: () = conn.set("update_processor_active", true).await.unwrap();

        UpdateProcessor {
            redis_url,

            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn pause(&self) {
        let mut conn = multiplexed_async_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        let _: () = conn.set("update_processor_active", false).await.unwrap();
        self.notify.notify_one(); // Wake up the processor loop if it's waiting
    }

    pub async fn start(&self) {
        let mut conn = multiplexed_async_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        let _: () = conn.set("update_processor_active", true).await.unwrap();
        self.notify.notify_one(); // Wake up the processor loop if it's waiting
    }

    async fn is_active(&self) -> bool {
        let mut conn = multiplexed_async_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        conn.get("update_processor_active").await.unwrap_or(false)
    }

    async fn get_batch_size(&self, key: &str) -> usize {
        let mut conn = multiplexed_async_connection(self.redis_url.as_ref().to_string())
            .await
            .unwrap();
        let list_size = conn.llen(key).await.unwrap_or(0) as usize;
        println!("Received list size: {}", list_size);
        list_size
    }
    async fn process_batch(&self, key: &str) {
        let batch_size = self.get_batch_size(&key).await;
        let updates = read_from_stream_async(&self.redis_url, &key, NonZero::new(batch_size))
            .await
            .unwrap();
        let mut connection = super::postgres::new_connection().unwrap();
        let start = Instant::now();
        for update in updates {
            Self::handle_update(&mut connection, update.clone()).await;
        }
        let duration = start.elapsed();
        Self::log_update_info(duration).await;
    }
    pub async fn process_updates(&self, key: String, mut rx: mpsc::Receiver<()>) {
        let key_clone = key.clone();
        loop {
            tokio::select! {
                            _ = self.notify.notified() => {
                                if !self.is_active().await {
                                    continue;
                                }
                               else {
            self.process_batch(key_clone.deref()).await;
                                }
                            },
                            Some(_) = rx.recv() => {
                                if !self.is_active().await {
                                    continue;
                                } else {
            self.process_batch(key_clone.deref()).await;
                                }

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

    async fn handle_update(con: &mut PgConnection, update: UpdateStreamEvent) {
        match update.data {
            Update::NewProduct(ref new_product) => {
                match crate::services::product::query::insert_product_query(
                    new_product.clone(),
                    con,
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
                let mut connection = super::postgres::new_connection().unwrap();
                match crate::services::product::query::set_product_query(
                    product.clone(),
                    &mut connection,
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
    }
}
