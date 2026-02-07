## ADDED Requirements

### Requirement: Support all comparison methods
The GUI SHALL support all four comparison methods: Filename, Filename & Size, Content Hash, and Sampled Hash.

#### Scenario: Filename comparison executes
- **WHEN** the user selects "Filename" method
- **AND** clicks "Compare"
- **THEN** the comparison SHALL use `FilenameOnlyStrategy`
- **AND** results SHALL display within 5 seconds for directories with < 1000 files

#### Scenario: Filename and Size comparison executes
- **WHEN** the user selects "Filename & Size" method
- **AND** clicks "Compare"
- **THEN** the comparison SHALL use `FilenameSizeStrategy`
- **AND** files with same name but different sizes SHALL appear in "Only in A" and "Only in B"

#### Scenario: Content Hash comparison executes
- **WHEN** the user selects "Content Hash" method
- **AND** clicks "Compare"
- **THEN** the comparison SHALL use `FastHashStrategy`
- **AND** files with identical content SHALL appear in "In Both"

#### Scenario: Sampled Hash comparison executes
- **WHEN** the user selects "Sampled Hash" method
- **AND** clicks "Compare"
- **THEN** the comparison SHALL use `SampledHashStrategy`
- **AND** comparison SHALL complete faster than Content Hash for large files

### Requirement: Comparison runs asynchronously
The GUI SHALL perform comparisons in a background thread to keep UI responsive.

#### Scenario: Compare button shows spinner
- **WHEN** the user clicks "Compare"
- **THEN** the UI SHALL display a spinner
- **AND** the "Compare" button SHALL be disabled
- **AND** the status bar SHALL show "Comparing..."

#### Scenario: Results display after comparison completes
- **WHEN** a background comparison completes
- **THEN** the spinner SHALL disappear
- **AND** results SHALL populate the tree view
- **AND** the summary bar SHALL show counts

#### Scenario: Error message displays on failure
- **WHEN** a comparison fails (e.g., permission denied)
- **THEN** an error message SHALL display in red
- **AND** the spinner SHALL disappear
- **AND** the "Compare" button SHALL be re-enabled
