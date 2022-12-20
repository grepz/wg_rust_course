// You should write simple error handler
// which takes error as anyhow::Error and format! it as string.
// if error contains Custom::One you should format as "Custom::One"
// if error contains Custom::Reason you should format as data from reason (look into tests)
// else format as is
fn error_handler(error: anyhow::Error) -> String {
    todo!()
}

#[derive(Debug, thiserror::Error)]
pub enum Custom {
    #[error("Custom::One error")]
    One,
    #[error("Custom::Reason({0}) error")]
    Reason(String),
}

fn main() {
    println!("Error!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handler() {
        let error = anyhow::Error::from(std::fs::File::open("/tmp/file_does_not_exists.txt").unwrap_err());
        assert_eq!("No such file or directory (os error 2)", &error_handler(error));

        let error = anyhow::Error::from(Custom::One);
        assert_eq!("Custom::One", &error_handler(error));

        let error = anyhow::Error::from(Custom::Reason(String::from("my reason")));
        assert_eq!("my reason", &error_handler(error));
    }
}
