use jlang::{ JProc, JData };
use gdnative::prelude::*;

#[derive(NativeClass)] #[inherit(Node)]
pub struct JLang { jp: JProc }

#[methods]
impl JLang {

  fn new(_owner: &Node) -> Self {
    let jp = JProc::load();
    JLang { jp }}

  #[export] fn _ready(&self, _n: &Node) {
    godot_print!("Hello from jlang-rs-gd!"); }

  /// run a j command and return a string variant
  #[export] fn cmd_s(&self, _:&Node, s:String)->Variant {
    Variant::from_str(self.jp.cmd_s(&s).as_str()) }

  /// fetch a j variable by name (nouns only)
  #[export] fn getv(&self, _:&Node, v:String)->Variant {
    let jv = self.jp.get_val(&v);
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
    self.jp.cmd_v(&s);
    self.getv(n, "RESULT_jrs_".to_string()) }}

fn init(handle: InitHandle) {
  handle.add_class::<JLang>(); }

godot_init!(init);
