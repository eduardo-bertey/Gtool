extends Node

var info_torrent = InfoTorrent.new()

func _ready() -> void:
	# 1. Ejemplo desde un Magnet/InfoHash (Asíncrono con Red)
	#test_magnet("magnet:?xt=urn:btih:01c137287d6f0ed05a56742dae794f632c79ff3d")
	
	# 2. Ejemplo desde archivo .torrent (Síncrono/Local)
	# test_file("C:/Users/Emabe/Downloads/ubuntu-24.04.torrent")
	pass
func _process(_delta: float) -> void:
	# IMPORTANTE: Necesario para recibir la metadata cuando se usa fetch_metadata()
	# poll_metadata() revisa si los hilos de fondo terminaron y emite la señal
	if info_torrent.poll_metadata():
		print("--- Metadata Poll: Éxito ---")

func test_magnet(uri: String):
	info_torrent = InfoTorrent.new()
	
	# Conectamos la señal para saber cuando la metadata esté lista
	info_torrent.metadata_loaded.connect(_on_metadata_ready.bind(info_torrent))
	
	if info_torrent.load_from_magnet(uri):
		print("--- Magnet Cargado ---")
		
		# OPCIONAL: Puedes añadir trackers personalizados desde GDScript (HTTPS soportado)
		info_torrent.add_tracker("https://tracker.nanoset.net:443/announce")
		# info_torrent.set_trackers(["udp://...", "https://..."])
		
		print("Hash: ", info_torrent.get_info_hash())
		print("Nombre (pre-metadata): ", info_torrent.get_display_name())
		print("Iniciando búsqueda en red (DHT/Trackers)...")
		
		info_torrent.fetch_metadata() # Arranca hilos de Rust para buscar metadatos
	else:
		print("Error en formato de Magnet/Hash")

func test_file(path: String):
	var info = InfoTorrent.new()
	if info.load_from_file(path):
		print("\n✅ Archivo .torrent cargado localmente:")
		print("Nombre: ", info.get_display_name())
		print("Hash: ", info.get_info_hash())
		print("Tamaño total: ", info.get_total_size())
		
		var files = info.get_files()
		for f in files:
			print("- ", f.path, " (", String.humanize_size(f.size), ")")
	else:
		print("Error cargando archivo: ", path)

func _on_metadata_ready(info: InfoTorrent):
	print("\n✅ Metadata recibida desde la red!")
	print("Nombre Real: ", info.get_display_name())
	print("Archivos encontrados:")
	var files = info.get_files()
	for f in files:
		print("- ", f.path, " (", String.humanize_size(f.size), ")")
	
	print("Tamaño Total: ", String.humanize_size(info.get_total_size()))

func _on_button_pressed() -> void:
	print("Total: ", info_torrent.get_total_size())


func _on_uri_pressed() -> void:
	test_magnet("magnet:?xt=urn:btih:01c137287d6f0ed05a56742dae794f632c79ff3d")
	pass # Replace with function body.


func _on_torrent_pressed() -> void:
	test_file("C:/Users/Emabe/Downloads/ubuntu-25.10-desktop-amd64.iso.torrent")
	pass # Replace with function body.
