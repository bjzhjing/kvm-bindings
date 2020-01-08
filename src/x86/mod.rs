// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "with-serde")]
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "fam-wrappers")]
mod fam_wrappers;

#[cfg(feature = "kvm-v4_14_0")]
mod bindings_v4_14_0;
#[cfg(feature = "kvm-v4_20_0")]
mod bindings_v4_20_0;

#[cfg(feature = "with-serde")]
mod serializers;

// Major hack to have a default version in case no feature is specified:
// If no version is specified by using the features, just use the latest one
// which currently is 4.20.
#[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
mod bindings_v4_20_0;

pub mod bindings {
    #[cfg(feature = "kvm-v4_14_0")]
    pub use super::bindings_v4_14_0::*;

    #[cfg(feature = "kvm-v4_20_0")]
    pub use super::bindings_v4_20_0::*;

    #[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
    pub use super::bindings_v4_20_0::*;

    #[cfg(feature = "fam-wrappers")]
    pub use super::fam_wrappers::*;

    #[cfg(feature = "with-serde")]
    pub use super::serializers::*;
}

#[cfg_attr(feature = "with-serde", derive(Deserialize, Serialize))]
#[cfg_attr(test, derive(Debug, PartialEq))]
/// Composite version of the autogenerated bindings.
pub struct Version {
    /// Architecture.
    pub arch: &'static str,
    /// Kernel version.
    pub kernel_ver: &'static str,
    /// Crate version.
    pub crate_ver: &'static str,
}

#[allow(unused)]
static VERSION: Version = Version {
    #[cfg(target_arch = "x86")]
    arch: "x86",
    #[cfg(target_arch = "x86_64")]
    arch: "x86_64",

    #[cfg(feature = "kvm-v4_14_0")]
    kernel_ver: "v4.14.0",
    #[cfg(feature = "kvm-v4_20_0")]
    kernel_ver: "v4.20.0",
    #[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
    kernel_ver: "v4.20.0",

    crate_ver: env!("CARGO_PKG_VERSION"),
};

#[cfg(test)]
mod tests {
    #[cfg(feature = "with-serde")]
    extern crate serde_json;

    #[cfg(feature = "with-serde")]
    use super::bindings::*;
    use super::VERSION;

    #[cfg(feature = "with-serde")]
    #[test]
    fn test_ser_deser() {
        {
            // Test kvm_regs ser/deser.
            let val = kvm_regs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_regs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_segment ser/deser.
            let val = kvm_segment::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_segment>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_dtable ser/deser.
            let val = kvm_dtable::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_dtable>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_sregs ser/deser.
            let val = kvm_sregs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_sregs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_pit_state2 ser/deser.
            // Also covers kvm_pit_channel_state.
            let val = kvm_pit_state2::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_pit_state2>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_vcpu_events ser/deser.
            // Also covers:
            // * kvm_vcpu_events__bindgen_ty_1
            // * kvm_vcpu_events__bindgen_ty_2
            // * kvm_vcpu_events__bindgen_ty_3
            // * kvm_vcpu_events__bindgen_ty_4
            let val = kvm_vcpu_events::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_vcpu_events>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_debugregs ser/deser.
            let val = kvm_debugregs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_debugregs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_xcrs ser/deser.
            // Also covers kvm_xcr.
            let val = kvm_xcrs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_xcrs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_mp_state ser/deser.
            let val = kvm_mp_state::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_mp_state>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_clock_data ser/deser.
            let val = kvm_clock_data::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_clock_data>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_regs ser/deser.
            let val = kvm_regs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_regs>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }

        {
            // Test kvm_lapic_state ser/deser.
            let val = kvm_lapic_state::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_lapic_state>(val_ser.as_str()).unwrap();
            assert_eq!(&val.regs[..], &val_deser.regs[..]);
        }

        {
            // Test kvm_xsave ser/deser.
            let val = kvm_xsave::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_xsave>(val_ser.as_str()).unwrap();
            assert_eq!(&val.region[..], &val_deser.region[..]);
        }

        {
            // Test kvm_msrs ser/deser.
            let val = kvm_msrs::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_msrs>(val_ser.as_str()).unwrap();
            // TODO encapsulate in FamStructWrappers and operate on those after
            // https://github.com/rust-vmm/vmm-sys-util/issues/65 is closed.
            // Until then, only partial comparison can be performed.
            assert_eq!(val.nmsrs, val_deser.nmsrs);
            assert_eq!(val_deser.pad, 0u32);
        }

        {
            // Test kvm_cpuid2 ser/deser.
            let val = kvm_cpuid2::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_cpuid2>(val_ser.as_str()).unwrap();
            // TODO encapsulate in FamStructWrappers and operate on those after
            // https://github.com/rust-vmm/vmm-sys-util/issues/65 is closed.
            // Until then, only partial comparison can be performed.
            assert_eq!(val.nent, val_deser.nent);
            assert_eq!(val_deser.padding, 0u32);
        }

        {
            // Test kvm_irqchip ser/deser.
            let val = kvm_irqchip::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_irqchip>(val_ser.as_str()).unwrap();
            assert_eq!(val.chip_id, val_deser.chip_id);
            assert_eq!(val_deser.pad, 0u32);
            unsafe {
                assert_eq!(
                    val.chip.ioapic.base_address,
                    val_deser.chip.ioapic.base_address
                );
                assert_eq!(val.chip.ioapic.id, val_deser.chip.ioapic.id);
                assert_eq!(val.chip.ioapic.ioregsel, val_deser.chip.ioapic.ioregsel);
                assert_eq!(val.chip.ioapic.irr, val_deser.chip.ioapic.irr);
                for i in 0..24 {
                    assert_eq!(
                        val.chip.ioapic.redirtbl[i].bits,
                        val_deser.chip.ioapic.redirtbl[i].bits
                    );
                }
            }
        }
        {
            // Test kvm_fpu ser/deser.
            let val = kvm_fpu::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_fpu>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }
        {
            // Test kvm_fpu ser/deser.
            let val = kvm_msr_entry::default();
            let val_ser = serde_json::to_string(&val).unwrap();
            let val_deser = serde_json::from_str::<kvm_msr_entry>(val_ser.as_str()).unwrap();
            assert_eq!(val, val_deser);
        }
    }

    #[test]
    fn test_version() {
        #[cfg(target_arch = "x86")]
        assert_eq!(VERSION.arch, "x86");
        #[cfg(target_arch = "x86_64")]
        assert_eq!(VERSION.arch, "x86_64");

        #[cfg(feature = "kvm-v4_14_0")]
        assert_eq!(VERSION.kernel_ver, "v4.14.0");
        #[cfg(feature = "kvm-v4_20_0")]
        assert_eq!(VERSION.kernel_ver, "v4.20.0");
        #[cfg(all(not(feature = "kvm-v4_14_0"), not(feature = "kvm-v4_20_0")))]
        assert_eq!(VERSION.kernel_ver, "v4.20.0");

        assert_eq!(VERSION.crate_ver, env!("CARGO_PKG_VERSION"));
    }
}
