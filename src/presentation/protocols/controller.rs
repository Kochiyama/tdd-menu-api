use crate::presentation::protocols::http::HttpResponse;

pub trait Controller<Req, Res> {
    fn handle(request: Req) -> HttpResponse<Res>;
}
