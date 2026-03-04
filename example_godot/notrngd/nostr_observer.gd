extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "En un sistema de Nostr con Gift Wraps (NIP-59), un observador es una tercera llave pública que recibe una copia del regalo pero no tiene permisos de escritura en la conversación.

Como el sistema se basa en criptografía, para que esto funcione sin que el servidor se entere, los dos que hablan tienen que generar un tercer sobre dirigido a la llave del observador.

El observador descarga el Gift Wrap del relay y usa su propia llave privada para abrir el envoltorio. Adentro encuentra el sello (Seal) y el mensaje (Rumor). Puede leer todo el texto plano porque los emisores cifraron una copia específicamente para él, pero si el observador intenta enviar un mensaje, los clientes de los otros dos lo ignorarán porque su firma no pertenece al grupo de chat original.

Es como una invitación de solo lectura donde el observador tiene la llave del buzón pero no tiene el sello oficial para firmar cartas nuevas que los demás acepten como válidas"
	pass # Replace with function body.


func _on_mouse_exited() -> void:
	#$"../../TextEdit".text = ""
	pass # Replace with function body.
