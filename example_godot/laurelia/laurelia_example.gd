extends Control

## Ejemplo de LaureliaChat (LLM Candle).
## Descarga el modelo con HTTPRequest nativo de Godot (funciona en Android,
## a diferencia de hf-hub/reqwest que paniquea con rustls-platform-verifier).

var chat = LaureliaChat.new()
var http = HTTPRequest.new()

const HF_ORG_REPO := "ScortexIA/laurelia"
const REVISION := "laurelia-llm"
const CKPT_FILE := "checkpoint.pt"
const TOK_FILE := "tokenizer.json"
const LOCAL_DIR := "user://hf_models/laurelia"

func _ready() -> void:
	$path_label.text = "Repo: %s @ %s" % [HF_ORG_REPO, REVISION]
	$status_label.text = "Esperando... hacé clic en Descargar modelo"

	http.timeout = 0
	http.request_completed.connect(_on_request_completed)
	add_child(http)

	$download_button.pressed.connect(_on_download_pressed)
	$load_button.pressed.connect(_on_load_pressed)
	$generate_button.pressed.connect(_on_generate_pressed)
	$unload_button.pressed.connect(_on_unload_pressed)

func _hf_url(filename: String) -> String:
	return "https://huggingface.co/%s/resolve/%s/%s" % [HF_ORG_REPO, REVISION, filename]

func _download_file(filename: String) -> void:
	var url := _hf_url(filename)
	$status_label.text = "Descargando %s..." % filename
	http.request(url)

func _on_request_completed(result: int, code: int, headers: PackedStringArray, body: PackedByteArray) -> void:
	if result != HTTPRequest.RESULT_SUCCESS or code != 200:
		$status_label.text = "❌ Falló la descarga (HTTP %d, result %d). Mirá la consola." % [code, result]
		$download_button.disabled = false
		$download_button.text = "Descargar modelo"
		return

	var dir := DirAccess.open(LOCAL_DIR)
	if dir == null:
		DirAccess.make_dir_recursive_absolute(ProjectSettings.globalize_path(LOCAL_DIR))
	var path := ProjectSettings.globalize_path(LOCAL_DIR).path_join(_current_file)
	var f := FileAccess.open(path, FileAccess.WRITE)
	if f == null:
		$status_label.text = "❌ No se pudo escribir %s" % path
		$download_button.disabled = false
		$download_button.text = "Descargar modelo"
		return
	f.store_buffer(body)
	f.close()

	$status_label.text = "✓ Descargado %s (%s)" % [_current_file, _fmt_bytes(body.size())]

	if _current_file == CKPT_FILE:
		chat.checkpoint_file = path
		_current_file = TOK_FILE
		_download_file(TOK_FILE)
	else:
		chat.tokenizer_file = path
		chat.auto_download = false
		$download_button.disabled = false
		$download_button.text = "Descargar modelo"
		$status_label.text = "Modelo descargado ✓. Ahora tocá Cargar modelo."

func _fmt_bytes(n: int) -> String:
	if n >= 1048576:
		return "%.1f MB" % (n / 1048576.0)
	if n >= 1024:
		return "%.1f KB" % (n / 1024.0)
	return "%d B" % n

var _current_file := ""

func _on_download_pressed() -> void:
	$download_button.disabled = true
	$download_button.text = "Descargando..."
	_current_file = CKPT_FILE
	_download_file(CKPT_FILE)

func _on_load_pressed() -> void:
	if chat.checkpoint_file == "" or chat.tokenizer_file == "":
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
