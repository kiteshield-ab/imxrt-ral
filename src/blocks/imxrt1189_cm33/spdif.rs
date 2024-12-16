#[doc = "SPDIF"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "SPDIF Configuration Register"]
    pub SCR: crate::RWRegister<u32>,
    #[doc = "CDText Control Register"]
    pub SRCD: crate::RWRegister<u32>,
    #[doc = "PhaseConfig Register"]
    pub SRPC: crate::RWRegister<u32>,
    #[doc = "InterruptEn Register"]
    pub SIE: crate::RWRegister<u32>,
    #[doc = "InterruptStat Register"]
    pub SIS: crate::RWRegister<u32>,
    #[doc = "SPDIFRxLeft Register"]
    pub SRL: crate::RORegister<u32>,
    #[doc = "SPDIFRxRight Register"]
    pub SRR: crate::RORegister<u32>,
    #[doc = "SPDIFRxCChannel_h Register"]
    pub SRCSH: crate::RORegister<u32>,
    #[doc = "SPDIFRxCChannel_l Register"]
    pub SRCSL: crate::RORegister<u32>,
    #[doc = "UchannelRx Register"]
    pub SRU: crate::RORegister<u32>,
    #[doc = "QchannelRx Register"]
    pub SRQ: crate::RORegister<u32>,
    #[doc = "SPDIFTxLeft Register"]
    pub STL: crate::WORegister<u32>,
    #[doc = "SPDIFTxRight Register"]
    pub STR: crate::WORegister<u32>,
    #[doc = "SPDIFTxCChannelCons_h Register"]
    pub STCSCH: crate::RWRegister<u32>,
    #[doc = "SPDIFTxCChannelCons_l Register"]
    pub STCSCL: crate::RWRegister<u32>,
    _reserved0: [u8; 0x08],
    #[doc = "FreqMeas Register"]
    pub SRFM: crate::RORegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "SPDIFTxClk Register"]
    pub STC: crate::RWRegister<u32>,
    _reserved2: [u8; 0x0c],
    #[doc = "SPDIF receive C channel register, bits 31-0"]
    pub SPDIFRXCCHANNEL_ADDR_31_0: crate::RORegister<u32>,
    #[doc = "SPDIF receive C channel register, bits 63-32"]
    pub SPDIFRXCCHANNEL_ADDR_63_32: crate::RORegister<u32>,
    #[doc = "SPDIF receive C channel register, bits 95-64"]
    pub SPDIFRXCCHANNEL_ADDR_95_64: crate::RORegister<u32>,
    #[doc = "SPDIF receive C channel register, bits 127-96"]
    pub SPDIFRXCCHANNEL_ADDR_127_96: crate::RORegister<u32>,
    #[doc = "SPDIF receive C channel register, bits 159-128"]
    pub SPDIFRXCCHANNEL_ADDR_159_128: crate::RORegister<u32>,
    #[doc = "SPDIF receive C channel register, bits 191-160"]
    pub SPDIFRXCCHANNEL_ADDR_191_160: crate::RORegister<u32>,
    #[doc = "SPDIF transmit C channel register, bits 31-0"]
    pub SPDIFTXCCHANNEL_ADDR_31_0: crate::RWRegister<u32>,
    #[doc = "SPDIF transmit C channel register, bits 63-32"]
    pub SPDIFTXCCHANNEL_ADDR_63_32: crate::RWRegister<u32>,
    #[doc = "SPDIF transmit C channel register, bits 95-64"]
    pub SPDIFTXCCHANNEL_ADDR_95_64: crate::RWRegister<u32>,
    #[doc = "SPDIF transmit C channel register, bits 127-96"]
    pub SPDIFTXCCHANNEL_ADDR_127_96: crate::RWRegister<u32>,
    #[doc = "SPDIF transmit C channel register, bits 159-128"]
    pub SPDIFTXCCHANNEL_ADDR_159_128: crate::RWRegister<u32>,
    #[doc = "SPDIF transmit C channel register, bits 191-160"]
    pub SPDIFTXCCHANNEL_ADDR_191_160: crate::RWRegister<u32>,
}
#[doc = "SPDIF Configuration Register"]
pub mod SCR {
    #[doc = "USrc_Sel"]
    pub mod USRC_SEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No embedded U channel"]
            pub const NONE: u32 = 0;
            #[doc = "U channel from SPDIF receive block (CD mode)"]
            pub const SPDIF_RXBLOCK: u32 = 0x01;
            #[doc = "U channel from on chip transmitter"]
            pub const CHIP_TRANSMIT: u32 = 0x03;
        }
    }
    #[doc = "TxSel"]
    pub mod TXSEL {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Off and output 0"]
            pub const OFF_OUT0: u32 = 0;
            #[doc = "Feed-through SPDIF_IN"]
            pub const FEEDTHRU001: u32 = 0x01;
            #[doc = "Tx Normal operation - From SPDIF Tx Block"]
            pub const NORMAL_OP: u32 = 0x05;
        }
    }
    #[doc = "ValCtrl"]
    pub mod VALCTRL {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Outgoing Validity always set"]
            pub const ALWAYS_SET: u32 = 0;
            #[doc = "Outgoing Validity always clear"]
            pub const ALWAYS_CLEAR: u32 = 0x01;
        }
    }
    #[doc = "InputSrcSel"]
    pub mod INPUTSRCSEL {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF_IN"]
            pub const SPDIF_IN1: u32 = 0;
            #[doc = "None"]
            pub const NONE_SEL_1: u32 = 0x01;
            #[doc = "None"]
            pub const NONE_SEL_2: u32 = 0x02;
            #[doc = "None"]
            pub const NONE_SEL_3: u32 = 0x03;
        }
    }
    #[doc = "DMA_TX_En"]
    pub mod DMA_TX_EN {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "DMA_Rx_En"]
    pub mod DMA_RX_EN {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxFIFO_Ctrl"]
    pub mod TXFIFO_CTRL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Send out digital zero on SPDIF Tx"]
            pub const SEND_ZERO: u32 = 0;
            #[doc = "Tx Normal operation"]
            pub const NORMAL: u32 = 0x01;
            #[doc = "Reset to 1 sample remaining"]
            pub const RESET_ONE: u32 = 0x02;
        }
    }
    #[doc = "soft_reset"]
    pub mod SOFT_RESET {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LOW_POWER"]
    pub mod LOW_POWER {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxFIFOEmpty_Sel"]
    pub mod TXFIFOEMPTY_SEL {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Empty interrupt if 0 sample in Tx left and right FIFOs"]
            pub const EMPTY_INT_0: u32 = 0;
            #[doc = "Empty interrupt if at most 4 sample in Tx left and right FIFOs"]
            pub const EMPTY_INT_4: u32 = 0x01;
            #[doc = "Empty interrupt if at most 8 sample in Tx left and right FIFOs"]
            pub const EMPTY_INT_8: u32 = 0x02;
            #[doc = "Empty interrupt if at most 12 sample in Tx left and right FIFOs"]
            pub const EMPTY_INT_12: u32 = 0x03;
        }
    }
    #[doc = "TxAutoSync"]
    pub mod TXAUTOSYNC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Tx FIFO auto sync off"]
            pub const OFF: u32 = 0;
            #[doc = "Tx FIFO auto sync on"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "RxAutoSync"]
    pub mod RXAUTOSYNC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Rx FIFO auto sync off"]
            pub const OFF: u32 = 0;
            #[doc = "RxFIFO auto sync on"]
            pub const ON: u32 = 0x01;
        }
    }
    #[doc = "RxFIFOFull_Sel"]
    pub mod RXFIFOFULL_SEL {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Full interrupt if at least 1 sample in Rx left and right FIFOs"]
            pub const FULL_INT_1: u32 = 0;
            #[doc = "Full interrupt if at least 4 sample in Rx left and right FIFOs"]
            pub const FULL_INT_4: u32 = 0x01;
            #[doc = "Full interrupt if at least 8 sample in Rx left and right FIFOs"]
            pub const FULL_INT_8: u32 = 0x02;
            #[doc = "Full interrupt if at least 16 sample in Rx left and right FIFO"]
            pub const FULL_INT_16: u32 = 0x03;
        }
    }
    #[doc = "RxFIFO_Rst"]
    pub mod RXFIFO_RST {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Reset register to 1 sample remaining"]
            pub const RESET_ONE: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO_Off_On"]
    pub mod RXFIFO_OFF_ON {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF Rx FIFO is on"]
            pub const ON_0: u32 = 0;
            #[doc = "SPDIF Rx FIFO is off. Does not accept data from interface"]
            pub const OFF_1: u32 = 0x01;
        }
    }
    #[doc = "RxFIFO_Ctrl"]
    pub mod RXFIFO_CTRL {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Normal operation"]
            pub const NORMAL: u32 = 0;
            #[doc = "Always read zero from Rx data register"]
            pub const ALWAYS_ZERO: u32 = 0x01;
        }
    }
    #[doc = "TXCChannel_192b_en"]
    pub mod TXCCHANNEL_192B_EN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF transmits 48 bits of C in audio stream. Other C bits in 49 to 192 frames are 0"]
            pub const DISABLE: u32 = 0;
            #[doc = "SPDIF transmits 192 bits of C in audio stream"]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "RXCChannel_192b_en"]
    pub mod RXCCHANNEL_192B_EN {
        pub const offset: u32 = 25;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "SPDIF receives only 48 bits of 192 C bits from input audio stream"]
            pub const DISABLE: u32 = 0;
            #[doc = "SPDIF receives 192 bits of C in audio stream"]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "CDText Control Register"]
pub mod SRCD {
    #[doc = "USyncMode"]
    pub mod USYNCMODE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Non-CD data"]
            pub const NON_CDDATA: u32 = 0;
            #[doc = "CD user channel subcode"]
            pub const CDUSER_CHSUBCODE: u32 = 0x01;
        }
    }
}
#[doc = "PhaseConfig Register"]
pub mod SRPC {
    #[doc = "GainSel"]
    pub mod GAINSEL {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "24*(2**10)"]
            pub const GAINSEL_0B000: u32 = 0;
            #[doc = "16*(2**10)"]
            pub const GAINSEL_0B001: u32 = 0x01;
            #[doc = "12*(2**10)"]
            pub const GAINSEL_0B010: u32 = 0x02;
            #[doc = "8*(2**10)"]
            pub const GAINSEL_0B011: u32 = 0x03;
            #[doc = "6*(2**10)"]
            pub const GAINSEL_0B100: u32 = 0x04;
            #[doc = "4*(2**10)"]
            pub const GAINSEL_0B101: u32 = 0x05;
            #[doc = "3*(2**10)"]
            pub const GAINSEL_0B110: u32 = 0x06;
        }
    }
    #[doc = "LOCK"]
    pub mod LOCK {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ClkSrc_Sel"]
    pub mod CLKSRC_SEL {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
            pub const CLKSRC_0B0000: u32 = 0;
            #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
            pub const CLKSRC_0B0001: u32 = 0x01;
            #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
            pub const CLKSRC_0B0011: u32 = 0x03;
            #[doc = "REF_CLK_32K (XTALOSC)"]
            pub const CLKSRC_0B0101: u32 = 0x05;
            #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
            pub const CLKSRC_0B0110: u32 = 0x06;
            #[doc = "SPDIF_EXT_CLK"]
            pub const CLKSRC_0B1000: u32 = 0x08;
        }
    }
}
#[doc = "InterruptEn Register"]
pub mod SIE {
    #[doc = "RxFIFOFul"]
    pub mod RXFIFOFUL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxEm"]
    pub mod TXEM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LockLoss"]
    pub mod LOCKLOSS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFOResyn"]
    pub mod RXFIFORESYN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFOUnOv"]
    pub mod RXFIFOUNOV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UQErr"]
    pub mod UQERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UQSync"]
    pub mod UQSYNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QRxOv"]
    pub mod QRXOV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QRxFul"]
    pub mod QRXFUL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "URxOv"]
    pub mod URXOV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "URxFul"]
    pub mod URXFUL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BitErr"]
    pub mod BITERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SymErr"]
    pub mod SYMERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ValNoGood"]
    pub mod VALNOGOOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CNew"]
    pub mod CNEW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxResyn"]
    pub mod TXRESYN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxUnOv"]
    pub mod TXUNOV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "InterruptStat Register"]
pub mod SIS {
    #[doc = "RxFIFOFul"]
    pub mod RXFIFOFUL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxEm"]
    pub mod TXEM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "LockLoss"]
    pub mod LOCKLOSS {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFOResyn"]
    pub mod RXFIFORESYN {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "RxFIFOUnOv"]
    pub mod RXFIFOUNOV {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UQErr"]
    pub mod UQERR {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "UQSync"]
    pub mod UQSYNC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QRxOv"]
    pub mod QRXOV {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "QRxFul"]
    pub mod QRXFUL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "URxOv"]
    pub mod URXOV {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "URxFul"]
    pub mod URXFUL {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "BitErr"]
    pub mod BITERR {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SymErr"]
    pub mod SYMERR {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "ValNoGood"]
    pub mod VALNOGOOD {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "CNew"]
    pub mod CNEW {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxResyn"]
    pub mod TXRESYN {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "TxUnOv"]
    pub mod TXUNOV {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Lock"]
    pub mod LOCK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxLeft Register"]
pub mod SRL {
    #[doc = "RxDataLeft"]
    pub mod RXDATALEFT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxRight Register"]
pub mod SRR {
    #[doc = "RxDataRight"]
    pub mod RXDATARIGHT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxCChannel_h Register"]
pub mod SRCSH {
    #[doc = "RxCChannel_h"]
    pub mod RXCCHANNEL_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFRxCChannel_l Register"]
pub mod SRCSL {
    #[doc = "RxCChannel_l"]
    pub mod RXCCHANNEL_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "UchannelRx Register"]
pub mod SRU {
    #[doc = "RxUChannel"]
    pub mod RXUCHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "QchannelRx Register"]
pub mod SRQ {
    #[doc = "RxQChannel"]
    pub mod RXQCHANNEL {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxLeft Register"]
pub mod STL {
    #[doc = "TxDataLeft"]
    pub mod TXDATALEFT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxRight Register"]
pub mod STR {
    #[doc = "TxDataRight"]
    pub mod TXDATARIGHT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxCChannelCons_h Register"]
pub mod STCSCH {
    #[doc = "TxCChannelCons_h"]
    pub mod TXCCHANNELCONS_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxCChannelCons_l Register"]
pub mod STCSCL {
    #[doc = "TxCChannelCons_l"]
    pub mod TXCCHANNELCONS_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "FreqMeas Register"]
pub mod SRFM {
    #[doc = "FreqMeas"]
    pub mod FREQMEAS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIFTxClk Register"]
pub mod STC {
    #[doc = "TxClk_DF"]
    pub mod TXCLK_DF {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "divider factor is 1"]
            pub const DIV1: u32 = 0;
            #[doc = "divider factor is 2"]
            pub const DIV2: u32 = 0x01;
            #[doc = "divider factor is 128"]
            pub const DIV128: u32 = 0x7f;
        }
    }
    #[doc = "tx_all_clk_en"]
    pub mod TX_ALL_CLK_EN {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "disable transfer clock."]
            pub const DISABLE: u32 = 0;
            #[doc = "enable transfer clock."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "TxClk_Source"]
    pub mod TXCLK_SOURCE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "SYSCLK_DF"]
    pub mod SYSCLK_DF {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "no clock signal"]
            pub const NO_CLK: u32 = 0;
            #[doc = "divider factor is 2"]
            pub const DIV2: u32 = 0x01;
            #[doc = "divider factor is 512"]
            pub const DIV512: u32 = 0x01ff;
        }
    }
}
#[doc = "SPDIF receive C channel register, bits 31-0"]
pub mod SPDIFRXCCHANNEL_ADDR_31_0 {
    #[doc = "RxCChannel_Addr_31_0"]
    pub mod RXCCHANNEL_ADDR_31_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF receive C channel register, bits 63-32"]
pub mod SPDIFRXCCHANNEL_ADDR_63_32 {
    #[doc = "RxCChannel_Addr_63_32"]
    pub mod RXCCHANNEL_ADDR_63_32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF receive C channel register, bits 95-64"]
pub mod SPDIFRXCCHANNEL_ADDR_95_64 {
    #[doc = "RxCChannel_Addr_95_64"]
    pub mod RXCCHANNEL_ADDR_95_64 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF receive C channel register, bits 127-96"]
pub mod SPDIFRXCCHANNEL_ADDR_127_96 {
    #[doc = "RxCChannel_Addr_127_96"]
    pub mod RXCCHANNEL_ADDR_127_96 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF receive C channel register, bits 159-128"]
pub mod SPDIFRXCCHANNEL_ADDR_159_128 {
    #[doc = "RxCChannel_Addr_159_128"]
    pub mod RXCCHANNEL_ADDR_159_128 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF receive C channel register, bits 191-160"]
pub mod SPDIFRXCCHANNEL_ADDR_191_160 {
    #[doc = "RxCChannel_Addr_191_160"]
    pub mod RXCCHANNEL_ADDR_191_160 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF transmit C channel register, bits 31-0"]
pub mod SPDIFTXCCHANNEL_ADDR_31_0 {
    #[doc = "TxCChannel_Addr_31_0"]
    pub mod TXCCHANNEL_ADDR_31_0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF transmit C channel register, bits 63-32"]
pub mod SPDIFTXCCHANNEL_ADDR_63_32 {
    #[doc = "TxCChannel_Addr_63_32"]
    pub mod TXCCHANNEL_ADDR_63_32 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF transmit C channel register, bits 95-64"]
pub mod SPDIFTXCCHANNEL_ADDR_95_64 {
    #[doc = "TxCChannel_Addr_95_64"]
    pub mod TXCCHANNEL_ADDR_95_64 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF transmit C channel register, bits 127-96"]
pub mod SPDIFTXCCHANNEL_ADDR_127_96 {
    #[doc = "TxCChannel_Addr_127_96"]
    pub mod TXCCHANNEL_ADDR_127_96 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF transmit C channel register, bits 159-128"]
pub mod SPDIFTXCCHANNEL_ADDR_159_128 {
    #[doc = "TxCChannel_Addr_159_128"]
    pub mod TXCCHANNEL_ADDR_159_128 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "SPDIF transmit C channel register, bits 191-160"]
pub mod SPDIFTXCCHANNEL_ADDR_191_160 {
    #[doc = "TxCChannel_Addr_191_160"]
    pub mod TXCCHANNEL_ADDR_191_160 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
