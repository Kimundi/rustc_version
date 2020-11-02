use semver::{self, Identifier};
use std::{self, error, fmt, io, num, str};

/// LLVM Version Parse Error
#[derive(Debug)]
pub enum LlvmVersionParseError {
    /// An error occurred in parsing a version component as an integer
    ParseIntError(num::ParseIntError),
    /// A version component must not have leading zeros
    ComponentMustNotHaveLeadingZeros,
    /// A version component has a sign
    ComponentMustNotHaveSign,
    /// Minor version component must be zero on LLVM versions later than 4.0
    MinorVersionMustBeZeroAfter4,
    /// Minor version component is required on LLVM versions earlier than 4.0
    MinorVersionRequiredBefore4,
    /// Too many components
    TooManyComponents,
}

impl From<num::ParseIntError> for LlvmVersionParseError {
    fn from(e: num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl fmt::Display for LlvmVersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseIntError(e) => write!(f, "error parsing LLVM version component: {}", e),
            Self::ComponentMustNotHaveLeadingZeros => {
                write!(f, "a version component must not have leading zeros")
            }
            Self::ComponentMustNotHaveSign => write!(f, "a version component must not have a sign"),
            Self::MinorVersionMustBeZeroAfter4 => write!(
                f,
                "LLVM's minor version component must be 0 for versions greater than 4.0"
            ),
            Self::MinorVersionRequiredBefore4 => write!(
                f,
                "LLVM's minor version component is required for versions less than 4.0"
            ),
            Self::TooManyComponents => write!(f, "too many version components"),
        }
    }
}

impl error::Error for LlvmVersionParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::ParseIntError(e) => Some(e),
            Self::ComponentMustNotHaveLeadingZeros
            | Self::ComponentMustNotHaveSign
            | Self::MinorVersionMustBeZeroAfter4
            | Self::MinorVersionRequiredBefore4
            | Self::TooManyComponents => None,
        }
    }
}

/// The error type for this crate.
#[derive(Debug)]
pub enum Error {
    /// An error occurred when executing the `rustc` command.
    CouldNotExecuteCommand(io::Error),
    /// The output of `rustc -vV` was not valid utf-8.
    Utf8Error(str::Utf8Error),
    /// The output of `rustc -vV` was not in the expected format.
    UnexpectedVersionFormat,
    /// An error occurred in parsing a `VersionReq`.
    ReqParseError(semver::ReqParseError),
    /// An error occurred in parsing the semver.
    SemVerError(semver::SemVerError),
    /// The pre-release tag is unknown.
    UnknownPreReleaseTag(Identifier),
    /// An error occurred in parsing a `LlvmVersion`.
    LlvmVersionError(LlvmVersionParseError),
}
use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CouldNotExecuteCommand(ref e) => write!(f, "could not execute command: {}", e),
            Utf8Error(_) => write!(f, "invalid UTF-8 output from `rustc -vV`"),
            UnexpectedVersionFormat => write!(f, "unexpected `rustc -vV` format"),
            ReqParseError(ref e) => write!(f, "error parsing version requirement: {}", e),
            SemVerError(ref e) => write!(f, "error parsing version: {}", e),
            UnknownPreReleaseTag(ref i) => write!(f, "unknown pre-release tag: {}", i),
            LlvmVersionError(ref e) => write!(f, "error parsing LLVM's version: {}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CouldNotExecuteCommand(ref e) => Some(e),
            Utf8Error(ref e) => Some(e),
            UnexpectedVersionFormat => None,
            ReqParseError(ref e) => Some(e),
            SemVerError(ref e) => Some(e),
            UnknownPreReleaseTag(_) => None,
            LlvmVersionError(ref e) => Some(e),
        }
    }
}

macro_rules! impl_from {
    ($($err_ty:ty => $variant:ident),* $(,)*) => {
        $(
            impl From<$err_ty> for Error {
                fn from(e: $err_ty) -> Error {
                    Error::$variant(e)
                }
            }
        )*
    }
}

impl_from! {
    str::Utf8Error => Utf8Error,
    semver::SemVerError => SemVerError,
    semver::ReqParseError => ReqParseError,
    LlvmVersionParseError => LlvmVersionError,
}

/// The result type for this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;
