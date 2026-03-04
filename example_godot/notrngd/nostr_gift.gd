extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "Nostr es un protocolo de notas y mensajes donde no hay un servidor central. Todo se basa en llaves criptograficas. Tu identidad es una llave publica y tu acceso es una llave privada. Los mensajes se envian a traves de relays que son servidores que solo retransmiten la informacion sin poder borrar tu cuenta.

El concepto de Gift en Nostr se refiere al NIP-59 que es el Gift Wrap o envoltorio de regalo. No tiene nada que ver con archivos de imagen gif. Se llama asi porque es un sistema de triple capa para proteger la privacidad.

Primero se crea un mensaje llamado Rumor que contiene el texto plano. Ese mensaje se cifra para el receptor. Despues ese mensaje cifrado se mete adentro de otro evento llamado Seal o sello que oculta quien es el emisor real. Finalmente todo eso se mete adentro de un Gift Wrap que es lo que ve el relay.

El relay solo ve un paquete dirigido a alguien pero no sabe quien lo mando originalmente porque el remitente externo es una llave aleatoria descartable. Esto sirve para ocultar los metadatos y que nadie pueda rastrear quien habla con quien mirando el trafico del servidor. Solo el receptor tiene la llave para desenvolver el regalo y leer el contenido original"
	pass # Replace with function body.


func _on_mouse_exited() -> void:
	#$"../../TextEdit".text = ""
	pass # Replace with function body.
