//! Raw x86_64 hardware transactional memory primitives

mod intrinsics {
    extern "C" {
        #[link_name = "llvm.x86.xbegin"]
        pub fn xbegin() -> i32;

        #[link_name = "llvm.x86.xend"]
        pub fn xend() -> ();

        #[link_name = "llvm.x86.xabort"]
        pub fn xabort(a: i8) -> ();

        #[link_name = "llvm.x86.xtest"]
        pub fn xtest() -> i32;
    }
}

pub trait XAbortConst {
    const CODE: i8;
}

#[inline]
pub unsafe fn xbegin() -> i32 {
    intrinsics::xbegin()
}

#[inline]
pub unsafe fn xend() {
    intrinsics::xend()
}

#[inline(always)]
pub unsafe fn xabort<T: XAbortConst>() -> ! {
    intrinsics::xabort(T::CODE);
    std::hint::unreachable_unchecked()
}

#[inline]
pub unsafe fn xtest() -> i32 {
    intrinsics::xtest()
}

pub const _XBEGIN_STARTED: i32 = !0 as i32;
pub const _XABORT_EXPLICIT: i32 = 1i32 << 0;
pub const _XABORT_RETRY: i32 = 1i32 << 1;
pub const _XABORT_CONFLICT: i32 = 1i32 << 2;
pub const _XABORT_CAPACITY: i32 = 1i32 << 3;
pub const _XABORT_DEBUG: i32 = 1i32 << 4;
pub const _XABORT_NESTED: i32 = 1i32 << 5;

#[allow(non_snake_case)]
#[inline(always)]
pub const fn _XABORT_CODE(x: i32) -> i32 {
    ((x) >> 24) & 0xFFi32
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
pub(super) struct BeginCode(i32);

impl BeginCode {
    #[inline]
    pub fn is_started(&self) -> bool {
        self.0 == _XBEGIN_STARTED
    }

    #[inline]
    pub fn is_explicit_abort(&self) -> bool {
        self.0 & _XABORT_EXPLICIT != 0
    }

    #[inline]
    pub fn is_retry(&self) -> bool {
        self.0 & _XABORT_RETRY != 0
    }

    #[inline]
    pub fn is_conflict(&self) -> bool {
        self.0 & _XABORT_CONFLICT != 0
    }

    #[inline]
    pub fn is_capacity(&self) -> bool {
        self.0 & _XABORT_CONFLICT != 0
    }
}

#[repr(transparent)]
#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
pub(super) struct TestCode(i32);

impl TestCode {
    #[inline]
    pub fn in_transaction(&self) -> bool {
        self.0 != 0
    }

    #[inline]
    pub fn is_suspended(&self) -> bool {
        false
    }
}

#[inline]
pub(super) unsafe fn begin() -> BeginCode {
    BeginCode(xbegin())
}

#[inline(always)]
pub(super) unsafe fn abort() -> ! {
    struct Code;
    impl XAbortConst for Code {
        const CODE: i8 = 0;
    }
    xabort::<Code>();
}

#[inline]
pub(super) unsafe fn test() -> TestCode {
    TestCode(xtest())
}

#[inline]
pub(super) unsafe fn end() {
    xend()
}

#[inline]
pub(super) const fn htm_supported() -> bool {
    true
}

#[inline]
pub(super) fn htm_supported_runtime() -> bool {
    unsafe { std::arch::x86_64::__cpuid_count(0x7, 0x0).ebx & (1 << 11) != 0 }
}
