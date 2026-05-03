use clap::builder::TypedValueParser;

#[derive(Clone)]
pub struct MinLengthParser {
    pub len: usize,
}

impl TypedValueParser for MinLengthParser {
    type Value = String;

    fn parse_ref(
        &self,
        _cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let s = value
            .to_str()
            .ok_or_else(|| clap::Error::new(clap::error::ErrorKind::InvalidUtf8))?;

        if s.len() >= self.len {
            Ok(s.to_string())
        } else {
            Err(clap::Error::raw(
                clap::error::ErrorKind::ValueValidation,
                format!("must be at least {} characters\n", self.len),
            ))
        }
    }
}
