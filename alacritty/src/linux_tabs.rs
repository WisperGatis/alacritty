//! Linux tabbing support for Alacritty.
//!
//! This module provides implementations for creating tabs within Alacritty
//! on Linux desktop environments.

use std::env;
use std::process::Command;

/// Detect the current desktop environment.
pub fn detect_desktop_environment() -> Option<DesktopEnvironment> {
    // Check for environment variables that indicate the desktop environment
    let desktop = env::var("XDG_CURRENT_DESKTOP").ok()?;
    
    if desktop.contains("GNOME") {
        Some(DesktopEnvironment::GNOME)
    } else if desktop.contains("KDE") {
        Some(DesktopEnvironment::KDE)
    } else {
        None
    }
}

/// Desktop environments with tabbing support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopEnvironment {
    GNOME,
    KDE,
}

impl DesktopEnvironment {
    /// Create a new tab within Alacritty.
    pub fn create_tab(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            DesktopEnvironment::GNOME => {
                // Try to create a new tab using gdbus for GNOME
                let output = Command::new("gdbus")
                    .args([
                        "call",
                        "--session",
                        "--dest", "org.gnome.Shell",
                        "--object-path", "/org/gnome/Shell",
                        "--method", "org.gnome.Shell.Eval",
                        "global.display.focus_window && global.display.focus_window.new_tab()"
                    ])
                    .output()?;
                
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("GNOME tab creation failed: {}", String::from_utf8_lossy(&output.stderr)).into())
                }
            },
            DesktopEnvironment::KDE => {
                // Try to create a new tab using qdbus for KDE
                let output = Command::new("qdbus")
                    .args([
                        "org.kde.konsole",
                        "/Konsole",
                        "org.kde.KMainWindow.newTab"
                    ])
                    .output()?;
                
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("KDE tab creation failed: {}", String::from_utf8_lossy(&output.stderr)).into())
                }
            }
        }
    }
    
    /// Select the next tab.
    pub fn select_next_tab(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            DesktopEnvironment::GNOME => {
                // Try to select next tab using gdbus for GNOME
                let output = Command::new("gdbus")
                    .args([
                        "call",
                        "--session",
                        "--dest", "org.gnome.Shell",
                        "--object-path", "/org/gnome/Shell",
                        "--method", "org.gnome.Shell.Eval",
                        "global.display.focus_window && global.display.focus_window.switch_to_next_tab()"
                    ])
                    .output()?;
                
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("GNOME next tab selection failed: {}", String::from_utf8_lossy(&output.stderr)).into())
                }
            },
            DesktopEnvironment::KDE => {
                // Try to select next tab using qdbus for KDE
                let output = Command::new("qdbus")
                    .args([
                        "org.kde.konsole",
                        "/Konsole",
                        "org.kde.KMainWindow.nextTab"
                    ])
                    .output()?;
                
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("KDE next tab selection failed: {}", String::from_utf8_lossy(&output.stderr)).into())
                }
            }
        }
    }
    
    /// Select the previous tab.
    pub fn select_previous_tab(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            DesktopEnvironment::GNOME => {
                // Try to select previous tab using gdbus for GNOME
                let output = Command::new("gdbus")
                    .args([
                        "call",
                        "--session",
                        "--dest", "org.gnome.Shell",
                        "--object-path", "/org/gnome/Shell",
                        "--method", "org.gnome.Shell.Eval",
                        "global.display.focus_window && global.display.focus_window.switch_to_previous_tab()"
                    ])
                    .output()?;
                
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("GNOME previous tab selection failed: {}", String::from_utf8_lossy(&output.stderr)).into())
                }
            },
            DesktopEnvironment::KDE => {
                // Try to select previous tab using qdbus for KDE
                let output = Command::new("qdbus")
                    .args([
                        "org.kde.konsole",
                        "/Konsole",
                        "org.kde.KMainWindow.previousTab"
                    ])
                    .output()?;
                
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("KDE previous tab selection failed: {}", String::from_utf8_lossy(&output.stderr)).into())
                }
            }
        }
    }
    
    /// Check if the desktop environment tools are available.
    pub fn is_available(&self) -> bool {
        match self {
            DesktopEnvironment::GNOME => {
                // Check if gdbus is available
                Command::new("which")
                    .arg("gdbus")
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            },
            DesktopEnvironment::KDE => {
                // Check if qdbus is available
                Command::new("which")
                    .arg("qdbus")
                    .output()
                    .map(|output| output.status.success())
                    .unwrap_or(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_desktop_environment() {
        // This test will only pass if run in the appropriate environment
        // We're just checking that it doesn't panic
        let _ = detect_desktop_environment();
    }
}