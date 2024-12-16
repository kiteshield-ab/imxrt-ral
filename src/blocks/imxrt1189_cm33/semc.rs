#[doc = "SEMC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Module Control Register"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "IO MUX Control Register"]
    pub IOCR: crate::RWRegister<u32>,
    #[doc = "Bus (AXI) Master Control Register 0"]
    pub BMCR0: crate::RWRegister<u32>,
    #[doc = "Bus (AXI) Master Control Register 1"]
    pub BMCR1: crate::RWRegister<u32>,
    #[doc = "Base Register n"]
    pub BR: [crate::RWRegister<u32>; 9usize],
    #[doc = "DLL Control Register"]
    pub DLLCR: crate::RWRegister<u32>,
    #[doc = "Interrupt Enable Register"]
    pub INTEN: crate::RWRegister<u32>,
    #[doc = "Interrupt Register"]
    pub INTR: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 0"]
    pub SDRAMCR0: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 1"]
    pub SDRAMCR1: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 2"]
    pub SDRAMCR2: crate::RWRegister<u32>,
    #[doc = "SDRAM Control Register 3"]
    pub SDRAMCR3: crate::RWRegister<u32>,
    #[doc = "NAND Control Register 0"]
    pub NANDCR0: crate::RWRegister<u32>,
    #[doc = "NAND Control Register 1"]
    pub NANDCR1: crate::RWRegister<u32>,
    #[doc = "NAND Control Register 2"]
    pub NANDCR2: crate::RWRegister<u32>,
    #[doc = "NAND Control Register 3"]
    pub NANDCR3: crate::RWRegister<u32>,
    #[doc = "NOR Control Register 0"]
    pub NORCR0: crate::RWRegister<u32>,
    #[doc = "NOR Control Register 1"]
    pub NORCR1: crate::RWRegister<u32>,
    #[doc = "NOR Control Register 2"]
    pub NORCR2: crate::RWRegister<u32>,
    #[doc = "NOR Control Register 3"]
    pub NORCR3: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 0"]
    pub SRAMCR0: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 1"]
    pub SRAMCR1: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 2"]
    pub SRAMCR2: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 3"]
    pub SRAMCR3: crate::RWRegister<u32>,
    #[doc = "DBI-B Control Register 0"]
    pub DBICR0: crate::RWRegister<u32>,
    #[doc = "DBI-B Control Register 1"]
    pub DBICR1: crate::RWRegister<u32>,
    #[doc = "DBI-B Control Register 2"]
    pub DBICR2: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "IP Command Control Register 0"]
    pub IPCR0: crate::RWRegister<u32>,
    #[doc = "IP Command Control Register 1"]
    pub IPCR1: crate::RWRegister<u32>,
    #[doc = "IP Command Control Register 2"]
    pub IPCR2: crate::RWRegister<u32>,
    #[doc = "IP Command Register"]
    pub IPCMD: crate::RWRegister<u32>,
    #[doc = "TX DATA Register"]
    pub IPTXDAT: crate::RWRegister<u32>,
    _reserved1: [u8; 0x0c],
    #[doc = "RX DATA Register"]
    pub IPRXDAT: crate::RORegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "Status Register 0"]
    pub STS0: crate::RORegister<u32>,
    #[doc = "Status Register 1"]
    pub STS1: crate::RORegister<u32>,
    #[doc = "Status Register 2"]
    pub STS2: crate::RORegister<u32>,
    #[doc = "Status Register 3"]
    pub STS3: crate::RORegister<u32>,
    #[doc = "Status Register 4"]
    pub STS4: crate::RORegister<u32>,
    #[doc = "Status Register 5"]
    pub STS5: crate::RORegister<u32>,
    #[doc = "Status Register 6"]
    pub STS6: crate::RORegister<u32>,
    #[doc = "Status Register 7"]
    pub STS7: crate::RORegister<u32>,
    #[doc = "Status Register 8"]
    pub STS8: crate::RORegister<u32>,
    #[doc = "Status Register 9"]
    pub STS9: crate::RORegister<u32>,
    #[doc = "Status Register 10"]
    pub STS10: crate::RORegister<u32>,
    #[doc = "Status Register 11"]
    pub STS11: crate::RORegister<u32>,
    #[doc = "Status Register 12"]
    pub STS12: crate::RORegister<u32>,
    #[doc = "Status Register 13"]
    pub STS13: crate::RORegister<u32>,
    #[doc = "Status Register 14"]
    pub STS14: crate::RORegister<u32>,
    #[doc = "Status Register 15"]
    pub STS15: crate::RORegister<u32>,
    #[doc = "Base Register 9"]
    pub BR9: crate::RWRegister<u32>,
    #[doc = "Base Register 10"]
    pub BR10: crate::RWRegister<u32>,
    #[doc = "Base Register 11"]
    pub BR11: crate::RWRegister<u32>,
    _reserved3: [u8; 0x14],
    #[doc = "SRAM Control Register 4"]
    pub SRAMCR4: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 5"]
    pub SRAMCR5: crate::RWRegister<u32>,
    #[doc = "SRAM Control Register 6"]
    pub SRAMCR6: crate::RWRegister<u32>,
    _reserved4: [u8; 0x14],
    #[doc = "NAND Buffer DATA Register"]
    pub NDBD: crate::RWRegister<u32>,
    #[doc = "NAND Buffer Address Register"]
    pub NDBA: crate::RWRegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "Delay Chain Control Register"]
    pub DCCR: crate::RWRegister<u32>,
    #[doc = "SDRAM Prefetch Control Register"]
    pub SDRAMPCR: crate::RWRegister<u32>,
}
#[doc = "Module Control Register"]
pub mod MCR {
    #[doc = "Software Reset"]
    pub mod SWRST {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No reset"]
            pub const NO_RESET: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "Module Disable"]
    pub mod MDIS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Module enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Module disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
    #[doc = "DQS (read strobe) mode"]
    pub mod DQSMD {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Dummy read strobe loopbacked internally"]
            pub const INTERNAL: u32 = 0;
            #[doc = "Dummy read strobe loopbacked from DQS pad"]
            pub const DQS_PAD: u32 = 0x01;
        }
    }
    #[doc = "WAIT/RDY polarity for SRAM/NOR"]
    pub mod WPOL0 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "WAIT/RDY polarity is not changed."]
            pub const UNCHANGED: u32 = 0;
            #[doc = "WAIT/RDY polarity is inverted."]
            pub const INVERTED: u32 = 0x01;
        }
    }
    #[doc = "R/B# polarity for NAND device"]
    pub mod WPOL1 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "R/B# polarity is not changed."]
            pub const UNCHANGED: u32 = 0;
            #[doc = "R/B# polarity is inverted."]
            pub const INVERTED: u32 = 0x01;
        }
    }
    #[doc = "Command Execution timeout cycles"]
    pub mod CTO {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bus timeout cycles"]
    pub mod BTO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "255*1"]
            pub const BTO_0: u32 = 0;
            #[doc = "255*2"]
            pub const BTO_1: u32 = 0x01;
            #[doc = "255*2^31"]
            pub const BTO_1F: u32 = 0x1f;
        }
    }
}
#[doc = "IO MUX Control Register"]
pub mod IOCR {
    #[doc = "SEMC_ADDR08 output selection"]
    pub mod MUX_A8 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_03_0: u32 = 0;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_03_1: u32 = 0x01;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_03_2: u32 = 0x02;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_03_3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const NAND_CEB: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const NOR_CEB: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const SRAM_CEB0: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const DBI_CSX: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const SRAM_CEB1: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const SRAM_CEB2: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const SRAM_CEB3: u32 = 0x0a;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_BF_11: u32 = 0x0b;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_BF_12: u32 = 0x0c;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_BF_13: u32 = 0x0d;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_BF_14: u32 = 0x0e;
            #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
            pub const SDRAM8_NORSRAM24_BF_15: u32 = 0x0f;
        }
    }
    #[doc = "SEMC_CSX0 output selection"]
    pub mod MUX_CSX0 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const NORSRAM24_03: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const SDRAM_CS1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const SDRAM_CS2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const SDRAM_CS3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const NAND_CEB: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const NOR_CEB: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const SRAM_CEB0: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const DBI_CSX: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const SRAM_CEB1: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const SRAM_CEB2: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const SRAM_CEB3: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const NORSRAM24_BF_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const NORSRAM24_BF_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const NORSRAM24_BF_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const NORSRAM24_BF_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
            pub const NORSRAM24_BF_15: u32 = 0x0f;
        }
    }
    #[doc = "SEMC_CSX1 output selection"]
    pub mod MUX_CSX1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const NORSRAM25_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const SDRAM_CS1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const SDRAM_CS2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const SDRAM_CS3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const NAND_CEB: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const NOR_CEB: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const SRAM_CEB0: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const DBI_CSX: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const SRAM_CEB1: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const SRAM_CEB2: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const SRAM_CEB3: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const NORSRAM25_BF_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const NORSRAM25_BF_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const NORSRAM25_BF_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const NORSRAM25_BF_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
            pub const NORSRAM25_BF_15: u32 = 0x0f;
        }
    }
    #[doc = "SEMC_CSX2 output selection"]
    pub mod MUX_CSX2 {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const NORSRAM26_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const SDRAM_CS1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const SDRAM_CS2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const SDRAM_CS3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const NAND_CEB: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const NOR_CEB: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const SRAM_CEB0: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const DBI_CSX: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const SRAM_CEB1: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const SRAM_CEB2: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const SRAM_CEB3: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const NORSRAM26_BF_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const NORSRAM26_BF_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const NORSRAM26_BF_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const NORSRAM26_BF_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
            pub const NORSRAM26_BF_15: u32 = 0x0f;
        }
    }
    #[doc = "SEMC_CSX3 output selection"]
    pub mod MUX_CSX3 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_0: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const SDRAM_CS1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const SDRAM_CS2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const SDRAM_CS3: u32 = 0x03;
            #[doc = "NAND CE#"]
            pub const NAND_CEB: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const NOR_CEB: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const SRAM_CEB0: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const DBI_CSX: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const SRAM_CEB1: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const SRAM_CEB2: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const SRAM_CEB3: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_BF_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_BF_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_BF_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_BF_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_BF_15: u32 = 0x0f;
        }
    }
    #[doc = "SEMC_RDY function selection"]
    pub mod MUX_RDY {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NAND R/B# input"]
            pub const NAND_RBB: u32 = 0;
            #[doc = "SDRAM CS1"]
            pub const SDRAM_CS1: u32 = 0x01;
            #[doc = "SDRAM CS2"]
            pub const SDRAM_CS2: u32 = 0x02;
            #[doc = "SDRAM CS3"]
            pub const SDRAM_CS3: u32 = 0x03;
            #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
            pub const NORSRAM27_4: u32 = 0x04;
            #[doc = "NOR CE#"]
            pub const NOR_CEB: u32 = 0x05;
            #[doc = "SRAM CE# 0"]
            pub const SRAM_CEB0: u32 = 0x06;
            #[doc = "DBI CSX"]
            pub const DBI_CSX: u32 = 0x07;
            #[doc = "SRAM CE# 1"]
            pub const SRAM_CEB1: u32 = 0x08;
            #[doc = "SRAM CE# 2"]
            pub const SRAM_CEB2: u32 = 0x09;
            #[doc = "SRAM CE# 3"]
            pub const SRAM_CEB3: u32 = 0x0a;
            #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
            pub const NORSRAM27_BF_11: u32 = 0x0b;
            #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
            pub const NORSRAM27_BF_12: u32 = 0x0c;
            #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
            pub const NORSRAM27_BF_13: u32 = 0x0d;
            #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
            pub const NORSRAM27_BF_14: u32 = 0x0e;
            #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
            pub const NORSRAM27_BF_15: u32 = 0x0f;
        }
    }
    #[doc = "SEMC_CLKX0 function selection"]
    pub mod MUX_CLKX0 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep low"]
            pub const KEEPLOW: u32 = 0;
            #[doc = "NOR clock"]
            pub const NOR_CLK: u32 = 0x01;
            #[doc = "SRAM clock"]
            pub const SRAM_CLK: u32 = 0x02;
            #[doc = "NOR and SRAM clock, suitable for Multi-Chip Product package"]
            pub const NORSRAM_CLK: u32 = 0x03;
        }
    }
    #[doc = "SEMC_CLKX1 function selection"]
    pub mod MUX_CLKX1 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Keep low"]
            pub const KEEPLOW: u32 = 0;
            #[doc = "NOR clock"]
            pub const NOR_CLK: u32 = 0x01;
            #[doc = "SRAM clock"]
            pub const SRAM_CLK: u32 = 0x02;
            #[doc = "NOR and SRAM clock, suitable for Multi-Chip Product package"]
            pub const NOR_SRAM_CLK: u32 = 0x03;
        }
    }
    #[doc = "SEMC_CLKX0 Always On"]
    pub mod CLKX0_AO {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SEMC_CLKX0 is controlled by MUX_CLKX0"]
            pub const MUX_CLKX0_CTL: u32 = 0;
            #[doc = "SEMC_CLKX0 is always on"]
            pub const ALWAYS_ON: u32 = 0x01;
        }
    }
    #[doc = "SEMC_CLKX1 Always On"]
    pub mod CLKX1_AO {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SEMC_CLKX1 is controlled by MUX_CLKX1"]
            pub const MUX_CLKX1_CTL: u32 = 0;
            #[doc = "SEMC_CLKX1 is always on"]
            pub const ALWAYS_ON: u32 = 0x01;
        }
    }
}
#[doc = "Bus (AXI) Master Control Register 0"]
pub mod BMCR0 {
    #[doc = "Weight of QOS"]
    pub mod WQOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of AGE"]
    pub mod WAGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Slave Hit without read/write switch"]
    pub mod WSH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of slave hit with Read/Write Switch"]
    pub mod WRWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Bus (AXI) Master Control Register 1"]
pub mod BMCR1 {
    #[doc = "Weight of QOS"]
    pub mod WQOS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of AGE"]
    pub mod WAGE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Page Hit"]
    pub mod WPH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of slave hit without Read/Write Switch"]
    pub mod WRWS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Weight of Bank Rotation"]
    pub mod WBR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register n"]
pub mod BR {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const INVALID: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const SIZE_4KB: u32 = 0;
            #[doc = "8KB"]
            pub const SIZE_8KB: u32 = 0x01;
            #[doc = "16KB"]
            pub const SIZE_16KB: u32 = 0x02;
            #[doc = "32KB"]
            pub const SIZE_32KB: u32 = 0x03;
            #[doc = "64KB"]
            pub const SIZE_64KB: u32 = 0x04;
            #[doc = "128KB"]
            pub const SIZE_128KB: u32 = 0x05;
            #[doc = "256KB"]
            pub const SIZE_256KB: u32 = 0x06;
            #[doc = "512KB"]
            pub const SIZE_512KB: u32 = 0x07;
            #[doc = "1MB"]
            pub const SIZE_1MB: u32 = 0x08;
            #[doc = "2MB"]
            pub const SIZE_2MB: u32 = 0x09;
            #[doc = "4MB"]
            pub const SIZE_4MB: u32 = 0x0a;
            #[doc = "8MB"]
            pub const SIZE_8MB: u32 = 0x0b;
            #[doc = "16MB"]
            pub const SIZE_16MB: u32 = 0x0c;
            #[doc = "32MB"]
            pub const SIZE_32MB: u32 = 0x0d;
            #[doc = "64MB"]
            pub const SIZE_64MB: u32 = 0x0e;
            #[doc = "128MB"]
            pub const SIZE_128MB: u32 = 0x0f;
            #[doc = "256MB"]
            pub const SIZE_256MB: u32 = 0x10;
            #[doc = "512MB"]
            pub const SIZE_512MB: u32 = 0x11;
            #[doc = "1GB"]
            pub const SIZE_1GB: u32 = 0x12;
            #[doc = "2GB"]
            pub const SIZE_2GB: u32 = 0x13;
            #[doc = "4GB"]
            pub const SIZE_4GB_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const SIZE_4GB_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const SIZE_4GB_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const SIZE_4GB_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const SIZE_4GB_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const SIZE_4GB_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const SIZE_4GB_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const SIZE_4GB_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const SIZE_4GB_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const SIZE_4GB_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const SIZE_4GB_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const SIZE_4GB_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DLL Control Register"]
pub mod DLLCR {
    #[doc = "DLL calibration enable"]
    pub mod DLLEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DLL calibration is disabled."]
            pub const CAL_DISABLE: u32 = 0;
            #[doc = "DLL calibration is enabled."]
            pub const CAL_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "DLL Reset"]
    pub mod DLLRESET {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "DLL is not reset."]
            pub const NO_RESET: u32 = 0;
            #[doc = "DLL is reset."]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "Delay Target for Slave"]
    pub mod SLVDLYTARGET {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Override Enable"]
    pub mod OVRDEN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The delay cell number is not overridden."]
            pub const NO_OVERRIDE: u32 = 0;
            #[doc = "The delay cell number is overridden."]
            pub const OVERRIDE: u32 = 0x01;
        }
    }
    #[doc = "Override Value"]
    pub mod OVRDVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Interrupt Enable Register"]
pub mod INTEN {
    #[doc = "IP command done interrupt enable"]
    pub mod IPCMDDONEEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "IP command error interrupt enable"]
    pub mod IPCMDERREN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AXI command error interrupt enable"]
    pub mod AXICMDERREN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "AXI bus error interrupt enable"]
    pub mod AXIBUSERREN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "NAND page end interrupt enable"]
    pub mod NDPAGEENDEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "NAND no pending AXI access interrupt enable"]
    pub mod NDNOPENDEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "NAND ECC fail interrupt enable"]
    pub mod NDECCFAILEN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "NAND buffer end interrupt enable"]
    pub mod NDBUFENDEN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Interrupt is disabled"]
            pub const INTERRUPT_DISABLE: u32 = 0;
            #[doc = "Interrupt is enabled"]
            pub const INTERRUPT_ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt Register"]
pub mod INTR {
    #[doc = "IP command normal done interrupt"]
    pub mod IPCMDDONE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IP command is not done."]
            pub const NOT_DONE: u32 = 0;
            #[doc = "IP command is done."]
            pub const DONE: u32 = 0x01;
        }
    }
    #[doc = "IP command error done interrupt"]
    pub mod IPCMDERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No IP command error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "IP command error occurs."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "AXI command error interrupt"]
    pub mod AXICMDERR {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No AXI command error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "AXI command error occurs."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "AXI bus error interrupt"]
    pub mod AXIBUSERR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No AXI bus error."]
            pub const NO_ERROR: u32 = 0;
            #[doc = "AXI bus error occurs."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "NAND page end interrupt"]
    pub mod NDPAGEEND {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The last address of main space in the NAND is not written by AXI command."]
            pub const NO_WRITE: u32 = 0;
            #[doc = "The last address of main space in the NAND is written by AXI command."]
            pub const WRITE: u32 = 0x01;
        }
    }
    #[doc = "NAND no pending AXI write transaction interrupt"]
    pub mod NDNOPEND {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "At least one NAND AXI write transaction is pending or no NAND write transaction is sent to the queue."]
            pub const NO_WRITE: u32 = 0;
            #[doc = "All NAND AXI write pending transactions are finished."]
            pub const WRITE: u32 = 0x01;
        }
    }
    #[doc = "NAND ECC fail interrupt"]
    pub mod NDECCFAIL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NAND ECC data correction pass."]
            pub const CORRECTION_PASS: u32 = 0;
            #[doc = "NAND ECC data correction fail."]
            pub const CORRECTION_FAIL: u32 = 0x01;
        }
    }
    #[doc = "NAND buffer end interrupt"]
    pub mod NDBUFEND {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Last valid address of NAND buffer is not written by AXI command."]
            pub const NO_WRITE: u32 = 0;
            #[doc = "Last valid address of NAND buffer is written by AXI command."]
            pub const WRITE: u32 = 0x01;
        }
    }
}
#[doc = "SDRAM Control Register 0"]
pub mod SDRAMCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_8BIT: u32 = 0;
            #[doc = "16bit"]
            pub const PS_16BIT: u32 = 0x01;
            #[doc = "32bit"]
            pub const PS_32BIT: u32 = 0x02;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BURST1: u32 = 0;
            #[doc = "2"]
            pub const BURST2: u32 = 0x01;
            #[doc = "4"]
            pub const BURST4: u32 = 0x02;
            #[doc = "8"]
            pub const BURST8_3: u32 = 0x03;
            #[doc = "8"]
            pub const BURST8_4: u32 = 0x04;
            #[doc = "8"]
            pub const BURST8_5: u32 = 0x05;
            #[doc = "8"]
            pub const BURST8_6: u32 = 0x06;
            #[doc = "8"]
            pub const BURST8_7: u32 = 0x07;
        }
    }
    #[doc = "Column 8 selection"]
    pub mod COL8 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Column address bit number is decided by COL field."]
            pub const COLFIELD: u32 = 0;
            #[doc = "Column address bit number is 8. COL field is ignored."]
            pub const BIT8: u32 = 0x01;
        }
    }
    #[doc = "Column address bit number"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12"]
            pub const BIT12: u32 = 0;
            #[doc = "11"]
            pub const BIT11: u32 = 0x01;
            #[doc = "10"]
            pub const BIT10: u32 = 0x02;
            #[doc = "9"]
            pub const BIT9: u32 = 0x03;
        }
    }
    #[doc = "CAS Latency"]
    pub mod CL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const CL1_0: u32 = 0;
            #[doc = "1"]
            pub const CL1_1: u32 = 0x01;
            #[doc = "2"]
            pub const CL2: u32 = 0x02;
            #[doc = "3"]
            pub const CL3: u32 = 0x03;
        }
    }
    #[doc = "2 Bank selection bit"]
    pub mod BANK2 {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDRAM device has 4 banks."]
            pub const BANK4: u32 = 0;
            #[doc = "SDRAM device has 2 banks."]
            pub const BANK2: u32 = 0x01;
        }
    }
}
#[doc = "SDRAM Control Register 1"]
pub mod SDRAMCR1 {
    #[doc = "PRECHARGE to ACTIVE/REFRESH command wait time"]
    pub mod PRE2ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACTIVE to READ/WRITE delay"]
    pub mod ACT2RW {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFRESH recovery time"]
    pub mod RFRC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WRITE recovery time"]
    pub mod WRC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CKE off minimum time"]
    pub mod CKEOFF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACTIVE to PRECHARGE minimum time"]
    pub mod ACT2PRE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM Control Register 2"]
pub mod SDRAMCR2 {
    #[doc = "SELF REFRESH recovery time"]
    pub mod SRRC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "REFRESH to REFRESH delay"]
    pub mod REF2REF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ACTIVE to ACTIVE delay"]
    pub mod ACT2ACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SDRAM idle timeout"]
    pub mod ITO {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "IDLE timeout period is 256*Prescale period."]
            pub const PRESCALEX256: u32 = 0;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_1: u32 = 0x01;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_2: u32 = 0x02;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_3: u32 = 0x03;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_4: u32 = 0x04;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_5: u32 = 0x05;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_6: u32 = 0x06;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_7: u32 = 0x07;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_8: u32 = 0x08;
            #[doc = "IDLE timeout period is ITO*Prescale period."]
            pub const PRESCALEXITO_9: u32 = 0x09;
        }
    }
}
#[doc = "SDRAM Control Register 3"]
pub mod SDRAMCR3 {
    #[doc = "Refresh enable"]
    pub mod REN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The SEMC does not send AUTO REFRESH command automatically"]
            pub const NO_AUTO_REFRESH: u32 = 0;
            #[doc = "The SEMC sends AUTO REFRESH command automatically"]
            pub const AUTO_REFRESH: u32 = 0x01;
        }
    }
    #[doc = "Refresh burst length"]
    pub mod REBL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const REFRESH_BURST1: u32 = 0;
            #[doc = "2"]
            pub const REFRESH_BURST2: u32 = 0x01;
            #[doc = "3"]
            pub const REFRESH_BURST3: u32 = 0x02;
            #[doc = "4"]
            pub const REFRESH_BURST4: u32 = 0x03;
            #[doc = "5"]
            pub const REFRESH_BURST5: u32 = 0x04;
            #[doc = "6"]
            pub const REFRESH_BURST6: u32 = 0x05;
            #[doc = "7"]
            pub const REFRESH_BURST7: u32 = 0x06;
            #[doc = "8"]
            pub const REFRESH_BURST8: u32 = 0x07;
        }
    }
    #[doc = "Prescaler period"]
    pub mod PRESCALE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(256*16+1) clock cycles"]
            pub const PRESCALE_256X16PLUS1: u32 = 0;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_1: u32 = 0x01;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_2: u32 = 0x02;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_3: u32 = 0x03;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_4: u32 = 0x04;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_5: u32 = 0x05;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_6: u32 = 0x06;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_7: u32 = 0x07;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_8: u32 = 0x08;
            #[doc = "(PRESCALE*16+1) clock cycles"]
            pub const PRESCALE_16PLUS1_9: u32 = 0x09;
        }
    }
    #[doc = "Refresh timer period"]
    pub mod RT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "(256+1)*(Prescaler period)"]
            pub const RT_256PLUS1XPRESCALE: u32 = 0;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_1: u32 = 0x01;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_2: u32 = 0x02;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_3: u32 = 0x03;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_4: u32 = 0x04;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_5: u32 = 0x05;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_6: u32 = 0x06;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_7: u32 = 0x07;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_8: u32 = 0x08;
            #[doc = "(RT+1)*(Prescaler period)"]
            pub const RT_RTPLUS1XPRESCALE_9: u32 = 0x09;
        }
    }
    #[doc = "Urgent refresh threshold"]
    pub mod UT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "256*(Prescaler period)"]
            pub const PRESCALEX256: u32 = 0;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_1: u32 = 0x01;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_2: u32 = 0x02;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_3: u32 = 0x03;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_4: u32 = 0x04;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_5: u32 = 0x05;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_6: u32 = 0x06;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_7: u32 = 0x07;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_8: u32 = 0x08;
            #[doc = "UT*(Prescaler period)"]
            pub const PRESCALEXUT_9: u32 = 0x09;
        }
    }
}
#[doc = "NAND Control Register 0"]
pub mod NANDCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_8BIT: u32 = 0;
            #[doc = "16bit"]
            pub const PS_16BIT: u32 = 0x01;
        }
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous mode is enabled."]
            pub const SYNC: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BURST1: u32 = 0;
            #[doc = "2"]
            pub const BURST2: u32 = 0x01;
            #[doc = "4"]
            pub const BURST4: u32 = 0x02;
            #[doc = "8"]
            pub const BURST8: u32 = 0x03;
            #[doc = "16"]
            pub const BURST16: u32 = 0x04;
            #[doc = "32"]
            pub const BURST32: u32 = 0x05;
            #[doc = "64"]
            pub const BURST64_6: u32 = 0x06;
            #[doc = "64"]
            pub const BURST64_7: u32 = 0x07;
        }
    }
    #[doc = "EDO mode enabled"]
    pub mod EDO {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "EDO mode disabled"]
            pub const EDO_DISABLE: u32 = 0;
            #[doc = "EDO mode enabled"]
            pub const EDO_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Column address bit number"]
    pub mod COL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "16"]
            pub const BIT16: u32 = 0;
            #[doc = "15"]
            pub const BIT15: u32 = 0x01;
            #[doc = "14"]
            pub const BIT14: u32 = 0x02;
            #[doc = "13"]
            pub const BIT13: u32 = 0x03;
            #[doc = "12"]
            pub const BIT12: u32 = 0x04;
            #[doc = "11"]
            pub const BIT11: u32 = 0x05;
            #[doc = "10"]
            pub const BIT10: u32 = 0x06;
            #[doc = "9"]
            pub const BIT9: u32 = 0x07;
        }
    }
    #[doc = "NAND buffer enable for AXI access"]
    pub mod BUFEN {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "AXI access to NAND device directly"]
            pub const AXI_DIRECT: u32 = 0;
            #[doc = "AXI access through NAND buffer. It must be enabled for error correction schemes."]
            pub const AXI_BUFFER: u32 = 0x01;
        }
    }
    #[doc = "ECC mode selection"]
    pub mod ECC_MODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No correction, ECC bypass"]
            pub const NO_ECC: u32 = 0;
            #[doc = "4-error correction (8 ECC bytes)"]
            pub const ECC_8BYTE: u32 = 0x01;
            #[doc = "8-error correction (16 ECC bytes)"]
            pub const ECC_16BYTE: u32 = 0x02;
        }
    }
    #[doc = "Sector numbers in NAND buffer"]
    pub mod SECTOR_NUM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "There is 1 sector in buffer"]
            pub const SECTOR1: u32 = 0;
            #[doc = "There are 2 sectors in buffer"]
            pub const SECTOR2: u32 = 0x01;
            #[doc = "There are 4 sectors in buffer"]
            pub const SECTOR4: u32 = 0x02;
        }
    }
    #[doc = "Size in bytes of one elementary unit of ECC correction."]
    pub mod SECTOR_SIZE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND Control Register 1"]
pub mod NANDCR1 {
    #[doc = "CE# setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE# low time"]
    pub mod WEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE# high time"]
    pub mod WEH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE# low time"]
    pub mod REL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE# high time"]
    pub mod REH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND Control Register 2"]
pub mod NANDCR2 {
    #[doc = "WE# high to RE# low time"]
    pub mod TWHR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE# high to WE# low time"]
    pub mod TRHW {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address cycle to data loading time"]
    pub mod TADL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ready to RE# low time"]
    pub mod TRR {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE# high to busy time"]
    pub mod TWB {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND Control Register 3"]
pub mod NANDCR3 {
    #[doc = "NAND option bit 1"]
    pub mod NDOPT1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAND option bit 2"]
    pub mod NDOPT2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAND option bit 3"]
    pub mod NDOPT3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NAND CLE Option"]
    pub mod CLE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Data Setup time"]
    pub mod RDS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read Data Hold time"]
    pub mod RDH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Setup time"]
    pub mod WDS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data Hold time"]
    pub mod WDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NOR Control Register 0"]
pub mod NORCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_8BIT: u32 = 0;
            #[doc = "16bit"]
            pub const PS_16BIT: u32 = 0x01;
        }
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
            pub const SYNC: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BURST1: u32 = 0;
            #[doc = "2"]
            pub const BURST2: u32 = 0x01;
            #[doc = "4"]
            pub const BURST4: u32 = 0x02;
            #[doc = "8"]
            pub const BURST8: u32 = 0x03;
            #[doc = "16"]
            pub const BURST16: u32 = 0x04;
            #[doc = "32"]
            pub const BURST32: u32 = 0x05;
            #[doc = "64"]
            pub const BURST64_6: u32 = 0x06;
            #[doc = "64"]
            pub const BURST64_7: u32 = 0x07;
        }
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address/Data MUX mode (ADMUX)"]
            pub const ADMUX: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode (AADM)"]
            pub const AADM: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const NON_ADMUX_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const NON_ADMUX_3: u32 = 0x03;
        }
    }
    #[doc = "ADV# Polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "ADV# is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is high during address hold state."]
            pub const HIGH: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const LOW: u32 = 0x01;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const BITWIDTH12_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const BITWIDTH11: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const BITWIDTH10: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const BITWIDTH9: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const BITWIDTH8: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const BITWIDTH7: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const BITWIDTH6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const BITWIDTH5: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const BITWIDTH4: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const BITWIDTH3: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const BITWIDTH2: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_B: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_C: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_D: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_E: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_F: u32 = 0x0f;
        }
    }
}
#[doc = "NOR Control Register 1"]
pub mod NORCR1 {
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE low time"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE high time"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE low time"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE high time"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NOR Control Register 2"]
pub mod NORCR2 {
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address to write data hold time"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NOR Control Register 3"]
pub mod NORCR3 {
    #[doc = "Address setup time for SYNC read"]
    pub mod ASSR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time for SYNC read"]
    pub mod AHSR {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Control Register 0"]
pub mod SRAMCR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_8BIT: u32 = 0;
            #[doc = "16bit"]
            pub const PS_16BIT: u32 = 0x01;
        }
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
            pub const SYNC: u32 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
    pub mod WAITEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The SEMC does not monitor wait pin."]
            pub const NO_MONITOR_WAIT: u32 = 0;
            #[doc = "The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted."]
            pub const MONITOR_WAIT: u32 = 0x01;
        }
    }
    #[doc = "Wait Sample"]
    pub mod WAITSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wait pin is directly used by the SEMC."]
            pub const WAIT_DIRECT: u32 = 0;
            #[doc = "Wait pin is sampled by internal clock before it is used."]
            pub const WAIT_SAMPLED: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BURST1: u32 = 0;
            #[doc = "2"]
            pub const BURST2: u32 = 0x01;
            #[doc = "4"]
            pub const BURST4: u32 = 0x02;
            #[doc = "8"]
            pub const BURST8: u32 = 0x03;
            #[doc = "16"]
            pub const BURST16: u32 = 0x04;
            #[doc = "32"]
            pub const BURST32: u32 = 0x05;
            #[doc = "64"]
            pub const BURST64_6: u32 = 0x06;
            #[doc = "64"]
            pub const BURST64_7: u32 = 0x07;
        }
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address/Data MUX mode (ADMUX)"]
            pub const ADMUX: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode (AADM)"]
            pub const AADM: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const NON_ADMUX_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const NON_ADMUX_3: u32 = 0x03;
        }
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "ADV# is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is high during address hold state."]
            pub const HIGH_HOLD: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const LOW_HOLD: u32 = 0x01;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const BITWIDTH12_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const BITWIDTH11: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const BITWIDTH10: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const BITWIDTH9: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const BITWIDTH8: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const BITWIDTH7: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const BITWIDTH6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const BITWIDTH5: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const BITWIDTH4: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const BITWIDTH3: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const BITWIDTH2: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_B: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_C: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_D: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_E: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_F: u32 = 0x0f;
        }
    }
}
#[doc = "SRAM Control Register 1"]
pub mod SRAMCR1 {
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE low time"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE high time"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE low time"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE high time"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Control Register 2"]
pub mod SRAMCR2 {
    #[doc = "Write Data setup time"]
    pub mod WDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data hold time"]
    pub mod WDH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address to write data hold time"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DBI-B Control Register 0"]
pub mod DBICR0 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_8BIT: u32 = 0;
            #[doc = "16bit"]
            pub const PS_16BIT: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BURST1: u32 = 0;
            #[doc = "2"]
            pub const BURST2: u32 = 0x01;
            #[doc = "4"]
            pub const BURST4: u32 = 0x02;
            #[doc = "8"]
            pub const BURST8: u32 = 0x03;
            #[doc = "16"]
            pub const BURST16: u32 = 0x04;
            #[doc = "32"]
            pub const BURST32: u32 = 0x05;
            #[doc = "64"]
            pub const BURST64_6: u32 = 0x06;
            #[doc = "64"]
            pub const BURST64_7: u32 = 0x07;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const BITWIDTH12_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const BITWIDTH11: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const BITWIDTH10: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const BITWIDTH9: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const BITWIDTH8: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const BITWIDTH7: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const BITWIDTH6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const BITWIDTH5: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const BITWIDTH4: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const BITWIDTH3: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const BITWIDTH2: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_B: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_C: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_D: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_E: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_F: u32 = 0x0f;
        }
    }
}
#[doc = "DBI-B Control Register 1"]
pub mod DBICR1 {
    #[doc = "CSX Setup Time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CSX Hold Time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WRX Low Time"]
    pub mod WEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WRX High Time"]
    pub mod WEH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RDX Low Time"]
    pub mod REL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RDX High Time"]
    pub mod REH {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DBI-B Control Register 2"]
pub mod DBICR2 {
    #[doc = "CSX interval time"]
    pub mod CEITV {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Control Register 0"]
pub mod IPCR0 {
    #[doc = "Slave address"]
    pub mod SA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Control Register 1"]
pub mod IPCR1 {
    #[doc = "Data Size in Byte"]
    pub mod DATSZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4"]
            pub const DATASZ_4BYTE: u32 = 0;
            #[doc = "1"]
            pub const DATASZ_1BYTE: u32 = 0x01;
            #[doc = "2"]
            pub const DATASZ_2BYTE: u32 = 0x02;
            #[doc = "3"]
            pub const DATASZ_3BYTE: u32 = 0x03;
            #[doc = "4"]
            pub const DATASZ_4BYTE_4: u32 = 0x04;
            #[doc = "4"]
            pub const DATASZ_4BYTE_5: u32 = 0x05;
            #[doc = "4"]
            pub const DATASZ_4BYTE_6: u32 = 0x06;
            #[doc = "4"]
            pub const DATASZ_4BYTE_7: u32 = 0x07;
        }
    }
    #[doc = "NAND Extended Address"]
    pub mod NAND_EXT_ADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "IP Command Control Register 2"]
pub mod IPCR2 {
    #[doc = "Byte Mask for Byte 0 (IPTXDAT bit 7:0)"]
    pub mod BM0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte is unmasked"]
            pub const UNMASKED: u32 = 0;
            #[doc = "Byte is masked"]
            pub const MASKED: u32 = 0x01;
        }
    }
    #[doc = "Byte Mask for Byte 1 (IPTXDAT bit 15:8)"]
    pub mod BM1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte is unmasked"]
            pub const UNMASKED: u32 = 0;
            #[doc = "Byte is masked"]
            pub const MASKED: u32 = 0x01;
        }
    }
    #[doc = "Byte Mask for Byte 2 (IPTXDAT bit 23:16)"]
    pub mod BM2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte is unmasked"]
            pub const UNMASKED: u32 = 0;
            #[doc = "Byte is masked"]
            pub const MASKED: u32 = 0x01;
        }
    }
    #[doc = "Byte Mask for Byte 3 (IPTXDAT bit 31:24)"]
    pub mod BM3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Byte is unmasked"]
            pub const UNMASKED: u32 = 0;
            #[doc = "Byte is masked"]
            pub const MASKED: u32 = 0x01;
        }
    }
}
#[doc = "IP Command Register"]
pub mod IPCMD {
    #[doc = "SDRAM Commands: 0x5: Extended Mode Register Set 0x6: Deep Power Down 0x8: Read 0x9: Write 0xA: Mode Register Set 0xB: Active 0xC: Auto Refresh 0xD: Self Refresh 0xE: Precharge 0xF: Precharge All Others: Reserved Self Refresh is sent to all SDRAM devices because they share the same SEMC_CLK pin"]
    pub mod CMD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field should be written with 0xA55A when trigging an IP command for all device types"]
    pub mod KEY {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "TX DATA Register"]
pub mod IPTXDAT {
    #[doc = "Data value to use for an IP write command"]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "RX DATA Register"]
pub mod IPRXDAT {
    #[doc = "Data returned by device for an IP read command."]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 0"]
pub mod STS0 {
    #[doc = "Indicating whether the SEMC is in idle state."]
    pub mod IDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicating NAND device Ready/WAIT# pin level."]
    pub mod NARDY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "NAND device is not ready"]
            pub const NOTREADY: u32 = 0;
            #[doc = "NAND device is ready"]
            pub const READY: u32 = 0x01;
        }
    }
}
#[doc = "Status Register 2"]
pub mod STS2 {
    #[doc = "This field indicating whether there is pending AXI command (write) to NAND device."]
    pub mod NDWRPEND {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No pending"]
            pub const NO_PENDING: u32 = 0;
            #[doc = "Pending"]
            pub const PENDING: u32 = 0x01;
        }
    }
}
#[doc = "Status Register 12"]
pub mod STS12 {
    #[doc = "This field indicating the last write address (AXI command) to NAND device (without base address in SEMC_BR4)."]
    pub mod NDADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status Register 13"]
pub mod STS13 {
    #[doc = "Sample clock slave delay line locked."]
    pub mod SLVLOCK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slave delay line is not locked."]
            pub const NOT_LOCKED: u32 = 0;
            #[doc = "Slave delay line is locked."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Sample clock reference delay line locked."]
    pub mod REFLOCK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Reference delay line is not locked."]
            pub const NOT_LOCKED: u32 = 0;
            #[doc = "Reference delay line is locked."]
            pub const LOCKED: u32 = 0x01;
        }
    }
    #[doc = "Sample clock slave delay line delay cell number selection."]
    pub mod SLVSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sample clock reference delay line delay cell number selection."]
    pub mod REFSEL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 9"]
pub mod BR9 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const INVALID: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const SIZE_4KB: u32 = 0;
            #[doc = "8KB"]
            pub const SIZE_8KB: u32 = 0x01;
            #[doc = "16KB"]
            pub const SIZE_16KB: u32 = 0x02;
            #[doc = "32KB"]
            pub const SIZE_32KB: u32 = 0x03;
            #[doc = "64KB"]
            pub const SIZE_64KB: u32 = 0x04;
            #[doc = "128KB"]
            pub const SIZE_128KB: u32 = 0x05;
            #[doc = "256KB"]
            pub const SIZE_256KB: u32 = 0x06;
            #[doc = "512KB"]
            pub const SIZE_512KB: u32 = 0x07;
            #[doc = "1MB"]
            pub const SIZE_1MB: u32 = 0x08;
            #[doc = "2MB"]
            pub const SIZE_2MB: u32 = 0x09;
            #[doc = "4MB"]
            pub const SIZE_4MB: u32 = 0x0a;
            #[doc = "8MB"]
            pub const SIZE_8MB: u32 = 0x0b;
            #[doc = "16MB"]
            pub const SIZE_16MB: u32 = 0x0c;
            #[doc = "32MB"]
            pub const SIZE_32MB: u32 = 0x0d;
            #[doc = "64MB"]
            pub const SIZE_64MB: u32 = 0x0e;
            #[doc = "128MB"]
            pub const SIZE_128MB: u32 = 0x0f;
            #[doc = "256MB"]
            pub const SIZE_256MB: u32 = 0x10;
            #[doc = "512MB"]
            pub const SIZE_512MB: u32 = 0x11;
            #[doc = "1GB"]
            pub const SIZE_1GB: u32 = 0x12;
            #[doc = "2GB"]
            pub const SIZE_2GB: u32 = 0x13;
            #[doc = "4GB"]
            pub const SIZE_4GB_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const SIZE_4GB_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const SIZE_4GB_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const SIZE_4GB_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const SIZE_4GB_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const SIZE_4GB_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const SIZE_4GB_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const SIZE_4GB_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const SIZE_4GB_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const SIZE_4GB_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const SIZE_4GB_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const SIZE_4GB_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 10"]
pub mod BR10 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const INVALID: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const SIZE_4KB: u32 = 0;
            #[doc = "8KB"]
            pub const SIZE_8KB: u32 = 0x01;
            #[doc = "16KB"]
            pub const SIZE_16KB: u32 = 0x02;
            #[doc = "32KB"]
            pub const SIZE_32KB: u32 = 0x03;
            #[doc = "64KB"]
            pub const SIZE_64KB: u32 = 0x04;
            #[doc = "128KB"]
            pub const SIZE_128KB: u32 = 0x05;
            #[doc = "256KB"]
            pub const SIZE_256KB: u32 = 0x06;
            #[doc = "512KB"]
            pub const SIZE_512KB: u32 = 0x07;
            #[doc = "1MB"]
            pub const SIZE_1MB: u32 = 0x08;
            #[doc = "2MB"]
            pub const SIZE_2MB: u32 = 0x09;
            #[doc = "4MB"]
            pub const SIZE_4MB: u32 = 0x0a;
            #[doc = "8MB"]
            pub const SIZE_8MB: u32 = 0x0b;
            #[doc = "16MB"]
            pub const SIZE_16MB: u32 = 0x0c;
            #[doc = "32MB"]
            pub const SIZE_32MB: u32 = 0x0d;
            #[doc = "64MB"]
            pub const SIZE_64MB: u32 = 0x0e;
            #[doc = "128MB"]
            pub const SIZE_128MB: u32 = 0x0f;
            #[doc = "256MB"]
            pub const SIZE_256MB: u32 = 0x10;
            #[doc = "512MB"]
            pub const SIZE_512MB: u32 = 0x11;
            #[doc = "1GB"]
            pub const SIZE_1GB: u32 = 0x12;
            #[doc = "2GB"]
            pub const SIZE_2GB: u32 = 0x13;
            #[doc = "4GB"]
            pub const SIZE_4GB_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const SIZE_4GB_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const SIZE_4GB_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const SIZE_4GB_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const SIZE_4GB_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const SIZE_4GB_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const SIZE_4GB_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const SIZE_4GB_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const SIZE_4GB_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const SIZE_4GB_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const SIZE_4GB_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const SIZE_4GB_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Base Register 11"]
pub mod BR11 {
    #[doc = "Valid"]
    pub mod VLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The memory is invalid, can not be accessed."]
            pub const INVALID: u32 = 0;
            #[doc = "The memory is valid, can be accessed."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Memory size"]
    pub mod MS {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "4KB"]
            pub const SIZE_4KB: u32 = 0;
            #[doc = "8KB"]
            pub const SIZE_8KB: u32 = 0x01;
            #[doc = "16KB"]
            pub const SIZE_16KB: u32 = 0x02;
            #[doc = "32KB"]
            pub const SIZE_32KB: u32 = 0x03;
            #[doc = "64KB"]
            pub const SIZE_64KB: u32 = 0x04;
            #[doc = "128KB"]
            pub const SIZE_128KB: u32 = 0x05;
            #[doc = "256KB"]
            pub const SIZE_256KB: u32 = 0x06;
            #[doc = "512KB"]
            pub const SIZE_512KB: u32 = 0x07;
            #[doc = "1MB"]
            pub const SIZE_1MB: u32 = 0x08;
            #[doc = "2MB"]
            pub const SIZE_2MB: u32 = 0x09;
            #[doc = "4MB"]
            pub const SIZE_4MB: u32 = 0x0a;
            #[doc = "8MB"]
            pub const SIZE_8MB: u32 = 0x0b;
            #[doc = "16MB"]
            pub const SIZE_16MB: u32 = 0x0c;
            #[doc = "32MB"]
            pub const SIZE_32MB: u32 = 0x0d;
            #[doc = "64MB"]
            pub const SIZE_64MB: u32 = 0x0e;
            #[doc = "128MB"]
            pub const SIZE_128MB: u32 = 0x0f;
            #[doc = "256MB"]
            pub const SIZE_256MB: u32 = 0x10;
            #[doc = "512MB"]
            pub const SIZE_512MB: u32 = 0x11;
            #[doc = "1GB"]
            pub const SIZE_1GB: u32 = 0x12;
            #[doc = "2GB"]
            pub const SIZE_2GB: u32 = 0x13;
            #[doc = "4GB"]
            pub const SIZE_4GB_20: u32 = 0x14;
            #[doc = "4GB"]
            pub const SIZE_4GB_21: u32 = 0x15;
            #[doc = "4GB"]
            pub const SIZE_4GB_22: u32 = 0x16;
            #[doc = "4GB"]
            pub const SIZE_4GB_23: u32 = 0x17;
            #[doc = "4GB"]
            pub const SIZE_4GB_24: u32 = 0x18;
            #[doc = "4GB"]
            pub const SIZE_4GB_25: u32 = 0x19;
            #[doc = "4GB"]
            pub const SIZE_4GB_26: u32 = 0x1a;
            #[doc = "4GB"]
            pub const SIZE_4GB_27: u32 = 0x1b;
            #[doc = "4GB"]
            pub const SIZE_4GB_28: u32 = 0x1c;
            #[doc = "4GB"]
            pub const SIZE_4GB_29: u32 = 0x1d;
            #[doc = "4GB"]
            pub const SIZE_4GB_30: u32 = 0x1e;
            #[doc = "4GB"]
            pub const SIZE_4GB_31: u32 = 0x1f;
        }
    }
    #[doc = "Base Address"]
    pub mod BA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x000f_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Control Register 4"]
pub mod SRAMCR4 {
    #[doc = "Port Size"]
    pub mod PS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8bit"]
            pub const PS_8BIT: u32 = 0;
            #[doc = "16bit"]
            pub const PS_16BIT: u32 = 0x01;
        }
    }
    #[doc = "Synchronous Mode Enable"]
    pub mod SYNCEN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asynchronous mode is enabled."]
            pub const ASYNC: u32 = 0;
            #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
            pub const SYNC: u32 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
    pub mod WAITEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The SEMC does not monitor wait pin."]
            pub const NO_MONITOR_WAIT: u32 = 0;
            #[doc = "The SEMC monitors wait pin. The SEMC does not transfer/receive data when wait pin is asserted."]
            pub const MONITOR_WAIT: u32 = 0x01;
        }
    }
    #[doc = "Wait Sample"]
    pub mod WAITSP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Wait pin is directly used by the SEMC."]
            pub const WAIT_DIRECT: u32 = 0;
            #[doc = "Wait pin is sampled by internal clock before it is used."]
            pub const WAIT_SAMPLED: u32 = 0x01;
        }
    }
    #[doc = "Burst Length"]
    pub mod BL {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const BURST1: u32 = 0;
            #[doc = "2"]
            pub const BURST2: u32 = 0x01;
            #[doc = "4"]
            pub const BURST4: u32 = 0x02;
            #[doc = "8"]
            pub const BURST8: u32 = 0x03;
            #[doc = "16"]
            pub const BURST16: u32 = 0x04;
            #[doc = "32"]
            pub const BURST32: u32 = 0x05;
            #[doc = "64"]
            pub const BURST64_6: u32 = 0x06;
            #[doc = "64"]
            pub const BURST64_7: u32 = 0x07;
        }
    }
    #[doc = "Address Mode"]
    pub mod AM {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address/Data MUX mode (ADMUX)"]
            pub const ADMUX: u32 = 0;
            #[doc = "Advanced Address/Data MUX mode (AADM)"]
            pub const AADM: u32 = 0x01;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const NON_ADMUX_2: u32 = 0x02;
            #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
            pub const NON_ADMUX_3: u32 = 0x03;
        }
    }
    #[doc = "ADV# polarity"]
    pub mod ADVP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is active low."]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "ADV# is active high."]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "ADV# level control during address hold state"]
    pub mod ADVH {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADV# is high during address hold state."]
            pub const HIGH_HOLD: u32 = 0;
            #[doc = "ADV# is low during address hold state."]
            pub const LOW_HOLD: u32 = 0x01;
        }
    }
    #[doc = "Column Address bit width"]
    pub mod COL {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "12 Bits"]
            pub const BITWIDTH12_0: u32 = 0;
            #[doc = "11 Bits"]
            pub const BITWIDTH11: u32 = 0x01;
            #[doc = "10 Bits"]
            pub const BITWIDTH10: u32 = 0x02;
            #[doc = "9 Bits"]
            pub const BITWIDTH9: u32 = 0x03;
            #[doc = "8 Bits"]
            pub const BITWIDTH8: u32 = 0x04;
            #[doc = "7 Bits"]
            pub const BITWIDTH7: u32 = 0x05;
            #[doc = "6 Bits"]
            pub const BITWIDTH6: u32 = 0x06;
            #[doc = "5 Bits"]
            pub const BITWIDTH5: u32 = 0x07;
            #[doc = "4 Bits"]
            pub const BITWIDTH4: u32 = 0x08;
            #[doc = "3 Bits"]
            pub const BITWIDTH3: u32 = 0x09;
            #[doc = "2 Bits"]
            pub const BITWIDTH2: u32 = 0x0a;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_B: u32 = 0x0b;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_C: u32 = 0x0c;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_D: u32 = 0x0d;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_E: u32 = 0x0e;
            #[doc = "12 Bits"]
            pub const BITWIDTH12_F: u32 = 0x0f;
        }
    }
}
#[doc = "SRAM Control Register 5"]
pub mod SRAMCR5 {
    #[doc = "CE setup time"]
    pub mod CES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE hold time"]
    pub mod CEH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address setup time"]
    pub mod AS {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address hold time"]
    pub mod AH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE low time"]
    pub mod WEL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "WE high time"]
    pub mod WEH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE low time"]
    pub mod REL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RE high time"]
    pub mod REH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SRAM Control Register 6"]
pub mod SRAMCR6 {
    #[doc = "Write Data setup time"]
    pub mod WDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Write Data hold time"]
    pub mod WDH {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Turnaround time"]
    pub mod TA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address to write data hold time"]
    pub mod AWDH {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Latency count"]
    pub mod LC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read time"]
    pub mod RD {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CE# interval time"]
    pub mod CEITV {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read hold time"]
    pub mod RDH {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND Buffer DATA Register"]
pub mod NDBD {
    #[doc = "NAND Buffer data. It is used for program or read operation from IPS bus."]
    pub mod DAT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "NAND Buffer Address Register"]
pub mod NDBA {
    #[doc = "NAND Buffer address. It is used for program or read operation from IPS bus. It should be configured to proper value before access to NDBD register."]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Delay Chain Control Register"]
pub mod DCCR {
    #[doc = "Delay chain insertion enable for SRAM device."]
    pub mod SDRAMEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Delay chain is not inserted."]
            pub const NO_DLY_CHAIN: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const DLY_CHAIN: u32 = 0x01;
        }
    }
    #[doc = "Clock delay line delay cell number selection value for SDRAM device."]
    pub mod SDRAMVAL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay chain insertion enable for NOR device."]
    pub mod NOREN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Delay chain is not inserted."]
            pub const NO_DLY_CHAIN: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const DLY_CHAIN: u32 = 0x01;
        }
    }
    #[doc = "Clock delay line delay cell number selection value for NOR device."]
    pub mod NORVAL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay chain insertion enable for SRAM device 0."]
    pub mod SRAM0EN {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Delay chain is not inserted."]
            pub const NO_DLY_CHAIN: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const DLY_CHAIN: u32 = 0x01;
        }
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 0."]
    pub mod SRAM0VAL {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delay chain insertion enable for SRAM device 1-3."]
    pub mod SRAMXEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Delay chain is not inserted."]
            pub const NO_DLY_CHAIN: u32 = 0;
            #[doc = "Delay chain is inserted."]
            pub const DLY_CHAIN: u32 = 0x01;
        }
    }
    #[doc = "Clock delay line delay cell number selection value for SRAM device 1-3."]
    pub mod SRAMXVAL {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SDRAM Prefetch Control Register"]
pub mod SDRAMPCR {
    #[doc = "SDRAM prefetch enable."]
    pub mod PREFETCH_EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SDRAM prefetch function is disabled."]
            pub const PREFETCH_DISABLE: u32 = 0;
            #[doc = "SDRAM prefetch function is enabled."]
            pub const PREFETCH_ENABLE: u32 = 0x01;
        }
    }
    #[doc = "SDRAM prefetch delay cycle."]
    pub mod PREFETCH_DLY {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
