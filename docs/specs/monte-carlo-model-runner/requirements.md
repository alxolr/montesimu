# Requirements Document

## Introduction

The Monte Carlo Model Runner is a feature that enables users to execute Monte Carlo simulations on models defined in the Model Builder. The feature provides a user interface for configuring and running simulations, integrates with a Rust backend via Tauri for high-performance execution, and displays comprehensive results including histograms and summary statistics. This feature builds upon the Model Builder by adding simulation execution capabilities.

## Glossary

- **Model_Runner**: The UI component that allows users to execute Monte Carlo simulations
- **Model_Builder**: The existing UI component for defining simulation models (variables and expressions)
- **Simulation_Engine**: The Rust backend component that executes Monte Carlo simulations via Tauri
- **Iteration**: A single execution of the model where each variable is sampled from its distribution and the expression is evaluated
- **Histogram**: A visual representation showing the frequency distribution of simulation output values
- **Summary_Statistics**: Calculated metrics including mean, median, standard deviation, minimum, maximum, and percentiles
- **Model_Definition**: The complete specification of a model including variables with distributions and the expression
- **Tauri_Command**: A Rust function exposed to the frontend via Tauri's IPC mechanism

## Requirements

### Requirement 1: Simulation Configuration

**User Story:** As a user, I want to configure simulation parameters before running, so that I can control the precision and duration of the simulation.

#### Acceptance Criteria

1. WHEN a user has defined at least one variable and a valid expression in the Model_Builder, THE Model_Runner SHALL display a "Run Simulation" button
2. WHEN a user clicks the "Run Simulation" button, THE Model_Runner SHALL display a configuration dialog
3. WHEN the configuration dialog is displayed, THE Model_Runner SHALL provide preset iteration options (1000, 10000, 100000)
4. WHEN the configuration dialog is displayed, THE Model_Runner SHALL allow users to enter a custom iteration count
5. WHEN a user enters a custom iteration count, THE Model_Runner SHALL validate that it is a positive integer
6. WHEN a user enters an invalid iteration count, THE Model_Runner SHALL display an error message and prevent simulation start
7. WHEN the configuration dialog is displayed, THE Model_Runner SHALL show a "Start" button to begin the simulation
8. WHEN the configuration dialog is displayed, THE Model_Runner SHALL show a "Cancel" button to close the dialog without running

### Requirement 2: Model Transmission to Backend

**User Story:** As a developer, I want the frontend to send the complete model definition to the Rust backend, so that the simulation can be executed with all necessary information.

#### Acceptance Criteria

1. WHEN a user starts a simulation, THE Model_Runner SHALL serialize the Model_Definition into a JSON format
2. WHEN serializing the Model_Definition, THE Model_Runner SHALL include all variables with their names, distribution types, and parameters
3. WHEN serializing the Model_Definition, THE Model_Runner SHALL include an array of intermediate expressions with their names and expression text
4. WHEN serializing the Model_Definition, THE Model_Runner SHALL include the target expression text
5. WHEN serializing the Model_Definition, THE Model_Runner SHALL include the iteration count
6. WHEN the Model_Definition is serialized, THE Model_Runner SHALL invoke a Tauri_Command to send the data to the Simulation_Engine
7. IF the Tauri_Command invocation fails, THE Model_Runner SHALL display an error message to the user

### Requirement 3: Backend Expression Parsing

**User Story:** As a developer, I want the Rust backend to parse mathematical expressions, so that they can be evaluated efficiently during simulation.

#### Acceptance Criteria

1. WHEN the Simulation_Engine receives a Model_Definition, THE Simulation_Engine SHALL parse all intermediate expressions and the target expression into abstract syntax trees
2. WHEN parsing expressions, THE Simulation_Engine SHALL support addition (+), subtraction (-), multiplication (*), division (/), and parentheses operators
3. WHEN parsing expressions, THE Simulation_Engine SHALL identify all variable and intermediate expression references
4. WHEN parsing expressions, THE Simulation_Engine SHALL build a dependency graph showing which expressions reference which variables and other expressions
5. IF any expression contains syntax errors, THE Simulation_Engine SHALL return an error message describing the syntax error
6. IF any expression contains undefined identifiers, THE Simulation_Engine SHALL return an error message listing the undefined identifiers
7. WHEN all expressions are successfully parsed, THE Simulation_Engine SHALL prepare them for evaluation in dependency order

### Requirement 4: Distribution Sampling

**User Story:** As a developer, I want the Rust backend to sample values from probability distributions, so that each simulation iteration uses random inputs.

#### Acceptance Criteria

1. WHEN executing a simulation iteration, THE Simulation_Engine SHALL sample a value from each variable's distribution
2. WHEN sampling from a Normal distribution, THE Simulation_Engine SHALL use the specified mean and standard deviation parameters
3. WHEN sampling from a Lognormal distribution, THE Simulation_Engine SHALL use the specified mean and standard deviation parameters
4. WHEN sampling from a Uniform distribution, THE Simulation_Engine SHALL use the specified minimum and maximum parameters
5. WHEN sampling from any distribution, THE Simulation_Engine SHALL use a cryptographically secure random number generator
6. WHEN all variables are sampled, THE Simulation_Engine SHALL have a complete set of input values for the iteration

### Requirement 5: Expression Evaluation

**User Story:** As a developer, I want the Rust backend to evaluate expressions with sampled values, so that each iteration produces an output result.

#### Acceptance Criteria

1. WHEN a simulation iteration has sampled all variable values, THE Simulation_Engine SHALL evaluate expressions in dependency order (topological sort)
2. WHEN evaluating intermediate expressions, THE Simulation_Engine SHALL substitute variable names and previously evaluated expression names with their values
3. WHEN evaluating the target expression, THE Simulation_Engine SHALL substitute variable names and intermediate expression names with their values
4. WHEN evaluating expressions, THE Simulation_Engine SHALL compute results following standard mathematical operator precedence
5. WHEN the target expression evaluation completes, THE Simulation_Engine SHALL store only the target expression result value
6. IF any expression evaluation encounters an error (e.g., division by zero), THE Simulation_Engine SHALL record the error and continue with the next iteration

### Requirement 6: Simulation Execution

**User Story:** As a user, I want the simulation to execute all iterations efficiently, so that I can get results in a reasonable time.

#### Acceptance Criteria

1. WHEN a simulation starts, THE Simulation_Engine SHALL execute the specified number of iterations
2. WHEN executing iterations, THE Simulation_Engine SHALL perform sampling and evaluation for each iteration
3. WHEN all iterations complete successfully, THE Simulation_Engine SHALL return all result values to the Model_Runner
4. WHEN returning results, THE Simulation_Engine SHALL include the array of all output values
5. WHEN returning results, THE Simulation_Engine SHALL include any error information from failed iterations
6. IF the simulation is interrupted or fails completely, THE Simulation_Engine SHALL return an error message

### Requirement 7: Progress Indication

**User Story:** As a user, I want to see progress while the simulation runs, so that I know the system is working and how long to wait.

#### Acceptance Criteria

1. WHEN a simulation starts, THE Model_Runner SHALL display a loading indicator
2. WHEN the loading indicator is displayed, THE Model_Runner SHALL show a message indicating the simulation is running
3. WHEN the loading indicator is displayed, THE Model_Runner SHALL show the total number of iterations being executed
4. WHEN a simulation is running, THE Model_Runner SHALL disable the "Run Simulation" button to prevent concurrent executions
5. WHEN the simulation completes or fails, THE Model_Runner SHALL hide the loading indicator
6. WHEN the simulation completes or fails, THE Model_Runner SHALL re-enable the "Run Simulation" button

### Requirement 8: Results Visualization

**User Story:** As a user, I want to see a histogram of simulation results, so that I can understand the distribution of possible outcomes.

#### Acceptance Criteria

1. WHEN simulation results are received, THE Model_Runner SHALL display a histogram showing the distribution of output values
2. WHEN displaying the histogram, THE Model_Runner SHALL automatically determine appropriate bin sizes based on the data range
3. WHEN displaying the histogram, THE Model_Runner SHALL use a bar chart with bins on the x-axis and frequency on the y-axis
4. WHEN displaying the histogram, THE Model_Runner SHALL label the x-axis with "Output Value" and the y-axis with "Frequency"
5. WHEN displaying the histogram, THE Model_Runner SHALL use a visually distinct color for the bars
6. THE Model_Runner SHALL display the histogram using PrimeNG Chart component

### Requirement 9: Summary Statistics

**User Story:** As a user, I want to see summary statistics of simulation results, so that I can quickly understand key characteristics of the output distribution.

#### Acceptance Criteria

1. WHEN simulation results are received, THE Model_Runner SHALL calculate the mean of all output values
2. WHEN simulation results are received, THE Model_Runner SHALL calculate the median of all output values
3. WHEN simulation results are received, THE Model_Runner SHALL calculate the standard deviation of all output values
4. WHEN simulation results are received, THE Model_Runner SHALL identify the minimum output value
5. WHEN simulation results are received, THE Model_Runner SHALL identify the maximum output value
6. WHEN simulation results are received, THE Model_Runner SHALL calculate the 25th percentile of output values
7. WHEN simulation results are received, THE Model_Runner SHALL calculate the 75th percentile of output values
8. WHEN simulation results are received, THE Model_Runner SHALL calculate the 95th percentile of output values
9. WHEN displaying summary statistics, THE Model_Runner SHALL show all calculated statistics in a clear, organized format
10. WHEN displaying summary statistics, THE Model_Runner SHALL format numeric values to an appropriate number of decimal places

### Requirement 10: Error Handling

**User Story:** As a user, I want clear error messages when simulations fail, so that I can understand and fix problems.

#### Acceptance Criteria

1. IF any expression contains syntax errors, THE Model_Runner SHALL prevent simulation execution and display an error message
2. IF the model has no variables defined, THE Model_Runner SHALL prevent simulation execution and display an error message
3. IF the target expression is empty or invalid, THE Model_Runner SHALL prevent simulation execution and display an error message
4. IF the expressions contain circular references, THE Simulation_Engine SHALL return an error message describing the circular dependency
5. IF the Simulation_Engine returns an error, THE Model_Runner SHALL display a user-friendly error message describing the failure
6. IF the Tauri communication fails, THE Model_Runner SHALL display an error message indicating a backend communication error
7. WHEN displaying error messages, THE Model_Runner SHALL use a visually distinct error indicator (icon and color)
8. WHEN an error occurs, THE Model_Runner SHALL allow the user to dismiss the error and return to the configuration dialog

### Requirement 11: Results Export

**User Story:** As a user, I want to export simulation results to CSV format, so that I can analyze them in external tools.

#### Acceptance Criteria

1. WHEN simulation results are displayed, THE Model_Runner SHALL provide an "Export to CSV" button
2. WHEN a user clicks the "Export to CSV" button, THE Model_Runner SHALL generate a CSV file containing all output values
3. WHEN generating the CSV file, THE Model_Runner SHALL include a header row with column names
4. WHEN generating the CSV file, THE Model_Runner SHALL include one row per iteration with the iteration number and output value
5. WHEN generating the CSV file, THE Model_Runner SHALL include the summary statistics as additional rows at the end
6. WHEN the CSV file is generated, THE Model_Runner SHALL trigger a browser download with a descriptive filename
7. THE Model_Runner SHALL use the format "simulation-results-YYYY-MM-DD-HHmmss.csv" for the filename

### Requirement 12: Integration with Model Builder

**User Story:** As a user, I want the simulation runner to integrate seamlessly with the model builder, so that I have a cohesive experience.

#### Acceptance Criteria

1. THE Model_Runner SHALL be accessible from the Model_Builder component
2. THE Model_Runner SHALL read the current model state including variables, intermediate expressions, and target expression from the Model_Builder's ModelService
3. WHEN the model state changes in the Model_Builder, THE Model_Runner SHALL reflect those changes immediately
4. WHEN the "Run Simulation" button is disabled, THE Model_Runner SHALL display a tooltip explaining why (e.g., "Define at least one variable and a valid target expression")
5. THE Model_Runner SHALL follow the same UI/UX guidelines as the Model_Builder (PrimeNG components, TailwindCSS styling)
