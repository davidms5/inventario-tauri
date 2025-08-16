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
        useAuthStore.getState().setRol(null);
        useAuthStore.getState().setUserId(null);
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
                {/**TODO: aqui ver tema de logica condicional, si el usuario es admin || usuario empleado tiene el flag habilitado para hacer crud de productos */}
                {rol === ADMIN && <button onClick={() => navigate("/inventario")}>inventario</button>}
                <button onClick={() => navigate("/ventas")}>nueva venta</button>
                {rol === ADMIN && <button onClick={() => navigate("/ventas-admin")}>ventas admin</button>}
                {rol === ADMIN && <button onClick={() => navigate("/combos")}>Combos</button>}
                <button className={styles['logout-button']}  onClick={handleLogout} style={{ marginLeft: '10px' }}>
                    Logout
                </button>
            </div>
        </div>
    );
}
