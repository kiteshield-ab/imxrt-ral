#[doc = "ACMP"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID Register"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter Register"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "CMP Control 0"]
    pub C0: crate::RWRegister<u32>,
    #[doc = "CMP Control 1"]
    pub C1: crate::RWRegister<u32>,
    #[doc = "CMP Control 2"]
    pub C2: crate::RWRegister<u32>,
    #[doc = "CMP Control 3"]
    pub C3: crate::RWRegister<u32>,
}
#[doc = "Version ID Register"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Minor Version Number"]
    pub mod MINOR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Major Version Number"]
    pub mod MAJOR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Parameter Register"]
pub mod PARAM {
    #[doc = "Parameters"]
    pub mod PARAM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "CMP Control 0"]
pub mod C0 {
    #[doc = "Filter Sample Count"]
    pub mod FILTER_CNT {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Filter is disabled (if C0\\[SE\\] = 1, then COUT is a logic zero (this is not a legal state, and is not recommended); if C0\\[SE\\] = 0, COUT = COUTA)"]
            pub const FILTER_DISABLED: u32 = 0;
            #[doc = "One consecutive sample (comparator output is simply sampled)"]
            pub const SAMPLE_1: u32 = 0x01;
            #[doc = "Two consecutive samples"]
            pub const SAMPLE_2: u32 = 0x02;
            #[doc = "Three consecutive samples"]
            pub const SAMPLE_3: u32 = 0x03;
            #[doc = "Four consecutive samples"]
            pub const SAMPLE_4: u32 = 0x04;
            #[doc = "Five consecutive samples"]
            pub const SAMPLE_5: u32 = 0x05;
            #[doc = "Six consecutive samples"]
            pub const SAMPLE_6: u32 = 0x06;
            #[doc = "Seven consecutive samples"]
            pub const SAMPLE_7: u32 = 0x07;
        }
    }
    #[doc = "Analog Comparator Module Enable"]
    pub mod EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const COMPARATOR_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const COMPARATOR_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Comparator Output Pin Enable"]
    pub mod OPE {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const OUTPUT_NOT_AVAILABLE: u32 = 0;
            #[doc = "Enable"]
            pub const OUTPUT_AVAILABLE: u32 = 0x01;
        }
    }
    #[doc = "Comparator Output Select"]
    pub mod COS {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "COUT"]
            pub const FILTERED: u32 = 0;
            #[doc = "COUTA"]
            pub const UNFILTERED: u32 = 0x01;
        }
    }
    #[doc = "Comparator Invert"]
    pub mod INVT {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not invert"]
            pub const NOT_INVERT: u32 = 0;
            #[doc = "Invert"]
            pub const INVERT: u32 = 0x01;
        }
    }
    #[doc = "Power Mode Select"]
    pub mod PMODE {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low-speed (LS)"]
            pub const LS: u32 = 0;
            #[doc = "High-speed (HS)"]
            pub const HS: u32 = 0x01;
        }
    }
    #[doc = "Windowing Enable"]
    pub mod WE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const NO_WINDOWING_MODE: u32 = 0;
            #[doc = "Enable"]
            pub const WINDOWING_MODE: u32 = 0x01;
        }
    }
    #[doc = "Sample Enable"]
    pub mod SE {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const NO_SAMPLEING_MODE: u32 = 0;
            #[doc = "Enable"]
            pub const SAMPLEING_MODE: u32 = 0x01;
        }
    }
    #[doc = "Filter Sample Period"]
    pub mod FPR {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog Comparator Output"]
    pub mod COUT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog Comparator Flag Falling"]
    pub mod CFF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const NO_FALLING_EDGE: u32 = 0;
            #[doc = "Detected"]
            pub const FALLING_EDGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Analog Comparator Flag Rising"]
    pub mod CFR {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const NO_RISING_EDGE: u32 = 0;
            #[doc = "Detected"]
            pub const RISING_EDGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator Interrupt Enable Falling"]
    pub mod IEF {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const INT_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Comparator Interrupt Enable Rising"]
    pub mod IER {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const INT_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "DMA Enable"]
    pub mod DMAEN {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const INT_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "CMP to DAC Link Enable"]
    pub mod LINKEN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const INT_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const INT_ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "CMP Control 1"]
pub mod C1 {
    #[doc = "DAC Output Voltage Select"]
    pub mod VOSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DAC Mode Select"]
    pub mod DMODE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Low-Speed and Low-Power mode"]
            pub const LOW_SPEED_LOW_POWER: u32 = 0;
            #[doc = "High-Speed and High-Power mode"]
            pub const HIGH_SPEED_HIGH_POWER: u32 = 0x01;
        }
    }
    #[doc = "Supply Voltage Reference Source Select"]
    pub mod VRSEL {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Vin1"]
            pub const VIN1: u32 = 0;
            #[doc = "Vin2"]
            pub const VIN2: u32 = 0x01;
        }
    }
    #[doc = "DAC Enable"]
    pub mod DACEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel 0 Input Enable"]
    pub mod CHN0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CHN0_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CHN0_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel 1 Input Enable"]
    pub mod CHN1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CHN1_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CHN1_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel 2 Input Enable"]
    pub mod CHN2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CHN2_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CHN2_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel 3 Input Enable"]
    pub mod CHN3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CHN3_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CHN3_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel 4 Input Enable"]
    pub mod CHN4 {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CHN4_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CHN4_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Channel 5 Input Enable"]
    pub mod CHN5 {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const CHN5_DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const CHN5_ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Minus Input MUX Control"]
    pub mod MSEL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal negative input 0 for minus channel (internal minus input)"]
            pub const NEGATIVE_INPUT_0: u32 = 0;
            #[doc = "External input 1 for minus channel (reference input 0)"]
            pub const INPUT_1: u32 = 0x01;
            #[doc = "External input 2 for minus channel (reference input 1)"]
            pub const INPUT_2: u32 = 0x02;
            #[doc = "External input 3 for minus channel (reference input 2)"]
            pub const INPUT_3: u32 = 0x03;
            #[doc = "External input 4 for minus channel (reference input 3)"]
            pub const INPUT_4: u32 = 0x04;
            #[doc = "Internal 8-bit DAC output"]
            pub const DAC: u32 = 0x07;
        }
    }
    #[doc = "Plus Input MUX Control"]
    pub mod PSEL {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal positive input 0 for plus channel (internal plus input)"]
            pub const POSITIVE_INPUT_0: u32 = 0;
            #[doc = "External input 1 for plus Channel (reference input 0)"]
            pub const INPUT_1: u32 = 0x01;
            #[doc = "External input 2 for plus channel (reference input 1)"]
            pub const INPUT_2: u32 = 0x02;
            #[doc = "External input 3 for plus channel (reference input 2)"]
            pub const INPUT_3: u32 = 0x03;
            #[doc = "External input 4 for plus channel (reference input 3)"]
            pub const INPUT_4: u32 = 0x04;
            #[doc = "Internal 8-bit DAC output"]
            pub const DAC: u32 = 0x07;
        }
    }
}
#[doc = "CMP Control 2"]
pub mod C2 {
    #[doc = "ACOn"]
    pub mod ACON {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Comparator and DAC Initialization Delay Modulus"]
    pub mod INITMOD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of Sample Clocks"]
    pub mod NSAM {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "As soon as the active channel is scanned in one round-robin clock"]
            pub const VALUE_0: u32 = 0;
            #[doc = "After one round-robin clock cycle"]
            pub const VALUE_1: u32 = 0x01;
            #[doc = "After two round-robin clock cycles"]
            pub const VALUE_2: u32 = 0x02;
            #[doc = "After three round-robin clock cycles"]
            pub const VALUE_3: u32 = 0x03;
        }
    }
    #[doc = "External Channel 0 Input Changed Flag"]
    pub mod CH0F {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Channel 1 Input Changed Flag"]
    pub mod CH1F {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Channel 2 Input Changed Flag"]
    pub mod CH2F {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Channel 3 Input Changed Flag"]
    pub mod CH3F {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Channel 4 Input Changed Flag"]
    pub mod CH4F {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "External Channel 5 Input Changed Flag"]
    pub mod CH5F {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Fixed Channel Select"]
    pub mod FXMXCH {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "External reference input 0"]
            pub const INPUT_0: u32 = 0;
            #[doc = "External reference input 1"]
            pub const INPUT_1: u32 = 0x01;
            #[doc = "External reference input 2"]
            pub const INPUT_2: u32 = 0x02;
            #[doc = "External reference input 3"]
            pub const INPUT_3: u32 = 0x03;
            #[doc = "External reference input 4"]
            pub const INPUT_4: u32 = 0x04;
            #[doc = "External reference input 5"]
            pub const INPUT_5: u32 = 0x05;
            #[doc = "8-bit DAC"]
            pub const DAC: u32 = 0x07;
        }
    }
    #[doc = "Fixed MUX Port"]
    pub mod FXMP {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Fix plus port"]
            pub const FIXED_PLUS_PORT: u32 = 0;
            #[doc = "Fix minus port"]
            pub const FIXED_MINUS_PORT: u32 = 0x01;
        }
    }
    #[doc = "Round-Robin Interrupt Enable"]
    pub mod RRIE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "CMP Control 3"]
pub mod C3 {
    #[doc = "Analog Comparator Phase 2 Timing Control"]
    pub mod ACPH2TC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Phase 2 active time in one sampling period equals to T"]
            pub const VALUE_T: u32 = 0;
            #[doc = "Phase 2 active time in one sampling period equals to 2 * T"]
            pub const VALUE_2T: u32 = 0x01;
            #[doc = "Phase 2 active time in one sampling period equals to 4 * T"]
            pub const VALUE_4T: u32 = 0x02;
            #[doc = "Phase 2 active time in one sampling period equals to 8 * T"]
            pub const VALUE_8T: u32 = 0x03;
            #[doc = "Phase 2 active time in one sampling period equals to 16 * T"]
            pub const VALUE_16T: u32 = 0x04;
            #[doc = "Phase 2 active time in one sampling period equals to 32 * T"]
            pub const VALUE_32T: u32 = 0x05;
            #[doc = "Phase 2 active time in one sampling period equals to 64 * T"]
            pub const VALUE_64T: u32 = 0x06;
            #[doc = "Phase 2 active time in one sampling period equals to 16 * T"]
            pub const VALUE7_16T: u32 = 0x07;
        }
    }
    #[doc = "Analog Comparator Phase 1 Timing Control"]
    pub mod ACPH1TC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Phase 1 active time in one sampling period equals to T"]
            pub const VALUE_T: u32 = 0;
            #[doc = "Phase 1 active time in one sampling period equals to 2 * T"]
            pub const VALUE_2T: u32 = 0x01;
            #[doc = "Phase 1 active time in one sampling period equals to 4 * T"]
            pub const VALUE_4T: u32 = 0x02;
            #[doc = "Phase 1 active time in one sampling period equals to 8 * T"]
            pub const VALUE_8T: u32 = 0x03;
            #[doc = "Phase 1 active time in one sampling period equals to T"]
            pub const VALUE4_T: u32 = 0x04;
            #[doc = "Phase 1 active time in one sampling period equals to T"]
            pub const VALUE5_T: u32 = 0x05;
            #[doc = "Phase 1 active time in one sampling period equals to T"]
            pub const VALUE6_T: u32 = 0x06;
            #[doc = "Phase 1 active time in one sampling period equals to 0"]
            pub const VALUE_0: u32 = 0x07;
        }
    }
    #[doc = "Analog Comparator Sampling Time Control"]
    pub mod ACSAT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The sampling time equals to T"]
            pub const VALUE_T: u32 = 0;
            #[doc = "The sampling time equals to 2 * T"]
            pub const VALUE_2T: u32 = 0x01;
            #[doc = "The sampling time equals to 4 * T"]
            pub const VALUE_4T: u32 = 0x02;
            #[doc = "The sampling time equals to 8 * T"]
            pub const VALUE_8T: u32 = 0x03;
            #[doc = "The sampling time equals to 16 * T"]
            pub const VALUE_16T: u32 = 0x04;
            #[doc = "The sampling time equals to 32 * T"]
            pub const VALUE_32T: u32 = 0x05;
            #[doc = "The sampling time equals to 64 * T"]
            pub const VALUE_64T: u32 = 0x06;
            #[doc = "The sampling time equals to 256 * T"]
            pub const VALUE_256T: u32 = 0x07;
        }
    }
    #[doc = "Discrete Mode Clock Select"]
    pub mod DMCS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Slow clock is selected for the timing generation."]
            pub const SLOW_CLOCK: u32 = 0;
            #[doc = "Fast clock is selected for the timing generation."]
            pub const FAST_CLOCK: u32 = 0x01;
        }
    }
    #[doc = "Resistor Divider Enable"]
    pub mod RDIVE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable even when either NCHEN or PCHEN is set to1 but the actual input is in the range of 0 to 1.8v."]
            pub const NOT_ENABLED: u32 = 0;
            #[doc = "Enable because the inputs are above 1.8v."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Negative Channel Continuous Mode Enable"]
    pub mod NCHCTEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "in Discrete Mode and special timing needs to be configured."]
            pub const DISABLED: u32 = 0;
            #[doc = "in Continuous Mode and no special timing is required."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Positive Channel Continuous Mode Enable"]
    pub mod PCHCTEN {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "in Discrete Mode and special timing needs to be configured"]
            pub const DISABLED: u32 = 0;
            #[doc = "in Continuous Mode and no special timing is required"]
            pub const CONTINUOUS_MODE: u32 = 0x01;
        }
    }
}
