// SPDX-License-Identifier: BSD-3-Clause

use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

pub fn available_space_for_path(path: &Path) -> std::io::Result<u64> {
    let path_c_string = CString::new(path.as_os_str().as_bytes()).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "path contains an interior NUL byte",
        )
    })?;

    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();

    let result = unsafe { libc::statvfs(path_c_string.as_ptr(), stat.as_mut_ptr()) };

    if result != 0 {
        return Err(std::io::Error::last_os_error());
    }

    let stat = unsafe { stat.assume_init() };

    Ok(stat.f_bavail as u64 * stat.f_frsize as u64)
}
