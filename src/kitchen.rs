use futures_util::StreamExt;
use lapin::{options::*, BasicProperties, Connection, ConnectionProperties};
use serde_json::Value;

#[tokio::main]
async fn main() {
    // connect to the RabbitMQ server
    let connection = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await
    .expect("failed to connect RabbitMQ server from kitchen");

    // create communication channel
    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create channel in kitchen module");

    // start consuming orders from "Orders" queue
    let mut consumer = channel
        .basic_consume(
            "orders",                       // consuming from queue: orders
            "kitchen",                      // consumer tag (identifier)
            BasicConsumeOptions::default(), // default consumption options
            Default::default(),             // default additional arguments
        )
        .await
        .expect("Failed to start consuming message");

    println!("Kitchen is Open. Waiting for orders..");

    // process msg from the queue
    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            // convert the msg body to a string
            let message = String::from_utf8_lossy(&delivery.data);

            // parse the msg into a json obj
            let mut order: Value =
                serde_json::from_str(&message).expect("failed to parse json msg");

            println!("Order received in the kitchen: {}", order);

            // update the status of the order
            order["status"] = "Ready".into();

            // publish the updated order to the "order_ready" queue
            channel
                .basic_publish(
                    "",                             // Exchange (empty strin o)
                    "orders_ready",                 // routing Key (queue name for processed orders)
                    BasicPublishOptions::default(), // Default publishing options
                    order.to_string().as_bytes(),   // msg body as a byte array
                    BasicProperties::default(),     // default msg properties
                )
                .await
                .expect("failed to publish the updated order");

            println!("Order prepared: {}", order);

            // ackknowledge the msg as processed
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ackknowledge the msg");
        }
    }
}
