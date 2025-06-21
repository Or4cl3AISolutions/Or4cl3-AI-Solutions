// or4cl3_core/src/astraea/mod.rs

//! ASTRÆA (Autonomous Multi-Agent Cognitive Mesh)
//! Manages a network of autonomous agents, facilitating communication,
//! coordination, and collective intelligence or problem-solving.

use std::collections::HashMap;

/// Represents an individual agent in the mesh.
#[derive(Debug, Clone, PartialEq)] // Added PartialEq for easier testing
pub struct AgentState {
    pub agent_id: String,
    pub status: String, // e.g., "active", "idle", "processing"
    pub current_task: Option<String>,
    pub capabilities: Vec<String>,
}

/// Trait for managing and coordinating a mesh of autonomous agents.
pub trait CognitiveMeshCoordinator {
    fn register_agent(&mut self, agent_id: String, capabilities: Vec<String>) -> Result<(), String>;
    fn assign_task_to_agent(&mut self, agent_id: &str, task_description: &str) -> Result<(), String>;
    fn get_agent_state(&self, agent_id: &str) -> Result<AgentState, String>;
    fn broadcast_message_to_mesh(&self, message: &str) -> Result<(), String>;
}

pub struct AstraeaCoordinator {
    agents: HashMap<String, AgentState>,
}

impl AstraeaCoordinator {
    pub fn new() -> Self {
        AstraeaCoordinator { agents: HashMap::new() }
    }
}

impl CognitiveMeshCoordinator for AstraeaCoordinator {
    fn register_agent(&mut self, agent_id: String, capabilities: Vec<String>) -> Result<(), String> {
        if self.agents.contains_key(&agent_id) {
            return Err(format!("Agent {} already registered", agent_id));
        }
        let state = AgentState {
            agent_id: agent_id.clone(),
            status: "idle".to_string(),
            current_task: None,
            capabilities,
        };
        self.agents.insert(agent_id, state);
        Ok(())
    }

    fn assign_task_to_agent(&mut self, agent_id: &str, task_description: &str) -> Result<(), String> {
        let agent_state = self.agents.get_mut(agent_id).ok_or_else(|| format!("Agent {} not found", agent_id))?;
        agent_state.current_task = Some(task_description.to_string());
        agent_state.status = "processing".to_string();
        Ok(())
    }

    fn get_agent_state(&self, agent_id: &str) -> Result<AgentState, String> {
        self.agents.get(agent_id).cloned().ok_or_else(|| format!("Agent {} not found", agent_id))
    }

    fn broadcast_message_to_mesh(&self, _message: &str) -> Result<(), String> {
        // Mock: In a real system, this would iterate and send messages.
        // For now, just print or log if needed.
        // println!("Broadcasting message: {}", _message);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astraea_registration_and_get_state() {
        let mut astraea = AstraeaCoordinator::new();
        let agent_id = "agent1".to_string();
        let capabilities = vec!["data_analysis".to_string(), "learning".to_string()];

        astraea.register_agent(agent_id.clone(), capabilities.clone()).unwrap();

        let state = astraea.get_agent_state(&agent_id).unwrap();
        assert_eq!(state.agent_id, agent_id);
        assert_eq!(state.capabilities, capabilities);
        assert_eq!(state.status, "idle");
        assert_eq!(state.current_task, None);
    }

    #[test]
    fn test_astraea_assign_task() {
        let mut astraea = AstraeaCoordinator::new();
        let agent_id = "agent2".to_string();
        astraea.register_agent(agent_id.clone(), vec!["processing".to_string()]).unwrap();

        let task_desc = "process dataset X";
        astraea.assign_task_to_agent(&agent_id, task_desc).unwrap();

        let state = astraea.get_agent_state(&agent_id).unwrap();
        assert_eq!(state.status, "processing");
        assert_eq!(state.current_task, Some(task_desc.to_string()));
    }

    #[test]
    fn test_astraea_get_nonexistent_agent() {
        let astraea = AstraeaCoordinator::new();
        assert!(astraea.get_agent_state("unknown_agent").is_err());
    }

    #[test]
    fn test_astraea_register_duplicate_agent() {
        let mut astraea = AstraeaCoordinator::new();
        let agent_id = "agent3".to_string();
        astraea.register_agent(agent_id.clone(), vec![]).unwrap();
        assert!(astraea.register_agent(agent_id, vec![]).is_err());
    }

    #[test]
    fn test_astraea_broadcast() {
        let astraea = AstraeaCoordinator::new();
        // Simple check that it doesn't panic
        assert!(astraea.broadcast_message_to_mesh("Test message").is_ok());
    }
}
