pub mod fuzzy;
pub mod resolver;
pub mod scanner;

pub use fuzzy::{DEFAULT_IGNORE, ScoredCandidate, find_candidates};
pub use resolver::{Action, RepairCase, Resolution, Summary};
pub use scanner::{BrokenSymlink, find_broken_symlinks};
