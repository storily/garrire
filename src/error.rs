#![allow(deprecated)]

error_chain::error_chain! {
    foreign_links {
        Discord(serenity::Error);
        Json(serde_json::Error);
        Http(http::Error);
        Curl(isahc::Error);
        Base64(base64::DecodeError);
        Utf8(std::string::FromUtf8Error);
        DatabasePool(r2d2::Error);
        Database(diesel::result::Error);
    }

    errors {
        Unreachable {
            description("unreachable")
            display("this is a bug / logic error: author declared this unreachable")
        }
        Nano(error: crate::nanowrimo::models::NanoError) {
            description("generic nanowrimo error")
            display("nanowrimo says: {:?}", error)
        }
        NanoUnauthorised(url: String) {
            description("authorisation missing or invalid")
            display("request to {} has invalid authorisation", url)
        }
        NanoUnexpected(kind: &'static str, data: crate::nanowrimo::models::Data) {
            description("unexpected nanowrimo data type")
            display("expected: {}, got: {:?}", kind, data)
        }
        NanoCreds(user: String) {
            description("invalid nanowrimo credentials")
            display("invalid nanowrimo credential for username '{}'", user)
        }
        NanoTokenFormat {
            description("invalid nano token format")
            display("nano token format is unexpected")
        }
    }
}

pub fn unreachable_err() -> Error {
    ErrorKind::Unreachable.into()
}

pub fn nano_unexpected<T>(data: crate::nanowrimo::models::Data) -> Error
where
    T: ?Sized,
{
    ErrorKind::NanoUnexpected(std::any::type_name::<T>(), data).into()
}
