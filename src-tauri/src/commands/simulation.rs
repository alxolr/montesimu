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
    let simulation_service: &dyn SimulationService = state.resolve_ref();
    
    // Validate all expressions before running simulation
    for intermediate_expr in &model.intermediate_expressions {
        simulation_service.validate_syntax(&intermediate_expr.expression)?;
    }
    simulation_service.validate_syntax(&model.target_expression)?;
    
    // Delegate to service layer for business logic
    simulation_service.run_simulation(&model)
}
