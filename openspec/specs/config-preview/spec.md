# config-preview Specification

## Purpose

TBD - created by archiving change config-download-preview. Update Purpose after archive.

## Requirements

### Requirement: Preview Dialog Display

The system MUST provide an interactive dialog for previewing and customizing configuration downloads.

#### Scenario: Dialog Opens on Menu Click

- **GIVEN** the user clicks "Download Config" in the tray menu
- **WHEN** the menu event is processed
- **THEN** the system MUST display a preview dialog
- **AND** the dialog MUST be modal (blocking other interactions)
- **AND** the dialog MUST show current configuration state

#### Scenario: Dialog Shows Port Selection

- **GIVEN** the preview dialog is open
- **WHEN** the dialog is displayed
- **THEN** the dialog MUST show a list of all configured ports
- **AND** each port MUST display its port number, role, and enabled status
- **AND** each port MUST have a checkbox for selection
- **AND** all ports MUST be selected by default

#### Scenario: Dialog Shows Format Options

- **GIVEN** the preview dialog is open
- **WHEN** the dialog is displayed
- **THEN** the dialog MUST show format selection options
- **AND** the options MUST include: JSON, YAML, Markdown
- **AND** JSON MUST be selected by default

#### Scenario: Dialog Shows Save Location

- **GIVEN** the preview dialog is open
- **WHEN** the dialog is displayed
- **THEN** the dialog MUST show the current save location
- **AND** the default location MUST be ~/Downloads
- **AND** the dialog MUST provide a way to change the location

#### Scenario: Dialog Shows Action Buttons

- **GIVEN** the preview dialog is open
- **WHEN** the dialog is displayed
- **THEN** the dialog MUST show a "Download" button
- **AND** the dialog MUST show a "Cancel" button
- **AND** both buttons MUST be clearly labeled

### Requirement: Port Selection

Users MUST be able to select which ports to include in the downloaded configuration.

#### Scenario: Select Individual Ports

- **GIVEN** the preview dialog is open
- **WHEN** the user toggles a port checkbox
- **THEN** the port's selection state MUST change
- **AND** the preview MUST update to reflect the change

#### Scenario: Select All Ports

- **GIVEN** the preview dialog is open
- **WHEN** all ports are selected
- **THEN** the configuration MUST include all enabled ports
- **AND** disabled ports MUST be excluded

#### Scenario: Select Subset of Ports

- **GIVEN** the preview dialog is open
- **WHEN** the user selects only some ports
- **THEN** the configuration MUST include only selected ports
- **AND** unselected ports MUST be excluded from the configuration

#### Scenario: Validation - At Least One Port Required

- **GIVEN** the preview dialog is open
- **WHEN** the user attempts to download with no ports selected
- **THEN** the system MUST show an error message
- **AND** the error MUST state "Please select at least one port"
- **AND** the download MUST NOT proceed

### Requirement: Format Selection

Users MUST be able to choose the output format for the configuration file.

#### Scenario: Select JSON Format

- **GIVEN** the preview dialog is open
- **WHEN** the user selects JSON format
- **THEN** the configuration MUST be generated as JSON
- **AND** the file extension MUST be .json
- **AND** the preview MUST show JSON-formatted content

#### Scenario: Select YAML Format

- **GIVEN** the preview dialog is open
- **WHEN** the user selects YAML format
- **THEN** the configuration MUST be generated as YAML
- **AND** the file extension MUST be .yaml
- **AND** the preview MUST show YAML-formatted content

#### Scenario: Select Markdown Format

- **GIVEN** the preview dialog is open
- **WHEN** the user selects Markdown format
- **THEN** the configuration MUST be generated as Markdown
- **AND** the file extension MUST be .md
- **AND** the preview MUST show Markdown-formatted content

### Requirement: Custom Save Location

Users MUST be able to choose where to save the configuration file.

#### Scenario: Choose Custom Location

- **GIVEN** the preview dialog is open
- **WHEN** the user clicks "Choose Location"
- **THEN** the system MUST open a native folder picker dialog
- **AND** the user MUST be able to select any writable directory
- **AND** the selected path MUST be displayed in the preview dialog

#### Scenario: Validate Write Permissions

- **GIVEN** the user selects a custom save location
- **WHEN** the location is selected
- **THEN** the system MUST verify write permissions
- **AND** if permissions are denied, show an error message
- **AND** the error MUST suggest selecting a different location

#### Scenario: Use Default Location

- **GIVEN** the preview dialog is open
- **WHEN** the user does not change the save location
- **THEN** the file MUST be saved to ~/Downloads
- **AND** the filename MUST follow the pattern: chaseai-config-YYYYMMDD-HHMMSS.{ext}

### Requirement: Configuration Preview

The dialog MUST show a preview of the configuration content before download.

#### Scenario: Preview Shows Generated Content

- **GIVEN** the preview dialog is open
- **WHEN** the preview is displayed
- **THEN** the preview MUST show the actual configuration content
- **AND** the content MUST match what will be saved
- **AND** the content MUST be formatted according to selected format

#### Scenario: Preview Updates on Selection Change

- **GIVEN** the preview dialog is open
- **WHEN** the user changes port selection
- **THEN** the preview MUST update to reflect the new selection
- **AND** the update MUST happen immediately

#### Scenario: Preview Updates on Format Change

- **GIVEN** the preview dialog is open
- **WHEN** the user changes the format
- **THEN** the preview MUST update to show the new format
- **AND** the content MUST be reformatted appropriately

#### Scenario: Preview is Scrollable

- **GIVEN** the preview dialog is open
- **WHEN** the configuration content is longer than the preview area
- **THEN** the preview MUST be scrollable
- **AND** the user MUST be able to view all content

### Requirement: Download Action

The download action MUST respect all user selections and save the file correctly.

#### Scenario: Successful Download

- **GIVEN** the user has made selections in the preview dialog
- **WHEN** the user clicks "Download"
- **THEN** the system MUST generate the configuration with selected ports
- **AND** the system MUST format the configuration in the selected format
- **AND** the system MUST save the file to the selected location
- **AND** the system MUST show a success notification
- **AND** the dialog MUST close

#### Scenario: Download with Custom Filename

- **GIVEN** the user has selected a save location and format
- **WHEN** the file is saved
- **THEN** the filename MUST include a timestamp
- **AND** the filename MUST have the correct extension for the format
- **AND** the filename MUST be unique (no overwrites)

#### Scenario: Download Error Handling

- **GIVEN** the user clicks "Download"
- **WHEN** an error occurs during file save
- **THEN** the system MUST show an error notification
- **AND** the error MUST include details about what went wrong
- **AND** the dialog MUST remain open
- **AND** the user MUST be able to retry or cancel

### Requirement: Cancel Action

Users MUST be able to cancel the download operation at any time.

#### Scenario: Cancel Before Download

- **GIVEN** the preview dialog is open
- **WHEN** the user clicks "Cancel"
- **THEN** the dialog MUST close
- **AND** no file MUST be saved
- **AND** no notification MUST be shown

#### Scenario: Cancel During Selection

- **GIVEN** the user is making selections in the dialog
- **WHEN** the user clicks "Cancel"
- **THEN** all selections MUST be discarded
- **AND** the dialog MUST close immediately

### Requirement: Backward Compatibility

The enhanced download feature MUST maintain compatibility with existing functionality.

#### Scenario: Programmatic Download Still Works

- **GIVEN** the `download_config_to()` method is called directly
- **WHEN** the method is invoked with a path
- **THEN** the configuration MUST be saved without showing a dialog
- **AND** all ports MUST be included
- **AND** JSON format MUST be used

#### Scenario: HTTP Endpoint Unchanged

- **GIVEN** the `/config` HTTP endpoint exists
- **WHEN** a client requests the endpoint
- **THEN** the endpoint MUST return configuration as before
- **AND** the endpoint MUST NOT be affected by dialog changes
