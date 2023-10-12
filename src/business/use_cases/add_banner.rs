use crate::business::models::banner::Banner;

pub trait AddBanner {
    fn execute(filename: String) -> Result<Banner, String>;
}
