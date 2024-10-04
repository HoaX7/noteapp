# The Noteapp
- Built with `Tauri` and `rust`.
- We use `tiptap` editor.

# Crash Logs
- Write logs to the OS specific logs directory.
```Platform-specific
Platform	Value	Example
Linux	{configDir}/{bundleIdentifier}	/home/alice/.config/com.tauri.dev
macOS	{homeDir}/Library/Logs/{bundleIdentifier}	/Users/Alice/Library/Logs/com.tauri.dev
Windows	{configDir}/{bundleIdentifier}```