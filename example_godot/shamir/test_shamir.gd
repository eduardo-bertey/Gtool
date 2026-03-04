extends Node

func send(data: PackedByteArray):
	var shamir = Shamir.new()
	
	# 1. Datos del secreto (ejemplo: 64 bytes con valor 42)
	if data.size() == 0:
		data = PackedByteArray()
		for i in range(64):
			data.append(42)
		
	var count = int($LineEdit/LineEdit.text)
	var threshold = int($LineEdit/LineEdit2.text)
	
	print("Creando %d participaciones con umbral %d..." % [count, threshold])
	
	# 2. Crear las participaciones (shares)
	var shares = shamir.create_shares(data, count, threshold)
	if shares == null:
		print("Error al crear shares")
		return
	
	# 3. Perder una participación (borramos el índice 3)
	var shares_copy = shares.duplicate()
	shares_copy.remove_at(0)
	print("Participación eliminada. Quedan: ", shares_copy.size())
	
	##verify
	#prints("tamaño del shares :", shares_copy[0].size())
	#
	#
	# 4. Restaurar el secreto (debe funcionar con 4 participaciones)
	var restored = shamir.combine_shares(shares_copy)
	if restored != null:
		if restored == data:
			$Control/peer/Label.text = to_text(restored)
			print("ÉXITO: Secreto restaurado correctamente con "+ str(shares_copy.size()) + " participaciones")
			$Control/peer.color= Color.BLUE
	else:
		$Control/peer.color= Color.RED
		$Control/peer/Label.text = to_text(restored)
		print("ERROR: No se pudo restaurar con "+ str(shares_copy.size()) + " participaciones")
		
	# 5. Perder otra participación (debe fallar con 3 participaciones ya que el umbral es 4)
	shares_copy.remove_at(0)
	print("Otra participación eliminada. Quedan: ", shares_copy.size())
	
	var restored2 = shamir.combine_shares(shares_copy)
	if restored2 == data:
		print("ÉXITO: Secreto restaurado correctamente con "+ str(shares_copy.size()) + " participaciones")
		$Control/peer2/Label.text = to_text(restored2)
		$Control/peer2.color= Color.BLUE
	else:
		$Control/peer2.color= Color.RED
		$Control/peer2/Label.text = to_text(restored2)
		print("ERROR: No se pudo restaurar con "+ str(shares_copy.size()) + " participaciones")

	# 6. Perder otra participación (debe fallar con 3 participaciones ya que el umbral es 4)
	shares_copy.remove_at(0)
	print("Otra participación eliminada. Quedan: ", shares_copy.size())
	
	var restored3 = shamir.combine_shares(shares_copy)
	if restored3 == data:
		$Control/peer3/Label.text = to_text(restored3)
		print("ÉXITO: Secreto restaurado correctamente con "+ str(shares_copy.size()) + " participaciones")
		$Control/peer3.color= Color.BLUE
	else:
		$Control/peer3.color= Color.RED
		$Control/peer3/Label.text = to_text(restored3)
		print("ERROR: No se pudo restaurar con "+ str(shares_copy.size()) + " participaciones")


func string_a_ansi(texto: String) -> Array:
	# Convertimos el texto a un array de bytes (0-255)
	var bytes = texto.to_ascii_buffer()
	
	var msg  = PackedByteArray()
	for b in bytes:
		msg.append(b)
	if msg.size() <= 64:
		for i in range(64 - msg.size()):
			msg.append(0)
		
	
	return msg

func to_text(bytes_input) -> String:
	if bytes_input == null:
		return " "
	var limpio = PackedByteArray()
	for b in bytes_input:
		if b != 0:
			limpio.append(b)
	
	return limpio.get_string_from_ascii()


func _on_send_pressed() -> void:
	var texto = $LineEdit.text
	var codigos = string_a_ansi(texto)
	print(codigos)
	send(codigos)
	pass # Replace with function body.


func _on_quit_pressed() -> void:
	self.queue_free()
	pass # Replace with function body.
