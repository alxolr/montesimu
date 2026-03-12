import { Injectable } from '@angular/core';

export interface DependencyNode {
  id: string;
  type: 'variable' | 'expression';
  dependencies: string[];
}

export interface CircularReferenceError {
  hasCircularReference: boolean;
  cyclePath?: string[];
  message?: string;
}

export interface DependencyGraph {
  nodes: Map<string, DependencyNode>;
}

@Injectable({
  providedIn: 'root'
})
export class DependencyGraphService {
  /**
   * Extract identifiers from a formula using regex
   * Identifiers are alphanumeric sequences that are not numeric literals
   */
  extractIdentifiers(formula: string): string[] {
    if (!formula) {
      return [];
    }

    // Match alphanumeric sequences
    const identifierPattern = /[a-zA-Z_][a-zA-Z0-9_]*/g;
    const matches = formula.match(identifierPattern) || [];
    
    // Remove duplicates and return
    return Array.from(new Set(matches));
  }

  /**
   * Build dependency graph from variables and intermediate expressions
   */
  buildGraph(
    variables: Array<{ name: string }>,
    intermediateExpressions: Array<{ name: string; formula: string }>
  ): DependencyGraph {
    const nodes = new Map<string, DependencyNode>();

    // Add variable nodes (no dependencies)
    variables.forEach(variable => {
      nodes.set(variable.name, {
        id: variable.name,
        type: 'variable',
        dependencies: []
      });
    });

    // Add intermediate expression nodes with dependencies
    intermediateExpressions.forEach(expr => {
      const dependencies = this.extractIdentifiers(expr.formula);
      nodes.set(expr.name, {
        id: expr.name,
        type: 'expression',
        dependencies
      });
    });

    return { nodes };
  }

  /**
   * Detect if adding/updating an expression would create a circular reference
   * Uses DFS with recursion stack to detect cycles
   */
  detectCircularReference(
    graph: DependencyGraph,
    startNode: string
  ): CircularReferenceError {
    const visited = new Set<string>();
    const recursionStack = new Set<string>();
    const path: string[] = [];

    const dfs = (nodeId: string): boolean => {
      if (!graph.nodes.has(nodeId)) {
        return false;
      }

      visited.add(nodeId);
      recursionStack.add(nodeId);
      path.push(nodeId);

      const node = graph.nodes.get(nodeId)!;
      
      for (const dependency of node.dependencies) {
        if (!visited.has(dependency)) {
          if (dfs(dependency)) {
            return true;
          }
        } else if (recursionStack.has(dependency)) {
          // Found a cycle
          path.push(dependency);
          return true;
        }
      }

      recursionStack.delete(nodeId);
      path.pop();
      return false;
    };

    if (dfs(startNode)) {
      // Reconstruct cycle path
      const cycleStart = path[path.length - 1];
      const cycleStartIndex = path.indexOf(cycleStart);
      const cyclePath = path.slice(cycleStartIndex);
      
      return {
        hasCircularReference: true,
        cyclePath,
        message: `Circular reference detected: ${cyclePath.join(' → ')}`
      };
    }

    return {
      hasCircularReference: false
    };
  }

  /**
   * Check if adding a new expression with given dependencies would create a cycle
   */
  wouldCreateCycle(
    graph: DependencyGraph,
    expressionName: string,
    dependencies: string[]
  ): CircularReferenceError {
    // Create a temporary graph with the new expression
    const tempGraph: DependencyGraph = {
      nodes: new Map(graph.nodes)
    };

    tempGraph.nodes.set(expressionName, {
      id: expressionName,
      type: 'expression',
      dependencies
    });

    return this.detectCircularReference(tempGraph, expressionName);
  }

  /**
   * Get evaluation order using topological sort (Kahn's algorithm)
   * Returns null if a cycle is detected
   */
  getEvaluationOrder(graph: DependencyGraph): string[] | null {
    // Calculate in-degrees
    const inDegree = new Map<string, number>();
    const adjList = new Map<string, string[]>();

    // Initialize
    graph.nodes.forEach((node, id) => {
      inDegree.set(id, 0);
      adjList.set(id, []);
    });

    // Build adjacency list and calculate in-degrees
    graph.nodes.forEach((node, id) => {
      node.dependencies.forEach(dep => {
        if (graph.nodes.has(dep)) {
          adjList.get(dep)!.push(id);
          inDegree.set(id, (inDegree.get(id) || 0) + 1);
        }
      });
    });

    // Queue nodes with in-degree 0
    const queue: string[] = [];
    inDegree.forEach((degree, id) => {
      if (degree === 0) {
        queue.push(id);
      }
    });

    const result: string[] = [];

    while (queue.length > 0) {
      const current = queue.shift()!;
      result.push(current);

      // Reduce in-degree for neighbors
      adjList.get(current)!.forEach(neighbor => {
        const newDegree = inDegree.get(neighbor)! - 1;
        inDegree.set(neighbor, newDegree);
        
        if (newDegree === 0) {
          queue.push(neighbor);
        }
      });
    }

    // If result doesn't contain all nodes, there's a cycle
    if (result.length !== graph.nodes.size) {
      return null;
    }

    return result;
  }
}
