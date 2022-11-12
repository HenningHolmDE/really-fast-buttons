#[cfg(feature = "sensor")]
use crate::SensorMessage;

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum Error {
    #[cfg(feature = "sensor")]
    #[snafu(display("Failed to write item {cfg:?}"))]
    Msg { cfg: SensorMessage },
}
