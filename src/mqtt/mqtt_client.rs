use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use crate::db::AppState;
use crate::models::device::NewDevice;
use crate::repository::{device_repo, device_status_repo};
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use tokio::task;

pub async fn start_mqtt_client(state: Arc<AppState>) {
    let mut mqttoptions = MqttOptions::new("fota-server", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(10));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client.subscribe("device/+/register", QoS::AtLeastOnce).await.unwrap();
    client.subscribe("device/+/status", QoS::AtLeastOnce).await.unwrap();
    client.subscribe("device/+/update/result", QoS::AtLeastOnce).await.unwrap();

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

pub async fn handle_message(topic: String, payload: Vec<u8>, state: Arc<AppState>) {
    println!("📥 Received MQTT message on topic: {}", topic);

    if topic.contains("/register") {
        handle_register(topic, payload, state).await;
    } else if topic.contains("/status") {
        handle_device_status(topic, payload, state).await;
    } else {
        println!("⚠️ Unknown or unhandled topic: {}", topic);
    }
}

pub async fn handle_register(topic: String, payload: Vec<u8>, state: Arc<AppState>) {
    if let Ok(json) = serde_json::from_slice::<Value>(&payload) {
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
    } else {
        println!("❌ Failed to parse payload as JSON");
    }
}

async fn handle_device_status(
    topic: String,
    payload: Vec<u8>,
    state: Arc<AppState>,
) {
    println!("📥 Handling device status message");

    let parts: Vec<&str> = topic.split('/').collect();
    if parts.len() < 3 {
        println!("❌ Invalid topic format for status message");
        return;
    }
    let device_id = parts[1];

    match device_repo::device_exists(&state.db, device_id).await {
        Ok(true) => {
            println!("✅ Device {} exists, processing status...", device_id);
            // Parse payload raw JSON
            match serde_json::from_slice::<serde_json::Value>(&payload) {
                Ok(json) => {
                    // Save payload log
                    if let Err(e) = device_status_repo::save_device_status_log(
                        &state.db,
                        device_id,
                        json,
                    ).await {
                        println!("❌ Failed to save device status log: {:?}", e);
                    } else {
                        println!("✅ Saved device status log for device: {}", device_id);
                    }
                }
                Err(e) => {
                    println!("❌ Failed to parse device status payload: {:?}", e);
                }
            }
        }
        Ok(false) => {
            println!("❌ Device {} not registered, ignoring status message.", device_id);
        }
        Err(e) => {
            println!("❌ Error checking device existence: {:?}", e);
        }
    }
}

