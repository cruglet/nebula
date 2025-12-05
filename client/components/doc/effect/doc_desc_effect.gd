class_name RichDocDescEffect
extends RichTextEffect


var bbcode: String = "desc"


func _process_custom_fx(char_fx: CharFXTransform) -> bool:
	char_fx.color.a *= float(char_fx.env.get("opacity", 1.0))
	
	return true
