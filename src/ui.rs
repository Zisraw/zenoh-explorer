use std::sync::{Arc, Mutex};
use std::thread;

use eframe::egui;
use zenoh:: Config;
use crate::topic_node::TopicNode;

/// Main application struct holding the root of the topic tree
pub struct App {
    topic_root: Arc<Mutex<TopicNode>>,
}

impl Default for App {
    fn default() -> Self {
        // Initialize the root node of the topic tree
        let topic_root = Arc::new(Mutex::new(TopicNode {
            name: "root".to_string(),
            ..Default::default()
        }));

        let tree_clone = topic_root.clone();

        // Spawn a background thread to handle Zenoh subscriptions
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                zenoh::init_log_from_env_or("error");
                let session = zenoh::open(Config::default()).await.unwrap();
                
                // Subscribe to all topics
                let subscriber = session.declare_subscriber("**").await.unwrap();
                
                // Listen for incoming samples/messages
                while let Ok(sample) = subscriber.recv_async().await {
                    let topic = sample.key_expr().as_str();
                    let path: Vec<&str> = topic.split('/').collect();
                    
                    // Try to convert the payload to a String
                    let payload = sample.payload()
                        .try_to_string()
                        .map(|cow| cow.into_owned())
                        .unwrap_or_else(|e| e.to_string());

                    // Lock the tree and add the received message
                    let mut tree = tree_clone.lock().unwrap();
                    tree.add_message(&path, payload);
                }
            });
        });

        App { topic_root }
    }
}

impl App {
    pub fn new(config_path: Option<&str>) -> Self {
        let topic_root = Arc::new(Mutex::new(TopicNode {
            name: "root".to_string(),
            ..Default::default()
        }));

        let tree_clone = topic_root.clone();

        // load config file
        let config = config_path
            .map(|path| zenoh::Config::from_file(path).unwrap_or_else(|e| {
                eprintln!("Failed to load config '{}': {}", path, e);
                std::process::exit(1);
            }))
            .unwrap_or_else(zenoh::Config::default);

        // Thread Zenoh
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async move {
                zenoh::init_log_from_env_or("error");
                
                match zenoh::open(config).await {
                    Ok(session) => {
                        match session.declare_subscriber("**").await {
                            Ok(subscriber) => {
                                while let Ok(sample) = subscriber.recv_async().await {
                                    let topic = sample.key_expr().as_str();
                                    let path: Vec<&str> = topic.split('/').collect();
                                    let payload = sample.payload()
                                        .try_to_string()
                                        .map(|cow| cow.into_owned())
                                        .unwrap_or_else(|e| e.to_string());
    
                                    let mut tree = tree_clone.lock().unwrap();
                                    tree.add_message(&path, payload);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to subscribe to topics: {}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to open Zenoh session: {}", e);
                        std::process::exit(1);
                    }
                }
            });
        });

        App { topic_root }
    }
}

/// Implement the eframe App trait for GUI rendering
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Zenoh Explorer");

            // Lock the tree and draw it in the UI
            let tree = self.topic_root.lock().unwrap();
            draw_tree(ui, &tree, 0);
        });
    }
}

/// Recursive function to draw the topic tree in the UI
fn draw_tree(ui: &mut egui::Ui, node: &TopicNode, indent: usize) {
    let indent_width = 15.0;
    ui.horizontal(|ui| {
        // Indent child nodes visually
        ui.add_space(indent as f32 * indent_width);

        if !node.children.is_empty() {
            // If the node has children, show a collapsible header
            egui::CollapsingHeader::new(&node.name)
                .default_open(false)
                .show(ui, |ui| {
                    for child in node.children.values() {
                        draw_tree(ui, child, indent + 1);
                    }
                });
        } else {
            // If the node is a leaf, show its messages in a collapsible header
            egui::CollapsingHeader::new(&node.name)
                .default_open(false)
                .show(ui, |ui| {
                    if node.messages.is_empty() {
                        ui.label("No message received");
                    } else {
                        // Display each message with its timestamp
                        for (ts, msg) in &node.messages {
                            ui.label(format!("{}: {}", ts.format("%H:%M:%S"), msg));
                        }
                    }
                });
        }
    });
}
