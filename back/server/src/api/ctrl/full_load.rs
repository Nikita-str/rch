use crate::security::Action;
use crate::api::header_use::*;
use crate::utility::save_load::Load;
use super::State as StateX;

use super::error_type::E;
type Error = ApiError<E>;

pub const REQ_METHOD: Method = Method::POST;

#[derive(Deserialize, Debug)]
pub struct Params {
    pwd_hash: String,
    save_name: String,
    single_file: bool,
}

pub async fn handler(
    State(state): State<StateX>,
    Json(params): Json<Params>,
) -> anyhow::Result<(), Error> {
    let save_name = params.save_name;
    let single_file = params.single_file;
    let pwd_hash = params.pwd_hash;
    
    {
        let x = state.secure_verify(pwd_hash, &save_name, Action::FullLoad)?;
        let mut state = x.state.write().map_err(|_|E::StateAccess(2))?;

        let mut args = super::init_args(save_name, single_file);
        let loaded_state = crate::app_state::CommonInfoState::load(&mut args)
            .map_err(|e|E::Internal.detailed(e))?;

        *state = loaded_state;
    }

    Ok(())
}
pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/full_load", 
        routing::post(handler).with_state(state.clone())
    )
}