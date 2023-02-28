use jlang::{ JProc, JData };
use godot::prelude::*;

struct JLangLib;
#[gdextension]
unsafe impl ExtensionLibrary for JLangLib {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JLang {
  #[base] base : Base<Node>,
  jp: JProc }

#[godot_api]
impl JLang {

  /// run a j command and return a string
  #[func] fn cmd_s(&self, gs:GodotString)->GodotString {
    let s = String::from(&gs);
    GodotString::from(self.jp.cmd_s(&s.as_str())) }

  /// fetch a j variable by name (nouns only)
  #[func] fn getv(&self, gs:GodotString)->Variant {
    let s: String = String::from(&gs);
    let jv = self.jp.get_v(&s.as_str());
    return match jv.data {
      JData::Int(i) => Variant::from(i),
      JData::IntV(v) => {
        if jv.rank > 1 { Variant::from(GodotString::from("TODO: rank>1")) }
        else {
          let vi32:Vec<i32> = v.iter().map(|&x| x as i32).collect();
          TypedArray::from_iter(vi32).to_variant()
        }},
      _ => Variant::nil() }}

  /// run a j command and return the actual data as a variant.
  #[func] fn cmd(&self, gs:GodotString)->Variant {
    let s = String::from(&gs);
    self.jp.cmd_v(&s.as_str());
    self.getv("RESULT_jrs_".into()) }}

#[godot_api]
impl GodotExt for JLang {
  fn init(base: Base<Self::Base>)  -> Self {
    Self { base, jp: JProc::load() }}

  fn ready(&mut self) {
    godot_print!("hello from jlang-rs-gd!"); }}
