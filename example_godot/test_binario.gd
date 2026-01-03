extends Control

func _ready():
	test_binary_modes()

func test_binary_modes():
	var nostringer = Nostringer.new()
	print("--- Iniciando Prueba BINARIA Multimodal ---")

	# Setup con 2 miembros (MÍNIMO REQUERIDO POR LA LIBRERÍA OFICIAL)
	var kp = nostringer.generate_keypair("xonly")
	var kp_dummy = nostringer.generate_keypair("xonly")
	var ring = [kp["public_key"], kp_dummy["public_key"]]
	var message = "Mensaje secreto".to_utf8_buffer()

	# 1. Modo SAG (Unlinkable) - Binario
	print("\n[MODO SAG]")
	var res_sag = nostringer.sign_bin(message, kp["private_key"], ring, "sag")
	var sag_sig = res_sag["signature"]
	var sag_ki = res_sag["key_image"]
	
	print("Tamaño Firma SAG: ", sag_sig.size())
	if sag_sig.size() > 0:
		var sag_ok = nostringer.verify_bin(sag_sig, message, ring, sag_ki)
		print("Verificación SAG Binaria: ", sag_ok)
		assert(sag_ok)

	# 2. Modo BLSAG (Linkable) - Binario
	print("\n[MODO BLSAG]")
	var res_blsag = nostringer.sign_bin(message, kp["private_key"], ring, "blsag")
	var blsag_sig = res_blsag["signature"]
	var blsag_ki = res_blsag["key_image"]
	
	print("Tamaño Firma BLSAG: ", blsag_sig.size())
	print("¿Tiene Key Image?: ", blsag_ki.size() > 0)
	
	if blsag_sig.size() > 0:
		var blsag_ok = nostringer.verify_bin(blsag_sig, message, ring, blsag_ki)
		print("Verificación BLSAG Binaria: ", blsag_ok)
		assert(blsag_ok)

	print("\n--- ¡Pruebas Binarias Finalizadas con Éxito! ---")
