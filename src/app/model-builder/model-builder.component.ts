import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardModule } from 'primeng/card';
import { VariableListComponent } from './variable-list/variable-list.component';
import { ExpressionInputComponent } from './expression-input/expression-input.component';
import { ModelService } from './services/model.service';

@Component({
  selector: 'app-model-builder',
  standalone: true,
  imports: [
    CommonModule,
    CardModule,
    VariableListComponent,
    ExpressionInputComponent
  ],
  templateUrl: './model-builder.component.html',
  styleUrl: './model-builder.component.css'
})
export class ModelBuilderComponent {
  constructor(public modelService: ModelService) {}
}
