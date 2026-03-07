import { Injectable } from '@angular/core';
import { DistributionType } from '../models/distribution.model';

export interface PDFPoint {
  x: number;
  y: number;
}

@Injectable({
  providedIn: 'root'
})
export class PDFCalculatorService {
  private readonly DEFAULT_NUM_POINTS = 200;

  /**
   * Calculate Normal distribution PDF value at x
   * Formula: PDF(x) = (1 / (σ * √(2π))) * e^(-((x - μ)²) / (2σ²))
   */
  calculateNormalPDF(x: number, mean: number, stdDev: number): number {
    if (stdDev <= 0) return 0;
    
    const coefficient = 1 / (stdDev * Math.sqrt(2 * Math.PI));
    const exponent = -Math.pow(x - mean, 2) / (2 * Math.pow(stdDev, 2));
    return coefficient * Math.exp(exponent);
  }

  /**
   * Calculate Lognormal distribution PDF value at x
   * Formula: PDF(x) = (1 / (x * σ * √(2π))) * e^(-((ln(x) - μ)²) / (2σ²))
   */
  calculateLognormalPDF(x: number, mean: number, stdDev: number): number {
    if (x <= 0 || stdDev <= 0) return 0;
    
    const coefficient = 1 / (x * stdDev * Math.sqrt(2 * Math.PI));
    const exponent = -Math.pow(Math.log(x) - mean, 2) / (2 * Math.pow(stdDev, 2));
    return coefficient * Math.exp(exponent);
  }

  /**
   * Calculate Uniform distribution PDF value at x
   * Formula: PDF(x) = 1 / (max - min) if min ≤ x ≤ max, 0 otherwise
   */
  calculateUniformPDF(x: number, min: number, max: number): number {
    if (max <= min) return 0;
    if (x >= min && x <= max) {
      return 1 / (max - min);
    }
    return 0;
  }

  /**
   * Generate array of PDF points for visualization
   */
  generatePDFPoints(
    distribution: DistributionType,
    params: { mean?: number; stdDev?: number; min?: number; max?: number },
    numPoints: number = this.DEFAULT_NUM_POINTS
  ): PDFPoint[] {
    const range = this.calculateXRange(distribution, params);
    if (!range) return [];

    const { xMin, xMax } = range;
    const step = (xMax - xMin) / (numPoints - 1);
    const points: PDFPoint[] = [];

    for (let i = 0; i < numPoints; i++) {
      const x = xMin + i * step;
      let y = 0;

      switch (distribution) {
        case 'Normal':
          if (params.mean !== undefined && params.stdDev !== undefined) {
            y = this.calculateNormalPDF(x, params.mean, params.stdDev);
          }
          break;
        case 'Lognormal':
          if (params.mean !== undefined && params.stdDev !== undefined) {
            y = this.calculateLognormalPDF(x, params.mean, params.stdDev);
          }
          break;
        case 'Uniform':
          if (params.min !== undefined && params.max !== undefined) {
            y = this.calculateUniformPDF(x, params.min, params.max);
          }
          break;
      }

      points.push({ x, y });
    }

    return points;
  }

  /**
   * Calculate appropriate x-axis range for each distribution type
   */
  private calculateXRange(
    distribution: DistributionType,
    params: { mean?: number; stdDev?: number; min?: number; max?: number }
  ): { xMin: number; xMax: number } | null {
    switch (distribution) {
      case 'Normal':
        if (params.mean !== undefined && params.stdDev !== undefined && params.stdDev > 0) {
          return {
            xMin: params.mean - 4 * params.stdDev,
            xMax: params.mean + 4 * params.stdDev
          };
        }
        break;
      case 'Lognormal':
        if (params.mean !== undefined && params.stdDev !== undefined && params.stdDev > 0) {
          return {
            xMin: 0.01,
            xMax: params.mean + 4 * params.stdDev
          };
        }
        break;
      case 'Uniform':
        if (params.min !== undefined && params.max !== undefined && params.max > params.min) {
          const padding = 0.1 * (params.max - params.min);
          return {
            xMin: params.min - padding,
            xMax: params.max + padding
          };
        }
        break;
    }
    return null;
  }
}
