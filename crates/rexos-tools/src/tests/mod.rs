use super::*;
use crate::defs::ensure_browser_url_allowed;
use crate::ops::fs::validate_relative_path;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine as _;
use std::ffi::OsString;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

mod browser;
mod compat;
mod fs;
mod media;
mod process;
mod web;
