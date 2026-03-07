---
inclusion: fileMatch
fileMatchPattern: '**/*.spec.ts,**/*.test.ts,**/tests/**'
---

# Testing Guidelines

## Testing Philosophy
- Write tests that provide confidence in code correctness
- Focus on behavior, not implementation details
- Test edge cases and error conditions
- Keep tests maintainable and readable

## Frontend Testing (Angular)

### Unit Testing with Jasmine/Karma

#### Component Testing
```typescript
import { ComponentFixture, TestBed } from '@angular/core/testing';
import { HistogramComponent } from './histogram.component';

describe('HistogramComponent', () => {
  let component: HistogramComponent;
  let fixture: ComponentFixture<HistogramComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HistogramComponent]  // Standalone component
    }).compileComponents();

    fixture = TestBed.createComponent(HistogramComponent);
    component = fixture.componentInstance;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should initialize chart with provided data', () => {
    component.labels = ['0-10', '10-20'];
    component.dataValues = [5, 10];
    component.ngOnInit();
    
    expect(component.data.labels).toEqual(['0-10', '10-20']);
    expect(component.data.datasets[0].data).toEqual([5, 10]);
  });

  it('should update chart when inputs change', () => {
    const changes = {
      labels: { currentValue: ['0-10'], previousValue: [], firstChange: false },
      dataValues: { currentValue: [5], previousValue: [], firstChange: false }
    };
    
    component.ngOnChanges(changes);
    
    expect(component.data.labels).toEqual(['0-10']);
  });
});
```

#### Testing Tauri Commands
```typescript
import { invoke } from '@tauri-apps/api/core';

// Mock Tauri invoke
jest.mock('@tauri-apps/api/core', () => ({
  invoke: jest.fn()
}));

describe('AppComponent Tauri Integration', () => {
  it('should call generate_histogram command', async () => {
    const mockResult = [
      { label: '0-10', count: 5, min: 0, max: 10 }
    ];
    
    (invoke as jest.Mock).mockResolvedValue(mockResult);
    
    const component = new AppComponent();
    component.generateHistogram();
    
    expect(invoke).toHaveBeenCalledWith('generate_histogram', {
      dataset: expect.any(Array),
      numBins: 50
    });
  });
});
```

### Testing Signals
```typescript
it('should update signal values', () => {
  const component = new AppComponent();
  
  component.labels.set(['0-10', '10-20']);
  component.dataValues.set([5, 10]);
  
  expect(component.labels()).toEqual(['0-10', '10-20']);
  expect(component.dataValues()).toEqual([5, 10]);
});
```

### Running Frontend Tests
```bash
# Run all tests
ng test

# Run tests once (CI mode)
ng test --watch=false

# Run with coverage
ng test --code-coverage
```

## Backend Testing (Rust)

### Unit Testing

#### Service Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_histogram_empty_dataset() {
        let service = HistogramServiceImpl;
        let result = service.calculate(&[], 10);
        
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_histogram_single_value() {
        let service = HistogramServiceImpl;
        let dataset = vec![5.0];
        let result = service.calculate(&dataset, 10);
        
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].count, 1);
        assert_eq!(result[0].min, 5.0);
        assert_eq!(result[0].max, 5.0);
    }

    #[test]
    fn test_histogram_normal_distribution() {
        let service = HistogramServiceImpl;
        let dataset = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = service.calculate(&dataset, 5);
        
        assert_eq!(result.len(), 5);
        
        // Verify bins cover the range
        assert_eq!(result[0].min, 1.0);
        assert_eq!(result[4].max, 10.0);
        
        // Verify all data points are counted
        let total_count: usize = result.iter().map(|b| b.count).sum();
        assert_eq!(total_count, dataset.len());
    }

    #[test]
    fn test_histogram_zero_bins() {
        let service = HistogramServiceImpl;
        let dataset = vec![1.0, 2.0, 3.0];
        let result = service.calculate(&dataset, 0);
        
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_histogram_bin_boundaries() {
        let service = HistogramServiceImpl;
        let dataset = vec![0.0, 5.0, 10.0];
        let result = service.calculate(&dataset, 2);
        
        // First bin: [0, 5)
        // Last bin: [5, 10] (inclusive)
        assert_eq!(result.len(), 2);
        assert!(result[0].count > 0);
        assert!(result[1].count > 0);
    }
}
```

#### Testing with Mock Dependencies
```rust
use shaku::HasComponent;

#[cfg(test)]
mod tests {
    use super::*;
    
    // Create test container
    fn create_test_container() -> Container {
        Container::builder().build()
    }
    
    #[test]
    fn test_service_resolution() {
        let container = create_test_container();
        let service: &dyn HistogramService = container.resolve_ref();
        
        let result = service.calculate(&[1.0, 2.0, 3.0], 2);
        assert_eq!(result.len(), 2);
    }
}
```

### Integration Testing
```rust
// tests/integration_test.rs
use montesimu_lib::services::{Container, histogram::HistogramService};
use shaku::HasComponent;

#[test]
fn test_histogram_integration() {
    let container = Container::builder().build();
    let service: &dyn HistogramService = container.resolve_ref();
    
    let dataset: Vec<f64> = (0..1000).map(|x| x as f64).collect();
    let result = service.calculate(&dataset, 10);
    
    assert_eq!(result.len(), 10);
    
    // Verify distribution
    let total: usize = result.iter().map(|b| b.count).sum();
    assert_eq!(total, 1000);
}
```

### Running Backend Tests
```bash
cd src-tauri

# Run all tests
cargo test

# Run specific test
cargo test test_histogram_empty_dataset

# Run with output
cargo test -- --nocapture

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

## Property-Based Testing

### When to Use
- Testing mathematical properties (commutativity, associativity)
- Validating invariants across many inputs
- Finding edge cases automatically

### Example with Rust (using proptest)
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_histogram_count_invariant(
        dataset in prop::collection::vec(any::<f64>(), 1..1000),
        num_bins in 1usize..100
    ) {
        let service = HistogramServiceImpl;
        let result = service.calculate(&dataset, num_bins);
        
        // Property: sum of bin counts equals dataset length
        let total_count: usize = result.iter().map(|b| b.count).sum();
        prop_assert_eq!(total_count, dataset.len());
    }
    
    #[test]
    fn test_histogram_range_coverage(
        dataset in prop::collection::vec(any::<f64>(), 1..1000),
        num_bins in 1usize..100
    ) {
        let service = HistogramServiceImpl;
        let result = service.calculate(&dataset, num_bins);
        
        if !result.is_empty() {
            let min = dataset.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = dataset.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            
            // Property: bins cover the entire data range
            prop_assert_eq!(result[0].min, min);
            prop_assert_eq!(result[result.len() - 1].max, max);
        }
    }
}
```

## Test Coverage Goals
- **Unit Tests**: 80%+ coverage for business logic
- **Integration Tests**: Cover critical user flows
- **Edge Cases**: Empty inputs, boundary values, large datasets
- **Error Cases**: Invalid inputs, error conditions

## Testing Best Practices

### General
- **Arrange-Act-Assert**: Structure tests clearly
- **One assertion per test**: Focus on single behavior
- **Descriptive names**: Test names should explain what's being tested
- **Independent tests**: No shared state between tests
- **Fast tests**: Keep unit tests quick

### Frontend
- **Test behavior, not implementation**: Don't test private methods
- **Mock external dependencies**: Isolate component under test
- **Test user interactions**: Simulate clicks, inputs, etc.
- **Accessibility**: Test keyboard navigation and screen readers

### Backend
- **Test public API**: Focus on service interfaces
- **Test edge cases**: Empty, null, boundary values
- **Test error handling**: Verify error conditions
- **Performance tests**: For computationally intensive code

## Continuous Integration
```bash
# CI test script
#!/bin/bash

# Frontend tests
npm test -- --watch=false --code-coverage

# Backend tests
cd src-tauri
cargo test
cargo clippy -- -D warnings

# Check formatting
cargo fmt -- --check
```

## Test Maintenance
- Review and update tests when requirements change
- Remove obsolete tests
- Refactor tests to reduce duplication
- Keep test data realistic but minimal
