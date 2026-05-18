extends Control

# Nodos UI
@onready var file_path_input = $MainLayout/Sidebar/PathSection/HBox/FilePathInput
@onready var mode_selector = $MainLayout/Sidebar/ModeSection/ModeSelector
@onready var chunk_size_input = $MainLayout/Sidebar/ChunkSection/ChunkSizeInput
@onready var status_label = $MainLayout/Sidebar/StatusLabel
@onready var next_block_btn = $MainLayout/Sidebar/ActionButtons/NextBlockBtn
@onready var load_btn = $MainLayout/Sidebar/ActionButtons/LoadBtn
@onready var info_text = $MainLayout/Sidebar/InfoPanel/InfoText

@onready var preview_title = $MainLayout/PreviewArea/PanelHeader/HBoxContainer/PreviewTitle
@onready var main_preview = $MainLayout/PreviewArea/PreviewPanel/ScrollContainer/MainPreview
@onready var file_dialog = $FileDialog

# Variables de Estado
var file_path: String = ""
var file_size: int = 0
var current_offset: int = 0
var chunk_size: int = 4096 # 4KB por defecto
var viewing_mode: String = "hex" # "text" o "hex"

# Contenedores para la vista inicial
var start_chunk_bytes: PackedByteArray = PackedByteArray()
var end_chunk_bytes: PackedByteArray = PackedByteArray()
var middle_loaded_chunks: Array = [] # Almacena partes cargadas al dar "Continuar"

func _ready() -> void:
	status_label.text = "Selecciona un archivo para previsualizar."
	next_block_btn.disabled = true
	main_preview.text = "Usa el panel de la izquierda para cargar un archivo."
	_update_info()

func _update_info() -> void:
	var info = "--- METADATOS ---\n"
	info += "Archivo: %s\n" % (file_path.get_file() if file_path != "" else "Ninguno")
	info += "Tamaño total: %s\n" % (_format_size(file_size) if file_path != "" else "0 bytes")
	info += "Offset actual: %d bytes\n" % current_offset
	info += "Tamaño de bloque: %s\n" % _format_size(chunk_size)
	info_text.text = info

func _format_size(bytes: int) -> String:
	if bytes < 1024:
		return "%d B" % bytes
	elif bytes < 1024 * 1024:
		return "%.2f KB" % (bytes / 1024.0)
	else:
		return "%.2f MB" % (bytes / (1024.0 * 1024.0))

func _on_browse_pressed() -> void:
	file_dialog.popup_centered_ratio(0.7)

func _on_file_dialog_file_selected(path: String) -> void:
	file_path_input.text = path
	_on_load_pressed()

func _on_load_pressed() -> void:
	var path = file_path_input.text
	if path == "":
		status_label.text = "[Error] Especifica una ruta de archivo válida."
		return
		
	var global_path = ProjectSettings.globalize_path(path)
	if not FileAccess.file_exists(global_path):
		status_label.text = "[Error] El archivo no existe en el disco."
		return
		
	file_path = global_path
	var file = FileAccess.open(file_path, FileAccess.READ)
	if not file:
		status_label.text = "[Error] No se pudo abrir el archivo."
		return
		
	file_size = file.get_length()
	current_offset = 0
	middle_loaded_chunks.clear()
	
	# Leer tamaño de bloque configurado
	var custom_size = chunk_size_input.text.to_int()
	if custom_size > 0:
		chunk_size = custom_size * 1024 # Convertir KB a Bytes
	else:
		chunk_size = 4096
		
	viewing_mode = "text" if mode_selector.selected == 0 else "hex"
	preview_title.text = "Visualizando: %s (%s)" % [file_path.get_file(), viewing_mode.to_upper()]
	
	# Caso 1: Archivo Pequeño (Cabe entero en un bloque)
	if file_size <= chunk_size:
		file.seek(0)
		start_chunk_bytes = file.get_buffer(file_size)
		next_block_btn.disabled = true
		status_label.text = "Archivo chico cargado por completo (%s)." % _format_size(file_size)
		_render_single_view()
	# Caso 2: Archivo Grande (Carga progresiva)
	else:
		# 1. Leer Inicio (Primeros bytes)
		file.seek(0)
		start_chunk_bytes = file.get_buffer(chunk_size)
		current_offset = chunk_size
		
		# 2. Leer Final (Últimos bytes)
		var end_offset = file_size - chunk_size
		if end_offset > chunk_size:
			file.seek(end_offset)
			end_chunk_bytes = file.get_buffer(chunk_size)
		else:
			end_chunk_bytes = PackedByteArray()
			
		next_block_btn.disabled = false
		status_label.text = "Archivo grande cargado. Mostrando Inicio y Fin. Haz clic en 'Continuar'."
		_render_progressive_view()
		
	file.close()
	_update_info()

# Renderizado de archivo pequeño completo
func _render_single_view() -> void:
	main_preview.clear()
	var heading = "--- ARCHIVO COMPLETO (%s) ---\n\n" % _format_size(file_size)
	main_preview.append_text(heading)
	
	if viewing_mode == "text":
		var text = start_chunk_bytes.get_string_from_utf8()
		if text == "":
			text = start_chunk_bytes.get_string_from_ascii()
		main_preview.append_text(text)
	else:
		main_preview.append_text(generate_hex_dump(start_chunk_bytes, 0))

# Renderizado de archivo grande (Inicio ... omitido ... Fin)
func _render_progressive_view() -> void:
	main_preview.clear()
	
	# Sección Inicio
	main_preview.append_text("--- INICIO DEL ARCHIVO (Primeros %s) ---\n\n" % _format_size(chunk_size))
	if viewing_mode == "text":
		var text = start_chunk_bytes.get_string_from_utf8()
		if text == "": text = start_chunk_bytes.get_string_from_ascii()
		main_preview.append_text(text)
	else:
		main_preview.append_text(generate_hex_dump(start_chunk_bytes, 0))
		
	# Bloques intermedios ya cargados secuencialmente
	for block in middle_loaded_chunks:
		main_preview.append_text("\n\n--- BLOQUE CONTINUADO (Bytes %d a %d) ---\n\n" % [block["offset"], block["offset"] + block["bytes"].size()])
		if viewing_mode == "text":
			var text = block["bytes"].get_string_from_utf8()
			if text == "": text = block["bytes"].get_string_from_ascii()
			main_preview.append_text(text)
		else:
			main_preview.append_text(generate_hex_dump(block["bytes"], block["offset"]))
			
	# Sección Omitidos / Cargados
	var remaining = file_size - current_offset - end_chunk_bytes.size()
	if remaining > 0:
		main_preview.append_text("\n\n... (Hay %s omitidos en la previsualización intermedio. Presiona 'Continuar' para cargarlos) ...\n" % _format_size(remaining))
	else:
		main_preview.append_text("\n\n... (Cuerpo central completamente cargado) ...\n")
		
	# Sección Fin
	if end_chunk_bytes.size() > 0:
		main_preview.append_text("\n--- FINAL DEL ARCHIVO (Últimos %s) ---\n\n" % _format_size(end_chunk_bytes.size()))
		if viewing_mode == "text":
			var text = end_chunk_bytes.get_string_from_utf8()
			if text == "": text = end_chunk_bytes.get_string_from_ascii()
			main_preview.append_text(text)
		else:
			main_preview.append_text(generate_hex_dump(end_chunk_bytes, file_size - end_chunk_bytes.size()))

# Cargar siguiente porción
func _on_next_block_pressed() -> void:
	if file_path == "": return
	
	var file = FileAccess.open(file_path, FileAccess.READ)
	if not file: return
	
	# Detenerse si ya chocamos con el final
	var end_boundary = file_size - end_chunk_bytes.size()
	if current_offset >= end_boundary:
		status_label.text = "Llegaste al límite previsualizado del final del archivo."
		next_block_btn.disabled = true
		file.close()
		return
		
	file.seek(current_offset)
	var bytes_to_read = chunk_size
	if current_offset + bytes_to_read > end_boundary:
		bytes_to_read = end_boundary - current_offset
		
	if bytes_to_read <= 0:
		next_block_btn.disabled = true
		file.close()
		return
		
	var new_bytes = file.get_buffer(bytes_to_read)
	middle_loaded_chunks.push_back({
		"offset": current_offset,
		"bytes": new_bytes
	})
	
	current_offset += new_bytes.size()
	status_label.text = "Cargados %s adicionales desde el centro." % _format_size(new_bytes.size())
	
	if current_offset >= end_boundary:
		next_block_btn.disabled = true
		status_label.text += " (Todo el archivo previsualizado)"
		
	file.close()
	_render_progressive_view()
	_update_info()

# Generador de Dump Hexadecimal elegante
func generate_hex_dump(bytes: PackedByteArray, start_address: int = 0) -> String:
	var result = ""
	var line_bytes = PackedByteArray()
	
	for i in range(bytes.size()):
		line_bytes.push_back(bytes[i])
		if line_bytes.size() == 16 or i == bytes.size() - 1:
			# Dirección hexadecimal
			var addr_str = "%04X" % (start_address + i - line_bytes.size() + 1)
			var hex_str = ""
			var ascii_str = ""
			
			for b in line_bytes:
				hex_str += "%02X " % b
				if b >= 32 and b <= 126:
					ascii_str += char(b)
				else:
					ascii_str += "."
			
			# Espaciado para completar última línea si es corta
			while hex_str.length() < 48:
				hex_str += "   "
				
			result += "%s | %s | %s\n" % [addr_str, hex_str, ascii_str]
			line_bytes.clear()
			
	return result

func _on_back_pressed() -> void:
	get_tree().change_scene_to_file("res://example_godot/unarc_test/advanced_unarc.tscn")
