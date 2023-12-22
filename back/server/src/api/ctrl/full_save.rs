use crate::security::Action;
use crate::api::header_use::*;
use crate::utility::save_load::Save;
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

    let hash_expected = hex::decode(&pwd_hash)
        .map_err(|_|Error::new_detailed_x(E::BadHash, pwd_hash))?;

    {
        let mut x = state.write().map_err(|_|E::StateAccess(1))?;

        let ok = x.secure.use_pwd(Action::FullSave, &save_name, &hash_expected)
            .map_err(|e|E::SecureInner.detailed(e))?;
        if !ok { return Err(E::SecureInvalid.into()) }

        let x = x.state.read().map_err(|_|E::StateAccess(2))?;
        let mut args = super::init_args(save_name, single_file);
        x.save(&mut args).map_err(|e|E::Internal.detailed(e))?;
    }

    Ok(())
}

pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/full_save", 
        routing::post(handler).with_state(Arc::clone(state))
    )
}