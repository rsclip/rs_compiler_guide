use super::symbols::SymbolTable;

use anyhow::{Error, Result};

/// Trait for Semantic Analysis
///
/// When implementing, you are expected to analyse
/// every part of the node, aggregating and returning
/// *all* errors found.
trait Analysis {
    /// Analyze the node
    fn analyze(&self, table: &SymbolTable) -> Result<(), Error>;
}
