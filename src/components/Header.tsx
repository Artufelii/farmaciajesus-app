import type { ReactElement } from "react"
import logo from '../assets/logo.png'

export function Header(): ReactElement {
  return(
    <header>
      <img src={logo} alt="Farmacia Jesus" />
      <nav>consultas</nav>
      <nav>pacientes</nav>
    </header>
  ) 
}
