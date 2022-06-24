#[macro_export]
macro_rules! unwrap_result_option {
    ($expression:expr) => {{
        let val = $expression;

        if val.as_ref().is_err() {
            return Err(val.err().unwrap());
        }

        if val.as_ref().unwrap().is_none() {
            return Ok(None);
        }

        val.unwrap().unwrap()
    }};
}
