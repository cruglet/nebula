extends Node
class_name Subprocess

var pipe: FileAccess
var stderr: FileAccess
var pid: int
var thread: Thread
var info: String

var _binded_fn: Callable

## A string array of all of the lines read by a threaded subprocess.
var output: Array[String] = []

signal pipe_in_progress
signal close_pipe
signal line_changed(line: String)
signal binded_success(line: String)

func run(path: String, args: PackedStringArray) -> void:
	OS.execute(_modify_path(path), args)
	
func run_and_return(path: String, args: PackedStringArray) -> Array[String]:
	OS.execute(_modify_path(path), args, output)
	return output
	
func run_and_return_str(path: String, args: PackedStringArray) -> String:
	OS.execute(_modify_path(path), args, output)
	var content: String = ""
	for line: String in output:
		content += line
	return content
	


func _modify_path(path: String) -> String:
	return ProjectSettings.globalize_path("res://" + path)


func run_threaded(bin: String, args: PackedStringArray) -> void:
	var info: Dictionary = OS.execute_with_pipe(bin, args)
	pipe = info["stdio"]
	stderr=  info["stderr"]
	pid = info["pid"]
	thread = Thread.new()
	close_pipe.connect(clean_thread)
	
func start() -> void:
	thread.start(_thread_func)

## This will run the [fn] callable per line and fire the `binded_success` signal if it returns true
func bind_filter(fn: Callable) -> void:
	_binded_fn = fn

func _thread_func() -> void:
	# read stdin and report to log.
	var line: String = ""
	var pipe_err: Error
	var std_err: String
	var count: int = 0
	if !pipe.is_open():
		pipe_in_progress.emit.call_deferred("Error opening .", pipe.get_error())
	
	while pipe.is_open():
		pipe_err = pipe.get_error() 
		if pipe_err == OK:
			line=pipe.get_line()
			output.append(line)
			count += 1
			pipe_in_progress.emit.call_deferred(line)
			line_changed.emit(line)
			if _binded_fn.bind(line).call():
				binded_success.emit(line)
		else:
			line=stderr.get_line()
			if line!="":
				pipe_in_progress.emit.call_deferred(line)
			else:
				break
	pipe_in_progress.emit.call_deferred(null)

func clean_thread() -> void:
	if thread.is_alive():
		thread.wait_to_finish()
	pipe.close()
	OS.kill(pid)
