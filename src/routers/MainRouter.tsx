import type { ReactElement } from "react"
import { Routes, Route } from "react-router-dom"
import { Main, Login } from "../screens"

export function MainRouter(): ReactElement {
  return (
    <Routes>
      <Route path="/" element={<Main />} />
      <Route path="login" element={<Login />} />
    </Routes>
  ) 
}
