#[doc = "OTFAD"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c00],
    #[doc = "Control Register"]
    pub CR: crate::RWRegister<u32>,
    #[doc = "Status Register"]
    pub SR: crate::RWRegister<u32>,
    _reserved1: [u8; 0xf8],
    #[doc = "Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
    pub CTX: [ctx::RegisterBlock; 4usize],
}
#[doc = "Control Register"]
pub mod CR {
    #[doc = "Force Error"]
    pub mod FERR {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect on the SR\\[KBERE\\] indicator."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "SR\\[KBERR\\] is immediately set after a write with this data bit set."]
            pub const FORCE_ERROR: u32 = 0x01;
        }
    }
    #[doc = "Force Logically Disabled Mode"]
    pub mod FLDM {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No effect on the operating mode."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Force entry into LDM after a write with this data bit set. SR\\[MODE\\] signals the operating mode."]
            pub const FORCE_LDM: u32 = 0x01;
        }
    }
    #[doc = "Key Blob Scramble Enable"]
    pub mod KBSE {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key blob KEK scrambling is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Key blob KEK scrambling is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Key Blob Processing Enable"]
    pub mod KBPE {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key blob processing is disabled."]
            pub const DISABLE: u32 = 0;
            #[doc = "Key blob processing is enabled."]
            pub const ENABLE: u32 = 0x01;
        }
    }
    #[doc = "Restricted Register Access Enable"]
    pub mod RRAE {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
            pub const NORMAL: u32 = 0;
            #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
            pub const RESTRICT: u32 = 0x01;
        }
    }
    #[doc = "Start key blob processing"]
    pub mod SKBP {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key blob processing is not initiated."]
            pub const NO_EFFECT: u32 = 0;
            #[doc = "Properly-enabled key blob processing is initiated."]
            pub const INIT_KB: u32 = 0x01;
        }
    }
    #[doc = "Global OTFAD Enable"]
    pub mod GE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OTFAD has decryption disabled. All data fetched by the QuadSPI bypasses OTFAD processing."]
            pub const DISABLE: u32 = 0;
            #[doc = "OTFAD has decryption enabled, and processes data fetched by the QuadSPI as defined by the hardware configuration."]
            pub const ENABLE: u32 = 0x01;
        }
    }
}
#[doc = "Status Register"]
pub mod SR {
    #[doc = "Key Blob Error"]
    pub mod KBERR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob error detected."]
            pub const NO_KB_ERR: u32 = 0;
            #[doc = "One or more key blob errors has been detected."]
            pub const KB_ERR: u32 = 0x01;
        }
    }
    #[doc = "MDPC Present"]
    pub mod MDPCP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operating Mode"]
    pub mod MODE {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Operating in Normal mode (NRM)"]
            pub const NORMAL: u32 = 0;
            #[doc = "Unused (reserved)"]
            pub const RES_01: u32 = 0x01;
            #[doc = "Unused (reserved)"]
            pub const RES_10_SVM: u32 = 0x02;
            #[doc = "Operating in Logically Disabled Mode (LDM)"]
            pub const LDM: u32 = 0x03;
        }
    }
    #[doc = "Number of Contexts"]
    pub mod NCTX {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Context Error"]
    pub mod CTXER0 {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob error was detected for context \"n\"."]
            pub const NOERROR: u32 = 0;
            #[doc = "A key blob integrity error might have been detected in context \"n\"."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Context Error"]
    pub mod CTXER1 {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob error was detected for context \"n\"."]
            pub const NOERROR: u32 = 0;
            #[doc = "A key blob integrity error might have been detected in context \"n\"."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Context Error"]
    pub mod CTXER2 {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob error was detected for context \"n\"."]
            pub const NOERROR: u32 = 0;
            #[doc = "A key blob integrity error might have been detected in context \"n\"."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Context Error"]
    pub mod CTXER3 {
        pub const offset: u32 = 11;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob error was detected for context \"n\"."]
            pub const NOERROR: u32 = 0;
            #[doc = "A key blob integrity error might have been detected in context \"n\"."]
            pub const ERROR: u32 = 0x01;
        }
    }
    #[doc = "Context Integrity Error"]
    pub mod CTXIE0 {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob integrity error was detected for context \"n\"."]
            pub const NOINTEGRITYERR: u32 = 0;
            #[doc = "A key blob integrity error was detected in context \"n\"."]
            pub const INTEGRITYERR: u32 = 0x01;
        }
    }
    #[doc = "Context Integrity Error"]
    pub mod CTXIE1 {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob integrity error was detected for context \"n\"."]
            pub const NOINTEGRITYERR: u32 = 0;
            #[doc = "A key blob integrity error was detected in context \"n\"."]
            pub const INTEGRITYERR: u32 = 0x01;
        }
    }
    #[doc = "Context Integrity Error"]
    pub mod CTXIE2 {
        pub const offset: u32 = 18;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob integrity error was detected for context \"n\"."]
            pub const NOINTEGRITYERR: u32 = 0;
            #[doc = "A key blob integrity error was detected in context \"n\"."]
            pub const INTEGRITYERR: u32 = 0x01;
        }
    }
    #[doc = "Context Integrity Error"]
    pub mod CTXIE3 {
        pub const offset: u32 = 19;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "No key blob integrity error was detected for context \"n\"."]
            pub const NOINTEGRITYERR: u32 = 0;
            #[doc = "A key blob integrity error was detected in context \"n\"."]
            pub const INTEGRITYERR: u32 = 0x01;
        }
    }
    #[doc = "Hardware Revision Level"]
    pub mod HRL {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Restricted Register Access Mode"]
    pub mod RRAM {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Register access is fully enabled. The OTFAD programming model registers can be accessed \"normally\"."]
            pub const NORMAL: u32 = 0;
            #[doc = "Register access is restricted and only the CR, SR and optional MDPC registers can be accessed; others are treated as RAZ/WI."]
            pub const RESTRICTED: u32 = 0x01;
        }
    }
    #[doc = "Global Enable Mode"]
    pub mod GEM {
        pub const offset: u32 = 29;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "OTFAD is disabled. All data fetched by the QuadSPI bypasses OTFAD processing."]
            pub const DISABLED: u32 = 0;
            #[doc = "OTFAD is enabled, and processes data fetched by the QuadSPI as defined by the hardware configuration."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Key Blob Processing Enable"]
    pub mod KBPE {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key blob processing is not enabled."]
            pub const DISABLED: u32 = 0;
            #[doc = "Key blob processing is enabled."]
            pub const ENABLED: u32 = 0x01;
        }
    }
    #[doc = "Key Blob Processing Done"]
    pub mod KBD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Key blob processing was not enabled, or is not complete."]
            pub const NOT_DONE: u32 = 0;
            #[doc = "Key blob processing was enabled and is complete."]
            pub const DONE: u32 = 0x01;
        }
    }
}
pub mod ctx {
    #[doc = "Array of registers: CTX_KEY%s, CTX_CTR%s, CTX_RGD_W0, CTX_RGD_W1"]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "AES Key Word"]
        pub CTX_KEY: [crate::RWRegister<u32>; 4usize],
        #[doc = "AES Counter Word"]
        pub CTX_CTR: [crate::RWRegister<u32>; 2usize],
        #[doc = "AES Region Descriptor Word0"]
        pub CTX_RGD_W0: crate::RWRegister<u32>,
        #[doc = "AES Region Descriptor Word1"]
        pub CTX_RGD_W1: crate::RWRegister<u32>,
        _reserved0: [u8; 0x20],
    }
    #[doc = "AES Key Word"]
    pub mod CTX_KEY {
        #[doc = "AES Key"]
        pub mod KEY {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "AES Counter Word"]
    pub mod CTX_CTR {
        #[doc = "AES Counter"]
        pub mod CTR {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0xffff_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "AES Region Descriptor Word0"]
    pub mod CTX_RGD_W0 {
        #[doc = "Start Address"]
        pub mod SRTADDR {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0x003f_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "AES Region Descriptor Word1"]
    pub mod CTX_RGD_W1 {
        #[doc = "Valid"]
        pub mod VLD {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Context is invalid."]
                pub const NOT_VALID: u32 = 0;
                #[doc = "Context is valid."]
                pub const VALID: u32 = 0x01;
            }
        }
        #[doc = "AES Decryption Enable."]
        pub mod ADE {
            pub const offset: u32 = 1;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "Bypass the fetched data."]
                pub const BYPASS: u32 = 0;
                #[doc = "Perform the CTR-AES128 mode decryption on the fetched data."]
                pub const DECRYPT: u32 = 0x01;
            }
        }
        #[doc = "Read-Only"]
        pub mod RO {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x01 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {
                #[doc = "The context registers can be accessed normally (as defined by SR\\[RRAM\\])."]
                pub const NORMAL: u32 = 0;
                #[doc = "The context registers are read-only and accesses may be further restricted based on SR\\[RRAM\\]."]
                pub const RESTRICT: u32 = 0x01;
            }
        }
        #[doc = "End Address"]
        pub mod ENDADDR {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0x003f_ffff << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}
