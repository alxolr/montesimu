# Requirements Document

## Introduction

The Monte Carlo Model Builder UI is a feature that enables users to define simulation models through a graphical interface. Users can create variables with probability distributions and write mathematical expressions that reference these variables and use literal values. This feature focuses on the model definition interface and does not include simulation execution capabilities.

## Glossary

- **Model_Builder**: The UI component that allows users to define Monte Carlo simulation models
- **Variable**: A model element with a name and probability distribution that generates random values during simulation
- **Distribution**: A probability distribution type (Normal, Lognormal, or Uniform) that defines how a variable generates values
- **Expression**: A mathematical formula written by the user that combines variables and literal numeric values using operators
- **Identifier**: A valid name for a variable (alphanumeric characters, no spaces)
- **Expression_Validator**: The component that checks expression syntax and variable references

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

### Requirement 2: Expression Building

**User Story:** As a user, I want to write mathematical expressions using my defined variables and literal numeric values, so that I can specify the calculation for my Monte Carlo model output.

#### Acceptance Criteria

1. WHEN a user enters an expression, THE Expression_Validator SHALL validate that all referenced identifiers correspond to defined variables
2. WHEN a user enters an expression, THE Expression_Validator SHALL validate that the mathematical syntax is correct
3. WHEN a user enters an expression, THE Expression_Validator SHALL support addition, subtraction, multiplication, division, parentheses operators, and numeric literals
4. WHEN an expression contains undefined identifiers, THE Expression_Validator SHALL mark the expression as invalid and display which identifiers are undefined
5. WHEN an expression contains syntax errors, THE Expression_Validator SHALL mark the expression as invalid and display the syntax error location
6. WHEN an expression is valid, THE Model_Builder SHALL provide visual feedback indicating validity
7. WHEN an expression is invalid, THE Model_Builder SHALL provide visual feedback indicating invalidity with specific error messages
8. WHEN a variable is deleted, THE Expression_Validator SHALL re-validate the expression and update validity status

### Requirement 3: User Interface Layout

**User Story:** As a user, I want a clear and organized interface for building my model, so that I can efficiently define variables and expressions.

#### Acceptance Criteria

1. THE Model_Builder SHALL display two distinct sections for variables and expression input
2. THE Model_Builder SHALL display the variables section before the expression section
3. WHEN displaying the variables section, THE Model_Builder SHALL show an add button, a list of existing variables, and edit/delete actions for each variable
4. WHEN displaying the expression section, THE Model_Builder SHALL show a text input field with real-time validation feedback
5. THE Model_Builder SHALL use PrimeNG components for all UI elements
6. THE Model_Builder SHALL use TailwindCSS for layout and styling
7. THE Model_Builder SHALL follow the responsive design principles defined in the UI/UX guidelines

### Requirement 4: Data Validation

**User Story:** As a user, I want immediate feedback on validation errors, so that I can correct mistakes as I build my model.

#### Acceptance Criteria

1. WHEN a user enters invalid data in any field, THE Model_Builder SHALL display an inline error message describing the validation failure
2. WHEN a user enters a non-numeric value for distribution parameters, THE Model_Builder SHALL display an error message
3. WHEN a user enters an invalid identifier name, THE Model_Builder SHALL display an error message explaining valid identifier rules
4. WHEN a user corrects invalid data, THE Model_Builder SHALL remove the error message immediately
5. WHEN a user attempts to save a variable with invalid data, THE Model_Builder SHALL prevent the save operation
6. THE Model_Builder SHALL validate distribution parameters are appropriate for the distribution type (e.g., standard deviation must be positive)

### Requirement 5: State Management

**User Story:** As a developer, I want the application to use Angular signals for state management, so that the UI reactively updates when model data changes.

#### Acceptance Criteria

1. THE Model_Builder SHALL use Angular signals to store the list of variables
2. THE Model_Builder SHALL use Angular signals to store the expression text
3. THE Model_Builder SHALL use Angular signals to store validation state
4. WHEN a signal value changes, THE Model_Builder SHALL automatically update all dependent UI elements
5. WHEN a variable is added, edited, or deleted, THE Model_Builder SHALL update the corresponding signal
6. WHEN the expression text changes, THE Model_Builder SHALL trigger re-validation through signal updates

### Requirement 6: Component Architecture

**User Story:** As a developer, I want the feature to use standalone Angular components, so that the codebase follows modern Angular architecture patterns.

#### Acceptance Criteria

1. THE Model_Builder SHALL be implemented as a standalone Angular component
2. THE Model_Builder SHALL import only the required PrimeNG modules
3. WHEN creating sub-components for variables or expression input, THE Model_Builder SHALL use standalone components
4. THE Model_Builder SHALL not depend on NgModule declarations
5. THE Model_Builder SHALL use Angular 20 features and APIs

### Requirement 7: Distribution Preview Visualization

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
