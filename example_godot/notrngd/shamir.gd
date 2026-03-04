extends Button


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass


func _on_mouse_entered() -> void:
	$"../../TextEdit".text = "El Esquema de Secreto Compartido de Shamir permite dividir un mensaje o clave secreta en varias partes llamadas fragmentos para que nadie tenga el control total por sí solo.

En un ejemplo de 5 fragmentos con un umbral de 3 funciona configurando un sistema donde entregas una parte a cada participante pero el mensaje original permanece bloqueado e ilegible hasta que se junten al menos 3 de ellos. Si alguien tiene solo 1 o 2 partes no obtiene absolutamente ninguna información sobre el contenido real ya que matemáticamente es imposible reconstruir el secreto sin alcanzar el mínimo requerido.

Este método utiliza polinomios matemáticos donde el secreto es el punto de origen y los fragmentos son puntos distintos en la curva por lo que con el número suficiente de puntos puedes trazar la línea y recuperar el origen pero con menos puntos la solución es infinita.

Es una técnica ideal para seguridad extrema porque permite perder algunos fragmentos o que algún custodio sea deshonesto sin que el sistema colapse siempre que se mantenga el quórum necesario para la reconstrucción final"
	pass # Replace with function body.
