fn main() {
    // comctl32 v6 + PerMonitorV2: without this, native controls look like
    // Windows 95 and DPI is wrong.
    // Only makes sense on Windows — on other platforms there is no .rc/.res.
    #[cfg(target_os = "windows")]
    {
        let _ = embed_resource::compile("app.rc", embed_resource::NONE);
    }
}
