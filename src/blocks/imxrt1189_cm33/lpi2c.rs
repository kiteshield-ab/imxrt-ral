#[doc = "Low-Power Inter-Integrated Circuit"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "Controller Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "Controller Status"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Controller Interrupt Enable"]
    pub MIER: crate::RWRegister<u32>,
    #[doc = "Controller DMA Enable"]
    pub MDER: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 0"]
    pub MCFGR0: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 1"]
    pub MCFGR1: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 2"]
    pub MCFGR2: crate::RWRegister<u32>,
    #[doc = "Controller Configuration 3"]
    pub MCFGR3: crate::RWRegister<u32>,
    _reserved1: [u8; 0x10],
    #[doc = "Controller Data Match"]
    pub MDMR: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "Controller Clock Configuration 0"]
    pub MCCR0: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Controller Clock Configuration 1"]
    pub MCCR1: crate::RWRegister<u32>,
    _reserved4: [u8; 0x04],
    #[doc = "Controller FIFO Control"]
    pub MFCR: crate::RWRegister<u32>,
    #[doc = "Controller FIFO Status"]
    pub MFSR: crate::RORegister<u32>,
    #[doc = "Controller Transmit Data"]
    pub MTDR: crate::WORegister<u32>,
    _reserved5: [u8; 0x0c],
    #[doc = "Controller Receive Data"]
    pub MRDR: crate::RORegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "Controller Receive Data Read Only"]
    pub MRDROR: crate::RORegister<u32>,
    _reserved7: [u8; 0x94],
    #[doc = "Target Control"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "Target Status"]
    pub SSR: crate::RWRegister<u32>,
    #[doc = "Target Interrupt Enable"]
    pub SIER: crate::RWRegister<u32>,
    #[doc = "Target DMA Enable"]
    pub SDER: crate::RWRegister<u32>,
    #[doc = "Target Configuration 0"]
    pub SCFGR0: crate::RWRegister<u32>,
    #[doc = "Target Configuration 1"]
    pub SCFGR1: crate::RWRegister<u32>,
    #[doc = "Target Configuration 2"]
    pub SCFGR2: crate::RWRegister<u32>,
    _reserved8: [u8; 0x14],
    #[doc = "Target Address Match"]
    pub SAMR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x0c],
    #[doc = "Target Address Status"]
    pub SASR: crate::RORegister<u32>,
    #[doc = "Target Transmit ACK"]
    pub STAR: crate::RWRegister<u32>,
    _reserved10: [u8; 0x08],
    #[doc = "Target Transmit Data"]
    pub STDR: crate::WORegister<u32>,
    _reserved11: [u8; 0x0c],
    #[doc = "Target Receive Data"]
    pub SRDR: crate::RORegister<u32>,
    _reserved12: [u8; 0x04],
    #[doc = "Target Receive Data Read Only"]
    pub SRDROR: crate::RORegister<u32>,
    _reserved13: [u8; 0x84],
    #[doc = "Controller Transmit Command Burst"]
    pub MTCBR: [crate::WORegister<u32>; 128usize],
    #[doc = "Transmit Data Burst"]
    pub MTDBR: [crate::WORegister<u32>; 256usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Specification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Controller only, with standard feature set"]
            pub const MASTER_ONLY: u32 = 0x02;
            #[doc = "Controller and target, with standard feature set"]
            pub const MASTER_AND_SLAVE: u32 = 0x03;
        }
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
#[doc = "Parameter"]
pub mod PARAM {
    #[doc = "Controller Transmit FIFO Size"]
    pub mod MTXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller Receive FIFO Size"]
    pub mod MRXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Control"]
pub mod MCR {
    #[doc = "Controller Enable"]
    pub mod MEN {
        pub const offset: u32 = 0;
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
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NOT_RESET: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "Doze Mode Enable"]
    pub mod DOZEN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const ENABLED: u32 = 0;
            #[doc = "Disable"]
            pub const DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Debug Enable"]
    pub mod DBGEN {
        pub const offset: u32 = 3;
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
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Reset transmit FIFO"]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Reset receive FIFO"]
            pub const RESET: u32 = 0x01;
        }
    }
}
#[doc = "Controller Status"]
pub mod MSR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data not requested"]
            pub const DISABLED: u32 = 0;
            #[doc = "Transmit data requested"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receive data not ready"]
            pub const DISABLED: u32 = 0;
            #[doc = "Receive data ready"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "End Packet Flag"]
    pub mod EPF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No Stop or repeated Start generated"]
            pub const INT_NO: u32 = 0;
            #[doc = "Stop or repeated Start generated"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Detect Flag"]
    pub mod SDF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No Stop condition generated"]
            pub const INT_NO: u32 = 0;
            #[doc = "Stop condition generated"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "NACK Detect Flag"]
    pub mod NDF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No unexpected NACK detected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Unexpected NACK detected"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Arbitration Lost Flag"]
    pub mod ALF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Controller did not lose arbitration"]
            pub const INT_NO: u32 = 0;
            #[doc = "Controller lost arbitration"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No FIFO error"]
            pub const INT_NO: u32 = 0;
            #[doc = "FIFO error"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Pin Low Timeout Flag"]
    pub mod PLTF {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Pin low timeout did not occur"]
            pub const INT_NO: u32 = 0;
            #[doc = "Pin low timeout occurred"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Match Flag"]
    pub mod DMF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Matching data not received"]
            pub const INT_NO: u32 = 0;
            #[doc = "Matching data received"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Start Flag"]
    pub mod STF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Start condition not detected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Start condition detected"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Controller Busy Flag"]
    pub mod MBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Bus Busy Flag"]
    pub mod BBF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
    }
}
#[doc = "Controller Interrupt Enable"]
pub mod MIER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
        pub const offset: u32 = 0;
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
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
        pub const offset: u32 = 1;
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
    #[doc = "End Packet Interrupt Enable"]
    pub mod EPIE {
        pub const offset: u32 = 8;
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
    #[doc = "Stop Detect Interrupt Enable"]
    pub mod SDIE {
        pub const offset: u32 = 9;
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
    #[doc = "NACK Detect Interrupt Enable"]
    pub mod NDIE {
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
    #[doc = "Arbitration Lost Interrupt Enable"]
    pub mod ALIE {
        pub const offset: u32 = 11;
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
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 12;
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
    #[doc = "Pin Low Timeout Interrupt Enable"]
    pub mod PLTIE {
        pub const offset: u32 = 13;
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
    #[doc = "Data Match Interrupt Enable"]
    pub mod DMIE {
        pub const offset: u32 = 14;
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
    #[doc = "Start Interrupt Enable"]
    pub mod STIE {
        pub const offset: u32 = 15;
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
#[doc = "Controller DMA Enable"]
pub mod MDER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
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
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
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
#[doc = "Controller Configuration 0"]
pub mod MCFGR0 {
    #[doc = "Host Request Enable"]
    pub mod HREN {
        pub const offset: u32 = 0;
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
    #[doc = "Host Request Polarity"]
    pub mod HRPOL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active low"]
            pub const ACTIVE_LOW: u32 = 0;
            #[doc = "Active high"]
            pub const ACTIVE_HIGH: u32 = 0x01;
        }
    }
    #[doc = "Host Request Select"]
    pub mod HRSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Host request input is pin HREQ"]
            pub const DISABLED: u32 = 0;
            #[doc = "Host request input is input trigger"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Host Request Direction"]
    pub mod HRDIR {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "HREQ pin is input (for LPI2C controller)"]
            pub const INPUT: u32 = 0;
            #[doc = "HREQ pin is output (for LPI2C target)"]
            pub const OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "Circular FIFO Enable"]
    pub mod CIRFIFO {
        pub const offset: u32 = 8;
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
    #[doc = "Receive Data Match Only"]
    pub mod RDMO {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received data is stored in the receive FIFO"]
            pub const DISABLED: u32 = 0;
            #[doc = "Received data is discarded unless MSR\\[DMF\\] is set"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Relaxed Mode"]
    pub mod RELAX {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const NORMAL_TRANSFER: u32 = 0;
            #[doc = "Relaxed transfer"]
            pub const RELAXED_TRANSFER: u32 = 0x01;
        }
    }
    #[doc = "Abort Transfer"]
    pub mod ABORT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transfer"]
            pub const DISABLED: u32 = 0;
            #[doc = "Abort existing transfer and do not start a new one"]
            pub const ENABLED: u32 = 0x01;
        }
    }
}
#[doc = "Controller Configuration 1"]
pub mod MCFGR1 {
    #[doc = "Prescaler"]
    pub mod PRESCALE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Divide by 1"]
            pub const DIVIDE_BY_1: u32 = 0;
            #[doc = "Divide by 2"]
            pub const DIVIDE_BY_2: u32 = 0x01;
            #[doc = "Divide by 4"]
            pub const DIVIDE_BY_4: u32 = 0x02;
            #[doc = "Divide by 8"]
            pub const DIVIDE_BY_8: u32 = 0x03;
            #[doc = "Divide by 16"]
            pub const DIVIDE_BY_16: u32 = 0x04;
            #[doc = "Divide by 32"]
            pub const DIVIDE_BY_32: u32 = 0x05;
            #[doc = "Divide by 64"]
            pub const DIVIDE_BY_64: u32 = 0x06;
            #[doc = "Divide by 128"]
            pub const DIVIDE_BY_128: u32 = 0x07;
        }
    }
    #[doc = "Automatic Stop Generation"]
    pub mod AUTOSTOP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Stop automatically generated"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Ignore NACK"]
    pub mod IGNACK {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const DISABLED: u32 = 0;
            #[doc = "Treat a received NACK as an ACK"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Timeout Configuration"]
    pub mod TIMECFG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SCL"]
            pub const IF_SCL_LOW: u32 = 0;
            #[doc = "SCL or SDA"]
            pub const IF_SCL_OR_SDA_LOW: u32 = 0x01;
        }
    }
    #[doc = "Stop Configuration"]
    pub mod STOPCFG {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Any Stop condition"]
            pub const ANY_STOP: u32 = 0;
            #[doc = "Last Stop condition"]
            pub const LAST_STOP: u32 = 0x01;
        }
    }
    #[doc = "Start Configuration"]
    pub mod STARTCFG {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sets when both I2C bus and LPI2C controller are idle"]
            pub const BOTH_I2C_AND_LPI2C_IDLE: u32 = 0;
            #[doc = "Sets when I2C bus is idle"]
            pub const I2C_IDLE: u32 = 0x01;
        }
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Match is disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Match is enabled: first data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]"]
            pub const FIRST_DATA_WORD_EQUALS_MATCH0_OR_MATCH1: u32 = 0x02;
            #[doc = "Match is enabled: any data word equals MDMR\\[MATCH0\\] OR MDMR\\[MATCH1\\]"]
            pub const ANY_DATA_WORD_EQUALS_MATCH0_OR_MATCH1: u32 = 0x03;
            #[doc = "Match is enabled: (first data word equals MDMR\\[MATCH0\\]) AND (second data word equals MDMR\\[MATCH1)"]
            pub const FIRST_DATA_WORD_MATCH0_AND_SECOND_DATA_WORD_MATCH1: u32 = 0x04;
            #[doc = "Match is enabled: (any data word equals MDMR\\[MATCH0\\]) AND (next data word equals MDMR\\[MATCH1)"]
            pub const ANY_DATA_WORD_MATCH0_NEXT_DATA_WORD_MATCH1: u32 = 0x05;
            #[doc = "Match is enabled: (first data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])"]
            pub const FIRST_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1: u32 = 0x06;
            #[doc = "Match is enabled: (any data word AND MDMR\\[MATCH1\\]) equals (MDMR\\[MATCH0\\] AND MDMR\\[MATCH1\\])"]
            pub const ANY_DATA_WORD_AND_MATCH1_EQUALS_MATCH0_AND_MATCH1: u32 = 0x07;
        }
    }
    #[doc = "Pin Configuration"]
    pub mod PINCFG {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Two-pin open drain mode"]
            pub const OPEN_DRAIN_2_PIN: u32 = 0;
            #[doc = "Two-pin output only mode (Ultra-Fast mode)"]
            pub const OUTPUT_2_PIN_ONLY: u32 = 0x01;
            #[doc = "Two-pin push-pull mode"]
            pub const PUSH_PULL_2_PIN: u32 = 0x02;
            #[doc = "Four-pin push-pull mode"]
            pub const PUSH_PULL_4_PIN: u32 = 0x03;
            #[doc = "Two-pin open-drain mode with separate LPI2C target"]
            pub const OPEN_DRAIN_2_PIN_W_LPI2C_SLAVE: u32 = 0x04;
            #[doc = "Two-pin output only mode (Ultra-Fast mode) with separate LPI2C target"]
            pub const OUTPUT_2_PIN_ONLY_W_LPI2C_SLAVE: u32 = 0x05;
            #[doc = "Two-pin push-pull mode with separate LPI2C target"]
            pub const PUSH_PULL_2_PIN_W_LPI2C_SLAVE: u32 = 0x06;
            #[doc = "Four-pin push-pull mode (inverted outputs)"]
            pub const PUSH_PULL_4_PIN_W_LPI2C_SLAVE: u32 = 0x07;
        }
    }
}
#[doc = "Controller Configuration 2"]
pub mod MCFGR2 {
    #[doc = "Bus Idle Timeout"]
    pub mod BUSIDLE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SCL"]
    pub mod FILTSCL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SDA"]
    pub mod FILTSDA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Configuration 3"]
pub mod MCFGR3 {
    #[doc = "Pin Low Timeout"]
    pub mod PINLOW {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Data Match"]
pub mod MDMR {
    #[doc = "Match 0 Value"]
    pub mod MATCH0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match 1 Value"]
    pub mod MATCH1 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Clock Configuration 0"]
pub mod MCCR0 {
    #[doc = "Clock Low Period"]
    pub mod CLKLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock High Period"]
    pub mod CLKHI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup Hold Delay"]
    pub mod SETHOLD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Clock Configuration 1"]
pub mod MCCR1 {
    #[doc = "Clock Low Period"]
    pub mod CLKLO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clock High Period"]
    pub mod CLKHI {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Setup Hold Delay"]
    pub mod SETHOLD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller FIFO Control"]
pub mod MFCR {
    #[doc = "Transmit FIFO Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller FIFO Status"]
pub mod MFSR {
    #[doc = "Transmit FIFO Count"]
    pub mod TXCOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Count"]
    pub mod RXCOUNT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Controller Transmit Data"]
pub mod MTDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command Data"]
    pub mod CMD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit the value in DATA\\[7:0\\]"]
            pub const TRANSMIT_DATA_7_THROUGH_0: u32 = 0;
            #[doc = "Receive (DATA\\[7:0\\] + 1) bytes"]
            pub const RECEIVE_DATA_7_THROUGH_0_PLUS_ONE: u32 = 0x01;
            #[doc = "Generate Stop condition on I2C bus"]
            pub const GENERATE_STOP_CONDITION: u32 = 0x02;
            #[doc = "Receive and discard (DATA\\[7:0\\] + 1) bytes"]
            pub const RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE: u32 = 0x03;
            #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\]"]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0: u32 = 0x04;
            #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] (this transfer expects a NACK to be returned)"]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK: u32 =
                0x05;
            #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode"]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE : u32 = 0x06 ;
            #[doc = "Generate (repeated) Start on the I2C bus and transmit the address in DATA\\[7:0\\] using HS mode (this transfer expects a NACK to be returned)"]
            pub const GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE_EXPECT_NACK : u32 = 0x07 ;
        }
    }
}
#[doc = "Controller Receive Data"]
pub mod MRDR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
}
#[doc = "Controller Receive Data Read Only"]
pub mod MRDROR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RX Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
}
#[doc = "Target Control"]
pub mod SCR {
    #[doc = "Target Enable"]
    pub mod SEN {
        pub const offset: u32 = 0;
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
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reset"]
            pub const NOT_RESET: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "Filter Enable"]
    pub mod FILTEN {
        pub const offset: u32 = 4;
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
    #[doc = "Filter Doze Enable"]
    pub mod FILTDZ {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const FILTER_ENABLED: u32 = 0;
            #[doc = "Disable"]
            pub const FILTER_DISABLED: u32 = 0x01;
        }
    }
    #[doc = "Reset Transmit FIFO"]
    pub mod RTF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "STDR is now empty"]
            pub const NOW_EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Reset Receive FIFO"]
    pub mod RRF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "SRDR is now empty"]
            pub const NOW_EMPTY: u32 = 0x01;
        }
    }
}
#[doc = "Target Status"]
pub mod SSR {
    #[doc = "Transmit Data Flag"]
    pub mod TDF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit data not requested"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Transmit data is requested"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Flag"]
    pub mod RDF {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not ready"]
            pub const NOT_READY: u32 = 0;
            #[doc = "Ready"]
            pub const READY: u32 = 0x01;
        }
    }
    #[doc = "Address Valid Flag"]
    pub mod AVF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not valid"]
            pub const NOT_VALID: u32 = 0;
            #[doc = "Valid"]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Transmit ACK Flag"]
    pub mod TAF {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not required"]
            pub const NOT_REQUIRED: u32 = 0;
            #[doc = "Required"]
            pub const REQUIRED: u32 = 0x01;
        }
    }
    #[doc = "Repeated Start Flag"]
    pub mod RSF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No repeated Start detected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Repeated Start detected"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Detect Flag"]
    pub mod SDF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No Stop detected"]
            pub const INT_NO: u32 = 0;
            #[doc = "Stop detected"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit Error Flag"]
    pub mod BEF {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No bit error occurred"]
            pub const INT_NO: u32 = 0;
            #[doc = "Bit error occurred"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "FIFO Error Flag"]
    pub mod FEF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No FIFO error"]
            pub const INT_NO: u32 = 0;
            #[doc = "FIFO error"]
            pub const INT_YES: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Match 0 Flag"]
    pub mod AM0F {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ADDR0 matching address not received"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "ADDR0 matching address received"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Address Match 1 Flag"]
    pub mod AM1F {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Matching address not received"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Matching address received"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "General Call Flag"]
    pub mod GCF {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "General call address disabled or not detected"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "General call address detected"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "SMBus Alert Response Flag"]
    pub mod SARF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled or not detected"]
            pub const NO_FLAG: u32 = 0;
            #[doc = "Enabled and detected"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Target Busy Flag"]
    pub mod SBF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
    }
    #[doc = "Bus Busy Flag"]
    pub mod BBF {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Busy"]
            pub const BUSY: u32 = 0x01;
        }
    }
}
#[doc = "Target Interrupt Enable"]
pub mod SIER {
    #[doc = "Transmit Data Interrupt Enable"]
    pub mod TDIE {
        pub const offset: u32 = 0;
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
    #[doc = "Receive Data Interrupt Enable"]
    pub mod RDIE {
        pub const offset: u32 = 1;
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
    #[doc = "Address Valid Interrupt Enable"]
    pub mod AVIE {
        pub const offset: u32 = 2;
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
    #[doc = "Transmit ACK Interrupt Enable"]
    pub mod TAIE {
        pub const offset: u32 = 3;
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
    #[doc = "Repeated Start Interrupt Enable"]
    pub mod RSIE {
        pub const offset: u32 = 8;
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
    #[doc = "Stop Detect Interrupt Enable"]
    pub mod SDIE {
        pub const offset: u32 = 9;
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
    #[doc = "Bit Error Interrupt Enable"]
    pub mod BEIE {
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
    #[doc = "FIFO Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 11;
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
    #[doc = "Address Match 0 Interrupt Enable"]
    pub mod AM0IE {
        pub const offset: u32 = 12;
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
    #[doc = "Address Match 1 Interrupt Enable"]
    pub mod AM1IE {
        pub const offset: u32 = 13;
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
    #[doc = "General Call Interrupt Enable"]
    pub mod GCIE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enabled"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "SMBus Alert Response Interrupt Enable"]
    pub mod SARIE {
        pub const offset: u32 = 15;
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
#[doc = "Target DMA Enable"]
pub mod SDER {
    #[doc = "Transmit Data DMA Enable"]
    pub mod TDDE {
        pub const offset: u32 = 0;
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
    #[doc = "Receive Data DMA Enable"]
    pub mod RDDE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable DMA request"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable DMA request"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Address Valid DMA Enable"]
    pub mod AVDE {
        pub const offset: u32 = 2;
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
    #[doc = "Repeated Start DMA Enable"]
    pub mod RSDE {
        pub const offset: u32 = 8;
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
    #[doc = "Stop Detect DMA Enable"]
    pub mod SDDE {
        pub const offset: u32 = 9;
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
#[doc = "Target Configuration 0"]
pub mod SCFGR0 {
    #[doc = "Read Request"]
    pub mod RDREQ {
        pub const offset: u32 = 0;
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
    #[doc = "Read Acknowledge Flag"]
    pub mod RDACK {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Read Request not acknowledged"]
            pub const NOT_ACKNOWLEDGED: u32 = 0;
            #[doc = "Read Request acknowledged"]
            pub const ACKNOWLEDGED: u32 = 0x01;
        }
    }
}
#[doc = "Target Configuration 1"]
pub mod SCFGR1 {
    #[doc = "Address SCL Stall"]
    pub mod ADRSTALL {
        pub const offset: u32 = 0;
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
    #[doc = "RX SCL Stall"]
    pub mod RXSTALL {
        pub const offset: u32 = 1;
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
    #[doc = "Transmit Data SCL Stall"]
    pub mod TXDSTALL {
        pub const offset: u32 = 2;
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
    #[doc = "ACK SCL Stall"]
    pub mod ACKSTALL {
        pub const offset: u32 = 3;
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
    #[doc = "Receive NACK"]
    pub mod RXNACK {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ACK or NACK always determined by STAR\\[TXNACK\\]"]
            pub const SET_BY_TXNACK: u32 = 0;
            #[doc = "NACK always generated on address overrun or receive data overrun, otherwise ACK or NACK is determined by STAR\\[TXNACK\\]"]
            pub const ALWAYS_GENERATED_ON_ADDRESS_OR_RECEIVE_DATA_OVERRUN: u32 = 0x01;
        }
    }
    #[doc = "General Call Enable"]
    pub mod GCEN {
        pub const offset: u32 = 8;
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
    #[doc = "SMBus Alert Enable"]
    pub mod SAEN {
        pub const offset: u32 = 9;
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
    #[doc = "Transmit Flag Configuration"]
    pub mod TXCFG {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "MSR\\[TDF\\] is set only during a target-transmit transfer when STDR is empty"]
            pub const ASSERTS_DURING_SLAVE_TRANSMIT_TRANSFER_WHEN_TX_DATA_EMPTY: u32 = 0;
            #[doc = "MSR\\[TDF\\] is set whenever STDR is empty"]
            pub const ASSERTS_WHEN_TX_DATA_EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Configuration"]
    pub mod RXCFG {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Return received data, clear MSR\\[RDF\\]"]
            pub const RETURNS_RECEIVED_DATA_AND_CLEARS_RX_DATA_FLAG: u32 = 0;
            #[doc = "Return SASR and clear SSR\\[AVF\\] when SSR\\[AVF\\] is set, return received data and clear MSR\\[RDF\\] when SSR\\[AFV\\] is not set"]
            pub const WHEN_ADDRESS_VALID_FLAG_SET_RETURNS_ADDRESS_STATUS_AND_CLEARS_ADDRESS_VALID_FLAG : u32 = 0x01 ;
        }
    }
    #[doc = "Ignore NACK"]
    pub mod IGNACK {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "End transfer on NACK"]
            pub const ENDS_TRANSFER_ON_NACK: u32 = 0;
            #[doc = "Do not end transfer on NACK"]
            pub const DOES_NOT_END_TRANSFER_ON_NACK: u32 = 0x01;
        }
    }
    #[doc = "HS Mode Enable"]
    pub mod HSMEN {
        pub const offset: u32 = 13;
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
    #[doc = "Address Configuration"]
    pub mod ADDRCFG {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address match 0 (7-bit)"]
            pub const ADDRESS_MATCH0_7_BIT: u32 = 0;
            #[doc = "Address match 0 (10-bit)"]
            pub const ADDRESS_MATCH0_10_BIT: u32 = 0x01;
            #[doc = "Address match 0 (7-bit) or address match 1 (7-bit)"]
            pub const ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_7_BIT: u32 = 0x02;
            #[doc = "Address match 0 (10-bit) or address match 1 (10-bit)"]
            pub const ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_10_BIT: u32 = 0x03;
            #[doc = "Address match 0 (7-bit) or address match 1 (10-bit)"]
            pub const ADDRESS_MATCH0_7_BIT_OR_ADDRESS_MATCH1_10_BIT: u32 = 0x04;
            #[doc = "Address match 0 (10-bit) or address match 1 (7-bit)"]
            pub const ADDRESS_MATCH0_10_BIT_OR_ADDRESS_MATCH1_7_BIT: u32 = 0x05;
            #[doc = "From address match 0 (7-bit) to address match 1 (7-bit)"]
            pub const FROM_ADDRESS_MATCH0_7_BIT_TO_ADDRESS_MATCH1_7_BIT: u32 = 0x06;
            #[doc = "From address match 0 (10-bit) to address match 1 (10-bit)"]
            pub const FROM_ADDRESS_MATCH0_10_BIT_TO_ADDRESS_MATCH1_10_BIT: u32 = 0x07;
        }
    }
    #[doc = "Receive All"]
    pub mod RXALL {
        pub const offset: u32 = 24;
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
    #[doc = "Repeated Start Configuration"]
    pub mod RSCFG {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Any repeated Start condition following an address match"]
            pub const ANY_REPEATED_START_AFTER_ADDRESS_MATCH: u32 = 0;
            #[doc = "Any repeated Start condition"]
            pub const ANY_REPEATED_START: u32 = 0x01;
        }
    }
    #[doc = "Stop Detect Configuration"]
    pub mod SDCFG {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Any Stop condition following an address match"]
            pub const ANY_STOP_AFTER_ADDRESS_MATCH: u32 = 0;
            #[doc = "Any Stop condition"]
            pub const ANY_STOP: u32 = 0x01;
        }
    }
}
#[doc = "Target Configuration 2"]
pub mod SCFGR2 {
    #[doc = "Clock Hold Time"]
    pub mod CLKHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data Valid Delay"]
    pub mod DATAVD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SCL"]
    pub mod FILTSCL {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Glitch Filter SDA"]
    pub mod FILTSDA {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Address Match"]
pub mod SAMR {
    #[doc = "Address 0 Value"]
    pub mod ADDR0 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address 1 Value"]
    pub mod ADDR1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Address Status"]
pub mod SASR {
    #[doc = "Received Address"]
    pub mod RADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Address Not Valid"]
    pub mod ANV {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Valid"]
            pub const VALID: u32 = 0;
            #[doc = "Not valid"]
            pub const NOT_VALID: u32 = 0x01;
        }
    }
}
#[doc = "Target Transmit ACK"]
pub mod STAR {
    #[doc = "Transmit NACK"]
    pub mod TXNACK {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmit ACK"]
            pub const TRANSMIT_ACK: u32 = 0;
            #[doc = "Transmit NACK"]
            pub const TRANSMIT_NACK: u32 = 0x01;
        }
    }
}
#[doc = "Target Transmit Data"]
pub mod STDR {
    #[doc = "Transmit Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Target Receive Data"]
pub mod SRDR {
    #[doc = "Received Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Received Address"]
    pub mod RADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Start of Frame"]
    pub mod SOF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not first"]
            pub const NOT_FIRST_DATA_WORD: u32 = 0;
            #[doc = "First"]
            pub const FIRST_DATA_WORD: u32 = 0x01;
        }
    }
}
#[doc = "Target Receive Data Read Only"]
pub mod SRDROR {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Received Address"]
    pub mod RADDR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Empty"]
    pub mod RXEMPTY {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not empty"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Start of Frame"]
    pub mod SOF {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not the first"]
            pub const NOT_FIRST_DATA_WORD: u32 = 0;
            #[doc = "First"]
            pub const FIRST_DATA_WORD: u32 = 0x01;
        }
    }
}
#[doc = "Controller Transmit Command Burst"]
pub mod MTCBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Command"]
    pub mod CMD {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Burst"]
pub mod MTDBR {
    #[doc = "Data"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data"]
    pub mod DATA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data"]
    pub mod DATA3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
