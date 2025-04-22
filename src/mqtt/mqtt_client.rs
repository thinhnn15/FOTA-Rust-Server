use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use crate::db::AppState;
use crate::models::device::NewDevice;
use crate::repository::device_repo;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

pub async fn start_mqtt_client(state: Arc<AppState>) {
    let mut mqttoptions = MqttOptions::new("fota-server", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("device/+/register", QoS::AtLeastOnce).await.unwrap();

    task::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(publish))) => {
                    handle_message(publish.topic, publish.payload.to_vec(), state.clone()).await;
                }
                Ok(_) => {}
                Err(e) => {
                    println!("❌ MQTT connection error: {:?}", e);
                    break;
                }
            }
            
        }
    });
}

async fn handle_message(topic: String, payload: Vec<u8>, state: Arc<AppState>) {
    println!("📥 Received MQTT message on topic: {}", topic);

    if let Ok(json) = serde_json::from_slice::<Value>(&payload) {
        if topic.contains("/register") {
            match serde_json::from_value::<NewDevice>(json) {
                Ok(new_device) => {
                    match device_repo::create_device(&state.db, new_device).await {
                        Ok(_) => println!("✅ Device registered successfully!"),
                        Err(e) => println!("❌ Failed to insert device: {:?}", e),
                    }
                }
                Err(e) => {
                    println!("❌ Failed to parse JSON to NewDevice: {:?}", e);
                }
            }
        }
    } else {
        println!("❌ Failed to parse payload as JSON (serde_json::from_slice)");
    }
}

