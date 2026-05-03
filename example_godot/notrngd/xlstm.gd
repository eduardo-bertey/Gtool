extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:


	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = " El modelo xLSTM es una arquitectura de red neuronal recurrente de última generación que evoluciona el concepto clásico de la LSTM
	 para permitir que estos modelos escalen al nivel de los grandes modelos de lenguaje actuales.
	 Su diseño se basa en dos variantes fundamentales que transforman cómo se procesa y almacena la información.
	 Por un lado, el sLSTM introduce un mecanismo de gating exponencial combinado con una normalización dinámica,
	 lo que permite que el modelo actualice su memoria interna de forma mucho más estable y precisa que las versiones antiguas.
	Por otro lado, el mLSTM sustituye el estado oculto vectorial tradicional por una estructura de memoria basada en matrices,
	 lo cual incrementa exponencialmente la capacidad del sistema para almacenar y recuperar información compleja,
	 funcionando de manera muy similar a los valores y claves de un Transformer pero sin perder la naturaleza recurrente.
	 Esta arquitectura logra ser masivamente paralelizable durante el entrenamiento, eliminando la lentitud típica de las RNN,
	 y mantiene un costo de memoria constante durante la inferencia, lo que significa que el modelo no se ralentiza ni consume más recursos a medida que el texto se vuelve más largo.
	 En esencia, xLSTM es un puente técnico que combina la capacidad de aprendizaje masivo de un Transformer con la eficiencia operativa y la velocidad de una red recurrente tradicional"
	
	pass # Replace with function body.
