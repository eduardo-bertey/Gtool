extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "Las firmas de anillo o ring signatures son una técnica criptográfica que permite a un usuario firmar un mensaje en nombre de un grupo sin revelar exactamente quién del grupo realizó la firma. En sistemas como Monero o implementaciones en Nostr como Nostringer el objetivo principal es la privacidad y la imposibilidad de rastreo.

Funciona mezclando la clave pública del firmante real con otras claves públicas elegidas al azar para formar un anillo de posibles firmantes. Al verificar la firma cualquiera puede confirmar que alguien dentro de ese grupo autorizó el mensaje pero es matemáticamente imposible determinar cuál de los miembros fue el autor original.

En el caso de Monero se utiliza para ocultar el origen de los fondos en una transacción vinculando varias entradas posibles. En protocolos sobre Nostr que usan esta técnica se busca que una identidad o un evento no pueda ser vinculado directamente a una única clave pública específica protegiendo el anonimato del usuario frente a los relays o observadores de la red.

A diferencia de una firma normal donde hay una relación uno a uno entre la clave y el mensaje aquí la relación es de uno a muchos. Esto añade una capa de negación plausible ya que el emisor siempre puede decir que cualquiera de las otras claves del anillo podría haber sido la responsable"
	pass # Replace with function body.
