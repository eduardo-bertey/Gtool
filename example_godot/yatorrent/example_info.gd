extends Node

func _ready() -> void:
	# 1. Ejemplo desde un Magnet/InfoHash (Asíncrono)
	test_magnet("9bccd461daf03b10a7ad6ce033435f88c30eb607")
	
	# 2. Ejemplo desde archivo .torrent (Síncrono/Inmediato)
	# test_file("res://sample.torrent")

func test_magnet(uri: String):
	var info_torrent = InfoTorrent.new()
	
	# La búsqueda en la red es asíncrona, conectamos la señal
	info_torrent.metadata_loaded.connect(_on_metadata_ready.bind(info_torrent))
	
	if info_torrent.load_from_magnet(uri):
		print("--- Iniciando búsqueda en red para Metadata ---")
		print("Hash: ", info_torrent.get_info_hash())
		info_torrent.fetch_metadata() # Arranca el motor de red
	else:
		print("Error en formato de Magnet/Hash")

func _on_metadata_ready(info: InfoTorrent):
	print("\n✅ Metadata recibida!")
	print("Archivos encontrados:")
	var files = info.get_files()
	for f in files:
		print("- ", f.path, " (", String.humanize_size(f.size), ")")

func test_file(path: String):
	var info = InfoTorrent.new()
	if info.load_from_file(path):
		print("\n✅ Archivo cargado:")
		for f in info.get_files():
			print("- ", f.path)
