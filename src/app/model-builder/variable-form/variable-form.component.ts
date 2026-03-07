import { Component, EventEmitter, Input, Output, OnInit, OnChanges, SimpleChanges } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { SelectModule } from 'primeng/select';
import { ButtonModule } from 'primeng/button';
import { Variable, DistributionType, Distribution } from '../models';
import { IdentifierValidatorService } from '../services/identifier-validator.service';
import { ModelService } from '../services/model.service';
import { DistributionPreviewComponent } from '../distribution-preview/distribution-preview.component';

interface DistributionOption {
  label: string;
  value: DistributionType;
}

@Component({
  selector: 'app-variable-form',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    DialogModule,
    InputTextModule,
    SelectModule,
    ButtonModule,
    DistributionPreviewComponent
  ],
  templateUrl: './variable-form.component.html',
  styleUrl: './variable-form.component.css'
})
export class VariableFormComponent implements OnInit, OnChanges {
  @Input() visible: boolean = false;
  @Input() variable?: Variable;
  @Output() visibleChange = new EventEmitter<boolean>();
  @Output() save = new EventEmitter<Variable>();

  formData: {
    name: string;
    distribution: DistributionType;
    mean?: number;
    stdDev?: number;
    min?: number;
    max?: number;
  } = {
    name: '',
    distribution: 'Normal'
  };

  distributionOptions: DistributionOption[] = [
    { label: 'Normal', value: 'Normal' },
    { label: 'Lognormal', value: 'Lognormal' },
    { label: 'Uniform', value: 'Uniform' }
  ];

  nameError: string = '';
  meanError: string = '';
  stdDevError: string = '';
  minError: string = '';
  maxError: string = '';

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
    if (changes['variable'] || changes['visible']) {
      this.initializeForm();
    }
  }

  initializeForm(): void {
    if (this.variable && this.visible) {
      this.isEditMode = true;
      this.originalName = this.variable.name;
      this.loadVariable(this.variable);
    } else if (this.visible && !this.variable) {
      this.resetForm();
    }
  }

  loadVariable(variable: Variable): void {
    this.formData.name = variable.name;
    this.formData.distribution = variable.distribution.type;

    if (variable.distribution.type === 'Normal' || variable.distribution.type === 'Lognormal') {
      this.formData.mean = variable.distribution.mean;
      this.formData.stdDev = variable.distribution.stdDev;
    } else if (variable.distribution.type === 'Uniform') {
      this.formData.min = variable.distribution.min;
      this.formData.max = variable.distribution.max;
    }
  }

  onDistributionChange(): void {
    // Reset parameter fields when distribution changes
    this.formData.mean = undefined;
    this.formData.stdDev = undefined;
    this.formData.min = undefined;
    this.formData.max = undefined;
    this.clearErrors();
  }

  onSubmit(event: Event): void {
    event.preventDefault();
    
    if (!this.isFormValid()) {
      return;
    }

    const distribution = this.buildDistribution();
    const variable: Variable = {
      name: this.formData.name,
      distribution
    };

    this.save.emit(variable);
    this.onCancel();
  }

  buildDistribution(): Distribution {
    if (this.formData.distribution === 'Normal') {
      return {
        type: 'Normal',
        mean: this.formData.mean!,
        stdDev: this.formData.stdDev!
      };
    } else if (this.formData.distribution === 'Lognormal') {
      return {
        type: 'Lognormal',
        mean: this.formData.mean!,
        stdDev: this.formData.stdDev!
      };
    } else {
      return {
        type: 'Uniform',
        min: this.formData.min!,
        max: this.formData.max!
      };
    }
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

    // Validate distribution parameters
    if (this.formData.distribution === 'Normal' || this.formData.distribution === 'Lognormal') {
      if (this.formData.mean === undefined || this.formData.mean === null) {
        this.meanError = 'Mean is required';
        isValid = false;
      } else if (isNaN(this.formData.mean)) {
        this.meanError = 'Mean must be a valid number';
        isValid = false;
      }

      if (this.formData.stdDev === undefined || this.formData.stdDev === null) {
        this.stdDevError = 'Standard deviation is required';
        isValid = false;
      } else if (isNaN(this.formData.stdDev)) {
        this.stdDevError = 'Standard deviation must be a valid number';
        isValid = false;
      } else if (this.formData.stdDev <= 0) {
        this.stdDevError = 'Standard deviation must be positive';
        isValid = false;
      }
    } else if (this.formData.distribution === 'Uniform') {
      if (this.formData.min === undefined || this.formData.min === null) {
        this.minError = 'Minimum is required';
        isValid = false;
      } else if (isNaN(this.formData.min)) {
        this.minError = 'Minimum must be a valid number';
        isValid = false;
      }

      if (this.formData.max === undefined || this.formData.max === null) {
        this.maxError = 'Maximum is required';
        isValid = false;
      } else if (isNaN(this.formData.max)) {
        this.maxError = 'Maximum must be a valid number';
        isValid = false;
      } else if (this.formData.min !== undefined && this.formData.max <= this.formData.min) {
        this.maxError = 'Maximum must be greater than minimum';
        isValid = false;
      }
    }

    return isValid;
  }

  clearErrors(): void {
    this.nameError = '';
    this.meanError = '';
    this.stdDevError = '';
    this.minError = '';
    this.maxError = '';
  }

  onCancel(): void {
    this.visible = false;
    this.visibleChange.emit(false);
    this.resetForm();
  }

  resetForm(): void {
    this.formData = {
      name: '',
      distribution: 'Normal'
    };
    this.clearErrors();
    this.isEditMode = false;
    this.originalName = '';
  }
}
