package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	fmt.Print("Ingrese su nombre: ")
	name, _ := reader.ReadString('\n')
	name = strings.TrimSuffix(name, "\n")

	fmt.Println("Elija una opción:")
	fmt.Println("1. Saludar")
	fmt.Println("2. Despedirse")

	option := readOption(reader)

	switch option {
	case 1:
		fmt.Printf("Hola %s!\n", name)
	case 2:
		fmt.Printf("Adiós %s!\n", name)
	default:
		fmt.Println("Opción inválida")
	}
}

func readOption(reader *bufio.Reader) int {
	for {
		fmt.Print("Ingrese su opción: ")
		option, err := reader.ReadString('\n')
		if err != nil {
			fmt.Println("Error al leer opción")
			continue
		}

		option = strings.TrimSuffix(option, "\n")
		switch option {
		case "1":
			return 1
		case "2":
			return 2
		default:
			fmt.Println("Opción inválida")
		}
	}
}

