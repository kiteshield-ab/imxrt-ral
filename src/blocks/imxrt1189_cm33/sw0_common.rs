#[doc = "Switch and ENETC common base"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1000],
    #[doc = "Ingress port capability register"]
    pub IPCAPR: crate::RORegister<u32>,
    #[doc = "Egress port capability register"]
    pub EPCAPR: crate::RORegister<u32>,
    _reserved1: [u8; 0x08],
    #[doc = "Operational state register"]
    pub OSR: crate::RORegister<u32>,
    _reserved2: [u8; 0x2c],
    #[doc = "Correctable memory error configuration register"]
    pub CMECR: crate::RWRegister<u32>,
    #[doc = "Correctable memory error status register"]
    pub CMESR: crate::RWRegister<u32>,
    _reserved3: [u8; 0x04],
    #[doc = "Correctable memory error count register"]
    pub CMECTR: crate::RORegister<u32>,
    _reserved4: [u8; 0x10],
    #[doc = "Uncorrectable non-fatal MAC error configuration register"]
    pub UNMACECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable non-fatal MAC error status register"]
    pub UNMACESR: crate::RORegister<u32>,
    _reserved5: [u8; 0x08],
    #[doc = "Uncorrectable non-fatal system bus error configuration register"]
    pub UNSBECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable non-fatal system bus error status register"]
    pub UNSBESR: crate::RWRegister<u32>,
    _reserved6: [u8; 0x04],
    #[doc = "Uncorrectable non-fatal system bus error count register"]
    pub UNSBECTR: crate::RORegister<u32>,
    #[doc = "Uncorrectable fatal system bus error configuration register"]
    pub UFSBECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable fatal system bus error status register"]
    pub UFSBESR: crate::RWRegister<u32>,
    _reserved7: [u8; 0x08],
    #[doc = "Uncorrectable non-fatal memory error configuration register"]
    pub UNMECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable non-fatal memory error status register 0"]
    pub UNMESR0: crate::RWRegister<u32>,
    #[doc = "Uncorrectable non-fatal memory error status register 1"]
    pub UNMESR1: crate::RORegister<u32>,
    #[doc = "Uncorrectable non-fatal memory error count register"]
    pub UNMECTR: crate::RORegister<u32>,
    #[doc = "Uncorrectable fatal memory error configuration register"]
    pub UFMECR: crate::RWRegister<u32>,
    #[doc = "Uncorrectable fatal memory error status register 0"]
    pub UFMESR0: crate::RWRegister<u32>,
    #[doc = "Uncorrectable fatal memory error status register 1"]
    pub UFMESR1: crate::RORegister<u32>,
    _reserved8: [u8; 0x34],
    #[doc = "Internal MDIO interrupt reason register"]
    pub IMDIOIRR: crate::RORegister<u32>,
    #[doc = "Internal MDIO MSI-X vector register"]
    pub IMDIOMSIVR: crate::RWRegister<u32>,
    #[doc = "External MDIO interrupt reason register"]
    pub EMDIOIRR: crate::RORegister<u32>,
    #[doc = "External MDIO MSI-X vector register"]
    pub EMDIOMSIVR: crate::RWRegister<u32>,
    _reserved9: [u8; 0x10],
    #[doc = "Time capture configuration register"]
    pub TCCR: crate::RWRegister<u32>,
    #[doc = "Time capture interrupt enable register"]
    pub TCIER: crate::RWRegister<u32>,
    #[doc = "Time capture receive port interrupt detect register"]
    pub TCRPIDR: crate::RWRegister<u32>,
    #[doc = "Time capture receive port status register"]
    pub TCRPSR: crate::RORegister<u32>,
    _reserved10: [u8; 0x04],
    #[doc = "Time capture receive port timestamp register"]
    pub TCRPTSR: crate::RORegister<u32>,
    #[doc = "Time capture MSI-X vector register"]
    pub TCMSIVR: crate::RWRegister<u32>,
    _reserved11: [u8; 0xe4],
    #[doc = "Custom VLAN Ethertype register 1"]
    pub CVLANR1: crate::RWRegister<u32>,
    #[doc = "Custom VLAN Ethertype register 2"]
    pub CVLANR2: crate::RWRegister<u32>,
    #[doc = "Pre-Standard RTAG Ethertype register"]
    pub PSRTAGETR: crate::RWRegister<u32>,
    _reserved12: [u8; 0x14],
    #[doc = "DoS L2 configuration register"]
    pub DOSL2CR: crate::RWRegister<u32>,
    _reserved13: [u8; 0xdc],
    #[doc = "VLAN to IPV mapping profile register set."]
    pub NUM_PROFILE: [numprofile::RegisterBlock; 2usize],
    _reserved14: [u8; 0x0320],
    #[doc = "Ingress port filter capability register"]
    pub IPFCAPR: crate::RORegister<u32>,
    #[doc = "Ingress port filter table capability register"]
    pub IPFTCAPR: crate::RORegister<u32>,
    #[doc = "Ingress port filter table memory operational register"]
    pub IPFTMOR: crate::RORegister<u32>,
    _reserved15: [u8; 0x01b4],
    #[doc = "Index table memory capability register"]
    pub ITMCAPR: crate::RORegister<u32>,
    _reserved16: [u8; 0x0c],
    #[doc = "Rate policer capability register"]
    pub RPCAPR: crate::RORegister<u32>,
    #[doc = "Rate policer index table capability register"]
    pub RPITCAPR: crate::RORegister<u32>,
    #[doc = "Rate policer index table memory allocation register"]
    pub RPITMAR: crate::RWRegister<u32>,
    #[doc = "Rate policer index table operational register"]
    pub RPITOR: crate::RORegister<u32>,
    _reserved17: [u8; 0x04],
    #[doc = "Ingress stream counter index table capability register"]
    pub ISCITCAPR: crate::RORegister<u32>,
    #[doc = "Ingress stream counter index table memory allocation register"]
    pub ISCITMAR: crate::RWRegister<u32>,
    #[doc = "Ingress stream counter index table operational register"]
    pub ISCITOR: crate::RORegister<u32>,
    #[doc = "Ingress stream capability register"]
    pub ISCAPR: crate::RORegister<u32>,
    #[doc = "Ingress stream index table capability register"]
    pub ISITCAPR: crate::RORegister<u32>,
    #[doc = "Ingress stream index table memory allocation register"]
    pub ISITMAR: crate::RWRegister<u32>,
    #[doc = "Ingress stream index table operational register"]
    pub ISITOR: crate::RORegister<u32>,
    _reserved18: [u8; 0x04],
    #[doc = "Ingress sequence generation index table capability register"]
    pub ISQGITCAPR: crate::RORegister<u32>,
    #[doc = "Ingress sequence generation index table memory allocation register"]
    pub ISQGITMAR: crate::RWRegister<u32>,
    #[doc = "Ingress sequence generation index table operational register"]
    pub ISQGITOR: crate::RORegister<u32>,
    _reserved19: [u8; 0x10],
    #[doc = "Stream gate capability register"]
    pub SGCAPR: crate::RORegister<u32>,
    #[doc = "Stream gate instance index table capability register"]
    pub SGIITCAPR: crate::RORegister<u32>,
    #[doc = "Stream gate instance index table memory allocation register"]
    pub SGIITMAR: crate::RWRegister<u32>,
    #[doc = "Stream gate instance index table operational register"]
    pub SGIITOR: crate::RORegister<u32>,
    _reserved20: [u8; 0x04],
    #[doc = "Stream gate control list index table capability register"]
    pub SGCLITCAPR: crate::RORegister<u32>,
    #[doc = "Stream gate control list index table memory allocation register"]
    pub SGCLITMAR: crate::RWRegister<u32>,
    #[doc = "Stream gate control list table memory operational register"]
    pub SGCLTMOR: crate::RORegister<u32>,
    #[doc = "Frame modification ingress capability register"]
    pub FMICAPR: crate::RORegister<u32>,
    #[doc = "Frame modification egress capability register"]
    pub FMECAPR: crate::RORegister<u32>,
    #[doc = "Frame modification index table capability register"]
    pub FMITCAPR: crate::RORegister<u32>,
    #[doc = "Frame modification index table memory allocation register"]
    pub FMITMAR: crate::RWRegister<u32>,
    #[doc = "Frame modification index table operational register"]
    pub FMITOR: crate::RORegister<u32>,
    #[doc = "Frame modification data index table capability register"]
    pub FMDITCAPR: crate::RORegister<u32>,
    #[doc = "Frame modification data index table memory allocation register"]
    pub FMDITMAR: crate::RWRegister<u32>,
    _reserved21: [u8; 0x24],
    #[doc = "Egress treatment capability register"]
    pub ETCAPR: crate::RORegister<u32>,
    #[doc = "Egress treatment table capability register"]
    pub ETTCAPR: crate::RORegister<u32>,
    _reserved22: [u8; 0x04],
    #[doc = "Egress treatment table operational register"]
    pub ETTOR: crate::RORegister<u32>,
    _reserved23: [u8; 0x04],
    #[doc = "Time gate scheduling table capability register"]
    pub TGSTCAPR: crate::RORegister<u32>,
    _reserved24: [u8; 0x04],
    #[doc = "Time gate scheduling table memory operation register"]
    pub TGSTMOR: crate::RORegister<u32>,
    #[doc = "Egress sequence recovery capability register"]
    pub ESQRCAPR: crate::RORegister<u32>,
    #[doc = "Egress sequence recovery table capability register"]
    pub ESQRTCAPR: crate::RORegister<u32>,
    _reserved25: [u8; 0x04],
    #[doc = "Egress counter table capability register"]
    pub ECTCAPR: crate::RORegister<u32>,
    _reserved26: [u8; 0x10],
    #[doc = "Hash table memory capability register"]
    pub HTMCAPR: crate::RORegister<u32>,
    #[doc = "Hash table memory operational register"]
    pub HTMOR: crate::RORegister<u32>,
    _reserved27: [u8; 0x08],
    #[doc = "Ingress stream identification capability register"]
    pub ISIDCAPR: crate::RORegister<u32>,
    #[doc = "Ingress stream identification hash table capability register"]
    pub ISIDHTCAPR: crate::RORegister<u32>,
    _reserved28: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 0 operational register"]
    pub ISIDKC0OR: crate::RORegister<u32>,
    #[doc = "Ingress stream identification key construction 0 configuration register 0"]
    pub ISIDKC0CR0: crate::RWRegister<u32>,
    _reserved29: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 0 payload field 0 configuration register"]
    pub ISIDKC0PF0CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 0 payload field 1 configuration register"]
    pub ISIDKC0PF1CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 0 payload field 2 configuration register"]
    pub ISIDKC0PF2CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 0 payload field 3 configuration register"]
    pub ISIDKC0PF3CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 1 operational register"]
    pub ISIDKC1OR: crate::RORegister<u32>,
    #[doc = "Ingress stream identification key construction 1 configuration register 0"]
    pub ISIDKC1CR0: crate::RWRegister<u32>,
    _reserved30: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 1 payload field 0 configuration register"]
    pub ISIDKC1PF0CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 1 payload field 1 configuration register"]
    pub ISIDKC1PF1CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 1 payload field 2 configuration register"]
    pub ISIDKC1PF2CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 1 payload field 3 configuration register"]
    pub ISIDKC1PF3CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 2 operational register"]
    pub ISIDKC2OR: crate::RORegister<u32>,
    #[doc = "Ingress stream identification key construction 2 configuration register 0"]
    pub ISIDKC2CR0: crate::RWRegister<u32>,
    _reserved31: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 2 payload field 0 configuration register"]
    pub ISIDKC2PF0CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 2 payload field 1 configuration register"]
    pub ISIDKC2PF1CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 2 payload field 2 configuration register"]
    pub ISIDKC2PF2CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 2 payload field 3 configuration register"]
    pub ISIDKC2PF3CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 3 operational register"]
    pub ISIDKC3OR: crate::RORegister<u32>,
    #[doc = "Ingress stream identification key construction 3 configuration register 0"]
    pub ISIDKC3CR0: crate::RWRegister<u32>,
    _reserved32: [u8; 0x08],
    #[doc = "Ingress stream identification key construction 3 payload field 0 configuration register"]
    pub ISIDKC3PF0CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 3 payload field 1 configuration register"]
    pub ISIDKC3PF1CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 3 payload field 2 configuration register"]
    pub ISIDKC3PF2CR: crate::RWRegister<u32>,
    #[doc = "Ingress stream identification key construction 3 payload field 3 configuration register"]
    pub ISIDKC3PF3CR: crate::RWRegister<u32>,
    _reserved33: [u8; 0x60],
    #[doc = "Ingress stream filter hash table capability register"]
    pub ISFHTCAPR: crate::RORegister<u32>,
    #[doc = "Ingress stream filter hash table operational register"]
    pub ISFHTOR: crate::RORegister<u32>,
}
#[doc = "Ingress port capability register"]
pub mod IPCAPR {
    #[doc = "Rate Policer function supported. 0: Not supported 1: Supported See RPCAPR for more information."]
    pub mod RP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Port Filtering supported (that is,: Ingress Port Filter table lookup)"]
    pub mod IPFLT {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Stream Identification functionality supported"]
    pub mod ISID {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    pub mod SDU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the number a of receive VLAN PCP/DE to QoS mapping profiles supported; see registers VLANIPVMPaR0/1 and VLANDRMPaR, where a={0"]
    pub mod NUM_VQMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress port capability register"]
pub mod EPCAPR {
    #[doc = "Egress Treatment function supported. 0: Not supported 1: Supported See ETCAPR for more information."]
    pub mod ET {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    pub mod SDU {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Specifies the number of transmit QoS to VLAN PCP mapping profiles supported; see register QOSVLANMPaR0/1/2/3 where a={0"]
    pub mod NUM_QVMP {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Operational state register"]
pub mod OSR {
    #[doc = "Indicates the function's operational state 0: Function is operationally ready"]
    pub mod STATE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates the index table memory (common memory) operational state"]
    pub mod ITM_STATE {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Correctable memory error configuration register"]
pub mod CMECR {
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Correctable memory error status register"]
pub mod CMESR {
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Single-bit ECC error"]
    pub mod SBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Correctable memory error count register"]
pub mod CMECTR {
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal MAC error configuration register"]
pub mod UNMACECR {
    #[doc = "Report disable port"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Report disable port"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Report disable port"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Report disable port"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Report disable port"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal MAC error status register"]
pub mod UNMACESR {
    #[doc = "Port 0 MAC error"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port 1 MAC error"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port 2 MAC error"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port 3 MAC error"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Port 4 MAC error"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal system bus error configuration register"]
pub mod UNSBECR {
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal system bus error status register"]
pub mod UNSBESR {
    #[doc = "System Bus ID"]
    pub mod SB_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System bus error"]
    pub mod SBE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal system bus error count register"]
pub mod UNSBECTR {
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal system bus error configuration register"]
pub mod UFSBECR {
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Enabled"]
            pub const ENABLE: u32 = 0;
            #[doc = "Disabled"]
            pub const DISABLE: u32 = 0x01;
        }
    }
}
#[doc = "Uncorrectable fatal system bus error status register"]
pub mod UFSBESR {
    #[doc = "System Bus ID"]
    pub mod SB_ID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple"]
    pub mod M {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "System bus error"]
    pub mod SBE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error configuration register"]
pub mod UNMECR {
    #[doc = "Threshold"]
    pub mod THRESHOLD {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 0"]
pub mod UNMESR0 {
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 1"]
pub mod UNMESR1 {
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable non-fatal memory error count register"]
pub mod UNMECTR {
    #[doc = "Count"]
    pub mod COUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal memory error configuration register"]
pub mod UFMECR {
    #[doc = "Report disable"]
    pub mod RD {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal memory error status register 0"]
pub mod UFMESR0 {
    #[doc = "Syndrome"]
    pub mod SYNDROME {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x07ff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Memory ID"]
    pub mod MEM_ID {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multiple"]
    pub mod M {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Multi-bit ECC error"]
    pub mod MBEE {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Uncorrectable fatal memory error status register 1"]
pub mod UFMESR1 {
    #[doc = "Address"]
    pub mod ADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Internal MDIO interrupt reason register"]
pub mod IMDIOIRR {
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Internal MDIO MSI-X vector register"]
pub mod IMDIOMSIVR {
    #[doc = "Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External MDIO interrupt reason register"]
pub mod EMDIOIRR {
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    pub mod PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "External MDIO MSI-X vector register"]
pub mod EMDIOMSIVR {
    #[doc = "Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time capture configuration register"]
pub mod TCCR {
    #[doc = "Indicates the duration time in nanoseconds the timestamp capture function is armed"]
    pub mod TIMEOUT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x00ff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if Timestamp Capture function is armed"]
    pub mod ARM {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time capture interrupt enable register"]
pub mod TCIER {
    #[doc = "Transmit interrupt"]
    pub mod TRANSMIT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout interrupt"]
    pub mod TIMEOUT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time capture receive port interrupt detect register"]
pub mod TCRPIDR {
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT0 {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT1 {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT2 {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT3 {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    pub mod TX_PORT4 {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Transmit interrupt"]
    pub mod TRANSMIT {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Timeout interrupt"]
    pub mod TIMEOUT {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time capture receive port status register"]
pub mod TCRPSR {
    #[doc = "Receive port which captured the receive timestamp stored in TCRPTSR"]
    pub mod RX_PORT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Count the number of times the time capture function was triggered since it was ARMed"]
    pub mod RX_CNT {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time capture receive port timestamp register"]
pub mod TCRPTSR {
    #[doc = "Timestamp value in ns relative to SFD of the received frame"]
    pub mod TIMESTAMP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff_ffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time capture MSI-X vector register"]
pub mod TCMSIVR {
    #[doc = "Index into MSI-X address/data table"]
    pub mod VECTOR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Custom VLAN Ethertype register 1"]
pub mod CVLANR1 {
    #[doc = "Ethertype"]
    pub mod ETYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Custom VLAN Ethertype register 2"]
pub mod CVLANR2 {
    #[doc = "Ethertype"]
    pub mod ETYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Valid"]
    pub mod V {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Pre-Standard RTAG Ethertype register"]
pub mod PSRTAGETR {
    #[doc = "802"]
    pub mod ETHERTYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "DoS L2 configuration register"]
pub mod DOSL2CR {
    #[doc = "This field specifies whether received frames with SMAC = DMAC are discarded"]
    pub mod SAMEADDR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "This field specifies whether received frames with Multicast SMAC address are discarded"]
    pub mod MSAMCC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress port filter capability register"]
pub mod IPFCAPR {
    #[doc = "Rate Policer function supported"]
    pub mod RP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ingress Stream Identification supported."]
    pub mod ISID {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress port filter table capability register"]
pub mod IPFTCAPR {
    #[doc = "Number of ternary memory words supported"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates if table entries are managed by software driver or by hardware"]
    pub mod MGMT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which Configuration Access Methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum number of consecutive words which can form a TM Entry"]
    pub mod ENTRY_MAX_WORDS {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word size in bits of the ternary memory. 0: 48 bits 1-3: Reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress port filter table memory operational register"]
pub mod IPFTMOR {
    #[doc = "Number of words in-use."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Index table memory capability register"]
pub mod ITMCAPR {
    #[doc = "Number of Words in the Index table memory"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rate policer capability register"]
pub mod RPCAPR {
    #[doc = "Two-Rate Three-Color Marker supported per MEF 10.3 standard."]
    pub mod TRTCM {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Color Mode capability 0: Support Color Blind mode only 1: Support Color Blind and Color Aware modes"]
    pub mod CM {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rate policer index table capability register"]
pub mod RPITCAPR {
    #[doc = "The number of entries assigned to this table. Reset value is specified by ROUNDDOWN(RPITMAR/4)."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rate policer index table memory allocation register"]
pub mod RPITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Rate policer index table operational register"]
pub mod RPITOR {
    #[doc = "The number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x3fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream counter index table capability register"]
pub mod ISCITCAPR {
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISCITMAR."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream counter index table memory allocation register"]
pub mod ISCITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream counter index table operational register"]
pub mod ISCITOR {
    #[doc = "The number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream capability register"]
pub mod ISCAPR {
    #[doc = "Ingress Sequence Generation specification supported"]
    pub mod ISQG {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Stream Gating specification is supported"]
    pub mod SG {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Rate Policer function specification supported"]
    pub mod RP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum SDU check supported"]
    pub mod MAXSDU {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "When set, can specify a set of destination to forward the frame."]
    pub mod FWD {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Egress Treatment table entries specification supported. 0: Not supported 1: Supported"]
    pub mod ET {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream index table capability register"]
pub mod ISITCAPR {
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISITMAR."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream index table memory allocation register"]
pub mod ISITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream index table operational register"]
pub mod ISITOR {
    #[doc = "The number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress sequence generation index table capability register"]
pub mod ISQGITCAPR {
    #[doc = "The number of entries assigned to this table"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress sequence generation index table memory allocation register"]
pub mod ISQGITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress sequence generation index table operational register"]
pub mod ISQGITOR {
    #[doc = "The number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate capability register"]
pub mod SGCAPR {
    #[doc = "Support Administrative and Operational Gate Control List. 0: Not supported 1: Supported"]
    pub mod GLC_AO {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Support configurable option indicating if GCL's Gate Check is from SFD only or SFD until EOF"]
    pub mod GLC_GC {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each Gate Control List Entry supports Interval Max Octet check. 0: Not supported 1: Supported"]
    pub mod GLC_IO {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each Gate Control List Entry supports configurable IPV. 0: Not supported 1: Supported"]
    pub mod GLC_IPV {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Each Gate Control List Entry supports configurable CTD (Cut-Through Disable state)"]
    pub mod GLC_CTD {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate instance index table capability register"]
pub mod SGIITCAPR {
    #[doc = "The number of entries assigned to this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate instance index table memory allocation register"]
pub mod SGIITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate instance index table operational register"]
pub mod SGIITOR {
    #[doc = "The number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate control list index table capability register"]
pub mod SGCLITCAPR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate control list index table memory allocation register"]
pub mod SGCLITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Stream gate control list table memory operational register"]
pub mod SGCLTMOR {
    #[doc = "Number of words in-use."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification ingress capability register"]
pub mod FMICAPR {
    #[doc = "Layer 2 frame modification actions supported"]
    pub mod L2_ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification egress capability register"]
pub mod FMECAPR {
    #[doc = "Layer 2 frame modification actions"]
    pub mod L2_ACT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Layer 3 frame modification actions"]
    pub mod L3_ACT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification index table capability register"]
pub mod FMITCAPR {
    #[doc = "The number of entries assigned to this table. The reset value is taken from FMITMAR\\[NUM_WORDS\\]."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification index table memory allocation register"]
pub mod FMITMAR {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification index table operational register"]
pub mod FMITOR {
    #[doc = "The number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x1fff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification data index table capability register"]
pub mod FMDITCAPR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Frame modification data index table memory allocation register"]
pub mod FMDITMAR {
    #[doc = "The number of words from index table memory assigned to this table"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress treatment capability register"]
pub mod ETCAPR {
    #[doc = "Egress Sequence Recovery supported"]
    pub mod ESQR {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress treatment table capability register"]
pub mod ETTCAPR {
    #[doc = "The number of entries assigned to this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress treatment table operational register"]
pub mod ETTOR {
    #[doc = "Number of entries allocated / in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Time gate scheduling table capability register"]
pub mod TGSTCAPR {
    #[doc = "Number of Words"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Access Method"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Index"]
            pub const INDEX: u32 = 0x01;
        }
    }
    #[doc = "Maximum Gate Control List Length"]
    pub mod MAX_GCL_LEN {
        pub const offset: u32 = 24;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "64"]
            pub const LEN_64: u32 = 0;
            #[doc = "128"]
            pub const LEN_128: u32 = 0x01;
            #[doc = "256"]
            pub const LEN_256: u32 = 0x02;
        }
    }
}
#[doc = "Time gate scheduling table memory operation register"]
pub mod TGSTMOR {
    #[doc = "The number of words in-use."]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress sequence recovery capability register"]
pub mod ESQRCAPR {
    #[doc = "Support Individual Recovery function and/or Sequence Recovery function x1: Individual Recovery function supported 1x: Sequence Recovery function supported"]
    pub mod SQR_TYPE {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Sequence Recovery Supported Algorithm. x1: Match Sequence Algorithm 1x: Vector Sequence Algorithm"]
    pub mod SQR_ALG {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum history length capability used by sequence recovery function. 0: 64 1: 128 2-7: Reserved"]
    pub mod SQR_MAX_HL {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress sequence recovery table capability register"]
pub mod ESQRTCAPR {
    #[doc = "The number of entries assigned to this table"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Egress counter table capability register"]
pub mod ECTCAPR {
    #[doc = "The number of entries assigned to this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash table memory capability register"]
pub mod HTMCAPR {
    #[doc = "Maximum number of words allotted to exact match hash table from the common memory's shared region"]
    pub mod NUM_WORDS {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    pub mod WORD_SIZE {
        pub const offset: u32 = 28;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
    pub mod MLOC {
        pub const offset: u32 = 30;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Hash table memory operational register"]
pub mod HTMOR {
    #[doc = "Number of Words in use by this function which has been allocated by the various hash tables."]
    pub mod AMOUNT {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "High WaterMark of Words allocated. Value reset to AMOUNT when read."]
    pub mod WATERMARK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification capability register"]
pub mod ISIDCAPR {
    #[doc = "Number of Exact Match Key Construction Instances supported for Ingress Stream Identification"]
    pub mod NUM_KC {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x03 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Number of configurable Payload Fields supported"]
    pub mod NUM_PF {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Maximum Key Size in bytes which can be constructed using the frame's fields."]
    pub mod MAX_KSIZE {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x1f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Unknown Frame Type (no header field parsing of the frame is necessary to construct the key) supported"]
    pub mod UFT {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Ethernet Frame Type (frame begins with standard 802"]
    pub mod ETHFT {
        pub const offset: u32 = 17;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification hash table capability register"]
pub mod ISIDHTCAPR {
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 0 operational register"]
pub mod ISIDKC0OR {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 0 configuration register 0"]
pub mod ISIDKC0CR0 {
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 0 configuration register"]
pub mod ISIDKC0PF0CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 1 configuration register"]
pub mod ISIDKC0PF1CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 2 configuration register"]
pub mod ISIDKC0PF2CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 3 configuration register"]
pub mod ISIDKC0PF3CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 1 operational register"]
pub mod ISIDKC1OR {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 1 configuration register 0"]
pub mod ISIDKC1CR0 {
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 0 configuration register"]
pub mod ISIDKC1PF0CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 1 configuration register"]
pub mod ISIDKC1PF1CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 2 configuration register"]
pub mod ISIDKC1PF2CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 3 configuration register"]
pub mod ISIDKC1PF3CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 2 operational register"]
pub mod ISIDKC2OR {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 2 configuration register 0"]
pub mod ISIDKC2CR0 {
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 0 configuration register"]
pub mod ISIDKC2PF0CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 1 configuration register"]
pub mod ISIDKC2PF1CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 2 configuration register"]
pub mod ISIDKC2PF2CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 3 configuration register"]
pub mod ISIDKC2PF3CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 3 operational register"]
pub mod ISIDKC3OR {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    pub mod EN {
        pub const offset: u32 = 31;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 3 configuration register 0"]
pub mod ISIDKC3CR0 {
    #[doc = "Valid"]
    pub mod VALID {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "The entire key construction rule is not valid including any configuration payload key fields defined."]
            pub const NOT_VALID: u32 = 0;
            #[doc = "The key construction rule is valid."]
            pub const VALID: u32 = 0x01;
        }
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    pub mod PORTP {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    pub mod SPMP {
        pub const offset: u32 = 2;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Destination MAC (address) Present"]
    pub mod DMACP {
        pub const offset: u32 = 3;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_INCLUDED: u32 = 0;
            #[doc = "Present"]
            pub const INCLUDED: u32 = 0x01;
        }
    }
    #[doc = "Source MAC (address) Present."]
    pub mod SMACP {
        pub const offset: u32 = 4;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer VID Present"]
    pub mod OVIDP {
        pub const offset: u32 = 5;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Outer PCP Present"]
    pub mod OPCPP {
        pub const offset: u32 = 6;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Outer VLAN header's PCP field not present in the key"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Outer VLAN header's PCP field present in the key"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner VID Present."]
    pub mod IVIDP {
        pub const offset: u32 = 7;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Inner PCP Present."]
    pub mod IPCPP {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "Sequence Tag (code point) Present."]
    pub mod SQTP {
        pub const offset: u32 = 9;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
    #[doc = "EtherType Present."]
    pub mod ETP {
        pub const offset: u32 = 10;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {
            #[doc = "Not present"]
            pub const NOT_PRESENT: u32 = 0;
            #[doc = "Present"]
            pub const PRESENT: u32 = 0x01;
        }
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 0 configuration register"]
pub mod ISIDKC3PF0CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 1 configuration register"]
pub mod ISIDKC3PF1CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 2 configuration register"]
pub mod ISIDKC3PF2CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 3 configuration register"]
pub mod ISIDKC3PF3CR {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    pub mod PFP {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0x01 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    pub mod NUM_BYTES {
        pub const offset: u32 = 1;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    pub mod BYTE_OFFSET {
        pub const offset: u32 = 8;
        pub const mask: u32 = 0x7f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    pub mod FBMASK {
        pub const offset: u32 = 16;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    pub mod LBMASK {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x07 << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream filter hash table capability register"]
pub mod ISFHTCAPR {
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    pub mod ACCESS_METH {
        pub const offset: u32 = 20;
        pub const mask: u32 = 0x0f << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
#[doc = "Ingress stream filter hash table operational register"]
pub mod ISFHTOR {
    #[doc = "Number of entries in-use by this table."]
    pub mod NUM_ENTRIES {
        pub const offset: u32 = 0;
        pub const mask: u32 = 0xffff << offset;
        pub mod R {}
        pub mod W {}
        pub mod RW {}
    }
}
pub mod numprofile {
    #[doc = "VLAN to IPV mapping profile register set."]
    #[repr(C)]
    pub struct RegisterBlock {
        #[doc = "VLAN to IPV mapping profile v register 0"]
        pub VLANIPVMPR0: crate::RWRegister<u32>,
        #[doc = "VLAN to IPV mapping profile v register 1"]
        pub VLANIPVMPR1: crate::RWRegister<u32>,
        #[doc = "VLAN to DR mapping profile v register"]
        pub VLANDRMPR: crate::RWRegister<u32>,
        _reserved0: [u8; 0x04],
    }
    #[doc = "VLAN to IPV mapping profile v register 0"]
    pub mod VLANIPVMPR0 {
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_1 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_2 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_3 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_4 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_5 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_6 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_7 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "VLAN to IPV mapping profile v register 1"]
    pub mod VLANIPVMPR1 {
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_8 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_9 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_10 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_11 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_12 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_13 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_14 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "IPV value used for receive data path."]
        pub mod PCP_DEI_15 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x07 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
    #[doc = "VLAN to DR mapping profile v register"]
    pub mod VLANDRMPR {
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_0 {
            pub const offset: u32 = 0;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_1 {
            pub const offset: u32 = 2;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_2 {
            pub const offset: u32 = 4;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_3 {
            pub const offset: u32 = 6;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_4 {
            pub const offset: u32 = 8;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_5 {
            pub const offset: u32 = 10;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_6 {
            pub const offset: u32 = 12;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_7 {
            pub const offset: u32 = 14;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_8 {
            pub const offset: u32 = 16;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_9 {
            pub const offset: u32 = 18;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_10 {
            pub const offset: u32 = 20;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_11 {
            pub const offset: u32 = 22;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_12 {
            pub const offset: u32 = 24;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_13 {
            pub const offset: u32 = 26;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_14 {
            pub const offset: u32 = 28;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
        #[doc = "DR value used for receive data path."]
        pub mod PCP_DEI_15 {
            pub const offset: u32 = 30;
            pub const mask: u32 = 0x03 << offset;
            pub mod R {}
            pub mod W {}
            pub mod RW {}
        }
    }
}