use crate::services::menu::parser;

#[derive(Debug)]
pub enum Error<E: parser::Error> {
    GtkInit(glib::BoolError),
    ParseError(E),
    DefaultScreen,
    DefaultDisplay,
    MonitorNotFound,
    SwayIPCError(String)
}

impl<E: parser::Error> From<glib::BoolError> for Error<E> {
    fn from(e: glib::BoolError) -> Self {
        Self::GtkInit(e)
    }
}

impl<E: parser::Error> From<E> for Error<E> {
    fn from(e: E) -> Self {
        Self::ParseError(e)
    }
}

impl<E: parser::Error> From<failure::Error> for Error<E> {
    fn from(e: failure::Error) -> Self {
        Self::SwayIPCError(e.to_string())
    }
}

impl<E: parser::Error> From<String> for Error<E> {
    fn from(e: String) -> Self {
        Self::SwayIPCError(e)
    }
}
