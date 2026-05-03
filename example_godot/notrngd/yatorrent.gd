extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = " El minGRU funciona eliminando la dependencia que tenían las puertas de la GRU tradicional respecto al estado anterior.
	 En una red normal, para saber qué olvidar, la IA tiene que mirar qué recordó en el paso previo,
	 lo que te obliga a procesar todo en fila, uno tras otro. El minGRU rompe esto haciendo que las puertas solo dependan de la entrada actual.
	 Esto permite aplicar algo llamado parallel scan, que básicamente es procesar toda la secuencia de datos al mismo tiempo en la GPU.
	 El resultado es un modelo que entrena volando y que, al ejecutarlo, solo necesita recordar un pequeño vector de datos,
	 sin importar si la conversación lleva diez o diez mil palabras.

El minLSTM es la misma medicina aplicada a la estructura de la LSTM.
Aquí se eliminan casi todas las puertas complejas y se deja una estructura lineal donde los datos se filtran mediante una normalización exponencial.
Al quitar las funciones no lineales que conectaban un paso con el siguiente, el modelo se vuelve totalmente paralelizable.
La diferencia clave es que mantiene una separación más clara entre lo que es memoria de trabajo y salida,
lo que suele darle una ventaja cuando el modelo tiene que aprender reglas lógicas estrictas o sintaxis de programación.
Es, en esencia, una forma de tener la capacidad de razonamiento de una LSTM pero con la velocidad de procesamiento de una multiplicación de matrices simple"
