extends Control

@onready var JI = $"../jlang-rs-gd" # J interpreter
@onready var output:RichTextLabel = $output
@onready var input:LineEdit = $input

func _ready():
	print("J_HOME:", OS.get_environment('J_HOME'))
	output.scroll_following = true


func _on_input_text_submitted(s):
	var res = JI.cmd_s(s)
	output.text += '[color=#77ff33]   '+s+'[/color]\n'
	output.text += res + '\n'
	input.clear()
