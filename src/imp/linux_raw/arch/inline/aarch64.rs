use crate::imp::reg::{ArgReg, FromAsm, RetReg, SyscallNumber, ToAsm, A0, A1, A2, A3, A4, A5, R0};
use core::arch::asm;

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall0_readonly(nr: SyscallNumber) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        lateout("x0") r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall1_noreturn(nr: SyscallNumber<'_>, a0: ArgReg<'_, A0>) -> ! {
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        in("x0") a0.to_asm(),
        options(noreturn)
    )
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall2_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall3_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall4(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        in("x3") a3.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall4_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        in("x3") a3.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall5(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        in("x3") a3.to_asm(),
        in("x4") a4.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall5_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        in("x3") a3.to_asm(),
        in("x4") a4.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall6(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
    a5: ArgReg<'_, A5>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        in("x3") a3.to_asm(),
        in("x4") a4.to_asm(),
        in("x5") a5.to_asm(),
        options(nostack, preserves_flags)
    );
    FromAsm::from_asm(r0)
}

#[inline]
#[must_use]
pub(in crate::imp) unsafe fn syscall6_readonly(
    nr: SyscallNumber<'_>,
    a0: ArgReg<'_, A0>,
    a1: ArgReg<'_, A1>,
    a2: ArgReg<'_, A2>,
    a3: ArgReg<'_, A3>,
    a4: ArgReg<'_, A4>,
    a5: ArgReg<'_, A5>,
) -> RetReg<R0> {
    let r0;
    asm!(
        "svc 0",
        in("x8") nr.to_asm(),
        inlateout("x0") a0.to_asm() => r0,
        in("x1") a1.to_asm(),
        in("x2") a2.to_asm(),
        in("x3") a3.to_asm(),
        in("x4") a4.to_asm(),
        in("x5") a5.to_asm(),
        options(nostack, preserves_flags, readonly)
    );
    FromAsm::from_asm(r0)
}
