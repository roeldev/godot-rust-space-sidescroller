extends Button

func _on_StartButton_pressed():
	if get_tree().change_scene("res://World.tscn") != OK:
		print("Failed to change to World scene")
