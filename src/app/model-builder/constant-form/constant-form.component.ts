import { Component, EventEmitter, Input, Output, OnInit, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { ButtonModule } from 'primeng/button';
import { Constant } from '../models';
import { IdentifierValidatorService } from '../services/identifier-validator.service';
import { ModelService } from '../services/model.service';

@Component({
  selector: 'app-constant-form',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    DialogModule,
    InputTextModule,
    ButtonModule
  ],
  templateUrl: './constant-form.component.html',
  styleUrl: './constant-form.component.css'
})
export class ConstantFormComponent implements OnInit, OnChanges {
  @Input() visible: boolean = false;
  @Input() constant?: Constant;
  @Output() visibleChange = new EventEmitter<boolean>();
  @Output() save = new EventEmitter<Constant>();

  formData: {
    name: string;
    value?: number;
  } = {
    name: ''
  };

  nameError: string = '';
  valueError: string = '';

  isEditMode: boolean = false;
  originalName: string = '';

  constructor(
    private identifierValidator: IdentifierValidatorService,
    private modelService: ModelService
  ) {}

  ngOnInit(): void {
    this.initializeForm();
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['constant'] || changes['visible']) {
      this.initializeForm();
    }
  }

  initializeForm(): void {
    if (this.constant && this.visible) {
      this.isEditMode = true;
      this.originalName = this.constant.name;
      this.loadConstant(this.constant);
    } else if (this.visible && !this.constant) {
      this.resetForm();
    }
  }

  loadConstant(constant: Constant): void {
    this.formData.name = constant.name;
    this.formData.value = constant.value;
  }

  onSubmit(event: Event): void {
    event.preventDefault();
    
    if (!this.isFormValid()) {
      return;
    }

    const constant: Constant = {
      name: this.formData.name,
      value: this.formData.value!
    };

    this.save.emit(constant);
    this.onCancel();
  }

  isFormValid(): boolean {
    this.clearErrors();
    let isValid = true;

    // Validate name
    const nameValidation = this.identifierValidator.validate(this.formData.name);
    if (!nameValidation.isValid) {
      this.nameError = nameValidation.error!;
      isValid = false;
    } else {
      // Check uniqueness
      const excludeName = this.isEditMode ? this.originalName : undefined;
      if (!this.modelService.isIdentifierUnique(this.formData.name, excludeName)) {
        this.nameError = 'A variable or constant with this name already exists';
        isValid = false;
      }
    }

    // Validate value
    if (this.formData.value === undefined || this.formData.value === null) {
      this.valueError = 'Value is required';
      isValid = false;
    } else if (isNaN(this.formData.value)) {
      this.valueError = 'Value must be a valid number';
      isValid = false;
    }

    return isValid;
  }

  clearErrors(): void {
    this.nameError = '';
    this.valueError = '';
  }

  onCancel(): void {
    this.visible = false;
    this.visibleChange.emit(false);
    this.resetForm();
  }

  resetForm(): void {
    this.formData = {
      name: ''
    };
    this.clearErrors();
    this.isEditMode = false;
    this.originalName = '';
  }
}
