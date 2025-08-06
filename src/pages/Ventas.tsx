import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import "../modules/ventas/styles/ventas.css";

type VentaDetalle = {
  id?: number;
  fecha: string;
  producto?: string;
  cantidad: number;
  precio_unitario: number;
  ingresos: number;
  costo_unitario: number;
  costo_total: number;
  ganancia: number;
  estado: string;
  forma_pago?: string;
};

type PaginatedVentas = {
  data: VentaDetalle[];
  total_pages: number;
  current_page: number;
};

export default function Ventas() {
  const [ventas, setVentas] = useState<VentaDetalle[]>([]);
  const [totalPages, setTotalPages] = useState(1);
  const [currentPage, setCurrentPage] = useState(1);
  const [filterFecha, setFilterFecha] = useState<string>('');
  const [filterEstado, setFilterEstado] = useState<string>('');
  const [filterPago, setFilterPago] = useState<string>('');
  const navigate = useNavigate();

  const load = async (page = 1) => {
    try {
      const res = await invoke<PaginatedVentas>('list_sales_paginated', {
        fecha: filterFecha || null,
        estado: filterEstado || null,
        formaPago: filterPago || null,
        page,
      });
      setVentas(res.data);
      setTotalPages(res.total_pages);
      setCurrentPage(res.current_page);
    } catch (err) {
      console.error('Error cargando ventas:', err);
    }
  };

  useEffect(() => {
    load(1);
  }, [filterFecha, filterEstado, filterPago]);

  const estadoLabel = (e: string) => {
    if (e === 'pagado') return '✅ Pagado';
    if (e === 'pendiente') return '⏳ Pendiente';
    return '❌ Anulado';
  };

  return (
    <div className="ventas-container">
      <h2>Módulo de Ventas</h2>
      <button onClick={() => navigate('/dashboard')}>Dashboard</button>
      <hr />

      <section className="filtros">
        <input type="date" value={filterFecha} onChange={e => setFilterFecha(e.target.value)} />

        <select value={filterEstado} onChange={e => setFilterEstado(e.target.value)}>
          <option value="">Todos los ESTADOS</option>
          <option value="pagado">Pagado</option>
          <option value="pendiente">Pendiente</option>
          <option value="anulado">Anulado</option>
        </select>

        <select value={filterPago} onChange={e => setFilterPago(e.target.value)}>
          <option value="">Todos los PAGOS</option>
          <option value="efectivo">Efectivo</option>
          <option value="tarjeta">Tarjeta</option>
          <option value="transferencia">transferencia</option>
          <option value="mercado_pago">Mercado Pago</option>
          {/* agrega más si hay */}
        </select>

        <button onClick={() => load(1)}>Filtrar</button>
      </section>

      <section className="dashboard-kpi">
        <div className="kpi-card">
          <h3>Ventas Totales</h3>
          <p>${ventas.reduce((sum, v) => sum + v.ingresos, 0).toFixed(2)}</p>
        </div>
        <div className="kpi-card">
          <h3>Número de Filas</h3>
          <p>{ventas.length}</p>
        </div>
        <div className="kpi-card">
          <h3>Estado Pagadas</h3>
          <p>{ventas.filter(v => v.estado === 'pagado').length}</p>
        </div>
      </section>

      <table className="report-table">
        <thead>
          <tr>
            <th>Fecha</th><th>Producto</th><th>Cantidad</th><th>Precio U.</th>
            <th>Ingresos</th><th>Costo U.</th><th>Costo Total</th>
            <th>Ganancia</th><th>Estado</th><th>Pago</th>
          </tr>
        </thead>
        <tbody>
          {ventas.map(v => (
            <tr key={v.id || `${v.fecha}-${v.producto}`}>
              <td>{v.fecha}</td>
              <td>{v.producto || '-'}</td>
              <td className="quantity">{v.cantidad}</td>
              <td className="price">${v.precio_unitario.toFixed(2)}</td>
              <td className="income">${v.ingresos.toFixed(2)}</td>
              <td className="cost_unit">${v.costo_unitario.toFixed(2)}</td>
              <td className="cost_total">${v.costo_total.toFixed(2)}</td>
              <td className="profit">${v.ganancia.toFixed(2)}</td>
              <td>{estadoLabel(v.estado)}</td>
              <td>{v.forma_pago || '-'}</td>
            </tr>
          ))}
        </tbody>
      </table>

      <div className="pagination-controls">
        <button disabled={currentPage === 1} onClick={() => load(currentPage - 1)}>« Anterior</button>
        <span>Página {currentPage} de {totalPages}</span>
        <button disabled={currentPage >= totalPages} onClick={() => load(currentPage + 1)}>Siguiente »</button>
      </div>
    </div>
  );
}
