## ADDED Requirements

### Requirement: Theme selection persistence
The application SHALL allow the user to select a visual theme (Light, Dark, or System) and respect this choice.

#### Scenario: Switching to Dark mode
- **WHEN** the user selects "Dark" theme
- **THEN** the UI updates to dark mode colors immediately

#### Scenario: Switching to System mode
- **WHEN** the user selects "System" theme
- **THEN** the UI adopts the operating system's current theme preference
- **AND** updates if the OS theme changes (if supported) or documents limitation

### Requirement: System theme limitations
If system theme detection/updates require a restart or have limitations, the UI SHALL inform the user or handle it gracefully.

#### Scenario: System theme reset limitation
- **WHEN** switching from an override back to "System" is difficult without reload
- **THEN** the UI either forces a reload OR warns the user OR is refactored to handle it correctly
