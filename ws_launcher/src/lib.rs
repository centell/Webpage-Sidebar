#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

extern crate ws_model;

mod components;
mod pages;

pub type MainModel = pages::sitelistpage::SiteListPage;
