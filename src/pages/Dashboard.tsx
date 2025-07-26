import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store/useAuthStore';
import { ADMIN } from '../shared/constants';
import styles from '../modules/dashboard/styles/Dashboard.module.css';
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
        <div className={styles['dashboard-container']}>
            <h2>Bienvenido al panel</h2>
            <div className={styles['nav-buttons']}>
                {rol === ADMIN && (
                <button onClick={goToUsers}>Ir a Usuarios</button>
                )}
                <button onClick={() => navigate("/inventario")}>inventario</button>
                <button onClick={() => navigate("/ventas")}>ventas</button>
                <button className={styles['logout-button']}  onClick={handleLogout} style={{ marginLeft: '10px' }}>
                    Logout
                </button>
            </div>
        </div>
    );
}
