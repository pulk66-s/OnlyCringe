use crate::data::{Friend, Role, User};
use crate::service;
use mysql::{from_row, Result, Row};
use rocket::serde::uuid::Uuid;
use std::collections::HashMap;
use OC_utils::jwt::Token;
use OC_utils::{database, encrypt, jwt, api};
use rocket::serde::json::Json;


