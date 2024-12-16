#[doc = "ASRC"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "ASRC Control"]
    pub ASRCTR: crate::RWRegister<u32>,
    #[doc = "ASRC Interrupt Enable"]
    pub ASRIER: crate::RWRegister<u32>,
    _reserved0: [u8; 0x04],
    #[doc = "ASRC Channel Number Configuration"]
    pub ASRCNCR: crate::RWRegister<u32>,
    #[doc = "ASRC Filter Configuration Status"]
    pub ASRCFG: crate::RWRegister<u32>,
    #[doc = "ASRC Clock Source"]
    pub ASRCSR: crate::RWRegister<u32>,
    #[doc = "ASRC Clock Divider 1"]
    pub ASRCDR1: crate::RWRegister<u32>,
    #[doc = "ASRC Clock Divider 2"]
    pub ASRCDR2: crate::RWRegister<u32>,
    #[doc = "ASRC Status"]
    pub ASRSTR: crate::RORegister<u32>,
    _reserved1: [u8; 0x1c],
    #[doc = "ASRC Parameter x"]
    pub ASRPM: [crate::RWRegister<u32>; 5usize],
    #[doc = "ASRC Task Queue FIFO 1"]
    pub ASRTFR1: crate::RWRegister<u32>,
    _reserved2: [u8; 0x04],
    #[doc = "ASRC Channel Counter"]
    pub ASRCCR: crate::RWRegister<u32>,
    #[doc = "ASRC Data Input for Pair x"]
    pub ASRDIA: crate::WORegister<u32>,
    #[doc = "ASRC Data Output for Pair x"]
    pub ASRDOA: crate::RORegister<u32>,
    #[doc = "ASRC Data Input for Pair x"]
    pub ASRDIB: crate::WORegister<u32>,
    #[doc = "ASRC Data Output for Pair x"]
    pub ASRDOB: crate::RORegister<u32>,
    #[doc = "ASRC Data Input for Pair x"]
    pub ASRDIC: crate::WORegister<u32>,
    #[doc = "ASRC Data Output for Pair x"]
    pub ASRDOC: crate::RORegister<u32>,
    _reserved3: [u8; 0x08],
    #[doc = "ASRC Ideal Ratio for Pair A-High Part"]
    pub ASRIDRHA: crate::RWRegister<u32>,
    #[doc = "ASRC Ideal Ratio for Pair A -Low Part"]
    pub ASRIDRLA: crate::RWRegister<u32>,
    #[doc = "ASRC Ideal Ratio for Pair B-High Part"]
    pub ASRIDRHB: crate::RWRegister<u32>,
    #[doc = "ASRC Ideal Ratio for Pair B-Low Part"]
    pub ASRIDRLB: crate::RWRegister<u32>,
    #[doc = "ASRC Ideal Ratio for Pair C-High Part"]
    pub ASRIDRHC: crate::RWRegister<u32>,
    #[doc = "ASRC Ideal Ratio for Pair C-Low Part"]
    pub ASRIDRLC: crate::RWRegister<u32>,
    #[doc = "ASRC 76 kHz Period"]
    pub ASR76K: crate::RWRegister<u32>,
    #[doc = "ASRC 56 kHz Period"]
    pub ASR56K: crate::RWRegister<u32>,
    #[doc = "ASRC Misc Control for Pair A"]
    pub ASRMCRA: crate::RWRegister<u32>,
    #[doc = "ASRC FIFO Status for Pair A"]
    pub ASRFSTA: crate::RORegister<u32>,
    #[doc = "ASRC Misc Control for Pair B"]
    pub ASRMCRB: crate::RWRegister<u32>,
    #[doc = "ASRC FIFO Status for Pair B"]
    pub ASRFSTB: crate::RORegister<u32>,
    #[doc = "ASRC Misc Control for Pair C"]
    pub ASRMCRC: crate::RWRegister<u32>,
    #[doc = "ASRC FIFO Status for Pair C"]
    pub ASRFSTC: crate::RORegister<u32>,
    _reserved4: [u8; 0x08],
    #[doc = "ASRC Misc Control 1 for Pair X"]
    pub ASRMCR1: [crate::RWRegister<u32>; 3usize],
}
#[doc = "ASRC Control"]
pub mod ASRCTR {
    #[doc = "ASRC Enable"]
    pub mod ASRCEN {
        pub const offset: u32 = 0;
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
    #[doc = "ASRC Enable A"]
    pub mod ASREA {
        pub const offset: u32 = 1;
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
    #[doc = "ASRC Enable B"]
    pub mod ASREB {
        pub const offset: u32 = 2;
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
    #[doc = "ASRC Enable C"]
    pub mod ASREC {
        pub const offset: u32 = 3;
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
    #[doc = "Software Reset"]
    pub mod SRST {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ASRC Software reset cleared"]
            pub const CLEARED: u32 = 0;
            #[doc = "ASRC Software reset generated. NOTE: This is a self-clear bit"]
            pub const RESET: u32 = 0x01;
        }
    }
    #[doc = "Use Ideal Ratio for Pair A"]
    pub mod IDRA {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ASRC internal measured ratio is used"]
            pub const IDRA_MEASURED: u32 = 0;
            #[doc = "Ideal ratio from the interface register ASRIDRHA, ASRIDRLA is used"]
            pub const IDRA_IDEAL: u32 = 0x01;
        }
    }
    #[doc = "Use Ratio for Pair A"]
    pub mod USRA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not use ratio as the input to ASRC for pair A"]
            pub const USE_RATIO_NO: u32 = 0;
            #[doc = "Use ratio as the input to ASRC for pair A"]
            pub const USE_RATIO: u32 = 0x01;
        }
    }
    #[doc = "Use Ideal Ratio for Pair B"]
    pub mod IDRB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ASRC internal measured ratio is used"]
            pub const IDRA_MEASURED: u32 = 0;
            #[doc = "Ideal ratio from the interface register ASRIDRHB, ASRIDRLB is used"]
            pub const IDRA_IDEAL: u32 = 0x01;
        }
    }
    #[doc = "Use Ratio for Pair B"]
    pub mod USRB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not use ratio as the input to ASRC for pair B"]
            pub const USE_RATIO_NO: u32 = 0;
            #[doc = "Use ratio as the input to ASRC for pair B"]
            pub const USE_RATIO: u32 = 0x01;
        }
    }
    #[doc = "Use Ideal Ratio for Pair C"]
    pub mod IDRC {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ASRC internal measured ratio is used"]
            pub const IDRA_MEASURED: u32 = 0;
            #[doc = "Ideal ratio from the interface register ASRIDRHC, ASRIDRLC is used"]
            pub const IDRA_IDEAL: u32 = 0x01;
        }
    }
    #[doc = "Use Ratio for Pair C"]
    pub mod USRC {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not use ratio as the input to ASRC for pair C"]
            pub const USE_RATIO_NO: u32 = 0;
            #[doc = "Use ratio as the input to ASRC for pair C"]
            pub const USE_RATIO: u32 = 0x01;
        }
    }
    #[doc = "ASRC Pair A Automatic Selection For Processing Options"]
    pub mod ATSA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair A does not automatically update its pre-processing and post-processing options"]
            pub const NO_AUTO_SELECT: u32 = 0;
            #[doc = "Pair A automatically updates its pre-processing and post-processing options"]
            pub const AUTO_SELECT: u32 = 0x01;
        }
    }
    #[doc = "ASRC Pair B Automatic Selection For Processing Options"]
    pub mod ATSB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair B does not automatically update its pre-processing and post-processing options"]
            pub const NO_AUTO_SELECT: u32 = 0;
            #[doc = "Pair B automatically updates its pre-processing and post-processing options"]
            pub const AUTO_SELECT: u32 = 0x01;
        }
    }
    #[doc = "ASRC Pair C Automatic Selection For Processing Options"]
    pub mod ATSC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair C does not automatically update its pre-processing and post-processing options"]
            pub const NO_AUTO_SELECT: u32 = 0;
            #[doc = "Pair C automatically updates its pre-processing and post-processing options"]
            pub const AUTO_SELECT: u32 = 0x01;
        }
    }
}
#[doc = "ASRC Interrupt Enable"]
pub mod ASRIER {
    #[doc = "Pair A Data Input Interrupt Enable"]
    pub mod ADIEA {
        pub const offset: u32 = 0;
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
    #[doc = "Pair B Data Input Interrupt Enable"]
    pub mod ADIEB {
        pub const offset: u32 = 1;
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
    #[doc = "Pair C Data Input Interrupt Enable"]
    pub mod ADIEC {
        pub const offset: u32 = 2;
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
    #[doc = "Pair A Data Output Interrupt Enable"]
    pub mod ADOEA {
        pub const offset: u32 = 3;
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
    #[doc = "Pair B Data Output Interrupt Enable"]
    pub mod ADOEB {
        pub const offset: u32 = 4;
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
    #[doc = "Pair C Data Output Interrupt Enable"]
    pub mod ADOEC {
        pub const offset: u32 = 5;
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
    #[doc = "Overload Interrupt Enable"]
    pub mod AOLIE {
        pub const offset: u32 = 6;
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
    #[doc = "FP in Wait State Interrupt Enable"]
    pub mod AFPWE {
        pub const offset: u32 = 7;
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
}
#[doc = "ASRC Channel Number Configuration"]
pub mod ASRCNCR {
    #[doc = "Number of A Channels"]
    pub mod ANCA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 channels in A (Pair A is disabled)"]
            pub const ZERO_CHAN: u32 = 0;
            #[doc = "1 channel in A"]
            pub const ONE_CHAN: u32 = 0x01;
            #[doc = "2 channels in A"]
            pub const TWO_CHAN: u32 = 0x02;
            #[doc = "3 channels in A"]
            pub const THREE_CHAN: u32 = 0x03;
            #[doc = "4 channels in A"]
            pub const FOUR_CHAN: u32 = 0x04;
            #[doc = "5 channels in A"]
            pub const FIVE_CHAN: u32 = 0x05;
            #[doc = "6 channels in A"]
            pub const SIX_CHAN: u32 = 0x06;
            #[doc = "7 channels in A"]
            pub const SEVEN_CHAN: u32 = 0x07;
            #[doc = "8 channels in A"]
            pub const EIGHT_CHAN: u32 = 0x08;
            #[doc = "9 channels in A"]
            pub const NINE_CHAN: u32 = 0x09;
            #[doc = "10 channels in A"]
            pub const TEN_CHAN: u32 = 0x0a;
            #[doc = "Should not be used."]
            pub const NOT_USED_11: u32 = 0x0b;
            #[doc = "Should not be used."]
            pub const NOT_USED_12: u32 = 0x0c;
            #[doc = "Should not be used."]
            pub const NOT_USED_13: u32 = 0x0d;
            #[doc = "Should not be used."]
            pub const NOT_USED_14: u32 = 0x0e;
            #[doc = "Should not be used."]
            pub const NOT_USED_15: u32 = 0x0f;
        }
    }
    #[doc = "Number of B Channels"]
    pub mod ANCB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 channels in B (Pair B is disabled)"]
            pub const ZERO_CHAN: u32 = 0;
            #[doc = "1 channel in B"]
            pub const ONE_CHAN: u32 = 0x01;
            #[doc = "2 channels in B"]
            pub const TWO_CHAN: u32 = 0x02;
            #[doc = "3 channels in B"]
            pub const THREE_CHAN: u32 = 0x03;
            #[doc = "4 channels in B"]
            pub const FOUR_CHAN: u32 = 0x04;
            #[doc = "5 channels in B"]
            pub const FIVE_CHAN: u32 = 0x05;
            #[doc = "6 channels in B"]
            pub const SIX_CHAN: u32 = 0x06;
            #[doc = "7 channels in B"]
            pub const SEVEN_CHAN: u32 = 0x07;
            #[doc = "8 channels in B"]
            pub const EIGHT_CHAN: u32 = 0x08;
            #[doc = "9 channels in B"]
            pub const NINE_CHAN: u32 = 0x09;
            #[doc = "10 channels in B"]
            pub const TEN_CHAN: u32 = 0x0a;
            #[doc = "Should not be used."]
            pub const NOT_USED_11: u32 = 0x0b;
            #[doc = "Should not be used."]
            pub const NOT_USED_12: u32 = 0x0c;
            #[doc = "Should not be used."]
            pub const NOT_USED_13: u32 = 0x0d;
            #[doc = "Should not be used."]
            pub const NOT_USED_14: u32 = 0x0e;
            #[doc = "Should not be used."]
            pub const NOT_USED_15: u32 = 0x0f;
        }
    }
    #[doc = "Number of C Channels"]
    pub mod ANCC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "0 channels in C (Pair C is disabled)"]
            pub const ZERO_CHAN: u32 = 0;
            #[doc = "1 channel in C"]
            pub const ONE_CHAN: u32 = 0x01;
            #[doc = "2 channels in C"]
            pub const TWO_CHAN: u32 = 0x02;
            #[doc = "3 channels in C"]
            pub const THREE_CHAN: u32 = 0x03;
            #[doc = "4 channels in C"]
            pub const FOUR_CHAN: u32 = 0x04;
            #[doc = "5 channels in C"]
            pub const FIVE_CHAN: u32 = 0x05;
            #[doc = "6 channels in C"]
            pub const SIX_CHAN: u32 = 0x06;
            #[doc = "7 channels in C"]
            pub const SEVEN_CHAN: u32 = 0x07;
            #[doc = "8 channels in C"]
            pub const EIGHT_CHAN: u32 = 0x08;
            #[doc = "9 channels in C"]
            pub const NINE_CHAN: u32 = 0x09;
            #[doc = "10 channels in C"]
            pub const TEN_CHAN: u32 = 0x0a;
            #[doc = "Should not be used."]
            pub const NOT_USED_11: u32 = 0x0b;
            #[doc = "Should not be used."]
            pub const NOT_USED_12: u32 = 0x0c;
            #[doc = "Should not be used."]
            pub const NOT_USED_13: u32 = 0x0d;
            #[doc = "Should not be used."]
            pub const NOT_USED_14: u32 = 0x0e;
            #[doc = "Should not be used."]
            pub const NOT_USED_15: u32 = 0x0f;
        }
    }
}
#[doc = "ASRC Filter Configuration Status"]
pub mod ASRCFG {
    #[doc = "Pre-Processing Configuration for Conversion Pair A"]
    pub mod PREMODA {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
            #[doc = "Select passthrough mode. In this case, POSTMODA\\[1:0\\] have no use."]
            pub const PASSTHRU: u32 = 0x03;
        }
    }
    #[doc = "Post-Processing Configuration for Conversion Pair A"]
    pub mod POSTMODA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
        }
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair B"]
    pub mod PREMODB {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
            #[doc = "Select passthrough mode. In this case, POSTMODB\\[1:0\\] have no use."]
            pub const PASSTHRU: u32 = 0x03;
        }
    }
    #[doc = "Post-Processing Configuration for Conversion Pair B"]
    pub mod POSTMODB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
        }
    }
    #[doc = "Pre-Processing Configuration for Conversion Pair C"]
    pub mod PREMODC {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Upsampling-by-2"]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection"]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2"]
            pub const DOWNSAMP_2: u32 = 0x02;
            #[doc = "Select passthrough mode. In this case, POSTMODC\\[1:0\\] have no use."]
            pub const PASSTHRU: u32 = 0x03;
        }
    }
    #[doc = "Post-Processing Configuration for Conversion Pair C"]
    pub mod POSTMODC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Select Upsampling-by-2 as defined in Signal Processing Flow."]
            pub const UPSAMP_2: u32 = 0;
            #[doc = "Select Direct-Connection as defined in Signal Processing Flow."]
            pub const DIRECT_CONNECT: u32 = 0x01;
            #[doc = "Select Downsampling-by-2 as defined in Signal Processing Flow."]
            pub const DOWNSAMP_2: u32 = 0x02;
        }
    }
    #[doc = "Not Use Default Parameters for RAM-stored Parameters For Conversion Pair A"]
    pub mod NDPRA {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
            pub const USE_DEFAULT: u32 = 0;
            #[doc = "Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM."]
            pub const NOT_DEFAULT: u32 = 0x01;
        }
    }
    #[doc = "Not Use Default Parameters for RAM-Stored Parameters For Conversion Pair B"]
    pub mod NDPRB {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
            pub const USE_DEFAULT: u32 = 0;
            #[doc = "Don't use default parameters for RAM-stored parameter. Use the parameters already stored in RAM."]
            pub const NOT_DEFAULT: u32 = 0x01;
        }
    }
    #[doc = "Not Use Default Parameters for RAM-Stored Parameters For Conversion Pair C"]
    pub mod NDPRC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use default parameters for RAM-stored parameters. Override any parameters already in RAM."]
            pub const USE_DEFAULT: u32 = 0;
            #[doc = "Don't use default parameters for RAM-stored parameters. Use the parameters already stored in RAM."]
            pub const NOT_DEFAULT: u32 = 0x01;
        }
    }
    #[doc = "Initialization for Conversion Pair A is served"]
    pub mod INIRQA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization for Conversion Pair A not served"]
            pub const INIT_NOTSERVED: u32 = 0;
            #[doc = "Initialization for Conversion Pair A served"]
            pub const INIT_SERVED: u32 = 0x01;
        }
    }
    #[doc = "Initialization for Conversion Pair B is Served"]
    pub mod INIRQB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization for Conversion Pair B not served"]
            pub const INIT_NOTSERVED: u32 = 0;
            #[doc = "Initialization for Conversion Pair B served"]
            pub const INIT_SERVED: u32 = 0x01;
        }
    }
    #[doc = "Initialization for Conversion Pair C is Served"]
    pub mod INIRQC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Initialization for Conversion Pair C not served"]
            pub const INIT_NOTSERVED: u32 = 0;
            #[doc = "Initialization for Conversion Pair C served"]
            pub const INIT_SERVED: u32 = 0x01;
        }
    }
}
#[doc = "ASRC Clock Source"]
pub mod ASRCSR {
    #[doc = "Input Clock Source A"]
    pub mod AICSA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "Bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "Bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "Bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "Bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "Bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "Bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "Bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "Bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "Bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "Bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "Bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "Bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "Bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "Bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "Clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
    }
    #[doc = "Input Clock Source B"]
    pub mod AICSB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "Bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "Bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "Bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "Bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "Bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "Bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "Bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "Bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "Bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "Bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "Bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "Bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "Bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "Bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "Clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
    }
    #[doc = "Input Clock Source C"]
    pub mod AICSC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "Bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "Bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "Bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "Bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "Bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "Bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "Bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "Bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "Bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "Bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "Bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "Bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "Bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "Bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "Clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
    }
    #[doc = "Output Clock Source A"]
    pub mod AOCSA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "Bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "Bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "Bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "Bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "Bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "Bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "Bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "Bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "Bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "Bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "Bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "Bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "Bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "Bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "Clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
    }
    #[doc = "Output Clock Source B"]
    pub mod AOCSB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "Bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "Bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "Bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "Bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "Bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "Bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "Bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "Bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "Bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "Bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "Bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "Bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "Bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "Bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "Clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
    }
    #[doc = "Output Clock Source C"]
    pub mod AOCSC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Bit clock 0"]
            pub const BITCLK0: u32 = 0;
            #[doc = "Bit clock 1"]
            pub const BITCLK1: u32 = 0x01;
            #[doc = "Bit clock 2"]
            pub const BITCLK2: u32 = 0x02;
            #[doc = "Bit clock 3"]
            pub const BITCLK3: u32 = 0x03;
            #[doc = "Bit clock 4"]
            pub const BITCLK4: u32 = 0x04;
            #[doc = "Bit clock 5"]
            pub const BITCLK5: u32 = 0x05;
            #[doc = "Bit clock 6"]
            pub const BITCLK6: u32 = 0x06;
            #[doc = "Bit clock 7"]
            pub const BITCLK7: u32 = 0x07;
            #[doc = "Bit clock 8"]
            pub const BITCLK8: u32 = 0x08;
            #[doc = "Bit clock 9"]
            pub const BITCLK9: u32 = 0x09;
            #[doc = "Bit clock A"]
            pub const BITCLKA: u32 = 0x0a;
            #[doc = "Bit clock B"]
            pub const BITCLKB: u32 = 0x0b;
            #[doc = "Bit clock C"]
            pub const BITCLKC: u32 = 0x0c;
            #[doc = "Bit clock D"]
            pub const BITCLKD: u32 = 0x0d;
            #[doc = "Bit clock E"]
            pub const BITCLKE: u32 = 0x0e;
            #[doc = "Clock disabled, connected to zero"]
            pub const CLK_DISABLED: u32 = 0x0f;
        }
    }
}
#[doc = "ASRC Clock Divider 1"]
pub mod ASRCDR1 {
    #[doc = "Input Clock Prescaler A"]
    pub mod AICPA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Clock Divider A"]
    pub mod AICDA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Clock Prescaler B"]
    pub mod AICPB {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Clock Divider B"]
    pub mod AICDB {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Clock Prescaler A"]
    pub mod AOCPA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Clock Divider A"]
    pub mod AOCDA {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Clock Prescaler B"]
    pub mod AOCPB {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Clock Divider B"]
    pub mod AOCDB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Clock Divider 2"]
pub mod ASRCDR2 {
    #[doc = "Input Clock Prescaler C"]
    pub mod AICPC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input Clock Divider C"]
    pub mod AICDC {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Clock Prescaler C"]
    pub mod AOCPC {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output Clock Divider C"]
    pub mod AOCDC {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Status"]
pub mod ASRSTR {
    #[doc = "Number of Data in Input Data Buffer A is Less than Threshold"]
    pub mod AIDEA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The threshold has been met and no data input A interrupt is generated"]
            pub const THRESH_MET: u32 = 0;
            #[doc = "When AIDEA is set, the ASRC generates data input A interrupt request to the processor if ASRIER\\[AIDEA\\] = 1"]
            pub const LESSTHAN_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Number of Data in Input Data Buffer B is Less than Threshold"]
    pub mod AIDEB {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The threshold has been met and no data input B interrupt is generated"]
            pub const THRESH_MET: u32 = 0;
            #[doc = "When AIDEB is set, the ASRC generates data input B interrupt request to the processor if ASRIER\\[AIDEB\\] = 1"]
            pub const LESSTHAN_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Number of Data in Input Data Buffer C is Less than Threshold"]
    pub mod AIDEC {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The threshold has been met and no data input C interrupt is generated"]
            pub const THRESH_MET: u32 = 0;
            #[doc = "When AIDEC is set, the ASRC generates data input C interrupt request to the processor if ASRIER\\[AIDEC\\] = 1"]
            pub const LESSTHAN_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Number of Data in Output Data Buffer A is Greater than Threshold"]
    pub mod AODFA {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The threshold has not yet been met and no data output A interrupt is generated"]
            pub const THRESH_NOTMET: u32 = 0;
            #[doc = "When AODFA is set, the ASRC generates data output A interrupt request to the processor if ASRIER\\[ADOEA\\] = 1"]
            pub const GREATERTHAN_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Number of data in Output Data Buffer B is Greater than Threshold"]
    pub mod AODFB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The threshold has not yet been met and no data output B interrupt is generated"]
            pub const THRESH_NOTMET: u32 = 0;
            #[doc = "When AODFB is set, the ASRC generates data output B interrupt request to the processor if ASRIER\\[ADOEB\\] = 1"]
            pub const GREATERTHAN_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Number of data in Output Data Buffer C is Greater than Threshold"]
    pub mod AODFC {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The threshold has not yet been met and no data output C interrupt is generated"]
            pub const THRESH_NOTMET: u32 = 0;
            #[doc = "When AODFC is set, the ASRC generates data output C interrupt request to the processor if ASRIER\\[ADOEC\\] = 1"]
            pub const GREATERTHAN_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Overload Error Flag"]
    pub mod AOLE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No overload"]
            pub const TASK_OK: u32 = 0;
            #[doc = "Task rate is too high"]
            pub const TOO_HIGH: u32 = 0x01;
        }
    }
    #[doc = "FP is in Wait States"]
    pub mod FPWT {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "ASRC is not in wait state"]
            pub const NO_WAITSTATE: u32 = 0;
            #[doc = "ASRC is in wait state"]
            pub const WAITSTATE: u32 = 0x01;
        }
    }
    #[doc = "Input Data Buffer A has Underflowed"]
    pub mod AIDUA {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Underflow in Input data buffer A"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow in Input data buffer A"]
            pub const UNDERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Input Data Buffer B has Underflowed"]
    pub mod AIDUB {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Underflow in Input data buffer B"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow in Input data buffer B"]
            pub const UNDERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Input Data Buffer C has Underflowed"]
    pub mod AIDUC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Underflow in Input data buffer C"]
            pub const NO_UNDERFLOW: u32 = 0;
            #[doc = "Underflow in Input data buffer C"]
            pub const UNDERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Output Data Buffer A has Overflowed"]
    pub mod AODOA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Overflow in Output data buffer A"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow in Output data buffer A"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Output Data Buffer B has Overflowed"]
    pub mod AODOB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Overflow in Output data buffer B"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow in Output data buffer B"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Output Data Buffer C has Overflowed"]
    pub mod AODOC {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No Overflow in Output data buffer C"]
            pub const NO_OVERFLOW: u32 = 0;
            #[doc = "Overflow in Output data buffer C"]
            pub const OVERFLOW: u32 = 0x01;
        }
    }
    #[doc = "Pair A Input Task Overload"]
    pub mod AIOLA {
        pub const offset: u32 = 14;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair A input task is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair A input task is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Pair B Input Task Overload"]
    pub mod AIOLB {
        pub const offset: u32 = 15;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair B input task is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair B input task is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Pair C Input Task Overload"]
    pub mod AIOLC {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair C input task is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair C input task is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Pair A Output Task Overload"]
    pub mod AOOLA {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair A output task is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair A output task is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Pair B Output Task Overload"]
    pub mod AOOLB {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair B output task is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair B output task is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Pair C Output Task Overload"]
    pub mod AOOLC {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Pair C output task is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Pair C output task is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Task Queue FIFO overload"]
    pub mod ATQOL {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Task queue FIFO logic is not overloaded"]
            pub const NO_OVERLOAD: u32 = 0;
            #[doc = "Task queue FIFO logic is overloaded"]
            pub const OVERLOAD: u32 = 0x01;
        }
    }
    #[doc = "Digital Servo Loop (DSL) Counter Input to FIFO Ready"]
    pub mod DSLCNT {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "New DSL counter information is in the process of storage into the internal ASRC FIFO"]
            pub const DSLCNT_PROC: u32 = 0;
            #[doc = "New DSL counter information is stored in the internal ASRC FIFO"]
            pub const DSLCNT_STORED: u32 = 0x01;
        }
    }
}
#[doc = "ASRC Parameter x"]
pub mod ASRPM {
    #[doc = "Parameter Value"]
    pub mod PARAMETER_VALUE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Task Queue FIFO 1"]
pub mod ASRTFR1 {
    #[doc = "Base Address for Task Queue FIFO"]
    pub mod TF_BASE {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Current Number of Entries in Task Queue FIFO"]
    pub mod TF_FILL {
        pub const offset: u32 = 13;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Channel Counter"]
pub mod ASRCCR {
    #[doc = "Channel Counter for Pair A's Input FIFO"]
    pub mod ACIA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Counter for Pair B's Input FIFO"]
    pub mod ACIB {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Counter for Pair C's Input FIFO"]
    pub mod ACIC {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Counter for Pair A's Output FIFO"]
    pub mod ACOA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Counter for Pair B's Output FIFO"]
    pub mod ACOB {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Channel Counter for Pair C's Output FIFO"]
    pub mod ACOC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Data Input for Pair x"]
pub mod ASRDIA {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Data Output for Pair x"]
pub mod ASRDOA {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Data Input for Pair x"]
pub mod ASRDIB {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Data Output for Pair x"]
pub mod ASRDOB {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Data Input for Pair x"]
pub mod ASRDIC {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Data Output for Pair x"]
pub mod ASRDOC {
    #[doc = "Data"]
    pub mod DATA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Ideal Ratio for Pair A-High Part"]
pub mod ASRIDRHA {
    #[doc = "Ideal Ratio A High"]
    pub mod IDRATIOA_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Ideal Ratio for Pair A -Low Part"]
pub mod ASRIDRLA {
    #[doc = "Ideal Ratio A Low"]
    pub mod IDRATIOA_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Ideal Ratio for Pair B-High Part"]
pub mod ASRIDRHB {
    #[doc = "Ideal Ratio B High"]
    pub mod IDRATIOB_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Ideal Ratio for Pair B-Low Part"]
pub mod ASRIDRLB {
    #[doc = "Ideal Ratio B Low"]
    pub mod IDRATIOB_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Ideal Ratio for Pair C-High Part"]
pub mod ASRIDRHC {
    #[doc = "Ideal Ratio C High"]
    pub mod IDRATIOC_H {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Ideal Ratio for Pair C-Low Part"]
pub mod ASRIDRLC {
    #[doc = "Ideal Ratio C Low"]
    pub mod IDRATIOC_L {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC 76 kHz Period"]
pub mod ASR76K {
    #[doc = "Value for the Period of the 76 kHz Sampling Clock"]
    pub mod ASR76K {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC 56 kHz Period"]
pub mod ASR56K {
    #[doc = "Value for the Period of the 56 kHz Sampling Clock"]
    pub mod ASR56K {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0001_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "ASRC Misc Control for Pair A"]
pub mod ASRMCRA {
    #[doc = "Threshold for Pair A's Input FIFO per Channel"]
    pub mod INFIFO_THRESHOLDA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    pub mod RSYNOFA {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not touch ASRCCR\\[ACOA\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACOA\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    pub mod RSYNIFA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not touch ASRCCR\\[ACIA\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACIA\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Pair A's Output FIFO per Channel"]
    pub mod OUTFIFO_THRESHOLDA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass Polyphase Filtering for Pair A"]
    pub mod BYPASSPOLYA {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't bypass polyphase filtering."]
            pub const NO_BYPASS: u32 = 0;
            #[doc = "Bypass polyphase filtering."]
            pub const BYPASS: u32 = 0x01;
        }
    }
    #[doc = "Stall Pair A Conversion in Case of Buffer Near Empty/Full Condition"]
    pub mod BUFSTALLA {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't stall Pair A conversion even in case of near empty/full FIFO conditions."]
            pub const NO_STALL: u32 = 0;
            #[doc = "Stall Pair A conversion in case of near empty/full FIFO conditions."]
            pub const STALL: u32 = 0x01;
        }
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair A"]
    pub mod EXTTHRSHA {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use default thresholds."]
            pub const USE_DEFAULT_THRESH: u32 = 0;
            #[doc = "Use external defined thresholds."]
            pub const USE_EXT_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Zero Buffer A"]
    pub mod ZEROBUFA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zeroize the buffer"]
            pub const ZERO_BUF: u32 = 0;
            #[doc = "Don't zeroize the buffer"]
            pub const DO_NOT_ZERO_BUF: u32 = 0x01;
        }
    }
}
#[doc = "ASRC FIFO Status for Pair A"]
pub mod ASRFSTA {
    #[doc = "Fillings for Pair A's Input FIFO per Channel"]
    pub mod INFIFO_FILLA {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input FIFO is Near Empty for Pair A"]
    pub mod IAEA {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input FIFO is not near empty for Pair A"]
            pub const NOT_NEAR_EMPTY: u32 = 0;
            #[doc = "Input FIFO is near empty for Pair A"]
            pub const NEAR_EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Fillings for Pair A's Output FIFO per Channel"]
    pub mod OUTFIFO_FILLA {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output FIFO is Near Full for Pair A"]
    pub mod OAFA {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output FIFO is not near full for Pair A"]
            pub const NOT_NEAR_FULL: u32 = 0;
            #[doc = "Output FIFO is near full for Pair A"]
            pub const NEAR_FULL: u32 = 0x01;
        }
    }
}
#[doc = "ASRC Misc Control for Pair B"]
pub mod ASRMCRB {
    #[doc = "Threshold for Pair B's Input FIFO per Channel"]
    pub mod INFIFO_THRESHOLDB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    pub mod RSYNOFB {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not touch ASRCCR\\[ACOB\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACOB\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    pub mod RSYNIFB {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not touch ASRCCR\\[ACIB\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACIB\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Pair B's Output FIFO per Channel"]
    pub mod OUTFIFO_THRESHOLDB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass Polyphase Filtering for Pair B"]
    pub mod BYPASSPOLYB {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't bypass polyphase filtering."]
            pub const NO_BYPASS: u32 = 0;
            #[doc = "Bypass polyphase filtering."]
            pub const BYPASS: u32 = 0x01;
        }
    }
    #[doc = "Stall Pair B Conversion in Case of Buffer Near Empty/Full Condition"]
    pub mod BUFSTALLB {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't stall Pair B conversion even in case of near empty/full FIFO conditions."]
            pub const NO_STALL: u32 = 0;
            #[doc = "Stall Pair B conversion in case of near empty/full FIFO conditions."]
            pub const STALL: u32 = 0x01;
        }
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair B"]
    pub mod EXTTHRSHB {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use default thresholds."]
            pub const USE_DEFAULT_THRESH: u32 = 0;
            #[doc = "Use external defined thresholds."]
            pub const USE_EXT_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Zero Buffer B"]
    pub mod ZEROBUFB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zeroize the buffer"]
            pub const ZERO_BUF: u32 = 0;
            #[doc = "Don't zeroize the buffer"]
            pub const DO_NOT_ZERO_BUF: u32 = 0x01;
        }
    }
}
#[doc = "ASRC FIFO Status for Pair B"]
pub mod ASRFSTB {
    #[doc = "Fillings for Pair B's Input FIFO per Channel"]
    pub mod INFIFO_FILLB {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input FIFO is Near Empty for Pair B"]
    pub mod IAEB {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input FIFO is not near empty for Pair B"]
            pub const NOT_NEAR_EMPTY: u32 = 0;
            #[doc = "Input FIFO is near empty for Pair B"]
            pub const NEAR_EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Fillings for Pair B's Output FIFO per Channel"]
    pub mod OUTFIFO_FILLB {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output FIFO is Near Full for Pair B"]
    pub mod OAFB {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output FIFO is not near full for Pair B"]
            pub const NOT_NEAR_FULL: u32 = 0;
            #[doc = "Output FIFO is near full for Pair B"]
            pub const NEAR_FULL: u32 = 0x01;
        }
    }
}
#[doc = "ASRC Misc Control for Pair C"]
pub mod ASRMCRC {
    #[doc = "Threshold for Pair C's Input FIFO per Channel"]
    pub mod INFIFO_THRESHOLDC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Re-sync Output FIFO Channel Counter"]
    pub mod RSYNOFC {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not touch ASRCCR\\[ACOC\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACOC\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Re-sync Input FIFO Channel Counter"]
    pub mod RSYNIFC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Do not touch ASRCCR\\[ACIC\\]"]
            pub const NO_RESYNC: u32 = 0;
            #[doc = "Force ASRCCR\\[ACIC\\]=0"]
            pub const RESYNC: u32 = 0x01;
        }
    }
    #[doc = "Threshold for Pair C's Output FIFO per Channel"]
    pub mod OUTFIFO_THRESHOLDC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x3f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bypass Polyphase Filtering for Pair C"]
    pub mod BYPASSPOLYC {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't bypass polyphase filtering."]
            pub const NO_BYPASS: u32 = 0;
            #[doc = "Bypass polyphase filtering."]
            pub const BYPASS: u32 = 0x01;
        }
    }
    #[doc = "Stall Pair C Conversion in Case of Buffer Near Empty/Full Condition"]
    pub mod BUFSTALLC {
        pub const offset: u32 = 21;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Don't stall Pair C conversion even in case of near empty/full FIFO conditions."]
            pub const NO_STALL: u32 = 0;
            #[doc = "Stall Pair C conversion in case of near empty/full FIFO conditions."]
            pub const STALL: u32 = 0x01;
        }
    }
    #[doc = "Use External Thresholds for FIFO Control of Pair C"]
    pub mod EXTTHRSHC {
        pub const offset: u32 = 22;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Use default thresholds."]
            pub const USE_DEFAULT_THRESH: u32 = 0;
            #[doc = "Use external defined thresholds."]
            pub const USE_EXT_THRESH: u32 = 0x01;
        }
    }
    #[doc = "Zero Buffer C"]
    pub mod ZEROBUFC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Zeroize the buffer"]
            pub const ZERO_BUF: u32 = 0;
            #[doc = "Don't zeroize the buffer"]
            pub const DO_NOT_ZERO_BUF: u32 = 0x01;
        }
    }
}
#[doc = "ASRC FIFO Status for Pair C"]
pub mod ASRFSTC {
    #[doc = "Fillings for Pair C's Input FIFO per Channel"]
    pub mod INFIFO_FILLC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Input FIFO is Near Empty for Pair C"]
    pub mod IAEC {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Input FIFO is not near empty for Pair C"]
            pub const NOT_NEAR_EMPTY: u32 = 0;
            #[doc = "Input FIFO is near empty for Pair C"]
            pub const NEAR_EMPTY: u32 = 0x01;
        }
    }
    #[doc = "Fillings for Pair C's Output FIFO per Channel"]
    pub mod OUTFIFO_FILLC {
        pub const offset: u32 = 12;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Output FIFO is Near Full for Pair C"]
    pub mod OAFC {
        pub const offset: u32 = 23;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Output FIFO is not near full for Pair C"]
            pub const NOT_NEAR_FULL: u32 = 0;
            #[doc = "Output FIFO is near full for Pair C"]
            pub const NEAR_FULL: u32 = 0x01;
        }
    }
}
#[doc = "ASRC Misc Control 1 for Pair X"]
pub mod ASRMCR1 {
    #[doc = "Bit Width Option of the Output FIFO"]
    pub mod OW16 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "24-bit output data"]
            pub const OUT_24BIT: u32 = 0;
            #[doc = "16-bit output data"]
            pub const OUT_16BIT: u32 = 0x01;
        }
    }
    #[doc = "Sign Extension Option of the Output FIFO"]
    pub mod OSGN {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No sign extension"]
            pub const NO_SIGN_EXT: u32 = 0;
            #[doc = "Sign extension"]
            pub const SIGN_EXT: u32 = 0x01;
        }
    }
    #[doc = "Data Alignment of the Output FIFO"]
    pub mod OMSB {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB aligned"]
            pub const LSB_ALIGNED: u32 = 0;
            #[doc = "MSB aligned"]
            pub const MSB_ALIGNED: u32 = 0x01;
        }
    }
    #[doc = "Data Alignment of the Input FIFO"]
    pub mod IMSB {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "LSB aligned"]
            pub const LSB_ALIGNED: u32 = 0;
            #[doc = "MSB aligned"]
            pub const MSB_ALIGNED: u32 = 0x01;
        }
    }
    #[doc = "Data Width of the Input FIFO"]
    pub mod IWD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "24-bit audio data."]
            pub const AUDIODATA_24BIT: u32 = 0;
            #[doc = "16-bit audio data."]
            pub const AUDIODATA_16BIT: u32 = 0x01;
            #[doc = "8-bit audio data."]
            pub const AUDIODATA_8BIT: u32 = 0x02;
        }
    }
}
