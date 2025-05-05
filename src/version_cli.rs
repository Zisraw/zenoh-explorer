mod topic_node;
mod ui;
use zenoh::{Config, sample::SampleKind};
use chrono::Local;
use colored::*;
use std::sync::{Arc, Mutex};
use topic_node::TopicNode;

#[tokio::main]
async fn main() {
    zenoh::init_log_from_env_or("error");

    println!("{}", "Opening Zenoh session...".blue());
    let session = zenoh::open(Config::default()).await.unwrap();

    println!("{}", "Subscribing to ** (all topics)...".green());
    let subscriber = session.declare_subscriber("**").await.unwrap();

    println!("{}", "Listening for messages (CTRL-C to quit)...\n".yellow());
    println!(
        "{:<25} | {:<10} | {:<30} | {}",
        "Time", "Kind", "Topic", "Payload"
    );
    println!("{}", "-".repeat(100));

    let topic_tree = Arc::new(Mutex::new(TopicNode::default()));

    while let Ok(sample) = subscriber.recv_async().await {
        let topic = sample.key_expr().as_str();
        let path: Vec<&str> = topic.split('/').collect();

        {
            // Met à jour l'arborescence
            let mut tree = topic_tree.lock().unwrap();
            tree.insert(&path);
        }
    
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let kind = match sample.kind() {
            SampleKind::Put => "PUT",
            SampleKind::Delete => "DEL",
            _ => "OTHER",
        };

        let payload = sample
            .payload()
            .try_to_string()
            .unwrap_or_else(|e| format!("(invalid UTF-8: {})", e).into());

        let topic = sample.key_expr().as_str();

        println!(
            "{:<25} | {:<10} | {:<30} | {}",
            now,
            kind,
            topic,
            payload
        );
    }
}
