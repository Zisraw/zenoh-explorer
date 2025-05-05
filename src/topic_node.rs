use std::collections::HashMap;
use chrono::{DateTime, Local};

/// Represents a node in the topic tree.
/// Each node can have children (subtopics) and store messages with timestamps.
#[derive(Debug, Default)]
pub struct TopicNode {
    pub name: String,
    pub children: HashMap<String, TopicNode>,
    pub is_leaf: bool,
    pub messages: Vec<(DateTime<Local>, String)>,
}

impl TopicNode {
    /// Recursively inserts a path into the topic tree.
    /// Returns a mutable reference to the final node in the path.
    pub fn insert(&mut self, path: &[&str]) -> &mut TopicNode {
        if path.is_empty() {
            self.is_leaf = true;
            return self;
        }

        let part = path[0];
        // Insert the child node if it doesn't exist, or get a mutable reference to it
        let child = self.children.entry(part.to_string()).or_insert_with(|| TopicNode {
            name: part.to_string(),
            ..Default::default()
        });

        child.insert(&path[1..])
    }

    /// Adds a message to the node at the given path.
    /// If the path does not exist, it is created.
    pub fn add_message(&mut self, path: &[&str], content: String) {
        let node = self.insert(path);
        let now = Local::now();
        node.messages.push((now, content));
    }
}
