# https://github.com/nebulaengine/nebula/issues/66675
func test():
	example(Node2D)

func example(thing):
	print(thing.has_method('asdf'))
