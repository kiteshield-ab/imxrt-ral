#[doc = "TMPSNS"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "Control 1"]
    pub CTRL1: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1_SET: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1_CLR: crate::RWRegister<u32>,
    #[doc = "Control 1"]
    pub CTRL1_TOG: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0_SET: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0_CLR: crate::RWRegister<u32>,
    #[doc = "Range 0"]
    pub RANGE0_TOG: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1_SET: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1_CLR: crate::RWRegister<u32>,
    #[doc = "Range 1"]
    pub RANGE1_TOG: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Status 0"]
    pub STATUS0: crate::RWRegister<u32>,
}
#[doc = "Control 1"]
pub mod CTRL1 {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Single Reading Mode. A new reading is available every time you change START from 0 to 1."]
            pub const SINGLE_MODE: u32 = 0;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_1: u32 = 0x01;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_2: u32 = 0x02;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_3: u32 = 0x03;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_4: u32 = 0x04;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_5: u32 = 0x05;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_6: u32 = 0x06;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_7: u32 = 0x07;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_8: u32 = 0x08;
            #[doc = "Continuous Reading Mode. TMPSNS takes the next temperature reading when it completes the programmed number of cycles after the current reading."]
            pub const CONTINUOUS_MODE_9: u32 = 0x09;
        }
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
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
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
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
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
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
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
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
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No read"]
            pub const NO_READING_TAKEN: u32 = 0;
            #[doc = "New read"]
            pub const NEW_READING: u32 = 0x01;
        }
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "Inactive"]
            pub const INACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "Inactive"]
            pub const INACTIVE: u32 = 0x01;
        }
    }
}
#[doc = "Control 1"]
pub mod CTRL1_SET {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 1"]
pub mod CTRL1_CLR {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control 1"]
pub mod CTRL1_TOG {
    #[doc = "Temperature Measurement Frequency"]
    pub mod FREQ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Measurement Finished Interrupt Enable"]
    pub mod FINISH_IE {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Low Temperature Interrupt Enable"]
    pub mod LOW_TEMP_IE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Interrupt Enable"]
    pub mod HIGH_TEMP_IE {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Panic Temperature Interrupt Enable"]
    pub mod PANIC_TEMP_IE {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Temperature Measurement"]
    pub mod START {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down Except Bias Current"]
    pub mod PWD {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Power Down"]
    pub mod PWD_FULL {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0 {
    #[doc = "Low Temperature Threshold Value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Threshold Value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0_SET {
    #[doc = "Low Temperature Threshold Value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Threshold Value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0_CLR {
    #[doc = "Low Temperature Threshold Value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Threshold Value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 0"]
pub mod RANGE0_TOG {
    #[doc = "Low Temperature Threshold Value"]
    pub mod LOW_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High Temperature Threshold Value"]
    pub mod HIGH_TEMP_VAL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1 {
    #[doc = "Panic Temperature Threshold Value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1_SET {
    #[doc = "Panic Temperature Threshold Value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1_CLR {
    #[doc = "Panic Temperature Threshold Value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Range 1"]
pub mod RANGE1_TOG {
    #[doc = "Panic Temperature Threshold Value"]
    pub mod PANIC_TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Status 0"]
pub mod STATUS0 {
    #[doc = "Measured Temperature Value"]
    pub mod TEMP_VAL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Temperature Measurement Complete"]
    pub mod FINISH {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No read"]
            pub const NO_READING: u32 = 0;
            #[doc = "New read"]
            pub const NEW_READING: u32 = 0x01;
        }
    }
    #[doc = "Low Temperature Alarm"]
    pub mod LOW_TEMP {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alert"]
            pub const NO_LOW_ALERT: u32 = 0;
            #[doc = "Alert"]
            pub const LOW_ALERT: u32 = 0x01;
        }
    }
    #[doc = "High Temperature Alarm"]
    pub mod HIGH_TEMP {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alert"]
            pub const NO_HIGH_ALERT: u32 = 0;
            #[doc = "Alert"]
            pub const HIGH_ALERT: u32 = 0x01;
        }
    }
    #[doc = "Panic Temperature Alarm"]
    pub mod PANIC_TEMP {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No alert"]
            pub const NO_PANIC_ALERT: u32 = 0;
            #[doc = "Alert"]
            pub const PANIC_ALERT: u32 = 0x01;
        }
    }
}
