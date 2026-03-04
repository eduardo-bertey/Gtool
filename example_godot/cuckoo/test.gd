extends Control

# Configuraciones
var columnas : int = 50  # Ancho (X)
var filas : int = 30     # Alto (Y)
var tamaño_pixel : int = 15 

# Nuestra matriz (Array de Arrays)
var matriz_nodos : Array = []
var valid : Array = []
func grid():
	crear_cuadricula_rectangular(columnas, filas , valid)

func crear_cuadricula_rectangular(ancho: int, alto: int , true_val):
	matriz_nodos.clear()
	
	# Usamos un GridContainer para que acomode los 1500 cuadros
	var grid = GridContainer.new()
	grid.columns = ancho  # Seteamos el ancho de la cuadrícula
	add_child(grid)
	var count = 0
	for y in range(alto):
		var fila_actual = []
		
		for x in range(ancho):
			var cuadro = ColorRect.new()
			cuadro.custom_minimum_size = Vector2(tamaño_pixel, tamaño_pixel)
			
			# Color de base para ver la forma
			if true_val[count] == true:
			#cuadro.color = Color.DARK_GRAY if (x + y) % 2 == 0 else Color.GRAY
				cuadro.color = Color.BLUE
			else:
				cuadro.color = Color.DARK_RED
			
			
			grid.add_child(cuadro)
			fila_actual.append(cuadro)
			count += 1
		matriz_nodos.append(fila_actual)
	prints(count , " total")
	print("Cuadrícula de ", ancho, "x", alto, " creada.")
	print("Total de nodos: ", ancho * alto) # Debería dar 1500

# pintar un cuadro específico (X, Y)
func pintar_cuadro(x: int, y: int, nuevo_color: Color):
	if y < matriz_nodos.size() and x < matriz_nodos[y].size():
		matriz_nodos[y][x].color = nuevo_color
