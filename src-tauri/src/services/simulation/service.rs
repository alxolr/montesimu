use shaku::{Component, Interface};
use std::collections::HashMap;

use super::parser::{Expr, parse_expression, validate_syntax};
use super::analyzer::{extract_identifiers, build_dependency_graph, topological_sort};

/// Service interface for simulation engine operations
pub trait SimulationService: Interface {
    /// Parse an expression string into an AST
    fn parse_expression(&self, input: &str) -> Result<Expr, String>;
    
    /// Validate expression syntax
    fn validate_syntax(&self, input: &str) -> Result<(), String>;
    
    /// Extract all identifiers from an expression
    fn extract_identifiers(&self, expr: &Expr) -> std::collections::HashSet<String>;
    
    /// Build dependency graph for expressions
    fn build_dependency_graph(
        &self,
        intermediate_exprs: &HashMap<String, Expr>,
        target_expr: &Expr,
    ) -> HashMap<String, std::collections::HashSet<String>>;
    
    /// Perform topological sort to determine evaluation order
    fn topological_sort(
        &self,
        intermediate_exprs: &HashMap<String, Expr>,
        target_expr: &Expr,
    ) -> Result<Vec<String>, String>;
}

/// Implementation of the simulation engine service
#[derive(Component)]
#[shaku(interface = SimulationService)]
pub struct SimulationServiceImpl;

impl SimulationService for SimulationServiceImpl {
    fn parse_expression(&self, input: &str) -> Result<Expr, String> {
        parse_expression(input)
    }
    
    fn validate_syntax(&self, input: &str) -> Result<(), String> {
        validate_syntax(input)
    }
    
    fn extract_identifiers(&self, expr: &Expr) -> std::collections::HashSet<String> {
        extract_identifiers(expr)
    }
    
    fn build_dependency_graph(
        &self,
        intermediate_exprs: &HashMap<String, Expr>,
        target_expr: &Expr,
    ) -> HashMap<String, std::collections::HashSet<String>> {
        build_dependency_graph(intermediate_exprs, target_expr)
    }
    
    fn topological_sort(
        &self,
        intermediate_exprs: &HashMap<String, Expr>,
        target_expr: &Expr,
    ) -> Result<Vec<String>, String> {
        topological_sort(intermediate_exprs, target_expr)
    }
}
