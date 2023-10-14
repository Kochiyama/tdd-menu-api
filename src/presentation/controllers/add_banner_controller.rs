use crate::business::models::banner::Banner;
use crate::business::use_cases::add_banner::AddBanner;
use crate::presentation::protocols::{controller::Controller, http::HttpResponse};

#[derive(Debug)]
struct AddBannerDto {
    file_name: String,
}

pub struct AddBannerController<'a> {
    add_banner: &'a dyn AddBanner,
}

impl<'a> AddBannerController<'a> {
    fn new(add_banner: &'a dyn AddBanner) -> Self {
        Self { add_banner }
    }
}

impl<'a> Controller<AddBannerDto, Banner> for AddBannerController<'a> {
    fn handle(&self, request: AddBannerDto) -> HttpResponse<Banner> {
        println!("request: {:#?}", request);
        match self.add_banner.execute(request.file_name) {
            Ok(banner) => HttpResponse::success(banner),
            Err(message) => HttpResponse::bad_request(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::business::use_cases::add_banner::MockAddBanner;
    use mockall::predicate::*;

    use super::*;

    #[test]
    fn should_call_add_banner_with_correct_values() {
        let dto = AddBannerDto {
            file_name: String::from("test_file.txt"),
        };

        let mut add_banner_mock = MockAddBanner::new();

        add_banner_mock
            .expect_execute()
            .with(eq(dto.file_name.clone()))
            .times(1)
            .returning(|file_name| Ok(Banner::new(String::from("fake_uuid"), file_name)));

        let sut = AddBannerController::new(&add_banner_mock);

        sut.handle(dto);
    }

    #[test]
    fn should_return_banner_on_success() {
        let dto = AddBannerDto {
            file_name: String::from("test_file.txt"),
        };

        let mut add_banner_mock = MockAddBanner::new();

        add_banner_mock
            .expect_execute()
            .with(eq(dto.file_name.clone()))
            .times(1)
            .returning(|file_name| Ok(Banner::new(String::from("fake_uuid"), file_name)));

        let sut = AddBannerController::new(&add_banner_mock);

        let result = sut.handle(dto);

        assert_eq!(
            result,
            HttpResponse::success(Banner::new(
                String::from("fake_uuid"),
                String::from("test_file.txt"),
            ))
        )
    }

    #[test]
    fn should_return_bad_request_with_correct_error_message() {
        let dto = AddBannerDto {
            file_name: String::from("test_file.txt"),
        };

        let mut add_banner_mock = MockAddBanner::new();

        add_banner_mock
            .expect_execute()
            .with(eq(dto.file_name.clone()))
            .times(1)
            .returning(|file_name| Err(String::from("sample error message")));

        let sut = AddBannerController::new(&add_banner_mock);

        let result = sut.handle(dto);

        assert_eq!(
            result,
            HttpResponse::bad_request(String::from("sample error message"))
        )
    }
}
