use binary_sv2::Error as BinarySv2Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
/// No NoPairableUpstream((min_v, max_v, all falgs supported))
pub enum Error {
    ExpectedLen32(usize),
    BinarySv2Error(BinarySv2Error),
    NoGroupsFound,
    WrongMessageType(u8),
    UnexpectedMessage,
    // min_v max_v all falgs supported
    NoPairableUpstream((u16, u16, u32)),
    /// Error if the hashmap `future_jobs` field in the `GroupChannelJobDispatcher` is empty.
    NoFutureJobs,
    NoDownstreamsConnected,
    PrevHashRequireNonExistentJobId(u32),
    RequestIdNotMapped(u32),
    NoUpstreamsConnected,
    UnknownRequestId(u32),
}

impl From<BinarySv2Error> for Error {
    fn from(v: BinarySv2Error) -> Error {
        Error::BinarySv2Error(v)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Error::*;
        match self {
            BinarySv2Error(v) => write!(
                f,
                "BinarySv2Error: error in serializing/deserilizing binary format {:?}",
                v
            ),
            ExpectedLen32(l) => write!(f, "Expected length of 32, but received length of {}", l),
            NoGroupsFound => write!(
                f,
                "A channel was attempted to be added to an Upstream, but no groups are specified"
            ),
            WrongMessageType(m) => write!(f, "Wrong message type: {}", m),
            UnexpectedMessage => write!(f, "Error: Unexpected message received"),
            NoPairableUpstream(a) => {
                write!(f, "No pairable upstream node: {:?}", a)
            }
            NoFutureJobs => write!(f, "GroupChannelJobDispatcher does not have any future jobs"),
            NoDownstreamsConnected => write!(f, "NoDownstreamsConnected"),
            PrevHashRequireNonExistentJobId(id) => {
                write!(f, "PrevHashRequireNonExistentJobId {}", id)
            }
            RequestIdNotMapped(id) => write!(f, "RequestIdNotMapped {}", id),
            NoUpstreamsConnected => write!(f, "There are no upstream connected"),
            UnknownRequestId(id) => write!(
                f,
                "Upstream is answering with a wrong request ID {} or
                DownstreamMiningSelector::on_open_standard_channel_request has not been called
                before relaying open channel request to upstream",
                id
            ),
        }
    }
}
