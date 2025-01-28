package main

import (
	"net/http"	
	"github.com/labstack/echo/v4"
)

// /getTID/:tName
func getTID(c echo.Context) error {
  tNam := c.Param("tNam")
  lM := "GET -> GET TID [" + tNam + "]"
  lPush(lM)
  return c.String(http.StatusOK, tNam)
}

// /getTName/:tid
func getTName(c echo.Context) error {
  tid := c.Param("tid")
  lM := "GET -> GET TNAME [" + tid + "]"
  lPush(lM)
  return c.String(http.StatusOK, tid)
}
