# Implementation Plan: Monte Carlo Model Runner

## Overview

This implementation plan breaks down the Monte Carlo Model Runner feature into incremental coding tasks. The approach follows a layered strategy: building backend simulation engine first, then frontend services, then UI components, and finally integrating everything together. Each task builds on previous work to ensure continuous progress with testable functionality.

## Tasks

- [x] 1. Set up Rust backend structure and dependencies
  - Add required dependencies to Cargo.toml (rand, rand_distr, serde, serde_json)
  - Create module structure for simulation engine
  - Set up Tauri command infrastructure
  - _Requirements: 2.6, 3.1_

- [ ] 2. Implement expression parser in Rust
  - [ ] 2.1 Create AST data structures (Expr, Operator enums)
    - Define expression tree node types
    - Implement Display trait for debugging
    - _Requirements: 3.1_
  
  - [ ] 2.2 Implement tokenizer
    - Parse expression string into tokens
    - Handle numbers, identifiers, operators, parentheses
    - _Requirements: 3.2_
  
  - [ ] 2.3 Implement recursive descent parser
    - Parse tokens into AST
    - Handle operator precedence (*, / before +, -)
    - Handle parentheses for grouping
    - _Requirements: 3.2_
  
  - [ ] 2.4 Implement syntax error detection
    - Detect unbalanced parentheses
    - Detect invalid operator placement
    - Return descriptive error messages
    - _Requirements: 3.4_
  
  - [ ] 2.5 Implement identifier extraction
    - Extract all unique identifiers from AST
    - Return list of variable/constant references
    - _Requirements: 3.3_
  
  - [ ]* 2.6 Write property test for expression parsing
    - **Property 3: Expression Parsing Correctness**
    - **Validates: Requirements 3.1, 3.2**
  
  - [ ]* 2.7 Write property test for syntax error detection
    - **Property 5: Syntax Error Detection**
    - **Validates: Requirements 3.4**
  
  - [ ]* 2.8 Write property test for identifier extraction
    - **Property 4: Identifier Extraction**
    - **Validates: Requirements 3.3**
  
  - [ ]* 2.9 Write unit tests for parser edge cases
    - Test deeply nested parentheses
    - Test expressions with only numbers
    - Test expressions with only identifiers

- [ ] 3. Implement expression evaluator in Rust
  - [ ] 3.1 Create evaluator with value context
    - Store variable and constant values in HashMap
    - Implement evaluation method
    - _Requirements: 5.1, 5.2, 5.3_
  
  - [ ] 3.2 Implement AST traversal and evaluation
    - Evaluate number nodes
    - Substitute identifier nodes with values
    - Evaluate binary operations with correct precedence
    - _Requirements: 5.4_
  
  - [ ] 3.3 Implement error handling
    - Handle division by zero
    - Handle undefined identifiers
    - Return descriptive error messages
    - _Requirements: 5.6_
  
  - [ ]* 3.4 Write property test for evaluation correctness
    - **Property 9: Expression Evaluation Correctness**
    - **Validates: Requirements 5.2, 5.3, 5.4**
  
  - [ ]* 3.5 Write unit tests for evaluation edge cases
    - Test division by zero
    - Test overflow/underflow
    - Test very large expressions

- [ ] 4. Implement distribution sampler in Rust
  - [ ] 4.1 Create sampler with RNG
    - Initialize ThreadRng
    - Create sampling methods for each distribution type
    - _Requirements: 4.1, 4.5_
  
  - [ ] 4.2 Implement Normal distribution sampling
    - Use rand_distr::Normal
    - Sample with specified mean and std_dev
    - _Requirements: 4.2_
  
  - [ ] 4.3 Implement Lognormal distribution sampling
    - Use rand_distr::LogNormal
    - Sample with specified mean and std_dev
    - _Requirements: 4.3_
  
  - [ ] 4.4 Implement Uniform distribution sampling
    - Use rand_distr::Uniform
    - Sample with specified min and max
    - _Requirements: 4.4_
  
  - [ ]* 4.5 Write property test for uniform sampling bounds
    - **Property 7: Distribution Sampling Bounds**
    - **Validates: Requirements 4.4**
  
  - [ ]* 4.6 Write property test for complete variable sampling
    - **Property 8: Complete Variable Sampling**
    - **Validates: Requirements 4.1, 4.6**
  
  - [ ]* 4.7 Write unit tests for sampler
    - Test each distribution type
    - Verify sampled values are reasonable

- [ ] 5. Checkpoint - Ensure all backend tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Implement simulation engine in Rust
  - [ ] 6.1 Create SimulationEngine struct
    - Store model definition (variables, constants, expression)
    - Store iteration count
    - _Requirements: 6.1_
  
  - [ ] 6.2 Implement single iteration execution
    - Sample all variables
    - Create value context with variables
    - Evaluate expression
    - Store result
    - Handle errors gracefully
    - _Requirements: 4.6, 5.1, 5.5, 5.6_
  
  - [ ] 6.3 Implement full simulation execution
    - Execute specified number of iterations
    - Collect all results
    - Collect error information
    - Return SimulationResults
    - _Requirements: 6.1, 6.3, 6.4, 6.5_
  
  - [ ]* 6.4 Write property test for iteration count accuracy
    - **Property 10: Iteration Count Accuracy**
    - **Validates: Requirements 6.1, 6.3**
  
  - [ ]* 6.5 Write property test for results array completeness
    - **Property 11: Results Array Completeness**
    - **Validates: Requirements 6.4**
  
  - [ ]* 6.6 Write property test for error information inclusion
    - **Property 12: Error Information Inclusion**
    - **Validates: Requirements 6.5**
  
  - [ ]* 6.7 Write unit tests for simulation engine
    - Test simulation with known model
    - Test simulation with errors
    - Test empty model handling

- [ ] 7. Implement Tauri command handler
  - [ ] 7.1 Create run_simulation Tauri command
    - Accept ModelDefinition parameter
    - Parse expression
    - Create and run simulation engine
    - Return SimulationResults
    - Handle all errors and return error messages
    - _Requirements: 2.6, 3.1, 6.6_
  
  - [ ] 7.2 Register command in Tauri app
    - Add command to Tauri builder
    - Test command invocation from frontend
    - _Requirements: 2.6_
  
  - [ ]* 7.3 Write integration tests for Tauri command
    - Test successful simulation
    - Test error cases
    - Test serialization/deserialization

- [ ] 8. Checkpoint - Ensure backend is fully functional
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 9. Implement frontend StatisticsService
  - [ ] 9.1 Create StatisticsService with calculation methods
    - Implement calculateStatistics method
    - Implement mean calculation
    - Implement median calculation
    - Implement standard deviation calculation
    - Implement min/max identification
    - Implement percentile calculations (25th, 75th, 95th)
    - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5, 9.6, 9.7, 9.8_
  
  - [ ]* 9.2 Write property test for statistical calculations
    - **Property 13: Statistical Calculation Correctness**
    - **Validates: Requirements 9.1, 9.2, 9.3, 9.4, 9.5, 9.6, 9.7, 9.8**
  
  - [ ]* 9.3 Write unit tests for statistics edge cases
    - Test with single value
    - Test with two values
    - Test with identical values
    - Test with very large/small numbers

- [ ] 10. Implement frontend CSVExportService
  - [ ] 10.1 Create CSVExportService
    - Implement exportResults method
    - Implement CSV content generation
    - Implement filename generation with timestamp
    - Implement browser download trigger
    - _Requirements: 11.2, 11.3, 11.4, 11.5, 11.6, 11.7_
  
  - [ ]* 10.2 Write property test for CSV structure
    - **Property 15: CSV Structure Completeness**
    - **Validates: Requirements 11.2, 11.3, 11.4, 11.5**
  
  - [ ]* 10.3 Write property test for CSV filename format
    - **Property 16: CSV Filename Format**
    - **Validates: Requirements 11.7**
  
  - [ ]* 10.4 Write unit tests for CSV export
    - Test CSV content format
    - Test filename generation
    - Test download trigger

- [ ] 11. Implement frontend SimulationService
  - [ ] 11.1 Create SimulationService with signals
    - Create signals for isRunning, results, error
    - Expose readonly signals
    - _Requirements: 7.1, 7.4, 7.5, 7.6_
  
  - [ ] 11.2 Implement model serialization
    - Convert ModelState to ModelDefinition
    - Include all variables with distributions
    - Include expression text
    - Include iteration count
    - _Requirements: 2.1, 2.2, 2.3, 2.4_
  
  - [ ] 11.3 Implement Tauri command invocation
    - Use Tauri invoke API
    - Handle async execution
    - Update signals during execution
    - Handle errors from backend
    - _Requirements: 2.6, 2.7_
  
  - [ ] 11.4 Implement runSimulation method
    - Validate model before running
    - Serialize model
    - Invoke Tauri command
    - Update signals with results or errors
    - _Requirements: 6.1, 10.1, 10.2, 10.3_
  
  - [ ]* 11.5 Write property test for complete model serialization
    - **Property 2: Complete Model Serialization**
    - **Validates: Requirements 2.1, 2.2, 2.3, 2.4**
  
  - [ ]* 11.6 Write property test for pre-simulation validation
    - **Property 17: Pre-Simulation Validation**
    - **Validates: Requirements 10.1, 10.2, 10.3**
  
  - [ ]* 11.7 Write unit tests for simulation service
    - Test successful simulation flow
    - Test error handling
    - Test signal updates
    - Mock Tauri commands

- [ ] 12. Checkpoint - Ensure all frontend services work
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 13. Implement SimulationConfigDialogComponent
  - [ ] 13.1 Create component structure and template
    - Create standalone component with PrimeNG Dialog
    - Add preset iteration buttons (1000, 10000, 100000)
    - Add custom iteration input field
    - Add Start and Cancel buttons
    - _Requirements: 1.2, 1.3, 1.4, 1.7, 1.8_
  
  - [ ] 13.2 Implement iteration count validation
    - Validate positive integer
    - Display error message for invalid input
    - Disable Start button when invalid
    - _Requirements: 1.5, 1.6_
  
  - [ ] 13.3 Implement dialog actions
    - Handle preset selection
    - Handle custom input
    - Emit start event with configuration
    - Close dialog on cancel
    - _Requirements: 1.7, 1.8_
  
  - [ ]* 13.4 Write property test for iteration count validation
    - **Property 1: Iteration Count Validation**
    - **Validates: Requirements 1.5, 1.6**
  
  - [ ]* 13.5 Write unit tests for config dialog
    - Test preset selection
    - Test custom input validation
    - Test dialog open/close
    - Test event emission

- [ ] 14. Implement ResultsDisplayComponent
  - [ ] 14.1 Create component structure and template
    - Create standalone component with PrimeNG Card and Chart
    - Add histogram section
    - Add summary statistics section
    - Add export button
    - _Requirements: 8.1, 8.3, 8.4, 8.6, 9.9, 11.1_
  
  - [ ] 14.2 Implement histogram generation
    - Calculate appropriate bin sizes based on data range
    - Generate histogram data for PrimeNG Chart
    - Configure chart with proper labels and styling
    - _Requirements: 8.2, 8.3, 8.4_
  
  - [ ] 14.3 Implement statistics display
    - Inject StatisticsService
    - Calculate statistics from results
    - Format numbers to appropriate decimal places
    - Display all statistics in organized grid
    - _Requirements: 9.9, 9.10_
  
  - [ ] 14.4 Implement CSV export
    - Inject CSVExportService
    - Call export on button click
    - _Requirements: 11.1, 11.2_
  
  - [ ]* 14.5 Write property test for histogram bin coverage
    - **Property 14: Histogram Bin Coverage**
    - **Validates: Requirements 8.2**
  
  - [ ]* 14.6 Write unit tests for results display
    - Test histogram generation
    - Test statistics display
    - Test CSV export trigger
    - Test number formatting

- [ ] 15. Implement ModelRunnerComponent
  - [ ] 15.1 Create component structure and template
    - Create standalone component
    - Add "Run Simulation" button with tooltip
    - Add SimulationConfigDialogComponent
    - Add loading indicator
    - Add ResultsDisplayComponent
    - _Requirements: 1.1, 7.1, 7.2, 7.3, 8.1_
  
  - [ ] 15.2 Implement simulation readiness check
    - Inject ModelService
    - Check for at least one variable
    - Check for valid expression
    - Enable/disable button accordingly
    - Provide tooltip explaining disabled state
    - _Requirements: 1.1, 12.4_
  
  - [ ] 15.3 Implement simulation execution flow
    - Inject SimulationService
    - Open config dialog on button click
    - Start simulation with selected configuration
    - Display loading indicator during execution
    - Display results when complete
    - Handle errors
    - _Requirements: 1.2, 6.1, 7.1, 7.4, 7.5, 7.6, 10.4, 10.5, 10.7_
  
  - [ ]* 15.4 Write property test for model state reactivity
    - **Property 18: Model State Reactivity**
    - **Validates: Requirements 12.2, 12.3**
  
  - [ ]* 15.5 Write unit tests for model runner
    - Test button enable/disable logic
    - Test simulation flow
    - Test error handling
    - Test loading states

- [ ] 16. Integrate ModelRunnerComponent into ModelBuilderComponent
  - [ ] 16.1 Add ModelRunnerComponent to ModelBuilder template
    - Import ModelRunnerComponent
    - Add component below expression section
    - Apply consistent styling
    - _Requirements: 12.1, 12.5_
  
  - [ ] 16.2 Test integration
    - Verify ModelRunner reads from ModelService
    - Verify reactive updates work
    - Verify UI consistency
    - _Requirements: 12.2, 12.3_
  
  - [ ]* 16.3 Write integration tests
    - Test end-to-end simulation flow
    - Test model changes reflected in runner
    - Test error scenarios

- [ ] 17. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 18. End-to-end testing and polish
  - [ ] 18.1 Manual testing of complete workflow
    - Test creating model and running simulation
    - Test various iteration counts
    - Test error scenarios
    - Test CSV export
    - Verify histogram and statistics accuracy
  
  - [ ] 18.2 UI/UX polish
    - Ensure consistent styling with Model Builder
    - Verify responsive design
    - Check loading indicators and transitions
    - Verify error messages are clear
  
  - [ ] 18.3 Performance testing
    - Test with large iteration counts (100,000+)
    - Verify UI remains responsive
    - Check memory usage

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate universal correctness properties using fast-check (frontend) and proptest/quickcheck (backend)
- Unit tests validate specific examples and edge cases
- Backend uses Rust with Tauri for high-performance simulation
- Frontend uses Angular 20 standalone components with PrimeNG and TailwindCSS
- The implementation follows a bottom-up approach: backend first, then services, then UI components
