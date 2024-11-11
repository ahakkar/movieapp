#![allow(dead_code)] 
mod constants;
mod db;
pub mod view;
pub mod app;
use db::connection::establish_connection;