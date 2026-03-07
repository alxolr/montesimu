import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';
import { TooltipModule } from 'primeng/tooltip';
import { ModelService } from '../services/model.service';
import { Variable } from '../models';
import { VariableFormComponent } from '../variable-form/variable-form.component';

@Component({
  selector: 'app-variable-list',
  standalone: true,
  imports: [CommonModule, CardModule, ButtonModule, TooltipModule, VariableFormComponent],
  templateUrl: './variable-list.component.html',
  styleUrl: './variable-list.component.css'
})
export class VariableListComponent {
  showDialog = false;
  editingVariable?: Variable;
  editingVariableName?: string;

  constructor(public modelService: ModelService) {}

  openAddDialog(): void {
    this.editingVariable = undefined;
    this.editingVariableName = undefined;
    this.showDialog = true;
  }

  openEditDialog(variable: Variable): void {
    this.editingVariable = variable;
    this.editingVariableName = variable.name;
    this.showDialog = true;
  }

  deleteVariable(variable: Variable): void {
    if (confirm(`Are you sure you want to delete variable "${variable.name}"?`)) {
      this.modelService.deleteVariable(variable.name);
    }
  }

  onVariableSave(variable: Variable): void {
    try {
      if (this.editingVariableName) {
        // Edit mode
        this.modelService.updateVariable(this.editingVariableName, variable);
      } else {
        // Add mode
        this.modelService.addVariable(variable);
      }
      this.showDialog = false;
    } catch (error) {
      console.error('Error saving variable:', error);
      alert(error instanceof Error ? error.message : 'Error saving variable');
    }
  }

  formatParams(variable: Variable): string {
    if (variable.distribution.type === 'Normal' || variable.distribution.type === 'Lognormal') {
      return `μ=${variable.distribution.mean}, σ=${variable.distribution.stdDev}`;
    } else {
      return `min=${variable.distribution.min}, max=${variable.distribution.max}`;
    }
  }
}
