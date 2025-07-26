import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store/useAuthStore';
import { ADMIN } from '../shared/constants';

export default function Dashboard() {

    const navigate = useNavigate();
    const rol = useAuthStore((state) => state.rol);

    const handleLogout = () => {
    // Acá podés agregar limpiar estado global o sesión
        useAuthStore.getState().setUser(null);
        navigate('/login', { replace: true });
    };

    const goToUsers = () => {
        navigate('/usuarios');
    };

    return (
        <div>
        <h2>Bienvenido al panel</h2>
        {rol === ADMIN && (
        <button onClick={goToUsers}>Ir a Usuarios</button>
        )}
        <button onClick={() => navigate("/inventario")}>inventario</button>
        <button onClick={handleLogout} style={{ marginLeft: '10px' }}>
            Logout
        </button>
        </div>
    );
}
