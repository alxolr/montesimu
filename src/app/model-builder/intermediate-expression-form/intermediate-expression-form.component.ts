import { Component, EventEmitter, Input, Output, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { ButtonModule } from 'primeng/button';
import { IntermediateExpression } from '../models/intermediate-expression.model';
import { ModelService, OperationResult } from '../services/model.service';
import { IdentifierValidatorService } from '../services/identifier-validator.service';
import { ExpressionValidatorService } from '../services/expression-validator.service';

@Component({
  selector: 'app-intermediate-expression-form',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    DialogModule,
    InputTextModule,
    ButtonModule
  ],
  templateUrl: './intermediate-expression-form.component.html',
  styleUrl: './intermediate-expression-form.component.css'
})
export class IntermediateExpressionFormComponent {
  @Input() visible = false;
  @Input() expression?: IntermediateExpression;
  @Output() visibleChange = new EventEmitter<boolean>();
  @Output() save = new EventEmitter<IntermediateExpression>();

  // Form data
  formData = {
    name: '',
    formula: ''
  };

  // Error messages
  nameError = signal<string>('');
  formulaError = signal<string>('');

  // Edit mode flag
  get isEditMode(): boolean {
    return !!this.expression;
  }

  constructor(
    public modelService: ModelService,
    private identifierValidator: IdentifierValidatorService,
    private expressionValidator: ExpressionValidatorService
  ) {}

  ngOnChanges(): void {
    if (this.visible && this.expression) {
      // Edit mode - populate form
      this.formData.name = this.expression.name;
      this.formData.formula = this.expression.formula;
    } else if (this.visible) {
      // Add mode - reset form
      this.formData.name = '';
      this.formData.formula = '';
    }
    
    // Clear errors when dialog opens
    if (this.visible) {
      this.nameError.set('');
      this.formulaError.set('');
    }
  }

  validateName(): void {
    if (!this.formData.name) {
      this.nameError.set('Name is required');
      return;
    }

    // Validate identifier format
    const identifierResult = this.identifierValidator.validate(this.formData.name);
    if (!identifierResult.isValid) {
      this.nameError.set(identifierResult.error || 'Invalid identifier');
      return;
    }

    // Check uniqueness (exclude current name in edit mode)
    const excludeName = this.isEditMode ? this.expression!.name : undefined;
    if (!this.modelService.isIdentifierUnique(this.formData.name, excludeName)) {
      this.nameError.set('An identifier with this name already exists');
      return;
    }

    this.nameError.set('');
  }

  validateFormula(): void {
    if (!this.formData.formula) {
      this.formulaError.set('Formula is required');
      return;
    }

    // Validate expression syntax and references
    const validationResult = this.expressionValidator.validate(
      this.formData.formula,
      this.modelService.allIdentifiers()
    );

    if (!validationResult.isValid) {
      this.formulaError.set(validationResult.errors.join(', '));
      return;
    }

    this.formulaError.set('');
  }

  isFormValid(): boolean {
    return this.formData.name !== '' &&
           this.formData.formula !== '' &&
           this.nameError() === '' &&
           this.formulaError() === '';
  }

  onSubmit(event: Event): void {
    event.preventDefault();

    // Validate all fields
    this.validateName();
    this.validateFormula();

    if (!this.isFormValid()) {
      return;
    }

    const newExpression: IntermediateExpression = {
      name: this.formData.name,
      formula: this.formData.formula
    };

    // Try to add or update
    let result: OperationResult;
    if (this.isEditMode) {
      result = this.modelService.updateIntermediateExpression(
        this.expression!.name,
        newExpression
      );
    } else {
      result = this.modelService.addIntermediateExpression(newExpression);
    }

    if (!result.success) {
      // Display error (likely circular reference)
      this.formulaError.set(result.error || 'Operation failed');
      return;
    }

    // Success - emit save event and close
    this.save.emit(newExpression);
    this.onCancel();
  }

  onCancel(): void {
    this.visible = false;
    this.visibleChange.emit(false);
    
    // Reset form
    this.formData.name = '';
    this.formData.formula = '';
    this.nameError.set('');
    this.formulaError.set('');
  }
}
