use paho_mqtt as mqtt;
use serde_json::json;
use std::{process, thread, time::Duration};
use sysinfo::{ProcessorExt, System, SystemExt};

const BROKER: &str = "tcp://broker.emqx.io:1883";
const TOPIC: &str = "test/topic123";
const CLIENT_ID: &str = "rust-123";

fn get_cpu_load(system: &mut System) -> f32 {
    system.refresh_cpu();
    let cpu = system.global_processor_info();
    cpu.cpu_usage()
}

fn main() {
    // Initialize the system for CPU usage
    let mut system = System::new_all();

    // Create MQTT client options
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(BROKER)
        .client_id(CLIENT_ID)
        .finalize();

    let mut client = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        eprintln!("Error creating the client: {}", err);
        process::exit(1);
    });

    // Set up connection options
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    // Connect to the broker
    if let Err(err) = client.connect(conn_opts) {
        eprintln!("Unable to connect: {}", err);
        process::exit(1);
    }

    // Subscribe to the topic
    if let Err(err) = client.subscribe(TOPIC, 1) {
        eprintln!("Error subscribing to topic: {}", err);
        process::exit(1);
    }

    // Set up a callback for incoming messages
    client.set_message_callback(|_client, msg| {
        if let Some(msg) = msg {
            println!("Received message: {} from topic: {}", msg.payload_str(), msg.topic());
        }
    });

    // Publish messages every 5 seconds
    let publish_client = client.clone();
    thread::spawn(move || {
        loop {
            let cpu_load = get_cpu_load(&mut System::new_all());
            let message = json!({ "CPU": format!("{:.2}", cpu_load) });

            if let Err(err) = publish_client.publish(mqtt::Message::new(TOPIC, message.to_string(), 1)) {
                eprintln!("Error publishing message: {}", err);
            }

            thread::sleep(Duration::from_secs(5));
        }
    });

    // Wait for termination signal
    ctrlc::set_handler(move || {
        println!("Exiting...");
        client.disconnect(None).unwrap();
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}export OPENSSL_ROOT_DIR=$(brew --prefix openssl)