use std::sync::mpsc;
use std::thread;

mod base;
use base::base;
mod oh_my;
use oh_my::oh_my;
fn main() {
    base();

}