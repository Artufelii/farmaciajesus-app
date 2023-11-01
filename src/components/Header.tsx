import type { ReactElement } from "react"
import logo from '../assets/logo.png'
import '../css/Header.css'

export function Header(): ReactElement {
  return(
    <header className="header">
      <div className="header__img-container">
        <img src={logo} alt="Farmacia Jesus" />
      </div> 
      <nav className="header__nav">
        <ul className="header__nav-list">
          <li>Inicio</li>
          <li>Consultas</li>
          <li>Pacientes</li>
        </ul>
      </nav>
    </header>
  ) 
}
