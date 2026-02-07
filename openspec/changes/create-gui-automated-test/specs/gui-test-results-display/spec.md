## ADDED Requirements

### Requirement: Results display in tree view
The GUI SHALL display comparison results in a collapsible tree view organized by category.

#### Scenario: A-only entries display in tree
- **WHEN** comparison results contain entries only in Directory A
- **THEN** the "Only in A" section SHALL expand
- **AND** entries SHALL display with red color (#FF6464)
- **AND** directories SHALL show trailing "/"

#### Scenario: B-only entries display in tree
- **WHEN** comparison results contain entries only in Directory B
- **THEN** the "Only in B" section SHALL expand
- **AND** entries SHALL display with green color (#64FF64)
- **AND** directories SHALL show trailing "/"

#### Scenario: Both entries display in tree
- **WHEN** comparison results contain entries in both directories
- **THEN** the "In Both" section SHALL expand
- **AND** entries SHALL display with blue color (#64C8FF)
- **AND** directories SHALL show trailing "/"

#### Scenario: Nested directories render hierarchically
- **WHEN** results contain nested directories (e.g., "dir/subdir/file.txt")
- **THEN** the tree SHALL render with expandable parent nodes
- **AND** child nodes SHALL be indented

### Requirement: Summary statistics display
The GUI SHALL display summary statistics after comparison completes.

#### Scenario: Summary bar shows counts
- **WHEN** comparison completes
- **THEN** the bottom bar SHALL show "A Only: N"
- **AND** the bottom bar SHALL show "B Only: N"
- **AND** the bottom bar SHALL show "Both: N"
- **AND** the bottom bar SHALL show "Total: N"

#### Scenario: Empty results show zero counts
- **WHEN** comparison completes with no differences
- **THEN** all counts SHALL display as 0
