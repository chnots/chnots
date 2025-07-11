use std::{
    cell::RefCell,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
    rc::Rc,
    time::UNIX_EPOCH,
};

use anyhow::Context;
use chin_tools::{AResult, EResult};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use log::info;

use crate::app::ShareAppState;

use super::RowCallback;

