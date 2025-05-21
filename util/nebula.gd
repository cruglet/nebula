class_name Nebula
extends Node

const VERSION: String = "0.0.0"
const BRANCH: String = "dev"

const GAME_LIST: Dictionary = {
	"SMN#01": {
		"banner": preload("uid://v23ehifiek1i"),
		"editor": "uid://ljkmme41v22c"
	}
}



## Finds the game and returns the regionless variation
## (e.g. SMNE01 -> SMN#01)
static func find_in_game_list(game_id: String) -> String:
	for pattern: String in GAME_LIST:
		var regex_pattern: String = "^" + pattern.replace("#", ".") + "$"
		var regex: RegEx = RegEx.new()
		regex.compile(regex_pattern)
		if regex.search(game_id):
			return pattern
	return ""
