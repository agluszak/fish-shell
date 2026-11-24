//! Safe wrappers around various libc functions that we might want to reuse across modules.

use std::os::fd::BorrowedFd;

pub fn isatty(fd: i32) -> bool {
    // This returns false if the fd is valid but not a tty, or is invalid.
    // No place we currently call it really cares about the difference.
    // A BorrowedFd cannot be constructed from a negative descriptor, so guard it here.
    if fd < 0 {
        return false;
    }
    nix::unistd::isatty(unsafe { BorrowedFd::borrow_raw(fd) }).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::isatty;

    #[test]
    fn test_isatty_invalid_fd_is_false() {
        assert!(!isatty(-1));
    }
}
