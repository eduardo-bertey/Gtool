extends Node

func _ready():
	# Instanciamos el ToolSec (registrado como GodotClass)
	var tool_sec = ToolSec.new()
	
	# Parámetros: 
	# path (res:// o ruta absoluta), seed (GString), overwrite (bool), run (bool)
	
	var script_path = "res://pepito.gd"
	#var global_path = "C:/ejemplo_gigante.txt"
	var my_seed = "mi_clave_secreta_123"
	
	print("--- Ejemplo de Cifrado de Script y Carga Dinámica ---")
	
	# MODO 1: Cifrar un script y ejecutarlo desde memoria
	# Activamos 'run' para que lo descifre y lo añada al SceneTree como Nodo.
	# Activamos 'overwrite' si queremos que el archivo en disco quede con el nuevo estado (cifrado/descifrado).
	var result = tool_sec.encode(script_path, my_seed, false, true)
	
	if result:
		print("Script procesado y cargado con éxito. Objeto devuelto: ", result)
	else:
		print("Error al procesar o cargar el script.")
		
	# MODO 2: Cifrar/Descifrar un archivo gigante paso a paso (Streaming)
	# Usamos una ruta global y solo activamos 'overwrite'. 'run' en false.
	# Esto soporta archivos de hasta 100GB sin saturar la RAM.
	# print("--- Procesando archivo gigante (Paso a paso) ---")
	# var big_file_result = tool_sec.encode(global_path, my_seed, true, false)
	# if big_file_result:
	# 	print("Archivo gigante procesado correctamente.")

	# Limpiamos el tool si no lo necesitamos más (siendo un Nodo, free() o queue_free())
	tool_sec.free()
