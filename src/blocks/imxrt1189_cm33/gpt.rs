#[doc = "GPT"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Prescaler"]
    pub PR: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub SR: crate::RWRegister<u32>,
    #[doc = "Interrupt"]
    pub IR: crate::RWRegister<u32>,
    #[doc = "Output Compare"]
    pub OCR: [crate::RWRegister<u32>; 3usize],
    #[doc = "Input Capture"]
    pub ICR: [crate::RORegister<u32>; 2usize],
    #[doc = "Counter"]
    pub CNT: crate::RORegister<u32>,
}
#[doc = "Control"]
pub mod CR {
    #[doc = "GPT Enable"]
    pub mod EN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "GPT Enable Mode"]
    pub mod ENMOD {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Restart counting from frozen values after GPT is enabled (EN=1)."]
            pub const RESUME_COUNT: u32 = 0;
            #[doc = "Reset counting from 0 after GPT is enabled (EN=1)."]
            pub const ZERO_COUNT: u32 = 0x01;
        }
    }
    #[doc = "GPT Debug Mode Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable in Debug mode"]
            pub const DEBUG_DIS: u32 = 0;
            #[doc = "Enable in Debug mode"]
            pub const DEBUG_EN: u32 = 0x01;
        }
    }
    #[doc = "GPT Wait Mode Enable"]
    pub mod WAITEN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable in Wait mode"]
            pub const WAIT_DIS: u32 = 0;
            #[doc = "Enable in Wait mode"]
            pub const WAIT_EN: u32 = 0x01;
        }
    }
    #[doc = "GPT Doze Mode Enable"]
    pub mod DOZEEN {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable in Doze mode"]
            pub const DOZE_DIS: u32 = 0;
            #[doc = "Enable in Doze mode"]
            pub const DOZE_EN: u32 = 0x01;
        }
    }
    #[doc = "GPT Stop Mode Enable"]
    pub mod STOPEN {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable in Stop mode"]
            pub const STOP_DIS: u32 = 0;
            #[doc = "Enable in Stop mode"]
            pub const STOP_EN: u32 = 0x01;
        }
    }
    #[doc = "Clock Source Select"]
    pub mod CLKSRC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No clock"]
            pub const NO_CLOCK: u32 = 0;
            #[doc = "Peripheral clock (MODULE_CLK)"]
            pub const CLOCK_001: u32 = 0x01;
            #[doc = "High-frequency reference clock (ipg_clk_highfreq)"]
            pub const CLOCK_010: u32 = 0x02;
            #[doc = "External clock"]
            pub const CLOCK_011: u32 = 0x03;
            #[doc = "Low-frequency reference clock (ipg_clk_32k)"]
            pub const CLOCK_100: u32 = 0x04;
        }
    }
    #[doc = "Free-Run or Restart Mode"]
    pub mod FRR {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Restart mode. After a compare event, the counter resets to 0000_0000h and resumes counting."]
            pub const RESTART: u32 = 0;
            #[doc = "Free-Run mode. After a compare event, the counter continues counting until FFFF_FFFFh and then rolls over to 0."]
            pub const FREE_RUN: u32 = 0x01;
        }
    }
    #[doc = "Enable Oscillator Clock Input"]
    pub mod EN_24M {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Software Reset"]
    pub mod SWR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "GPT is not in software reset state"]
            pub const NOT_SWRESET: u32 = 0;
            #[doc = "GPT is in software reset state"]
            pub const SWRESET: u32 = 0x01;
        }
    }
    #[doc = "Input Capture Operating Mode for Channel 1"]
    pub mod IM1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Operating Mode for Channel 2"]
    pub mod IM2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Operating Mode for Channel 1"]
    pub mod OM1 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Operating Mode for Channel 2"]
    pub mod OM2 {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Operating Mode for Channel 3"]
    pub mod OM3 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Output Compare for Channel 1"]
    pub mod FO1 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Output Compare for Channel 2"]
    pub mod FO2 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Force Output Compare for Channel 3"]
    pub mod FO3 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Prescaler"]
pub mod PR {
    #[doc = "Prescaler Divide Value"]
    pub mod PRESCALER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIV_BY_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIV_BY_2: u32 = 0x01;
            #[doc = "Divide by 4096"]
            pub const DIV_BY_4096: u32 = 0x0fff;
        }
    }
    #[doc = "Prescaler Divide Value for the Oscillator Clock"]
    pub mod PRESCALER24M {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIV_BY_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIV_BY_2: u32 = 0x01;
            #[doc = "Divide by 16"]
            pub const DIV_BY_16: u32 = 0x0f;
        }
    }
}
#[doc = "Status"]
pub mod SR {
    #[doc = "Output Compare Flag for Channel 1"]
    pub mod OF1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Flag for Channel 2"]
    pub mod OF2 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Flag for Channel 3"]
    pub mod OF3 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Flag for Channel 1"]
    pub mod IF1 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Flag for Channel 2"]
    pub mod IF2 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rollover Flag"]
    pub mod ROV {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rollover has not occurred."]
            pub const NO_ROLLOVER: u32 = 0;
            #[doc = "Rollover has occurred."]
            pub const ROLLOVER: u32 = 0x01;
        }
    }
}
#[doc = "Interrupt"]
pub mod IR {
    #[doc = "Output Compare Flag for Channel 1 Interrupt Enable"]
    pub mod OF1IE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Flag for Channel 2 Interrupt Enable"]
    pub mod OF2IE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Compare Flag for Channel 3 Interrupt Enable"]
    pub mod OF3IE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Flag for Channel 1 Interrupt Enable"]
    pub mod IF1IE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Flag for Channel 2 Interrupt Enable"]
    pub mod IF2IE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rollover Interrupt Enable"]
    pub mod ROVIE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLE: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Output Compare"]
pub mod OCR {
    #[doc = "Compare Value"]
    pub mod COMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Input Capture"]
pub mod ICR {
    #[doc = "Capture Value"]
    pub mod CAPT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Counter"]
pub mod CNT {
    #[doc = "Counter Value"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
