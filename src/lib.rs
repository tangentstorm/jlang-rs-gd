use jlang::{ JT, JS, JContainer };
use gdnative::{prelude::*, api::script::*};


#[derive(NativeClass)] #[inherit(Node)]
pub struct JLang {
  c: JContainer,
  jt: JT
}

#[methods]
impl JLang {

  fn new(_owner: &Node) -> Self {
    let c = jlang::load();
    let jt = c.init();
    JLang { c, jt }}

  #[export] fn _ready(&self, n: &Node) {
    godot_print!("Hello from jlang-rs-gd!");
    if let Some(r) = n.get_script() {
      let s0 = r.cast::<gdnative::api::Script>().unwrap();
      let s = unsafe { s0.assume_safe() };
      godot_print!("script source code:\n\n{}", s.source_code()); }
    else { godot_print!("no script attached.")}}

  #[export] fn cmd(&self, _:&Node, s:String)->Variant {
    godot_print!("[j cmd] <- {}", s);
    let cs = std::ffi::CString::new(s).unwrap();
    let c = JS::from_ptr(cs.as_ptr());
    godot_print!("input: {}", c.to_str());
    let _rc = self.c.jdo(self.jt, c);
    let js = self.c.getr(self.jt);
    let s = js.to_str();
    godot_print!("[j res] : {}", s);
    let res = Variant::from_str(s);
    godot_print!("[j cmd] -> {:?}", res);
    res }

  #[export] fn getv(&self, _:&Node, name:String)->Variant {
    Variant::new()}
}

fn init(handle: InitHandle) {
  handle.add_class::<JLang>(); }

godot_init!(init);
