// or4cl3_core/src/conversational_interface/mod.rs
//! Defines the structures and traits for the OR4CL3 Conversational Interface.
//! This interface handles user interactions, processes queries through the
//! Recursive Cognition Engine, and formulates system responses that include
//! self-reflective behaviors and ethical alignment indicators.

use std::collections::HashMap;
use crate::recursive_cognition_engine::{CognitiveState, EthicalAssessmentReport, Stimulus, StimulusContent, RecursiveCognitionEngine, HumanFeedback}; // Assuming access to the engine trait

// --- Data Structures for Interaction ---

#[derive(Debug, Clone)]
pub struct UserQuery {
    pub session_id: String,
    pub query_id: String,
    pub text: String,
    pub metadata: Option<HashMap<String, String>>, // e.g., user preferences, location
}

#[derive(Debug, Clone)]
pub enum RichContentElement {
    Button { label: String, action: String }, // action could be a payload or command
    Card { title: String, text: String, image_url: Option<String>, actions: Option<Vec<RichContentElement>> },
    Link { text: String, url: String },
    // Add more types as needed, e.g., lists, images directly
}

#[derive(Debug, Clone)]
pub struct EpistemicUncertaintyInfo {
    pub level: f64, // 0.0 (completely uncertain) to 1.0 (completely certain)
    pub explanation: Option<String>, // Brief explanation of uncertainty if applicable
}

#[derive(Debug, Clone)]
pub struct EthicalReflectionInfo {
    pub pas_score_snapshot: Option<f64>, // PAS score relevant to this response
    pub summary: String, // e.g., "Considered privacy implications and fairness."
    pub details_query_suggestion: Option<String>, // e.g., "Ask 'explain ethics' for more details."
}

#[derive(Debug, Clone)]
pub struct SystemResponse {
    pub session_id: String,
    pub response_to_query_id: String,
    pub response_id: String,
    pub text_content: String,
    pub rich_content: Option<Vec<RichContentElement>>,
    pub epistemic_uncertainty: Option<EpistemicUncertaintyInfo>,
    pub ethical_reflection: Option<EthicalReflectionInfo>,
    pub follow_up_suggestions: Option<Vec<String>>, // e.g., related questions
    pub diagnostic_info: Option<HashMap<String, String>>, // For debugging, e.g., engine processing time
}

// --- Conversational Interface Trait ---

/// Defines the core functionality for handling user interactions.
pub trait Or4cl3ConversationalInterface {
    /// Processes a user's query and returns a system response.
    /// This involves converting the query to a stimulus, running it through
    /// the Recursive Cognition Engine, and then formatting the output.
    fn handle_user_query(&self, query: UserQuery) -> Result<SystemResponse, String>;
}

// --- Placeholder Implementation ---

/// Basic placeholder implementation for the conversational interface.
/// This would require a concrete instance of a RecursiveCognitionEngine.
pub struct BasicConversationalInterface<RCE: RecursiveCognitionEngine> {
    engine: RCE, // The interface uses a cognition engine
    // session_manager: SessionManager, // Future: manage conversation history, context
}

impl<RCE: RecursiveCognitionEngine> BasicConversationalInterface<RCE> {
    pub fn new(engine: RCE) -> Self {
        Self { engine }
    }

    fn map_cognitive_state_to_response(
        &self,
        cognitive_state: CognitiveState,
        query: &UserQuery
    ) -> SystemResponse {
        let epistemic_uncertainty = Some(EpistemicUncertaintyInfo {
            level: cognitive_state.confidence_level,
            explanation: if cognitive_state.confidence_level < 0.7 {
                Some("Confidence is moderate. More data or context could improve certainty.".to_string())
            } else { None },
        });

        let ethical_reflection = cognitive_state.ethical_assessment.as_ref().map(|assessment| {
            EthicalReflectionInfo {
                pas_score_snapshot: Some(assessment.pas_score),
                summary: format!("Ethical status: {}. Concerns: {}.",
                                 assessment.alignment_status,
                                 assessment.ethical_concerns.join(", ")),
                details_query_suggestion: Some("Ask 'Tell me more about the ethics of this response.'".to_string()),
            }
        });

        SystemResponse {
            session_id: query.session_id.clone(),
            response_to_query_id: query.query_id.clone(),
            response_id: format!("response_to_{}", query.query_id), // Simple response ID
            text_content: cognitive_state.current_hypothesis,
            rich_content: None, // Placeholder
            epistemic_uncertainty,
            ethical_reflection,
            follow_up_suggestions: Some(vec!["Ask another question.".to_string()]), // Placeholder
            diagnostic_info: Some(HashMap::from([
                ("state_id".to_string(), cognitive_state.state_id),
                ("history_log_entries".to_string(), cognitive_state.history_log.len().to_string())
            ])),
        }
    }
}

impl<RCE: RecursiveCognitionEngine> Or4cl3ConversationalInterface for BasicConversationalInterface<RCE> {
    fn handle_user_query(&self, query: UserQuery) -> Result<SystemResponse, String> {
        // 1. Convert UserQuery to Stimulus
        let stimulus_id = format!("stimulus_for_{}", query.query_id);
        let stimulus = Stimulus {
            id: stimulus_id.clone(),
            content: StimulusContent::Text(query.text.clone()),
            metadata: query.metadata.clone().unwrap_or_default(),
        };

        // 2. Process through Recursive Cognition Engine
        // This uses the engine passed during construction.
        match self.engine.execute_full_cycle(stimulus) {
            Ok(cognitive_state) => {
                // 3. Map CognitiveState to SystemResponse
                Ok(self.map_cognitive_state_to_response(cognitive_state, &query))
            }
            Err(e) => Err(format!("Error processing query via cognition engine: {}", e)),
        }
    }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use crate::recursive_cognition_engine::{BasicRecursiveCognitionEngine, Stimulus, HumanFeedback}; // For a concrete engine

    // Mock engine for testing the interface independently if needed, or use BasicRecursiveCognitionEngine
    struct MockEngine;
    impl RecursiveCognitionEngine for MockEngine {
        fn initialize_state_from_stimulus(&self, stimulus: Stimulus) -> Result<CognitiveState, String> {
             Ok(CognitiveState {
                state_id: "mock_state_01".to_string(),
                stimulus_id: stimulus.id,
                current_hypothesis: "Mock hypothesis from mock engine.".to_string(),
                confidence_level: 0.85,
                supporting_evidence_ids: vec![],
                ethical_assessment: Some(EthicalAssessmentReport {
                    pas_score: 0.95,
                    ethical_concerns: vec!["Mock concern".to_string()],
                    suggested_mitigations: vec![],
                    alignment_status: "Aligned (Mock)".to_string(),
                }),
                history_log: vec!["Processed by MockEngine".to_string()],
            })
        }
        fn assess_ethics(&self, state: &CognitiveState) -> Result<CognitiveState, String> { Ok(state.clone()) }
        fn refine_cognition(&self, state: &CognitiveState) -> Result<CognitiveState, String> { Ok(state.clone()) }
        fn validate_self(&self, state: &CognitiveState) -> Result<CognitiveState, String> { Ok(state.clone()) }
        fn incorporate_feedback(&self, state: &CognitiveState, _feedback: HumanFeedback) -> Result<CognitiveState, String> { Ok(state.clone()) }
        fn execute_full_cycle(&self, stimulus: Stimulus) -> Result<CognitiveState, String> {
            self.initialize_state_from_stimulus(stimulus) // Simplified cycle for mock
        }
    }


    #[test]
    fn test_handle_user_query_with_mock_engine() {
        let mock_engine = MockEngine {};
        let interface = BasicConversationalInterface::new(mock_engine);

        let query = UserQuery {
            session_id: "session123".to_string(),
            query_id: "query001".to_string(),
            text: "Hello OR4CL3".to_string(),
            metadata: None,
        };

        let response_result = interface.handle_user_query(query.clone());
        assert!(response_result.is_ok());
        let response = response_result.unwrap();

        assert_eq!(response.session_id, query.session_id);
        assert_eq!(response.response_to_query_id, query.query_id);
        assert_eq!(response.text_content, "Mock hypothesis from mock engine.");

        assert!(response.epistemic_uncertainty.is_some());
        let uncertainty = response.epistemic_uncertainty.unwrap();
        assert_eq!(uncertainty.level, 0.85);

        assert!(response.ethical_reflection.is_some());
        let reflection = response.ethical_reflection.unwrap();
        assert_eq!(reflection.pas_score_snapshot, Some(0.95));
        assert!(reflection.summary.contains("Mock concern"));
    }

    #[test]
    fn test_handle_user_query_with_basic_engine() {
        // This test uses the BasicRecursiveCognitionEngine, which has its own mock logic.
        let basic_engine = BasicRecursiveCognitionEngine {}; // From recursive_cognition_engine module
        let interface = BasicConversationalInterface::new(basic_engine);

        let query = UserQuery {
            session_id: "session456".to_string(),
            query_id: "query002".to_string(),
            text: "Tell me a fact.".to_string(),
            metadata: None,
        };

        let response_result = interface.handle_user_query(query.clone());
        assert!(response_result.is_ok());
        let response = response_result.unwrap();

        assert!(response.text_content.contains("Initial hypothesis for stimulus: stimulus_for_query002 (refined)"));
        assert!(response.ethical_reflection.is_some());
        let reflection = response.ethical_reflection.unwrap();
        assert_eq!(reflection.pas_score_snapshot, Some(0.92)); // From BasicRecursiveCognitionEngine mock
    }
}
