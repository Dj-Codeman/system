use std::fmt;
use crate::core::errors::enums::errors::Errors;
use crate::core::errors::structs::warning_item::WarningArrayItem;
use crate::core::errors::structs::error_item::ErrorArrayItem;

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            // File-related errors
            Errors::OpeningFile                => "while opening a file",
            Errors::ReadingFile                => "while reading from a file",
            Errors::CreatingFile               => "while creating a file",
            Errors::DeletingFile               => "while deleting a file",
            Errors::SettingPermissionsFile     => "while setting permissions on a file",
            Errors::UntaringFile               => "while untaring a file",
            Errors::InvalidFile                => "Invalid file",

            // Directory-related errors
            Errors::CreatingDirectory          => "while creating a directory",
            Errors::DeletingDirectory          => "while deleting a directory",
            Errors::SettingPermissionsDirectory=> "while setting permissions on a directory",

            // JSON-related errors
            Errors::JsonCreation               => "while creating JSON data",
            Errors::JsonReading                => "while reading JSON data",

            // JWT errors
            Errors::JWT                        => "General JWT error",
            Errors::JWTAUTH                    => "Invalid JWT token",

            // Data-related errors
            Errors::InvalidType                => "Invalid data type",
            Errors::InvalidChunkData           => "Invalid chunk data",
            Errors::InvalidHMACData            => "Invalid HMAC data",
            Errors::InvalidHMACSize            => "Invalid HMAC size",
            Errors::InvalidKey                 => "Invalid encryption key",
            Errors::InvalidHexData             => "Invalid hexadecimal data",
            Errors::InvalidIvData              => "Invalid initialization vector (IV) data",
            Errors::InvalidBlockData           => "Invalid block data",
            Errors::InvalidAuthRequest         => "Invalid authentication request",
            Errors::InvalidMapRequest          => "Invalid map request",
            Errors::InvalidMapVersion          => "Invalid map version",
            Errors::InvalidMapData             => "Invalid map data",
            Errors::InvalidMapHash             => "Invalid map hash",
            Errors::InvalidBufferFit           => "Invalid buffer fit",
            Errors::InvalidUtf8Data            => "Invalid UTF-8 data",
            Errors::InvalidSignature           => "Invalid signature",

            // Keystore errors
            Errors::KeyStoreUnavaible          => "Keystore server unavailable",
            Errors::KeyStoreInvalidKey         => "Invalid keystore key",
            Errors::KeyStoreTimedout           => "Keystore request timed out",

            // Permission and access errors
            Errors::PermissionDenied           => "Permission denied",
            Errors::Unauthorized               => "Unauthorized access",
            Errors::NotFound                   => "Resource not found",

            // Network and protocol errors
            Errors::Network                    => "Network error",
            Errors::Protocol                   => "Protocol error",
            Errors::ConnectionError            => "Connection error",
            Errors::Timeout                    => "Timeout error",
            Errors::ConnectionTimedOut         => "Connection timed out",
            Errors::PortalNotFound             => "Portal not found",
            Errors::PortalConnectionFailed     => "Portal connection failed",

            // Authentication-related errors
            Errors::AuthenticationError        => "Authentication error",
            Errors::IdentityError              => "Identity error",
            Errors::IdentityInvalid            => "Invalid identity",

            // Application state and config errors
            Errors::AppState                   => "Application state error",
            Errors::ConfigReading              => "Error reading configuration",
            Errors::ConfigParsing              => "Error parsing configuration",

            // Resource and memory errors
            Errors::OutOfMemory                => "Out of memory",
            Errors::OverRamLimit               => "Over RAM limit",

            // Message encoding/decoding errors
            Errors::MessageDecode              => "Error decoding a message",
            Errors::MessageEncode              => "Error encoding a message",

            // Locking and sync errors
            Errors::TimedOut                   => "Operation timed out",
            Errors::LockWithTimeoutRead        => "Read lock timed out",
            Errors::LockWithTimeoutWrite       => "Write lock timed out",

            // Process supervision errors
            Errors::SupervisedChild            => "Supervised child process error",
            Errors::SupervisedChildDied        => "Supervised child process died unexpectedly",
            Errors::SupervisedChildKilled      => "Supervised child process was killed",
            Errors::SupervisedChildLost        => "Supervised child process was lost",
            Errors::SupervisedChildFat         => "Error in supervised child process",

            // General-purpose errors
            Errors::InputOutput                => "General input/output error",
            Errors::GeneralError               => "General error",
            Errors::InitializationError        => "Initialization error",
            Errors::SecretArray                => "Error related to a secret array",

            // Git-related errors
            Errors::Git                        => "Git error",
            Errors::GitFileMissing             => "Git file missing",
            Errors::GitFileIllegible           => "Git file illegible",

            // Toggle control errors
            Errors::ToggleControl              => "Toggle control error",

            // Deprecated errors
            Errors::DEPSYSTEM                  => "Deprecated system error",
            Errors::DEPLOGGER                  => "Deprecated logger error",
            Errors::DEPRECS                    => "Deprecated recommendation system error",
        };
        write!(f, "{}", msg)
    }
}

// Pretty display for WarningArrayItem
impl fmt::Display for WarningArrayItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.warn_mesg {
            Some(d) => write!(f, "Warning: {:#?} - {}", self.warn_type, d),
            None => write!(f, "Warning: {:#?}", self.warn_type),
        }
    }
}

// 1) Display: what shows up when someone does `println!("{}", err)`
impl fmt::Display for ErrorArrayItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // This assumes your `Errors` enum implements Display;
        // if not, you can use `{:?}` instead of `{}` here.
        write!(f, "{}: {}", self.err_type, self.err_mesg)
    }
}