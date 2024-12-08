use cosmwasm_std::{Deps, DepsMut, MessageInfo, Response, StdResult, StdError, Coin, BankMsg};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub borrower: Option<String>,
    pub loan_amount: u128,
    pub tree_available: bool,
}

pub fn save_state(deps: DepsMut, state: &State) -> StdResult<()> {
    deps.storage.set(b"state", &cosmwasm_std::to_json_binary(state)?);
    Ok(())
}

pub fn load_state(deps: Deps) -> StdResult<State> {
    let state_bytes = deps.storage.get(b"state").ok_or_else(|| {
        StdError::not_found("State not found in storage")
    })?;
    
    let state: State = cosmwasm_std::from_json(state_bytes)?;
    Ok(state)
}

pub fn borrow_tree(deps: DepsMut, info: MessageInfo, loan_amount: u128) -> StdResult<Response> {
    let mut state = load_state(deps.as_ref())?;

    if !state.tree_available {
        return Err(StdError::generic_err("The tree is not available for borrowing"));
    }

    if info.funds.iter().any(|coin| coin.denom == "token" && coin.amount.u128() >= loan_amount) {
        state.borrower = Some(info.sender.to_string());
        state.loan_amount = loan_amount;
        state.tree_available = false;
        save_state(deps, &state)?;

        Ok(Response::new()
            .add_attribute("action", "borrow_tree")
            .add_attribute("borrower", info.sender)
            .add_attribute("loan_amount", loan_amount.to_string()))
    } else {
        Err(StdError::generic_err(format!("Insufficient funds. Expected at least {} tokens", loan_amount)))
    }
}

pub fn return_tree(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    let mut state = load_state(deps.as_ref())?;

    if state.borrower.as_deref() != Some(info.sender.as_str()) {
        return Err(StdError::generic_err("You are not the borrower of the tree"));
    }

    if !state.tree_available {
        let refund_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: vec![Coin::new(state.loan_amount, "token")],
        };

        state.borrower = None;
        state.loan_amount = 0;
        state.tree_available = true;
        save_state(deps, &state)?;
        
        Ok(Response::new()
            .add_attribute("action", "return_tree")
            .add_attribute("refund_amount", state.loan_amount.to_string())
            .add_message(refund_msg))
    } else {
        Err(StdError::generic_err("The tree is not currently borrowed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_info};
    use cosmwasm_std::coins;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        // Initialize state
        let state = State {
            borrower: None,
            loan_amount: 0,
            tree_available: true,
        };
        save_state(deps.as_mut(), &state).unwrap();

        // Check that we can load it back
        let loaded_state = load_state(deps.as_ref()).unwrap();
        assert_eq!(state, loaded_state);
    }

    #[test]
    fn borrow_and_return() {
        let mut deps = mock_dependencies();

        // Initialize state
        let state = State {
            borrower: None,
            loan_amount: 0,
            tree_available: true,
        };
        save_state(deps.as_mut(), &state).unwrap();

        // Borrow the tree
        let info = mock_info("borrower", &coins(100, "token"));
        let res = borrow_tree(deps.as_mut(), info, 100).unwrap();
        assert_eq!(3, res.attributes.len());

        // Try to borrow again
        let info = mock_info("another_borrower", &coins(100, "token"));
        let res = borrow_tree(deps.as_mut(), info, 100);
        assert!(res.is_err());

        // Return the tree
        let info = mock_info("borrower", &[]);
        let res = return_tree(deps.as_mut(), info).unwrap();
        assert_eq!(2, res.attributes.len());
        assert_eq!(1, res.messages.len());

        // Check that the tree is available again
        let state = load_state(deps.as_ref()).unwrap();
        assert!(state.tree_available);
    }
}
