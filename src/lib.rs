use gdnative::prelude::*;

#[derive(NativeClass)] #[inherit(Node)]
pub struct JLang;

#[methods]
impl JLang {

  fn new(_owner: &Node) -> Self { JLang }

  #[export] fn _ready(&self, _owner: &Node) {
    godot_print!("Hello from jlang-rs-gd!"); }}

fn init(handle: InitHandle) {
  handle.add_class::<JLang>(); }

godot_init!(init);
