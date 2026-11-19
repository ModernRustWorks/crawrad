//! Inter-process communication (IPC) for the in-IDE MCP server and stdio bridge.
//!
//! Provides named pipes on Windows (`\\.\pipe\crawrad-mcp-<instance_id>`)
//! and Unix domain sockets on Unix platforms.

use std::io::{self, Read, Write};

#[cfg(target_os = "windows")]
use std::os::windows::io::{FromRawHandle, RawHandle};

#[cfg(target_os = "windows")]
use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_PIPE_CONNECTED, GetLastError, INVALID_HANDLE_VALUE,
};
#[cfg(target_os = "windows")]
use windows_sys::Win32::Storage::FileSystem::{FILE_FLAG_FIRST_PIPE_INSTANCE, PIPE_ACCESS_DUPLEX};
#[cfg(target_os = "windows")]
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE,
    PIPE_UNLIMITED_INSTANCES, PIPE_WAIT,
};

use crawrad_note::note;
#[cfg(unix)]
use std::os::unix::net::{UnixListener, UnixStream};
#[cfg(unix)]
use std::path::PathBuf;

#[note("ipc-ipcstream")]
#[derive(Debug)]
pub struct IpcStream {
    #[cfg(target_os = "windows")]
    inner: std::fs::File,
    #[cfg(unix)]
    inner: UnixStream,
}

impl IpcStream {
    #[note("ipc-ipcstream-connect")]
    pub fn connect(instance_id: &str) -> io::Result<Self> {
        #[cfg(target_os = "windows")]
        {
            let pipe_name = format!(r"\\.\pipe\crawrad-mcp-{}", instance_id);
            let mut retries = 0;
            loop {
                match std::fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&pipe_name)
                {
                    Ok(file) => return Ok(Self { inner: file }),
                    Err(e)
                        if (e.kind() == io::ErrorKind::NotFound
                            || e.raw_os_error() == Some(231))
                            && retries < 15 =>
                    {
                        retries += 1;
                        std::thread::sleep(std::time::Duration::from_millis(20));
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        #[cfg(unix)]
        {
            let path = socket_path_for_instance(instance_id);
            let mut retries = 0;
            loop {
                match UnixStream::connect(&path) {
                    Ok(stream) => return Ok(Self { inner: stream }),
                    Err(e) if e.kind() == io::ErrorKind::NotFound && retries < 15 => {
                        retries += 1;
                        std::thread::sleep(std::time::Duration::from_millis(20));
                    }
                    Err(e) => return Err(e),
                }
            }
        }
    }

    #[note("ipc-ipcstream-try-clone")]
    pub fn try_clone(&self) -> io::Result<Self> {
        Ok(Self {
            inner: self.inner.try_clone()?,
        })
    }
}

impl Read for IpcStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for IpcStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::io::AsRawHandle;
            let handle = self.inner.as_raw_handle() as windows_sys::Win32::Foundation::HANDLE;
            unsafe {
                windows_sys::Win32::Storage::FileSystem::FlushFileBuffers(handle);
            }
        }
        self.inner.flush()
    }
}

#[note("ipc-ipclistener")]
pub struct IpcListener {
    instance_id: String,
    #[cfg(target_os = "windows")]
    pipe_name_wide: Vec<u16>,
    #[cfg(target_os = "windows")]
    initial_handle: std::sync::Mutex<Option<usize>>,
    #[cfg(unix)]
    listener: UnixListener,
    #[cfg(unix)]
    socket_path: PathBuf,
}

impl IpcListener {
    #[note("ipc-ipclistener-bind")]
    pub fn bind(instance_id: &str) -> io::Result<Self> {
        #[cfg(target_os = "windows")]
        {
            let pipe_name = format!(r"\\.\pipe\crawrad-mcp-{}", instance_id);
            let mut pipe_name_wide: Vec<u16> = pipe_name.encode_utf16().collect();
            pipe_name_wide.push(0);

            let handle = unsafe {
                CreateNamedPipeW(
                    pipe_name_wide.as_ptr(),
                    PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE,
                    PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                    PIPE_UNLIMITED_INSTANCES,
                    65536,
                    65536,
                    0,
                    std::ptr::null(),
                )
            };

            if handle == INVALID_HANDLE_VALUE {
                return Err(io::Error::last_os_error());
            }

            Ok(Self {
                instance_id: instance_id.to_string(),
                pipe_name_wide,
                initial_handle: std::sync::Mutex::new(Some(handle as usize)),
            })
        }

        #[cfg(unix)]
        {
            let socket_path = socket_path_for_instance(instance_id);
            if socket_path.exists() {
                let _ = std::fs::remove_file(&socket_path);
            }
            let listener = UnixListener::bind(&socket_path)?;
            Ok(Self {
                instance_id: instance_id.to_string(),
                listener,
                socket_path,
            })
        }
    }

    #[note("ipc-ipclistener-instance-id")]
    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }

    #[note("ipc-ipclistener-accept")]
    pub fn accept(&self) -> io::Result<IpcStream> {
        #[cfg(target_os = "windows")]
        {
            let handle = {
                let mut guard = self.initial_handle.lock().unwrap();
                if let Some(h) = guard.take() {
                    h as windows_sys::Win32::Foundation::HANDLE
                } else {
                    unsafe {
                        CreateNamedPipeW(
                            self.pipe_name_wide.as_ptr(),
                            PIPE_ACCESS_DUPLEX,
                            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                            PIPE_UNLIMITED_INSTANCES,
                            65536,
                            65536,
                            0,
                            std::ptr::null(),
                        )
                    }
                }
            };

            if handle == INVALID_HANDLE_VALUE {
                return Err(io::Error::last_os_error());
            }

            let connect_res = unsafe { ConnectNamedPipe(handle, std::ptr::null_mut()) };
            if connect_res == 0 {
                let err = unsafe { GetLastError() };
                if err != ERROR_PIPE_CONNECTED {
                    unsafe { CloseHandle(handle) };
                    return Err(io::Error::from_raw_os_error(err as i32));
                }
            }

            let file = unsafe { std::fs::File::from_raw_handle(handle as RawHandle) };
            Ok(IpcStream { inner: file })
        }

        #[cfg(unix)]
        {
            let (stream, _) = self.listener.accept()?;
            Ok(IpcStream { inner: stream })
        }
    }
}

#[cfg(unix)]
fn socket_path_for_instance(instance_id: &str) -> PathBuf {
    std::env::temp_dir().join(format!("crawrad-mcp-{}.sock", instance_id))
}

impl Drop for IpcListener {
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        {
            if let Ok(mut guard) = self.initial_handle.lock()
                && let Some(handle) = guard.take()
            {
                unsafe { CloseHandle(handle as windows_sys::Win32::Foundation::HANDLE) };
            }
        }

        #[cfg(unix)]
        {
            let _ = std::fs::remove_file(&self.socket_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_roundtrip() {
        let id = format!("test-{}", std::process::id());
        let listener = IpcListener::bind(&id).expect("bind listener");

        let handle = std::thread::spawn(move || {
            let mut stream = listener.accept().expect("accept connection");
            let mut buf = [0u8; 5];
            stream.read_exact(&mut buf).expect("read from stream");
            assert_eq!(&buf, b"ping\n");
            stream.write_all(b"pong\n").expect("write to stream");
            stream.flush().expect("flush stream");
        });

        // Connect from client
        let mut client = IpcStream::connect(&id).expect("connect to listener");
        client.write_all(b"ping\n").expect("client write");
        client.flush().expect("client flush");

        let mut resp = [0u8; 5];
        client.read_exact(&mut resp).expect("client read");
        assert_eq!(&resp, b"pong\n");

        handle.join().expect("join server thread");
    }

    #[test]
    fn test_two_different_instances_do_not_collide() {
        let id1 = format!("inst1-{}", std::process::id());
        let id2 = format!("inst2-{}", std::process::id());

        let l1 = IpcListener::bind(&id1).expect("bind l1");
        let l2 = IpcListener::bind(&id2).expect("bind l2");

        assert_ne!(l1.instance_id(), l2.instance_id());

        // Spawn l1 echo thread
        let h1 = std::thread::spawn(move || {
            let mut s = l1.accept().expect("accept l1");
            let mut b = [0u8; 2];
            s.read_exact(&mut b).expect("read l1");
            assert_eq!(&b, b"1\n");
            s.write_all(b"R1\n").expect("write l1");
            s.flush().expect("flush l1");
        });

        // Spawn l2 echo thread
        let h2 = std::thread::spawn(move || {
            let mut s = l2.accept().expect("accept l2");
            let mut b = [0u8; 2];
            s.read_exact(&mut b).expect("read l2");
            assert_eq!(&b, b"2\n");
            s.write_all(b"R2\n").expect("write l2");
            s.flush().expect("flush l2");
        });

        let mut c1 = IpcStream::connect(&id1).expect("connect c1");
        c1.write_all(b"1\n").expect("write c1");
        c1.flush().expect("flush c1");
        let mut r1 = [0u8; 3];
        c1.read_exact(&mut r1).expect("read c1");
        assert_eq!(&r1, b"R1\n");

        let mut c2 = IpcStream::connect(&id2).expect("connect c2");
        c2.write_all(b"2\n").expect("write c2");
        c2.flush().expect("flush c2");
        let mut r2 = [0u8; 3];
        c2.read_exact(&mut r2).expect("read c2");
        assert_eq!(&r2, b"R2\n");

        h1.join().expect("join h1");
        h2.join().expect("join h2");
    }
}
