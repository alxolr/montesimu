---
inclusion: fileMatch
fileMatchPattern: '**/services/**,**/commands/**,**/components/**'
---

# Architecture Patterns

## Overview
Montesimu follows a clean architecture approach with clear separation between frontend (Angular) and backend (Rust/Tauri), connected via Tauri's IPC mechanism.

## Backend Architecture (Rust)

### Layered Architecture
```
┌─────────────────────────────────────┐
│     Tauri Commands (API Layer)     │  ← Entry points for frontend
├─────────────────────────────────────┤
│     Services (Business Logic)      │  ← Core application logic
├─────────────────────────────────────┤
│     DTOs (Data Transfer Objects)   │  ← Data contracts
└─────────────────────────────────────┘
```

### Dependency Injection Pattern
Uses Shaku for IoC (Inversion of Control):

```rust
// 1. Define service interface
pub trait HistogramService: Interface {
    fn calculate(&self, dataset: &[f64], num_bins: usize) -> Vec<HistogramBin>;
}

// 2. Implement service
#[derive(Component)]
#[shaku(interface = HistogramService)]
pub struct HistogramServiceImpl;

impl HistogramService for HistogramServiceImpl {
    fn calculate(&self, dataset: &[f64], num_bins: usize) -> Vec<HistogramBin> {
        // Implementation
    }
}

// 3. Register in container
module! {
    pub Container {
        components = [HistogramServiceImpl],
        providers = []
    }
}

// 4. Resolve in commands
#[tauri::command]
pub fn generate_histogram(
    state: State<'_, Container>,
    dataset: Vec<f64>,
    num_bins: usize,
) -> Vec<HistogramBin> {
    let service: &dyn HistogramService = state.resolve_ref();
    service.calculate(&dataset, num_bins)
}
```

### Benefits
- **Testability**: Services can be mocked/stubbed
- **Maintainability**: Clear separation of concerns
- **Extensibility**: Easy to add new services
- **Type Safety**: Compile-time dependency resolution

### Command Pattern
Commands are thin wrappers that:
1. Accept Tauri State and parameters
2. Resolve service from DI container
3. Delegate to service
4. Return result to frontend

**Do NOT** put business logic in commands - keep them focused on orchestration.

## Frontend Architecture (Angular)

### Component Architecture
```
┌─────────────────────────────────────┐
│      App Component (Root)           │
├─────────────────────────────────────┤
│      Feature Components             │
├─────────────────────────────────────┤
│      Shared Components              │  ← Reusable UI components
└─────────────────────────────────────┘
```

### Standalone Components
Modern Angular approach without NgModules:

```typescript
@Component({
  selector: 'app-histogram',
  standalone: true,
  imports: [ChartModule, CardModule],  // Direct imports
  templateUrl: './histogram.component.html'
})
export class HistogramComponent {
  // Component logic
}
```

### Signal-Based State Management
Use Angular signals for reactive state:

```typescript
export class AppComponent {
  // Writable signals for mutable state
  labels: WritableSignal<string[]> = signal([]);
  dataValues: WritableSignal<number[]> = signal([]);
  title: WritableSignal<string> = signal('Histogram');

  updateData(newLabels: string[], newValues: number[]) {
    this.labels.set(newLabels);
    this.dataValues.set(newValues);
  }
}
```

### Benefits
- **Reactivity**: Automatic UI updates
- **Performance**: Fine-grained change detection
- **Simplicity**: Less boilerplate than observables
- **Type Safety**: Full TypeScript support

## Communication Patterns

### Frontend → Backend (Tauri IPC)
```typescript
import { invoke } from '@tauri-apps/api/core';

// Call Rust command
invoke<HistogramBin[]>('generate_histogram', {
  dataset: [1, 2, 3, 4, 5],
  numBins: 10
}).then(result => {
  // Handle result
});
```

### Data Flow
```
User Interaction
    ↓
Angular Component
    ↓
Tauri invoke()
    ↓
Tauri Command
    ↓
Service (Business Logic)
    ↓
Return DTO
    ↓
Angular Component (Update Signals)
    ↓
UI Update
```

## Design Patterns in Use

### Service Layer Pattern
- Encapsulates business logic
- Provides reusable functionality
- Independent of UI and framework

### DTO Pattern
- Defines data contracts between layers
- Ensures type safety across boundaries
- Serializable with Serde

### Component Pattern
- Self-contained UI units
- Reusable and composable
- Clear inputs and outputs

### Dependency Injection
- Loose coupling between components
- Easier testing and mocking
- Centralized configuration

## Best Practices

### Backend
1. **Keep commands thin**: Delegate to services
2. **Use traits for abstractions**: Enable testing and flexibility
3. **Validate inputs**: Check for edge cases in services
4. **Return proper types**: Use Result<T, E> for error handling
5. **Document public APIs**: Use doc comments

### Frontend
1. **Use signals for state**: Prefer over traditional observables
2. **Keep components focused**: Single responsibility
3. **Leverage PrimeNG**: Use existing UI components
4. **Handle errors**: Catch promise rejections from invoke()
5. **Type everything**: Leverage TypeScript's type system

### Integration
1. **Define clear contracts**: Use DTOs for data exchange
2. **Handle async properly**: Use async/await or promises
3. **Validate data**: Check data on both sides
4. **Error handling**: Graceful degradation on failures
5. **Performance**: Minimize IPC calls for large datasets

## Anti-Patterns to Avoid

### Backend
- ❌ Business logic in commands
- ❌ Direct state mutation without services
- ❌ Tight coupling between services
- ❌ Ignoring error cases

### Frontend
- ❌ Direct DOM manipulation
- ❌ Business logic in templates
- ❌ Unhandled promise rejections
- ❌ Excessive component nesting

### Integration
- ❌ Frequent small IPC calls (batch instead)
- ❌ Passing large objects unnecessarily
- ❌ Inconsistent data formats
- ❌ Missing error boundaries
