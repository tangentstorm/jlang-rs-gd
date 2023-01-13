extends Control

onready var JI = $"../jlang-rs-gd" # J interpreter
onready var output = $output

func _ready():
	print("J_HOME:", OS.get_environment('J_HOME'))
	output.scroll_following = true

func _on_input_text_entered(s):
	var res = JI.cmd_s(s)
	output.bbcode_text += '[color=#77ff33]   '+s+'[/color]\n'
	output.bbcode_text += res + '\n'
