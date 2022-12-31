#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:hello-world";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        greeting_message: format!("Hello Wolrd! {}", &msg.message),
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("greeting_message", msg.message.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Update { message } => execute::update_greeting_message(deps, info, message),
    }
}

pub mod execute {
    use super::*;

    pub fn update_greeting_message(
        deps: DepsMut,
        info: MessageInfo,
        message: String,
    ) -> Result<Response, ContractError> {
        STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }

            state.greeting_message = format!("Hello Wolrd! {}", message);
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "update_greeting_message"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetMessage {} => to_binary(&query::get_message(deps)?),
    }
}

pub mod query {
    use crate::msg::GetMessageResponse;

    use super::*;

    pub fn get_message(deps: Deps) -> StdResult<GetMessageResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetMessageResponse {
            message: state.greeting_message,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::GetMessageResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            message: format!("Hello Wolrd! Cosmwasm"),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessage {}).unwrap();
        let value: GetMessageResponse = from_binary(&res).unwrap();
        assert_eq!(format!("Hello Wolrd! Cosmwasm"), value.message);
    }

    #[test]
    fn update_message() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            message: format!("Hello Wolrd! Cosmwasm"),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Update {
            message: "Max".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetMessage {}).unwrap();
        let value: GetMessageResponse = from_binary(&res).unwrap();
        assert_eq!(format!("Hello Wolrd! Max"), value.message);
    }
}
