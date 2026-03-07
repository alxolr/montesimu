import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';
import { TooltipModule } from 'primeng/tooltip';
import { ModelService } from '../services/model.service';
import { Constant } from '../models';
import { ConstantFormComponent } from '../constant-form/constant-form.component';

@Component({
  selector: 'app-constant-list',
  standalone: true,
  imports: [CommonModule, CardModule, ButtonModule, TooltipModule, ConstantFormComponent],
  templateUrl: './constant-list.component.html',
  styleUrl: './constant-list.component.css'
})
export class ConstantListComponent {
  showDialog = false;
  editingConstant?: Constant;
  editingConstantName?: string;

  constructor(public modelService: ModelService) {}

  openAddDialog(): void {
    this.editingConstant = undefined;
    this.editingConstantName = undefined;
    this.showDialog = true;
  }

  openEditDialog(constant: Constant): void {
    this.editingConstant = constant;
    this.editingConstantName = constant.name;
    this.showDialog = true;
  }

  deleteConstant(constant: Constant): void {
    if (confirm(`Are you sure you want to delete constant "${constant.name}"?`)) {
      this.modelService.deleteConstant(constant.name);
    }
  }

  onConstantSave(constant: Constant): void {
    try {
      if (this.editingConstantName) {
        // Edit mode
        this.modelService.updateConstant(this.editingConstantName, constant);
      } else {
        // Add mode
        this.modelService.addConstant(constant);
      }
      this.showDialog = false;
    } catch (error) {
      console.error('Error saving constant:', error);
      alert(error instanceof Error ? error.message : 'Error saving constant');
    }
  }
}
