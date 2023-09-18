import logo from "../assets/logo.png"

export function Login() {
  return(
    <form>
      <div>
        <img src={logo} alt="Farmacia Jesus" />
      </div>
      <input type="text" name="usuario" placeholder="Usuario"/>
      <input type="password" name="password" placeholder="Contraseña"/>
      <button type="submit">Inciar sesión</button>
    </form>
  )
}

