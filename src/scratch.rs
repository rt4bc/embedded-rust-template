use core::arch::asm;

pub fn asm_test() {
    unsafe {
        asm!("nop");
    }

    unsafe {
        let a: u32;
        asm!(
            "mov {}, 1",
            out(reg) a
        );
        assert_eq!(a, 1);
    }

    unsafe {
        let i: u32 = 3;
        let o: u32;
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
        assert_eq!(o, 8);
    }

    unsafe {
        let i: u32 = 3;
        let o: u32;
        asm!(
            "mov {0}, {tmp}",
            "add {0}, {1}",
            out(reg) o,
            in(reg) i,
            tmp = in(reg) 10,
        );
        assert_eq!(o, 13);
    }
}

pub fn fugit_test() {
    use fugit::{Duration, ExtU32};
    let d = Duration::<u32, 1, 1_000>::from_ticks(111);
}
