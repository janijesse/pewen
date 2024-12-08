use cosmwasm_std::{Addr, Uint128, StdResult, DepsMut, MessageInfo, Response};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContributionToken {
    pub owner: Addr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: Addr,
    pub total_contributions: Uint128,
}

pub const STATE: Item<State> = Item::new("state");
pub const CONTRIBUTIONS: Map<&Addr, ContributionToken> = Map::new("contributions");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    IssueContribution { recipient: String, amount: Uint128 },
    TransferContribution { recipient: String, amount: Uint128 },
}

// Main execution function to issue contribution tokens
pub fn execute_issue_contribution(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> StdResult<Response> {
    // Load current state from storage
    let state = STATE.load(deps.storage)?;

    // Check if the sender is the admin
    if info.sender != state.admin {
        return Err(cosmwasm_std::StdError::generic_err("Unauthorized"));
    }

    // Convert recipient string to Addr using unchecked (clone recipient to avoid moving)
    let recipient_addr = Addr::unchecked(recipient.clone());

    // Load the recipient's contribution token from storage, or create a new one if it doesn't exist
    let mut contribution = CONTRIBUTIONS.load(deps.storage, &recipient_addr).unwrap_or_else(|_| ContributionToken {
        owner: recipient_addr.clone(),
        amount: Uint128::zero(),
    });

    // Update the recipient's contribution
    contribution.amount += amount;

    // Save the updated contribution back into storage
    CONTRIBUTIONS.save(deps.storage, &recipient_addr, &contribution)?;

    // Update the total contributions in the state
    let mut state = state;
    state.total_contributions += amount;
    STATE.save(deps.storage, &state)?;

    // Return a successful response
    Ok(Response::new()
        .add_attribute("action", "issue_contribution")
        .add_attribute("recipient", recipient)  // Aquí puedes usar `recipient` directamente
        .add_attribute("amount", amount.to_string()))
}
