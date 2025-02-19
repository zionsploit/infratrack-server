#[derive(Debug)]
#[allow(unused)]
pub enum ResponseOkMessage {
    TotalAffectedRows,
    NewAccountCreated,
    TokenIsValid,
    NewDataCreated,
    NewDataUpdate,
    DataDeleteSuccess
}

#[derive(Debug)]
#[allow(unused)]
pub enum ResponseErrorMessage {
    ReadingPasswordKeyNotFound,
    PasswordIsNotSame,
    UsernameIsExists,
    ExecutingQueryError,
    AccountNotFound,
    ErrorGeneratingToken,
    TokenIsNotValid,
    AuthorizationRequired,
    AlreadyExists,
    DataNotModified,
    RequestNotAcceptable,
    DataNotDelete,
    DataNotFound,
    InvalidRequest
}

#[derive(Debug)]
pub enum VerifiedToken {
    TokenIsNotValid,
    TokenIsValid
}