use super::symbols::SymbolTable;

use anyhow::Error;

/// Trait for Semantic Analysis
///
/// When implementing, you are expected to analyse
/// every part of the node, aggregating and returning
/// *all* errors found.
pub trait Analysis {
    /// Analyze the node
    fn analyze(&self, table: &mut SymbolTable) -> Vec<Error>;
}
