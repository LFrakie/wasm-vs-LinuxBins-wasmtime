package main

import (
    "strconv"
    "syscall/js"
)

func main() {

    name := prompt("What's your name?")
    age := promptInt("How old are you?")

    js.Global().Get("console").Call("log", "Hello, "+name+"! You are "+strconv.Itoa(age)+" years old.")
}

func prompt(question string) string {
    js.Global().Get("console").Call("log", question)
    input := js.Global().Get("prompt").Invoke(question)
    return input.String()
}

func promptInt(question string) int {
    for {
        input := prompt(question)
        age, err := strconv.Atoi(input)
        if err == nil {
            return age
        }
        js.Global().Get("console").Call("log", "Please enter a valid age.")
    }
}

