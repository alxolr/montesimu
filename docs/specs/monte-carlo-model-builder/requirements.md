# Requirements Document

## Introduction

The Monte Carlo Model Builder UI is a feature that enables users to define simulation models through a graphical interface. Users can create variables with probability distributions and write mathematical expressions that reference these variables and use literal values. This feature focuses on the model definition interface and does not include simulation execution capabilities.

## Glossary

- **Model_Builder**: The UI component that allows users to define Monte Carlo simulation models
- **Variable**: A model element with a name and probability distribution that generates random values during simulation
- **Distribution**: A probability distribution type (Normal, Lognormal, or Uniform) that defines how a variable generates values
- **Intermediate_Expression**: A named mathematical formula that can reference variables and other intermediate expressions, used to build complex calculations incrementally
- **Target_Expression**: The final mathematical formula that produces the model output, can reference variables and intermediate expressions
- **Expression**: A general term for either an intermediate expression or the target expression
- **Identifier**: A valid name for a variable or intermediate expression (alphanumeric characters, no spaces)
- **Expression_Validator**: The component that checks expression syntax and variable/expression references
- **Circular_Reference**: A dependency cycle where an expression directly or indirectly references itself
- **Dependency_Graph**: A directed graph representing which expressions depend on which variables and other expressions
- **Evaluation_Order**: The sequence in which expressions must be evaluated to ensure dependencies are resolved before use

## Requirements

### Requirement 1: Variable Management

**User Story:** As a user, I want to create and manage variables with probability distributions, so that I can define the uncertain inputs to my Monte Carlo model.

#### Acceptance Criteria

1. WHEN a user creates a new variable, THE Model_Builder SHALL require a name, distribution type, and distribution parameters
2. WHEN a user selects Normal distribution, THE Model_Builder SHALL require mean and standard deviation parameters
3. WHEN a user selects Lognormal distribution, THE Model_Builder SHALL require mean and standard deviation parameters
4. WHEN a user selects Uniform distribution, THE Model_Builder SHALL require minimum and maximum parameters
5. WHEN a user provides a variable name, THE Model_Builder SHALL validate that it is a valid identifier containing only alphanumeric characters
6. WHEN a user attempts to create a variable with a duplicate name, THE Model_Builder SHALL prevent creation and display an error message
7. WHEN a user edits an existing variable, THE Model_Builder SHALL update the variable definition and re-validate any expressions using that variable
8. WHEN a user deletes a variable, THE Model_Builder SHALL remove it from the model and mark any expressions using that variable as invalid
9. THE Model_Builder SHALL display a list of all defined variables with their names, distribution types, and parameters

### Requirement 2: Intermediate Expression Management

**User Story:** As a user, I want to create and manage intermediate expressions that can reference variables and other intermediate expressions, so that I can build complex calculations incrementally.

#### Acceptance Criteria

1. WHEN a user creates a new intermediate expression, THE Model_Builder SHALL require a name and formula
2. WHEN a user provides an intermediate expression name, THE Model_Builder SHALL validate that it is a valid identifier containing only alphanumeric characters
3. WHEN a user attempts to create an intermediate expression with a name that duplicates a variable or another intermediate expression, THE Model_Builder SHALL prevent creation and display an error message
4. WHEN a user enters an intermediate expression formula, THE Expression_Validator SHALL validate that all referenced identifiers correspond to defined variables or other intermediate expressions
5. WHEN a user enters an intermediate expression formula, THE Expression_Validator SHALL validate that the mathematical syntax is correct
6. WHEN a user enters an intermediate expression formula, THE Expression_Validator SHALL support addition, subtraction, multiplication, division, parentheses operators, and numeric literals
7. WHEN an intermediate expression formula contains undefined identifiers, THE Expression_Validator SHALL mark the expression as invalid and display which identifiers are undefined
8. WHEN an intermediate expression formula contains syntax errors, THE Expression_Validator SHALL mark the expression as invalid and display the syntax error location
9. WHEN an intermediate expression is valid, THE Model_Builder SHALL provide visual feedback indicating validity
10. WHEN an intermediate expression is invalid, THE Model_Builder SHALL provide visual feedback indicating invalidity with specific error messages
11. WHEN a user edits an existing intermediate expression, THE Model_Builder SHALL update the expression definition and re-validate any expressions that reference it
12. WHEN a user deletes an intermediate expression, THE Model_Builder SHALL check if any other expressions reference it and prevent deletion if dependencies exist, displaying which expressions depend on it
13. THE Model_Builder SHALL display a list of all defined intermediate expressions with their names and formulas

### Requirement 3: Target Expression Management

**User Story:** As a user, I want to define a single target expression that produces the final model output, so that I can specify what value the simulation should calculate.

#### Acceptance Criteria

1. WHEN a user enters a target expression formula, THE Expression_Validator SHALL validate that all referenced identifiers correspond to defined variables or intermediate expressions
2. WHEN a user enters a target expression formula, THE Expression_Validator SHALL validate that the mathematical syntax is correct
3. WHEN a user enters a target expression formula, THE Expression_Validator SHALL support addition, subtraction, multiplication, division, parentheses operators, and numeric literals
4. WHEN the target expression formula contains undefined identifiers, THE Expression_Validator SHALL mark the expression as invalid and display which identifiers are undefined
5. WHEN the target expression formula contains syntax errors, THE Expression_Validator SHALL mark the expression as invalid and display the syntax error location
6. WHEN the target expression is valid, THE Model_Builder SHALL provide visual feedback indicating validity
7. WHEN the target expression is invalid, THE Model_Builder SHALL provide visual feedback indicating invalidity with specific error messages
8. WHEN a variable or intermediate expression is deleted, THE Expression_Validator SHALL re-validate the target expression and update validity status
9. THE Model_Builder SHALL allow only one target expression to be defined

### Requirement 4: Circular Reference Detection

**User Story:** As a user, I want the system to prevent circular references in my expressions, so that my model can be evaluated without infinite loops.

#### Acceptance Criteria

1. WHEN a user creates or edits an intermediate expression that would create a circular reference, THE Model_Builder SHALL prevent the operation and display an error message
2. WHEN displaying a circular reference error, THE Model_Builder SHALL show the cycle path (e.g., "A → B → C → A")
3. WHEN a user creates an intermediate expression that references itself directly, THE Model_Builder SHALL detect this as a circular reference
4. WHEN a user creates an intermediate expression that references itself indirectly through other expressions, THE Model_Builder SHALL detect this as a circular reference
5. THE Model_Builder SHALL validate for circular references before saving any intermediate expression changes

### Requirement 5: Dependency Graph and Evaluation Order

**User Story:** As a developer, I want the system to build a dependency graph and determine the correct evaluation order, so that expressions are evaluated after their dependencies.

#### Acceptance Criteria

1. THE Model_Builder SHALL construct a dependency graph showing which expressions depend on which variables and other expressions
2. THE Model_Builder SHALL use topological sorting to determine the evaluation order for all expressions
3. WHEN the dependency graph contains a cycle, THE Model_Builder SHALL detect it during topological sort and report a circular reference error
4. THE Model_Builder SHALL ensure intermediate expressions are evaluated before any expressions that reference them
5. THE Model_Builder SHALL ensure the target expression is evaluated last, after all intermediate expressions it depends on

### Requirement 6: User Interface Layout

**User Story:** As a user, I want a clear and organized interface for building my model, so that I can efficiently define variables, intermediate expressions, and the target expression.

#### Acceptance Criteria

1. THE Model_Builder SHALL display three distinct sections for variables, intermediate expressions, and target expression input
2. THE Model_Builder SHALL display the sections in order: variables, intermediate expressions, target expression
3. WHEN displaying the variables section, THE Model_Builder SHALL show an add button, a list of existing variables, and edit/delete actions for each variable
4. WHEN displaying the intermediate expressions section, THE Model_Builder SHALL show an add button, a list of existing intermediate expressions with their names and formulas, and edit/delete actions for each expression
5. WHEN displaying the target expression section, THE Model_Builder SHALL show a text input field with real-time validation feedback
6. WHEN displaying an expression (intermediate or target), THE Model_Builder SHALL show which other identifiers it depends on
7. THE Model_Builder SHALL use PrimeNG components for all UI elements
8. THE Model_Builder SHALL use TailwindCSS for layout and styling
9. THE Model_Builder SHALL follow the responsive design principles defined in the UI/UX guidelines

### Requirement 7: Data Validation

**User Story:** As a user, I want immediate feedback on validation errors, so that I can correct mistakes as I build my model.

#### Acceptance Criteria

1. WHEN a user enters invalid data in any field, THE Model_Builder SHALL display an inline error message describing the validation failure
2. WHEN a user enters a non-numeric value for distribution parameters, THE Model_Builder SHALL display an error message
3. WHEN a user enters an invalid identifier name, THE Model_Builder SHALL display an error message explaining valid identifier rules
4. WHEN a user corrects invalid data, THE Model_Builder SHALL remove the error message immediately
5. WHEN a user attempts to save a variable or intermediate expression with invalid data, THE Model_Builder SHALL prevent the save operation
6. THE Model_Builder SHALL validate distribution parameters are appropriate for the distribution type (e.g., standard deviation must be positive)
7. THE Model_Builder SHALL detect circular references and display descriptive error messages with the cycle path

### Requirement 8: State Management

**User Story:** As a developer, I want the application to use Angular signals for state management, so that the UI reactively updates when model data changes.

#### Acceptance Criteria

1. THE Model_Builder SHALL use Angular signals to store the list of variables
2. THE Model_Builder SHALL use Angular signals to store the list of intermediate expressions
3. THE Model_Builder SHALL use Angular signals to store the target expression text
4. THE Model_Builder SHALL use Angular signals to store validation state for all expressions
5. WHEN a signal value changes, THE Model_Builder SHALL automatically update all dependent UI elements
6. WHEN a variable is added, edited, or deleted, THE Model_Builder SHALL update the corresponding signal
7. WHEN an intermediate expression is added, edited, or deleted, THE Model_Builder SHALL update the corresponding signal and re-validate dependent expressions
8. WHEN the target expression text changes, THE Model_Builder SHALL trigger re-validation through signal updates

### Requirement 9: Component Architecture

**User Story:** As a developer, I want the feature to use standalone Angular components, so that the codebase follows modern Angular architecture patterns.

#### Acceptance Criteria

1. THE Model_Builder SHALL be implemented as a standalone Angular component
2. THE Model_Builder SHALL import only the required PrimeNG modules
3. WHEN creating sub-components for variables or expression input, THE Model_Builder SHALL use standalone components
4. THE Model_Builder SHALL not depend on NgModule declarations
5. THE Model_Builder SHALL use Angular 20 features and APIs

### Requirement 10: Distribution Preview Visualization

**User Story:** As a user, I want to see a real-time preview of the probability distribution when adding or editing a variable, so that I can understand the shape and characteristics of the distribution I'm defining.

#### Acceptance Criteria

1. WHEN a user opens the variable form dialog, THE Model_Builder SHALL display a preview chart on the right side of the form
2. WHEN a user selects Normal distribution, THE Model_Builder SHALL display a bell curve visualization of the probability density function
3. WHEN a user selects Lognormal distribution, THE Model_Builder SHALL display a skewed distribution visualization of the probability density function
4. WHEN a user selects Uniform distribution, THE Model_Builder SHALL display a flat distribution visualization between min and max values
5. WHEN a user changes distribution parameters (mean, stdDev, min, max), THE Model_Builder SHALL update the preview chart in real-time
6. WHEN displaying the preview chart, THE Model_Builder SHALL show the min and max values on the chart axes
7. THE Model_Builder SHALL use a line chart component to visualize the probability density function
8. WHEN the form dialog is displayed, THE Model_Builder SHALL arrange the form fields on the left and the preview chart on the right
