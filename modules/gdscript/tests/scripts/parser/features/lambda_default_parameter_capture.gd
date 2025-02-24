# https://github.com/nebulaengine/nebula/issues/56751

func test():
	var x = "local"
	var lambda = func(param = x):
		print(param)
	lambda.call()
