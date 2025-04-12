# Queue-task

A personal learning project to explore and understand RabbitMQ functionalities.

## Overview

This project serves as a hands-on learning experience with RabbitMQ, focusing on understanding message queuing concepts and implementations.

## Learning Objectives

- Understanding basic RabbitMQ concepts
- Implementing different messaging patterns
- Working with queues, exchanges, and bindings
- Exploring message persistence and reliability
- Learning about pub/sub patterns

## Project Structure

The project consists of three main components:

- `customer.rs`: Simulates customers placing orders
- `kitchen.rs`: Processes incoming orders and marks them as ready
- `attendant.rs`: Notifies when orders are ready for pickup

## Setup

1. Install Docker and Docker Compose
2. Clone this repository
3. Run `docker-compose up` to start RabbitMQ
4. Build and run the components:

   ```bash
   cargo run --bin customer
   cargo run --bin kitchen
   cargo run --bin attendant
   ```

## Resources

- [RabbitMQ Official Documentation](https://www.rabbitmq.com/documentation.html)
- [RabbitMQ Tutorials](https://www.rabbitmq.com/getstarted.html)
