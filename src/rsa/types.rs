// ============================================================================
// RSA Cryptographic Types
// ============================================================================

/// RSA key pair containing both public and private keys
#[derive(Clone)]
pub struct RsaKeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

/// RSA public key used for encryption
#[derive(Clone)]
pub struct RsaPublicKey {
    pub n: u64,
    pub e: u64,
}

/// RSA private key used for decryption
#[derive(Clone)]
pub struct RsaPrivateKey {
    pub n: u64,
    pub d: u64,
}

// ============================================================================
// Input Types for Demo Application
// ============================================================================

/// Input type wrapper for demo orchestration
pub(super) enum InputType {
    Number(u64),
    Text(String),
}

impl InputType {
    /// Converts input to a number for RSA encryption
    /// Returns None if text is too long or empty
    pub(super) fn to_number(&self, text_converter: impl Fn(&str) -> Option<u64>) -> Option<u64> {
        match self {
            InputType::Number(n) => Some(*n),
            InputType::Text(text) => text_converter(text),
        }
    }

    pub(super) fn is_text(&self) -> bool {
        matches!(self, InputType::Text(_))
    }

    pub(super) fn as_text(&self) -> Option<&str> {
        match self {
            InputType::Text(text) => Some(text),
            _ => None,
        }
    }
}

/// Message format type for output display
pub(super) enum MessageFormat {
    Number,
    Text(String),
}

impl MessageFormat {
    pub(super) fn as_text(&self) -> Option<&str> {
        match self {
            MessageFormat::Text(text) => Some(text),
            _ => None,
        }
    }
}

impl From<&InputType> for MessageFormat {
    fn from(input: &InputType) -> Self {
        match input {
            InputType::Number(_) => MessageFormat::Number,
            InputType::Text(text) => MessageFormat::Text(text.clone()),
        }
    }
}
