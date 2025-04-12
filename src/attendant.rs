use futures_util::StreamExt;
use lapin::{options::*, Connection, ConnectionProperties};

#[tokio::main]
async fn main() {
    // connect to the RabbitMQ server
    let connection = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await
    .expect("failed to make connection with RabbitMQ server");

    // create a channel
    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create channel in attendant module");

    // declare a queue named "orders_ready"
    channel
        .queue_declare(
            "orders_ready",
            QueueDeclareOptions::default(),
            Default::default(),
        )
        .await
        .expect("Failed to decare the 'orders_ready' queue");

    // start consuming orders from kitchen's 'orders-ready' queue
    let mut consumer = channel
        .basic_consume(
            "orders_ready",                 // consuming from queue: orders_ready
            "attendants",                    // consumer tag
            BasicConsumeOptions::default(), // Default consumption option
            Default::default(),             // default additional options
        )
        .await
        .expect("Failed to create consumer");

    // process the orders
    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let message = String::from_utf8_lossy(&delivery.data);

            println!("Notification received: {}", message);

            // confirm the process msg by ackknowledging
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("failed to acknowledge notification");
        }
    }
}
