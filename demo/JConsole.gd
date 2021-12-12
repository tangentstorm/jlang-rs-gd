extends Control

onready var input = $input
onready var output = $output
onready var jrg = $'../jlang-rs-gd'


func bbi(s:String):
	return '[color=#ffcc33]  '+s+'[/color]\n'
	
func bbo(s:String):
	return s

func _on_jcmd(s:String):
	var r = jrg.cmd_s(s)
	output.bbcode_text += bbi(s)
	output.bbcode_text += bbo(r)
