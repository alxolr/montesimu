import { Component, signal, WritableSignal } from "@angular/core";
import { CommonModule } from "@angular/common";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { CardModule } from "primeng/card";
import { ButtonModule } from "primeng/button";
import { InputTextModule } from "primeng/inputtext";
import { HistogramComponent } from "./shared/components/histogram/histogram.component";
import { VariableFormComponent } from "./model-builder/variable-form/variable-form.component";
import { Variable } from "./model-builder/models";
import { ModelService } from "./model-builder/services";

@Component({
  selector: "app-root",
  imports: [CommonModule, RouterOutlet, CardModule, ButtonModule, InputTextModule, HistogramComponent, VariableFormComponent],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  greetingMessage = "";

  labels: WritableSignal<string[]> = signal([]);
  dataValues: WritableSignal<number[]> = signal([]);
  title: WritableSignal<string> = signal('Histogram');

  // Model builder test
  showVariableDialog = false;

  constructor(public modelService: ModelService) {}


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

  openVariableDialog(): void {
    this.showVariableDialog = true;
  }

  onVariableSave(variable: Variable): void {
    try {
      this.modelService.addVariable(variable);
      console.log('Variable added:', variable);
      console.log('All variables:', this.modelService.variables());
    } catch (error) {
      console.error('Error adding variable:', error);
    }
  }
}
