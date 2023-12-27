use crate::security::Action;
use crate::api::header_use::*;
use super::State as StateX;

use super::error_type::E;
type Error = ApiError<E>;

pub const REQ_METHOD: Method = Method::POST;

#[derive(Deserialize, Debug)]
pub struct Params {
    pwd_hash: String,
}

pub async fn handler(
    State(state): State<StateX>,
    Json(params): Json<Params>,
) -> anyhow::Result<(), Error> {

    {
        let _x = state
            .secure_verify(params.pwd_hash, "#", Action::Shutdown)
            .map_err(|x|x.map())?;
    }
    let sx = crate::SHUTDOWN.get().ok_or(E::StateAccess(3))?;
    sx.send(()).map_err(|e|E::StateAccess(4).detailed(e))?;
    Ok(())
}
pub fn router(state: &StateX) -> Router {
    Router::new().route(
        "/shutdown", 
        routing::post(handler).with_state(state.clone())
    )
}
