import { useNavigate } from 'react-router-dom';
import { useAuthStore } from '../store/useAuthStore';
import { ADMIN } from '../shared/constants';
import styles from '../modules/dashboard/styles/Dashboard.module.css';
import { invoke } from '@tauri-apps/api/core';
import type { TodaySummary } from '../modules/dashboard/types';
import { useState, useEffect } from 'react';

export default function Dashboard() {

    const navigate = useNavigate();
    const rol = useAuthStore((state) => state.rol);

    const [openResumen, setOpenResumen] = useState(false);
    const [loading, setLoading] = useState(false);
    const [summary, setSummary] = useState<TodaySummary | null>(null);
    //const refreshTimer = useRef<number | null>(null);

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

    async function loadSummary() {
        try {
            setLoading(true);
            const data = await invoke<TodaySummary>('get_today_sales_summary');
            setSummary(data);
        } catch (e) {
            console.error(e);
        } finally {
            setLoading(false);
        }
    }

    useEffect(() => {
        if (openResumen) {
        loadSummary();
        }

  }, [openResumen]);

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
                {rol === ADMIN && <button onClick={() => navigate("/cierres-diarios")}>Cierres diarios</button>}

                <button onClick={() => setOpenResumen(true)}>Resumen de hoy</button>

                <button className={styles['logout-button']}  onClick={handleLogout} style={{ marginLeft: '10px' }}>
                    Logout
                </button>
            </div>

             {/* Overlay */}
                {openResumen && (
                    <div className={styles.overlay}>
                    <div className={styles.overlayCard}>
                        <div className={styles.overlayHeader}>
                        <h3>Ventas de hoy</h3>
                        <button onClick={() => setOpenResumen(false)}>✕</button>
                        </div>

                        {loading && <p>Cargando…</p>}

                        {!loading && summary && (
                        <div className={styles.resumenGrid}>
                            <div className={styles.kpiCard}>
                            <div className={styles.kpiLabel}>Ventas realizadas</div>
                            <div className={styles.kpiValue}>{summary.ventas_count}</div>
                            </div>
                            <div className={styles.kpiCard}>
                            <div className={styles.kpiLabel}>Total acumulado</div>
                            <div className={styles.kpiValue}>${summary.total_dia.toFixed(2)}</div>
                            </div>

                            {/* Desglose por forma de pago (opcional)
                            <div className={styles.kpiSpan}>
                            <h4>Por forma de pago</h4>
                            {summary.por_forma_pago.length === 0 ? (
                                <p>Sin pagos registrados aún.</p>
                            ) : (
                                <table className={styles.reportTable}>
                                <thead><tr><th>Forma de pago</th><th>Monto</th></tr></thead>
                                <tbody>
                                    {summary.por_forma_pago.map((r, i) => (
                                    <tr key={i}>
                                        <td>{r.forma_pago}</td>
                                        <td>${r.monto.toFixed(2)}</td>
                                    </tr>
                                    ))}
                                </tbody>
                                </table>
                            )}
                            </div>
                             */}
                        </div>
                        )}
                    </div>
                    </div>
                )}
        </div>
    );
}
