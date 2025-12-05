extends RichTextLabel


func _ready() -> void:
	var parsed_text: String = NebulaDocParser.parse(text)
	text = parsed_text
