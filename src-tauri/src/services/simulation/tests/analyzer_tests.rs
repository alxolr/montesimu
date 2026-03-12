use std::collections::HashMap;
use crate::services::simulation::parser::{Expr, parse_expression};
use crate::services::simulation::analyzer::{extract_identifiers, build_dependency_graph, topological_sort};

#[test]
fn test_extract_identifiers_empty() {
    let expr = Expr::Number(42.0);
    let ids = extract_identifiers(&expr);
    assert_eq!(ids.len(), 0);
}

#[test]
fn test_extract_identifiers_single() {
    let expr = Expr::Identifier("x".to_string());
    let ids = extract_identifiers(&expr);
    assert_eq!(ids.len(), 1);
    assert!(ids.contains("x"));
}

#[test]
fn test_extract_identifiers_multiple() {
    let expr = parse_expression("x + y").unwrap();
    let ids = extract_identifiers(&expr);
    assert_eq!(ids.len(), 2);
    assert!(ids.contains("x"));
    assert!(ids.contains("y"));
}

#[test]
fn test_extract_identifiers_duplicates() {
    let expr = parse_expression("x + x").unwrap();
    let ids = extract_identifiers(&expr);
    assert_eq!(ids.len(), 1);
    assert!(ids.contains("x"));
}

#[test]
fn test_build_dependency_graph_empty() {
    let intermediate_exprs = HashMap::new();
    let target_expr = parse_expression("42").unwrap();
    
    let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
    
    assert_eq!(graph.len(), 1);
    assert!(graph.contains_key("__target__"));
    assert_eq!(graph.get("__target__").unwrap().len(), 0);
}

#[test]
fn test_build_dependency_graph_target_with_variables() {
    let intermediate_exprs = HashMap::new();
    let target_expr = parse_expression("x + y").unwrap();
    
    let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
    
    assert_eq!(graph.len(), 1);
    let target_deps = graph.get("__target__").unwrap();
    assert_eq!(target_deps.len(), 2);
    assert!(target_deps.contains("x"));
    assert!(target_deps.contains("y"));
}

#[test]
fn test_build_dependency_graph_with_intermediate_expressions() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("expr1".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("expr2".to_string(), parse_expression("z * 2").unwrap());
    
    let target_expr = parse_expression("expr1 + expr2").unwrap();
    
    let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
    
    assert_eq!(graph.len(), 3);
    
    let expr1_deps = graph.get("expr1").unwrap();
    assert_eq!(expr1_deps.len(), 2);
    assert!(expr1_deps.contains("x"));
    assert!(expr1_deps.contains("y"));
    
    let expr2_deps = graph.get("expr2").unwrap();
    assert_eq!(expr2_deps.len(), 1);
    assert!(expr2_deps.contains("z"));
    
    let target_deps = graph.get("__target__").unwrap();
    assert_eq!(target_deps.len(), 2);
    assert!(target_deps.contains("expr1"));
    assert!(target_deps.contains("expr2"));
}

#[test]
fn test_build_dependency_graph_mixed_references() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("expr1".to_string(), parse_expression("x * 2").unwrap());
    
    let target_expr = parse_expression("expr1 + y").unwrap();
    
    let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
    
    assert_eq!(graph.len(), 2);
    
    let expr1_deps = graph.get("expr1").unwrap();
    assert_eq!(expr1_deps.len(), 1);
    assert!(expr1_deps.contains("x"));
    
    let target_deps = graph.get("__target__").unwrap();
    assert_eq!(target_deps.len(), 2);
    assert!(target_deps.contains("expr1"));
    assert!(target_deps.contains("y"));
}

#[test]
fn test_build_dependency_graph_complex_expressions() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("a * z").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("w / 2").unwrap());
    
    let target_expr = parse_expression("(b + c) * a").unwrap();
    
    let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
    
    assert_eq!(graph.len(), 4);
    
    let a_deps = graph.get("a").unwrap();
    assert_eq!(a_deps.len(), 2);
    assert!(a_deps.contains("x"));
    assert!(a_deps.contains("y"));
    
    let b_deps = graph.get("b").unwrap();
    assert_eq!(b_deps.len(), 2);
    assert!(b_deps.contains("a"));
    assert!(b_deps.contains("z"));
    
    let c_deps = graph.get("c").unwrap();
    assert_eq!(c_deps.len(), 1);
    assert!(c_deps.contains("w"));
    
    let target_deps = graph.get("__target__").unwrap();
    assert_eq!(target_deps.len(), 3);
    assert!(target_deps.contains("b"));
    assert!(target_deps.contains("c"));
    assert!(target_deps.contains("a"));
}

#[test]
fn test_build_dependency_graph_no_duplicates() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("expr1".to_string(), parse_expression("x + x + x").unwrap());
    
    let target_expr = parse_expression("expr1 + expr1").unwrap();
    
    let graph = build_dependency_graph(&intermediate_exprs, &target_expr);
    
    let expr1_deps = graph.get("expr1").unwrap();
    assert_eq!(expr1_deps.len(), 1);
    assert!(expr1_deps.contains("x"));
    
    let target_deps = graph.get("__target__").unwrap();
    assert_eq!(target_deps.len(), 1);
    assert!(target_deps.contains("expr1"));
}

#[test]
fn test_topological_sort_no_expressions() {
    let intermediate_exprs = HashMap::new();
    let target_expr = parse_expression("x + y").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 0);
}

#[test]
fn test_topological_sort_single_expression() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("expr1".to_string(), parse_expression("x + y").unwrap());
    
    let target_expr = parse_expression("expr1 * 2").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "expr1");
}

#[test]
fn test_topological_sort_linear_dependency() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("a * 2").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("b + 5").unwrap());
    
    let target_expr = parse_expression("c / 3").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 3);
    
    let a_pos = result.iter().position(|x| x == "a").unwrap();
    let b_pos = result.iter().position(|x| x == "b").unwrap();
    let c_pos = result.iter().position(|x| x == "c").unwrap();
    
    assert!(a_pos < b_pos);
    assert!(b_pos < c_pos);
}

#[test]
fn test_topological_sort_independent_expressions() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("z * 2").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("w / 3").unwrap());
    
    let target_expr = parse_expression("a + b + c").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 3);
    assert!(result.contains(&"a".to_string()));
    assert!(result.contains(&"b".to_string()));
    assert!(result.contains(&"c".to_string()));
}

#[test]
fn test_topological_sort_diamond_dependency() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("a * 2").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("a * 3").unwrap());
    intermediate_exprs.insert("d".to_string(), parse_expression("b + c").unwrap());
    
    let target_expr = parse_expression("d / 2").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 4);
    
    let a_pos = result.iter().position(|x| x == "a").unwrap();
    let b_pos = result.iter().position(|x| x == "b").unwrap();
    let c_pos = result.iter().position(|x| x == "c").unwrap();
    let d_pos = result.iter().position(|x| x == "d").unwrap();
    
    assert!(a_pos < b_pos);
    assert!(a_pos < c_pos);
    assert!(b_pos < d_pos);
    assert!(c_pos < d_pos);
}

#[test]
fn test_topological_sort_circular_dependency_simple() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("b + 1").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("a + 1").unwrap());
    
    let target_expr = parse_expression("a + b").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr);
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Circular dependency detected"));
    assert!(err.contains("a") || err.contains("b"));
}

#[test]
fn test_topological_sort_circular_dependency_complex() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("b + 1").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("c + 1").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("a + 1").unwrap());
    
    let target_expr = parse_expression("a").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr);
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Circular dependency detected"));
}

#[test]
fn test_topological_sort_self_reference() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("a + 1").unwrap());
    
    let target_expr = parse_expression("a * 2").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr);
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Circular dependency detected"));
    assert!(err.contains("a"));
}

#[test]
fn test_topological_sort_mixed_dependencies() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("a + z").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("w * 2").unwrap());
    intermediate_exprs.insert("d".to_string(), parse_expression("b + c").unwrap());
    
    let target_expr = parse_expression("d + a").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 4);
    
    let a_pos = result.iter().position(|x| x == "a").unwrap();
    let b_pos = result.iter().position(|x| x == "b").unwrap();
    let d_pos = result.iter().position(|x| x == "d").unwrap();
    
    assert!(a_pos < b_pos);
    assert!(b_pos < d_pos);
    
    let c_pos = result.iter().position(|x| x == "c").unwrap();
    assert!(c_pos < d_pos);
}

#[test]
fn test_topological_sort_only_variable_dependencies() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("z * w").unwrap());
    
    let target_expr = parse_expression("a + b").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"a".to_string()));
    assert!(result.contains(&"b".to_string()));
}

#[test]
fn test_topological_sort_partial_circular() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + y").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("c + 1").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("b + 1").unwrap());
    
    let target_expr = parse_expression("a + b").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr);
    
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Circular dependency detected"));
}

#[test]
fn test_topological_sort_long_chain() {
    let mut intermediate_exprs = HashMap::new();
    intermediate_exprs.insert("a".to_string(), parse_expression("x + 1").unwrap());
    intermediate_exprs.insert("b".to_string(), parse_expression("a + 1").unwrap());
    intermediate_exprs.insert("c".to_string(), parse_expression("b + 1").unwrap());
    intermediate_exprs.insert("d".to_string(), parse_expression("c + 1").unwrap());
    intermediate_exprs.insert("e".to_string(), parse_expression("d + 1").unwrap());
    
    let target_expr = parse_expression("e").unwrap();
    
    let result = topological_sort(&intermediate_exprs, &target_expr).unwrap();
    
    assert_eq!(result.len(), 5);
    assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
}
