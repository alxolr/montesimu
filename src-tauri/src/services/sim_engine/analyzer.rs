use std::collections::{HashMap, HashSet, VecDeque};
use super::parser::Expr;

/// Extracts all unique identifiers from an expression
pub fn extract_identifiers(expr: &Expr) -> HashSet<String> {
    let mut identifiers = HashSet::new();
    extract_identifiers_recursive(expr, &mut identifiers);
    identifiers
}

/// Builds a dependency graph showing which expressions reference which identifiers
///
/// # Arguments
/// * `intermediate_exprs` - Map of intermediate expression names to their parsed ASTs
/// * `target_expr` - The target expression AST
///
/// # Returns
/// A HashMap where keys are expression names and values are sets of identifiers they depend on
/// The target expression is represented with the key "__target__"
pub fn build_dependency_graph(
    intermediate_exprs: &HashMap<String, Expr>,
    target_expr: &Expr,
) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();

    // Add dependencies for each intermediate expression
    for (name, expr) in intermediate_exprs {
        let dependencies = extract_identifiers(expr);
        graph.insert(name.clone(), dependencies);
    }

    // Add dependencies for target expression
    let target_dependencies = extract_identifiers(target_expr);
    graph.insert("__target__".to_string(), target_dependencies);

    graph
}

/// Performs topological sort on the dependency graph to determine evaluation order
///
/// # Arguments
/// * `intermediate_exprs` - Map of intermediate expression names to their parsed ASTs
/// * `target_expr` - The target expression AST
///
/// # Returns
/// A vector of expression names in the order they should be evaluated
/// Returns an error if circular dependencies are detected
///
/// # Example
/// If expr1 depends on x, expr2 depends on expr1, and target depends on expr2,
/// the evaluation order would be: ["expr1", "expr2"]
pub fn topological_sort(
    intermediate_exprs: &HashMap<String, Expr>,
    target_expr: &Expr,
) -> Result<Vec<String>, String> {
    // Build the dependency graph
    let dep_graph = build_dependency_graph(intermediate_exprs, target_expr);

    // Separate variables from expressions
    let expr_names: HashSet<String> = intermediate_exprs.keys().cloned().collect();

    // Build adjacency list (only for expressions, not variables)
    let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();

    // Initialize in-degree for all expressions
    for name in &expr_names {
        in_degree.insert(name.clone(), 0);
        adj_list.insert(name.clone(), Vec::new());
    }

    // Build adjacency list and calculate in-degrees
    for (expr_name, dependencies) in &dep_graph {
        // Skip target expression
        if expr_name == "__target__" {
            continue;
        }

        for dep in dependencies {
            // Only consider dependencies on other expressions (not variables)
            if expr_names.contains(dep) {
                // dep -> expr_name (dep must be evaluated before expr_name)
                adj_list.get_mut(dep).unwrap().push(expr_name.clone());
                *in_degree.get_mut(expr_name).unwrap() += 1;
            }
        }
    }

    // Kahn's algorithm for topological sort
    let mut queue: VecDeque<String> = VecDeque::new();
    let mut result: Vec<String> = Vec::new();

    // Start with expressions that have no dependencies on other expressions
    for (name, &degree) in &in_degree {
        if degree == 0 {
            queue.push_back(name.clone());
        }
    }

    while let Some(current) = queue.pop_front() {
        result.push(current.clone());

        // Reduce in-degree for all neighbors
        if let Some(neighbors) = adj_list.get(&current) {
            for neighbor in neighbors {
                let degree = in_degree.get_mut(neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    // Check for circular dependencies
    if result.len() != expr_names.len() {
        // Find the expressions involved in the cycle
        let sorted_set: HashSet<String> = result.iter().cloned().collect();
        let cycle_exprs: Vec<String> = expr_names
            .iter()
            .filter(|name| !sorted_set.contains(*name))
            .cloned()
            .collect();

        return Err(format!(
            "Circular dependency detected among expressions: {}",
            cycle_exprs.join(", ")
        ));
    }

    Ok(result)
}

fn extract_identifiers_recursive(expr: &Expr, identifiers: &mut HashSet<String>) {
    match expr {
        Expr::Number(_) => {}
        Expr::Identifier(id) => {
            identifiers.insert(id.clone());
        }
        Expr::BinaryOp { left, right, .. } => {
            extract_identifiers_recursive(left, identifiers);
            extract_identifiers_recursive(right, identifiers);
        }
    }
}
