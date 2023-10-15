use crate::presentation::protocols::http::HttpResponse;

use super::http::HttpRequest;

pub trait Controller<ReqBody, ReqQuery, ReqParams, Res> {
    fn handle(&self, request: HttpRequest<ReqBody, ReqQuery, ReqParams>) -> HttpResponse<Res>;
}
