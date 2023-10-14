pub struct HttpResponse<T> {
    status_code: i32,
    body: Result<T, String>,
}

impl<T> HttpResponse<T> {
    pub fn success(body: T) -> Self {
        Self {
            status_code: 200,
            body: Ok(body),
        }
    }

    pub fn bad_request<'a>(message: String) -> Self {
        Self {
            status_code: 400,
            body: Err(message),
        }
    }

    pub fn internal_error<'a>(message: String) -> Self {
        Self {
            status_code: 500,
            body: Err(message),
        }
    }
}
