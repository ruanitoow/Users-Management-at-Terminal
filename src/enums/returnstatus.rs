use crate::structs::user::User;

pub enum ReturnStatus {
    SuccessUser(User),
    Success,
    Error,
    Null
}