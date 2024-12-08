use cosmwasm_std::{entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Estructura que representa un área de reforestación
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReforestationArea {
    pub id: u32,
    pub location: String,
    pub trees_planted: u32,
}

// Enum para mensajes de ejecución
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    RegisterPlantedTree { area_id: u32 },
    AddReforestationArea { location: String },
}

// Estructura que representa el estado del contrato
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub total_trees_planted: u32,
    pub reforestation_areas: Vec<ReforestationArea>,
}

// Punto de entrada para la instanciación del contrato
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> StdResult<Response> {
    let state = State {
        total_trees_planted: 0,
        reforestation_areas: vec![],
    };
    save_state(deps, &state)?; // Guardar el estado inicial
    Ok(Response::default())
}

// Punto de entrada para ejecutar mensajes
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::RegisterPlantedTree { area_id } => register_planted_tree(deps, area_id),
        ExecuteMsg::AddReforestationArea { location } => add_reforestation_area(deps, location),
    }
}

// Función para registrar un árbol plantado en un área específica
pub fn register_planted_tree(deps: DepsMut, area_id: u32) -> StdResult<Response> {
    let mut state = load_state(deps)?;
    
    // Incrementar el número de árboles plantados en el área especificada
    if let Some(area) = state.reforestation_areas.iter_mut().find(|a| a.id == area_id) {
        area.trees_planted += 1;
        state.total_trees_planted += 1;
        save_state(deps, &state)?;
        
        Ok(Response::new()
            .add_attribute("action", "register_planted_tree")
            .add_attribute("area_id", area_id.to_string())
            .add_attribute("total_trees_planted", state.total_trees_planted.to_string()))
    } else {
        Err(cosmwasm_std::StdError::generic_err("Area not found"))
    }
}

// Función para agregar una nueva área de reforestación
pub fn add_reforestation_area(deps: DepsMut, location: String) -> StdResult<Response> {
    let mut state = load_state(deps)?;
    
    // Crear una nueva área de reforestación y agregarla al estado
    let new_area = ReforestationArea {
        id: (state.reforestation_areas.len() + 1) as u32,
        location,
        trees_planted: 0,
    };
    
    state.reforestation_areas.push(new_area);
    
    save_state(deps, &state)?;
    
    Ok(Response::new()
        .add_attribute("action", "add_reforestation_area")
        .add_attribute("location", location))
}

// Funciones auxiliares para cargar y guardar el estado
fn load_state(deps: Deps) -> StdResult<State> {
    let state_bytes = deps.storage.get(b"state").ok_or_else(|| cosmwasm_std::StdError::not_found("State not found"))?;
    let state = cosmwasm_std::from_binary(&state_bytes)?;
    Ok(state) // Asegúrate de devolver el estado cargado
}

fn save_state(deps: DepsMut, state: &State) -> StdResult<()> {
    deps.storage.set(b"state", &to_binary(state)?); 
    Ok(())
}
