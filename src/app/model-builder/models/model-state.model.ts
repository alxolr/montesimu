import { Variable } from './variable.model';

export interface ModelState {
  variables: Variable[];
  expression: string;
}
