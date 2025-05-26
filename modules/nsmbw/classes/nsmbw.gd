extends Node
class_name NSMBW

## This handles how sprites are rendered and handled ingame.
enum ZONE_THEME {
	OVERWORLD,
	UNDERGROUND,
	UNDERWATER,
	UNDERGROUND_LAVA,
	DESERT,
	BEACH,
	FOREST,
	SNOW,
	SKY,
	MOUNTAINS,
	TOWER,
	CASTLE,
	GHOST_HOUSE,
	RIVER_CAVE,
	GHOST_HOUSE_OUTSIDE,
	UNDERWATER_CAVE,
	DESERT_CAVE,
	ICE_CAVE,
	LAVA,
	FINAL_BATTLE,
	FINAL_TOWER,
	FINAL_AIRSHIP,
	W7_TOWER_INDOORS
}

## These are all of the lighting settings for NSMBW.
enum ZONE_LIGHTING {
	NORMAL,
	UNDERGROUND,
	UNDERWATER,
	LAVA
}


static var stage_folder: String:
	get():
		return Nebula.Config.Editor.default_game_path.path_join("SMN#01").path_join("Stage")

#region LEVELS
## Levels are seperated by a vector of hashmaps generated in rust. [br]
## Each index represents an area/subarea, where 0 is the primary area and each
## valid index represents the [n+1]th area. [br][br]
## For Godot, this is translated to an [Array] of [Dictionary] entries.
## However, note that the keys in these dictionaries are [b]not[/b] in order are are typically random...
## not that it really matters, since dictionaries are meant to be parsed by keys anyway. 
## [br][br]
## Let's say you wanted to access Area 2's sprites, it would look something like: [br]
## [code]level[1]["sprites"][/code] or [code]level[1].sprites[/code] [br] 
## Alternatively, you could also use the respective [Dictionary] [code]get()[/code] methods. [br]
## [br]
## The area keys are as follows: [br][br]
## [code]"options"[/code] : [Dictionary] [br]
## The general configuration for the area. [br][br]
## -     [code]"events_a"[/code] : [int] Startup events flag A. [br]
## -     [code]"events_b"[/code] : [int] Startup events flag B. [br]
## -     [code]"can_wrap"[/code] : [bool] Whether the stage wraps wraps around edges.[br]
## -     [code]"is_credits"[/code] : [bool] Flag for whether the zone is the credits.[br]
## -     [code]"start_entrance"[/code] : [int] The entrance ID that is used when first starting a level.[br]
## -     [code]"time_limit"[/code] : [int] The time limit when starting the level.[br]
## [br]
## [code]"tilesets"[/code] : [Array][[String]] [br]
## Always has [u]4[/u] [String] elements, whether they are empty strings or valid strings.
## Each string represents the currently selected tileset(s). The index of the string also
## represents the [i]type[/i]/index of the tileset, since you can only load 4 different tilesets at once.
## Usually, the first tileset (index 0) is the special objects tileset (? blocks, bricks, pipes, etc.)[br]
## An example of a valid tileset array could be: ["Pa0_jyotyu", "Pa1_chika", "Pa2_doukutu", ""][br]
## [br]
## [code]"entrances"[/code] : [Array][[Dictionary]] [br]
## Defines all entrances/warps for the area. 
## Each entrance is described as a dictionary with the following keys: [br][br]
## -     [code]"id"[/code] : [int] Unique ID for the entrance.[br]
## -     [code]"type"[/code] : [int] Indicates the type of entrance (e.g., pipe, door).[br]
## -     [code]"path"[/code] : [int] Path ID, linking the entrance to a specific travel route.[br]
## -     [code]"pos_x"[/code] : [int] X Position coordinate.[br]
## -     [code]"pos_y"[/code] : [int] Y Position coordinate.[br]
## -     [code]"zone"[/code] : [int] Zone ID the entrance belongs to.[br]
## -     [code]"exit_to_map"[/code] : [bool] Specifies if the entrance leads to a map exit.[br]
## -     [code]"zone"[/code] : [int] Zone ID the entrance belongs to.[br]
## -     [code]"destination_area"[/code] : [int] Area ID the entrance connects to.[br]
## -     [code]"destination_entrance"[/code] : [int] ID of the entrance in the destination area.[br]
## -     [code]"layer"[/code] : [int] Layer index for the entrance.[br]
## -     [code]"connected_pipe_direction"[/code] : [int] Direction for end pipe connection.[br]
## -     [code]"enterable"[/code] : [bool] Indicates whether the entrance is accessible.[br]
## [br]
## [code]"sprites"[/code] : [Array][[Dictionary]] [br]
## The sprites array defines objects or entities in the area. 
## Each sprite has:[br][br]
## -     [code]"type"[/code] : [int] Unique ID for the sprite. [br]
## -     [code]"pos_x"[/code] : [int] X Position coordinate.[br]
## -     [code]"pos_y"[/code] : [int] Y Position coordinate.[br]
## -     [code]"data"[/code] : [Array][[int]]: Array of data values defining sprite-specific parameters.[br]
## [br]
## [code]"zones"[/code] : [Array][[Dictionary]]
## The zones section defines the properties and constraints of specific gameplay areas. 
## Each zone has its own settings, determining layout, mechanics, and interaction details:
## [br][br]
## -     [code]"size_y"[/code] : [int]
## Height of the zone in units. [br]
## -     [code]"size_x"[/code] : [int]
## Width of the zone in units. [br]
## -     [code]"pos_x"[/code] : [int]
## Horizontal starting position of the zone. [br]
## -     [code]"pos_y"[/code] : [int]
## Vertical starting position of the zone. [br]
## -     [code]"echo"[/code] : [int]
## Echo effect level applied in the zone (e.g., for sound effects). [br]
## -     [code]"theme"[/code] : [int]
## Thematic design or assets used for the zone. [br]
## -     [code]"lakitu_lower_bound"[/code] : [int]
## The minimum Y-coordinate where Lakitu can operate. [br]
## -     [code]"lakitu_upper_bound"[/code] : [int]
## The maximum Y-coordinate where Lakitu can operate. [br]
## -     [code]"fg_spotlight"[/code] : [bool]
## Whether the zone has a foreground spotlight. [br]
## -     [code]"lighting"[/code] : [int]
## Lighting configuration for the zone. [br]
## -     [code]"multiplayer_upper_bound"[/code] : [int]
## Upper screen boundary in multiplayer mode. [br]
## -     [code]"multiplayer_lower_bound"[/code] : [int]
## Lower screen boundary in multiplayer mode. [br]
## -     [code]"multiplayer_fly_screen_adjust"[/code] : [int]
## Adjustment value for flying camera behavior in multiplayer mode. [br]
## -     [code]"is_dark"[/code] : [bool]
## Whether the zone is in darkness. [br]
## -     [code]"upper_bound"[/code] : [int]
## Upper vertical boundary for general gameplay. [br]
## -     [code]"lower_bound"[/code] : [int]
## Lower vertical boundary for general gameplay. [br]
## -     [code]"music"[/code] : [int]
## Identifier for the background music used in the zone. [br]
## -     [code]"boss_room"[/code] : [bool]
## Whether the zone is a boss room. [br]
## -     [code]"id"[/code] : [int]
## Unique identifier for the zone. [br]
## [br][br]
## [code]"backgrounds"[/code] : [Array][[Dictionary]] [br]
## The backgrounds section describes the visual layers used to create depth and ambiance in a level. 
## Each background is composed of two layers: back and front. [br][br]
## -     [code]"id"[/code] : [int]
## Unique identifier for the background configuration.
## This is not stored in the front/back dictionaries [br]
## -     [code]"front"[/code] / [code]"back"[/code]: [Dictionary] [br]
##-            [code]"zoom"[/code] : [int]
##     Zoom factor for the background. [br]
##-            [code]"scroll_rate_x"[/code] : [int]
##     Horizontal scrolling speed for the background. [br]
##-            [code]"scroll_rate_y"[/code] : [int]
##     Vertical scrolling speed for the background. [br]
##-            [code]"pos_x"[/code] : [int]
##     Initial horizontal position of the background. [br]
## -           [code]"pos_y"[/code] : [int]
##     Initial vertical position of the background. [br]
## -           [code]"instance"[/code] : [int]
##     Instance ID for the background. [br]
var level: Array[Dictionary]
#endregion

func read_level(path: String) -> Dictionary:
	return {}
