use crate::security::Action;
use crate::api::header_use::*;
use crate::utility::save_load::Load;
use super::State as StateX;

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
) -> anyhow::Result<Json<bool>, ()> {
    let save_name = params.save_name;
    let single_file = params.single_file;
    let pwd_hash = params.pwd_hash;

    {
        let mut x = state.write().map_err(|_|())?;
        let ok = x.secure.use_pwd(Action::FullLoad, &save_name, &pwd_hash).map_err(|_|())?;
        if !ok { return Err(()) }

        let mut state = x.state.write().map_err(|_|())?;
        let mut args = super::init_args(save_name, single_file);
        let loaded_state = crate::app_state::CommonInfoState::load(&mut args).map_err(|_|())?;
        *state = loaded_state;
    }

    Ok(Json(true))
}
pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/full_load", 
        routing::post(handler).with_state(Arc::clone(state))
    )
}