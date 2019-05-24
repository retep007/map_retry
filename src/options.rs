use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Options {
    pub (crate) preserve_order: bool,
    pub (crate) num_retries: u8,
    pub(crate) min_delay: Option<Duration>,
}

impl Default for Options {
    fn default() -> Self {
        OptionsBuilder::new().finalize()
    }
}

#[derive(Default)]
pub struct OptionsBuilder {
    preserve_order: Option<bool>,
    num_retries: Option<u8>,
    min_delay: Option<Duration>,
}

impl OptionsBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn preserve_order(self, preserve_order: bool) -> OptionsBuilder {
        Self {
            preserve_order: Some(preserve_order),
            .. self
        }
    }

    pub fn num_retries(self, num_retries: u8) -> OptionsBuilder {
        Self {
            num_retries: Some(num_retries),
            .. self
        }
    }

    pub fn min_delay(self, min_delay: Duration) -> OptionsBuilder {
        Self {
            min_delay: Some(min_delay),
            .. self
        }
    }

    pub fn finalize(self) -> Options {
        Options {
            preserve_order: self.preserve_order.unwrap_or(false),
            num_retries: self.num_retries.unwrap_or(1),
            min_delay: None,
        }
    }
}