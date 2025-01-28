package main

import (
	"net/http"	
	"github.com/labstack/echo/v4"
)

// e.GET("/getTID/theaterName")
func getTID(c echo.Context) error {
  theater_name := c.QueryParam("theaterName")
  return c.String(http.StatusOK, theater_name)
}
