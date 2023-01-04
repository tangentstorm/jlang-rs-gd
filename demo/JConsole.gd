extends Control

onready var JI = $JLang # J interpreter
onready var output = $panel/vbox/output

func _ready():
	print("J_HOME:", OS.get_environment('J_HOME'))
	JI.cmd("ARGV_z_=:''")
	JI.cmd("load 'tangentstorm/j-kvm/vid'")
	JI.cmd("coinsert 'kvm'")
	JI.cmd("1!:44 'd:/ver/jprez'")
	JI.cmd("load 'repl.ijs'")
	JI.cmd("init_world_''")
	# --
	JI.cmd("repl =: 'UiRepl' conew~ ''")
	JI.cmd("B__ed__repl =: '+/\\p: i. 10 NB. running sum of first 10 primes'")
	JI.cmd("accept__repl''")
	JI.cmd("H__repl =: 25 [ W__repl =: 80")
	JI.cmd("C__repl =: '' conew 'vid' ")
	refresh_console()
	$GsConsole.grab_focus()
	output.scroll_following = true

func to_colors(ints):
	var res = PoolColorArray()
	for i in ints:
		if i < 0: res.append($GsConsole.pal[-i])
		else: res.append(Color(i*0x100+0xFF))
	return res

func _on_input_text_entered(s):
	var res = JI.cmd_s(s)
	output.bbcode_text += '[color=#77ff33]   '+s+'[/color]\n'
	output.bbcode_text += res + '\n'

func refresh_console():
	JI.cmd("sethw__C__repl 25 80")
	# JI.cmd("rnd__C__repl'.'") # draw some junk to make sure it's working
	JI.cmd("pushterm C__repl")
	JI.cmd("render__repl 1")
	JI.cmd("popterm''")
	$GsConsole.cursor_visible = false
	$GsConsole.CHB = JI.cmd("a.i.,CHB__C__repl")
	$GsConsole.FGB = to_colors(JI.cmd(",FGB__C__repl"))
	$GsConsole.BGB = to_colors(JI.cmd(",BGB__C__repl"))
	$GsConsole.update()

func _on_GsConsole_keypress(code, ch, fns):
	print('keypress('+str({'code':code, 'ch':ch, 'fns':fns})+')')
	var s = ""
	for fn in fns: s += "'"+fn+"';"
	JI.cmd("fn =: > {. (#~ 3 = 4!:0) ,&'__ed__repl' L:0 ("+s+";'k_any')")
	JI.cmd("(fn~)'"+ch+"'")
	refresh_console()
