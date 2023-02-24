use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Project,
    Job,
    JobExecution,
    User,
    Runtime,
    Artifact,
    Document,
    Host,
}
