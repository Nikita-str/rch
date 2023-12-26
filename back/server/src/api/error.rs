use std::borrow::Cow;
use crate::api::header_use::*;
pub trait ErrType {
    const MAX_ERR_CODE: usize;
    fn err_code(&self) -> usize;
    fn err_status(&self) -> StatusCode;
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
    pub fn map<E1:ErrType + From<E>>(self) -> ApiError<E1> {
        ApiError {
            ty: self.ty.into(), 
            detailed: self.detailed,
        }
    }
}
impl<E: ErrType> From<E> for ApiError<E> {
    fn from(value: E) -> Self {
        Self::new(value)
    }
}
// impl<E1: ErrType, E2: ErrType> From<ApiError<E1>> for ApiError<E2>
// where E1: Into<E2> {
//     fn from(value: ApiError<E1>) -> Self {
//         Self {
//             ty: value.ty.into(),
//             detailed: value.detailed,
//         }
//     }
// }
impl<E: ErrType> IntoResponse for ApiError<E> {
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