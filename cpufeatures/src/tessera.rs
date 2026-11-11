//! Mosaik on the Tessera microkernel (`target_os = "tessera"`).
//!
//! The kernel writes a table of the features programs may use once at
//! boot, each key a register of the architecture that tells features with
//! the fields programs may use at their safe value over every CPU and the
//! others zero (Mosaik's protocol `cpu.features.v1`). The program's
//! runtime reads the keys once at its start and answers them through
//! `mosaik_runtime_cpu_feature_v1`, as `getauxval` answers on Linux; this
//! module asks it. A key the table does not hold reads as zero: every
//! field absent.

/// What `mosaik_runtime_cpu_feature_v1` answers for a key the table does
/// not hold (`cpu.features.v1`'s `NOT_HELD`).
const NOT_HELD: u64 = u64::MAX;

unsafe extern "C" {
    /// The value of `key` in the table of CPU features, or `NOT_HELD`.
    /// Defined by Mosaik's runtime, which every program links.
    fn mosaik_runtime_cpu_feature_v1(key: u64) -> u64;
}

/// The register `key` names as the table holds it; zero, every field
/// absent, where the table does not hold it.
#[must_use]
pub fn register(key: u64) -> u64 {
    // SAFETY: the runtime's function reads what it read once at the start
    // and has no precondition.
    match unsafe { mosaik_runtime_cpu_feature_v1(key) } {
        NOT_HELD => 0,
        value => value,
    }
}

/// The keys of `cpu.features.v1` for `aarch64`: an ID register by its
/// encoding as `MRS` takes it, with the kind of a register in bits 60 to
/// 63.
#[cfg(target_arch = "aarch64")]
pub mod keys {
    /// `ID_AA64PFR0_EL1`.
    pub const ID_AA64PFR0_EL1: u64 = 0x1000_0000_0000_C020;
    /// `ID_AA64ISAR0_EL1`.
    pub const ID_AA64ISAR0_EL1: u64 = 0x1000_0000_0000_C030;
}

/// The keys of `cpu.features.v1` for `x86_64`: a register of a CPUID leaf,
/// the leaf in bits 24 to 55, the subleaf in bits 8 to 23 and the register
/// in bits 0 and 1 (`eax` to `edx`), with the kind of a register in bits
/// 60 to 63.
#[cfg(target_arch = "x86_64")]
pub mod keys {
    /// Leaf 1, `ecx`.
    pub const LEAF_1_ECX: u64 = 0x1000_0000_0100_0002;
    /// Leaf 1, `edx`.
    pub const LEAF_1_EDX: u64 = 0x1000_0000_0100_0003;
    /// Leaf 7, subleaf 0, `ebx`.
    pub const LEAF_7_EBX: u64 = 0x1000_0000_0700_0001;
    /// Leaf 7, subleaf 0, `ecx`.
    pub const LEAF_7_ECX: u64 = 0x1000_0000_0700_0002;
    /// Leaf 7, subleaf 0, `edx`.
    pub const LEAF_7_EDX: u64 = 0x1000_0000_0700_0003;
    /// Leaf 7, subleaf 1, `eax`.
    pub const LEAF_7_1_EAX: u64 = 0x1000_0000_0700_0100;
}

/// The field of four bits at `shift` of an ID register of `aarch64`.
#[cfg(target_arch = "aarch64")]
#[must_use]
pub const fn field(register: u64, shift: u32) -> u64 {
    (register >> shift) & 0xF
}

/// The register of a CPUID leaf `key` names, as the table holds it.
#[cfg(target_arch = "x86_64")]
#[must_use]
#[allow(clippy::cast_possible_truncation)] // The table holds 32 bits.
pub fn cpuid_register(key: u64) -> u32 {
    register(key) as u32
}
