use rocket::response::Responder;

#[derive(Responder)]
pub(crate) struct SCSPErr<'r> {
    msg: &'r str,
}
