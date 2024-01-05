extends Control

@onready var v_box_container = $VBoxContainer
@onready var v_box_container_2 = $VBoxContainer2
@onready var v_box_container_3 = $VBoxContainer3
@onready var select = $Select


func _on_button_pressed():
	v_box_container.visible = false
	v_box_container_2.visible = true
	select.play()

func _on_button_2_pressed():
	select.play()
	await select.finished
	get_tree().quit()

func _on_pvp_pressed():
	select.play()
	await select.finished
	get_tree().change_scene_to_file("res://scenes/arena.tscn")
	
func _on_pve_pressed():
	v_box_container_2.visible = false
	v_box_container_3.visible = true
	select.play()

func _on_eas_pressed():
	select.play()
	await select.finished
	get_tree().change_scene_to_file("res://scenes/arena_pvp_easy.tscn")

func _on_nor_pressed():
	select.play()
	await select.finished
	get_tree().change_scene_to_file("res://scenes/arenaPVE.tscn")

func _on_hard_pressed():
	select.play()
	await select.finished
	get_tree().change_scene_to_file("res://scenes/arena_pvp_hard.tscn")


func _on_view_pressed():
	select.play()
	$VBoxContainer.visible = false
	$VBoxContainer2.visible = false
	$VBoxContainer3.visible = false
	$Title.visible = false
	$Transp.visible = false
	$View.visible = false

func _input(event: InputEvent):
	if event.is_action_pressed("sair"):
		get_tree().reload_current_scene()
