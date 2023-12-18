use lazy_static::lazy_static;
use crate::DOTENV_FILEPATH;

lazy_static! {
    pub static ref OPENAI_API_KEY: String = {
        let _ = DOTENV_FILEPATH.as_ref();
        dotenv::var("OPENAI_API_KEY").unwrap()
    };
}
