extends Control

func _ready():
	test_votacion_realista()

func test_votacion_realista():
	var n = Nostringer.new()
	print("--- TEST DE VOTACIÓN REALISTA (Simulación de Red) ---")

	# 1. Configuración del Anillo Público
	var llaves_privadas = []
	var anillo = []
	for i in range(5):
		var kp = n.generate_keypair("xonly")
		llaves_privadas.append(kp["private_key"])
		anillo.append(kp["public_key"])
	
	# La urna solo guarda los paquetes de datos que llegan de la red
	var urna = []

	# 2. EMISIÓN DE VOTOS (Simulado P2P)
	print("\n--- Fase de Recepción ---")
	
	# Voto 1
	urna.append(enviar_voto_a_red(n, "Propuesta A", llaves_privadas[1], anillo))
	
	# Voto 2
	urna.append(enviar_voto_a_red(n, "Propuesta B", llaves_privadas[3], anillo))
	
	# FRAUDE: El mismo autor del Voto 1 envía otro mensaje
	print("[!] Recibiendo paquete sospechoso...")
	urna.append(enviar_voto_a_red(n, "Propuesta C (Trampa)", llaves_privadas[1], anillo))

	# 3. AUDITORÍA (Se averigua quién fue después de cerrar la urna)
	procesar_auditoria(n, urna, anillo)

func enviar_voto_a_red(instancia: Nostringer, msg: String, nsec: String, ring: Array) -> Dictionary:
	var res = instancia.sign(msg.to_utf8_buffer(), nsec, ring, "blsag")
	if not res.has("signature"): 
		return {}
	
	# Simula el paquete que viaja por la red
	return {
		"sig": res["signature"],
		"msg": msg,
		"ki": res["key_image"] # El servidor/peer lo extrae de la firma
	}

func procesar_auditoria(instancia: Nostringer, datos_urna: Array, ring: Array):
	print("\n--- Iniciando Auditoría y Conteo ---")
	
	var mapa_imagenes = {} # key_image -> count
	
	for i in range(datos_urna.size()):
		var voto = datos_urna[i]
		var ki = voto["ki"]
		
		if mapa_imagenes.has(ki):
			print("\n[!] FRAUDE: Identidad duplicada (Key Image colisionada).")
			
			# Averiguar el autor: se busca en el anillo quién es el responsable de esta firma
			for pk in ring:
				if instancia.verify(voto["sig"], voto["msg"].to_utf8_buffer(), ring, ki):
					print("    >> INFRACTOR IDENTIFICADO: ", pk)
					break
		else:
			mapa_imagenes[ki] = 1
			print("Voto %d validado." % i)

	print("\n--- Auditoría completada. ---")
