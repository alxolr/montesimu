import { Injectable, signal, computed, Signal } from '@angular/core';
import { Variable } from '../models/variable.model';
import { Constant } from '../models/constant.model';
import { ExpressionValidatorService, ValidationResult } from './expression-validator.service';

@Injectable({
  providedIn: 'root'
})
export class ModelService {
  // Private writable signals
  private variablesSignal = signal<Variable[]>([]);
  private constantsSignal = signal<Constant[]>([]);
  private expressionSignal = signal<string>('');

  // Public readonly signals
  readonly variables: Signal<Variable[]> = this.variablesSignal.asReadonly();
  readonly constants: Signal<Constant[]> = this.constantsSignal.asReadonly();
  readonly expression: Signal<string> = this.expressionSignal.asReadonly();

  // Computed signal for all identifiers
  readonly allIdentifiers: Signal<string[]> = computed(() => [
    ...this.variables().map(v => v.name),
    ...this.constants().map(c => c.name)
  ]);

  // Computed signal for expression validation
  readonly expressionValidation: Signal<ValidationResult> = computed(() => {
    return this.expressionValidator.validate(
      this.expression(),
      this.allIdentifiers()
    );
  });

  constructor(private expressionValidator: ExpressionValidatorService) {}

  // Variable operations
  addVariable(variable: Variable): void {
    if (!this.isIdentifierUnique(variable.name)) {
      throw new Error(`A variable or constant with name "${variable.name}" already exists`);
    }
    this.variablesSignal.update(vars => [...vars, variable]);
  }

  updateVariable(oldName: string, variable: Variable): void {
    if (oldName !== variable.name && !this.isIdentifierUnique(variable.name)) {
      throw new Error(`A variable or constant with name "${variable.name}" already exists`);
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

  // Constant operations
  addConstant(constant: Constant): void {
    if (!this.isIdentifierUnique(constant.name)) {
      throw new Error(`A variable or constant with name "${constant.name}" already exists`);
    }
    this.constantsSignal.update(consts => [...consts, constant]);
  }

  updateConstant(oldName: string, constant: Constant): void {
    if (oldName !== constant.name && !this.isIdentifierUnique(constant.name)) {
      throw new Error(`A variable or constant with name "${constant.name}" already exists`);
    }
    this.constantsSignal.update(consts =>
      consts.map(c => c.name === oldName ? constant : c)
    );
  }

  deleteConstant(name: string): void {
    this.constantsSignal.update(consts => consts.filter(c => c.name !== name));
  }

  getConstant(name: string): Constant | undefined {
    return this.constants().find(c => c.name === name);
  }

  // Expression operations
  setExpression(expression: string): void {
    this.expressionSignal.set(expression);
  }

  // Validation helper
  isIdentifierUnique(name: string, excludeName?: string): boolean {
    const allIds = this.allIdentifiers();
    const filtered = excludeName 
      ? allIds.filter(id => id !== excludeName)
      : allIds;
    return !filtered.includes(name);
  }
}
