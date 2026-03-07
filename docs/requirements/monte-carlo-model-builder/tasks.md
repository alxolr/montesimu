# Implementation Plan: Monte Carlo Model Builder UI

## Overview

This implementation plan breaks down the Monte Carlo Model Builder UI feature into incremental coding tasks. The approach follows a bottom-up strategy: building core services and data models first, then implementing individual components, and finally integrating everything together. Each task builds on previous work to ensure continuous progress with testable functionality.

## Tasks

- [x] 1. Set up data models and type definitions
  - Create TypeScript interfaces for Variable, Constant, Distribution types
  - Define DistributionType union type ('Normal' | 'Lognormal' | 'Uniform')
  - Create ModelState interface
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
    - **Validates: Requirements 1.5, 2.2, 5.4**
  
  - [x] 2.3 Create ExpressionValidatorService
    - Implement `validate(expression: string, validIdentifiers: string[])` method
    - Implement syntax validation (balanced parentheses, operator placement)
    - Implement identifier extraction and validation
    - Return ValidationResult with isValid and errors array
    - _Requirements: 3.1, 3.2, 3.3, 3.4, 3.5_
  
  - [ ]* 2.4 Write property test for expression syntax validation
    - **Property 10: Expression Syntax Validation**
    - **Validates: Requirements 3.2, 3.5**
  
  - [ ]* 2.5 Write property test for undefined identifier detection
    - **Property 9: Undefined Identifier Detection**
    - **Validates: Requirements 3.1, 3.4**
  
  - [ ]* 2.6 Write property test for operator support
    - **Property 11: Operator Support**
    - **Validates: Requirements 3.3**

- [x] 3. Implement ModelService with Angular signals
  - [x] 3.1 Create ModelService with signal-based state
    - Create private writable signals for variables, constants, expression
    - Expose readonly signals for components
    - Create computed signal for allIdentifiers
    - _Requirements: 6.1, 6.2, 6.3, 6.4_
  
  - [x] 3.2 Implement variable CRUD operations
    - Implement addVariable, updateVariable, deleteVariable, getVariable methods
    - Update signals when operations occur
    - Validate uniqueness before adding/updating
    - _Requirements: 1.6, 1.7, 1.8_
  
  - [x] 3.3 Implement constant CRUD operations
    - Implement addConstant, updateConstant, deleteConstant, getConstant methods
    - Update signals when operations occur
    - Validate uniqueness before adding/updating
    - _Requirements: 2.3, 2.5, 2.6_
  
  - [x] 3.4 Implement expression management
    - Implement setExpression method
    - Integrate ExpressionValidatorService for validation
    - Create computed signal for validation results
    - _Requirements: 3.6, 3.7, 3.8_
  
  - [ ]* 3.5 Write property test for identifier uniqueness
    - **Property 4: Identifier Uniqueness**
    - **Validates: Requirements 1.6, 2.3**
  
  - [ ]* 3.6 Write property test for cascading validation on edit
    - **Property 5: Cascading Validation on Edit**
    - **Validates: Requirements 1.7, 2.5**
  
  - [ ]* 3.7 Write property test for cascading invalidation on deletion
    - **Property 6: Cascading Invalidation on Deletion**
    - **Validates: Requirements 1.8, 2.6, 3.8**

- [x] 4. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 5. Implement VariableFormComponent
  - [x] 5.1 Create component structure and template
    - Create standalone component with PrimeNG Dialog, InputText, Dropdown
    - Set up form with name, distribution type, and parameter fields
    - Implement conditional parameter fields based on distribution type
    - Add TailwindCSS styling for layout
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 4.7, 4.8_
  
  - [x] 5.2 Implement form validation logic
    - Inject IdentifierValidatorService and ModelService
    - Validate name format and uniqueness
    - Validate numeric parameters
    - Validate distribution-specific constraints (stdDev > 0, max > min)
    - Display inline error messages
    - _Requirements: 1.5, 1.6, 5.1, 5.2, 5.4, 5.5, 5.7_
  
  - [x] 5.3 Implement form submission
    - Prevent submission if form is invalid
    - Call ModelService to add or update variable
    - Close dialog on successful save
    - _Requirements: 5.6_
  
  - [ ]* 5.4 Write property test for required fields enforcement
    - **Property 1: Required Fields Enforcement**
    - **Validates: Requirements 1.1, 2.1**
  
  - [ ]* 5.5 Write property test for distribution-specific parameter validation
    - **Property 2: Distribution-Specific Parameter Validation**
    - **Validates: Requirements 1.2, 1.3, 1.4**
  
  - [ ]* 5.6 Write property test for numeric field validation
    - **Property 8: Numeric Field Validation**
    - **Validates: Requirements 2.4, 5.2, 5.3**
  
  - [ ]* 5.7 Write property test for distribution parameter constraints
    - **Property 15: Distribution Parameter Constraints**
    - **Validates: Requirements 5.7**
  
  - [ ]* 5.8 Write unit tests for form edge cases
    - Test empty form submission prevention
    - Test error message clearing on correction
    - Test distribution type change resetting parameters

- [ ] 6. Implement VariableListComponent
  - [ ] 6.1 Create component structure and template
    - Create standalone component with PrimeNG Card, Button
    - Display list of variables from ModelService signal
    - Add "Add Variable" button
    - Add edit/delete buttons for each variable
    - Show empty state when no variables exist
    - _Requirements: 1.9, 4.1, 4.4_
  
  - [ ] 6.2 Implement variable management actions
    - Open VariableFormComponent dialog for add/edit
    - Call ModelService.deleteVariable for delete action
    - Format distribution parameters for display
    - _Requirements: 1.7, 1.8_
  
  - [ ]* 6.3 Write property test for complete list display
    - **Property 7: Complete List Display**
    - **Validates: Requirements 1.9, 2.7**

- [ ] 7. Implement ConstantFormComponent
  - [ ] 7.1 Create component structure and template
    - Create standalone component with PrimeNG Dialog, InputText
    - Set up form with name and value fields
    - Add TailwindCSS styling for layout
    - _Requirements: 2.1, 4.7, 4.8_
  
  - [ ] 7.2 Implement form validation logic
    - Inject IdentifierValidatorService and ModelService
    - Validate name format and uniqueness
    - Validate numeric value
    - Display inline error messages
    - _Requirements: 2.2, 2.3, 2.4, 5.1, 5.3, 5.4, 5.5_
  
  - [ ] 7.3 Implement form submission
    - Prevent submission if form is invalid
    - Call ModelService to add or update constant
    - Close dialog on successful save
    - _Requirements: 5.6_
  
  - [ ]* 7.4 Write property test for form submission prevention
    - **Property 14: Form Submission Prevention**
    - **Validates: Requirements 5.6**
  
  - [ ]* 7.5 Write unit tests for constant form edge cases
    - Test negative, zero, and positive values
    - Test very large and very small numbers

- [ ] 8. Implement ConstantListComponent
  - [ ] 8.1 Create component structure and template
    - Create standalone component with PrimeNG Card, Button
    - Display list of constants from ModelService signal
    - Add "Add Constant" button
    - Add edit/delete buttons for each constant
    - Show empty state when no constants exist
    - _Requirements: 2.7, 4.1, 4.5_
  
  - [ ] 8.2 Implement constant management actions
    - Open ConstantFormComponent dialog for add/edit
    - Call ModelService.deleteConstant for delete action
    - _Requirements: 2.5, 2.6_

- [ ] 9. Implement ExpressionInputComponent
  - [ ] 9.1 Create component structure and template
    - Create standalone component with PrimeNG InputText
    - Add text input bound to ModelService expression signal
    - Display validation status (valid/invalid) with icons
    - Display error messages for invalid expressions
    - Display list of available identifiers
    - _Requirements: 3.6, 3.7, 4.1, 4.6_
  
  - [ ] 9.2 Implement real-time validation
    - Update expression in ModelService on input change
    - Use computed signal for validation results
    - Display validation feedback immediately
    - _Requirements: 3.1, 3.2, 3.4, 3.5, 3.8_
  
  - [ ]* 9.3 Write property test for expression validation feedback
    - **Property 12: Expression Validation Feedback**
    - **Validates: Requirements 3.6, 3.7, 4.6**
  
  - [ ]* 9.4 Write property test for inline error messages
    - **Property 13: Inline Error Messages**
    - **Validates: Requirements 5.1, 5.5**
  
  - [ ]* 9.5 Write unit tests for expression edge cases
    - Test empty expression
    - Test expression with all identifiers deleted
    - Test very long expressions
    - Test deeply nested parentheses

- [ ] 10. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 11. Implement ModelBuilderComponent (main container)
  - [ ] 11.1 Create component structure and template
    - Create standalone component with PrimeNG Card
    - Import and use VariableListComponent, ConstantListComponent, ExpressionInputComponent
    - Arrange sections in correct order (variables, constants, expression)
    - Apply TailwindCSS layout styling
    - _Requirements: 4.1, 4.2, 4.3, 4.7, 4.8, 7.1, 7.2_
  
  - [ ] 11.2 Provide ModelService to child components
    - Inject ModelService at component level
    - Ensure all child components can access the service
    - _Requirements: 6.5, 6.6_
  
  - [ ]* 11.3 Write unit tests for component integration
    - Test section ordering in DOM
    - Test that all three sections render
    - Test signal reactivity across components

- [ ] 12. Add routing and integrate into application
  - [ ] 12.1 Add route for ModelBuilderComponent
    - Update app.routes.ts to include model-builder route
    - Add navigation link in main app component
    - _Requirements: 7.1_
  
  - [ ] 12.2 Test end-to-end workflow
    - Manually test creating variables, constants, and expressions
    - Verify all validation works correctly
    - Verify UI updates reactively
    - Test edit and delete operations

- [ ] 13. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties using fast-check library
- Unit tests validate specific examples and edge cases
- All components use Angular 20 standalone architecture
- All UI components use PrimeNG and TailwindCSS per project guidelines
