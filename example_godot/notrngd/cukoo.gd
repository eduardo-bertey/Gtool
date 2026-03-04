extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "Los filtros de Cuckoo son una estructura de datos probabilística diseñada para verificar si un elemento pertenece a un conjunto usando muy poca memoria y con una velocidad de respuesta extremadamente alta.

A diferencia de un filtro de Bloom tradicional el filtro de Cuckoo guarda una pequeña firma o fingerprint del hash de cada elemento en lugar del hash completo. Al usar solo 1 o 2 bytes por cada entrada permite comprimir miles de registros en un espacio minúsculo facilitando que quepan en la memoria caché del procesador para una validación casi instantánea.

El funcionamiento se basa en el hash de cuco donde cada elemento tiene dos o más posiciones posibles en una tabla. Si al insertar un nuevo hash la celda está ocupada el sistema expulsa al elemento anterior y lo mueve a su posición alternativa resolviendo colisiones de forma dinámica y manteniendo una densidad de llenado muy alta sin perder rendimiento.

Como ejemplo si quieres comprobar si una clave pública de Nostr está en una lista de bloqueo en lugar de comparar 32 bytes de cada hex el filtro solo compara 1 o de 2 bytes de su firma. Esto reduce el uso de RAM de gigabytes a unos pocos megabytes permitiendo procesar cientos de miles de comprobaciones por segundo con una tasa de error de falsos positivos ajustable según el tamaño de la firma elegida.

Es el método ideal para sistemas que necesitan sincronizar estados rápidos o verificar membresía en redes distribuidas donde el ancho de banda y la latencia son críticos"
	pass # Replace with function body.
