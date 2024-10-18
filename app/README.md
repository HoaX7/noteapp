# The Noteapp
- Built with `Tauri` and `rust`.
- We use `tiptap` editor.

# Crash Logs
- Write logs to the OS specific logs directory.
```Platform-specific
Platform	Value	Example
Linux	{configDir}/{bundleIdentifier}	/home/alice/.config/com.scribe.app
macOS	{homeDir}/Library/Logs/{bundleIdentifier}	/Users/Alice/Library/Logs/com.scribe.app
Windows	{configDir}/{bundleIdentifier} C:\Users\Alice\AppData\Roaming\com.scribe.app```