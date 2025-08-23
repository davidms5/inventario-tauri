import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
//import reactLogo from "./assets/react.svg";
//import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Login from "./pages/Login";
import Dashboard from "./pages/Dashboard";
import { useAuthStore } from './store/useAuthStore';
import Usuarios from './pages/Usuarios';
import { ADMIN } from './shared/constants';
import Inventario from './pages/Inventario';
import Ventas from './pages/Ventas';
import Combos from './pages/Combos';
import VentaNueva from './pages/VentasEmpleado';
import CierresDiarios from './pages/CierresDiarios';

function App() {
  //const [loggedUser, setLoggedUser] = useState<string | null>(null);
  //const [greetMsg, setGreetMsg] = useState("");
  //const [name, setName] = useState("");

  //async function greet() {
  //  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //  setGreetMsg(await invoke("greet", { name }));
  //}
  const user = useAuthStore((state) => state.user);
  const rol = useAuthStore((state) => state.rol);
  return (
    <>
    <BrowserRouter>
      <Routes>
        <Route
          path="/login"
          element={!user ? <Login onLogin={(u) => useAuthStore.getState().setUser(u)} /> : <Navigate to="/dashboard" replace />}
        />
        <Route
          path="/dashboard"
          element={user ? <Dashboard /> : <Navigate to="/login" replace />}
        />
        <Route
          path="/usuarios"
          element={user && rol === ADMIN ? <Usuarios /> : <Navigate to="/dashboard" replace />}
        />
        <Route 
        path='/inventario'
        element={user ? <Inventario/> : <Navigate to="/dashboard" replace />}/>
        <Route
          path="/ventas-admin"
          element={user && rol === ADMIN ? <Ventas /> : <Navigate to="/dashboard" replace />}
        />
        <Route
          path="/combos"
          element={user && rol === ADMIN ? <Combos /> : <Navigate to="/dashboard" replace />} />

        <Route
          path="/ventas"
          element={user ? <VentaNueva /> : <Navigate to="/dashboard" replace />}/>

        <Route 
          path="/cierres-diarios"
          element={user && rol === ADMIN ? <CierresDiarios /> : <Navigate to="/dashboard" replace />}
        />

        <Route path="*" element={<Navigate to={user ? '/dashboard' : '/login'} replace />} />
      </Routes>
    </BrowserRouter>
    </>
  );
}

export default App;
