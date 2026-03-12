import { Variable } from './variable.model';
import { IntermediateExpression } from './intermediate-expression.model';

export interface ModelState {
  variables: Variable[];
  intermediateExpressions: IntermediateExpression[];
  targetExpression: string;
}
