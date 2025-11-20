
import { Component, OnInit, PLATFORM_ID, ChangeDetectorRef, inject, effect } from '@angular/core';
import { CardModule } from 'primeng/card';
import { ChartModule } from 'primeng/chart';

@Component({
  selector: 'app-histogram',
  templateUrl: './histogram.component.html',
  standalone: true,
  imports: [ChartModule, CardModule]
})
export class HistogramComponent {
  data: any;
  options: any;

  constructor(private cd: ChangeDetectorRef) {



    this.data = {
      labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July', "August", "September", "October", "November", "December"],
      datasets: [
        {
          label: 'My First dataset',
          backgroundColor: [
            'rgba(54, 162, 235, 0.2)', // 70% opacity
          ],
          borderColor: [
            'rgba(54, 162, 235, 1)',
          ],
          borderWidth: 1,
          data: [65, 59, 80, 81, 56, 55, 40, 45, 50, 55, 60, 65]
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