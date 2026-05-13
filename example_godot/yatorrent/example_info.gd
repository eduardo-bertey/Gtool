extends Node

var info_torrent = InfoTorrent.new()
var peer = TPeer.new()
var peerip
var peerport
var inx = 1
var data
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
		
		# OPCIONAL: Puedes añadir trackers personalizados (UDP y HTTPS soportados)
		info_torrent.add_tracker("udp://tracker.opentrackr.org:1337/announce")
		info_torrent.add_tracker("udp://tracker.openbittorrent.com:6969/announce")
		info_torrent.add_tracker("udp://exodus.desync.com:6969/announce")
		info_torrent.add_tracker("https://tracker.nanoset.net:443/announce")
		
		# Trackers HTTP adicionales
		info_torrent.add_tracker("http://tracker.opentrackr.org:1337/announce")
		info_torrent.add_tracker("http://vito-tracker.space:6969/announce")
		info_torrent.add_tracker("http://tracker.qu.ax:6969/announce")
		info_torrent.add_tracker("https://tracker.nanoset.net:443/announce")
		info_torrent.add_tracker("http://tracker.zhuqiy.com:80/announce")
		info_torrent.add_tracker("http://tracker.tritan.gg:8080/announce")
		info_torrent.add_tracker("http://tracker.renfei.net:8080/announce")
		info_torrent.add_tracker("http://tracker.mywaifu.best:6969/announce")
		info_torrent.add_tracker("http://tracker.dler.org:6969/announce")
		info_torrent.add_tracker("http://tracker.dhitechnical.com:6969/announce")
		info_torrent.add_tracker("http://tracker.23794.top:6969/announce")
		info_torrent.add_tracker("http://tr.kxmp.cf:80/announce")
		info_torrent.add_tracker("http://t.overflow.biz:6969/announce")
		info_torrent.add_tracker("http://bvarf.tracker.sh:2086/announce")
		info_torrent.add_tracker("http://bittorrent-tracker.e-n-c-r-y-p-t.net:1337/announce")
		info_torrent.add_tracker("http://004430.xyz:80/announce")
		info_torrent.add_tracker("http://tracker2.dler.org:80/announce")
		info_torrent.add_tracker("http://tracker.waaa.moe:6969/announce")
		info_torrent.add_tracker("http://tracker.skyts.net:6969/announce")
		info_torrent.add_tracker("http://tracker.ghostchu-services.top:80/announce")
		info_torrent.add_tracker("http://tracker.dler.com:6969/announce")
		info_torrent.add_tracker("http://tracker.bt4g.com:2095/announce")
		info_torrent.add_tracker("http://tr.highstar.shop:80/announce")
		info_torrent.add_tracker("http://aboutbeautifulgallopinghorsesinthegreenpasture.online:80/announce")
		info_torrent.add_tracker("http://1337.abcvg.info:80/announce")
		
		# Trackers HTTPS adicionales
		info_torrent.add_tracker("https://tracker.zhuqiy.com:443/announce")
		info_torrent.add_tracker("https://tracker.yemekyedim.com:443/announce")
		info_torrent.add_tracker("https://tracker.pmman.tech:443/announce")
		info_torrent.add_tracker("https://tracker.nekomi.cn:443/announce")
		info_torrent.add_tracker("https://tracker.moeking.me:443/announce")
		info_torrent.add_tracker("https://tracker.ghostchu-services.top:443/announce")
		info_torrent.add_tracker("https://tracker.bt4g.com:443/announce")
		info_torrent.add_tracker("https://http1.torrust-tracker-demo.com:443/announce")
		
		# Trackers UDP adicionales
		info_torrent.add_tracker("udp://open.stealth.si:80/announce")
		info_torrent.add_tracker("udp://wepzone.net:6969/announce")
		info_torrent.add_tracker("udp://vito-tracker.space:6969/announce")
		info_torrent.add_tracker("udp://tracker.torrent.eu.org:451/announce")
		info_torrent.add_tracker("udp://explodie.org:6969/announce")
		info_torrent.add_tracker("udp://tracker.qu.ax:6969/announce")
		info_torrent.add_tracker("udp://tracker.playground.ru:6969/announce")
		info_torrent.add_tracker("udp://open.ftorrent.com:443/announce")
		
		print("Hash: ", info_torrent.get_info_hash())
		print("Nombre (pre-metadata): ", info_torrent.get_display_name())
		print("Iniciando búsqueda en red (DHT/Trackers)...")
		
		info_torrent.fetch_metadata() # Arranca hilos de Rust para buscar metadatos
	else:
		print("Error en formato de Magnet/Hash")

func test_file(path: String):
	info_torrent = InfoTorrent.new()
	if info_torrent.load_from_file(path):
		print("\n✅ Archivo .torrent cargado localmente:")
		print("Nombre: ", info_torrent.get_display_name())
		print("Hash: ", info_torrent.get_info_hash())
		print("Tamaño total: ", info_torrent.get_total_size())
		prints( info_torrent.get_files())
		prints("pieses ",info_torrent.get_piece_count())
		var files = info_torrent.get_files()
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
	if info_torrent.get_info_hash() == "":
		prints("valor nulo")
		return
	var conf : String = info_torrent.get_info_hash()
	print("hash tamaño",conf.length())
	print("Total: ", info_torrent.get_total_size())
	print("Nombre: ", info_torrent.get_display_name())
	print("Hash: ", info_torrent.get_info_hash())
	print("All Piece: ", info_torrent.get_piece_count())
	$panel.text += "Nombre: "+ str( info_torrent.get_display_name() + "\n")
	$panel.text += "Hash: "+ str( info_torrent.get_info_hash()+ "\n")
	$panel.text += "All Piece: "+ str( info_torrent.get_piece_count())+ "\n"

	var indice_a_pedir = 0
	var hash_bruto = get_hash_de_pieza(indice_a_pedir)
	var hash_hex = bytes_a_hex(hash_bruto)
	var ha =  info_torrent.get_piece_hashes()
	print("Hash Piece ", indice_a_pedir, " (Hex): ", hash_hex)
	print("Hash Piece ", indice_a_pedir, " (Raw): ", hash_bruto)
	print("Hash All : ", ha.size() / 20, " (Raw Size): ", hash_bruto.size())
	print("Tamaño total: ", info_torrent.get_total_size())
	prints("tamaño del piece " , info_torrent.get_piece_length())
	prints("piece custom ",get_piece_size(0))
	var files = info_torrent.get_files()
	for f in files:
		print("- ", f.path, " (", String.humanize_size(f.size), ")")

# hash de 20 bytes de una pieza 
func get_hash_de_pieza(indice: int) -> PackedByteArray:
	var todos_los_hashes = info_torrent.get_piece_hashes()
	var inicio = indice * 20
	return todos_los_hashes.slice(inicio, inicio + 20)

# Convierte de Bytes a Hexadecimal (SHA1 format)
func bytes_a_hex(bytes: PackedByteArray) -> String:
	var hex = ""
	for b in bytes:
		hex += "%02x" % b
	return hex


func get_piece_size(idx) -> int:
	if idx == null:
		push_warning("get_piece_size: El índice es Nil, se usará 0 por defecto.")
		idx = 0
		
	if not info_torrent:
		return 0
		
	var total_size = info_torrent.get_total_size()
	var piece_length = info_torrent.get_piece_length()
	var piece_count = info_torrent.get_piece_count()
	
	if idx < 0 or idx >= piece_count:
		return 0
	
	if idx == piece_count - 1:
		var remainder = total_size % piece_length
		return remainder if remainder > 0 else piece_length
		
	return piece_length


func _on_uri_pressed() -> void:
	#test_magnet("magnet:?xt=urn:btih:01c137287d6f0ed05a56742dae794f632c79ff3d")
	test_magnet("magnet:?xt=urn:btih:d69f91e6b2ae4c542468d1073a71d4ea13879a7f")
	pass # Replace with function body.


func _on_torrent_pressed() -> void:
	var absolute_path = ProjectSettings.globalize_path("res://example_godot/torrent/sample.torrent")
	test_file(absolute_path)

	#test_file("C:/Users/Emabe/Downloads/ubuntu-25.10-desktop-amd64.iso.torrent")
	pass # Replace with function body.


func _on_piece_pressed() -> void:
	var piece_idx = int(inx)
	data = peer.request_piece_raw(
		info_torrent.get_info_hash(),
		str($VBoxContainer/ip.text),
		int($VBoxContainer/port.text),
		piece_idx,
		get_piece_size(piece_idx),  # tamaño REAL de esta pieza (última puede ser más chica)
		int(info_torrent.get_total_size()),    # tamaño TOTAL del torrent
		info_torrent.get_piece_hash(piece_idx)
	)

func dowload(ip , port):
	var conf : String = info_torrent.get_info_hash()
	prints(conf.length())
	var piece_idx = int(inx)
	data = peer.request_piece_raw(
		info_torrent.get_info_hash(),
		str(ip),
		int(port),
		piece_idx,
		get_piece_size(piece_idx),  # tamaño REAL de esta pieza (última puede ser más chica)
		int(info_torrent.get_total_size()),    # tamaño TOTAL del torrent
		info_torrent.get_piece_hash(piece_idx)
	)
	if data.size() > 0:
		var bytes = PackedByteArray(data)
		var texto = bytes.get_string_from_utf8()
		print("✅ ¡Éxito! Recibida pieza 0 desde localhost (", data.size(), " bytes)")
		prints("longotud del texto : ",texto.length())
	else:
		print("❌ Falló la descarga desde localhost. Revisa el puerto y que el cliente de torrent esté abierto.")
	

func test_localhost(port: int):
	if not info_torrent.is_loaded():
		print("Error: El torrent no está cargado. Primero usa 'Cargar Torrent'.")
		return
		
	print("--- Probando Localhost en puerto ", port, " ---")
	var piece_idx = 0 # Pedimos la primera pieza
	
	data = peer.request_piece_raw(
		info_torrent.get_info_hash(),
		"127.0.0.1",
		port,
		piece_idx,
		get_piece_size(piece_idx),
		int(info_torrent.get_total_size()),
		info_torrent.get_piece_hash(piece_idx)
	)
	
	if data.size() > 0:
		var bytes = PackedByteArray(data)
		var texto = bytes.get_string_from_utf8()
		print("✅ ¡Éxito! Recibida pieza 0 desde localhost (", data.size(), " bytes)")
		#prints(texto)
		prints("longotud del texto : ",texto.length())
	else:
		print("❌ Falló la descarga desde localhost. Revisa el puerto y que el cliente de torrent esté abierto.")
	
	# Mostramos los bitfields capturados durante la conexión
	imprimir_bitfields()

func imprimir_bitfields():
	var bitfields = info_torrent.get_peer_bitfields()
	print("\n--- 🧩 ESTADO DE PIEZAS POR PEER (Bitfields) ---")
	if bitfields.is_empty():
		print("No hay bitfields registrados. ¿El peer envió el mensaje de bitfield?")
		return
		
	for peer_addr in bitfields:
		var torrents = bitfields[peer_addr]
		for info_hash in torrents:
			var bf_data = torrents[info_hash]
			var piezas_listo = decodificar_bitfield(bf_data, info_torrent.get_piece_count())
			print("📍 Peer: ", peer_addr)
			print("   Hash: ", info_hash)
			print("   Piezas Disponibles (", piezas_listo.size(), "/", info_torrent.get_piece_count(), "): ", piezas_listo)
			if piezas_listo.size() > 0:
				$panel.text += "\nPeer " + peer_addr + " tiene " + str(piezas_listo.size()) + " piezas."

func decodificar_bitfield(bitfield: PackedByteArray, total_pieces: int) -> Array:
	var pieces = []
	for i in range(total_pieces):
		var byte_index = i / 8
		var bit_index = 7 - (i % 8) # BitTorrent usa big-endian para los bits dentro del byte
		
		if byte_index < bitfield.size():
			if (bitfield[byte_index] >> bit_index) & 1:
				pieces.append(i)
	return pieces

func _on_exit_pressed() -> void:
	queue_free()
	pass # Replace with function body.


func _on_getpeer_pressed() -> void:
	update_peers()
	pass # Replace with function body.
	



func update_peers():
	var peers = info_torrent.get_peers()
	if peers.size() == 1 and peers[0] == "await":

		return
	
	var count = 0
	for ip in peers:
		count += 1
		$panel.text += "Peer encontrado: "+ str( ip+ "\n")
	#	extraer_ip_puerto(ip)
		print("Peer encontrado: ", ip)
	$panel.text += "Peer total: "+ str( count)+ "\n"

func loop_test():
	var peers = info_torrent.get_peers()
	if peers.size() == 1 and peers[0] == "await":

		return
	
	var count = 0
	for ip in peers:
		count += 1
		$panel.text += "Peer automaticos: "+ str( ip+ "\n")
		var peer = extraer_ip_puerto(ip)
		dowload(peer.ip , peer.port)
		print("Peer encontrado: ", ip)
		if count >= 20:
			break
	$panel.text += "Peer consultados: "+ str( count)+ "\n"
func _on_clear_pressed():
	info_torrent.clear_peers()


func _on_clearpanel_pressed() -> void:
	$panel.text = ""
	pass # Replace with function body.

func extraer_ip_puerto(raw_string: String) -> Dictionary:
	# rfind es el que no falla en G4 para buscar desde el final
	var cursor: int = raw_string.rfind(":")
	
	if cursor == -1:
		return {"ip": raw_string, "port": 0}
		
	var ip: String = raw_string.left(cursor)
	var port_str: String = raw_string.right(-(cursor + 1))
	
	# Limpieza de IPv6: Si empieza con '[' y termina con ']'
	if ip.begins_with("[") and ip.ends_with("]"):
		# substr(posicion_inicio, longitud)
		# Empezamos en 1 y le quitamos 2 a la longitud (el '[' y el ']')
		ip = ip.substr(1, ip.length() - 2)
		
	return {
		"ip": ip,
		"port": port_str.to_int()
	}


func _on_autoloop_pressed() -> void:
	loop_test()
	pass # Replace with function body.


func _on_localhost_pressed() -> void:
	test_localhost(16556)
	pass # Replace with function body.
