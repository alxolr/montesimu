import { Variable } from './variable.model';
import { Constant } from './constant.model';

export interface ModelState {
  variables: Variable[];
  constants: Constant[];
  expression: string;
}
