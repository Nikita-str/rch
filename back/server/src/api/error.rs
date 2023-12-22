use std::borrow::Cow;
use crate::api::header_use::*;
pub trait ErrType {
    fn err_code(self) -> usize;
    fn err_status(self) -> StatusCode;
    fn err_msg(self) -> Cow<'static, str>;

    fn detailed(self, e: impl ToString) -> ApiError<Self>
    where Self: Sized {
        ApiError::new_detailed(self, e)
    }
}

#[derive(Clone)]
pub struct ApiError<E: ErrType> {
    ty: E,
    detailed: Option<String>,
}
impl<E: ErrType> ApiError<E> {
    pub fn new(ty: E) -> Self {
        Self {
            ty,
            detailed: None,
        }
    }
    pub fn new_detailed_x(ty: E, detailed: String) -> Self {
        Self {
            ty,
            detailed: Some(detailed),
        }
    }
    pub fn new_detailed<D: ToString>(ty: E, detailed: D) -> Self {
        Self::new_detailed_x(ty, detailed.to_string())
    }
}
impl<E: ErrType> From<E> for ApiError<E> {
    fn from(value: E) -> Self {
        Self::new(value)
    }
}
impl<E: ErrType + Copy> IntoResponse for ApiError<E> {
    fn into_response(self) -> Response {
        let status = self.ty.err_status();
        let code = self.ty.err_code();

        let msg = self.ty.err_msg();
        let msg = match self.detailed {
            Some(detailed) => Cow::Owned(format!("{msg}: {detailed}")),
            None => msg,
        };

        let body = Json(json!({
            "code": code,
            "msg": msg,
        }));

        (status, body).into_response()
    }
}