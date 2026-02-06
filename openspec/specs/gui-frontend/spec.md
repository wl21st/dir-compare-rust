## ADDED Requirements

### Requirement: Application window SHALL display with title bar and controls
The GUI application SHALL create a native application window with a title bar containing the application name "dir-compare" and standard window controls (minimize, maximize, close) appropriate to the operating system.

#### Scenario: Window opens on application launch
- **WHEN** the user launches the GUI application
- **THEN** a window with title "dir-compare" SHALL appear on screen

#### Scenario: Window can be minimized
- **WHEN** the user clicks the minimize button
- **THEN** the window SHALL minimize to the system taskbar/dock

#### Scenario: Window can be maximized
- **WHEN** the user clicks the maximize button
- **THEN** the window SHALL expand to fill the available screen space

#### Scenario: Window can be closed
- **WHEN** the user clicks the close button
- **THEN** the window SHALL close and the application SHALL exit

### Requirement: Directory path inputs SHALL accept user input
The GUI SHALL provide text input fields for specifying the two directories to compare.

#### Scenario: User can enter path for directory A
- **WHEN** the user types a valid directory path into the "Directory A" input field
- **THEN** the path SHALL be displayed in the input field
- **AND** the field SHALL display a validation indicator if the path is valid

#### Scenario: User can enter path for directory B
- **WHEN** the user types a valid directory path into the "Directory B" input field
- **THEN** the path SHALL be displayed in the input field
- **AND** the field SHALL display a validation indicator if the path is valid

#### Scenario: Invalid path shows error indicator
- **WHEN** the user enters a path that does not exist or is not a directory
- **THEN** the input field SHALL display an error indicator
- **AND** the comparison button SHALL remain disabled

### Requirement: File picker dialogs SHALL allow directory selection
The GUI SHALL provide button controls that open native file picker dialogs for directory selection.

#### Scenario: File picker opens for directory A
- **WHEN** the user clicks the "Browse" button next to Directory A
- **THEN** a native file picker dialog SHALL open
- **AND** selecting a directory and clicking "Open" SHALL populate the Directory A input field

#### Scenario: File picker opens for directory B
- **WHEN** the user clicks the "Browse" button next to Directory B
- **THEN** a native file picker dialog SHALL open
- **AND** selecting a directory and clicking "Open" SHALL populate the Directory B input field

#### Scenario: File picker defaults to directories only
- **WHEN** the file picker dialog opens
- **THEN** the dialog SHALL be configured to show only directories by default
- **AND** the user SHALL be prevented from selecting files

### Requirement: Comparison method selector SHALL allow method choice
The GUI SHALL provide a dropdown or radio button selection for choosing the comparison method.

#### Scenario: User can select filename method
- **WHEN** the user selects "Filename" from the comparison method dropdown
- **THEN** the comparison SHALL match files with the same name regardless of size or content

#### Scenario: User can select size method
- **WHEN** the user selects "Size" from the comparison method dropdown
- **THEN** the comparison SHALL match files with the same name AND same size

#### Scenario: User can select hash method
- **WHEN** the user selects "Hash" from the comparison method dropdown
- **THEN** the comparison SHALL match files with the same name AND identical content hash

### Requirement: Compare button SHALL trigger directory comparison
The GUI SHALL provide a "Compare" button that initiates the directory comparison when both directories are valid.

#### Scenario: Compare button enabled when directories are valid
- **WHEN** both Directory A and Directory B contain valid paths
- **THEN** the "Compare" button SHALL be enabled

#### Scenario: Compare button disabled when directories are invalid
- **WHEN** either Directory A or Directory B contains an invalid path
- **THEN** the "Compare" button SHALL be disabled

#### Scenario: Clicking Compare triggers comparison
- **WHEN** the user clicks the "Compare" button while both directories are valid
- **THEN** the application SHALL execute the directory comparison
- **AND** a loading indicator SHALL be displayed during comparison
- **AND** the results SHALL appear when comparison completes

### Requirement: Comparison results SHALL display in a tree view
The GUI SHALL display comparison results as an expandable tree view showing directory structure and file statuses.

#### Scenario: Results show A-only entries
- **WHEN** comparison completes
- **THEN** entries that exist only in Directory A SHALL be listed under "A-only"
- **AND** each entry SHALL indicate whether it is a file or directory

#### Scenario: Results show B-only entries
- **WHEN** comparison completes
- **THEN** entries that exist only in Directory B SHALL be listed under "B-only"
- **AND** each entry SHALL indicate whether it is a file or directory

#### Scenario: Results show matching entries
- **WHEN** comparison completes
- **THEN** entries that exist in both directories SHALL be listed under "Both"
- **AND** matching entries SHALL show side-by-side comparison

#### Scenario: Directory entries are expandable
- **WHEN** the user clicks on a directory entry with children
- **THEN** the directory SHALL expand to show its contents
- **AND** the entry SHALL display an expand indicator

#### Scenario: Expanded directories can collapse
- **WHEN** the user clicks on an expanded directory entry
- **THEN** the directory SHALL collapse to hide its contents
- **AND** the entry SHALL display a collapse indicator

### Requirement: File and directory entries SHALL be visually distinguishable
The GUI SHALL use visual indicators to distinguish between files and directories.

#### Scenario: Directories show folder icon
- **WHEN** a directory entry is displayed in the results
- **THEN** the entry SHALL display a folder icon

#### Scenario: Files show file icon
- **WHEN** a file entry is displayed in the results
- **THEN** the entry SHALL display a generic file icon

### Requirement: Statistics summary SHALL display after comparison
The GUI SHALL display summary statistics about the comparison results.

#### Scenario: Statistics show count of A-only entries
- **WHEN** comparison completes
- **THEN** the summary SHALL display the count of entries only in Directory A

#### Scenario: Statistics show count of B-only entries
- **WHEN** comparison completes
- **THEN** the summary SHALL display the count of entries only in Directory B

#### Scenario: Statistics show count of matching entries
- **WHEN** comparison completes
- **THEN** the summary SHALL display the count of entries in both directories

#### Scenario: Statistics show total entries
- **WHEN** comparison completes
- **THEN** the summary SHALL display the total number of entries scanned
