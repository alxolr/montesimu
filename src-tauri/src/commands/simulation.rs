use shaku::HasComponent;
use tauri::State;

use crate::dtos::simulation::{ModelDefinition, SimulationResults};
use crate::services::{simulation::SimulationService, Container};

// Tauri command handler
#[tauri::command]
pub async fn run_simulation(
    state: State<'_, Container>,
    model: ModelDefinition,
) -> Result<SimulationResults, String> {
    let sim_engine: &dyn SimulationService = state.resolve_ref();
    
    // Parse and validate all expressions
    for intermediate_expr in &model.intermediate_expressions {
        sim_engine.validate_syntax(&intermediate_expr.expression)?;
    }
    sim_engine.validate_syntax(&model.target_expression)?;
    
    // Parse expressions into AST
    let mut intermediate_exprs = std::collections::HashMap::new();
    for intermediate_expr in &model.intermediate_expressions {
        let ast = sim_engine.parse_expression(&intermediate_expr.expression)?;
        intermediate_exprs.insert(intermediate_expr.name.clone(), ast);
    }
    let target_ast = sim_engine.parse_expression(&model.target_expression)?;
    
    // Build dependency graph and get evaluation order
    let _eval_order = sim_engine.topological_sort(&intermediate_exprs, &target_ast)?;
    
    // TODO: Implement actual simulation execution
    // For now, return an error indicating the feature is not yet fully implemented
    Err("Simulation execution not yet implemented - parser and analyzer ready".to_string())
}
