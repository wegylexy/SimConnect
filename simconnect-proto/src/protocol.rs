//! SimConnect protocol/version negotiation table.
//!
//! `SimConnect_Open` sends an application-reported SimConnect version; the
//! sim replies with `SIMCONNECT_RECV_ID_OPEN` on success or an `Exception`
//! (`VersionMismatch`) on failure. A client that wants to run against a range
//! of simulator builds retries with successively older entries from this
//! table until one is accepted, then remembers which `ProtocolVersion` it
//! landed on to gate later opcode/enum usage.
//!
//! **This table identifies a packet format, never a product.** There is exactly
//! one entry per accepted format, because that is all the handshake can tell
//! you: the version quadruple in `SimConnect_Open` is *self-reported by the
//! client*, so a sim that can parse the packet accepts it whatever it says.
//! MSFS 2020 in particular accepts the MSFS 2024 quadruple rather than
//! replying `VersionMismatch`, so no negotiated value can distinguish the two
//! — see [`SimProduct`], which reads the sim's own `szApplicationName` out of
//! the `Open` reply and is the only sound way to ask "which product is this".
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
    /// Microsoft Flight Simulator (2020 "KittyHawk" and 2024 "SunRise" alike).
    ///
    /// One entry, not two, because there is one packet format here: both
    /// releases speak `wire_version` 4 and both accept the same `Open`. The
    /// crate used to carry a `KittyHawk` and a `SunRise` variant differing
    /// only in the quadruple announced to the sim — but 2020 accepts the 2024
    /// quadruple, so the newest entry always won on both and the older one was
    /// never reached. Keeping both invited every consumer to read the
    /// negotiated value as the sim's identity, which it has never been. Ask
    /// [`SimProduct`] for the release.
    ///
    /// Gated on `kittyhawk`, the earliest MSFS feature. The `kittyhawk` and
    /// `sunrise` features stay as they are and keep their meaning — they
    /// select which release's *opcode set* to compile (`opcode::kittyhawk`,
    /// `opcode::sunrise`), which is a real per-release difference. What is not
    /// per-release is the packet format, which is this type.
    #[cfg(feature = "kittyhawk")]
    Msfs,
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
            Self::Msfs => 4,
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
            // The MSFS 2024 ("SunRise") build report, announced to 2020
            // installs as well. Not a guess: this is already the quadruple
            // every MSFS connection sends today, because it was the first
            // entry in the negotiation order and 2020 accepts it. Collapsing
            // the two entries changes which variant is *reported*, not a
            // single byte of what goes on the wire. The 2020 quadruple
            // (11.0.62651.3) is therefore no longer sent by anything, and is
            // recorded here rather than in a dead table entry.
            #[cfg(feature = "kittyhawk")]
            Self::Msfs => SimConnectVersion {
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
            #[cfg(feature = "kittyhawk")]
            Self::Msfs,
            Self::FsxSp2,
            Self::FsxSp1,
            Self::FsxRtm,
        ]
    }

    /// Whether this connection negotiated the MSFS packet format, i.e. FSX
    /// and pre-2020 are ruled out.
    ///
    /// This is a sound question to ask of a negotiated protocol, unlike "is
    /// this 2024" — FSX rejects the MSFS quadruple with `VersionMismatch` and
    /// falls back, so reaching this entry really does mean an MSFS-era sim.
    /// Gate format-level differences on it (the `_HZ` radio events, say).
    /// There is deliberately no `at_least_sunrise` counterpart: the negotiated
    /// protocol cannot tell 2020 from 2024, and the one this crate used to
    /// offer answered "is 2024" with "did we happen to send the 2024
    /// quadruple", which is always yes. Use [`SimProduct`] instead.
    #[cfg(feature = "kittyhawk")]
    pub const fn is_msfs(self) -> bool {
        matches!(self, Self::Msfs)
    }
}

/// Which simulator is on the other end of the connection, from the
/// `szApplicationName` the sim itself reports in `SIMCONNECT_RECV_OPEN`.
///
/// The sim names itself on the very first packet it sends, in a 256-byte
/// Latin-1 field ([`crate::recv::RecvOpen::application_name`]). That report is
/// the authority on product identity, and the only one available:
///
/// - the negotiated [`ProtocolVersion`] cannot distinguish 2020 from 2024 (see
///   this module's own doc);
/// - a local process scan cannot either, once someone runs both sims at once —
///   and has nothing to look at at all when the sim is remote over
///   `SimConnect.cfg`, which is precisely when a wrong answer is hardest to
///   notice.
///
/// The strings below are the ones each release reports. Anything unrecognized
/// is [`Self::Unknown`] rather than being forced into the nearest match, and
/// callers keep the raw `application_name` for logging — a new sim, or a build
/// that renames itself, should read as "I don't know this one" instead of
/// silently claiming to be something it isn't.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimProduct {
    /// FSX, FSX: Acceleration, or FSX: Steam Edition.
    Fsx,
    /// Lockheed Martin Prepar3D.
    Prepar3D,
    /// MSFS 2020, which names itself "KittyHawk".
    Msfs2020,
    /// MSFS 2024, which names itself "SunRise".
    Msfs2024,
    /// Some other sim, or a build reporting a name this list doesn't know.
    Unknown,
}

impl SimProduct {
    /// Classifies a `SIMCONNECT_RECV_OPEN` `szApplicationName`.
    ///
    /// Case-insensitive and whitespace-trimmed, and matched on a prefix, since
    /// a release commonly appends a build or edition to its own name.
    pub fn from_application_name(application_name: &str) -> Self {
        let name = application_name.trim();
        let starts_with = |prefix: &str| {
            name.len() >= prefix.len() && name[..prefix.len()].eq_ignore_ascii_case(prefix)
        };
        if starts_with("SunRise") {
            Self::Msfs2024
        } else if starts_with("KittyHawk") {
            Self::Msfs2020
        } else if starts_with("Prepar3D") {
            Self::Prepar3D
        } else if starts_with("FSX") || starts_with("Microsoft Flight Simulator X") {
            Self::Fsx
        } else {
            Self::Unknown
        }
    }

    /// Whether this is MSFS 2024 specifically — the question the 2024-only
    /// model-matching tables and SimConnect opcodes actually need answered.
    pub const fn is_msfs2024(self) -> bool {
        matches!(self, Self::Msfs2024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negotiation_order_starts_newest() {
        let order = ProtocolVersion::negotiation_order();
        assert_eq!(*order.last().unwrap(), ProtocolVersion::FsxRtm);
        #[cfg(feature = "kittyhawk")]
        assert_eq!(order[0], ProtocolVersion::Msfs);
    }

    #[test]
    fn msfs_build_number() {
        #[cfg(feature = "kittyhawk")]
        {
            let v = ProtocolVersion::Msfs.sim_connect_version();
            assert_eq!(v.major, 12);
            assert_eq!(v.build_major, 282174);
        }
    }

    /// One entry per accepted format: the two MSFS releases share
    /// `wire_version` 4 *and* accept the same `Open`, so a second entry could
    /// only ever be dead — and was read as product identity instead.
    #[test]
    fn one_negotiation_entry_per_wire_version() {
        let mut seen: Vec<u32> = ProtocolVersion::negotiation_order()
            .iter()
            .map(|p| p.wire_version())
            .collect();
        let before = seen.len();
        seen.sort_unstable();
        seen.dedup();
        // FSX SP2 and MSFS both report 4 — a genuine collision, since they
        // announce different quadruples and the sim accepts on the quadruple.
        assert_eq!(seen.len(), before - 1);
    }

    #[test]
    fn the_sim_names_itself_and_that_is_what_identifies_it() {
        use SimProduct::*;
        assert_eq!(SimProduct::from_application_name("SunRise"), Msfs2024);
        assert_eq!(SimProduct::from_application_name("KittyHawk"), Msfs2020);
        // Trimmed, case-insensitive, and matched on a prefix: a release may
        // append a build or edition to its own name.
        assert_eq!(SimProduct::from_application_name("  kittyhawk "), Msfs2020);
        assert_eq!(SimProduct::from_application_name("FSX-SE"), Fsx);
        assert_eq!(SimProduct::from_application_name("Prepar3D v5"), Prepar3D);
        // Never forced into the nearest match.
        assert_eq!(SimProduct::from_application_name(""), Unknown);
        assert_eq!(SimProduct::from_application_name("XPlane12"), Unknown);
        // The regression the whole split exists for: 2020 is 2020 even though
        // it negotiated the 2024 quadruple, which is what every MSFS
        // connection sends.
        assert!(!SimProduct::from_application_name("KittyHawk").is_msfs2024());
        assert!(SimProduct::from_application_name("SunRise").is_msfs2024());
    }
}
