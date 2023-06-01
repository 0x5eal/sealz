use tide::{Response, Body};

pub struct ResponseOptions<T>
where
    T: Into<Body>,
{
    pub status: u16,
    pub content_type: &'static str,
    pub contents: T,
}

pub fn build_response<T>(opts: ResponseOptions<T>) -> Response
where
    T: Into<Body>,
{
    let mut resp = Response::new(opts.status);

    resp.append_header("Content-Type", opts.content_type);
    resp.set_body(opts.contents);

    return resp;
}