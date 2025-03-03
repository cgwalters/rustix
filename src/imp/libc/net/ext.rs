use super::super::c;

/// The windows `sockaddr_in6` type is a union with accessor functions which
/// are not `const fn`. Define our own layout-compatible version so that we
/// can transmute in and out of it.
#[cfg(windows)]
#[repr(C)]
struct sockaddr_in6 {
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: c::in6_addr,
    sin6_scope_id: u32,
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in_addr_s_addr(addr: c::in_addr) -> u32 {
    addr.s_addr
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) const fn in_addr_s_addr(addr: c::in_addr) -> u32 {
    // This should be `*addr.S_un.S_addr()`, except that isn't a `const fn`.
    unsafe { core::mem::transmute(addr) }
}

// TODO: With Rust 1.55, we can use the above `in_addr_s_addr` definition
// that uses a const-fn transmute.
#[cfg(feature = "std")]
#[cfg(windows)]
#[inline]
pub(crate) fn in_addr_s_addr(addr: c::in_addr) -> u32 {
    // This should be `*addr.S_un.S_addr()`, except that isn't a `const fn`.
    unsafe { core::mem::transmute(addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> c::in_addr {
    c::in_addr { s_addr }
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) const fn in_addr_new(s_addr: u32) -> c::in_addr {
    unsafe { core::mem::transmute(s_addr) }
}

// TODO: With Rust 1.55, we can use the above `in_addr_new` definition
// that uses a const-fn transmute.
#[cfg(feature = "std")]
#[cfg(windows)]
#[inline]
pub(crate) fn in_addr_new(s_addr: u32) -> c::in_addr {
    unsafe { core::mem::transmute(s_addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: c::in6_addr) -> [u8; 16] {
    addr.s6_addr
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) const fn in6_addr_s6_addr(addr: c::in6_addr) -> [u8; 16] {
    unsafe { core::mem::transmute(addr) }
}

// TODO: With Rust 1.55, we can use the above `in6_addr_s6_addr` definition
// that uses a const-fn transmute.
#[cfg(feature = "std")]
#[cfg(windows)]
#[inline]
pub(crate) fn in6_addr_s6_addr(addr: c::in6_addr) -> [u8; 16] {
    unsafe { core::mem::transmute(addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> c::in6_addr {
    c::in6_addr { s6_addr }
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) const fn in6_addr_new(s6_addr: [u8; 16]) -> c::in6_addr {
    unsafe { core::mem::transmute(s6_addr) }
}

// TODO: With Rust 1.55, we can use the above `in6_addr_new` definition
// that uses a const-fn transmute.
#[cfg(feature = "std")]
#[cfg(windows)]
#[inline]
pub(crate) fn in6_addr_new(s6_addr: [u8; 16]) -> c::in6_addr {
    unsafe { core::mem::transmute(s6_addr) }
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: c::sockaddr_in6) -> u32 {
    addr.sin6_scope_id
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) const fn sockaddr_in6_sin6_scope_id(addr: c::sockaddr_in6) -> u32 {
    let addr: sockaddr_in6 = unsafe { core::mem::transmute(addr) };
    addr.sin6_scope_id
}

// TODO: With Rust 1.55, we can use the above `sockaddr_in6_sin6_scope_id`
// definition that uses a const-fn transmute.
#[cfg(feature = "std")]
#[cfg(windows)]
#[inline]
pub(crate) fn sockaddr_in6_sin6_scope_id(addr: c::sockaddr_in6) -> u32 {
    let addr: sockaddr_in6 = unsafe { core::mem::transmute(addr) };
    addr.sin6_scope_id
}

#[cfg(not(feature = "std"))]
#[cfg(not(windows))]
#[inline]
pub(crate) fn sockaddr_in6_sin6_scope_id_mut(addr: &mut c::sockaddr_in6) -> &mut u32 {
    &mut addr.sin6_scope_id
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) fn sockaddr_in6_sin6_scope_id_mut(addr: &mut c::sockaddr_in6) -> &mut u32 {
    let addr: &mut sockaddr_in6 = unsafe { core::mem::transmute(addr) };
    &mut addr.sin6_scope_id
}

#[cfg(not(windows))]
#[inline]
pub(crate) const fn sockaddr_in6_new(
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "ios",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    sin6_len: u8,
    sin6_family: c::sa_family_t,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: c::in6_addr,
    sin6_scope_id: u32,
) -> c::sockaddr_in6 {
    c::sockaddr_in6 {
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        sin6_len,
        sin6_family,
        sin6_port,
        sin6_flowinfo,
        sin6_addr,
        sin6_scope_id,
    }
}

#[cfg(not(feature = "std"))]
#[cfg(windows)]
#[inline]
pub(crate) const fn sockaddr_in6_new(
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: c::in6_addr,
    sin6_scope_id: u32,
) -> c::sockaddr_in6 {
    let addr = sockaddr_in6 {
        sin6_family,
        sin6_port,
        sin6_flowinfo,
        sin6_addr,
        sin6_scope_id,
    };
    unsafe { core::mem::transmute(addr) }
}

// TODO: With Rust 1.55, we can use the above `sockaddr_in6_new`
// definition that uses a const-fn transmute.
#[cfg(feature = "std")]
#[cfg(windows)]
#[inline]
pub(crate) fn sockaddr_in6_new(
    sin6_family: u16,
    sin6_port: u16,
    sin6_flowinfo: u32,
    sin6_addr: c::in6_addr,
    sin6_scope_id: u32,
) -> c::sockaddr_in6 {
    let addr = sockaddr_in6 {
        sin6_family,
        sin6_port,
        sin6_flowinfo,
        sin6_addr,
        sin6_scope_id,
    };
    unsafe { core::mem::transmute(addr) }
}
