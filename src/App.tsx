import { invoke } from "@tauri-apps/api";
import "./App.css";

function App() {

  const getPaciente = async () => {
    const reponse = await invoke("get_paciente", {pacienteId: 2})
    console.log(reponse)
  }

  return (
    <div className="container">
      <button onClick={getPaciente}>Cargar pacientes</button>
    </div>
  );
}

export default App;
