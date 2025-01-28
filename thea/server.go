package main

import (
  "fmt"
  "github.com/labstack/echo/v4"
)

func main() {
  logGood := lPush("--------------------------------------------Tare--------------------------------------------")
  lPush("Log started")
  if logGood {
    lPush("Server started")
    e := echo.New()

    e.GET("/", index)
    e.GET("/info", info)
    e.GET("/getTID/:tNam", getTID)
    e.GET("/getTName/:tid", getTName)
    
    e.Logger.Fatal(e.Start(":1323"))
  } else {
    fmt.Println("A terrible error has happened and stuff is on fire")
  }
  fmt.Println("Done")
}
