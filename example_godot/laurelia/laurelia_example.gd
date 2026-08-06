extends Control

## Ejemplo de LaureliaChat (LLM Candle) usando HFGodot para descargar.
## Si no hay modelo local, HFGodot descarga del repo original:
##   https://huggingface.co/ScortexIA/laurelia (revisión laurelia-llm)
## El checkpoint puede pesar varios GB y tarda según la conexión.

var hf = HFGodot.new()
var chat = LaureliaChat.new()

const HF_ORG_REPO := "ScortexIA/laurelia"
const CKPT_FILE := "checkpoint.pt"
const TOK_FILE := "tokenizer.json"
const LOCAL_DIR := "user://hf_models/laurelia"

func _ready() -> void:
	$path_label.text = "Repo: %s @ laurelia-llm" % HF_ORG_REPO
	$status_label.text = "Esperando... hacé clic en Descargar modelo"

	$download_button.pressed.connect(_on_download_pressed)
	$load_button.pressed.connect(_on_load_pressed)
	$generate_button.pressed.connect(_on_generate_pressed)
	$unload_button.pressed.connect(_on_unload_pressed)

func _on_download_pressed() -> void:
	$status_label.text = "Descargando modelo... (varios GB la primera vez)"
	$download_button.disabled = true
	$download_button.text = "Descargando..."
	await get_tree().process_frame

	# Sin token: funciona para repos públicos.
	var ok := hf.init_client("")

	var ckpt: String = ""
	var tok: String = ""
	if ok:
		ckpt = hf.download_file(HF_ORG_REPO, CKPT_FILE, LOCAL_DIR, "model")
		tok = hf.download_file(HF_ORG_REPO, TOK_FILE, LOCAL_DIR, "model")

	$download_button.disabled = false
	$download_button.text = "Descargar modelo"
	if ckpt != "" and tok != "":
		$status_label.text = "Descargado ✓. Ahora tocá Cargar modelo."
		chat.hf_org = ""
		chat.hf_repo = ""
		chat.auto_download = false
		chat.checkpoint_file = ckpt
		chat.tokenizer_file = tok
	else:
		$status_label.text = "❌ Falló la descarga. Mirá la consola."

func _on_load_pressed() -> void:
	if not chat.checkpoint_file or chat.checkpoint_file == "":
		$status_label.text = "Primero descargá el modelo."
		return
	$status_label.text = "Cargando modelo en memoria..."
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
