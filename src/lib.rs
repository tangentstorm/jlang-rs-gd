use jlang::{ JT, JS, JProc, JData };
use gdnative::prelude::*;

#[derive(NativeClass)] #[inherit(Node)]
pub struct JLang {
  jp: JProc,
  jt: JT }

#[methods]
impl JLang {

  fn new(_owner: &Node) -> Self {
    let jp = JProc::load();
    let jt = jp.c.init();
    JLang { jp, jt }}

  fn j_cmd(&self, s:String) {
    let cs = std::ffi::CString::new(s).unwrap();
    let js = JS::from_ptr(cs.as_ptr());
    let _rc = self.jp.c.jdo(self.jt, js); }

  #[export] fn _ready(&self, _n: &Node) {
    godot_print!("Hello from jlang-rs-gd!");
    // if let Some(r) = n.get_script() {
    //   let s0 = r.cast::<gdnative::api::Script>().unwrap();
    //   let s = unsafe { s0.assume_safe() };
    //   godot_print!("script source code:\n\n{}", s.source_code()); }
    // else { godot_print!("no script attached.")}
  }

  /// run a j command and return a string variant
  #[export] fn cmd_s(&self, _:&Node, s:String)->Variant {
    self.j_cmd(s);
    let js = self.jp.c.getr(self.jt);
    Variant::from_str(js.to_str()) }

  /// fetch a j variable by name (nouns only)
  #[export] fn getv(&self, _:&Node, v:String)->Variant {
    let jv = self.jp.get_val(self.jt, &v);
    return match jv.data {
      JData::Int(i) => Variant::from_i64(i),
      JData::IntV(v) => {
        if jv.rank > 1 { Variant::from_str("TODO: rank>1") }
        else {
          let vi32:Vec<i32> = v.iter().map(|&x| x as i32).collect();
          TypedArray::from_vec(vi32).to_variant() }},
      _ => Variant::new() }}

  /// run a j command and return the actual data as a variant.
  #[export] fn cmd(&self, n:&Node, s:String)->Variant {
    let mut cmd = "RESULT_jrsgd_ =: ".to_string(); cmd += &s;
    self.j_cmd(cmd);
    self.getv(n, "RESULT_jrsgd_".to_string()) }}

fn init(handle: InitHandle) {
  handle.add_class::<JLang>(); }

godot_init!(init);
