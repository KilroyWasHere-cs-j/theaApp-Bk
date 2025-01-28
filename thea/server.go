package main

import (
  "github.com/labstack/echo/v4"
)

func main() {
  e := echo.New()
  e.GET("/", index)
  e.GET("/info", info)
  e.GET("/getTID/<theaterName>")
  e.Logger.Fatal(e.Start(":1323"))
}
