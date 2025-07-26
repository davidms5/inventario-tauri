import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import  "../modules/ventas/styles/ventas.css"
type Venta = {
  id: number;
  fecha: string;
  cliente: string;
  total: number;
  estado: 'pendiente' | 'pagado' | 'anulado';
};

export default function Ventas() {
  const [ventas, setVentas] = useState<Venta[]>([]);
  const [filterFecha, setFilterFecha] = useState<string>('');
  const [filterEstado, setFilterEstado] = useState<string>('');
  const navigate = useNavigate();

  const load = async () => {
    const list = await invoke<Venta[]>('list_sales', {
      fecha: filterFecha || null,
      estado: filterEstado || null
    });
    setVentas(list);
  };

  useEffect(() => { load(); }, [filterFecha, filterEstado]);

  const estadoLabel = (e: string) => {
    if (e === 'pagado') return '✅ Pagado';
    if (e === 'pendiente') return '⏳ Pendiente';
    return '❌ Anulado';
  };

  return (
    <div className="ventas-container">
      <h2>Módulo de Ventas</h2>
      <button onClick={() => navigate("/dashboard")}>dashboard</button>
        <hr />
      <section className="filtros">
        <input type="date" value={filterFecha}
          onChange={e => setFilterFecha(e.target.value)} />
        <select value={filterEstado}
          onChange={e => setFilterEstado(e.target.value)}>
          <option value="">Todos</option>
          <option value="pagado">Pagado</option>
          <option value="pendiente">Pendiente</option>
          <option value="anulado">Anulado</option>
        </select>
        <button onClick={load}>Filtrar</button>
      </section>

      <section className="dashboard-kpi">
        <div className="kpi-card">
          <h3>Ventas Totales</h3>
          <p>${ventas.reduce((sum, v) => sum + v.total, 0).toFixed(2)}</p>
        </div>
        <div className="kpi-card">
          <h3>Número de Ventas</h3>
          <p>{ventas.length}</p>
        </div>
        <div className="kpi-card">
          <h3>Pagadas</h3>
          <p>{ventas.filter(v => v.estado === 'pagado').length}</p>
        </div>
      </section>

      <table className="ventas-table">
        <thead>
          <tr>
            <th>ID</th>
            <th>Fecha</th>
            <th>Cliente</th>
            <th>Total</th>
            <th>Estado</th>
          </tr>
        </thead>
        <tbody>
          {ventas.map(v => (
            <tr key={v.id}>
              <td>{v.id}</td>
              <td>{v.fecha}</td>
              <td>{v.cliente}</td>
              <td>${v.total.toFixed(2)}</td>
              <td>{estadoLabel(v.estado)}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
