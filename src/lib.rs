use jlang::{ JProc, JData };
use godot::prelude::*;
use godot::engine::INode;

struct JLangExt;
#[gdextension]
unsafe impl ExtensionLibrary for JLangExt {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JLang {
  base : Base<Node>,
  jp: JProc }

#[godot_api]
impl INode for JLang {

  fn init(base: Base<Node>)  -> Self {
    godot_print!("JLang init");
    Self { base, jp: JProc::load() }}

  fn ready(&mut self) {
    godot_print!("hello from jlang-rs-gd!"); }}

#[godot_api]
impl JLang {
  /// run a j command and return a string
  #[func] fn cmd_s(&self, s:String)->String {
    String::from(self.jp.cmd_s(&s.as_str())) }

  /// fetch a j variable by name (nouns only)
  #[func] fn getv(&self, s:String)->Variant {
    let jv = self.jp.get_v(&s.as_str());
    return match jv.data {
      JData::Int(i) => Variant::from(i),
      JData::IntV(v) => {
        if jv.rank > 1 { Variant::from("TODO: rank>1") }
        else {
          let vi32:Vec<i32> = v.iter().map(|&x| x as i32).collect();
          Array::<i32>::from_iter(vi32).to_variant() }},
      _ => Variant::nil() }}

  /// run a j command and return the actual data as a variant.
  #[func] fn cmd(&self, s:String)->Variant {
    self.jp.cmd_v(&s.as_str());
    self.getv("RESULT_jrs_".into()) }}
