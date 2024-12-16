#[doc = "TMR"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Timer Channel Compare Register 1"]
    pub COMP10: crate::RWRegister<u16>,
    #[doc = "Timer Channel Compare Register 2"]
    pub COMP20: crate::RWRegister<u16>,
    #[doc = "Timer Channel Capture Register"]
    pub CAPT0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Load Register"]
    pub LOAD0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Hold Register"]
    pub HOLD0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Counter Register"]
    pub CNTR0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Control Register"]
    pub CTRL0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Status and Control Register"]
    pub SCTRL0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 1"]
    pub CMPLD10: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 2"]
    pub CMPLD20: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Status and Control Register"]
    pub CSCTRL0: crate::RWRegister<u16>,
    #[doc = "Timer Channel Input Filter Register"]
    pub FILT0: crate::RWRegister<u16>,
    #[doc = "Timer Channel DMA Enable Register"]
    pub DMA0: crate::RWRegister<u16>,
    _reserved0: [u8; 0x04],
    #[doc = "Timer Channel Enable Register"]
    pub ENBL: crate::RWRegister<u16>,
    #[doc = "Timer Channel Compare Register 1"]
    pub COMP11: crate::RWRegister<u16>,
    #[doc = "Timer Channel Compare Register 2"]
    pub COMP21: crate::RWRegister<u16>,
    #[doc = "Timer Channel Capture Register"]
    pub CAPT1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Load Register"]
    pub LOAD1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Hold Register"]
    pub HOLD1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Counter Register"]
    pub CNTR1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Control Register"]
    pub CTRL1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Status and Control Register"]
    pub SCTRL1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 1"]
    pub CMPLD11: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 2"]
    pub CMPLD21: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Status and Control Register"]
    pub CSCTRL1: crate::RWRegister<u16>,
    #[doc = "Timer Channel Input Filter Register"]
    pub FILT1: crate::RWRegister<u16>,
    #[doc = "Timer Channel DMA Enable Register"]
    pub DMA1: crate::RWRegister<u16>,
    _reserved1: [u8; 0x06],
    #[doc = "Timer Channel Compare Register 1"]
    pub COMP12: crate::RWRegister<u16>,
    #[doc = "Timer Channel Compare Register 2"]
    pub COMP22: crate::RWRegister<u16>,
    #[doc = "Timer Channel Capture Register"]
    pub CAPT2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Load Register"]
    pub LOAD2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Hold Register"]
    pub HOLD2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Counter Register"]
    pub CNTR2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Control Register"]
    pub CTRL2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Status and Control Register"]
    pub SCTRL2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 1"]
    pub CMPLD12: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 2"]
    pub CMPLD22: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Status and Control Register"]
    pub CSCTRL2: crate::RWRegister<u16>,
    #[doc = "Timer Channel Input Filter Register"]
    pub FILT2: crate::RWRegister<u16>,
    #[doc = "Timer Channel DMA Enable Register"]
    pub DMA2: crate::RWRegister<u16>,
    _reserved2: [u8; 0x06],
    #[doc = "Timer Channel Compare Register 1"]
    pub COMP13: crate::RWRegister<u16>,
    #[doc = "Timer Channel Compare Register 2"]
    pub COMP23: crate::RWRegister<u16>,
    #[doc = "Timer Channel Capture Register"]
    pub CAPT3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Load Register"]
    pub LOAD3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Hold Register"]
    pub HOLD3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Counter Register"]
    pub CNTR3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Control Register"]
    pub CTRL3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Status and Control Register"]
    pub SCTRL3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 1"]
    pub CMPLD13: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Load Register 2"]
    pub CMPLD23: crate::RWRegister<u16>,
    #[doc = "Timer Channel Comparator Status and Control Register"]
    pub CSCTRL3: crate::RWRegister<u16>,
    #[doc = "Timer Channel Input Filter Register"]
    pub FILT3: crate::RWRegister<u16>,
    #[doc = "Timer Channel DMA Enable Register"]
    pub DMA3: crate::RWRegister<u16>,
}
#[doc = "Timer Channel Compare Register 1"]
pub mod COMP10 {
    #[doc = "Comparison Value 1"]
    pub mod COMPARISON_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Compare Register 2"]
pub mod COMP20 {
    #[doc = "Comparison Value 2"]
    pub mod COMPARISON_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Capture Register"]
pub mod CAPT0 {
    #[doc = "Capture Value"]
    pub mod CAPTURE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Load Register"]
pub mod LOAD0 {
    #[doc = "Timer Load Register"]
    pub mod LOAD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Hold Register"]
pub mod HOLD0 {
    #[doc = "HOLD"]
    pub mod HOLD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Counter Register"]
pub mod CNTR0 {
    #[doc = "COUNTER"]
    pub mod COUNTER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Control Register"]
pub mod CTRL0 {
    #[doc = "Output Mode"]
    pub mod OUTMODE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asserted while counter is active"]
            pub const COUNTER_ACTIVE: u16 = 0;
            #[doc = "Clear OFLAG output on successful compare"]
            pub const CLEAR_OFLAG: u16 = 0x01;
            #[doc = "Set OFLAG output on successful compare"]
            pub const SET_OFLAG: u16 = 0x02;
            #[doc = "Toggle OFLAG output on successful compare"]
            pub const TOGGLE_OFLAG_SUCCESS: u16 = 0x03;
            #[doc = "Toggle OFLAG output using alternating compare registers"]
            pub const TOGGLE_OFLAG_ALT: u16 = 0x04;
            #[doc = "Set on compare, cleared on secondary source input edge"]
            pub const CLEAR_ON_SECONDARY: u16 = 0x05;
            #[doc = "Set on compare, cleared on counter rollover"]
            pub const CLEAR_ON_ROLLOVER: u16 = 0x06;
            #[doc = "Enable gated clock output while counter is active"]
            pub const ENABLE_GATED_OUT: u16 = 0x07;
        }
    }
    #[doc = "Co-Channel Initialization"]
    pub mod COINIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
            pub const DISABLE: u16 = 0;
            #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Count Direction"]
    pub mod DIR {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count up."]
            pub const COUNTUP: u16 = 0;
            #[doc = "Count down."]
            pub const COUNTDOWN: u16 = 0x01;
        }
    }
    #[doc = "Count Length"]
    pub mod LENGTH {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count until roll over at $FFFF and then continue by re-initializing the counter from the LOAD register."]
            pub const UNTIL_ROLLOVER: u16 = 0;
            #[doc = "Count until compare, then re-initialize using the LOAD register. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Count Once"]
    pub mod ONCE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count repeatedly."]
            pub const REPEAT: u16 = 0;
            #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Secondary Count Source"]
    pub mod SCS {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
        }
    }
    #[doc = "Primary Count Source"]
    pub mod PCS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
            #[doc = "Counter 0 output"]
            pub const COUNTER0_OUT: u16 = 0x04;
            #[doc = "Counter 1 output"]
            pub const COUNTER1_OUT: u16 = 0x05;
            #[doc = "Counter 2 output"]
            pub const COUNTER2_OUT: u16 = 0x06;
            #[doc = "Counter 3 output"]
            pub const COUNTER3_OUT: u16 = 0x07;
            #[doc = "IP bus clock divide by 1 prescaler"]
            pub const BUS_DIVBY1: u16 = 0x08;
            #[doc = "IP bus clock divide by 2 prescaler"]
            pub const BUS_DIVBY2: u16 = 0x09;
            #[doc = "IP bus clock divide by 4 prescaler"]
            pub const BUS_DIVBY4: u16 = 0x0a;
            #[doc = "IP bus clock divide by 8 prescaler"]
            pub const BUS_DIVBY8: u16 = 0x0b;
            #[doc = "IP bus clock divide by 16 prescaler"]
            pub const BUS_DIVBY16: u16 = 0x0c;
            #[doc = "IP bus clock divide by 32 prescaler"]
            pub const BUS_DIVBY32: u16 = 0x0d;
            #[doc = "IP bus clock divide by 64 prescaler"]
            pub const BUS_DIVBY64: u16 = 0x0e;
            #[doc = "IP bus clock divide by 128 prescaler"]
            pub const BUS_DIVBY128: u16 = 0x0f;
        }
    }
    #[doc = "Count Mode"]
    pub mod CM {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NOOP: u16 = 0;
            #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
            pub const RISING_ONLY: u16 = 0x01;
            #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
            pub const RISING_AND_FALLING: u16 = 0x02;
            #[doc = "Count rising edges of primary source while secondary input high active"]
            pub const RISING_WHILE_SEC_HIGH: u16 = 0x03;
            #[doc = "Quadrature count mode, uses primary and secondary sources"]
            pub const QUADRATURE: u16 = 0x04;
            #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
            pub const RISING_SEC_DIR: u16 = 0x05;
            #[doc = "Edge of secondary source triggers primary count until compare"]
            pub const SECONDARY: u16 = 0x06;
            #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
            pub const CASCADE: u16 = 0x07;
        }
    }
}
#[doc = "Timer Channel Status and Control Register"]
pub mod SCTRL0 {
    #[doc = "Output Enable"]
    pub mod OEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The external pin is configured as an input."]
            pub const INPUT: u16 = 0;
            #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
            pub const OFLAG_OUT: u16 = 0x01;
        }
    }
    #[doc = "Output Polarity Select"]
    pub mod OPS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "True polarity."]
            pub const TRUE: u16 = 0;
            #[doc = "Inverted polarity."]
            pub const INVERTED: u16 = 0x01;
        }
    }
    #[doc = "Force OFLAG Output"]
    pub mod FORCE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forced OFLAG Value"]
    pub mod VAL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable External OFLAG Force"]
    pub mod EEOF {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Master Mode"]
    pub mod MSTR {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Mode"]
    pub mod CAPTURE_MODE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Capture function is disabled"]
            pub const DISABLED: u16 = 0;
            #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
            pub const ENABLE_RISING: u16 = 0x01;
            #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
            pub const ENABLE_FALLING: u16 = 0x02;
            #[doc = "Load capture register on both edges of input"]
            pub const ENABLE_BOTH: u16 = 0x03;
        }
    }
    #[doc = "External Input Signal"]
    pub mod INPUT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Polarity Select"]
    pub mod IPS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    pub mod IEFIE {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag"]
    pub mod IEF {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    pub mod TOFIE {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag"]
    pub mod TOF {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    pub mod TCFIE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag"]
    pub mod TCF {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod CMPLD10 {
    #[doc = "COMPARATOR_LOAD_1"]
    pub mod COMPARATOR_LOAD_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod CMPLD20 {
    #[doc = "COMPARATOR_LOAD_2"]
    pub mod COMPARATOR_LOAD_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod CSCTRL0 {
    #[doc = "Compare Load Control 1"]
    pub mod CL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Compare Load Control 2"]
    pub mod CL2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    pub mod TCF1 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    pub mod TCF2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    pub mod TCF1EN {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    pub mod TCF2EN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counting Direction Indicator"]
    pub mod UP {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The last count was in the DOWN direction."]
            pub const DOWN: u16 = 0;
            #[doc = "The last count was in the UP direction."]
            pub const UP: u16 = 0x01;
        }
    }
    #[doc = "Triggered Count Initialization Control"]
    pub mod TCI {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const STOP: u16 = 0;
            #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const RELOAD: u16 = 0x01;
        }
    }
    #[doc = "Reload on Capture"]
    pub mod ROC {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Alternative Load Enable"]
    pub mod ALT_LOAD {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter can be re-initialized only with the LOAD register."]
            pub const DISABLE: u16 = 0;
            #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Fault Enable"]
    pub mod FAULT {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Debug Actions Enable"]
    pub mod DBG_EN {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue with normal operation during debug mode. (default)"]
            pub const NORMAL: u16 = 0;
            #[doc = "Halt TMR counter during debug mode."]
            pub const HALT_TMR: u16 = 0x01;
            #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
            pub const FORCE_0: u16 = 0x02;
            #[doc = "Both halt counter and force output to 0 during debug mode."]
            pub const HALT_AND_FORCE_0: u16 = 0x03;
        }
    }
}
#[doc = "Timer Channel Input Filter Register"]
pub mod FILT0 {
    #[doc = "Input Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel DMA Enable Register"]
pub mod DMA0 {
    #[doc = "Input Edge Flag DMA Enable"]
    pub mod IEFDE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    pub mod CMPLD1DE {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    pub mod CMPLD2DE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Enable Register"]
pub mod ENBL {
    #[doc = "Timer Channel Enable"]
    pub mod ENBL {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables the timer channel."]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables the timer channel. (default)"]
            pub const ENABLE: u16 = 0x01;
        }
    }
}
#[doc = "Timer Channel Compare Register 1"]
pub mod COMP11 {
    #[doc = "Comparison Value 1"]
    pub mod COMPARISON_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Compare Register 2"]
pub mod COMP21 {
    #[doc = "Comparison Value 2"]
    pub mod COMPARISON_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Capture Register"]
pub mod CAPT1 {
    #[doc = "Capture Value"]
    pub mod CAPTURE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Load Register"]
pub mod LOAD1 {
    #[doc = "Timer Load Register"]
    pub mod LOAD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Hold Register"]
pub mod HOLD1 {
    #[doc = "HOLD"]
    pub mod HOLD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Counter Register"]
pub mod CNTR1 {
    #[doc = "COUNTER"]
    pub mod COUNTER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Control Register"]
pub mod CTRL1 {
    #[doc = "Output Mode"]
    pub mod OUTMODE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asserted while counter is active"]
            pub const COUNTER_ACTIVE: u16 = 0;
            #[doc = "Clear OFLAG output on successful compare"]
            pub const CLEAR_OFLAG: u16 = 0x01;
            #[doc = "Set OFLAG output on successful compare"]
            pub const SET_OFLAG: u16 = 0x02;
            #[doc = "Toggle OFLAG output on successful compare"]
            pub const TOGGLE_OFLAG_SUCCESS: u16 = 0x03;
            #[doc = "Toggle OFLAG output using alternating compare registers"]
            pub const TOGGLE_OFLAG_ALT: u16 = 0x04;
            #[doc = "Set on compare, cleared on secondary source input edge"]
            pub const CLEAR_ON_SECONDARY: u16 = 0x05;
            #[doc = "Set on compare, cleared on counter rollover"]
            pub const CLEAR_ON_ROLLOVER: u16 = 0x06;
            #[doc = "Enable gated clock output while counter is active"]
            pub const ENABLE_GATED_OUT: u16 = 0x07;
        }
    }
    #[doc = "Co-Channel Initialization"]
    pub mod COINIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
            pub const DISABLE: u16 = 0;
            #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Count Direction"]
    pub mod DIR {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count up."]
            pub const COUNTUP: u16 = 0;
            #[doc = "Count down."]
            pub const COUNTDOWN: u16 = 0x01;
        }
    }
    #[doc = "Count Length"]
    pub mod LENGTH {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count until roll over at $FFFF and then continue by re-initializing the counter from the LOAD register."]
            pub const UNTIL_ROLLOVER: u16 = 0;
            #[doc = "Count until compare, then re-initialize using the LOAD register. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Count Once"]
    pub mod ONCE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count repeatedly."]
            pub const REPEAT: u16 = 0;
            #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Secondary Count Source"]
    pub mod SCS {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
        }
    }
    #[doc = "Primary Count Source"]
    pub mod PCS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
            #[doc = "Counter 0 output"]
            pub const COUNTER0_OUT: u16 = 0x04;
            #[doc = "Counter 1 output"]
            pub const COUNTER1_OUT: u16 = 0x05;
            #[doc = "Counter 2 output"]
            pub const COUNTER2_OUT: u16 = 0x06;
            #[doc = "Counter 3 output"]
            pub const COUNTER3_OUT: u16 = 0x07;
            #[doc = "IP bus clock divide by 1 prescaler"]
            pub const BUS_DIVBY1: u16 = 0x08;
            #[doc = "IP bus clock divide by 2 prescaler"]
            pub const BUS_DIVBY2: u16 = 0x09;
            #[doc = "IP bus clock divide by 4 prescaler"]
            pub const BUS_DIVBY4: u16 = 0x0a;
            #[doc = "IP bus clock divide by 8 prescaler"]
            pub const BUS_DIVBY8: u16 = 0x0b;
            #[doc = "IP bus clock divide by 16 prescaler"]
            pub const BUS_DIVBY16: u16 = 0x0c;
            #[doc = "IP bus clock divide by 32 prescaler"]
            pub const BUS_DIVBY32: u16 = 0x0d;
            #[doc = "IP bus clock divide by 64 prescaler"]
            pub const BUS_DIVBY64: u16 = 0x0e;
            #[doc = "IP bus clock divide by 128 prescaler"]
            pub const BUS_DIVBY128: u16 = 0x0f;
        }
    }
    #[doc = "Count Mode"]
    pub mod CM {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NOOP: u16 = 0;
            #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
            pub const RISING_ONLY: u16 = 0x01;
            #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
            pub const RISING_AND_FALLING: u16 = 0x02;
            #[doc = "Count rising edges of primary source while secondary input high active"]
            pub const RISING_WHILE_SEC_HIGH: u16 = 0x03;
            #[doc = "Quadrature count mode, uses primary and secondary sources"]
            pub const QUADRATURE: u16 = 0x04;
            #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
            pub const RISING_SEC_DIR: u16 = 0x05;
            #[doc = "Edge of secondary source triggers primary count until compare"]
            pub const SECONDARY: u16 = 0x06;
            #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
            pub const CASCADE: u16 = 0x07;
        }
    }
}
#[doc = "Timer Channel Status and Control Register"]
pub mod SCTRL1 {
    #[doc = "Output Enable"]
    pub mod OEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The external pin is configured as an input."]
            pub const INPUT: u16 = 0;
            #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
            pub const OFLAG_OUT: u16 = 0x01;
        }
    }
    #[doc = "Output Polarity Select"]
    pub mod OPS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "True polarity."]
            pub const TRUE: u16 = 0;
            #[doc = "Inverted polarity."]
            pub const INVERTED: u16 = 0x01;
        }
    }
    #[doc = "Force OFLAG Output"]
    pub mod FORCE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forced OFLAG Value"]
    pub mod VAL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable External OFLAG Force"]
    pub mod EEOF {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Master Mode"]
    pub mod MSTR {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Mode"]
    pub mod CAPTURE_MODE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Capture function is disabled"]
            pub const DISABLED: u16 = 0;
            #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
            pub const ENABLE_RISING: u16 = 0x01;
            #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
            pub const ENABLE_FALLING: u16 = 0x02;
            #[doc = "Load capture register on both edges of input"]
            pub const ENABLE_BOTH: u16 = 0x03;
        }
    }
    #[doc = "External Input Signal"]
    pub mod INPUT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Polarity Select"]
    pub mod IPS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    pub mod IEFIE {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag"]
    pub mod IEF {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    pub mod TOFIE {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag"]
    pub mod TOF {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    pub mod TCFIE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag"]
    pub mod TCF {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod CMPLD11 {
    #[doc = "COMPARATOR_LOAD_1"]
    pub mod COMPARATOR_LOAD_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod CMPLD21 {
    #[doc = "COMPARATOR_LOAD_2"]
    pub mod COMPARATOR_LOAD_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod CSCTRL1 {
    #[doc = "Compare Load Control 1"]
    pub mod CL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Compare Load Control 2"]
    pub mod CL2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    pub mod TCF1 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    pub mod TCF2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    pub mod TCF1EN {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    pub mod TCF2EN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counting Direction Indicator"]
    pub mod UP {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The last count was in the DOWN direction."]
            pub const DOWN: u16 = 0;
            #[doc = "The last count was in the UP direction."]
            pub const UP: u16 = 0x01;
        }
    }
    #[doc = "Triggered Count Initialization Control"]
    pub mod TCI {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const STOP: u16 = 0;
            #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const RELOAD: u16 = 0x01;
        }
    }
    #[doc = "Reload on Capture"]
    pub mod ROC {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Alternative Load Enable"]
    pub mod ALT_LOAD {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter can be re-initialized only with the LOAD register."]
            pub const DISABLE: u16 = 0;
            #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Fault Enable"]
    pub mod FAULT {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Debug Actions Enable"]
    pub mod DBG_EN {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue with normal operation during debug mode. (default)"]
            pub const NORMAL: u16 = 0;
            #[doc = "Halt TMR counter during debug mode."]
            pub const HALT_TMR: u16 = 0x01;
            #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
            pub const FORCE_0: u16 = 0x02;
            #[doc = "Both halt counter and force output to 0 during debug mode."]
            pub const HALT_AND_FORCE_0: u16 = 0x03;
        }
    }
}
#[doc = "Timer Channel Input Filter Register"]
pub mod FILT1 {
    #[doc = "Input Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel DMA Enable Register"]
pub mod DMA1 {
    #[doc = "Input Edge Flag DMA Enable"]
    pub mod IEFDE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    pub mod CMPLD1DE {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    pub mod CMPLD2DE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Compare Register 1"]
pub mod COMP12 {
    #[doc = "Comparison Value 1"]
    pub mod COMPARISON_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Compare Register 2"]
pub mod COMP22 {
    #[doc = "Comparison Value 2"]
    pub mod COMPARISON_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Capture Register"]
pub mod CAPT2 {
    #[doc = "Capture Value"]
    pub mod CAPTURE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Load Register"]
pub mod LOAD2 {
    #[doc = "Timer Load Register"]
    pub mod LOAD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Hold Register"]
pub mod HOLD2 {
    #[doc = "HOLD"]
    pub mod HOLD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Counter Register"]
pub mod CNTR2 {
    #[doc = "COUNTER"]
    pub mod COUNTER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Control Register"]
pub mod CTRL2 {
    #[doc = "Output Mode"]
    pub mod OUTMODE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asserted while counter is active"]
            pub const COUNTER_ACTIVE: u16 = 0;
            #[doc = "Clear OFLAG output on successful compare"]
            pub const CLEAR_OFLAG: u16 = 0x01;
            #[doc = "Set OFLAG output on successful compare"]
            pub const SET_OFLAG: u16 = 0x02;
            #[doc = "Toggle OFLAG output on successful compare"]
            pub const TOGGLE_OFLAG_SUCCESS: u16 = 0x03;
            #[doc = "Toggle OFLAG output using alternating compare registers"]
            pub const TOGGLE_OFLAG_ALT: u16 = 0x04;
            #[doc = "Set on compare, cleared on secondary source input edge"]
            pub const CLEAR_ON_SECONDARY: u16 = 0x05;
            #[doc = "Set on compare, cleared on counter rollover"]
            pub const CLEAR_ON_ROLLOVER: u16 = 0x06;
            #[doc = "Enable gated clock output while counter is active"]
            pub const ENABLE_GATED_OUT: u16 = 0x07;
        }
    }
    #[doc = "Co-Channel Initialization"]
    pub mod COINIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
            pub const DISABLE: u16 = 0;
            #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Count Direction"]
    pub mod DIR {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count up."]
            pub const COUNTUP: u16 = 0;
            #[doc = "Count down."]
            pub const COUNTDOWN: u16 = 0x01;
        }
    }
    #[doc = "Count Length"]
    pub mod LENGTH {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count until roll over at $FFFF and then continue by re-initializing the counter from the LOAD register."]
            pub const UNTIL_ROLLOVER: u16 = 0;
            #[doc = "Count until compare, then re-initialize using the LOAD register. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Count Once"]
    pub mod ONCE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count repeatedly."]
            pub const REPEAT: u16 = 0;
            #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Secondary Count Source"]
    pub mod SCS {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
        }
    }
    #[doc = "Primary Count Source"]
    pub mod PCS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
            #[doc = "Counter 0 output"]
            pub const COUNTER0_OUT: u16 = 0x04;
            #[doc = "Counter 1 output"]
            pub const COUNTER1_OUT: u16 = 0x05;
            #[doc = "Counter 2 output"]
            pub const COUNTER2_OUT: u16 = 0x06;
            #[doc = "Counter 3 output"]
            pub const COUNTER3_OUT: u16 = 0x07;
            #[doc = "IP bus clock divide by 1 prescaler"]
            pub const BUS_DIVBY1: u16 = 0x08;
            #[doc = "IP bus clock divide by 2 prescaler"]
            pub const BUS_DIVBY2: u16 = 0x09;
            #[doc = "IP bus clock divide by 4 prescaler"]
            pub const BUS_DIVBY4: u16 = 0x0a;
            #[doc = "IP bus clock divide by 8 prescaler"]
            pub const BUS_DIVBY8: u16 = 0x0b;
            #[doc = "IP bus clock divide by 16 prescaler"]
            pub const BUS_DIVBY16: u16 = 0x0c;
            #[doc = "IP bus clock divide by 32 prescaler"]
            pub const BUS_DIVBY32: u16 = 0x0d;
            #[doc = "IP bus clock divide by 64 prescaler"]
            pub const BUS_DIVBY64: u16 = 0x0e;
            #[doc = "IP bus clock divide by 128 prescaler"]
            pub const BUS_DIVBY128: u16 = 0x0f;
        }
    }
    #[doc = "Count Mode"]
    pub mod CM {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NOOP: u16 = 0;
            #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
            pub const RISING_ONLY: u16 = 0x01;
            #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
            pub const RISING_AND_FALLING: u16 = 0x02;
            #[doc = "Count rising edges of primary source while secondary input high active"]
            pub const RISING_WHILE_SEC_HIGH: u16 = 0x03;
            #[doc = "Quadrature count mode, uses primary and secondary sources"]
            pub const QUADRATURE: u16 = 0x04;
            #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
            pub const RISING_SEC_DIR: u16 = 0x05;
            #[doc = "Edge of secondary source triggers primary count until compare"]
            pub const SECONDARY: u16 = 0x06;
            #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
            pub const CASCADE: u16 = 0x07;
        }
    }
}
#[doc = "Timer Channel Status and Control Register"]
pub mod SCTRL2 {
    #[doc = "Output Enable"]
    pub mod OEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The external pin is configured as an input."]
            pub const INPUT: u16 = 0;
            #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
            pub const OFLAG_OUT: u16 = 0x01;
        }
    }
    #[doc = "Output Polarity Select"]
    pub mod OPS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "True polarity."]
            pub const TRUE: u16 = 0;
            #[doc = "Inverted polarity."]
            pub const INVERTED: u16 = 0x01;
        }
    }
    #[doc = "Force OFLAG Output"]
    pub mod FORCE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forced OFLAG Value"]
    pub mod VAL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable External OFLAG Force"]
    pub mod EEOF {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Master Mode"]
    pub mod MSTR {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Mode"]
    pub mod CAPTURE_MODE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Capture function is disabled"]
            pub const DISABLED: u16 = 0;
            #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
            pub const ENABLE_RISING: u16 = 0x01;
            #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
            pub const ENABLE_FALLING: u16 = 0x02;
            #[doc = "Load capture register on both edges of input"]
            pub const ENABLE_BOTH: u16 = 0x03;
        }
    }
    #[doc = "External Input Signal"]
    pub mod INPUT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Polarity Select"]
    pub mod IPS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    pub mod IEFIE {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag"]
    pub mod IEF {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    pub mod TOFIE {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag"]
    pub mod TOF {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    pub mod TCFIE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag"]
    pub mod TCF {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod CMPLD12 {
    #[doc = "COMPARATOR_LOAD_1"]
    pub mod COMPARATOR_LOAD_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod CMPLD22 {
    #[doc = "COMPARATOR_LOAD_2"]
    pub mod COMPARATOR_LOAD_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod CSCTRL2 {
    #[doc = "Compare Load Control 1"]
    pub mod CL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Compare Load Control 2"]
    pub mod CL2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    pub mod TCF1 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    pub mod TCF2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    pub mod TCF1EN {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    pub mod TCF2EN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counting Direction Indicator"]
    pub mod UP {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The last count was in the DOWN direction."]
            pub const DOWN: u16 = 0;
            #[doc = "The last count was in the UP direction."]
            pub const UP: u16 = 0x01;
        }
    }
    #[doc = "Triggered Count Initialization Control"]
    pub mod TCI {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const STOP: u16 = 0;
            #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const RELOAD: u16 = 0x01;
        }
    }
    #[doc = "Reload on Capture"]
    pub mod ROC {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Alternative Load Enable"]
    pub mod ALT_LOAD {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter can be re-initialized only with the LOAD register."]
            pub const DISABLE: u16 = 0;
            #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Fault Enable"]
    pub mod FAULT {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Debug Actions Enable"]
    pub mod DBG_EN {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue with normal operation during debug mode. (default)"]
            pub const NORMAL: u16 = 0;
            #[doc = "Halt TMR counter during debug mode."]
            pub const HALT_TMR: u16 = 0x01;
            #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
            pub const FORCE_0: u16 = 0x02;
            #[doc = "Both halt counter and force output to 0 during debug mode."]
            pub const HALT_AND_FORCE_0: u16 = 0x03;
        }
    }
}
#[doc = "Timer Channel Input Filter Register"]
pub mod FILT2 {
    #[doc = "Input Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel DMA Enable Register"]
pub mod DMA2 {
    #[doc = "Input Edge Flag DMA Enable"]
    pub mod IEFDE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    pub mod CMPLD1DE {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    pub mod CMPLD2DE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Compare Register 1"]
pub mod COMP13 {
    #[doc = "Comparison Value 1"]
    pub mod COMPARISON_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Compare Register 2"]
pub mod COMP23 {
    #[doc = "Comparison Value 2"]
    pub mod COMPARISON_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Capture Register"]
pub mod CAPT3 {
    #[doc = "Capture Value"]
    pub mod CAPTURE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Load Register"]
pub mod LOAD3 {
    #[doc = "Timer Load Register"]
    pub mod LOAD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Hold Register"]
pub mod HOLD3 {
    #[doc = "HOLD"]
    pub mod HOLD {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Counter Register"]
pub mod CNTR3 {
    #[doc = "COUNTER"]
    pub mod COUNTER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Control Register"]
pub mod CTRL3 {
    #[doc = "Output Mode"]
    pub mod OUTMODE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Asserted while counter is active"]
            pub const COUNTER_ACTIVE: u16 = 0;
            #[doc = "Clear OFLAG output on successful compare"]
            pub const CLEAR_OFLAG: u16 = 0x01;
            #[doc = "Set OFLAG output on successful compare"]
            pub const SET_OFLAG: u16 = 0x02;
            #[doc = "Toggle OFLAG output on successful compare"]
            pub const TOGGLE_OFLAG_SUCCESS: u16 = 0x03;
            #[doc = "Toggle OFLAG output using alternating compare registers"]
            pub const TOGGLE_OFLAG_ALT: u16 = 0x04;
            #[doc = "Set on compare, cleared on secondary source input edge"]
            pub const CLEAR_ON_SECONDARY: u16 = 0x05;
            #[doc = "Set on compare, cleared on counter rollover"]
            pub const CLEAR_ON_ROLLOVER: u16 = 0x06;
            #[doc = "Enable gated clock output while counter is active"]
            pub const ENABLE_GATED_OUT: u16 = 0x07;
        }
    }
    #[doc = "Co-Channel Initialization"]
    pub mod COINIT {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Co-channel counter/timers cannot force a re-initialization of this counter/timer"]
            pub const DISABLE: u16 = 0;
            #[doc = "Co-channel counter/timers may force a re-initialization of this counter/timer"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Count Direction"]
    pub mod DIR {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count up."]
            pub const COUNTUP: u16 = 0;
            #[doc = "Count down."]
            pub const COUNTDOWN: u16 = 0x01;
        }
    }
    #[doc = "Count Length"]
    pub mod LENGTH {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count until roll over at $FFFF and then continue by re-initializing the counter from the LOAD register."]
            pub const UNTIL_ROLLOVER: u16 = 0;
            #[doc = "Count until compare, then re-initialize using the LOAD register. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, alternating values of COMP1 and COMP2 are used to generate successful comparisons. For example, the counter counts until a COMP1 value is reached, re-initializes, counts until COMP2 value is reached, re-initializes, counts until COMP1 value is reached, and so on."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Count Once"]
    pub mod ONCE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Count repeatedly."]
            pub const REPEAT: u16 = 0;
            #[doc = "Count until compare and then stop. If counting up, a successful compare occurs when the counter reaches a COMP1 value. If counting down, a successful compare occurs when the counter reaches a COMP2 value. When output mode $4 is used, the counter re-initializes after reaching the COMP1 value, continues to count to the COMP2 value, and then stops."]
            pub const UNTIL_COMPARE: u16 = 0x01;
        }
    }
    #[doc = "Secondary Count Source"]
    pub mod SCS {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
        }
    }
    #[doc = "Primary Count Source"]
    pub mod PCS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter 0 input pin"]
            pub const COUNTER0_IN: u16 = 0;
            #[doc = "Counter 1 input pin"]
            pub const COUNTER1_IN: u16 = 0x01;
            #[doc = "Counter 2 input pin"]
            pub const COUNTER2_IN: u16 = 0x02;
            #[doc = "Counter 3 input pin"]
            pub const COUNTER3_IN: u16 = 0x03;
            #[doc = "Counter 0 output"]
            pub const COUNTER0_OUT: u16 = 0x04;
            #[doc = "Counter 1 output"]
            pub const COUNTER1_OUT: u16 = 0x05;
            #[doc = "Counter 2 output"]
            pub const COUNTER2_OUT: u16 = 0x06;
            #[doc = "Counter 3 output"]
            pub const COUNTER3_OUT: u16 = 0x07;
            #[doc = "IP bus clock divide by 1 prescaler"]
            pub const BUS_DIVBY1: u16 = 0x08;
            #[doc = "IP bus clock divide by 2 prescaler"]
            pub const BUS_DIVBY2: u16 = 0x09;
            #[doc = "IP bus clock divide by 4 prescaler"]
            pub const BUS_DIVBY4: u16 = 0x0a;
            #[doc = "IP bus clock divide by 8 prescaler"]
            pub const BUS_DIVBY8: u16 = 0x0b;
            #[doc = "IP bus clock divide by 16 prescaler"]
            pub const BUS_DIVBY16: u16 = 0x0c;
            #[doc = "IP bus clock divide by 32 prescaler"]
            pub const BUS_DIVBY32: u16 = 0x0d;
            #[doc = "IP bus clock divide by 64 prescaler"]
            pub const BUS_DIVBY64: u16 = 0x0e;
            #[doc = "IP bus clock divide by 128 prescaler"]
            pub const BUS_DIVBY128: u16 = 0x0f;
        }
    }
    #[doc = "Count Mode"]
    pub mod CM {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No operation"]
            pub const NOOP: u16 = 0;
            #[doc = "Count rising edges of primary sourceRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1. If the primary count source is IP bus clock divide by 1, only rising edges are counted regardless of the value of SCTRL\\[IPS\\]."]
            pub const RISING_ONLY: u16 = 0x01;
            #[doc = "Count rising and falling edges of primary sourceIP bus clock divide by 1 cannot be used as a primary count source in edge count mode."]
            pub const RISING_AND_FALLING: u16 = 0x02;
            #[doc = "Count rising edges of primary source while secondary input high active"]
            pub const RISING_WHILE_SEC_HIGH: u16 = 0x03;
            #[doc = "Quadrature count mode, uses primary and secondary sources"]
            pub const QUADRATURE: u16 = 0x04;
            #[doc = "Count rising edges of primary source; secondary source specifies directionRising edges are counted only when SCTRL\\[IPS\\] = 0. Falling edges are counted when SCTRL\\[IPS\\] = 1."]
            pub const RISING_SEC_DIR: u16 = 0x05;
            #[doc = "Edge of secondary source triggers primary count until compare"]
            pub const SECONDARY: u16 = 0x06;
            #[doc = "Cascaded counter mode (up/down)The primary count source must be set to one of the counter outputs."]
            pub const CASCADE: u16 = 0x07;
        }
    }
}
#[doc = "Timer Channel Status and Control Register"]
pub mod SCTRL3 {
    #[doc = "Output Enable"]
    pub mod OEN {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The external pin is configured as an input."]
            pub const INPUT: u16 = 0;
            #[doc = "The OFLAG output signal is driven on the external pin. Other timer groups using this external pin as their input see the driven value. The polarity of the signal is determined by OPS."]
            pub const OFLAG_OUT: u16 = 0x01;
        }
    }
    #[doc = "Output Polarity Select"]
    pub mod OPS {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "True polarity."]
            pub const TRUE: u16 = 0;
            #[doc = "Inverted polarity."]
            pub const INVERTED: u16 = 0x01;
        }
    }
    #[doc = "Force OFLAG Output"]
    pub mod FORCE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Forced OFLAG Value"]
    pub mod VAL {
        pub const offset: u16 = 3;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Enable External OFLAG Force"]
    pub mod EEOF {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Master Mode"]
    pub mod MSTR {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Capture Mode"]
    pub mod CAPTURE_MODE {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Capture function is disabled"]
            pub const DISABLED: u16 = 0;
            #[doc = "Load capture register on rising edge (when IPS=0) or falling edge (when IPS=1) of input"]
            pub const ENABLE_RISING: u16 = 0x01;
            #[doc = "Load capture register on falling edge (when IPS=0) or rising edge (when IPS=1) of input"]
            pub const ENABLE_FALLING: u16 = 0x02;
            #[doc = "Load capture register on both edges of input"]
            pub const ENABLE_BOTH: u16 = 0x03;
        }
    }
    #[doc = "External Input Signal"]
    pub mod INPUT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Polarity Select"]
    pub mod IPS {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag Interrupt Enable"]
    pub mod IEFIE {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Edge Flag"]
    pub mod IEF {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag Interrupt Enable"]
    pub mod TOFIE {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Overflow Flag"]
    pub mod TOF {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag Interrupt Enable"]
    pub mod TCFIE {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare Flag"]
    pub mod TCF {
        pub const offset: u16 = 15;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 1"]
pub mod CMPLD13 {
    #[doc = "COMPARATOR_LOAD_1"]
    pub mod COMPARATOR_LOAD_1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Load Register 2"]
pub mod CMPLD23 {
    #[doc = "COMPARATOR_LOAD_2"]
    pub mod COMPARATOR_LOAD_2 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel Comparator Status and Control Register"]
pub mod CSCTRL3 {
    #[doc = "Compare Load Control 1"]
    pub mod CL1 {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Compare Load Control 2"]
    pub mod CL2 {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Never preload"]
            pub const NEVER: u16 = 0;
            #[doc = "Load upon successful compare with the value in COMP1"]
            pub const COMP1: u16 = 0x01;
            #[doc = "Load upon successful compare with the value in COMP2"]
            pub const COMP2: u16 = 0x02;
        }
    }
    #[doc = "Timer Compare 1 Interrupt Flag"]
    pub mod TCF1 {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Flag"]
    pub mod TCF2 {
        pub const offset: u16 = 5;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 1 Interrupt Enable"]
    pub mod TCF1EN {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timer Compare 2 Interrupt Enable"]
    pub mod TCF2EN {
        pub const offset: u16 = 7;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Counting Direction Indicator"]
    pub mod UP {
        pub const offset: u16 = 9;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The last count was in the DOWN direction."]
            pub const DOWN: u16 = 0;
            #[doc = "The last count was in the UP direction."]
            pub const UP: u16 = 0x01;
        }
    }
    #[doc = "Triggered Count Initialization Control"]
    pub mod TCI {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Stop the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const STOP: u16 = 0;
            #[doc = "Reload the counter upon receiving a second trigger event while still counting from the first trigger event."]
            pub const RELOAD: u16 = 0x01;
        }
    }
    #[doc = "Reload on Capture"]
    pub mod ROC {
        pub const offset: u16 = 11;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Alternative Load Enable"]
    pub mod ALT_LOAD {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Counter can be re-initialized only with the LOAD register."]
            pub const DISABLE: u16 = 0;
            #[doc = "Counter can be re-initialized with the LOAD or CMPLD2 registers depending on count direction."]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Fault Enable"]
    pub mod FAULT {
        pub const offset: u16 = 13;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disables"]
            pub const DISABLE: u16 = 0;
            #[doc = "Enables"]
            pub const ENABLE: u16 = 0x01;
        }
    }
    #[doc = "Debug Actions Enable"]
    pub mod DBG_EN {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Continue with normal operation during debug mode. (default)"]
            pub const NORMAL: u16 = 0;
            #[doc = "Halt TMR counter during debug mode."]
            pub const HALT_TMR: u16 = 0x01;
            #[doc = "Force TMR output to logic 0 (prior to consideration of SCTRL\\[OPS\\])."]
            pub const FORCE_0: u16 = 0x02;
            #[doc = "Both halt counter and force output to 0 during debug mode."]
            pub const HALT_AND_FORCE_0: u16 = 0x03;
        }
    }
}
#[doc = "Timer Channel Input Filter Register"]
pub mod FILT3 {
    #[doc = "Input Filter Sample Period"]
    pub mod FILT_PER {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Filter Sample Count"]
    pub mod FILT_CNT {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timer Channel DMA Enable Register"]
pub mod DMA3 {
    #[doc = "Input Edge Flag DMA Enable"]
    pub mod IEFDE {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 1 DMA Enable"]
    pub mod CMPLD1DE {
        pub const offset: u16 = 1;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Preload Register 2 DMA Enable"]
    pub mod CMPLD2DE {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
