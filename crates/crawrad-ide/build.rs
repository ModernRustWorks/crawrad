fn main() {
    // comctl32 v6 + PerMonitorV2: without this, native controls look like
    // Windows 95 and DPI is wrong.
    // Only makes sense on Windows — on other platforms there is no .rc/.res
    #[cfg(target_os = "windows")]
    {
        // THIS RESULT USED TO BE DISCARDED WITH `let _ =`, AND IT HIDES TWO THINGS
        // AT ONCE: `app.rc` carries the comctl32 v6 + PerMonitorV2 manifest AND the
        // icon (`1 ICON "crawrad.ico"`). A machine without `rc.exe` therefore
        // produced a GREEN build whose executable had Windows 95 controls, wrong
        // DPI, and the generic Windows icon — while a developer's own machine, which
        // has the SDK, showed the crab. "The installed app's icon is not the IDE's"
        // is exactly the shape that failure takes, and nothing in the build said a
        // word.
        //
        // The crate names the right call `manifest_required`, because here it is.
        // `NotWindows` is reachable even inside this block: the `cfg` above is the
        // HOST, so cross-compiling from Windows to another target lands here with
        // nothing to embed, which is not a failure.
        match embed_resource::compile("app.rc", embed_resource::NONE) {
            embed_resource::CompilationResult::Ok
            | embed_resource::CompilationResult::NotWindows => {}
            other => panic!(
                "app.rc was not embedded ({other:?}). This build would ship with the \
                 generic Windows icon instead of the crab, and with no comctl32 v6 \
                 manifest, so its controls would look like Windows 95 and its DPI \
                 would be wrong. Install the Windows SDK (it provides rc.exe) or the \
                 MSVC build tools."
            ),
        }
    }
}
