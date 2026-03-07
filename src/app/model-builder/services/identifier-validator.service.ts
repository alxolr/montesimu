import { Injectable } from '@angular/core';

export interface IdentifierValidationResult {
  isValid: boolean;
  error?: string;
}

@Injectable({
  providedIn: 'root'
})
export class IdentifierValidatorService {
  private readonly ALPHANUMERIC_REGEX = /^[a-zA-Z0-9]+$/;

  validate(name: string): IdentifierValidationResult {
    if (!name || name.trim().length === 0) {
      return {
        isValid: false,
        error: 'Name is required'
      };
    }

    if (!this.ALPHANUMERIC_REGEX.test(name)) {
      return {
        isValid: false,
        error: 'Name must contain only alphanumeric characters'
      };
    }

    return {
      isValid: true
    };
  }
}
