use crate::business::models::banner::Banner;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait AddBannerRepository {
    fn create_banner(&self, file_name: String) -> Result<Banner, String>;
}
