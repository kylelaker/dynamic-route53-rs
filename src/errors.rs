use core::fmt;
use hickory_client::error::{ClientError, ClientErrorKind};
use hickory_client::proto::error::ProtoError;
use std::error::Error;
use std::io;
use std::net::AddrParseError;

#[derive(Debug)]
pub enum AddressResolutionError {
    InvalidHostFormat(io::Error),
    DnsResolutionFailure(String),
    InvalidCheckIpResponse(AddrParseError),
}

impl From<io::Error> for AddressResolutionError {
    fn from(e: io::Error) -> AddressResolutionError {
        AddressResolutionError::InvalidHostFormat(e)
    }
}

impl From<AddrParseError> for AddressResolutionError {
    fn from(e: AddrParseError) -> AddressResolutionError {
        AddressResolutionError::InvalidCheckIpResponse(e)
    }
}

impl From<ClientError> for AddressResolutionError {
    fn from(e: ClientError) -> AddressResolutionError {
        match e.kind() {
            ClientErrorKind::Msg(s) => AddressResolutionError::DnsResolutionFailure(s.to_owned()),
            ClientErrorKind::Message(s) => {
                AddressResolutionError::DnsResolutionFailure((*s).to_owned())
            }
            &_ => AddressResolutionError::DnsResolutionFailure(format!("{:?}", e)),
        }
    }
}

impl From<ProtoError> for AddressResolutionError {
    fn from(e: ProtoError) -> AddressResolutionError {
        AddressResolutionError::DnsResolutionFailure(format!("{:?}", e))
    }
}

impl fmt::Display for AddressResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressResolutionError::DnsResolutionFailure(host) => {
                write!(f, "AddressResolutionError({})", host)
            }
            AddressResolutionError::InvalidCheckIpResponse(e) => {
                write!(f, "InvalidCheckIpResponse({})", e)
            }
            AddressResolutionError::InvalidHostFormat(e) => write!(f, "InvalidHostFormat({})", e),
        }
    }
}

impl Error for AddressResolutionError {}
