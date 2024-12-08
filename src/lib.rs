// Import the main contract module
pub mod contract;  // This is the main contract (could be the Tree Loan contract)

// Import additional contract modules
pub mod contribution_token;  // Module for the contribution token contract
//pub mod reforestation_manager;  // Module for the reforestation management contract

// Other modules
mod error;        // Error handling module
pub mod helpers;  // Helper functions module
pub mod msg;      // Message types module
pub mod state;    // State management module

// Export the contract error type for use in other modules
pub use crate::error::ContractError;
