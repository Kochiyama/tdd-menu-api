use crate::{
    application::repositories::add_banner_repository::AddBannerRepository,
    business::{models::banner::Banner, use_cases::add_banner::AddBanner},
};

pub struct AddBannerUseCase<'a> {
    add_banner_repository: &'a dyn AddBannerRepository,
}

impl<'a> AddBannerUseCase<'a> {
    fn new(add_banner_repository: &'a dyn AddBannerRepository) -> Self {
        Self {
            add_banner_repository,
        }
    }
}

impl<'a> AddBanner for AddBannerUseCase<'a> {
    fn execute(&self, file_name: String) -> Result<Banner, String> {
        self.add_banner_repository.create_banner(file_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::repositories::add_banner_repository::MockAddBannerRepository,
        business::use_cases::add_banner::MockAddBanner,
    };
    use mockall::predicate::*;

    use super::*;

    #[test]
    fn should_call_db_add_banner_with_correct_values() {
        let mut add_banner_repository_mock = MockAddBannerRepository::new();

        add_banner_repository_mock
            .expect_create_banner()
            .with(eq(String::from("test_file.txt")))
            .times(1)
            .returning(|file_name| Ok(Banner::new(String::from("fake_uuid"), file_name)));

        let sut = AddBannerUseCase::new(&add_banner_repository_mock);

        sut.execute(String::from("test_file.txt")).unwrap();
    }
}
