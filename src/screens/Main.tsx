import { useRef, useState, type ReactElement, ChangeEvent, useEffect } from "react"
import { useReactToPrint} from "react-to-print"
import { Layout } from "../components"

export function Main(): ReactElement {
  const [turno, setTurno] = useState<number>(1)
  const [inputValue, setInputValue] = useState<String>("Consulta Médica")
  const turnoRef = useRef(null)

  const print=useReactToPrint({
    content: () => turnoRef.current
  })

  const selectType = (e:ChangeEvent<HTMLInputElement>) => {
    setInputValue(e.target.value)
  }

  const imprimirTurno = () => {
    const currentTurn = turno + 1
    setTurno(currentTurn)
    localStorage.setItem('turno', currentTurn.toString())
    localStorage.setItem('fecha', new Date().toLocaleDateString())
    print()
  }

  useEffect(() => {
    const fecha = localStorage.getItem('fecha')
    if (fecha !== new Date().toLocaleDateString()) {
      localStorage.clear()
    }
  }, []);

  useEffect(() => {
    const turno = localStorage.getItem('turno')
    if (turno) {
      setTurno(parseInt(turno))
    }
  }, []);

  return(
    <Layout>
      <form>
        <label htmlFor="consulta">Consulta Médica</label>
        <input 
          type="radio" 
          name="tipo" 
          id="consulta" 
          value="Consulta Médica" 
          checked={inputValue === "Consulta Médica"}
          onChange={selectType}
        />
        <label htmlFor="certificado">Certificado Médico</label>
        <input 
          type="radio" 
          name="tipo" 
          id="certificado" 
          value="Certificado Médico" 
          checked={inputValue === "Certificado Médico"} 
          onChange={selectType}
        />
      </form>
      <div ref={turnoRef}>
        <p>{inputValue}</p>
        <p>Turno:</p>
        <h1>{turno}</h1>
        <div>{new Date().toLocaleString()}</div>
      </div>
      <button onClick={imprimirTurno}>Imprimir turno</button>
    </Layout>
  ) 
}
