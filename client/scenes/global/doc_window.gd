class_name NebulaDocWindow
extends NebulaWindow

@export var doc_label: RichTextLabel


func parse_docs(doctext: String) -> void:
	doc_label.text = NebulaDocParser.parse(doctext)
