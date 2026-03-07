---
inclusion: auto
---

# Development Workflow

## Environment Setup

### Prerequisites
- Node.js (for Angular)
- Rust toolchain (for Tauri backend)
- npm or yarn package manager

### Initial Setup
```bash
# Install frontend dependencies
npm install

# Install Tauri CLI (if not already installed)
npm install -g @tauri-apps/cli

# Verify Rust installation
rustc --version
cargo --version
```

## Development Commands

### Frontend Development
```bash
# Start Angular dev server (port 1420)
npm run start

# Build Angular app
npm run build

# Watch mode for development
npm run watch
```

### Tauri Development
```bash
# Run Tauri in development mode (starts both frontend and backend)
npm run tauri dev

# Build Tauri application
npm run tauri build
```

### Backend Development
```bash
# Navigate to Rust backend
cd src-tauri

# Build Rust code
cargo build

# Run tests
cargo test

# Check for errors
cargo check

# Format code
cargo fmt

# Run linter
cargo clippy
```

## Project Workflow

### Adding New Features

#### Frontend (Angular)
1. Create feature folder in appropriate location (`core/`, `shared/`, or feature-specific)
2. Generate component: `ng generate component path/to/component --standalone`
3. Implement component logic using signals for state
4. Add component to parent imports array
5. Style with TailwindCSS and PrimeNG components
6. Test component functionality

#### Backend (Rust)
1. Define DTO in `src-tauri/src/dtos/`
2. Create service trait and implementation in `src-tauri/src/services/`
3. Register service in Container module (`services/mod.rs`)
4. Create Tauri command in `src-tauri/src/commands/`
5. Register command in `lib.rs` invoke_handler
6. Test service logic

#### Integration
1. Import `invoke` from `@tauri-apps/api/core` in Angular component
2. Call Tauri command: `invoke<ReturnType>('command_name', { params })`
3. Handle response with `.then()` or async/await
4. Update component state with results

### Code Review Checklist
- [ ] Code follows project coding standards
- [ ] TypeScript strict mode passes
- [ ] Rust code compiles without warnings
- [ ] Tests are written and passing
- [ ] Documentation is updated
- [ ] No console errors or warnings
- [ ] Performance is acceptable
- [ ] Error handling is implemented

## Build and Deployment

### Development Build
```bash
# Build frontend
npm run build

# Build Tauri app (development)
npm run tauri build -- --debug
```

### Production Build
```bash
# Build optimized frontend
npm run build -- --configuration production

# Build Tauri app (production)
npm run tauri build
```

### Build Artifacts
- Frontend: `dist/montesimu/browser/`
- Tauri bundles: `src-tauri/target/release/bundle/`

## Debugging

### Frontend Debugging
- Use browser DevTools (F12)
- Angular DevTools extension
- Console logging with `console.log()`
- Network tab for Tauri command inspection

### Backend Debugging
- Use `println!()` or `dbg!()` macros
- Rust debugger (lldb/gdb)
- Tauri DevTools for IPC inspection
- Check Tauri console output

### Common Issues
- **Port 1420 already in use**: Kill existing process or change port in `angular.json`
- **Rust compilation errors**: Run `cargo clean` and rebuild
- **Tauri command not found**: Ensure command is registered in `lib.rs`
- **CORS issues**: Check Tauri security configuration

## Testing Strategy

### Unit Tests
- Test individual functions and methods
- Mock external dependencies
- Focus on business logic

### Integration Tests
- Test Tauri command integration
- Test Angular component integration
- Verify data flow between frontend and backend

### Manual Testing
- Test UI interactions
- Verify histogram generation with various datasets
- Check edge cases (empty data, single value, large datasets)
- Test on target platforms (Windows, macOS, Linux)

## Performance Optimization

### Frontend
- Use OnPush change detection strategy
- Lazy load routes and components
- Optimize bundle size
- Minimize re-renders

### Backend
- Profile Rust code with `cargo flamegraph`
- Optimize algorithms for large datasets
- Use efficient data structures
- Consider parallel processing for heavy computations

## Maintenance

### Dependency Updates
```bash
# Update npm packages
npm update

# Update Rust dependencies
cd src-tauri
cargo update
```

### Security Audits
```bash
# Check npm vulnerabilities
npm audit

# Check Rust vulnerabilities
cargo audit
```
