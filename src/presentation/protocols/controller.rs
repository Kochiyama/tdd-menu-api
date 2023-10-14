use crate::presentation::protocols::http::HttpResponse;

pub trait Controller<Req, Res> {
    fn handle(&self, request: Req) -> HttpResponse<Res>;
}
