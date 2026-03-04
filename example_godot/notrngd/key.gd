extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "Nostr utiliza criptografía de clave pública basada en la curva elíptica secp256k1 y el sistema de firmas Schnorr que permite firmas más cortas y seguras que el estándar tradicional.

La generación empieza con una clave privada que es un número aleatorio de 256 bits y mediante una función matemática se deriva la clave pública siendo un proceso de una sola dirección donde es imposible calcular la privada teniendo solo la pública.

Existen dos formatos principales para estas llaves el modo HEX que es el formato crudo de números y letras usado por protocolos y relays para procesar datos y el modo texto basado en NIP-19 que es una codificación para humanos con detección de errores.

El formato de texto usa los prefijos npub para la clave pública que compartes para que te sigan y nsec para la clave privada que es tu secreto total y acceso a la cuenta siendo el estándar para el usuario final en las aplicaciones."
	pass # Replace with function body.
