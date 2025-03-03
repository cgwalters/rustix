use super::super::c;
use super::super::conv::{borrowed_fd, ret, ret_owned_fd, ret_send_recv, send_recv_len};
use super::super::fd::BorrowedFd;
use super::ext::{in6_addr_new, in_addr_new};
#[cfg(not(windows))]
use super::{encode_sockaddr_unix, SocketAddrUnix};
#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
use super::{
    encode_sockaddr_v4, encode_sockaddr_v6, read_sockaddr_os, AcceptFlags, AddressFamily, Protocol,
    RecvFlags, SendFlags, Shutdown, SocketFlags, SocketType,
};
use crate::as_ptr;
use crate::io::{self, OwnedFd};
use crate::net::{SocketAddrAny, SocketAddrV4, SocketAddrV6};
use core::convert::TryInto;
use core::mem::{size_of, MaybeUninit};
#[cfg(not(any(target_os = "redox", target_os = "wasi",)))]
use core::ptr::null_mut;

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn recv(fd: BorrowedFd<'_>, buf: &mut [u8], flags: RecvFlags) -> io::Result<usize> {
    let nrecv = unsafe {
        ret_send_recv(c::recv(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast(),
            send_recv_len(buf.len()),
            flags.bits(),
        ))?
    };
    Ok(nrecv as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn send(fd: BorrowedFd<'_>, buf: &[u8], flags: SendFlags) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_send_recv(c::send(
            borrowed_fd(fd),
            buf.as_ptr().cast(),
            send_recv_len(buf.len()),
            flags.bits(),
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn recvfrom(
    fd: BorrowedFd<'_>,
    buf: &mut [u8],
    flags: RecvFlags,
) -> io::Result<(usize, SocketAddrAny)> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        let nread = ret_send_recv(c::recvfrom(
            borrowed_fd(fd),
            buf.as_mut_ptr().cast(),
            send_recv_len(buf.len()),
            flags.bits(),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok((
            nread as usize,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_v4(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV4,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_send_recv(c::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast(),
            send_recv_len(buf.len()),
            flags.bits(),
            as_ptr(&encode_sockaddr_v4(addr)).cast::<c::sockaddr>(),
            size_of::<SocketAddrV4>() as _,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_v6(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrV6,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_send_recv(c::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast(),
            send_recv_len(buf.len()),
            flags.bits(),
            as_ptr(&encode_sockaddr_v6(addr)).cast::<c::sockaddr>(),
            size_of::<SocketAddrV6>() as _,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub(crate) fn sendto_unix(
    fd: BorrowedFd<'_>,
    buf: &[u8],
    flags: SendFlags,
    addr: &SocketAddrUnix,
) -> io::Result<usize> {
    let nwritten = unsafe {
        ret_send_recv(c::sendto(
            borrowed_fd(fd),
            buf.as_ptr().cast(),
            send_recv_len(buf.len()),
            flags.bits(),
            as_ptr(&encode_sockaddr_unix(addr)).cast::<c::sockaddr>(),
            size_of::<SocketAddrUnix>() as _,
        ))?
    };
    Ok(nwritten as usize)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket(
    domain: AddressFamily,
    type_: SocketType,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(c::socket(
            domain.0 as c::c_int,
            type_.0 as c::c_int,
            protocol.0,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn socket_with(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<OwnedFd> {
    unsafe {
        ret_owned_fd(c::socket(
            domain.0 as c::c_int,
            type_.0 as c::c_int | flags.bits(),
            protocol.0,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(c::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v4(addr)).cast(),
            size_of::<c::sockaddr_in>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(c::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v6(addr)).cast(),
            size_of::<c::sockaddr_in6>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub(crate) fn bind_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(c::bind(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_unix(addr)).cast(),
            size_of::<c::sockaddr_un>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v4(sockfd: BorrowedFd<'_>, addr: &SocketAddrV4) -> io::Result<()> {
    unsafe {
        ret(c::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v4(addr)).cast(),
            size_of::<c::sockaddr_in>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_v6(sockfd: BorrowedFd<'_>, addr: &SocketAddrV6) -> io::Result<()> {
    unsafe {
        ret(c::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_v6(addr)).cast(),
            size_of::<c::sockaddr_in6>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub(crate) fn connect_unix(sockfd: BorrowedFd<'_>, addr: &SocketAddrUnix) -> io::Result<()> {
    unsafe {
        ret(c::connect(
            borrowed_fd(sockfd),
            as_ptr(&encode_sockaddr_unix(addr)).cast(),
            size_of::<c::sockaddr_un>() as c::socklen_t,
        ))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn listen(sockfd: BorrowedFd<'_>, backlog: c::c_int) -> io::Result<()> {
    unsafe { ret(c::listen(borrowed_fd(sockfd), backlog)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn accept(sockfd: BorrowedFd<'_>) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(c::accept(borrowed_fd(sockfd), null_mut(), null_mut()))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(
    windows,
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, flags: AcceptFlags) -> io::Result<OwnedFd> {
    unsafe {
        let owned_fd = ret_owned_fd(c::accept4(
            borrowed_fd(sockfd),
            null_mut(),
            null_mut(),
            flags.bits(),
        ))?;
        Ok(owned_fd)
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn acceptfrom(sockfd: BorrowedFd<'_>) -> io::Result<(OwnedFd, SocketAddrAny)> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        let owned_fd = ret_owned_fd(c::accept(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok((
            owned_fd,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

#[cfg(not(any(
    windows,
    target_os = "ios",
    target_os = "macos",
    target_os = "redox",
    target_os = "wasi"
)))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddrAny)> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        let owned_fd = ret_owned_fd(c::accept4(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
            flags.bits(),
        ))?;
        Ok((
            owned_fd,
            read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()),
        ))
    }
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(any(windows, target_os = "ios", target_os = "macos"))]
pub(crate) fn accept_with(sockfd: BorrowedFd<'_>, _flags: AcceptFlags) -> io::Result<OwnedFd> {
    accept(sockfd)
}

/// Darwin lacks `accept4`, but does have `accept`. We define
/// `AcceptFlags` to have no flags, so we can discard it here.
#[cfg(any(windows, target_os = "ios", target_os = "macos"))]
pub(crate) fn acceptfrom_with(
    sockfd: BorrowedFd<'_>,
    _flags: AcceptFlags,
) -> io::Result<(OwnedFd, SocketAddrAny)> {
    acceptfrom(sockfd)
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn shutdown(sockfd: BorrowedFd<'_>, how: Shutdown) -> io::Result<()> {
    unsafe { ret(c::shutdown(borrowed_fd(sockfd), how as c::c_int)) }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getsockname(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        ret(c::getsockname(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) fn getpeername(sockfd: BorrowedFd<'_>) -> io::Result<SocketAddrAny> {
    unsafe {
        let mut storage = MaybeUninit::<c::sockaddr_storage>::uninit();
        let mut len = size_of::<c::sockaddr_storage>() as c::socklen_t;
        ret(c::getpeername(
            borrowed_fd(sockfd),
            storage.as_mut_ptr().cast(),
            &mut len,
        ))?;
        Ok(read_sockaddr_os(storage.as_ptr(), len.try_into().unwrap()))
    }
}

#[cfg(not(any(windows, target_os = "redox", target_os = "wasi")))]
pub(crate) fn socketpair(
    domain: AddressFamily,
    type_: SocketType,
    flags: SocketFlags,
    protocol: Protocol,
) -> io::Result<(OwnedFd, OwnedFd)> {
    unsafe {
        let mut fds = MaybeUninit::<[OwnedFd; 2]>::uninit();
        ret(c::socketpair(
            domain.0 as c::c_int,
            type_.0 as c::c_int | flags.bits(),
            protocol.0,
            fds.as_mut_ptr().cast::<c::c_int>(),
        ))?;

        let [fd0, fd1] = fds.assume_init();
        Ok((fd0, fd1))
    }
}

#[cfg(not(any(target_os = "redox", target_os = "wasi")))]
pub(crate) mod sockopt {
    use super::{c, in6_addr_new, in_addr_new, BorrowedFd};
    use crate::net::sockopt::Timeout;
    use crate::net::{Ipv4Addr, Ipv6Addr, SocketType};
    use crate::{as_mut_ptr, io};
    use core::convert::TryInto;
    use core::time::Duration;

    // TODO: With Rust 1.53 we can use `Duration::ZERO` instead.
    const DURATION_ZERO: Duration = Duration::from_secs(0);

    #[inline]
    fn getsockopt<T>(fd: BorrowedFd<'_>, level: i32, optname: i32) -> io::Result<T> {
        use super::*;
        unsafe {
            let mut value = MaybeUninit::<T>::uninit();
            let mut optlen = core::mem::size_of::<T>().try_into().unwrap();
            ret(c::getsockopt(
                borrowed_fd(fd),
                level,
                optname,
                as_mut_ptr(&mut value).cast(),
                &mut optlen,
            ))?;
            assert_eq!(
                optlen as usize,
                size_of::<T>(),
                "unexpected getsockopt size"
            );
            Ok(value.assume_init())
        }
    }

    #[inline]
    fn setsockopt<T>(fd: BorrowedFd<'_>, level: i32, optname: i32, value: T) -> io::Result<()> {
        use super::*;
        unsafe {
            let optlen = core::mem::size_of::<T>().try_into().unwrap();
            ret(c::setsockopt(
                borrowed_fd(fd),
                level,
                optname,
                as_ptr(&value).cast(),
                optlen,
            ))
        }
    }

    #[inline]
    pub(crate) fn get_socket_type(fd: BorrowedFd<'_>) -> io::Result<SocketType> {
        getsockopt(fd, c::SOL_SOCKET as _, c::SO_TYPE)
    }

    #[inline]
    pub(crate) fn set_socket_reuseaddr(fd: BorrowedFd<'_>, reuseaddr: bool) -> io::Result<()> {
        setsockopt(
            fd,
            c::SOL_SOCKET as _,
            c::SO_REUSEADDR,
            from_bool(reuseaddr),
        )
    }

    #[inline]
    pub(crate) fn set_socket_broadcast(fd: BorrowedFd<'_>, broadcast: bool) -> io::Result<()> {
        setsockopt(
            fd,
            c::SOL_SOCKET as _,
            c::SO_BROADCAST,
            from_bool(broadcast),
        )
    }

    #[inline]
    pub(crate) fn get_socket_broadcast(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, c::SOL_SOCKET as _, c::SO_BROADCAST)
    }

    #[inline]
    pub(crate) fn set_socket_linger(
        fd: BorrowedFd<'_>,
        linger: Option<Duration>,
    ) -> io::Result<()> {
        let linger = c::linger {
            l_onoff: linger.is_some() as _,
            l_linger: linger
                .unwrap_or_default()
                .as_secs()
                .try_into()
                .map_err(|_convert_err| io::Error::INVAL)?,
        };
        setsockopt(fd, c::SOL_SOCKET as _, c::SO_LINGER, linger)
    }

    #[inline]
    pub(crate) fn get_socket_linger(fd: BorrowedFd<'_>) -> io::Result<Option<Duration>> {
        let linger: c::linger = getsockopt(fd, c::SOL_SOCKET as _, c::SO_LINGER)?;
        // TODO: With Rust 1.50, this could use `.then`.
        Ok(if linger.l_onoff != 0 {
            Some(Duration::from_secs(linger.l_linger as u64))
        } else {
            None
        })
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline]
    pub(crate) fn set_socket_passcred(fd: BorrowedFd<'_>, passcred: bool) -> io::Result<()> {
        setsockopt(fd, c::SOL_SOCKET as _, c::SO_PASSCRED, from_bool(passcred))
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    #[inline]
    pub(crate) fn get_socket_passcred(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, c::SOL_SOCKET as _, c::SO_PASSCRED)
    }

    #[inline]
    pub(crate) fn set_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
        timeout: Option<Duration>,
    ) -> io::Result<()> {
        let timeout = match timeout {
            Some(timeout) => {
                if timeout == DURATION_ZERO {
                    return Err(io::Error::INVAL);
                }

                let tv_sec = timeout.as_secs().try_into();
                #[cfg(not(all(target_arch = "x86_64", target_pointer_width = "32")))]
                let tv_sec = tv_sec.unwrap_or(c::c_long::MAX);
                #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
                let tv_sec = tv_sec.unwrap_or(i64::MAX);

                let mut timeout = c::timeval {
                    tv_sec,
                    tv_usec: timeout.subsec_micros() as _,
                };
                if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                    timeout.tv_usec = 1;
                }
                timeout
            }
            None => c::timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        };
        let optname = match id {
            Timeout::Recv => c::SO_RCVTIMEO,
            Timeout::Send => c::SO_SNDTIMEO,
        };
        setsockopt(fd, c::SOL_SOCKET, optname, timeout)
    }

    #[inline]
    pub(crate) fn get_socket_timeout(
        fd: BorrowedFd<'_>,
        id: Timeout,
    ) -> io::Result<Option<Duration>> {
        let optname = match id {
            Timeout::Recv => c::SO_RCVTIMEO,
            Timeout::Send => c::SO_SNDTIMEO,
        };
        let timeout: c::timeval = getsockopt(fd, c::SOL_SOCKET, optname)?;
        if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
            Ok(None)
        } else {
            Ok(Some(
                Duration::from_secs(timeout.tv_sec as u64)
                    + Duration::from_micros(timeout.tv_usec as u64),
            ))
        }
    }

    #[inline]
    pub(crate) fn set_ip_ttl(fd: BorrowedFd<'_>, ttl: u32) -> io::Result<()> {
        setsockopt(fd, c::IPPROTO_IP as _, c::IP_TTL, ttl)
    }

    #[inline]
    pub(crate) fn get_ip_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(fd, c::IPPROTO_IP as _, c::IP_TTL)
    }

    #[inline]
    pub(crate) fn set_ipv6_v6only(fd: BorrowedFd<'_>, only_v6: bool) -> io::Result<()> {
        setsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_V6ONLY, from_bool(only_v6))
    }

    #[inline]
    pub(crate) fn get_ipv6_v6only(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_V6ONLY)
    }

    #[inline]
    pub(crate) fn set_ip_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            c::IPPROTO_IP as _,
            c::IP_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ip_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, c::IPPROTO_IP as _, c::IP_MULTICAST_LOOP)
    }

    #[inline]
    pub(crate) fn set_ip_multicast_ttl(fd: BorrowedFd<'_>, multicast_ttl: u32) -> io::Result<()> {
        setsockopt(fd, c::IPPROTO_IP as _, c::IP_MULTICAST_TTL, multicast_ttl)
    }

    #[inline]
    pub(crate) fn get_ip_multicast_ttl(fd: BorrowedFd<'_>) -> io::Result<u32> {
        getsockopt(fd, c::IPPROTO_IP as _, c::IP_MULTICAST_TTL)
    }

    #[inline]
    pub(crate) fn set_ipv6_multicast_loop(
        fd: BorrowedFd<'_>,
        multicast_loop: bool,
    ) -> io::Result<()> {
        setsockopt(
            fd,
            c::IPPROTO_IPV6 as _,
            c::IPV6_MULTICAST_LOOP,
            from_bool(multicast_loop),
        )
    }

    #[inline]
    pub(crate) fn get_ipv6_multicast_loop(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, c::IPPROTO_IPV6 as _, c::IPV6_MULTICAST_LOOP)
    }

    #[inline]
    pub(crate) fn set_ip_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(fd, c::IPPROTO_IP as _, c::IP_ADD_MEMBERSHIP, mreq)
    }

    #[inline]
    pub(crate) fn set_ipv6_add_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "illumos",
            target_os = "solaris",
            target_os = "haiku",
            target_os = "l4re"
        )))]
        use c::IPV6_ADD_MEMBERSHIP;
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "illumos",
            target_os = "solaris",
            target_os = "haiku",
            target_os = "l4re"
        ))]
        use c::IPV6_JOIN_GROUP as IPV6_ADD_MEMBERSHIP;

        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(fd, c::IPPROTO_IPV6 as _, IPV6_ADD_MEMBERSHIP, mreq)
    }

    #[inline]
    pub(crate) fn set_ip_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv4Addr,
        interface: &Ipv4Addr,
    ) -> io::Result<()> {
        let mreq = to_imr(multiaddr, interface);
        setsockopt(fd, c::IPPROTO_IP as _, c::IP_DROP_MEMBERSHIP, mreq)
    }

    #[inline]
    pub(crate) fn set_ipv6_drop_membership(
        fd: BorrowedFd<'_>,
        multiaddr: &Ipv6Addr,
        interface: u32,
    ) -> io::Result<()> {
        #[cfg(not(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "illumos",
            target_os = "solaris",
            target_os = "haiku",
            target_os = "l4re"
        )))]
        use c::IPV6_DROP_MEMBERSHIP;
        #[cfg(any(
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "ios",
            target_os = "macos",
            target_os = "openbsd",
            target_os = "netbsd",
            target_os = "illumos",
            target_os = "solaris",
            target_os = "haiku",
            target_os = "l4re"
        ))]
        use c::IPV6_LEAVE_GROUP as IPV6_DROP_MEMBERSHIP;

        let mreq = to_ipv6mr(multiaddr, interface);
        setsockopt(fd, c::IPPROTO_IPV6 as _, IPV6_DROP_MEMBERSHIP, mreq)
    }

    #[inline]
    pub(crate) fn set_tcp_nodelay(fd: BorrowedFd<'_>, nodelay: bool) -> io::Result<()> {
        setsockopt(fd, c::IPPROTO_TCP as _, c::TCP_NODELAY, from_bool(nodelay))
    }

    #[inline]
    pub(crate) fn get_tcp_nodelay(fd: BorrowedFd<'_>) -> io::Result<bool> {
        getsockopt(fd, c::IPPROTO_TCP as _, c::TCP_NODELAY)
    }

    #[inline]
    fn to_imr(multiaddr: &Ipv4Addr, interface: &Ipv4Addr) -> c::ip_mreq {
        c::ip_mreq {
            imr_multiaddr: to_imr_addr(multiaddr),
            imr_interface: to_imr_addr(interface),
        }
    }

    #[inline]
    fn to_imr_addr(addr: &Ipv4Addr) -> c::in_addr {
        in_addr_new(u32::from_ne_bytes(addr.octets()))
    }

    #[inline]
    fn to_ipv6mr(multiaddr: &Ipv6Addr, interface: u32) -> c::ipv6_mreq {
        c::ipv6_mreq {
            ipv6mr_multiaddr: to_ipv6mr_multiaddr(multiaddr),
            ipv6mr_interface: to_ipv6mr_interface(interface),
        }
    }

    #[inline]
    fn to_ipv6mr_multiaddr(multiaddr: &Ipv6Addr) -> c::in6_addr {
        in6_addr_new(multiaddr.octets())
    }

    #[cfg(target_os = "android")]
    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c::c_int {
        interface as c::c_int
    }

    #[cfg(not(target_os = "android"))]
    #[inline]
    fn to_ipv6mr_interface(interface: u32) -> c::c_uint {
        interface as c::c_uint
    }

    #[inline]
    fn from_bool(value: bool) -> c::c_uint {
        value as c::c_uint
    }
}
