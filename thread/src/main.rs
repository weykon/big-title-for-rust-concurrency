mod base;
use base::base;
mod second;
use second::second;
mod i_like_to_move;
use i_like_to_move::i_like_to_move;
mod oh_my_scope;
use oh_my_scope::scope_thread;
fn main() {
    base();
    second();
    i_like_to_move();
    scope_thread();
}
