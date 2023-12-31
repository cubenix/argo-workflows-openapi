use serde::{Deserialize, Serialize};

/// Outputs hold parameters, artifacts, and results from a step.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Outputs {
    /// Artifacts holds the list of output artifacts produced by a step
    #[serde(rename = "artifacts", skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<super::Artifact>>,
    
    /// ExitCode holds the exit code of a script template
    #[serde(rename = "exitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<String>,
    
    /// Parameters holds the list of output parameters produced by a step
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<super::Parameter>>,
    
    /// Result holds the result (stdout) of a script template
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl Outputs {
    pub fn new() -> Outputs {
        Outputs {
            artifacts: None,
            exit_code: None,
            parameters: None,
            result: None,
        }
    }
}
