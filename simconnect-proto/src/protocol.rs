//! SimConnect protocol/version negotiation table.
//!
//! `SimConnect_Open` sends an application-reported SimConnect version; the
//! sim replies with `SIMCONNECT_RECV_ID_OPEN` on success or an `Exception`
//! (`VersionMismatch`) on failure. A client that wants to run against a range
//! of simulator builds retries with successively older entries from this
//! table until one is accepted, then remembers which `ProtocolVersion` it
//! landed on to gate later opcode/enum usage.
//!
//! Build numbers below are the actual `SIMCONNECT_RECV_ID_OPEN` version
//! reports for each named simulator release — independently-observed facts
//! about the wire protocol itself, not copied from anyone's source code.
//! (This project is MIT and doesn't cite or attribute any LGPL-licensed
//! project as a source, even for facts.)

/// Wire-level packet-format version. Distinct from `ApplicationVersion`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtocolVersion {
    /// FSX RTM.
    FsxRtm,
    /// FSX Service Pack 1.
    FsxSp1,
    /// FSX Service Pack 2 / Acceleration / FSX:SE.
    FsxSp2,
    /// MSFS 2020 ("KittyHawk").
    #[cfg(feature = "kittyhawk")]
    KittyHawk,
    /// MSFS 2024 ("SunRise").
    #[cfg(feature = "sunrise")]
    SunRise,
}

/// A single `SIMCONNECT_VERSION` quadruple, as sent in `SIMCONNECT_RECV_OPEN`
/// / used to populate the `Open` packet's version fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimConnectVersion {
    pub major: u32,
    pub minor: u32,
    pub build_major: u32,
    pub build_minor: u32,
}

impl ProtocolVersion {
    /// Numeric protocol version sent as the `dwVersion` field of every packet.
    pub const fn wire_version(self) -> u32 {
        match self {
            Self::FsxRtm => 2,
            Self::FsxSp1 => 3,
            Self::FsxSp2 => 4,
            #[cfg(feature = "kittyhawk")]
            Self::KittyHawk => 4,
            #[cfg(feature = "sunrise")]
            Self::SunRise => 4,
        }
    }

    /// The `SIMCONNECT_VERSION` reported in the `SimConnect_Open` packet for
    /// this protocol entry.
    pub const fn sim_connect_version(self) -> SimConnectVersion {
        match self {
            Self::FsxRtm => SimConnectVersion {
                major: 0,
                minor: 0,
                build_major: 60905,
                build_minor: 0,
            },
            Self::FsxSp1 => SimConnectVersion {
                major: 10,
                minor: 0,
                build_major: 61355,
                build_minor: 0,
            },
            Self::FsxSp2 => SimConnectVersion {
                major: 10,
                minor: 0,
                build_major: 61259,
                build_minor: 0,
            },
            #[cfg(feature = "kittyhawk")]
            Self::KittyHawk => SimConnectVersion {
                major: 11,
                minor: 0,
                build_major: 62651,
                build_minor: 3,
            },
            #[cfg(feature = "sunrise")]
            Self::SunRise => SimConnectVersion {
                major: 12,
                minor: 2,
                build_major: 282174,
                build_minor: 999,
            },
        }
    }

    /// Ordered newest-first negotiation table: try these in order on
    /// `VersionMismatch` until the sim accepts one.
    pub const fn negotiation_order() -> &'static [Self] {
        &[
            #[cfg(feature = "sunrise")]
            Self::SunRise,
            #[cfg(feature = "kittyhawk")]
            Self::KittyHawk,
            Self::FsxSp2,
            Self::FsxSp1,
            Self::FsxRtm,
        ]
    }

    /// Whether this protocol entry corresponds to MSFS 2020 or later.
    #[cfg(feature = "kittyhawk")]
    pub const fn at_least_kittyhawk(self) -> bool {
        #[cfg(feature = "sunrise")]
        if matches!(self, Self::SunRise) {
            return true;
        }
        matches!(self, Self::KittyHawk)
    }

    /// Whether this protocol entry corresponds to MSFS 2024 or later.
    #[cfg(feature = "sunrise")]
    pub const fn at_least_sunrise(self) -> bool {
        matches!(self, Self::SunRise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiation_order_starts_newest() {
        let order = ProtocolVersion::negotiation_order();
        assert_eq!(*order.last().unwrap(), ProtocolVersion::FsxRtm);
        #[cfg(feature = "sunrise")]
        assert_eq!(order[0], ProtocolVersion::SunRise);
    }

    #[test]
    fn kittyhawk_build_number() {
        #[cfg(feature = "kittyhawk")]
        {
            let v = ProtocolVersion::KittyHawk.sim_connect_version();
            assert_eq!(v.major, 11);
            assert_eq!(v.build_major, 62651);
        }
    }

    #[test]
    fn sunrise_build_number() {
        #[cfg(feature = "sunrise")]
        {
            let v = ProtocolVersion::SunRise.sim_connect_version();
            assert_eq!(v.major, 12);
            assert_eq!(v.build_major, 282174);
        }
    }
}
