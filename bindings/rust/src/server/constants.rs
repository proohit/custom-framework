pub enum Constants {
    NoRequestError,
    EmptyResponseError,
}

impl Constants {
    pub fn get(&self) -> i32 {
        match self {
            Constants::NoRequestError => -1,
            Constants::EmptyResponseError => -1,
        }
    }
}
