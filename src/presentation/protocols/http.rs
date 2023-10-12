pub struct HttpResponse<T> {
    status_code: i32,
    body: Result<T, &'static str>,
}

impl<T> HttpResponse<T> {
    pub fn success(body: T) -> Self {
        Self {
            status_code: 200,
            body: Ok(body),
        }
    }

    pub fn bad_request(message: &'static str) -> Self {
        Self {
            status_code: 400,
            body: Err(message),
        }
    }

    pub fn internal_error(message: &'static str) -> Self {
        Self {
            status_code: 500,
            body: Err(message),
        }
    }
}
