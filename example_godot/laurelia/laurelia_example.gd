extends Control

var chat = LaureliaChat.new()
var http = HTTPRequest.new()

const URL := "https://huggingface.co/ScortexIA/laurelia/resolve/laurelia-llm/"
const CKPT := "checkpoint.pt"
const TOK := "tokenizer.json"
const DIR := "user://hf_models/laurelia"

var _file := ""

func _ready() -> void:
	http.request_completed.connect(_on_done)
	add_child(http)
	$download.pressed.connect(_download)
	$load.pressed.connect(_load)
	$generate.pressed.connect(_generate)
	$unload.pressed.connect(_unload)
	_status("Listo. Tocá Descargar o Cargar.")

func _path(name: String) -> String:
	return ProjectSettings.globalize_path(DIR).path_join(name)

func _have() -> bool:
	return FileAccess.file_exists(_path(CKPT)) and FileAccess.file_exists(_path(TOK))

func _status(msg: String) -> void:
	$status.text = msg

func _download() -> void:
	if _have():
		_status("Ya está el modelo.")
		return
	$download.disabled = true
	_file = CKPT
	_status("Descargando checkpoint...")
	http.request(URL + CKPT)

func _on_done(result: int, code: int, _headers: PackedStringArray, body: PackedByteArray) -> void:
	if result != HTTPRequest.RESULT_SUCCESS or code != 200:
		$download.disabled = false
		_status("Error HTTP %d." % code)
		return
	var dir := ProjectSettings.globalize_path(DIR)
	DirAccess.make_dir_recursive_absolute(dir)
	var f := FileAccess.open(_path(_file), FileAccess.WRITE)
	f.store_buffer(body)
	f.close()
	if _file == CKPT:
		_file = TOK
		_status("Descargando tokenizer...")
		http.request(URL + TOK)
	else:
		$download.disabled = false
		_status("Modelo descargado. Tocá Cargar.")

func _load() -> void:
	if not _have():
		_status("Primero descargá el modelo.")
		return
	chat.checkpoint_file = _path(CKPT)
	chat.tokenizer_file = _path(TOK)
	chat.auto_download = false
	$load.disabled = true
	_status("Cargando modelo...")
	await get_tree().process_frame
	if chat.load_model():
		$load.disabled = false
		_status("Modelo cargado. Tocá Generar.")
	else:
		$load.disabled = false
		_status("Error al cargar. Mirá la consola.")

func _generate() -> void:
	if not chat.is_loaded():
		_status("Primero cargá el modelo.")
		return
	$output.text = "Generando..."
	$stats.text = ""
	await get_tree().process_frame
	var start: int = Time.get_ticks_msec()
	var out: String = String(chat.generate($input.text))
	$output.text = out
	var elapsed: float = float(Time.get_ticks_msec() - start) / 1000.0
	var tokens: int = int(chat.max_new_tokens)
	var tps: float = float(tokens) / elapsed if elapsed > 0.0 else 0.0
	$stats.text = "%.1f s | %d tokens generados | %.1f tok/s" % [elapsed, tokens, tps]
	print("generation stats: %.1f s | %d tokens | %.1f tok/s" % [elapsed, tokens, tps])
	_status("Listo.")

func _unload() -> void:
	chat.unload_model()
	$output.text = ""
	_status("Modelo liberado.")
