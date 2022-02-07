use std::sync::Once;

use curl::easy::Easy;

use crate::page_node::RequestType;

static CURL_INIT_ONCE: Once = Once::new();

#[derive(Debug, Default)]
pub struct Response {
    pub code: u32,
    pub body: String,
}

pub fn request(uri: &str, _request_type: RequestType) -> Result<Response, curl::Error> {
    CURL_INIT_ONCE.call_once(|| curl::init());

    let mut easy = Easy::new();
    easy.signal(false)?;
    easy.url(uri)?;

    let mut body = vec![];
    let mut transfer = easy.transfer();

    match _request_type {
        RequestType::Get => transfer.write_function(|data| {
            body.extend(data);
            Ok(data.len())
        })?,
        RequestType::Head => {}
    }
    transfer.perform()?;

    drop(transfer);

    let code = easy.response_code()?;
    let body = String::from_utf8_lossy(&body).into_owned();

    Response { code, body }.into_ok()
}

trait IntoResult {
    fn into_ok<E>(self) -> Result<Self, E>
    where
        Self: Sized,
    {
        Ok(self)
    }

    fn into_err<T>(self) -> Result<T, Self>
    where
        Self: Sized,
    {
        Err(self)
    }
}

impl<T> IntoResult for T {}

#[cfg(test)]
mod test {
    use crate::page_node;

    #[test]
    fn giberish() {
        let url = "https://www.ghinfliea.com/";

        let response = super::request(url, page_node::RequestType::Get);

        let response = match response {
            Ok(response) => response,
            Err(err) => {
                if err.is_http_returned_error() {
                    panic!("is http error");
                }
                panic!("{:?}", err);
            }
        };

        println!("{}", response.code);

        println!("{}", &response.body[..100]);
    }
}
