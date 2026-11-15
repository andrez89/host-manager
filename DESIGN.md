---
name: Libadwaita Slate
colors:
  surface: '#131316'
  surface-dim: '#131316'
  surface-bright: '#39393c'
  surface-container-lowest: '#0e0e11'
  surface-container-low: '#1b1b1e'
  surface-container: '#1f1f22'
  surface-container-high: '#2a2a2d'
  surface-container-highest: '#353438'
  on-surface: '#e4e1e6'
  on-surface-variant: '#bbcabf'
  inverse-surface: '#e4e1e6'
  inverse-on-surface: '#303033'
  outline: '#86948a'
  outline-variant: '#3c4a42'
  surface-tint: '#4edea3'
  primary: '#4edea3'
  on-primary: '#003824'
  primary-container: '#10b981'
  on-primary-container: '#00422b'
  inverse-primary: '#006c49'
  secondary: '#c6c5cf'
  on-secondary: '#2f3038'
  secondary-container: '#4a4b53'
  on-secondary-container: '#bcbbc5'
  tertiary: '#ffb95f'
  on-tertiary: '#472a00'
  tertiary-container: '#e29100'
  on-tertiary-container: '#523200'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#6ffbbe'
  primary-fixed-dim: '#4edea3'
  on-primary-fixed: '#002113'
  on-primary-fixed-variant: '#005236'
  secondary-fixed: '#e3e1ec'
  secondary-fixed-dim: '#c6c5cf'
  on-secondary-fixed: '#1a1b22'
  on-secondary-fixed-variant: '#46464e'
  tertiary-fixed: '#ffddb8'
  tertiary-fixed-dim: '#ffb95f'
  on-tertiary-fixed: '#2a1700'
  on-tertiary-fixed-variant: '#653e00'
  background: '#131316'
  on-background: '#e4e1e6'
  surface-variant: '#353438'
typography:
  display:
    fontFamily: Inter
    fontSize: 2rem
    fontWeight: '700'
    lineHeight: 2.5rem
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 1.5rem
    fontWeight: '600'
    lineHeight: 2rem
    letterSpacing: -0.015em
  headline-md:
    fontFamily: Inter
    fontSize: 1.25rem
    fontWeight: '600'
    lineHeight: 1.75rem
    letterSpacing: -0.01em
  headline-sm:
    fontFamily: Inter
    fontSize: 1.125rem
    fontWeight: '600'
    lineHeight: 1.5rem
  title-md:
    fontFamily: Inter
    fontSize: 1rem
    fontWeight: '500'
    lineHeight: 1.5rem
  body-lg:
    fontFamily: Inter
    fontSize: 1rem
    fontWeight: '400'
    lineHeight: 1.5rem
  body-md:
    fontFamily: Inter
    fontSize: 0.875rem
    fontWeight: '400'
    lineHeight: 1.375rem
  body-sm:
    fontFamily: Inter
    fontSize: 0.75rem
    fontWeight: '400'
    lineHeight: 1.125rem
  label-lg:
    fontFamily: Inter
    fontSize: 0.875rem
    fontWeight: '600'
    lineHeight: 1.25rem
  label-md:
    fontFamily: Inter
    fontSize: 0.75rem
    fontWeight: '600'
    lineHeight: 1rem
    letterSpacing: 0.02em
  label-sm:
    fontFamily: Inter
    fontSize: 0.6875rem
    fontWeight: '500'
    lineHeight: 0.875rem
    letterSpacing: 0.04em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  gutter: 1rem
  gutter-compact: 0.5rem
  margin: 1.5rem
  margin-compact: 1rem
  space-xs: 0.25rem
  space-sm: 0.5rem
  space-md: 0.75rem
  space-lg: 1rem
  space-xl: 1.5rem
---

## Brand & Style

This design system delivers a focused, native desktop computing experience rooted in modern GNOME Human Interface Guidelines (HIG) and Libadwaita patterns. Engineered specifically for high-performance Linux workstations and developer tooling, it balances technical efficiency with refined visual restraint.

The aesthetic blends structural minimalism with tactile depth:
- Deep neutral foundations (#18181B to #27272A) create an ambient, eye-resting workspace.
- Sharp, precise layout grids prioritize functional clarity and spatial balance over decorative noise.
- Crisp status indicators in vibrant emerald (#10B981) give immediate, confidence-inspiring feedback for secure, active, and compiled states.
- Tactile feedback relies on low-contrast surface step-ups, hairline inner highlights, and distinct active states rather than heavy drop shadows.

## Colors

The palette adopts standard Libadwaita surface stacking over dark charcoal foundations:

- **Primary (`#10B981`)**: The Emerald accent. Denotes active switches, primary confirmations, success indicators, and secure operational states.
- **Secondary (`#71717A`)**: Neutral steel. Applied to auxiliary icons, inactive toggles, and secondary control states.
- **Tertiary (`#F59E0B`)**: Amber caution. Reserved for degraded hardware states, unsaved changes, or compilation warnings.
- **Neutral (`#18181B`)**: Zinc baseline. Defines the deepest background layers of the window canvas.

### Surface Tiers
- **Window Base (`#18181B`)**: Underneath all split views and headerbars.
- **View / Sidebar (`#1E1E24`)**: Navigation panes and lateral tool panels.
- **Card / Surface Container (`#27272A`)**: Interactive cards, input fields, popovers, and elevated toolbar elements.
- **Surface Hover (`#323238`)**: Micro-interaction feedback layer for elevated controls.
- **Surface Stroke (`#3F3F46`)**: Subdued 1px borders providing contrast separation between stacked surfaces.

## Typography

Inter serves as the single typographic workhorse, configured to emulate native desktop system fonts. 

- **Weight Hierarchy**: Standardized on Regular (400) for reading text and descriptive labels, Medium (500) for list row headers and tab labels, and Semi-Bold (600) for interactive triggers, modal titles, and section dividers.
- **Readability**: Letter spacing tightens slightly on larger headings to retain cohesion on desktop displays, while smaller captions and badges use tracking expansion (`0.02em` to `0.04em`) to ensure legibility on dark backgrounds.
- **Numbers & Metrics**: Tabular figures (`tnum`) should be enabled for counters, gauges, IP displays, and hardware stats to maintain visual alignment across updating lists.

## Layout & Spacing

The layout adheres to GTK4 desktop application architecture:

- **Window Model**: Root application windows utilize a multi-pane layout composed of a collapsable sidebar (fixed width `240px`–`280px`) alongside a fluid content canvas.
- **Headerbars**: Integrated client-side decoration (CSD) headerbars with a standard vertical height of `48px`, anchoring window controls, search bars, and primary view switchers.
- **Content Panes**: View containers respect a `1.5rem` (`24px`) margin on standard windows, stepping down to `1rem` (`16px`) when tiled or rendered in compact split modes.
- **Internal Rhythm**: Component groups use an `8px` baseline grid. Vertical spacing between stacked cards or settings blocks is governed by `space-md` (`12px`) or `space-lg` (`16px`) to produce compact, high-density interfaces suited to native desktop software.

## Elevation & Depth

Libadwaita avoids diffuse real-world shadows in favor of stacked surface luminances and structured outlines:

- **Tonal Stepping**: Depth is conveyed by lifting surfaces toward lighter neutrals. The app canvas rests at `#18181B`, cards and sidebars sit at `#1E1E24` or `#27272A`, and elevated dropdowns or modals occupy `#2D2D32`.
- **Inner Stroke Definition**: Flat interfaces are prone to visual blending. All container elements, buttons, and popovers employ a 1px solid border (`rgba(255, 255, 255, 0.08)` or `#3F3F46`) to yield a crisp, machined edge.
- **Window Shadows**: Top-level modal sheets and popovers feature a tightly focused perimeter drop shadow (`0 8px 24px rgba(0, 0, 0, 0.45)`) combined with a 1px border highlight (`rgba(255, 255, 255, 0.12)`) on the upper edge to simulate top-down ambient light.

## Shapes

The design system adopts level `2` roundedness, matching GNOME HIG specifications:

- **Standard Controls (0.5rem / 8px)**: Buttons, text inputs, list row highlights, and toggle tracks utilize `8px` corners.
- **Containers & Cards (1rem / 16px)**: Settings groups, elevated cards, and floating status banners scale up to `16px` outer radiuses.
- **Window Corners (0.75rem / 12px)**: Root application windows use Libadwaita's smooth continuous curvature.
- **Badges & Switches**: Inline status dots, pill counters, and switch toggles use fully rounded capsule ends to differentiate interactive state badges from static container geometry.

## Components

### Buttons
- **Standard Action**: Background `#27272A`, border 1px `#3F3F46`, radius `8px`, text `#E4E4E7`. On hover, background shifts to `#323238`. On active click, scales down subtly with an inset highlight.
- **Suggested / Primary**: Background `#10B981`, text `#09090B` (bold contrast), border 1px `rgba(255, 255, 255, 0.15)`. Hover lightens to `#34D399`.
- **Destructive**: Background `#EF4444`, text `#FFFFFF`. Hover lightens to `#F87171`.
- **Flat / Ghost**: Transparent fill, text `#A1A1AA`, hover fill `rgba(255, 255, 255, 0.06)`. Used for toolbar utilities.

### Switches & Toggles
- **Track**: Track dimensions `44px × 24px`, fully rounded. Inactive track is `#27272A` with 1px border `#3F3F46`. Active track fills with `#10B981`.
- **Thumb**: Smooth white/off-white circle (`#FAFAFA`), `20px` diameter, centered with a 2px margin. Smooth horizontal transition (200ms ease-out).

### Cards & Settings Rows (Preferences Group)
- Groups of configuration rows are consolidated into unified rounded containers (`12px` or `16px` radius) with background `#1E1E24` and a 1px `#27272A` border.
- Each row contains an optional icon, label, descriptive subtitle, and control trailing widget (switch, dropdown, or action chevron).
- Separators between rows are 1px hairline rules (`rgba(255, 255, 255, 0.05)`), inset from the leading icon margin.

### Checkboxes & Radios
- **Checkbox**: `18px × 18px` square with `4px` radius. Inactive: `#27272A` with `#3F3F46` border. Checked: `#10B981` with white checkmark icon.
- **Radio**: `18px × 18px` circle. Checked state features a centered `#10B981` filled pip with an emerald outer ring.

### Input Fields
- Single-line inputs use background `#18181B`, border 1px `#3F3F46`, radius `8px`, and horizontal padding `12px`.
- Active focus state transitions the border to `#10B981` with a subtle inner glow ring (`0 0 0 1px #10B981`).

### Status Badges
- Compact pill elements (`height: 22px`) with semi-transparent tinted backgrounds.
- **Protected / Online**: Background `rgba(16, 185, 129, 0.12)`, text `#34D399`, leading `6px` solid dot in `#10B981`.
- **Warning / Degraded**: Background `rgba(245, 158, 11, 0.12)`, text `#FBBF24`, leading `6px` solid dot in `#F59E0B`.
- **Neutral / Idle**: Background `rgba(113, 113, 122, 0.15)`, text `#A1A1AA`, leading `6px` dot in `#71717A`.