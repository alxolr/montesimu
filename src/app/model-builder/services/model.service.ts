import { Injectable, signal, computed, Signal } from '@angular/core';
import { Variable } from '../models/variable.model';
import { IntermediateExpression } from '../models/intermediate-expression.model';
import { ExpressionValidatorService, ValidationResult } from './expression-validator.service';
import { DependencyGraphService, CircularReferenceError } from './dependency-graph.service';

export interface OperationResult<T = void> {
  success: boolean;
  data?: T;
  error?: string;
}

@Injectable({
  providedIn: 'root'
})
export class ModelService {
  // Private writable signals
  private variablesSignal = signal<Variable[]>([]);
  private intermediateExpressionsSignal = signal<IntermediateExpression[]>([]);
  private targetExpressionSignal = signal<string>('');

  // Public readonly signals
  readonly variables: Signal<Variable[]> = this.variablesSignal.asReadonly();
  readonly intermediateExpressions: Signal<IntermediateExpression[]> = this.intermediateExpressionsSignal.asReadonly();
  readonly targetExpression: Signal<string> = this.targetExpressionSignal.asReadonly();

  // Computed signal for all identifiers (variables + intermediate expressions)
  readonly allIdentifiers: Signal<string[]> = computed(() => [
    ...this.variables().map(v => v.name),
    ...this.intermediateExpressions().map(e => e.name)
  ]);

  // Computed signal for target expression validation
  readonly targetExpressionValidation: Signal<ValidationResult> = computed(() => {
    return this.expressionValidator.validate(
      this.targetExpression(),
      this.allIdentifiers()
    );
  });

  // Computed signal for evaluation order
  readonly evaluationOrder: Signal<string[] | null> = computed(() => {
    const graph = this.dependencyGraph.buildGraph(
      this.variables(),
      this.intermediateExpressions()
    );
    return this.dependencyGraph.getEvaluationOrder(graph);
  });

  constructor(
    private expressionValidator: ExpressionValidatorService,
    private dependencyGraph: DependencyGraphService
  ) {}

  // Variable operations
  addVariable(variable: Variable): void {
    if (!this.isIdentifierUnique(variable.name)) {
      throw new Error(`An identifier with name "${variable.name}" already exists`);
    }
    this.variablesSignal.update(vars => [...vars, variable]);
  }

  updateVariable(oldName: string, variable: Variable): void {
    if (oldName !== variable.name && !this.isIdentifierUnique(variable.name)) {
      throw new Error(`An identifier with name "${variable.name}" already exists`);
    }
    this.variablesSignal.update(vars =>
      vars.map(v => v.name === oldName ? variable : v)
    );
  }

  deleteVariable(name: string): void {
    this.variablesSignal.update(vars => vars.filter(v => v.name !== name));
  }

  getVariable(name: string): Variable | undefined {
    return this.variables().find(v => v.name === name);
  }

  // Intermediate expression operations
  addIntermediateExpression(expression: IntermediateExpression): OperationResult {
    // Check uniqueness
    if (!this.isIdentifierUnique(expression.name)) {
      return {
        success: false,
        error: `An identifier with name "${expression.name}" already exists`
      };
    }

    // Check for circular references
    const dependencies = this.dependencyGraph.extractIdentifiers(expression.formula);
    const graph = this.dependencyGraph.buildGraph(
      this.variables(),
      this.intermediateExpressions()
    );
    
    const circularCheck = this.dependencyGraph.wouldCreateCycle(
      graph,
      expression.name,
      dependencies
    );

    if (circularCheck.hasCircularReference) {
      return {
        success: false,
        error: circularCheck.message
      };
    }

    // Add expression
    this.intermediateExpressionsSignal.update(exprs => [...exprs, expression]);
    return { success: true };
  }

  updateIntermediateExpression(oldName: string, expression: IntermediateExpression): OperationResult {
    // Check uniqueness if name changed
    if (oldName !== expression.name && !this.isIdentifierUnique(expression.name)) {
      return {
        success: false,
        error: `An identifier with name "${expression.name}" already exists`
      };
    }

    // Check for circular references with updated formula
    const dependencies = this.dependencyGraph.extractIdentifiers(expression.formula);
    const tempExpressions = this.intermediateExpressions().map(e =>
      e.name === oldName ? expression : e
    );
    
    const graph = this.dependencyGraph.buildGraph(
      this.variables(),
      tempExpressions
    );
    
    const circularCheck = this.dependencyGraph.detectCircularReference(graph, expression.name);

    if (circularCheck.hasCircularReference) {
      return {
        success: false,
        error: circularCheck.message
      };
    }

    // Update expression
    this.intermediateExpressionsSignal.update(exprs =>
      exprs.map(e => e.name === oldName ? expression : e)
    );
    return { success: true };
  }

  deleteIntermediateExpression(name: string): OperationResult {
    // Check if any expressions reference this one
    const referencingExpressions = this.getExpressionsReferencingIdentifier(name);
    
    if (referencingExpressions.length > 0) {
      return {
        success: false,
        error: `Cannot delete "${name}": Referenced by ${referencingExpressions.join(', ')}`
      };
    }

    // Check if target expression references this
    const targetDeps = this.dependencyGraph.extractIdentifiers(this.targetExpression());
    if (targetDeps.includes(name)) {
      return {
        success: false,
        error: `Cannot delete "${name}": Referenced by target expression`
      };
    }

    // Delete expression
    this.intermediateExpressionsSignal.update(exprs => exprs.filter(e => e.name !== name));
    return { success: true };
  }

  getIntermediateExpression(name: string): IntermediateExpression | undefined {
    return this.intermediateExpressions().find(e => e.name === name);
  }

  // Target expression operations
  setTargetExpression(expression: string): void {
    this.targetExpressionSignal.set(expression);
  }

  // Helper methods
  getDependencies(formula: string): string[] {
    return this.dependencyGraph.extractIdentifiers(formula);
  }

  getExpressionsReferencingIdentifier(identifier: string): string[] {
    return this.intermediateExpressions()
      .filter(expr => {
        const deps = this.dependencyGraph.extractIdentifiers(expr.formula);
        return deps.includes(identifier);
      })
      .map(expr => expr.name);
  }

  isIdentifierUnique(name: string, excludeName?: string): boolean {
    const allIds = this.allIdentifiers();
    const filtered = excludeName 
      ? allIds.filter(id => id !== excludeName)
      : allIds;
    return !filtered.includes(name);
  }
}
