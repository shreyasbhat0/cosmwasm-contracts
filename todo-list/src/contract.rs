#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{TodoList, TODOLIST};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:todo-list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = TodoList::init(
        info.sender.clone(),
        msg.name.clone(),
        msg.descripton.clone(),
    );
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    TODOLIST.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender.clone())
        .add_attribute(
            "task",
            format!("{:?}", (msg.name.clone(), msg.descripton.clone())),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddTask { name, description } => {
            execute::add_task(deps, info, name, description)
        }
        ExecuteMsg::UpdateTask { name, description } => {
            execute::update_task(deps, info, name, description)
        }
        ExecuteMsg::DeleteTask { name } => execute::delete_task(deps, info, name),
        ExecuteMsg::Reset {} => execute::reset(deps, info),
        ExecuteMsg::Completed { name, completed } => {
            execute::mark_task_completed(deps, info, name, completed)
        }
    }
}

pub mod execute {

    use super::*;

    pub fn add_task(
        deps: DepsMut,
        info: MessageInfo,
        name: String,
        description: String,
    ) -> Result<Response, ContractError> {
        TODOLIST.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.add_task(name, description);
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "add_task"))
    }

    pub fn delete_task(
        deps: DepsMut,
        info: MessageInfo,
        name: String,
    ) -> Result<Response, ContractError> {
        TODOLIST.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.delete_task(name);
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "delete_task"))
    }

    pub fn update_task(
        deps: DepsMut,
        info: MessageInfo,
        name: String,
        description: String,
    ) -> Result<Response, ContractError> {
        TODOLIST.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.update_task(name, description);
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "update_task"))
    }

    pub fn reset(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        TODOLIST.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.reset();
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "reset"))
    }
    pub fn mark_task_completed(
        deps: DepsMut,
        info: MessageInfo,
        name: String,
        completed: bool,
    ) -> Result<Response, ContractError> {
        TODOLIST.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.completed_task(name, completed);
            Ok(state)
        })?;
        Ok(Response::new().add_attribute("action", "update_task"))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTasks {} => to_binary(&query::tasks(deps)?),
    }
}

pub mod query {
    use crate::{msg::TaskListResponse, state::Task};

    use super::*;

    pub fn tasks(deps: Deps) -> StdResult<TaskListResponse> {
        let state = TODOLIST.load(deps.storage)?;
        let mut result: TaskListResponse = TaskListResponse { tasks: vec![] };
        state.tasks.into_iter().for_each(|task| {
            result.tasks.push(Task::new(
                task.name().to_string(),
                task.description().to_string(),
            ))
        });
        Ok(result)
    }
}

#[cfg(test)]
mod tests {

    use crate::msg::TaskListResponse;
    use crate::state::Task;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            name: "Task1".to_string(),
            descripton: "some des".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetTasks {}).unwrap();
        let value: TaskListResponse = from_binary(&res).unwrap();

        let mut actual = TaskListResponse { tasks: vec![] };
        actual
            .tasks
            .push(Task::new("Task1".to_string(), "some des".to_string()));
        assert_eq!(actual, value);
    }

    #[test]
    fn add_tasks() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            name: "Task1".to_string(),
            descripton: "some des".to_string(),
        };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::AddTask {
            name: "task2".to_string(),
            description: "desc".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetTasks {}).unwrap();
        let value: TaskListResponse = from_binary(&res).unwrap();

        let mut actual = TaskListResponse { tasks: vec![] };
        actual
            .tasks
            .push(Task::new("Task1".to_string(), "some des".to_string()));
        actual
            .tasks
            .push(Task::new("task2".to_string(), "desc".to_string()));

        assert_eq!(actual, value);
    }
}
