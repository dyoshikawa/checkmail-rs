use crate::Error::{BadFormat, Unexpected};
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid format: {0}")]
    BadFormat(String),
    #[error("unexpected error ({0})")]
    Unexpected(String),
}

pub fn validate_format(email: impl Into<String>) -> Result<(), Error> {
    let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
        .map_err(|e| Unexpected(e.to_string()))?;
    let email_string = email.into();
    let ok = re.is_match(&email_string);
    if !ok {
        return Err(BadFormat(email_string));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Sample {
        mail: String,
        format: bool,
        account: bool,
    }

    fn setup() -> Vec<Sample> {
        vec![
            Sample {
                mail: "florian@carrere.cc".to_string(),
                format: true,
                account: true,
            },
            Sample {
                mail: "support@g2mail.com".to_string(),
                format: true,
                account: false,
            },
            Sample {
                mail: "florian@carrere.cc ".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "test@912-wrong-domain902.com".to_string(),
                format: true,
                account: false,
            },
            Sample {
                mail: "0932910-qsdcqozuioqkdmqpeidj8793@gmail.com".to_string(),
                format: true,
                account: false,
            },
            Sample {
                mail: " florian@carrere.cc".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "@gmail.com".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "test@gmail@gmail.com".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "test test@gmail.com".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: " test@gmail.com".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "test@wrong domain.com".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "é&ààà@gmail.com".to_string(),
                format: false,
                account: false,
            },
            Sample {
                mail: "admin@busyboo.com".to_string(),
                format: true,
                account: false,
            },
            Sample {
                mail: "a@gmail.fi".to_string(),
                format: true,
                account: false,
            },
        ]
    }

    #[test]
    fn test_validate_format() {
        let samples = setup();
        for s in samples.iter() {
            let res = validate_format(&s.mail);
            if s.format {
                res.expect(format!("expected valid mail: {}", &s.mail).as_str());
            } else {
                match res {
                    Ok(_) => {
                        panic!(format!("expected invalid mail: {}", &s.mail));
                    }
                    Err(e) => {
                        assert_eq!(e.to_string(), format!("invalid format: {}", &s.mail));
                    }
                }
            }
        }
    }
}
