use shaku::{Component, Interface};
use std::collections::HashMap;

use crate::dtos::simulation::{ModelDefinition, SimulationResults};
use super::parser::{Expr, parse_expression, validate_syntax};
use super::analyzer::{extract_identifiers, build_dependency_graph, topological_sort};

/// Service interface for simulation engine operations
pub trait SimulationService: Interface {
    /// Run a complete simulation from model definition to results
    /// This is the main entry point that orchestrates the entire simulation process
    fn run_simulation(&self, model: &ModelDefinition) -> Result<SimulationResults, String>;
    
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
    fn run_simulation(&self, model: &ModelDefinition) -> Result<SimulationResults, String> {
        // Step 1: Parse all intermediate expressions into ASTs
        let mut intermediate_exprs: HashMap<String, Expr> = HashMap::new();
        for intermediate_expr in &model.intermediate_expressions {
            let ast = self.parse_expression(&intermediate_expr.expression)?;
            intermediate_exprs.insert(intermediate_expr.name.clone(), ast);
        }
        
        // Step 2: Parse target expression into AST
        let target_ast = self.parse_expression(&model.target_expression)?;
        
        // Step 3: Build dependency graph and get evaluation order
        let eval_order = self.topological_sort(&intermediate_exprs, &target_ast)?;
        
        // Step 4: TODO - Execute simulation with the evaluation order
        // This will be implemented in later tasks (evaluator, sampler, engine)
        // For now, return an error indicating the feature is not yet fully implemented
        Err(format!(
            "Simulation execution not yet implemented. Parsed {} intermediate expressions, target expression, and determined evaluation order: {:?}",
            intermediate_exprs.len(),
            eval_order
        ))
    }
    
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
