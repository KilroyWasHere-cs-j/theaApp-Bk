package main

import (
  "net/http"
  "github.com/labstack/echo/v4"
)

func index(c echo.Context) error {
  return c.String(http.StatusOK, "This is the base route for the Thea API")
}

func info(c echo.Context) error {
  return c.String(http.StatusOK, "Info")
}
