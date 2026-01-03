extends Control

var timer
var relay_url: Array[String] = [
	#"wss://relay.mostro.network",
	#"wss://nos.lol",
	#"wss://relay.damus.io"
]
var platform
var save_name = "relay.dat"
var save_path# = SAVE_DIR 
var local = "res://"


func _ready() -> void:
	
	platform = OS.get_name()
	if platform == "Android":
		local = "user://"
		print("Estamos en Android")
	
	#timer = Timer.new()
	#timer.wait_time = 3.0  # Consultar cada 3 segundos
	#timer.connect("timeout", Callable(self, "list_relays"))
	#add_child(timer)
	#timer.start()
	#print("📡 Polling iniciado (cada 3s)...")
	load_data()



# Quitar relay por posición (índice)
func remove_relay_at(index: int) -> void:
	if index >= 0 and index < relay_url.size():
		relay_url.remove_at(index)
		list_relays()
	else:
		push_error("Índice fuera de rango")

# Agregar un relay al final
func add_relay(url: String) -> void:
	if not relay_url.has(url):
		relay_url.append(url)
	else:
		push_warning("Relay ya existe: %s" % url)
	list_relays()


# Listar todos los relays
func list_relays() -> void:
	$ScrollContainer/relays.text = ""
	for i in range(relay_url.size()):
		$ScrollContainer/relays.text += "\n Nº: " + str(i) + " # relay : " + str(relay_url[i])
		print("%d: %s" % [i, relay_url[i]])


func _on_addrelay_pressed() -> void:
	add_relay($editerelay.text)
	prints("add relauys ", $editerelay.text)
	pass # Replace with function body.


func _on_quitrelay_pressed() -> void:
	var text_int = $editerelay/indexrelay.text
	var index = int(text_int)
	if text_int == "":
		prints("agrega un numero")
		return
	
	if index <= relay_url.size():
		pass
	else:
		return
	
	remove_relay_at(index)
	prints("quit relay :" ,index)


func load_data():
	#if $LineEdit.text == "":
		#prints("error no ingreso pass")
	save_path = local + save_name
	var file = FileAccess.open_encrypted_with_pass(save_path, FileAccess.READ, "1234")
	if file == null:
		list_relays()
		return
	if file.file_exists(save_path):
		var error = file.get_open_error()
		if error == OK:
			var data = file.get_var()
			self.relay_url = data
			prints(data)
			file.close()
			prints()
	list_relays()

func save_data():
	#if $LineEdit.text == "":
		#prints("error no ingreso pass")
	save_path = local + save_name
	var file = FileAccess.open_encrypted_with_pass(save_path, FileAccess.WRITE,"1234")
	if file == null:
		return
	var error = file.get_open_error()
	if error == OK:
		#if file.file_exists(save_path):
			#prints("el archivo existe editalo o eliminalo primero ")
			#file.close()
			#return
		file.store_var(relay_url)
		#prints(file.get_sha256(save_path))
		file.close()
	prints(relay_url.size())
	prints("data saved")


func _on_button_pressed() -> void:
	save_data()
	pass # Replace with function body.


func _on_button_2_pressed() -> void:
	load_data()
	
	pass # Replace with function body.
