use crate::CopyOptions;
use crate::CreateDirectoryOptions;
use crate::ExecutorFileSystem;
use crate::ExecutorFileSystemFuture;
use crate::FileMetadata;
use crate::FileSystemReadStream;
use crate::FileSystemSandboxContext;
use crate::ReadDirectoryEntry;
use crate::RemoveOptions;
use codex_utils_path_uri::PathUri;
use std::fs;
use std::io;

pub struct LocalFileSystem;

pub static LOCAL_FS: LocalFileSystem = LocalFileSystem;

impl ExecutorFileSystem for LocalFileSystem {
    fn canonicalize<'a>(
        &'a self,
        path: &'a PathUri,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, PathUri> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            let canonical = native.canonicalize()?;
            PathUri::from_abs_path(&canonical)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))
        })
    }

    fn read_file<'a>(
        &'a self,
        path: &'a PathUri,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<u8>> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            fs::read(native)
        })
    }

    fn read_file_stream<'a>(
        &'a self,
        path: &'a PathUri,
        sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileSystemReadStream> {
        Box::pin(async move {
            let bytes = self.read_file(path, sandbox).await?;
            Ok(FileSystemReadStream::new(futures::stream::once(async move {
                Ok(bytes::Bytes::from(bytes))
            })))
        })
    }

    fn write_file<'a>(
        &'a self,
        path: &'a PathUri,
        contents: Vec<u8>,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            if let Some(parent) = native.parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::write(native, contents)
        })
    }

    fn create_directory<'a>(
        &'a self,
        path: &'a PathUri,
        options: CreateDirectoryOptions,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            if options.recursive {
                fs::create_dir_all(native)
            } else {
                fs::create_dir(native)
            }
        })
    }

    fn get_metadata<'a>(
        &'a self,
        path: &'a PathUri,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, FileMetadata> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            let meta = fs::metadata(native)?;
            Ok(FileMetadata {
                is_directory: meta.is_dir(),
                is_file: meta.is_file(),
                is_symlink: meta.file_type().is_symlink(),
                size: meta.len(),
                created_at_ms: 0,
                modified_at_ms: 0,
            })
        })
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a PathUri,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, Vec<ReadDirectoryEntry>> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            let rd = fs::read_dir(native)?;
            let mut entries = Vec::new();
            for entry in rd.flatten() {
                let ft = entry.file_type().ok();
                entries.push(ReadDirectoryEntry {
                    file_name: entry.file_name().to_string_lossy().to_string(),
                    is_directory: ft.as_ref().map_or(false, |t| t.is_dir()),
                    is_file: ft.as_ref().map_or(false, |t| t.is_file()),
                });
            }
            Ok(entries)
        })
    }

    fn remove<'a>(
        &'a self,
        path: &'a PathUri,
        options: RemoveOptions,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(async move {
            let native = path.to_abs_path()?;
            if options.recursive {
                fs::remove_dir_all(native)
            } else {
                fs::remove_file(native)
            }
        })
    }

    fn copy<'a>(
        &'a self,
        source_path: &'a PathUri,
        destination_path: &'a PathUri,
        _copy_options: CopyOptions,
        _sandbox: Option<&'a FileSystemSandboxContext>,
    ) -> ExecutorFileSystemFuture<'a, ()> {
        Box::pin(async move {
            let src = source_path.to_abs_path()?;
            let dst = destination_path.to_abs_path()?;
            fs::copy(src, dst).map(|_| ())
        })
    }
}
