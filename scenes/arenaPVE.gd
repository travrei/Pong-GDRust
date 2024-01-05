extends Node2D

func _input(event: InputEvent):
	if (event.is_action_pressed("sair")):
		get_tree().change_scene_to_file("res://scenes/menu.tscn")
