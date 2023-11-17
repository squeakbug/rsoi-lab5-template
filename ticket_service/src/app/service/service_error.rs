use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum ServiceError {
    #[allow(unused)]
    #[display(fmt = "not found error")]
    NotFoundError,

    #[allow(unused)]
    #[display(fmt = "bad request")]
    BadClientData,

    #[allow(unused)]
    #[display(fmt = "timeout")]
    Timeout,

    #[display(fmt = "not implemented")]
    #[allow(unused)]
    NotImplemented,

    #[allow(unused)]
    #[display(fmt = "not implemented")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, ServiceError>;
