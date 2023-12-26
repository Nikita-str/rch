use std::borrow::Cow;
use crate::api::header_use::*;

#[derive(Clone, Copy)]
pub enum E {
    StateAccess(/* index: */ u8),
    BadHash,
    SecureInner,
    SecureInvalid,
    Internal,
}
impl ErrType for E {
    const MAX_ERR_CODE: usize = 5;
    fn err_code(&self) -> usize {
        match self {
            E::StateAccess(_) => 1,
            E::BadHash => 2,
            E::SecureInner => 3,
            E::SecureInvalid => 4,
            E::Internal => 5,
        }
    }
    fn err_status(&self) -> StatusCode {
        match self {
            E::StateAccess(_) => StatusCode::INTERNAL_SERVER_ERROR,
            E::BadHash => StatusCode::BAD_REQUEST,
            E::SecureInner => StatusCode::INTERNAL_SERVER_ERROR,
            E::SecureInvalid => StatusCode::BAD_REQUEST,
            E::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn err_msg(self) -> Cow<'static, str> {
        match self {
            E::StateAccess(index) => Cow::Owned(format!("shared state access error (i={index})")),
            E::BadHash => Cow::Borrowed("incorrect hash"),
            E::SecureInner => Cow::Borrowed("error while trying access pwd state"),
            E::SecureInvalid => Cow::Borrowed("incorrect pwd"),
            E::Internal => Cow::Borrowed("internal error while performing requested action"),
        }
    }
}
