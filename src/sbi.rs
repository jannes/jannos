use core::{arch::asm, fmt};

#[repr(C)]
pub struct SbiRet {
    error: i64,
    value: i64,
}

pub struct SBIOut;

impl fmt::Write for SBIOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = sbi_dbg_write(s.as_bytes());
        Ok(())
    }
}

pub fn sbi_out() -> impl fmt::Write {
    SBIOut {}
}

pub fn sbi_dbg_write(data: &[u8]) -> SbiRet {
    let mut sbi_ret = SbiRet { error: 0, value: 0 };

    unsafe {
        asm!(
            "ecall",
            in("a7") DBG_WRITE_EID,
            in("a6") 0,
            in("a0") data.len(),
            in("a1") data.as_ptr(),
            in("a2") 0,
            lateout("a0") sbi_ret.error,
            lateout("a1") sbi_ret.value,
        );
    }

    sbi_ret
}

pub fn sbi_hart_start(hart_id: usize, start_addr: usize, opaque: usize) -> SbiRet {
    let mut sbi_ret = SbiRet { error: 0, value: 0 };

    unsafe {
        asm!(
            "ecall",
            in("a7") HART_START_EID,
            in("a6") HART_START_FID,
            in("a0") hart_id,
            in("a1") start_addr,
            in("a2") opaque,
            lateout("a0") sbi_ret.error,
            lateout("a1") sbi_ret.value,
        )
    }

    sbi_ret
}

const DBG_WRITE_EID: u64 = 0x4442434E;
const HART_START_EID: u64 = 0x48534D;
const HART_START_FID: u64 = 0;
