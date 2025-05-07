use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use crate::errors::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub bio: Option<String>,
    pub interests: Vec<String>,
    pub connections: Vec<Connection>,
}

impl User {
    pub fn new(username: String, bio: Option<String>, interests: Vec<String>) -> Self {
        Self {
            username,
            bio,
            interests,
            connections: Vec::new(),
        }
    }

    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    pub fn has_similar_interests(&self, other: &User) -> Vec<String> {
        let my_interests: HashSet<_> = self.interests.iter().collect();
        let their_interests: HashSet<_> = other.interests.iter().collect();
        
        my_interests.intersection(&their_interests)
            .map(|s| (*s).clone())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub to_username: String,
    pub kind: ConnectionType,
    pub since: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionType {
    Mentor,
    Collaborator,
    Follower,
    ProjectBuddy,
}

#[derive(Debug, Default)]
pub struct CommunityGraph {
    pub members: HashMap<String, User>,
}

impl CommunityGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_user(&mut self, user: User) -> Result<()> {
        if self.members.contains_key(&user.username) {
            return Err(AppError::UserAlreadyExists(user.username));
        }
        self.members.insert(user.username.clone(), user);
        Ok(())
    }

    pub fn connect_users(
        &mut self, 
        from: &str, 
        to: &str, 
        kind: ConnectionType, 
        tags: Vec<String>, 
        since: String
    ) -> Result<()> {
        // Verify both users exist
        if !self.members.contains_key(from) {
            return Err(AppError::UserNotFound(from.to_string()));
        }
        if !self.members.contains_key(to) {
            return Err(AppError::UserNotFound(to.to_string()));
        }

        let connection = Connection {
            to_username: to.to_string(),
            kind,
            since,
            tags,
        };

        // Use get_mut to avoid multiple mutable borrows
        if let Some(user) = self.members.get_mut(from) {
            user.add_connection(connection);
            Ok(())
        } else {
            Err(AppError::ConnectionFailed(format!("Failed to connect {} to {}", from, to)))
        }
    }

    pub fn get_user(&self, username: &str) -> Result<&User> {
        self.members.get(username)
            .ok_or_else(|| AppError::UserNotFound(username.to_string()))
    }

    pub fn find_users_by_interest(&self, interest: &str) -> Vec<&User> {
        self.members.values()
            .filter(|user| user.interests.iter().any(|i| i == interest))
            .collect()
    }

    pub fn recommend_connections(&self, username: &str) -> Result<Vec<RecommendedConnection>> {
        let user = self.get_user(username)?;
        
        // Users this person is already connected to
        let connected_users: HashSet<&String> = user.connections
            .iter()
            .map(|conn| &conn.to_username)
            .collect();
        
        let mut recommendations = Vec::new();
        
        for (other_name, other_user) in self.members.iter() {
            // Skip self or already connected users
            if other_name == &username || connected_users.contains(other_name) {
                continue;
            }
            
            // Find shared interests
            let shared_interests = user.has_similar_interests(other_user);
            if !shared_interests.is_empty() {
                recommendations.push(RecommendedConnection {
                    username: other_name.clone(),
                    shared_interests,
                    connection_type: recommend_connection_type(user, other_user),
                });
            }
        }
        
        // Sort by number of shared interests (descending)
        recommendations.sort_by(|a, b| b.shared_interests.len().cmp(&a.shared_interests.len()));
        
        Ok(recommendations)
    }
}

#[derive(Debug, Serialize)]
pub struct RecommendedConnection {
    pub username: String,
    pub shared_interests: Vec<String>,
    pub connection_type: ConnectionType,
}

// Helper function to recommend connection type based on user profiles
fn recommend_connection_type(user: &User, other: &User) -> ConnectionType {
    // This is a simple heuristic - could be made more sophisticated
    let shared_interests = user.has_similar_interests(other);
    
    if shared_interests.len() > 3 {
        ConnectionType::ProjectBuddy
    } else if other.connections.len() > user.connections.len() * 2 {
        ConnectionType::Mentor
    } else if user.connections.len() > other.connections.len() * 2 {
        ConnectionType::Follower
    } else {
        ConnectionType::Collaborator
    }
}