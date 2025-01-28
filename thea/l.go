package main

import (
  "log"
  "os"
)

const logName = "main.log"

func lPush(message string) bool {
  file, err := os.OpenFile("main.log", os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)

  if err != nil {
    log.Fatal(err)
    return false
  }
  defer file.Close()

  log.SetOutput(file)
  log.Println(message)
  return true
}
