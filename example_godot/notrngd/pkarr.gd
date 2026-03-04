extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "Pkarr es un sistema de resolucion de nombres de dominio descentralizado que funciona de forma parecida a una libreta de contactos compartida por todo el mundo. No usa servidores centrales como el DNS comun sino que se apoya en una red de nodos iguales.

Usa una tabla de hash distribuida llamada Kademlia. Esta red sirve para guardar y buscar datos de forma eficiente. Cuando buscas una direccion en Pkarr no le preguntas a un servidor de Google sino que le preguntas a los nodos mas cercanos a esa llave en la red hasta que alguien te da la respuesta.

El funcionamiento se basa en llaves publicas. Tu nombre de dominio es tu propia llave publica. Esto significa que sos el unico dueño de tu direccion y nadie te la puede quitar o censurar porque sos el unico que tiene la llave privada para firmar y actualizar esos datos.

Se usa principalmente para resolver direcciones IP o datos temporales. Un usuario publica su IP actual firmada con su llave en la red Kademlia. Los demas usuarios buscan esa llave en la red y obtienen la IP actualizada. Es muy util para dispositivos que cambian de conexion seguido o para servicios que corren en redes privadas porque permite encontrarlos sin usar un registro central.

Los datos en Pkarr son momentaneos. Tienen un tiempo de vida corto y se borran si no se actualizan. Esto mantiene la red limpia y rapida para lo que mas se usa que es encontrar la ubicacion actual de un servicio o una persona en internet de forma privada y segura"
	pass # Replace with function body.


func _on_mouse_exited() -> void:
	pass # Replace with function body.
