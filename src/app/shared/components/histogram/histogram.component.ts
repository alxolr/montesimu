
import { Component, OnInit, Input, OnChanges, SimpleChanges } from '@angular/core';
import { CardModule } from 'primeng/card';
import { ChartModule } from 'primeng/chart';

@Component({
  selector: 'app-histogram',
  templateUrl: './histogram.component.html',
  standalone: true,
  imports: [ChartModule, CardModule]
})
export class HistogramComponent implements OnInit, OnChanges {
  @Input() labels: string[] = [];
  @Input() dataValues: number[] = [];
  @Input() title: string = 'Histogram';

  data: any;
  options: any;

  ngOnInit(): void {
    this.initializeChart();
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['labels'] || changes['dataValues'] || changes['title']) {
      this.initializeChart();
    }
  }


  initializeChart(): void {
    const backgrounds = ['rgba(9, 76, 124, 0.2)'];
    const borders = ['rgba(9, 76, 124, 1)'];

    this.data = {
      labels: this.labels,
      datasets: [
        {
          label: this.title,
          backgroundColor: backgrounds,
          borderColor: borders,
          borderWidth: 1,
          data: this.dataValues
        },
      ]
    };

    this.options = {
      maintainAspectRatio: false,
      responsive: true,
      devicePixelRatio: 2,
      animation: {
        duration: 0
      },
      plugins: {
        legend: {
          labels: {
            color: "#000000",
            font: {
              size: 12
            }
          }
        }
      },
      scales: {
        x: {
          ticks: {
            color: "#757575",
            font: {
              weight: 500
            }
          },
          grid: {
            color: "#e0e0e0",
            drawBorder: false
          }
        },
        y: {
          ticks: {
            color: "#757575"
          },
          grid: {
            color: "#e0e0e0",
            drawBorder: false
          }
        }
      }
    };
  }
}
