import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { CardModule } from 'primeng/card';
import { VariableListComponent } from './variable-list/variable-list.component';
import { IntermediateExpressionListComponent } from './intermediate-expression-list/intermediate-expression-list.component';
import { TargetExpressionInputComponent } from './target-expression-input/target-expression-input.component';
import { ModelService } from './services/model.service';

@Component({
  selector: 'app-model-builder',
  standalone: true,
  imports: [
    CommonModule,
    CardModule,
    VariableListComponent,
    IntermediateExpressionListComponent,
    TargetExpressionInputComponent
  ],
  templateUrl: './model-builder.component.html',
  styleUrl: './model-builder.component.css'
})
export class ModelBuilderComponent {
  constructor(public modelService: ModelService) {}
}
