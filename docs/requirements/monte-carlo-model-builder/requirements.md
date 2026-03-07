# Requirements Document

## Introduction

The Monte Carlo Model Builder UI is a feature that enables users to define simulation models through a graphical interface. Users can create variables with probability distributions, define constants with fixed values, and write mathematical expressions that combine these elements. This feature focuses on the model definition interface and does not include simulation execution capabilities.

## Glossary

- **Model_Builder**: The UI component that allows users to define Monte Carlo simulation models
- **Variable**: A model element with a name and probability distribution that generates random values during simulation
- **Constant**: A model element with a name and fixed numeric value
- **Distribution**: A probability distribution type (Normal, Lognormal, or Uniform) that defines how a variable generates values
- **Expression**: A mathematical formula written by the user that combines variables and constants using operators
- **Identifier**: A valid name for a variable or constant (alphanumeric characters, no spaces)
- **Expression_Validator**: The component that checks expression syntax and variable/constant references

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

### Requirement 2: Constant Management

**User Story:** As a user, I want to create and manage constants with fixed values, so that I can define known parameters in my Monte Carlo model.

#### Acceptance Criteria

1. WHEN a user creates a new constant, THE Model_Builder SHALL require a name and numeric value
2. WHEN a user provides a constant name, THE Model_Builder SHALL validate that it is a valid identifier containing only alphanumeric characters
3. WHEN a user attempts to create a constant with a duplicate name, THE Model_Builder SHALL prevent creation and display an error message
4. WHEN a user provides a constant value, THE Model_Builder SHALL validate that it is a valid number
5. WHEN a user edits an existing constant, THE Model_Builder SHALL update the constant definition and re-validate any expressions using that constant
6. WHEN a user deletes a constant, THE Model_Builder SHALL remove it from the model and mark any expressions using that constant as invalid
7. THE Model_Builder SHALL display a list of all defined constants with their names and values

### Requirement 3: Expression Building

**User Story:** As a user, I want to write mathematical expressions using my defined variables and constants, so that I can specify the calculation for my Monte Carlo model output.

#### Acceptance Criteria

1. WHEN a user enters an expression, THE Expression_Validator SHALL validate that all referenced identifiers correspond to defined variables or constants
2. WHEN a user enters an expression, THE Expression_Validator SHALL validate that the mathematical syntax is correct
3. WHEN a user enters an expression, THE Expression_Validator SHALL support addition, subtraction, multiplication, division, and parentheses operators
4. WHEN an expression contains undefined identifiers, THE Expression_Validator SHALL mark the expression as invalid and display which identifiers are undefined
5. WHEN an expression contains syntax errors, THE Expression_Validator SHALL mark the expression as invalid and display the syntax error location
6. WHEN an expression is valid, THE Model_Builder SHALL provide visual feedback indicating validity
7. WHEN an expression is invalid, THE Model_Builder SHALL provide visual feedback indicating invalidity with specific error messages
8. WHEN a variable or constant is deleted, THE Expression_Validator SHALL re-validate the expression and update validity status

### Requirement 4: User Interface Layout

**User Story:** As a user, I want a clear and organized interface for building my model, so that I can efficiently define variables, constants, and expressions.

#### Acceptance Criteria

1. THE Model_Builder SHALL display three distinct sections for variables, constants, and expression input
2. THE Model_Builder SHALL display the variables section before the constants section
3. THE Model_Builder SHALL display the constants section before the expression section
4. WHEN displaying the variables section, THE Model_Builder SHALL show an add button, a list of existing variables, and edit/delete actions for each variable
5. WHEN displaying the constants section, THE Model_Builder SHALL show an add button, a list of existing constants, and edit/delete actions for each constant
6. WHEN displaying the expression section, THE Model_Builder SHALL show a text input field with real-time validation feedback
7. THE Model_Builder SHALL use PrimeNG components for all UI elements
8. THE Model_Builder SHALL use TailwindCSS for layout and styling
9. THE Model_Builder SHALL follow the responsive design principles defined in the UI/UX guidelines

### Requirement 5: Data Validation

**User Story:** As a user, I want immediate feedback on validation errors, so that I can correct mistakes as I build my model.

#### Acceptance Criteria

1. WHEN a user enters invalid data in any field, THE Model_Builder SHALL display an inline error message describing the validation failure
2. WHEN a user enters a non-numeric value for distribution parameters, THE Model_Builder SHALL display an error message
3. WHEN a user enters a non-numeric value for constant values, THE Model_Builder SHALL display an error message
4. WHEN a user enters an invalid identifier name, THE Model_Builder SHALL display an error message explaining valid identifier rules
5. WHEN a user corrects invalid data, THE Model_Builder SHALL remove the error message immediately
6. WHEN a user attempts to save a variable or constant with invalid data, THE Model_Builder SHALL prevent the save operation
7. THE Model_Builder SHALL validate distribution parameters are appropriate for the distribution type (e.g., standard deviation must be positive)

### Requirement 6: State Management

**User Story:** As a developer, I want the application to use Angular signals for state management, so that the UI reactively updates when model data changes.

#### Acceptance Criteria

1. THE Model_Builder SHALL use Angular signals to store the list of variables
2. THE Model_Builder SHALL use Angular signals to store the list of constants
3. THE Model_Builder SHALL use Angular signals to store the expression text
4. THE Model_Builder SHALL use Angular signals to store validation state
5. WHEN a signal value changes, THE Model_Builder SHALL automatically update all dependent UI elements
6. WHEN a variable or constant is added, edited, or deleted, THE Model_Builder SHALL update the corresponding signal
7. WHEN the expression text changes, THE Model_Builder SHALL trigger re-validation through signal updates

### Requirement 7: Component Architecture

**User Story:** As a developer, I want the feature to use standalone Angular components, so that the codebase follows modern Angular architecture patterns.

#### Acceptance Criteria

1. THE Model_Builder SHALL be implemented as a standalone Angular component
2. THE Model_Builder SHALL import only the required PrimeNG modules
3. WHEN creating sub-components for variables, constants, or expression input, THE Model_Builder SHALL use standalone components
4. THE Model_Builder SHALL not depend on NgModule declarations
5. THE Model_Builder SHALL use Angular 20 features and APIs
