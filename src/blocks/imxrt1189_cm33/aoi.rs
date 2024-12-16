#[doc = "AOI"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0"]
    pub BFCRT010: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0"]
    pub BFCRT230: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT1"]
    pub BFCRT011: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT1"]
    pub BFCRT231: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT2"]
    pub BFCRT012: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT2"]
    pub BFCRT232: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 0 and 1 Configuration for EVENT3"]
    pub BFCRT013: crate::RWRegister<u16>,
    #[doc = "Boolean Function Term 2 and 3 Configuration for EVENT3"]
    pub BFCRT233: crate::RWRegister<u16>,
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT0"]
pub mod BFCRT010 {
    #[doc = "Product Term 1, Input D Configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input C Configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input B Configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input A Configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input D Configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input C Configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input B Configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input A Configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT0"]
pub mod BFCRT230 {
    #[doc = "Product Term 3, Input D Configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input C Configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input B Configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input A Configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input D Configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input C Configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input B Configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input A Configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT1"]
pub mod BFCRT011 {
    #[doc = "Product Term 1, Input D Configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input C Configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input B Configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input A Configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input D Configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input C Configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input B Configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input A Configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT1"]
pub mod BFCRT231 {
    #[doc = "Product Term 3, Input D Configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input C Configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input B Configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input A Configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input D Configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input C Configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input B Configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input A Configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT2"]
pub mod BFCRT012 {
    #[doc = "Product Term 1, Input D Configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input C Configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input B Configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input A Configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input D Configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input C Configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input B Configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input A Configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT2"]
pub mod BFCRT232 {
    #[doc = "Product Term 3, Input D Configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input C Configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input B Configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input A Configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input D Configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input C Configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input B Configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input A Configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 0 and 1 Configuration for EVENT3"]
pub mod BFCRT013 {
    #[doc = "Product Term 1, Input D Configuration"]
    pub mod PT1_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input C Configuration"]
    pub mod PT1_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input B Configuration"]
    pub mod PT1_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 1, Input A Configuration"]
    pub mod PT1_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input D Configuration"]
    pub mod PT0_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input C Configuration"]
    pub mod PT0_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input B Configuration"]
    pub mod PT0_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 0, Input A Configuration"]
    pub mod PT0_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
#[doc = "Boolean Function Term 2 and 3 Configuration for EVENT3"]
pub mod BFCRT233 {
    #[doc = "Product Term 3, Input D Configuration"]
    pub mod PT3_DC {
        pub const offset: u16 = 0;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input C Configuration"]
    pub mod PT3_CC {
        pub const offset: u16 = 2;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input B Configuration"]
    pub mod PT3_BC {
        pub const offset: u16 = 4;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 3, Input A Configuration"]
    pub mod PT3_AC {
        pub const offset: u16 = 6;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input D Configuration"]
    pub mod PT2_DC {
        pub const offset: u16 = 8;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input D to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input D"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input D"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input D to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input C Configuration"]
    pub mod PT2_CC {
        pub const offset: u16 = 10;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input C to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input C"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input C"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input C to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input B Configuration"]
    pub mod PT2_BC {
        pub const offset: u16 = 12;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input B to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input B"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input B"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input B to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
    #[doc = "Product Term 2, Input A Configuration"]
    pub mod PT2_AC {
        pub const offset: u16 = 14;
        pub const mask: u16 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Force input A to become 0"]
            pub const FORCE_0: u16 = 0;
            #[doc = "Pass input A"]
            pub const PASS: u16 = 0x01;
            #[doc = "Complement input A"]
            pub const COMPLEMENT: u16 = 0x02;
            #[doc = "Force input A to become 1"]
            pub const FORCE_1: u16 = 0x03;
        }
    }
}
