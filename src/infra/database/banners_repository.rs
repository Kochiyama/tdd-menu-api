use crate::{
    business::models::banner::Banner,
    data::repositories::add_banner_repository::AddBannerRepository,
};

pub struct BannersRepository {}

impl AddBannerRepository for BannersRepository {
    fn create_banner(&self, file_name: String) -> Result<Banner, String> {
        Err(String::from("not implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
