use crate::business::models::banner::Banner;
use crate::presentation::protocols::{controller::Controller, http::HttpResponse};

struct AddBannerDto {
    file_name: String,
}

pub struct AddBannerController {}

impl Controller<AddBannerDto, Banner> for AddBannerController {
    fn handle(_request: AddBannerDto) -> HttpResponse<Banner> {
        HttpResponse::internal_error("Controller not implemented!")
    }
}
