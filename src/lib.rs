pub mod cpu;
pub mod registers;
pub mod opcodes;

pub trait NegativeSet {
    fn negative_set(&self) -> bool;
}

impl NegativeSet for u8 {
    fn negative_set(&self) -> bool {
        //  Is Set    Is Reset
        //  1010_0010 0111_0100
        // &1000_0000 1000_0000
        // -
        //  1000_0000 0000_0000

        // So,  (0b1000_0000 & 0b1000_0000 != 0) == true
        // And, (0b0000_0000 & 0b1000_0000 != 0) == false
        self & 0b1000_0000 != 0
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::Cpu;

    #[test]
    fn test_zero() {
        let mut cpu = Cpu::new();

        cpu.load_and_run(&vec![0xA9, 0x00, 0x00]);

        assert_eq!(cpu.registers.status.zero, true);

        cpu.load_and_run(&vec![0xA9, 0x01, 0x00]);

        assert_eq!(cpu.registers.status.zero, false);
    }

    #[test]
    fn test_negative() {
        let mut cpu = Cpu::new();

        // 0x80 has bit 7 set so negative should be true
        cpu.load_and_run(&vec![0xA9, 0x80, 0x00]);

        assert_eq!(cpu.registers.status.negative, true);

        // 0x00 has bit 7 reset so negative should be false
        cpu.load_and_run(&vec![0xA9, 0x00, 0x00]);

        assert_eq!(cpu.registers.status.negative, false);
    }

    #[test]
    fn test_lda_immediate() {
        let mut cpu = Cpu::new();

        cpu.load_and_run(&vec![0xA9, 0x11, 0x00]);

        assert_eq!(cpu.registers.a, 0x11);
    }

    #[test]
    fn test_lda_zeropage() {
        let mut cpu = Cpu::new();

        // Load A with 0xff, Store a at 0x01, Load A with 0x00, Load A with the data at 0x01
        cpu.load_and_run(&vec![0xa9, 0xff, 0x85, 0x01, 0xa9, 0x00, 0xa5, 0x01, 0x00]);

        assert_eq!(cpu.registers.a, 0xff);
    }
}