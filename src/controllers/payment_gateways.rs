use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewayUpdate {
    /// Payment gateway enabled status.
    pub enabled: bool,
}
