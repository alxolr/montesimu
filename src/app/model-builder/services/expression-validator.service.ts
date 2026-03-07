import { Injectable } from '@angular/core';

export interface ValidationResult {
  isValid: boolean;
  errors: string[];
}

@Injectable({
  providedIn: 'root'
})
export class ExpressionValidatorService {
  private readonly IDENTIFIER_REGEX = /[a-zA-Z0-9]+/g;
  private readonly VALID_OPERATORS = ['+', '-', '*', '/', '^', '(', ')'];

  validate(expression: string, validIdentifiers: string[]): ValidationResult {
    const errors: string[] = [];

    if (!expression || expression.trim().length === 0) {
      return { isValid: true, errors: [] };
    }

    // Check syntax
    const syntaxErrors = this.checkSyntax(expression);
    errors.push(...syntaxErrors);

    // Check identifiers
    const identifierErrors = this.checkIdentifiers(expression, validIdentifiers);
    errors.push(...identifierErrors);

    return {
      isValid: errors.length === 0,
      errors
    };
  }

  private checkSyntax(expression: string): string[] {
    const errors: string[] = [];

    // Check balanced parentheses
    let balance = 0;
    for (let i = 0; i < expression.length; i++) {
      if (expression[i] === '(') balance++;
      if (expression[i] === ')') balance--;
      if (balance < 0) {
        errors.push('Syntax error: unbalanced parentheses');
        break;
      }
    }
    if (balance > 0) {
      errors.push('Syntax error: unbalanced parentheses');
    }

    // Check for invalid operator placement
    const trimmed = expression.trim();
    const operators = ['+', '-', '*', '/', '^'];
    
    // Check if starts or ends with operator (excluding minus for negative numbers)
    if (operators.includes(trimmed[0]) && trimmed[0] !== '-') {
      errors.push('Syntax error: expression cannot start with an operator');
    }
    if (operators.includes(trimmed[trimmed.length - 1])) {
      errors.push('Syntax error: expression cannot end with an operator');
    }

    // Check for consecutive operators
    for (let i = 0; i < expression.length - 1; i++) {
      const current = expression[i];
      const next = expression[i + 1];
      
      if (operators.includes(current) && operators.includes(next)) {
        // Allow negative numbers like "5 + -3"
        if (!(current !== '-' && next === '-')) {
          errors.push(`Syntax error: consecutive operators at position ${i}`);
          break;
        }
      }
    }

    return errors;
  }

  private checkIdentifiers(expression: string, validIdentifiers: string[]): string[] {
    const errors: string[] = [];
    const extractedIdentifiers = this.extractIdentifiers(expression);
    const undefinedIdentifiers = extractedIdentifiers.filter(
      id => !validIdentifiers.includes(id)
    );

    if (undefinedIdentifiers.length > 0) {
      errors.push(`Undefined identifiers: ${undefinedIdentifiers.join(', ')}`);
    }

    return errors;
  }

  private extractIdentifiers(expression: string): string[] {
    const matches = expression.match(this.IDENTIFIER_REGEX);
    if (!matches) return [];
    
    // Remove duplicates and return
    return Array.from(new Set(matches));
  }
}
