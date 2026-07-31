extends Control

## Ejemplo de la clase Tokenizer (Hugging Face tokenizer.json)
## Carga un tokenizer desde JSON, sin conocer los tokens de antemano.

var tk = Tokenizer.new()
var tokenizer_path := "res://example_godot/tokenizer/tokenizer.json"

func _ready() -> void:
	print("\n═══════════════════════════════════════════════════")
	print("  TOKENIZER — Hugging Face tokenizer.json")
	print("═══════════════════════════════════════════════════\n")

	$path_label.text = "Tokenizador: %s" % tokenizer_path

	# Cargar tokenizer desde JSON
	var ok := tk.load_from_file(tokenizer_path)
	if not ok:
		$status_label.text = "Fallo al cargar %s" % tokenizer_path
		print("❌ No se pudo cargar el tokenizer.")
		print("   Coloca un tokenizer.json (HF) en: ", tokenizer_path)
		return

	$status_label.text = "Tokenizador cargado ✓"
	print("✅ Tokenizer cargado desde JSON")
	print("   Vocabulario: %d tokens" % tk.vocab_size())

	# Encode / decode
	$input.text = "Hola mundo, esto es una prueba de tokenización."
	_on_encode_pressed()

func _on_encode_pressed() -> void:
	if not tk.is_loaded():
		return

	var text: String = $input.text
	var ids := tk.encode(text)

	var tokens := PackedStringArray()
	for id in ids:
		tokens.append(tk.id_to_token(id))

	$ids_label.text = "IDs: %s" % str(ids)
	$tokens_label.text = "Tokens: %s" % ", ".join(tokens)
	print("Texto : ", text)
	print("IDs   : ", ids)
	print("Tokens: ", tokens)

func _on_decode_pressed() -> void:
	if not tk.is_loaded():
		return

	var ids := tk.encode($input.text)
	var decoded := tk.decode(ids)
	$decoded_label.text = "Decodificado: %s" % decoded
	print("Decoded: ", decoded)
