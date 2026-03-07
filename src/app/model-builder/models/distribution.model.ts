export type DistributionType = 'Normal' | 'Lognormal' | 'Uniform';

export interface NormalDistribution {
  type: 'Normal';
  mean: number;
  stdDev: number;
}

export interface LognormalDistribution {
  type: 'Lognormal';
  mean: number;
  stdDev: number;
}

export interface UniformDistribution {
  type: 'Uniform';
  min: number;
  max: number;
}

export type Distribution = NormalDistribution | LognormalDistribution | UniformDistribution;
