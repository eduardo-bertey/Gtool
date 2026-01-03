extends Node

var nostr_peer: NostrPeer
var timer: Timer
@onready var scroll = $"../peer 2/ScrollContainer"
# Credenciales de Peer 2
var peer2_secret = ""
var peer1_pubkey = ""
var relay_url: Array[String] = ["wss://relay.mostro.network", "wss://nos.lol", "wss://relay.damus.io"]
var instancia = 5

func _conect():
	
	print("\n=== Peer 2 - Mostro Chat Demo ===\n")
	#relay_url.append($"../editerelay".text)
	var aux = $"../relayedit".relay_url
	if  aux.size() > 5:
		aux.shuffle()
		relay_url = relay_url#aux.slice(0, instancia)
	else:
		relay_url = relay_url#aux
	prints("se iniciaron : " , instancia , " relays :" , relay_url)
	print("\n=================================\n")
	
	
	# 1. Crear el nodo NostrPeer$"../edite relay"
	
	nostr_peer = NostrPeer.new()
	add_child(nostr_peer)
	
	peer2_secret = $peernsec.text
	peer1_pubkey = $peernpub.text
	$peernsec.visible = false
	$peernpub.visible = false
	# 2. Inicializar como participante
	print("Inicializando Peer 2...")
	var success = nostr_peer.init_participant(peer2_secret, peer1_pubkey, relay_url)
	
	if success:
		print("✅ Peer 2 inicializado correctamente")
	else:
		print("❌ Error al inicializar Peer 2")
		return
	
	# 3. Enviar mensaje de prueba
	print("Enviando mensaje de prueba...")
	if nostr_peer.send_message("Hola desde Peer 2! " + str(Time.get_unix_time_from_system())):
		print("✅ Mensaje enviado")
	else:
		print("❌ Error al enviar mensaje")
	
	# 4. Configurar polling de mensajes
	timer = Timer.new()
	timer.wait_time = 3.0  # Consultar cada 3 segundos
	timer.connect("timeout", Callable(self, "_on_poll_messages"))
	add_child(timer)
	timer.start()
	print("📡 Polling iniciado (cada 3s)...")

func _on_poll_messages():
	if self.visible == false:
		return
	var messages = nostr_peer.poll_messages()
	
	if messages.size() > 0:
		prints("proseso mensaje")
		$ScrollContainer/mensaje.text += str("\n📨 [PEER 2] Recibidos " + str(messages.size()) + " mensajes:")
		for msg in messages:
			$ScrollContainer/mensaje.text += str("\n  ----------------------------------------")
			$ScrollContainer/mensaje.text += str("\n  De: " + str(msg["pubkey"]))
			$ScrollContainer/mensaje.text += str("\n  Mensaje: " + str(msg["content"]))
			$ScrollContainer/mensaje.text += str("\n  Timestamp: " +  str(msg["created_at"]))
			$ScrollContainer/mensaje.text += str("\n  ----------------------------------------")
			prints(msg["pubkey"])
		await get_tree().process_frame
		scroll.scroll_vertical = scroll.get_v_scroll_bar().max_value
		
	#if messages.size() > 0:
		#print("\n📨 [PEER 2] Recibidos ", messages.size(), " mensajes:")
		#for msg in messages:
			#print("  ----------------------------------------")
			#print("  De: ", msg["pubkey"])
			#print("  Mensaje: ", msg["content"])
			#print("  Timestamp: ", msg["created_at"])
			#print("  ----------------------------------------")

# Función para enviar mensajes desde UI
func send_custom_message(text: String):
	if nostr_peer.send_message(text):
		print("✅ Mensaje enviado: ", text)
	else:
		print("❌ Error al enviar mensaje")


func _on_peer_2_pressed() -> void:
	_conect()
	pass # Replace with function body.


func _on_button_pressed() -> void:
	if $TextEdit.text != "" and not nostr_peer == null:
		nostr_peer.send_message($TextEdit.text)
	$TextEdit.text = ""
	pass # Replace with function body.
