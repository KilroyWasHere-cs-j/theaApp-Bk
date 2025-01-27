package main

import (
  "net/http"
  "github.com/labstack/echo/v4"
  "fmt"
  "thea/api"
)

func main() {
  e := echo.New()
  e.GET("/", api.Index) 
  e.Logger.Fatal(e.Start(":1323"))
}
