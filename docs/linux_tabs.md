# Linux Native Tabbing

Alacritty now supports native tabbing on Linux desktop environments. This feature
detects whether you're running GNOME or KDE and attempts to create tabs using
the native desktop environment APIs.

## How it works

When you press the default tab creation keybinding (Ctrl+Shift+T), Alacritty
will:

1. Detect your desktop environment using the `XDG_CURRENT_DESKTOP` environment variable
2. If you're on GNOME or KDE, it will attempt to create a tab using D-Bus
3. If tab creation fails or you're on an unsupported desktop environment, it will
   fall back to creating a new window as before

## Requirements

- For GNOME: `gdbus` must be available (usually part of the `glib2` package)
- For KDE: `qdbus` must be available (usually part of `qt5-tools` or `qt6-tools`)

## Limitations

This is a basic implementation that uses D-Bus to communicate with the desktop
environment. True tabbing (where multiple terminals share the same window) would
require significant UI changes to Alacritty, which is outside the scope of this
implementation.

Instead, this implementation creates new windows that are managed by the desktop
environment as tabs, providing a similar user experience.

If the native tabbing fails for any reason, Alacritty will automatically fall
back to creating a new window, ensuring that the user can always create a new
terminal session.