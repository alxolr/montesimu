import { Component, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { InputTextModule } from 'primeng/inputtext';
import { CardModule } from 'primeng/card';
import { ModelService } from '../services/model.service';

@Component({
  selector: 'app-target-expression-input',
  standalone: true,
  imports: [CommonModule, FormsModule, InputTextModule, CardModule],
  templateUrl: './target-expression-input.component.html',
  styleUrl: './target-expression-input.component.css'
})
export class TargetExpressionInputComponent {
  expressionText: string = '';

  // Computed signals from ModelService
  validationResult = computed(() => this.modelService.targetExpressionValidation());
  availableIdentifiers = computed(() => this.modelService.allIdentifiers());
  dependencies = computed(() => {
    const expr = this.modelService.targetExpression();
    return expr ? this.modelService.getDependencies(expr) : [];
  });

  constructor(public modelService: ModelService) {
    // Initialize with current target expression
    this.expressionText = this.modelService.targetExpression();
  }

  onExpressionChange(value: string): void {
    this.modelService.setTargetExpression(value);
    this.expressionText = value;
  }
}
