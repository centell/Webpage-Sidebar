#![feature(async_await, await_macro, futures_api)]

#[macro_use]
extern crate stdweb;

extern crate ws_model;

mod webextensions;

use ws_model::prelude::*;
use webextensions::*;

pub struct Storage {
    sites: Vec::<Site>
}

impl Storage {
    pub fn new() -> Self {
        Storage { sites: Vec::<Site>::new() }
    }

    pub fn bind_js(&self) {
        
    }

    async fn get_sync(&self) -> Vec::<Site> {
        let sites = await!( browser::storage::sync::get::<Vec::<Site>>("sites") );
        sites
    }

    // pub async fn update_from_sync(&self) -> Vec::<Site> {
        
    // }
}