import { Component, signal } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';
import { TooltipModule } from 'primeng/tooltip';
import { IntermediateExpression } from '../models/intermediate-expression.model';
import { ModelService } from '../services/model.service';
import { IntermediateExpressionFormComponent } from '../intermediate-expression-form/intermediate-expression-form.component';

@Component({
  selector: 'app-intermediate-expression-list',
  standalone: true,
  imports: [
    CommonModule,
    CardModule,
    ButtonModule,
    TooltipModule,
    IntermediateExpressionFormComponent
  ],
  templateUrl: './intermediate-expression-list.component.html',
  styleUrl: './intermediate-expression-list.component.css'
})
export class IntermediateExpressionListComponent {
  // Dialog state
  formDialogVisible = signal(false);
  selectedExpression = signal<IntermediateExpression | undefined>(undefined);

  constructor(public modelService: ModelService) {}

  openAddDialog(): void {
    this.selectedExpression.set(undefined);
    this.formDialogVisible.set(true);
  }

  openEditDialog(expression: IntermediateExpression): void {
    this.selectedExpression.set(expression);
    this.formDialogVisible.set(true);
  }

  deleteExpression(expression: IntermediateExpression): void {
    const result = this.modelService.deleteIntermediateExpression(expression.name);
    
    if (!result.success) {
      // Show error message (in a real app, use a toast/notification service)
      alert(result.error);
    }
  }

  getDependencies(expression: IntermediateExpression): string[] {
    return this.modelService.getDependencies(expression.formula);
  }

  getReferencedBy(expression: IntermediateExpression): string[] {
    return this.modelService.getExpressionsReferencingIdentifier(expression.name);
  }
}
