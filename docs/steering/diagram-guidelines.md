---
inclusion: fileMatch
fileMatchPattern: '**/design.md,**/requirements.md,**/architecture/**'
---

# Diagram Guidelines

## Overview

All diagrams in documentation should use Mermaid syntax for consistency, version control friendliness, and easy rendering in Markdown viewers.

## Why Mermaid?

- **Version Control**: Text-based diagrams work well with Git
- **Consistency**: Standardized syntax across all documentation
- **Rendering**: Supported by GitHub, GitLab, VS Code, and many Markdown viewers
- **Maintainability**: Easy to update without specialized tools
- **Accessibility**: Can be read as text when rendering is unavailable

## Diagram Types

### Architecture Diagrams

Use flowcharts or block diagrams for system architecture:

```mermaid
graph TB
    Frontend[Angular Frontend]
    Backend[Rust Backend]
    DB[(Database)]
    
    Frontend -->|Tauri IPC| Backend
    Backend -->|Query| DB
```

### Component Structure

Use flowcharts for component hierarchies:

```mermaid
graph TD
    App[App Component]
    App --> ModelBuilder[Model Builder]
    App --> ModelRunner[Model Runner]
    
    ModelBuilder --> VariableList[Variable List]
    ModelBuilder --> ConstantList[Constant List]
    ModelBuilder --> ExpressionInput[Expression Input]
```

### Data Flow

Use sequence diagrams for data flow and interactions:

```mermaid
sequenceDiagram
    participant User
    participant Frontend
    participant Backend
    participant Engine
    
    User->>Frontend: Click "Run Simulation"
    Frontend->>Backend: Send Model Definition
    Backend->>Engine: Execute Simulation
    Engine-->>Backend: Return Results
    Backend-->>Frontend: Send Results
    Frontend-->>User: Display Histogram
```

### State Machines

Use state diagrams for component states:

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Configuring: Click Run
    Configuring --> Running: Start Simulation
    Configuring --> Idle: Cancel
    Running --> DisplayingResults: Complete
    Running --> Error: Failure
    DisplayingResults --> Idle: Close
    Error --> Idle: Dismiss
```

### Class Diagrams

Use class diagrams for data models and interfaces:

```mermaid
classDiagram
    class Variable {
        +string name
        +Distribution distribution
    }
    
    class Distribution {
        <<interface>>
        +string type
    }
    
    class NormalDistribution {
        +number mean
        +number stdDev
    }
    
    Distribution <|-- NormalDistribution
    Variable --> Distribution
```

### Entity Relationships

Use ER diagrams for data relationships:

```mermaid
erDiagram
    MODEL ||--o{ VARIABLE : contains
    MODEL ||--o{ CONSTANT : contains
    MODEL ||--|| EXPRESSION : has
    
    VARIABLE {
        string name
        string distributionType
    }
    
    CONSTANT {
        string name
        number value
    }
```

## Best Practices

### 1. Keep Diagrams Simple

- Focus on one concept per diagram
- Avoid overcrowding with too many elements
- Use multiple diagrams if needed

### 2. Use Consistent Naming

- Match names in diagrams to code/documentation
- Use PascalCase for components/classes
- Use camelCase for methods/properties

### 3. Add Descriptive Labels

- Label all arrows and connections
- Use clear, concise descriptions
- Avoid abbreviations unless well-known

### 4. Use Colors Sparingly

Mermaid supports styling, but use it judiciously:

```mermaid
graph LR
    A[Input] -->|Valid| B[Process]
    A -->|Invalid| C[Error]
    
    style B fill:#90EE90
    style C fill:#FFB6C1
```

### 5. Include Diagram Titles

Always add a title or caption above the diagram:

**System Architecture Overview**
```mermaid
graph TB
    ...
```

### 6. Test Rendering

- Preview diagrams in VS Code or GitHub
- Ensure syntax is correct
- Verify all elements are visible

## Common Patterns

### Frontend-Backend Communication

```mermaid
sequenceDiagram
    participant F as Frontend
    participant T as Tauri IPC
    participant B as Backend
    
    F->>T: invoke("command", data)
    T->>B: Execute Command
    B-->>T: Return Result
    T-->>F: Promise Resolves
```

### Component Hierarchy

```mermaid
graph TD
    Parent[Parent Component]
    Parent --> Child1[Child Component 1]
    Parent --> Child2[Child Component 2]
    
    Child1 --> Service1[Service A]
    Child2 --> Service1
    Child2 --> Service2[Service B]
```

### State Management Flow

```mermaid
graph LR
    Action[User Action] --> Component
    Component --> Service
    Service --> Signal[Update Signal]
    Signal --> UI[UI Updates]
```

## Mermaid Syntax Reference

### Graph Directions

- `TB` or `TD`: Top to Bottom
- `BT`: Bottom to Top
- `LR`: Left to Right
- `RL`: Right to Left

### Node Shapes

- `[Text]`: Rectangle
- `(Text)`: Rounded rectangle
- `([Text])`: Stadium shape
- `[[Text]]`: Subroutine
- `[(Text)]`: Cylindrical (database)
- `((Text))`: Circle
- `{Text}`: Rhombus (decision)

### Arrow Types

- `-->`: Solid arrow
- `-.->`: Dotted arrow
- `==>`: Thick arrow
- `--text-->`: Arrow with label

### Sequence Diagram Arrows

- `->`: Solid line
- `-->`: Dotted line
- `->>`: Solid arrow
- `-->>`: Dotted arrow

## Tools and Resources

### VS Code Extensions

- **Markdown Preview Mermaid Support**: Renders Mermaid in preview
- **Mermaid Editor**: Dedicated Mermaid editor

### Online Editors

- [Mermaid Live Editor](https://mermaid.live/): Test and export diagrams
- [Mermaid Documentation](https://mermaid.js.org/): Official docs

### Rendering

- GitHub and GitLab render Mermaid automatically
- Most modern Markdown viewers support Mermaid
- Can export to PNG/SVG if needed

## Migration from ASCII/Text Diagrams

When updating existing documentation:

1. Identify all ASCII art or text-based diagrams
2. Determine appropriate Mermaid diagram type
3. Convert to Mermaid syntax
4. Test rendering
5. Remove old ASCII art

## Examples in This Project

See these files for Mermaid diagram examples:
- `docs/requirements/monte-carlo-model-runner/design.md` - Architecture diagrams
- `docs/requirements/monte-carlo-model-builder/design.md` - Component structure

## Checklist for New Diagrams

- [ ] Diagram uses Mermaid syntax
- [ ] Diagram has a descriptive title/caption
- [ ] All elements are clearly labeled
- [ ] Diagram renders correctly in preview
- [ ] Diagram follows project naming conventions
- [ ] Diagram is focused on a single concept
- [ ] Arrows and connections are labeled
- [ ] Styling is minimal and purposeful
