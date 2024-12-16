#[doc = "LPUART"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Version ID"]
    pub VERID: crate::RORegister<u32>,
    #[doc = "Parameter"]
    pub PARAM: crate::RORegister<u32>,
    #[doc = "Global"]
    pub GLOBAL: crate::RWRegister<u32>,
    #[doc = "Pin Configuration"]
    pub PINCFG: crate::RWRegister<u32>,
    #[doc = "Baud Rate"]
    pub BAUD: crate::RWRegister<u32>,
    #[doc = "Status"]
    pub STAT: crate::RWRegister<u32>,
    #[doc = "Control"]
    pub CTRL: crate::RWRegister<u32>,
    #[doc = "Data"]
    pub DATA: crate::RWRegister<u32>,
    #[doc = "Match Address"]
    pub MATCH: crate::RWRegister<u32>,
    #[doc = "MODEM IrDA"]
    pub MODIR: crate::RWRegister<u32>,
    #[doc = "FIFO"]
    pub FIFO: crate::RWRegister<u32>,
    #[doc = "Watermark"]
    pub WATER: crate::RWRegister<u32>,
    #[doc = "Data Read-Only"]
    pub DATARO: crate::RORegister<u32>,
    _reserved0: [u8; 0x0c],
    #[doc = "MODEM Control"]
    pub MCR: crate::RWRegister<u32>,
    #[doc = "MODEM Status"]
    pub MSR: crate::RWRegister<u32>,
    #[doc = "Receiver Extended Idle"]
    pub REIR: crate::RWRegister<u32>,
    #[doc = "Transmitter Extended Idle"]
    pub TEIR: crate::RWRegister<u32>,
    #[doc = "Half Duplex Control"]
    pub HDCR: crate::RWRegister<u32>,
    _reserved1: [u8; 0x04],
    #[doc = "Timeout Control"]
    pub TOCR: crate::RWRegister<u32>,
    #[doc = "Timeout Status"]
    pub TOSR: crate::RWRegister<u32>,
    #[doc = "Timeout N"]
    pub TIMEOUT: [crate::RWRegister<u32>; 4usize],
    _reserved2: [u8; 0x0190],
    #[doc = "Transmit Command Burst"]
    pub TCBR: [crate::WORegister<u32>; 128usize],
    #[doc = "Transmit Data Burst"]
    pub TDBR: [crate::WORegister<u32>; 256usize],
}
#[doc = "Version ID"]
pub mod VERID {
    #[doc = "Feature Identification Number"]
    pub mod FEATURE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Standard feature set"]
            pub const STANDARD: u32 = 0x01;
            #[doc = "Standard feature set with MODEM and IrDA support"]
            pub const MODEM: u32 = 0x03;
            #[doc = "Enhanced feature set with full MODEM, IrDA, and enhanced idle detection"]
            pub const MODEM_IDLE: u32 = 0x07;
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
    #[doc = "Transmit FIFO Size"]
    pub mod TXFIFO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Size"]
    pub mod RXFIFO {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Global"]
pub mod GLOBAL {
    #[doc = "Software Reset"]
    pub mod RST {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not reset"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Reset"]
            pub const RESET: u32 = 0x01;
        }
    }
}
#[doc = "Pin Configuration"]
pub mod PINCFG {
    #[doc = "Trigger Select"]
    pub mod TRGSEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input trigger disabled"]
            pub const DISABLED: u32 = 0;
            #[doc = "Input trigger used instead of the RXD pin input"]
            pub const TRG_RXD: u32 = 0x01;
            #[doc = "Input trigger used instead of the CTS_B pin input"]
            pub const TRG_CTS: u32 = 0x02;
            #[doc = "Input trigger used to modulate the TXD pin output, which (after TXINV configuration) is internally ANDed with the input trigger"]
            pub const TRG_TXD: u32 = 0x03;
        }
    }
}
#[doc = "Baud Rate"]
pub mod BAUD {
    #[doc = "Baud Rate Modulo Divisor"]
    pub mod SBR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stop Bit Number Select"]
    pub mod SBNS {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "One stop bit"]
            pub const ONE: u32 = 0;
            #[doc = "Two stop bits"]
            pub const TWO: u32 = 0x01;
        }
    }
    #[doc = "RX Input Active Edge Interrupt Enable"]
    pub mod RXEDGIE {
        pub const offset: u32 = 14;
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
    #[doc = "LIN Break Detect Interrupt Enable"]
    pub mod LBKDIE {
        pub const offset: u32 = 15;
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
    #[doc = "Resynchronization Disable"]
    pub mod RESYNCDIS {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enable"]
            pub const RESYNC: u32 = 0;
            #[doc = "Disable"]
            pub const NO_RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Both Edge Sampling"]
    pub mod BOTHEDGE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rising edge"]
            pub const DISABLED: u32 = 0;
            #[doc = "Both rising and falling edges"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Match Configuration"]
    pub mod MATCFG {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Address match wake-up"]
            pub const ADDR_MATCH: u32 = 0;
            #[doc = "Idle match wake-up"]
            pub const IDLE_MATCH: u32 = 0x01;
            #[doc = "Match on and match off"]
            pub const ONOFF_MATCH: u32 = 0x02;
            #[doc = "Enables RWU on data match and match on or off for the transmitter CTS input"]
            pub const RWU_MATCH: u32 = 0x03;
        }
    }
    #[doc = "Receiver Idle DMA Enable"]
    pub mod RIDMAE {
        pub const offset: u32 = 20;
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
    #[doc = "Receiver Full DMA Enable"]
    pub mod RDMAE {
        pub const offset: u32 = 21;
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
    #[doc = "Transmitter DMA Enable"]
    pub mod TDMAE {
        pub const offset: u32 = 23;
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
    #[doc = "Oversampling Ratio"]
    pub mod OSR {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Results in an OSR of 16"]
            pub const DEFAULT: u32 = 0;
            #[doc = "Results in an OSR of 4 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
            pub const OSR_4: u32 = 0x03;
            #[doc = "Results in an OSR of 5 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
            pub const OSR_5: u32 = 0x04;
            #[doc = "Results in an OSR of 6 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
            pub const OSR_6: u32 = 0x05;
            #[doc = "Results in an OSR of 7 (requires BAUD\\[BOTHEDGE\\] to be 1)"]
            pub const OSR_7: u32 = 0x06;
            #[doc = "Results in an OSR of 8"]
            pub const OSR_8: u32 = 0x07;
            #[doc = "Results in an OSR of 9"]
            pub const OSR_9: u32 = 0x08;
            #[doc = "Results in an OSR of 10"]
            pub const OSR_10: u32 = 0x09;
            #[doc = "Results in an OSR of 11"]
            pub const OSR_11: u32 = 0x0a;
            #[doc = "Results in an OSR of 12"]
            pub const OSR_12: u32 = 0x0b;
            #[doc = "Results in an OSR of 13"]
            pub const OSR_13: u32 = 0x0c;
            #[doc = "Results in an OSR of 14"]
            pub const OSR_14: u32 = 0x0d;
            #[doc = "Results in an OSR of 15"]
            pub const OSR_15: u32 = 0x0e;
            #[doc = "Results in an OSR of 16"]
            pub const OSR_16: u32 = 0x0f;
            #[doc = "Results in an OSR of 17"]
            pub const OSR_17: u32 = 0x10;
            #[doc = "Results in an OSR of 18"]
            pub const OSR_18: u32 = 0x11;
            #[doc = "Results in an OSR of 19"]
            pub const OSR_19: u32 = 0x12;
            #[doc = "Results in an OSR of 20"]
            pub const OSR_20: u32 = 0x13;
            #[doc = "Results in an OSR of 21"]
            pub const OSR_21: u32 = 0x14;
            #[doc = "Results in an OSR of 22"]
            pub const OSR_22: u32 = 0x15;
            #[doc = "Results in an OSR of 23"]
            pub const OSR_23: u32 = 0x16;
            #[doc = "Results in an OSR of 24"]
            pub const OSR_24: u32 = 0x17;
            #[doc = "Results in an OSR of 25"]
            pub const OSR_25: u32 = 0x18;
            #[doc = "Results in an OSR of 26"]
            pub const OSR_26: u32 = 0x19;
            #[doc = "Results in an OSR of 27"]
            pub const OSR_27: u32 = 0x1a;
            #[doc = "Results in an OSR of 28"]
            pub const OSR_28: u32 = 0x1b;
            #[doc = "Results in an OSR of 29"]
            pub const OSR_29: u32 = 0x1c;
            #[doc = "Results in an OSR of 30"]
            pub const OSR_30: u32 = 0x1d;
            #[doc = "Results in an OSR of 31"]
            pub const OSR_31: u32 = 0x1e;
            #[doc = "Results in an OSR of 32"]
            pub const OSR_32: u32 = 0x1f;
        }
    }
    #[doc = "10-Bit Mode Select"]
    pub mod M10 {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters"]
            pub const DISABLED: u32 = 0;
            #[doc = "Receiver and transmitter use 10-bit data characters"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Match Address Mode Enable 2"]
    pub mod MAEN2 {
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
    #[doc = "Match Address Mode Enable 1"]
    pub mod MAEN1 {
        pub const offset: u32 = 31;
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
#[doc = "Status"]
pub mod STAT {
    #[doc = "LIN Break Flag Enable"]
    pub mod LBKFE {
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
    #[doc = "Address Mark Enable"]
    pub mod AME {
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
    #[doc = "MODEM Status Flag"]
    pub mod MSF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is 0"]
            pub const NOFLAG: u32 = 0;
            #[doc = "Field is 1"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Timeout Status Flag"]
    pub mod TSF {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Field is 0"]
            pub const NOFLAG: u32 = 0;
            #[doc = "Field is 1"]
            pub const FLAG: u32 = 0x01;
        }
    }
    #[doc = "Match 2 Flag"]
    pub mod MA2F {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not equal to MA2"]
            pub const NOMATCH: u32 = 0;
            #[doc = "Equal to MA2"]
            pub const MATCH: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match 1 Flag"]
    pub mod MA1F {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not equal to MA1"]
            pub const NOMATCH: u32 = 0;
            #[doc = "Equal to MA1"]
            pub const MATCH: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Parity Error Flag"]
    pub mod PF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No parity error detected"]
            pub const NOPARITY: u32 = 0;
            #[doc = "Parity error detected"]
            pub const PARITY: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Framing Error Flag"]
    pub mod FE {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No framing error detected (this does not guarantee that the framing is correct)"]
            pub const NOERROR: u32 = 0;
            #[doc = "Framing error detected"]
            pub const ERROR: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Noise Flag"]
    pub mod NF {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No noise detected"]
            pub const NONOISE: u32 = 0;
            #[doc = "Noise detected"]
            pub const NOISE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receiver Overrun Flag"]
    pub mod OR {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No overrun"]
            pub const NO_OVERRUN: u32 = 0;
            #[doc = "Receive overrun (new LPUART data is lost)"]
            pub const OVERRUN: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle Line Flag"]
    pub mod IDLE {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Idle line detected"]
            pub const NOIDLE: u32 = 0;
            #[doc = "Idle line not detected"]
            pub const IDLE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Data Register Full Flag"]
    pub mod RDRF {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Equal to or less than watermark"]
            pub const NO_RXDATA: u32 = 0;
            #[doc = "Greater than watermark"]
            pub const RXDATA: u32 = 0x01;
        }
    }
    #[doc = "Transmission Complete Flag"]
    pub mod TC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Transmitter active"]
            pub const ACTIVE: u32 = 0;
            #[doc = "Transmitter idle"]
            pub const COMPLETE: u32 = 0x01;
        }
    }
    #[doc = "Transmit Data Register Empty Flag"]
    pub mod TDRE {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Greater than watermark"]
            pub const TXDATA: u32 = 0;
            #[doc = "Equal to or less than watermark"]
            pub const NO_TXDATA: u32 = 0x01;
        }
    }
    #[doc = "Receiver Active Flag"]
    pub mod RAF {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle, waiting for a start bit"]
            pub const IDLE: u32 = 0;
            #[doc = "Receiver active (RXD pin input not idle)"]
            pub const ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "LIN Break Detection Enable"]
    pub mod LBKDE {
        pub const offset: u32 = 25;
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
    #[doc = "Break Character Generation Length"]
    pub mod BRK13 {
        pub const offset: u32 = 26;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "9 to 13 bit times"]
            pub const SHORT: u32 = 0;
            #[doc = "12 to 15 bit times"]
            pub const LONG: u32 = 0x01;
        }
    }
    #[doc = "Receive Wake Up Idle Detect"]
    pub mod RWUID {
        pub const offset: u32 = 27;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "STAT\\[IDLE\\] does not become 1"]
            pub const IDLE_NOTSET: u32 = 0;
            #[doc = "STAT\\[IDLE\\] becomes 1"]
            pub const IDLE_SET: u32 = 0x01;
        }
    }
    #[doc = "Receive Data Inversion"]
    pub mod RXINV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Inverted"]
            pub const NOT_INVERTED: u32 = 0;
            #[doc = "Not inverted"]
            pub const INVERTED: u32 = 0x01;
        }
    }
    #[doc = "MSB First"]
    pub mod MSBF {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB"]
            pub const LSB_FIRST: u32 = 0;
            #[doc = "MSB"]
            pub const MSB_FIRST: u32 = 0x01;
        }
    }
    #[doc = "RXD Pin Active Edge Interrupt Flag"]
    pub mod RXEDGIF {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not occurred"]
            pub const NO_EDGE: u32 = 0;
            #[doc = "Occurred"]
            pub const EDGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIN Break Detect Interrupt Flag"]
    pub mod LBKDIF {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Not detected"]
            pub const NOT_DETECTED: u32 = 0;
            #[doc = "Detected"]
            pub const DETECTED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Control"]
pub mod CTRL {
    #[doc = "Parity Type"]
    pub mod PT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Even parity"]
            pub const EVEN: u32 = 0;
            #[doc = "Odd parity"]
            pub const ODD: u32 = 0x01;
        }
    }
    #[doc = "Parity Enable"]
    pub mod PE {
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
    #[doc = "Idle Line Type Select"]
    pub mod ILT {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "After the start bit"]
            pub const FROM_START: u32 = 0;
            #[doc = "After the stop bit"]
            pub const FROM_STOP: u32 = 0x01;
        }
    }
    #[doc = "Receiver Wake-Up Method Select"]
    pub mod WAKE {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Idle"]
            pub const IDLE: u32 = 0;
            #[doc = "Mark"]
            pub const MARK: u32 = 0x01;
        }
    }
    #[doc = "9-Bit Or 8-Bit Mode Select"]
    pub mod M {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit"]
            pub const DATA8: u32 = 0;
            #[doc = "9-bit"]
            pub const DATA9: u32 = 0x01;
        }
    }
    #[doc = "Receiver Source Select"]
    pub mod RSRC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Internal Loopback mode"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Single-wire mode"]
            pub const ONEWIRE: u32 = 0x01;
        }
    }
    #[doc = "Doze Mode"]
    pub mod DOZEEN {
        pub const offset: u32 = 6;
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
    #[doc = "Loop Mode Select"]
    pub mod LOOPS {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation: RXD and TXD use separate pins"]
            pub const NOFFECT: u32 = 0;
            #[doc = "Loop mode or Single-Wire mode"]
            pub const LOOPBACK: u32 = 0x01;
        }
    }
    #[doc = "Idle Configuration"]
    pub mod IDLECFG {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const IDLE_1: u32 = 0;
            #[doc = "2"]
            pub const IDLE_2: u32 = 0x01;
            #[doc = "4"]
            pub const IDLE_4: u32 = 0x02;
            #[doc = "8"]
            pub const IDLE_8: u32 = 0x03;
            #[doc = "16"]
            pub const IDLE_16: u32 = 0x04;
            #[doc = "32"]
            pub const IDLE_32: u32 = 0x05;
            #[doc = "64"]
            pub const IDLE_64: u32 = 0x06;
            #[doc = "128"]
            pub const IDLE_128: u32 = 0x07;
        }
    }
    #[doc = "7-Bit Mode Select"]
    pub mod M7 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "8-bit to 10-bit"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "7-bit"]
            pub const DATA7: u32 = 0x01;
        }
    }
    #[doc = "Match 2 (MA2F) Interrupt Enable"]
    pub mod MA2IE {
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
    #[doc = "Match 1 (MA1F) Interrupt Enable"]
    pub mod MA1IE {
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
    #[doc = "Send Break"]
    pub mod SBK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal transmitter operation"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Queue break character(s) to be sent"]
            pub const TX_BREAK: u32 = 0x01;
        }
    }
    #[doc = "Receiver Wake-Up Control"]
    pub mod RWU {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal receiver operation"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "LPUART receiver in standby, waiting for a wake-up condition"]
            pub const RX_WAKEUP: u32 = 0x01;
        }
    }
    #[doc = "Receiver Enable"]
    pub mod RE {
        pub const offset: u32 = 18;
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
    #[doc = "Transmitter Enable"]
    pub mod TE {
        pub const offset: u32 = 19;
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
    #[doc = "Idle Line Interrupt Enable"]
    pub mod ILIE {
        pub const offset: u32 = 20;
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
    #[doc = "Receiver Interrupt Enable"]
    pub mod RIE {
        pub const offset: u32 = 21;
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
    #[doc = "Transmission Complete Interrupt Enable"]
    pub mod TCIE {
        pub const offset: u32 = 22;
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
    #[doc = "Transmit Interrupt Enable"]
    pub mod TIE {
        pub const offset: u32 = 23;
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
    #[doc = "Parity Error Interrupt Enable"]
    pub mod PEIE {
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
    #[doc = "Framing Error Interrupt Enable"]
    pub mod FEIE {
        pub const offset: u32 = 25;
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
    #[doc = "Noise Error Interrupt Enable"]
    pub mod NEIE {
        pub const offset: u32 = 26;
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
    #[doc = "Overrun Interrupt Enable"]
    pub mod ORIE {
        pub const offset: u32 = 27;
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
    #[doc = "Transmit Data Inversion"]
    pub mod TXINV {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not inverted"]
            pub const NOT_INVERTED: u32 = 0;
            #[doc = "Inverted"]
            pub const INVERTED: u32 = 0x01;
        }
    }
    #[doc = "TXD Pin Direction in Single-Wire Mode"]
    pub mod TXDIR {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input"]
            pub const TX_INPUT: u32 = 0;
            #[doc = "Output"]
            pub const TX_OUTPUT: u32 = 0x01;
        }
    }
    #[doc = "Receive Bit 9 Transmit Bit 8"]
    pub mod R9T8 {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Bit 8 Transmit Bit 9"]
    pub mod R8T9 {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data"]
pub mod DATA {
    #[doc = "Read receive FIFO bit 0 or write transmit FIFO bit 0"]
    pub mod R0T0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 1 or write transmit FIFO bit 1"]
    pub mod R1T1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 2 or write transmit FIFO bit 2"]
    pub mod R2T2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 3 or write transmit FIFO bit 3"]
    pub mod R3T3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 4 or write transmit FIFO bit 4"]
    pub mod R4T4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 5 or write transmit FIFO bit 5"]
    pub mod R5T5 {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 6 or write transmit FIFO bit 6"]
    pub mod R6T6 {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 7 or write transmit FIFO bit 7"]
    pub mod R7T7 {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 8 or write transmit FIFO bit 8"]
    pub mod R8T8 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Read receive FIFO bit 9 or write transmit FIFO bit 9"]
    pub mod R9T9 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LIN Break"]
    pub mod LINBRK {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not detected"]
            pub const NO_BREAK: u32 = 0;
            #[doc = "Detected"]
            pub const BREAK: u32 = 0x01;
        }
    }
    #[doc = "Idle Line"]
    pub mod IDLINE {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not idle"]
            pub const NO_IDLE: u32 = 0;
            #[doc = "Idle"]
            pub const IDLE: u32 = 0x01;
        }
    }
    #[doc = "Receive Buffer Empty"]
    pub mod RXEMPT {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Valid data"]
            pub const NOT_EMPTY: u32 = 0;
            #[doc = "Invalid data and empty"]
            pub const EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Frame Error Transmit Special Character"]
    pub mod FRETSC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received without a frame error on reads or transmits a normal character on writes"]
            pub const NO_ERROR: u32 = 0;
            #[doc = "Received with a frame error on reads or transmits an idle or break character on writes"]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Parity Error"]
    pub mod PARITYE {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received without a parity error"]
            pub const NO_PARITY: u32 = 0;
            #[doc = "Received with a parity error"]
            pub const PARITY: u32 = 0x01;
        }
    }
    #[doc = "Noisy Data Received"]
    pub mod NOISY {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Received without noise"]
            pub const NO_NOISE: u32 = 0;
            #[doc = "Received with noise"]
            pub const NOISE: u32 = 0x01;
        }
    }
}
#[doc = "Match Address"]
pub mod MATCH {
    #[doc = "Match Address 1"]
    pub mod MA1 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Match Address 2"]
    pub mod MA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MODEM IrDA"]
pub mod MODIR {
    #[doc = "Transmitter CTS Enable"]
    pub mod TXCTSE {
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
    #[doc = "Transmitter RTS Enable"]
    pub mod TXRTSE {
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
    #[doc = "Transmitter RTS Polarity"]
    pub mod TXRTSPOL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Active low"]
            pub const LOW: u32 = 0;
            #[doc = "Active high"]
            pub const HIGH: u32 = 0x01;
        }
    }
    #[doc = "Receiver RTS Enable"]
    pub mod RXRTSE {
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
    #[doc = "Transmit CTS Configuration"]
    pub mod TXCTSC {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Sampled at the start of each character"]
            pub const START: u32 = 0;
            #[doc = "Sampled when the transmitter is idle"]
            pub const IDLE: u32 = 0x01;
        }
    }
    #[doc = "Transmit CTS Source"]
    pub mod TXCTSSRC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The CTS_B pin"]
            pub const CTS: u32 = 0;
            #[doc = "An internal connection to the receiver address match result"]
            pub const MATCH: u32 = 0x01;
        }
    }
    #[doc = "Receive RTS Configuration"]
    pub mod RTSWATER {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter Narrow Pulse"]
    pub mod TNP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1 / OSR"]
            pub const ONE_SAMPLE: u32 = 0;
            #[doc = "2 / OSR"]
            pub const TWO_SAMPLE: u32 = 0x01;
            #[doc = "3 / OSR"]
            pub const THREE_SAMPLE: u32 = 0x02;
            #[doc = "4 / OSR"]
            pub const FOUR_SAMPLE: u32 = 0x03;
        }
    }
    #[doc = "IR Enable"]
    pub mod IREN {
        pub const offset: u32 = 18;
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
#[doc = "FIFO"]
pub mod FIFO {
    #[doc = "Receive FIFO Buffer Depth"]
    pub mod RXFIFOSIZE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const FIFO_1: u32 = 0;
            #[doc = "4"]
            pub const FIFO_4: u32 = 0x01;
            #[doc = "8"]
            pub const FIFO_8: u32 = 0x02;
            #[doc = "16"]
            pub const FIFO_16: u32 = 0x03;
            #[doc = "32"]
            pub const FIFO_32: u32 = 0x04;
            #[doc = "64"]
            pub const FIFO_64: u32 = 0x05;
            #[doc = "128"]
            pub const FIFO_128: u32 = 0x06;
            #[doc = "256"]
            pub const FIFO_256: u32 = 0x07;
        }
    }
    #[doc = "Receive FIFO Enable"]
    pub mod RXFE {
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
    #[doc = "Transmit FIFO Buffer Depth"]
    pub mod TXFIFOSIZE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "1"]
            pub const FIFO_1: u32 = 0;
            #[doc = "4"]
            pub const FIFO_4: u32 = 0x01;
            #[doc = "8"]
            pub const FIFO_8: u32 = 0x02;
            #[doc = "16"]
            pub const FIFO_16: u32 = 0x03;
            #[doc = "32"]
            pub const FIFO_32: u32 = 0x04;
            #[doc = "64"]
            pub const FIFO_64: u32 = 0x05;
            #[doc = "128"]
            pub const FIFO_128: u32 = 0x06;
            #[doc = "256"]
            pub const FIFO_256: u32 = 0x07;
        }
    }
    #[doc = "Transmit FIFO Enable"]
    pub mod TXFE {
        pub const offset: u32 = 7;
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
    #[doc = "Receive FIFO Underflow Interrupt Enable"]
    pub mod RXUFE {
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
    #[doc = "Transmit FIFO Overflow Interrupt Enable"]
    pub mod TXOFE {
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
    #[doc = "Receiver Idle Empty Enable"]
    pub mod RXIDEN {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for one character"]
            pub const IDLE_1: u32 = 0x01;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for two characters"]
            pub const IDLE_2: u32 = 0x02;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for four characters"]
            pub const IDLE_4: u32 = 0x03;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for eight characters"]
            pub const IDLE_8: u32 = 0x04;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 16 characters"]
            pub const IDLE_16: u32 = 0x05;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 32 characters"]
            pub const IDLE_32: u32 = 0x06;
            #[doc = "Enable STAT\\[RDRF\\] to become 1 because of partially filled FIFO when the receiver is idle for 64 characters"]
            pub const IDLE_64: u32 = 0x07;
        }
    }
    #[doc = "Receive FIFO Flush"]
    pub mod RXFLUSH {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "All data flushed out"]
            pub const RXFIFO_RST: u32 = 0x01;
        }
    }
    #[doc = "Transmit FIFO Flush"]
    pub mod TXFLUSH {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "All data flushed out"]
            pub const TXFIFO_RST: u32 = 0x01;
        }
    }
    #[doc = "Receiver FIFO Underflow Flag"]
    pub mod RXUF {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No underflow"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow"]
            pub const UNDERFLOW: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmitter FIFO Overflow Flag"]
    pub mod TXOF {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "No overflow"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow"]
            pub const OVERFLOW: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive FIFO Or Buffer Empty"]
    pub mod RXEMPT {
        pub const offset: u32 = 22;
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
    #[doc = "Transmit FIFO Or Buffer Empty"]
    pub mod TXEMPT {
        pub const offset: u32 = 23;
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
#[doc = "Watermark"]
pub mod WATER {
    #[doc = "Transmit Watermark"]
    pub mod TXWATER {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit Counter"]
    pub mod TXCOUNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Watermark"]
    pub mod RXWATER {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Receive Counter"]
    pub mod RXCOUNT {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Data Read-Only"]
pub mod DATARO {
    #[doc = "Receive Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "MODEM Control"]
pub mod MCR {
    #[doc = "Clear To Send"]
    pub mod CTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Data Set Ready"]
    pub mod DSR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Ring Indicator"]
    pub mod RIN {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Data Carrier Detect"]
    pub mod DCD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Disable interrupt"]
            pub const DISABLED: u32 = 0;
            #[doc = "Enable interrupt"]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Data Terminal Ready"]
    pub mod DTR {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Request To Send"]
    pub mod RTS {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
}
#[doc = "MODEM Status"]
pub mod MSR {
    #[doc = "Delta Clear To Send"]
    pub mod DCTS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delta Data Set Ready"]
    pub mod DDSR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delta Ring Indicator"]
    pub mod DRI {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Delta Data Carrier Detect"]
    pub mod DDCD {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {
            #[doc = "Did not change state"]
            pub const NO_CHANGE: u32 = 0;
            #[doc = "Changed state"]
            pub const CHANGE: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Clear To Send"]
    pub mod CTS {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Data Set Ready"]
    pub mod DSR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Ring Indicator"]
    pub mod RIN {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
    #[doc = "Data Carrier Detect"]
    pub mod DCD {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Logic one"]
            pub const LOGIC_ONE: u32 = 0;
            #[doc = "Logic zero"]
            pub const LOGIC_ZERO: u32 = 0x01;
        }
    }
}
#[doc = "Receiver Extended Idle"]
pub mod REIR {
    #[doc = "Idle Time"]
    pub mod IDTIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmitter Extended Idle"]
pub mod TEIR {
    #[doc = "Idle Time"]
    pub mod IDTIME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Half Duplex Control"]
pub mod HDCR {
    #[doc = "Transmit Stall"]
    pub mod TXSTALL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Does not become busy"]
            pub const RX_ACTIVE: u32 = 0x01;
        }
    }
    #[doc = "Receive Select"]
    pub mod RXSEL {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "RXD"]
            pub const PIN_RXD: u32 = 0;
            #[doc = "TXD"]
            pub const PIN_TXD: u32 = 0x01;
        }
    }
    #[doc = "Receive FIFO Write Mask"]
    pub mod RXWRMSK {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not mask"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Mask"]
            pub const TX_RTS: u32 = 0x01;
        }
    }
    #[doc = "Receive Mask"]
    pub mod RXMSK {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not mask"]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Mask"]
            pub const TX_RTS: u32 = 0x01;
        }
    }
    #[doc = "RTS Extended"]
    pub mod RTSEXT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout Control"]
pub mod TOCR {
    #[doc = "Timeout Enable"]
    pub mod TOEN {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Interrupt Enable"]
    pub mod TOIE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout Status"]
pub mod TOSR {
    #[doc = "Timeout Zero"]
    pub mod TOZ {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout Flag"]
    pub mod TOF {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {
            #[doc = "Not occurred"]
            pub const NOT_OCCURRED: u32 = 0;
            #[doc = "Occurred"]
            pub const OCCURRED: u32 = 0x01;
        }
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Timeout N"]
pub mod TIMEOUT {
    #[doc = "Timeout Value"]
    pub mod TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Idle Configuration"]
    pub mod CFG {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Becomes 1 after timeout characters are received"]
            pub const CNT_CHAR: u32 = 0;
            #[doc = "Becomes 1 when idle for timeout bit clocks"]
            pub const CNT_IDLE: u32 = 0x01;
            #[doc = "Becomes 1 when idle for timeout bit clocks following the next character"]
            pub const CNT_BUSY_IDLE: u32 = 0x02;
            #[doc = "Becomes 1 when idle for at least timeout bit clocks, but a new character is detected before the extended idle timeout is reached"]
            pub const CNT_CHAR_IDLE: u32 = 0x03;
        }
    }
}
#[doc = "Transmit Command Burst"]
pub mod TCBR {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Transmit Data Burst"]
pub mod TDBR {
    #[doc = "Data0"]
    pub mod DATA0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data1"]
    pub mod DATA1 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data2"]
    pub mod DATA2 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Data3"]
    pub mod DATA3 {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
