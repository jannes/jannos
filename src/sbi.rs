use core::arch::asm;

#[repr(C)]
pub struct SbiRet {
    error: i64,
    value: i64,
}

pub fn sbi_dbg_write(data: &[u8]) -> SbiRet {
    let mut sbi_ret = SbiRet { error: 0, value: 0 };

    unsafe {
        asm!(
            "ecall",
            in("a7") DBCN_EID,
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

const DBCN_EID: u64 = 0x4442434E;
