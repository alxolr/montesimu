import { Component, computed, input } from '@angular/core';
import { CommonModule } from '@angular/common';
import { ChartModule } from 'primeng/chart';
import { DistributionType } from '../models/distribution.model';
import { PDFCalculatorService } from '../services/pdf-calculator.service';

@Component({
  selector: 'app-distribution-preview',
  standalone: true,
  imports: [CommonModule, ChartModule],
  templateUrl: './distribution-preview.component.html',
  styleUrl: './distribution-preview.component.css'
})
export class DistributionPreviewComponent {
  // Inputs
  distribution = input.required<DistributionType>();
  mean = input<number>();
  stdDev = input<number>();
  min = input<number>();
  max = input<number>();

  // Chart configuration
  chartOptions = {
    maintainAspectRatio: false,
    responsive: true,
    devicePixelRatio: 2,
    animation: {
      duration: 0
    },
    plugins: {
      legend: {
        display: false
      },
      title: {
        display: true,
        text: 'Probability Density Function',
        color: '#000000',
        font: {
          size: 14,
          weight: 500
        }
      }
    },
    scales: {
      x: {
        title: {
          display: true,
          text: 'Value',
          color: '#000000',
          font: {
            size: 12,
            weight: 500
          }
        },
        ticks: {
          color: '#757575',
          font: { size: 11 }
        },
        grid: {
          color: '#e0e0e0',
          drawBorder: false
        }
      },
      y: {
        title: {
          display: true,
          text: 'Probability Density',
          color: '#000000',
          font: {
            size: 12,
            weight: 500
          }
        },
        ticks: {
          color: '#757575',
          font: { size: 11 }
        },
        grid: {
          color: '#e0e0e0',
          drawBorder: false
        }
      }
    }
  };

  // Computed signal for chart data
  chartData = computed(() => {
    const points = this.pdfCalculator.generatePDFPoints(
      this.distribution(),
      {
        mean: this.mean(),
        stdDev: this.stdDev(),
        min: this.min(),
        max: this.max()
      }
    );

    if (points.length === 0) {
      return {
        labels: [],
        datasets: []
      };
    }

    return {
      labels: points.map(p => p.x.toFixed(2)),
      datasets: [
        {
          label: 'PDF',
          data: points.map(p => p.y),
          borderColor: 'rgba(9, 76, 124, 1)',
          backgroundColor: 'rgba(9, 76, 124, 0.2)',
          fill: true,
          tension: 0.4,
          pointRadius: 0,
          borderWidth: 2
        }
      ]
    };
  });

  constructor(private pdfCalculator: PDFCalculatorService) {}
}
