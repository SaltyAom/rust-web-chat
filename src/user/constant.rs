use crate::models::api::APIStaticResult;

pub struct ValidationError;

impl ValidationError {
    pub const TOO_SHORT: APIStaticResult = SIGN_TOO_SHORT;
    pub const TOO_LONG: APIStaticResult = SIGN_TOO_LONG;
}

pub const SIGN_TOO_SHORT: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Username and Password need to be atleast 5 characters",
};

pub const SIGN_TOO_LONG: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Username and Password need to be atleast 5 characters",
};

pub struct Sign;

impl Sign {
    pub const SUCCESS: APIStaticResult = SIGN_IN;
    pub const SIGNED: APIStaticResult = ALREADY_SIGNED_IN;
    pub const OUT: APIStaticResult = SIGN_OUT;
    pub const INCORRECT: APIStaticResult = INCORRECT_SIGN;
    pub const EXISTED: APIStaticResult = USER_EXISTED;
    pub const UNAUTHORIZED: APIStaticResult = UNAUTHORIZED;
}

pub const SIGN_IN: APIStaticResult = APIStaticResult {
    success: true,
    detail: "Logged in",
};

pub const INCORRECT_SIGN: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Username or Password is incorrect",
};

pub const SIGN_OUT: APIStaticResult = APIStaticResult {
    success: true,
    detail: "Sign out",
};

pub const USER_EXISTED: APIStaticResult = APIStaticResult {
    success: false,
    detail: "This user is already existed",
};

pub const ALREADY_SIGNED_IN: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Already sign in",
};

pub const UNAUTHORIZED: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Unauthorized",
};

pub struct JWTError;

impl JWTError {
    pub const CREATION: APIStaticResult = JWT_CREATION_ERROR;
    pub const INVALID: APIStaticResult = JWT_INVALID;
    pub const EXPIRED: APIStaticResult = JWT_EXPIRED;
}

pub const JWT_CREATION_ERROR: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Unable to create token",
};

pub const JWT_INVALID: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Invalid Token",
};

pub const JWT_EXPIRED: APIStaticResult = APIStaticResult {
    success: false,
    detail: "Token Expired",
};
