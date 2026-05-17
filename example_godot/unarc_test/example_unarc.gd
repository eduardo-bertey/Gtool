extends Control

# Instanciamos la clase Unarc expuesta desde Rust via GDExtension
var unarc = Unarc.new()

const TEST_ZIP_PATH = "user://test_archive.zip"
const EXTRACT_DIR = "user://unarc_extracted"

func _ready():
	$Label.text = "Haz clic en 'Crear y Probar ZIP' para iniciar la prueba de unarc-rs."

func _on_test_pressed() -> void:
	$Label.text = "Iniciando prueba...\n"
	
	# 1. Crear un archivo ZIP de prueba usando el ZIPPacker interno de Godot
	var packer = ZIPPacker.new()
	var err = packer.open(TEST_ZIP_PATH, ZIPPacker.APPEND_CREATE)
	if err != OK:
		$Label.text += "Error: No se pudo crear el archivo de prueba ZIP.\n"
		return
		
	# Agregar archivo 1
	packer.start_file("hola_mundo.txt")
	packer.write_file("¡Hola Mundo! Este es un archivo extraído con unarc-rs y Rust en Godot 4.\n".to_utf8_buffer())
	packer.close_file()
	
	# Agregar archivo 2 (carpeta simulada)
	packer.start_file("datos/config.json")
	packer.write_file('{"nombre": "Gtool", "version": "1.0", "crate": "unarc-rs"}'.to_utf8_buffer())
	packer.close_file()
	
	packer.close()
	
	$Label.text += "1. Archivo ZIP creado con éxito en: %s\n" % ProjectSettings.globalize_path(TEST_ZIP_PATH)
	
	# Obtener ruta absoluta del sistema para Rust
	var absolute_zip_path = ProjectSettings.globalize_path(TEST_ZIP_PATH)
	var absolute_extract_dir = ProjectSettings.globalize_path(EXTRACT_DIR)
	
	# 2. Listar entradas con unarc (Rust)
	$Label.text += "\n2. Leyendo entradas con unarc (Rust):\n"
	var entries = unarc.get_entries(absolute_zip_path)
	for entry in entries:
		$Label.text += "   - Nombre: %s | Tamaño original: %d bytes | ¿Directorio?: %s\n" % [
			entry["name"], entry["size"], "Sí" if entry["is_directory"] else "No"
		]
	
	# 3. Leer una entrada específica en memoria como bytes (Rust)
	$Label.text += "\n3. Leyendo 'hola_mundo.txt' en memoria:\n"
	var bytes = unarc.read_entry_bytes(absolute_zip_path, "hola_mundo.txt")
	if bytes.size() > 0:
		var contenido_texto = bytes.get_string_from_utf8()
		$Label.text += "   Contenido leído en memoria: '%s'\n" % contenido_texto.strip_edges()
	else:
		$Label.text += "   Error: No se pudo leer los bytes de 'hola_mundo.txt'\n"
		
	# 4. Extraer todo a un directorio temporal (Rust)
	$Label.text += "\n4. Extrayendo todo el contenido a: %s\n" % absolute_extract_dir
	var extract_success = unarc.extract_all(absolute_zip_path, absolute_extract_dir)
	if extract_success:
		$Label.text += "   ¡Extracción completa exitosa!\n"
		
		# Verificar que los archivos existen físicamente en disco
		var file_exists = FileAccess.file_exists(EXTRACT_DIR + "/hola_mundo.txt")
		var config_exists = FileAccess.file_exists(EXTRACT_DIR + "/datos/config.json")
		
		$Label.text += "   Verificación física en disco:\n"
		$Label.text += "   - hola_mundo.txt existe: %s\n" % ("Sí" if file_exists else "No")
		$Label.text += "   - datos/config.json existe: %s\n" % ("Sí" if config_exists else "No")
		
		if config_exists:
			var file = FileAccess.open(EXTRACT_DIR + "/datos/config.json", FileAccess.READ)
			$Label.text += "     Contenido de config.json: %s\n" % file.get_as_text()
			file.close()
	else:
		$Label.text += "   Error: Falló la extracción con unarc-rs.\n"

func _on_exit_pressed() -> void:
	self.queue_free()
