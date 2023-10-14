use crate::business::models::banner::Banner;

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait AddBanner {
    fn execute(&self, filename: String) -> Result<Banner, String>;
}
