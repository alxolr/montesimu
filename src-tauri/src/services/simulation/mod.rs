pub mod lexer;
pub mod parser;
pub mod analyzer;
pub mod service;

#[cfg(test)]
mod tests;

// Re-export commonly used items
pub use lexer::{Token, tokenize};
pub use parser::{Expr, Operator, parse_expression, validate_syntax};
pub use analyzer::{extract_identifiers, build_dependency_graph, topological_sort};
pub use service::{SimulationService, SimulationServiceImpl};
