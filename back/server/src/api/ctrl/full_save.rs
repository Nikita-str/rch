use crate::security::Action;
use crate::api::header_use::*;
use crate::utility::save_load::Save;
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
        let ok = x.secure.use_pwd(Action::FullSave, &save_name, &pwd_hash).map_err(|_|())?;
        if !ok { return Err(()) }

        let x = x.state.read().map_err(|_|())?;
        let mut args = super::init_args(save_name, single_file);
        x.save(&mut args).map_err(|_|())?;
    }

    Ok(Json(true))
}

pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/full_save", 
        routing::post(handler).with_state(Arc::clone(state))
    )
}