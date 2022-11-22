use crate::domain::entities::{TemperatureValue, TemperatureScale};
use std::convert::TryFrom;


struct Request {
    value: f32,
    scale: String,
}

enum Response {
    Ok(f32),
    BadRequest,
}

fn execute(req: Request) -> Response {
    match (
        TemperatureValue::try_from(req.value),
        TemperatureScale::try_from(req.scale),
    ) {
        (Ok(value), Ok(_)) => Response::Ok(f32::from(value)),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_temperature() {
        let value = 25.0;
        let req = Request {
            value,
            scale: "Celsius".to_string(),
        };

        let res = execute(req);
        match res {
            Response::Ok(res_number) => assert_eq!(res_number, value),
            _ => unreachable!(),
        };
    }

    #[test]
    fn it_should_return_a_bad_request_error_when_request_is_invalid() {
        let req = Request {
            value: 25.0,
            scale: "".to_string(),
        };

        let res = execute(req);

        match res {
            Response::BadRequest => {}
            _ => unreachable!(),
        };
    }
}
