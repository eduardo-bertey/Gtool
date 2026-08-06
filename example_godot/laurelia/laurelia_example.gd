extends Control

## Ejemplo de la clase LaureliaChat (LLM Candle + descarga automática HF).
## Si no hay modelo local, descarga el del proyecto original:
##   https://huggingface.co/ScortexIA/laurelia (revisión laurelia-llm)
## El checkpoint puede pesar varios GB y tarda según la conexión.
## Nota: la carga es síncrona; la UI queda congelada durante la descarga/load.

var chat = LaureliaChat.new()

func _ready() -> void:
	$path_label.text = "Repo: ScortexIA/laurelia @ laurelia-llm (auto-download = %s)" % chat.auto_download
	$status_label.text = "Esperando... hacé clic en Cargar modelo"

	$load_button.pressed.connect(_on_load_pressed)
	$generate_button.pressed.connect(_on_generate_pressed)
	$unload_button.pressed.connect(_on_unload_pressed)

func _on_load_pressed() -> void:
	$status_label.text = "Cargando modelo... (descarga varios GB la primera vez)"
	$load_button.disabled = true
	$load_button.text = "Cargando..."
	await get_tree().process_frame

	var ok := chat.load_model()

	$load_button.disabled = false
	$load_button.text = "Cargar modelo"
	if ok:
		$status_label.text = "Modelo cargado ✓"
	else:
		$status_label.text = "❌ Falló la carga. Mirá la consola."

func _on_generate_pressed() -> void:
	if not chat.is_loaded():
		$status_label.text = "Primero cargá el modelo."
		return
	$output.text = "Generando..."
	await get_tree().process_frame
	$output.text = chat.generate($input.text)

func _on_unload_pressed() -> void:
	chat.unload_model()
	$status_label.text = "Modelo liberado."
	$output.text = ""
