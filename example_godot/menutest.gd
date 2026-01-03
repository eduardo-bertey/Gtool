extends Control





func _on_nostr_pressed() -> void:
	
	var data_exten = load("res://example_godot/notrngd/nostrn_prueba_observador.tscn").instantiate()

	self.add_child(data_exten)

	prints("⭐️ DATOS AL NODO INSTANCIADO ⭐️" )

	pass # Replace with function body.


func _on_nostr_gift_pressed() -> void:
	var data_exten = load("res://example_godot/notrngd/nostrn_gif.tscn").instantiate()

	self.add_child(data_exten)
	
	prints("⭐️ DATOS AL NODO INSTANCIADO ⭐️" )

	pass # Replace with function body.
