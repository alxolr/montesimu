---
inclusion: fileMatch
fileMatchPattern: '**/*.component.ts,**/*.component.html,**/*.css'
---

# UI/UX Guidelines

## Design System

### UI Framework: PrimeNG
Montesimu uses PrimeNG 20.3.0 with the Lara theme for consistent, professional UI components.

### Theme Configuration
```typescript
// app.config.ts
providePrimeNG({
  theme: {
    preset: Preset,
    options: {
      darkModeSelector: false  // Light mode only
    }
  }
})
```

### Styling Stack
- **TailwindCSS 4.1.17**: Utility-first CSS framework
- **PrimeUI Plugin**: TailwindCSS integration for PrimeNG
- **Component Styles**: Scoped CSS per component

## Component Guidelines

### PrimeNG Components in Use
- **CardModule**: Container for content sections
- **ButtonModule**: Interactive buttons
- **InputTextModule**: Text input fields
- **ChartModule**: Data visualization (Chart.js wrapper)

### Using PrimeNG Components
```typescript
import { CardModule } from 'primeng/card';
import { ButtonModule } from 'primeng/button';

@Component({
  selector: 'app-example',
  standalone: true,
  imports: [CardModule, ButtonModule],
  template: `
    <p-card header="Title">
      <p>Content goes here</p>
      <p-button label="Click Me" (onClick)="handleClick()"></p-button>
    </p-card>
  `
})
```

## Layout Principles

### Responsive Design
- Design mobile-first, scale up
- Use TailwindCSS responsive utilities
- Test on multiple screen sizes
- Default window size: 1440x900

### Spacing
Use TailwindCSS spacing scale:
- `p-4`: Padding (1rem)
- `m-2`: Margin (0.5rem)
- `gap-4`: Grid/flex gap (1rem)

### Grid and Flexbox
```html
<!-- Flex container -->
<div class="flex items-center justify-between gap-4">
  <div>Item 1</div>
  <div>Item 2</div>
</div>

<!-- Grid container -->
<div class="grid grid-cols-2 gap-4">
  <div>Column 1</div>
  <div>Column 2</div>
</div>
```

## Color Palette

### Primary Colors
Based on PrimeNG Lara theme:
- **Primary**: Blue tones for main actions
- **Background**: White/light gray
- **Text**: Dark gray/black (#000000, #757575)
- **Borders**: Light gray (#e0e0e0)

### Chart Colors
```typescript
// Histogram component colors
const backgrounds = ['rgba(9, 76, 124, 0.2)'];  // Light blue
const borders = ['rgba(9, 76, 124, 1)'];        // Dark blue
```

### Semantic Colors
- **Success**: Green
- **Warning**: Yellow/Orange
- **Error**: Red
- **Info**: Blue

## Typography

### Font Hierarchy
- Use system fonts for performance
- PrimeNG provides default typography
- Consistent font sizes across components

### Text Styles
```css
/* Headings */
h1 { font-size: 2rem; font-weight: 600; }
h2 { font-size: 1.5rem; font-weight: 600; }
h3 { font-size: 1.25rem; font-weight: 500; }

/* Body */
p { font-size: 1rem; line-height: 1.5; }

/* Small text */
.text-sm { font-size: 0.875rem; }
```

## Chart Visualization

### Chart.js Configuration
```typescript
options = {
  maintainAspectRatio: false,
  responsive: true,
  devicePixelRatio: 2,  // High DPI displays
  animation: {
    duration: 0  // Disable for performance with large datasets
  },
  plugins: {
    legend: {
      labels: {
        color: "#000000",
        font: { size: 12 }
      }
    }
  },
  scales: {
    x: {
      ticks: {
        color: "#757575",
        font: { weight: 500 }
      },
      grid: {
        color: "#e0e0e0",
        drawBorder: false
      }
    },
    y: {
      ticks: { color: "#757575" },
      grid: {
        color: "#e0e0e0",
        drawBorder: false
      }
    }
  }
};
```

### Chart Best Practices
- Disable animations for large datasets (performance)
- Use high DPI ratio for crisp rendering
- Consistent color scheme across charts
- Clear axis labels and legends
- Responsive sizing

## Button Guidelines

### Action Buttons
For inline actions (edit, delete, etc.) in lists or tables:
- Use icon-only buttons without text labels
- Add tooltips for accessibility and clarity
- Use PrimeIcons for consistency:
  - Edit: `pi-pencil`
  - Delete: `pi-trash` or `pi-times`
  - Add: `pi-plus`
  - Save: `pi-check`
  - Cancel: `pi-times`
- Use appropriate severity colors:
  - Edit: `info` (light blue)
  - Delete: `danger` (light red)
  - Primary actions: default or `primary`
- Use `[outlined]="true"` for subtle appearance with colored border and icon
- Use `[rounded]="true"` for modern circular buttons
- Always include `pTooltip` directive for accessibility
- Buttons are always visible (do NOT use `[text]="true"` which makes them hover-only)

Example:
```html
<!-- Edit button (light blue outline) -->
<p-button 
  icon="pi pi-pencil" 
  (onClick)="edit()"
  severity="info"
  size="small"
  [outlined]="true"
  [rounded]="true"
  pTooltip="Edit"
  tooltipPosition="top"
></p-button>

<!-- Delete button (light red outline) -->
<p-button 
  icon="pi pi-times" 
  (onClick)="delete()"
  severity="danger"
  size="small"
  [outlined]="true"
  [rounded]="true"
  pTooltip="Delete"
  tooltipPosition="top"
></p-button>
```

### Primary Action Buttons
For main actions (submit forms, create new items):
- Include both icon and text label
- Use larger size if needed
- Place prominently in the UI

Example:
```html
<p-button 
  label="Add Variable" 
  icon="pi pi-plus" 
  (onClick)="add()"
></p-button>
```

## Accessibility

### ARIA Labels
```html
<button aria-label="Generate histogram">Generate</button>
<input aria-label="Enter name" type="text" />
```

### Keyboard Navigation
- All interactive elements must be keyboard accessible
- Logical tab order
- Visible focus indicators
- Escape key to close modals/dialogs

### Screen Reader Support
- Use semantic HTML
- Provide alt text for images
- Label form inputs properly
- Announce dynamic content changes

### Color Contrast
- Maintain WCAG AA standards (4.5:1 for normal text)
- Don't rely solely on color to convey information
- Test with color blindness simulators

## Form Design

### Input Fields
```html
<form (submit)="handleSubmit($event)">
  <label for="name">Name</label>
  <input 
    pInputText 
    id="name" 
    type="text" 
    [(ngModel)]="name"
    placeholder="Enter your name"
    required
  />
  <p-button type="submit" label="Submit"></p-button>
</form>
```

### Validation
- Inline validation messages
- Clear error states
- Disable submit until valid
- Helpful error messages

## Performance Considerations

### Chart Rendering
- Disable animations for large datasets
- Use `devicePixelRatio: 2` for quality
- Consider data sampling for very large datasets
- Lazy load chart components if needed

### Component Optimization
```typescript
@Component({
  changeDetection: ChangeDetectionStrategy.OnPush  // When appropriate
})
```

### Image Optimization
- Use appropriate image formats (WebP, SVG)
- Lazy load images below the fold
- Provide width/height attributes

## Component Structure

### Template Organization
```html
<!-- Header -->
<header>
  <h1>{{ title }}</h1>
</header>

<!-- Main content -->
<main>
  <p-card>
    <!-- Content -->
  </p-card>
</main>

<!-- Footer (if needed) -->
<footer>
  <!-- Footer content -->
</footer>
```

### Style Scoping
```typescript
@Component({
  selector: 'app-histogram',
  styleUrl: './histogram.component.css'  // Scoped styles
})
```

## Interaction Patterns

### Loading States
```html
<p-button 
  [loading]="isLoading" 
  label="Generate"
  (onClick)="generate()"
></p-button>
```

### Error Handling
```html
<div *ngIf="error" class="error-message">
  {{ error }}
</div>
```

### Empty States
```html
<div *ngIf="data.length === 0" class="empty-state">
  <p>No data available</p>
  <p-button label="Generate Data" (onClick)="generateData()"></p-button>
</div>
```

## Animation Guidelines

### When to Animate
- Page transitions
- Modal open/close
- Hover states
- Loading indicators

### When NOT to Animate
- Large dataset rendering (performance)
- Frequent updates
- Accessibility concerns (motion sensitivity)

### Angular Animations
```typescript
import { trigger, transition, style, animate } from '@angular/animations';

animations: [
  trigger('fadeIn', [
    transition(':enter', [
      style({ opacity: 0 }),
      animate('300ms', style({ opacity: 1 }))
    ])
  ])
]
```

## Testing UI Components

### Visual Testing
- Test on different screen sizes
- Test with different data volumes
- Test loading and error states
- Test keyboard navigation

### User Testing
- Observe real users interacting with UI
- Gather feedback on usability
- Iterate based on findings

## Design Checklist
- [ ] Consistent spacing and alignment
- [ ] Proper color contrast
- [ ] Responsive on all screen sizes
- [ ] Keyboard accessible
- [ ] Screen reader friendly
- [ ] Loading states implemented
- [ ] Error states handled
- [ ] Empty states designed
- [ ] Performance optimized
- [ ] Cross-browser tested
