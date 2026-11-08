use serde::{Deserialize, Serialize};

/// Which way a related transaction points: `Forward` to one that depends
/// on this one, `Backward` to one this one depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Forward,
    Backward,
}
