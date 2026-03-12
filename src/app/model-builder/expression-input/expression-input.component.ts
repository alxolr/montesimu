import { Component, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { ModelService } from '../services/model.service';

@Component({
  selector: 'app-expression-input',
  standalone: true,
  imports: [CommonModule, FormsModule, InputTextModule],
  templateUrl: './expression-input.component.html',
  styleUrl: './expression-input.component.css'
})
export class ExpressionInputComponent {
  expressionText: string = '';

  // Computed signals from ModelService
  validationResult = computed(() => this.modelService.targetExpressionValidation());
  availableIdentifiers = computed(() => this.modelService.allIdentifiers());

  constructor(private modelService: ModelService) {
    // Initialize with current target expression
    this.expressionText = this.modelService.targetExpression();
  }

  onExpressionChange(value: string): void {
    this.modelService.setTargetExpression(value);
  }
}
