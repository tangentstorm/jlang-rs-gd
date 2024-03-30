cargo build \
  && cp target/debug/jlang_rs_gd.dll demo/ \
  && ./Godot_v4.2.1-stable_win64.exe --path demo

