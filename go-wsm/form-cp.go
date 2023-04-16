package main

import (
    "bufio"
    "fmt"
    "os"
)

func main() {
    reader := bufio.NewReader(os.Stdin)

    fmt.Print("Ingrese su nombre: ")
    name, _ := reader.ReadString('\n')
    name = name[:len(name)-1]

    fmt.Print("Ingrese su edad: ")
    var age int
    fmt.Scan(&age)

    fmt.Print("Ingrese su correo electrónico: ")
    email, _ := reader.ReadString('\n')
    email = email[:len(email)-1]

    fmt.Printf("Hola %s, tu edad es %d y tu correo electrónico es %s\n", name, age, email)
}

