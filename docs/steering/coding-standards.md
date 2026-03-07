---
inclusion: auto
---

# Coding Standards

## TypeScript/Angular Standards

### General Rules
- Use strict TypeScript configuration (enabled in tsconfig.json)
- Enable all strict compiler options: `noImplicitOverride`, `noPropertyAccessFromIndexSignature`, `noImplicitReturns`, `noFallthroughCasesInSwitch`
- Target ES2022 with module preservation
- Use experimental decorators for Angular

### Component Standards
- Use standalone components (no NgModules)
- Prefer signals over traditional observables for state management
- Use `WritableSignal<T>` for mutable state
- Component selector prefix: `app-`
- Style encapsulation: Component-level CSS files
- Import only required modules in component imports array

### Naming Conventions
- Components: PascalCase with `.component.ts` suffix
- Services: PascalCase with `.service.ts` suffix
- Interfaces: PascalCase with `I` prefix (e.g., `IHistogramData`)
- Constants: UPPER_SNAKE_CASE
- Variables/functions: camelCase

### File Organization
- Co-locate related files (component, template, styles, tests)
- Use feature folders under `src/app/`
- Shared components go in `src/app/shared/components/`
- Core services go in `src/app/core/`

### Angular Specific
- Use `inject()` function for dependency injection in modern Angular
- Prefer `@Input()` signals for component inputs when possible
- Use `OnInit` and `OnChanges` lifecycle hooks appropriately
- Implement proper change detection strategies

## Rust Standards

### General Rules
- Follow Rust 2021 edition conventions
- Use `cargo fmt` for consistent formatting
- Run `cargo clippy` for linting
- Enable all compiler warnings

### Project Structure
- Organize code into modules: `commands/`, `services/`, `dtos/`
- Use `mod.rs` for module exports
- Keep business logic in services, not commands

### Dependency Injection (Shaku)
- Define traits for service interfaces
- Implement services with `#[derive(Component)]`
- Use `#[shaku(interface = TraitName)]` attribute
- Register all services in the Container module
- Resolve dependencies via `HasComponent` trait

### Tauri Commands
- Use `#[tauri::command]` attribute
- Accept `State<'_, Container>` for DI access
- Keep commands thin - delegate to services
- Use proper error handling with `Result<T, E>`

### Naming Conventions
- Modules: snake_case
- Structs/Enums: PascalCase
- Traits: PascalCase with descriptive names (e.g., `HistogramService`)
- Functions: snake_case
- Constants: UPPER_SNAKE_CASE

### Data Transfer Objects
- Use `#[derive(Debug, Serialize, Deserialize)]` for DTOs
- Keep DTOs in `dtos/` module
- Use descriptive field names
- Document complex structures

### Service Pattern
```rust
pub trait ServiceName: Interface {
    fn method_name(&self, param: Type) -> ReturnType;
}

#[derive(Component)]
#[shaku(interface = ServiceName)]
pub struct ServiceNameImpl;

impl ServiceName for ServiceNameImpl {
    fn method_name(&self, param: Type) -> ReturnType {
        // Implementation
    }
}
```

### Command Pattern
```rust
#[tauri::command]
pub fn command_name(
    state: State<'_, Container>,
    param: Type,
) -> ReturnType {
    let service: &dyn ServiceTrait = state.resolve_ref();
    service.method(param)
}
```

## Testing Standards

### TypeScript Tests
- Use Jasmine/Karma (Angular default)
- Test file suffix: `.spec.ts`
- Co-locate tests with source files
- Test component inputs, outputs, and user interactions
- Mock external dependencies

### Rust Tests
- Use built-in Rust test framework
- Unit tests in same file with `#[cfg(test)]`
- Integration tests in `tests/` directory
- Test public API surface
- Use `cargo test` to run tests

## Code Quality

### Documentation
- Document public APIs with JSDoc (TypeScript) and doc comments (Rust)
- Include usage examples for complex functions
- Document component inputs/outputs
- Explain non-obvious business logic

### Error Handling
- Use proper error types (Result in Rust, try-catch in TypeScript)
- Provide meaningful error messages
- Log errors appropriately
- Handle edge cases (empty arrays, null values, etc.)

### Performance
- Avoid unnecessary re-renders in Angular
- Use OnPush change detection when appropriate
- Optimize Rust algorithms for large datasets
- Profile performance-critical code

## Version Control
- Write clear, descriptive commit messages
- Use conventional commits format when possible
- Keep commits focused and atomic
- Review code before committing
