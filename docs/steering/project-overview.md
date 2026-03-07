---
inclusion: auto
---

# Montesimu Project Overview

## Project Description
Montesimu is a desktop application built with Tauri 2, Angular 20, and Rust. It provides statistical visualization capabilities, specifically histogram generation from datasets.

## Tech Stack

### Frontend
- **Framework**: Angular 20.1.4
- **UI Library**: PrimeNG 20.3.0 with Lara theme
- **Styling**: TailwindCSS 4.1.17 with PrimeUI plugin
- **Charts**: Chart.js 4.5.1 via PrimeNG ChartModule
- **Language**: TypeScript 5.8.3

### Backend
- **Runtime**: Tauri 2
- **Language**: Rust (Edition 2021)
- **Dependency Injection**: Shaku 0.6.2
- **Serialization**: Serde 1.x

### Build Tools
- **Angular CLI**: 20.1.4
- **Tauri CLI**: 2.x
- **Dev Server Port**: 1420

## Project Structure

```
montesimu/
├── src/                          # Angular frontend
│   ├── app/
│   │   ├── core/                 # Core services and utilities
│   │   ├── shared/               # Shared components
│   │   │   └── components/
│   │   │       └── histogram/    # Histogram visualization component
│   │   ├── app.component.*       # Root component
│   │   ├── app.config.ts         # Application configuration
│   │   └── app.routes.ts         # Routing configuration
│   ├── assets/                   # Static assets
│   └── styles.css                # Global styles
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── commands/             # Tauri command handlers
│   │   ├── services/             # Business logic services
│   │   ├── dtos/                 # Data transfer objects
│   │   ├── lib.rs                # Library entry point
│   │   └── main.rs               # Application entry point
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
└── docs/                         # Documentation
    ├── requirements/
    ├── steering/
    ├── specs/
    ├── architecture/
    └── guides/
```

## Architecture Patterns

### Backend (Rust)
- **Dependency Injection**: Uses Shaku for IoC container
- **Service Layer**: Business logic separated into services
- **Command Pattern**: Tauri commands delegate to services
- **DTOs**: Explicit data transfer objects for API boundaries

### Frontend (Angular)
- **Standalone Components**: Modern Angular standalone architecture
- **Signals**: Reactive state management with Angular signals
- **Component-based**: Modular, reusable components
- **Dependency Injection**: Angular's built-in DI system

## Key Features
- Histogram generation from numerical datasets
- Normal distribution visualization
- Statistical data processing in Rust backend
- Interactive chart rendering with Chart.js
