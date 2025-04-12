use lapin::{options::*, BasicProperties, Connection, ConnectionProperties};
use serde_json::json;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // connect to RabbitMQ server
    let connection = Connection::connect(
        "amqp://guest:guest@localhost:5672",
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ from customers");

    //create a communication channel
    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create a channel in customer module");

    // declare a queue named "orders"
    channel
        .queue_declare(
            "orders",                       // channel name
            QueueDeclareOptions::default(), // default queue options
            Default::default(),             // default addtional arguments
        )
        .await
        .expect("Failed to declare the 'orders' queue ");

    // sending orders to the queue
    for id in 1..=5 {
        let order = json!({
            "order_id" : id,                         // order-id
            "items" : ["pizza", "Soda"],             // items in the order
            "status" : "New"                         // initial status of the order
        });

        // publish the order msg to the "orders" queue
        channel
            .basic_publish(
                "",                             // exchange (empty string for default exchange)
                "orders",                       // routing key (queue name)
                BasicPublishOptions::default(), // default publishing options
                order.to_string().as_bytes(),   // msg body as a byte array
                BasicProperties::default(),     // default msg properties
            )
            .await
            .expect("Failed to send order");

        // log the sent order to the console
        println!("Order sent: {}", order);

        // wait for 5 sec before sending the next order
        sleep(Duration::from_secs(5)).await;
    }
}
