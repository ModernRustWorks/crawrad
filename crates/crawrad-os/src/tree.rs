//! **A process and everything it starts, ended together.**
//!
//! Killing a process on Windows kills that one process: its children live on
//! with a dead parent. rust-analyzer starts `cargo check` (flycheck), `cargo
//! check` starts `rustc`; killing rust-analyzer left them **orphaned**, holding
//! the project's build lock and the pipes the IDE was reading.
//!
//! The Windows answer is a *job object*: a process assigned to a job stays in
//! it, and so does every process it starts afterwards. Ending the job ends
//! all of them. The job here is also *kill on close*: if the IDE dies without
//! calling [`ProcessTree::end`], Windows ends the tree when the last handle
//! closes — the IDE's own exit closes it.
//!
//! On other platforms the tree is only the process (`end` kills it); their
//! orphans are a separate matter.

use crawrad_note::note;
use std::process::Child;

#[note("tree-processtree")]
#[derive(Debug)]
pub struct ProcessTree {
    /// The job handle, as an integer so the type stays `Send + Sync`.
    #[cfg(windows)]
    job: isize,
    #[cfg(not(windows))]
    pid: u32,
}

#[cfg(windows)]
mod win {
    pub use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    pub use windows_sys::Win32::System::JobObjects::{
        AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
        JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
        JobObjectBasicAccountingInformation, JobObjectExtendedLimitInformation,
        QueryInformationJobObject, SetInformationJobObject, TerminateJobObject,
    };
}

impl ProcessTree {
    #[note("tree-processtree-adopt")]
    #[cfg(target_os = "windows")]
    pub fn adopt(child: &Child) -> std::io::Result<Self> {
        use std::os::windows::io::AsRawHandle;
        use win::*;
        // SAFETY: no attributes and no name: an anonymous job owned by us.
        let job = unsafe { CreateJobObjectW(std::ptr::null(), std::ptr::null()) };
        if job.is_null() {
            return Err(std::io::Error::last_os_error());
        }
        let tree = Self { job: job as isize };
        // SAFETY: zeroed is a valid value of this plain C struct.
        let mut limits: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = unsafe { std::mem::zeroed() };
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        // SAFETY: `job` is ours and open; the pointer and size describe
        // `limits`, which outlives the call.
        let set = unsafe {
            SetInformationJobObject(
                job,
                JobObjectExtendedLimitInformation,
                (&raw const limits).cast(),
                std::mem::size_of_val(&limits) as u32,
            )
        };
        // SAFETY: `job` is ours; the process handle belongs to `child`, alive
        // for the call.
        if set == 0
            || unsafe { AssignProcessToJobObject(job, child.as_raw_handle() as HANDLE) } == 0
        {
            return Err(std::io::Error::last_os_error());
        }
        Ok(tree)
    }

    #[note("tree-processtree-adopt-2")]
    #[cfg(not(target_os = "windows"))]
    pub fn adopt(child: &Child) -> std::io::Result<Self> {
        Ok(Self { pid: child.id() })
    }

    #[note("tree-processtree-alive")]
    #[cfg(target_os = "windows")]
    pub fn alive(&self) -> u32 {
        use win::*;
        // SAFETY: zeroed is a valid value of this plain C struct.
        let mut info: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION = unsafe { std::mem::zeroed() };
        // SAFETY: the job is open while `self` lives; pointer and size
        // describe `info`.
        let ok = unsafe {
            QueryInformationJobObject(
                self.job as HANDLE,
                JobObjectBasicAccountingInformation,
                (&raw mut info).cast(),
                std::mem::size_of_val(&info) as u32,
                std::ptr::null_mut(),
            )
        };
        if ok == 0 { 0 } else { info.ActiveProcesses }
    }

    #[note("tree-processtree-alive-2")]
    #[cfg(not(target_os = "windows"))]
    pub fn alive(&self) -> u32 {
        u32::from(std::path::Path::new(&format!("/proc/{}", self.pid)).exists())
    }

    #[note("tree-processtree-end")]
    #[cfg(target_os = "windows")]
    pub fn end(&self) {
        // SAFETY: the job is open while `self` lives.
        unsafe { win::TerminateJobObject(self.job as win::HANDLE, 1) };
    }

    #[note("tree-processtree-end-2")]
    #[cfg(not(target_os = "windows"))]
    pub fn end(&self) {
        let _ = crate::process::no_console("kill")
            .args(["-9", &self.pid.to_string()])
            .status();
    }
}

#[cfg(windows)]
impl Drop for ProcessTree {
    fn drop(&mut self) {
        // SAFETY: the handle is ours and closed once. Kill-on-close ends
        // whatever is still in the tree.
        unsafe { win::CloseHandle(self.job as win::HANDLE) };
    }
}

#[cfg(test)]
#[path = "test/tree/test.rs"]
mod tests;
