use serde::{Deserialize, Serialize};

/// Represents different types of generic errors.
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize, Eq, PartialOrd, Ord)]
pub enum Errors {
    // File-related errors
    /// Error encountered while opening a file.
    OpeningFile,
    /// Error encountered while reading from a file.
    ReadingFile,
    /// Error encountered while creating a file.
    CreatingFile,
    /// Error encountered while deleting a file.
    DeletingFile,
    /// Error encountered while setting permissions on a file.
    SettingPermissionsFile,
    /// Error encountered while untaring a file.
    UntaringFile,
    /// Invalid file.
    InvalidFile,

    // Directory-related errors
    /// Error encountered while creating a directory.
    CreatingDirectory,
    /// Error encountered while deleting a directory.
    DeletingDirectory,
    /// Error encountered while setting permissions on a directory.
    SettingPermissionsDirectory,

    // JSON-related errors
    /// Error encountered while creating JSON data.
    JsonCreation,
    /// Error encountered while reading JSON data.
    JsonReading,

    // Json Web Token errors
    /// General JWT ERROR
    JWT,
    /// Invalid jwt token
    JWTAUTH,

    // Data-related errors
    /// Invalid data type.
    InvalidType,
    /// Invalid chunk data.
    InvalidChunkData,
    /// Invalid HMAC data.
    InvalidHMACData,
    /// Invalid HMAC size.
    InvalidHMACSize,
    /// Invalid encryption key.
    InvalidKey,
    /// Invalid hexadecimal data.
    InvalidHexData,
    /// Invalid initialization vector (IV) data.
    InvalidIvData,
    /// Invalid block data.
    InvalidBlockData,
    /// Invalid authentication request.
    InvalidAuthRequest,
    /// Invalid map request.
    InvalidMapRequest,
    /// Invalid map version.
    InvalidMapVersion,
    /// Invalid map data.
    InvalidMapData,
    /// Invalid map hash.
    InvalidMapHash,
    /// Invalid buffer fit.
    InvalidBufferFit,
    /// Invalid UTF-8 data.
    InvalidUtf8Data,
    /// Invalid signature.
    InvalidSignature,

    // Keystore errors
    /// Error accessing the keystore server
    KeyStoreUnavaible,
    /// We were expecting a diffrent key
    KeyStoreInvalidKey,
    /// They left me on read ....
    KeyStoreTimedout,

    // Permission and access errors
    /// Permission denied.
    PermissionDenied,
    /// Unauthorized access.
    Unauthorized,
    /// Resource not found.
    NotFound,

    // Network and protocol errors
    /// Network error.
    Network,
    /// Protocol error.
    Protocol,
    /// Connection error.
    ConnectionError,
    /// Timeout error.
    Timeout,
    /// Connection timed out.
    ConnectionTimedOut,
    /// Portal not found.
    PortalNotFound,
    /// Portal connection failed.
    PortalConnectionFailed,

    // Authentication-related errors
    /// Authentication error.
    AuthenticationError,
    /// Error related to identity.
    IdentityError,
    /// Invalid identity.
    IdentityInvalid,

    // Application state and configuration errors
    /// Error in application state.
    AppState,
    /// Error reading configuration.
    ConfigReading,
    /// Error parsing configuration.
    ConfigParsing,

    // Resource and memory-related errors
    /// Out of memory.
    OutOfMemory,
    /// Over RAM limit.
    OverRamLimit,

    // Message encoding/decoding errors
    /// Error decoding a message.
    MessageDecode,
    /// Error encoding a message.
    MessageEncode,

    // Locking and synchronization errors
    /// Timed out.
    TimedOut,
    /// Error with read lock timeout.
    LockWithTimeoutRead,
    /// Error with write lock timeout.
    LockWithTimeoutWrite,

    // Process supervision errors
    /// Supervised child process error.
    SupervisedChild,
    /// Supervised child process died unexpectedly.
    SupervisedChildDied,
    /// Supervised child process was killed.
    SupervisedChildKilled,
    /// Supervised child process was lost.
    SupervisedChildLost,
    /// Error in supervised child process.
    SupervisedChildFat,

    // General-purpose errors
    /// General input/output error.
    InputOutput,
    /// General error.
    GeneralError,
    /// Initialization error.
    InitializationError,
    /// Error related to a secret array.
    SecretArray,

    // Git-related errors
    /// Git error.
    Git,
    /// Git file
    GitFileMissing,
    /// Parsing git file error
    GitFileIllegible,

    // Toggle control errors
    /// Toggle control error.
    ToggleControl,

    // Deprecated errors
    /// Deprecated system errors.
    DEPSYSTEM,
    /// Deprecated logger errors.
    DEPLOGGER,
    /// Deprecated recommendation system errors.
    DEPRECS,
}