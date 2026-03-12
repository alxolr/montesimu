# Implementation Plan: Monte Carlo Model Builder UI

## Overview

This implementation plan breaks down the Monte Carlo Model Builder UI feature into incremental coding tasks. The approach follows a bottom-up strategy: building core services and data models first, then implementing individual components, and finally integrating everything together. Each task builds on previous work to ensure continuous progress with testable functionality.

## Tasks

- [x] 1. Set up data models and type definitions
  - Create TypeScript interfaces for Variable, Distribution types
  - Define DistributionType union type ('Normal' | 'Lognormal' | 'Uniform')
  - Create IntermediateExpression interface with name and formula fields
  - Create ModelState interface with variables, intermediateExpressions, and targetExpression
  - Place in `src/app/model-builder/models/` directory
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1_

- [x] 2. Implement validation services
  - [x] 2.1 Create IdentifierValidatorService
    - Implement `validate(name: string)` method
    - Check for alphanumeric-only characters using regex
    - Return IdentifierValidationResult with isValid and error message
    - _Requirements: 1.5, 2.2_
  
  - [ ]* 2.2 Write property test for identifier validation
    - **Property 3: Identifier Format Validation**
    - **Validates: Requirements 1.5, 4.3**
  
  - [x] 2.3 Create ExpressionValidatorService
    - Implement `validate(expression: string, validIdentifiers: string[])` method
    - Implement syntax validation (balanced parentheses, operator placement)
    - Implement identifier extraction and validation
    - Support numeric literals in expressions
    - Return ValidationResult with isValid and errors array
    - _Requirements: 2.4, 2.5, 2.6, 2.7, 2.8, 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ]* 2.4 Write property test for expression syntax validation
    - **Property 10: Expression Syntax Validation**
    - **Validates: Requirements 2.5, 2.8, 3.2, 3.5**
  
  - [ ]* 2.5 Write property test for undefined identifier detection
    - **Property 9: Undefined Identifier Detection**
    - **Validates: Requirements 2.4, 2.7**
  
  - [ ]* 2.6 Write property test for operator support
    - **Property 11: Operator Support**
    - **Validates: Requirements 2.6, 3.3**

- [x] 2.7 Implement DependencyGraphService
  - [x] 2.7.1 Create DependencyGraphService with graph building
    - Implement `buildGraph()` method to construct dependency graph
    - Implement `extractIdentifiers()` method to parse formulas
    - Create DependencyNode interface
    - _Requirements: 5.1_
  
  - [x] 2.7.2 Implement circular reference detection
    - Implement `detectCircularReference()` using DFS with recursion stack
    - Implement cycle path reconstruction for error messages
    - Implement `wouldCreateCycle()` for pre-validation
    - Return CircularReferenceError with cycle path
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_
  
  - [x] 2.7.3 Implement topological sorting
    - Implement `getEvaluationOrder()` using Kahn's algorithm
    - Calculate in-degrees for all nodes
    - Process nodes in dependency order
    - Detect cycles during sort
    - _Requirements: 5.2, 5.3, 5.4, 5.5_
  
  - [ ]* 2.7.4 Write property test for direct circular reference detection
    - **Property 22: Direct Circular Reference Detection**
    - **Validates: Requirements 4.1, 4.3**
  
  - [ ]* 2.7.5 Write property test for indirect circular reference detection
    - **Property 23: Indirect Circular Reference Detection**
    - **Validates: Requirements 4.1, 4.4**
  
  - [ ]* 2.7.6 Write property test for evaluation order correctness
    - **Property 27: Evaluation Order Correctness**
    - **Validates: Requirements 5.2, 5.4, 5.5**
  
  - [ ]* 2.7.7 Write property test for topological sort cycle detection
    - **Property 28: Topological Sort Cycle Detection**
    - **Validates: Requirements 5.3**
  
  - [ ]* 2.7.8 Write unit tests for dependency graph edge cases
    - Test empty graph
    - Test single node with no dependencies
    - Test long dependency chains
    - Test multiple independent chains
    - Test complex dependency graphs

- [x] 3. Implement ModelService with Angular signals
  - [x] 3.1 Create ModelService with signal-based state
    - Create private writable signals for variables, intermediateExpressions, targetExpression
    - Expose readonly signals for components
    - Create computed signal for allIdentifiers (variables + intermediate expressions)
    - Create computed signal for evaluationOrder
    - _Requirements: 8.1, 8.2, 8.3, 8.4_
  
  - [x] 3.2 Implement variable CRUD operations
    - Implement addVariable, updateVariable, deleteVariable, getVariable methods
    - Update signals when operations occur
    - Validate uniqueness across variables and intermediate expressions
    - _Requirements: 1.6, 1.7, 1.8_
  
  - [ ] 3.3 Implement intermediate expression CRUD operations
    - Implement addIntermediateExpression, updateIntermediateExpression, deleteIntermediateExpression methods
    - Return Result type with success/error for operations that can fail
    - Check for circular references before adding/updating
    - Check for dependencies before deleting
    - Update signals when operations occur
    - Validate uniqueness across variables and intermediate expressions
    - _Requirements: 2.1, 2.2, 2.3, 2.11, 2.12_
  
  - [x] 3.4 Implement target expression management
    - Implement setTargetExpression method
    - Integrate ExpressionValidatorService for validation
    - Create computed signal for validation results
    - _Requirements: 3.6, 3.7, 3.8_
  
  - [ ] 3.5 Implement helper methods
    - Implement getDependencies(formula: string) to extract identifiers
    - Implement getExpressionsReferencingIdentifier(identifier: string)
    - Implement isIdentifierUnique with namespace checking
    - _Requirements: 2.3, 2.12, 6.6_
  
  - [ ]* 3.6 Write property test for identifier uniqueness across namespace
    - **Property 20: Intermediate Expression Identifier Uniqueness**
    - **Property 29: Identifier Namespace Uniqueness**
    - **Validates: Requirements 2.3**
  
  - [ ]* 3.7 Write property test for cascading validation on variable edit
    - **Property 5: Cascading Validation on Edit**
    - **Validates: Requirements 1.7**
  
  - [ ]* 3.8 Write property test for cascading validation on variable deletion
    - **Property 6: Cascading Invalidation on Deletion**
    - **Validates: Requirements 1.8, 3.8**
  
  - [ ]* 3.9 Write property test for cascading validation on intermediate expression edit
    - **Property 25: Cascading Validation on Intermediate Expression Edit**
    - **Validates: Requirements 2.11**
  
  - [ ]* 3.10 Write property test for dependency-based deletion prevention
    - **Property 24: Dependency-Based Deletion Prevention**
    - **Validates: Requirements 2.12**

- [x] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4.5 Implement IntermediateExpressionFormComponent
  - [ ] 4.5.1 Create component structure and template
    - Create standalone component with PrimeNG Dialog, InputText
    - Set up form with name and formula fields
    - Display available identifiers list
    - Add TailwindCSS styling for layout
    - _Requirements: 2.1, 2.2, 6.4, 6.7, 6.8_
  
  - [ ] 4.5.2 Implement form validation logic
    - Inject IdentifierValidatorService, ExpressionValidatorService, and ModelService
    - Validate name format and uniqueness across all identifiers
    - Validate formula syntax and identifier references
    - Check for circular references using ModelService
    - Display inline error messages
    - _Requirements: 2.2, 2.3, 2.4, 2.5, 2.7, 2.8, 4.1, 4.2, 4.5, 7.1, 7.3, 7.4, 7.7_
  
  - [ ] 4.5.3 Implement form submission
    - Prevent submission if form is invalid
    - Call ModelService to add or update intermediate expression
    - Handle Result type and display errors if operation fails
    - Close dialog on successful save
    - _Requirements: 2.1, 2.11, 4.5, 7.5_
  
  - [ ]* 4.5.4 Write property test for intermediate expression formula validation
    - **Property 21: Intermediate Expression Formula Validation**
    - **Validates: Requirements 2.4, 2.7**
  
  - [ ]* 4.5.5 Write unit tests for form edge cases
    - Test empty form submission prevention
    - Test error message clearing on correction
    - Test circular reference error display
    - Test duplicate name error display

- [ ] 4.6 Implement IntermediateExpressionListComponent
  - [ ] 4.6.1 Create component structure and template
    - Create standalone component with PrimeNG Card, Button
    - Display list of intermediate expressions from ModelService signal
    - Add "Add Expression" button
    - Add edit/delete buttons for each expression
    - Show empty state when no expressions exist
    - Display dependencies for each expression
    - _Requirements: 2.13, 6.1, 6.2, 6.4, 6.6_
  
  - [ ] 4.6.2 Implement expression management actions
    - Open IntermediateExpressionFormComponent dialog for add/edit
    - Call ModelService.deleteIntermediateExpression for delete action
    - Handle Result type and display error if deletion fails (dependencies exist)
    - Implement getDependencies method to show what each expression depends on
    - _Requirements: 2.11, 2.12_
  
  - [ ]* 4.6.3 Write unit tests for list component
    - Test empty state display
    - Test expression list rendering
    - Test dependencies display
    - Test delete prevention with error message

- [ ] 4.7 Implement TargetExpressionInputComponent
  - [ ] 4.7.1 Create component structure and template
    - Create standalone component with PrimeNG InputText
    - Add text input bound to ModelService targetExpression signal
    - Display validation status (valid/invalid) with icons
    - Display error messages for invalid expressions
    - Display list of available identifiers
    - Display dependencies
    - _Requirements: 3.6, 3.7, 6.1, 6.5, 6.6_
  
  - [ ] 4.7.2 Implement real-time validation
    - Update target expression in ModelService on input change
    - Use computed signal for validation results
    - Display validation feedback immediately
    - Implement dependencies() computed signal
    - _Requirements: 3.1, 3.2, 3.4, 3.5, 3.8_
  
  - [ ]* 4.7.3 Write property test for target expression validation
    - **Property 26: Target Expression Validation**
    - **Validates: Requirements 3.1, 3.4**
  
  - [ ]* 4.7.4 Write unit tests for target expression edge cases
    - Test empty expression
    - Test expression with all identifiers deleted
    - Test very long expressions
    - Test deeply nested parentheses

- [x] 5. Implement VariableFormComponent
  - [x] 5.1 Create component structure and template
    - Create standalone component with PrimeNG Dialog, InputText, Dropdown
    - Set up form with name, distribution type, and parameter fields
    - Implement conditional parameter fields based on distribution type
    - Add TailwindCSS styling for layout
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 3.5, 3.6_
  
  - [x] 5.2 Implement form validation logic
    - Inject IdentifierValidatorService and ModelService
    - Validate name format and uniqueness
    - Validate numeric parameters
    - Validate distribution-specific constraints (stdDev > 0, max > min)
    - Display inline error messages
    - _Requirements: 1.5, 1.6, 4.1, 4.2, 4.3, 4.4, 4.6_
  
  - [x] 5.3 Implement form submission
    - Prevent submission if form is invalid
    - Call ModelService to add or update variable
    - Close dialog on successful save
    - _Requirements: 4.5_
  
  - [ ]* 5.4 Write property test for required fields enforcement
    - **Property 1: Required Fields Enforcement**
    - **Validates: Requirements 1.1**
  
  - [ ]* 5.5 Write property test for distribution-specific parameter validation
    - **Property 2: Distribution-Specific Parameter Validation**
    - **Validates: Requirements 1.2, 1.3, 1.4**
  
  - [ ]* 5.6 Write property test for numeric field validation
    - **Property 8: Numeric Field Validation**
    - **Validates: Requirements 4.2**
  
  - [ ]* 5.7 Write property test for distribution parameter constraints
    - **Property 15: Distribution Parameter Constraints**
    - **Validates: Requirements 4.6**
  
  - [ ]* 5.8 Write unit tests for form edge cases
    - Test empty form submission prevention
    - Test error message clearing on correction
    - Test distribution type change resetting parameters

- [x] 6. Implement VariableListComponent
  - [x] 6.1 Create component structure and template
    - Create standalone component with PrimeNG Card, Button
    - Display list of variables from ModelService signal
    - Add "Add Variable" button
    - Add edit/delete buttons for each variable
    - Show empty state when no variables exist
    - _Requirements: 1.9, 3.1, 3.3_
  
  - [x] 6.2 Implement variable management actions
    - Open VariableFormComponent dialog for add/edit
    - Call ModelService.deleteVariable for delete action
    - Format distribution parameters for display
    - _Requirements: 1.7, 1.8_
  
  - [ ]* 6.3 Write property test for complete list display
    - **Property 7: Complete List Display**
    - **Validates: Requirements 1.9**

- [ ] 7. Implement ExpressionInputComponent (DEPRECATED - replaced by IntermediateExpressionListComponent and TargetExpressionInputComponent)
  - This component is no longer used in the nested expressions design
  - See tasks 4.6 and 4.7 for replacement components

- [ ] 8. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8.5 Implement PDF calculation service
  - [x] 8.5.1 Create PDFCalculatorService
    - Implement calculateNormalPDF method using the normal distribution formula
    - Implement calculateLognormalPDF method using the lognormal distribution formula
    - Implement calculateUniformPDF method using the uniform distribution formula
    - Implement generatePDFPoints method to generate array of {x, y} points
    - Implement x-range calculation for each distribution type
    - _Requirements: 7.2, 7.3, 7.4_
  
  - [ ]* 8.5.2 Write property test for Normal distribution bell curve shape
    - **Property 16: Normal Distribution Bell Curve Shape**
    - **Validates: Requirements 7.2**
  
  - [ ]* 8.5.3 Write property test for Lognormal distribution skewed shape
    - **Property 17: Lognormal Distribution Skewed Shape**
    - **Validates: Requirements 7.3**
  
  - [ ]* 8.5.4 Write property test for Uniform distribution flat shape
    - **Property 18: Uniform Distribution Flat Shape**
    - **Validates: Requirements 7.4**
  
  - [ ]* 8.5.5 Write unit tests for PDF calculations
    - Test edge cases: zero stdDev, negative values for lognormal
    - Test boundary conditions for uniform distribution
    - Test numerical accuracy of PDF formulas

- [x] 8.6 Implement DistributionPreviewComponent
  - [x] 8.6.1 Create component structure and template
    - Create standalone component with PrimeNG Chart
    - Set up chart configuration for line chart
    - Configure x and y axes with appropriate labels
    - Add TailwindCSS styling
    - _Requirements: 7.1, 7.6, 7.7, 7.8_
  
  - [x] 8.6.2 Implement reactive chart data generation
    - Create computed signal that generates chart data from input parameters
    - Inject PDFCalculatorService
    - Call generatePDFPoints based on distribution type and parameters
    - Format data for PrimeNG Chart component
    - _Requirements: 7.2, 7.3, 7.4, 7.5_
  
  - [ ]* 8.6.3 Write property test for chart updates on parameter change
    - **Property 19: Chart Updates on Parameter Change**
    - **Validates: Requirements 7.5**
  
  - [ ]* 8.6.4 Write unit tests for chart component
    - Test chart renders with valid parameters
    - Test chart handles missing/undefined parameters gracefully
    - Test chart configuration includes axis labels

- [x] 8.7 Update VariableFormComponent to include preview
  - [x] 8.7.1 Update component template
    - Increase dialog width to accommodate side-by-side layout
    - Add flex container with left (form) and right (preview) sections
    - Import and use DistributionPreviewComponent
    - Pass form parameters to preview component as inputs
    - _Requirements: 7.1, 7.8_
  
  - [x] 8.7.2 Update component styling
    - Add CSS for side-by-side layout
    - Ensure responsive behavior
    - Match UI/UX guidelines
    - _Requirements: 7.8_
  
  - [ ]* 8.7.3 Write unit tests for updated form layout
    - Test preview component is rendered when dialog is visible
    - Test preview component receives correct input parameters
    - Test layout structure (left/right arrangement)

- [ ] 9. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 10. Implement ModelBuilderComponent (main container)
  - [x] 10.1 Create component structure and template
    - Create standalone component with PrimeNG Card
    - Import and use VariableListComponent, IntermediateExpressionListComponent, TargetExpressionInputComponent
    - Arrange sections in correct order (variables, intermediate expressions, target expression)
    - Apply TailwindCSS layout styling
    - _Requirements: 6.1, 6.2, 6.7, 6.8_
  
  - [x] 10.2 Provide ModelService to child components
    - Inject ModelService at component level
    - Ensure all child components can access the service
    - _Requirements: 8.5_
  
  - [ ]* 10.3 Write unit tests for component integration
    - Test section ordering in DOM
    - Test that all three sections render
    - Test signal reactivity across components

- [x] 11. Add routing and integrate into application
  - [x] 11.1 Add route for ModelBuilderComponent
    - Update app.routes.ts to include model-builder route
    - Add navigation link in main app component
    - _Requirements: 9.1_
  
  - [x] 11.2 Test end-to-end workflow
    - Manually test creating variables, intermediate expressions, and target expression
    - Verify all validation works correctly
    - Verify circular reference detection works
    - Verify dependency checking works
    - Verify UI updates reactively
    - Test edit and delete operations
    - Test evaluation order computation

- [ ] 12. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties using fast-check library
- Unit tests validate specific examples and edge cases
- All components use Angular 20 standalone architecture
- All UI components use PrimeNG and TailwindCSS per project guidelines
- The nested expressions functionality adds significant complexity with circular reference detection and dependency graph management
- DependencyGraphService is a critical component that must be thoroughly tested
- Evaluation order computation ensures expressions are evaluated in the correct sequence during simulation
