# Alacritty Tab Implementation for Linux

## Summary
Successfully implemented comprehensive in-window tab functionality for Alacritty on Linux with Ctrl+Shift+T shortcut support and full tab management.

## Changes Made

### 1. Key Binding Addition  
- **File**: `alacritty/src/config/bindings.rs`
- **Change**: Added `"t", ModifiersState::CONTROL | ModifiersState::SHIFT; Action::CreateNewTab;` to the `common_keybindings()` function
- **Effect**: Enables Ctrl+Shift+T shortcut on Linux and Windows platforms

### 2. Tab Data Structures (Linux Only)
- **File**: `alacritty/src/window_context.rs`
- **Changes**:
  - Added `TabInfo` struct to store terminal, title, master_fd, and shell_pid per tab
  - Added `TabManager` struct to manage multiple tabs with methods for adding, removing, and switching tabs
  - Modified `WindowContext` to use `TabManager` on Linux instead of single terminal
  - Added accessor methods for cross-platform terminal access

### 3. Tab UI Rendering
- **File**: `alacritty/src/display/mod.rs`
- **Changes**:
  - Added `draw_tab_bar()` method to render tab bar at top of terminal
  - Modified `draw()` method to accept tab manager parameter and render tabs when multiple exist
  - Tab bar shows tab titles with active tab highlighting using different colors

### 4. Input Handling and Action Processing
- **File**: `alacritty/src/input/mod.rs`
- **Changes**:
  - Added Linux-specific tab management methods to `ActionContext` trait
  - Updated all tab selection actions (SelectNextTab, SelectPreviousTab, SelectTab1-9, SelectLastTab) to work on Linux
  - Distinguished between macOS (native), Linux (in-window tabs), and other platforms

### 5. Event System Integration  
- **File**: `alacritty/src/event.rs`
- **Changes**:
  - Added tab manager reference to `ActionContext` struct for Linux
  - Implemented all tab management methods with proper terminal/PTY creation
  - Added tab switching logic with display dirty marking for redraws

## Implementation Details

### Key Architecture
- **macOS**: Uses native window tabbing APIs (unchanged)  
- **Linux**: In-window tab system with tab bar UI and multiple terminal instances
- **Other platforms**: Creates separate windows (unchanged)

### Tab Management (Linux)
```rust
pub struct TabManager {
    pub tabs: Vec<TabInfo>,
    pub active_tab: usize,
}

pub struct TabInfo {
    pub terminal: Arc<FairMutex<Term<EventProxy>>>,
    pub title: String,
    pub master_fd: RawFd,
    pub shell_pid: u32,
}
```

### Key Bindings
```rust
"t",    ModifiersState::CONTROL | ModifiersState::SHIFT;   Action::CreateNewTab;
"w",    ModifiersState::CONTROL | ModifiersState::SHIFT;   Action::CloseTab;
```

### Action Handlers (Linux)
- **Ctrl+Shift+T**: Creates new tab with new terminal instance and PTY
- **Ctrl+Shift+W**: Closes current active tab (closes window if last tab)
- **Tab switching**: Changes active tab, marks display dirty for redraw
- **Tab selection**: Supports Tab1-Tab9 and LastTab actions
- **Mouse clicks**: Click on tab in tab bar to switch to that tab

## Functionality
- **Ctrl+Shift+T**: Creates new in-window tab on Linux
- **Ctrl+Shift+W**: Closes current active tab on Linux
- **Tab Bar**: Displays at top when multiple tabs exist  
- **Tab Switching**: All standard tab navigation shortcuts work
- **Mouse Support**: Click on tabs in tab bar to switch between them
- **Terminal Independence**: Each tab has its own terminal session and shell process
- **Visual Feedback**: Active tab highlighted with different colors

## Testing
- Code compiles successfully: ✅
- Release build works: ✅  
- Tab bar renders when multiple tabs exist: ✅
- Tab switching actions integrated: ✅
- Tab closing with Ctrl+Shift+W implemented: ✅
- Mouse click tab switching implemented: ✅
- Tab switching functionality fixed: ✅
- Command input in new tabs fixed: ✅

## Build Commands
```bash
cargo check      # Quick compile check
cargo build --release   # Full optimized build
```

## Usage
1. Launch Alacritty on Linux
2. Press **Ctrl+Shift+T** to create a new tab
3. Tab bar appears at the top showing "Terminal 1", "Terminal 2", etc.
4. Use tab switching shortcuts or click tabs to navigate between them
5. Press **Ctrl+Shift+W** to close the current active tab
6. Each tab maintains independent terminal session

## Architecture Benefits
- **Performance**: Each tab runs its own shell process with proper PTY handling
- **Independence**: Tabs don't interfere with each other
- **Consistency**: Uses same terminal rendering engine as single-tab mode
- **Extensibility**: Easy to add features like tab renaming, closing, reordering

This implementation provides Linux users with true in-window tabs while maintaining Alacritty's high performance and architectural integrity.

## Recent Fixes (Sept 2025)

### Tab Switching Issues Resolved
- **Fixed ActionContext structure**: Added `tab_manager` field to enable proper tab management on Linux
- **Fixed PTY input routing**: Updated `write_to_pty()` method to write directly to active tab's master_fd on Linux
- **Fixed compilation errors**: Resolved duplicate field declarations and import issues
- **Verified all tab operations**: Tab switching shortcuts, mouse clicks, and new tab creation now work correctly

### UI Layout Issue Fixed (Sept 2025)
- **Fixed tab bar overlay issue**: Tab bar no longer covers terminal content or cursor
- **Implemented coordinate offset system**: Terminal content is vertically offset when tab bar is present
- **Direct coordinate transformation**: Modified SizeInfo with padding_y offset for terminal content rendering
- **Clean parameter handling**: Removed unused tab_manager parameters from handle_update and submit_display_update
- **Eliminated all warnings**: Cleaned up unused variables and parameters

### Technical Details
- **Input routing**: On Linux, keyboard input now properly routes to the active tab's PTY using direct `libc::write()` calls to the master file descriptor
- **Cross-platform compatibility**: Non-Linux platforms continue to use the original notifier-based approach
- **Memory safety**: All PTY operations use proper unsafe blocks with appropriate error handling
- **Coordinate offset system**: Creates separate SizeInfo instances - original for tab bar, offset for terminal content
- **Rendering separation**: Tab bar renders at `Point::new(0, Column(0))` while terminal content uses offset SizeInfo with increased padding_y

The tab implementation is now fully functional with working input, tab switching, proper PTY communication, and correct UI layout with no cursor hiding issues.

### Coordinate Offset Fix (Sept 2025)

The previous space reservation approach using `reserve_lines()` was replaced with a direct coordinate offset system that properly separates tab bar and terminal content rendering:

**Problem**: Tab bar at (0,0) overlapped with terminal content also starting at (0,0)

**Solution**: Create two SizeInfo instances:
- `size_info`: Original coordinates for tab bar and UI elements
- `terminal_size_info`: Offset coordinates for terminal content (padding_y += cell_height)

**Implementation**:
```rust
// Create offset SizeInfo for terminal content when tabs are present on Linux
#[cfg(target_os = "linux")]
let terminal_size_info = if tab_manager.map_or(false, |tm| tm.tabs.len() > 1) {
    let mut offset_size_info = size_info;
    // Offset terminal content by one line height (tab bar height)
    offset_size_info.padding_y += size_info.cell_height();
    offset_size_info
} else {
    size_info
};
```

**Key Changes**:
- `draw_cells()`: Uses `terminal_size_info` instead of `size_info`
- `cursor.rects()`: Uses `terminal_size_info` for proper cursor positioning
- `lines.rects()`: Uses `terminal_size_info` for selection and underlines
- Tab bar rendering: Continues using original `size_info` at absolute (0,0)

This approach directly addresses the coordinate overlap issue without interfering with other UI elements like search bars and message bars.