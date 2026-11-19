fn main() {
    // The app's icon in the .exe (Explorer, taskbar, Alt+Tab): resource 1 (CrawRAD).
    #[cfg(target_os = "windows")]
    {
        let _ = embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
