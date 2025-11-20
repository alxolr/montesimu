import { Component, signal, WritableSignal } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { CardModule } from "primeng/card";
import { ButtonModule } from "primeng/button";
import { InputTextModule } from "primeng/inputtext";
import { HistogramComponent } from "./shared/components/histogram/histogram.component";

@Component({
  selector: "app-root",
  imports: [RouterOutlet, CardModule, ButtonModule, InputTextModule, HistogramComponent],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  greetingMessage = "";

  labels: WritableSignal<string[]> = signal([]);
  dataValues: WritableSignal<number[]> = signal([]);
  title: WritableSignal<string> = signal('Histogram');


  greet(event: SubmitEvent, name: string): void {
    event.preventDefault();

    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    invoke<string>("greet", { name }).then((text) => {
      this.greetingMessage = text;

      this.generateHistogram();
    });
  }

  generateHistogram(): void {
    // Generate normal distribution using Box-Muller transform
    const normalRandom = () => {
      const u1 = Math.random();
      const u2 = Math.random();
      return Math.sqrt(-2 * Math.log(u1)) * Math.cos(2 * Math.PI * u2);
    };

    invoke<any>("generate_histogram", {
      dataset: Array.from({ length: 100000 }, () => normalRandom() * 10 + 50),
      numBins: 50,
    }).then((histogram) => {
      const labels = histogram.map((bin: any) => bin.label);
      const dataValues = histogram.map((bin: any) => bin.count);

      this.labels.set(labels);
      this.dataValues.set(dataValues);
      this.title.set('Normal Distribution Histogram');
    });
  }
}
