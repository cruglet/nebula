extends RichTextLabel


func _ready() -> void:
	#print(text)
	#print("\n--------\n")
	#print(DocParser.parse(text))
	text = DocParser.parse(text)
	#print("TEXT:", text)
