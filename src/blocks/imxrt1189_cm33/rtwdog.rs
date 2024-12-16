#[doc = "WDOG"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "WDOG Control and Status"]
    pub CS: crate::RWRegister<u32>,
    #[doc = "WDOG Counter"]
    pub CNT: crate::RWRegister<u32>,
    #[doc = "WDOG Timeout Value"]
    pub TOVAL: crate::RWRegister<u32>,
    #[doc = "Watchdog Window"]
    pub WIN: crate::RWRegister<u32>,
}
#[doc = "WDOG Control and Status"]
pub mod CS {
    #[doc = "Stop Enable"]
    pub mod STOP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Wait Enable"]
    pub mod WAIT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG Test"]
    pub mod TST {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable WDOG Test mode"]
            pub const DIS: u32 = 0;
            #[doc = "Enable WDOG User mode"]
            pub const EN: u32 = 0x01;
            #[doc = "Enable WDOG Test mode"]
            pub const ENABLES_2: u32 = 0x02;
            #[doc = "Enable WDOG Test mode"]
            pub const ENABLES_3: u32 = 0x03;
        }
    }
    #[doc = "Updates Allowed"]
    pub mod UPDATE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Updates not allowed"]
            pub const DIS: u32 = 0;
            #[doc = "Updates allowed"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG Interrupt"]
    pub mod INT {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG Enable"]
    pub mod EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG Clock"]
    pub mod CLK {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Reconfiguration Success"]
    pub mod RCS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Unsuccessful"]
            pub const RECONFIG: u32 = 0;
            #[doc = "Successful"]
            pub const SUCCESS: u32 = 0x01;
        }
    }
    #[doc = "Unlock Status"]
    pub mod ULK {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Locked"]
            pub const LOCK: u32 = 0;
            #[doc = "Unlocked"]
            pub const UNLOCK: u32 = 0x01;
        }
    }
    #[doc = "WDOG Prescaler"]
    pub mod PRES {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "Command 32 Enable"]
    pub mod CMD32EN {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
    #[doc = "WDOG Interrupt Flag"]
    pub mod FLG {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No interrupt occurred"]
            pub const NO: u32 = 0;
            #[doc = "An interrupt occurred"]
            pub const YES: u32 = 0x01;
        }
    }
    #[doc = "WDOG Window"]
    pub mod WIN {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DIS: u32 = 0;
            #[doc = "Enable"]
            pub const EN: u32 = 0x01;
        }
    }
}
#[doc = "WDOG Counter"]
pub mod CNT {
    #[doc = "Counter High Byte"]
    pub mod CNTLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counter Low Byte"]
    pub mod CNTHIGH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "WDOG Timeout Value"]
pub mod TOVAL {
    #[doc = "Timeout Value Low"]
    pub mod TOVALLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Value High"]
    pub mod TOVALHIGH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Watchdog Window"]
pub mod WIN {
    #[doc = "Low Byte"]
    pub mod WINLOW {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Byte"]
    pub mod WINHIGH {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
