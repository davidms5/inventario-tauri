// src/pages/CierresDiarios.tsx
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useNavigate } from 'react-router-dom';
import '../modules/cierres/styles/cierres.css';

type PaymentTuple = [string, number];

type PreviewResp = [number, number, PaymentTuple[]]; // (ventas_count, total, [[forma_pago, monto]...])

type Row = {
  fecha: string;
  total: number;
  ventas_count: number;
  creado_por: number;
  efectivo: number;
  tarjeta: number;
  transferencia: number;
  mercado_pago: number;
  otros: number;
};

export default function CierresDiarios() {
  const navigate = useNavigate();
  const [fecha, setFecha] = useState<string>(new Date().toISOString().slice(0,10));
  const [preview, setPreview] = useState<PreviewResp | null>(null);
  const [busy, setBusy] = useState(false);

  const [month, setMonth] = useState<string>(new Date().toISOString().slice(0,7)); // YYYY-MM
  const [formaPago, setFormaPago] = useState<string>('');
  const [rows, setRows] = useState<Row[]>([]);
  const [msg, setMsg] = useState<string>('');

  const doPreview = async () => {
    setMsg('');
    setBusy(true);
    try {
      const p = await invoke<PreviewResp>('preview_daily_closure', { dateYmd: fecha });
      setPreview(p);
    } catch (e: any) {
      console.error(e);
      setMsg('No se pudo calcular el preview.');
    } finally { setBusy(false); }
  };

  const doCreate = async () => {
    setMsg('');
    setBusy(true);
    try {
      // user_id: traelo de tu store real
      const user_id = 1;
      await invoke('create_daily_closure', { dateYmd: fecha, userId: user_id });
      setMsg('Cierre creado correctamente.');
      setPreview(null);
      await loadList();
    } catch (e: any) {
      console.error(e);
      const s = String(e);
      if (s.includes('cierre') || s.includes('existe')) {
        alert('⚠️ Ya existe un cierre para esa fecha.');
      } else {
        alert('Error creando cierre.');
      }
    } finally { setBusy(false); }
  };

  const loadList = async () => {
    setBusy(true);
    try {
      const res = await invoke<Row[]>('list_daily_closures', {
        monthYm: month,
        formaPago: formaPago || null
      });
      setRows(res);
    } catch (e) {
      console.error(e);
    } finally { setBusy(false); }
  };

  useEffect(() => { loadList(); }, [month, formaPago]);

  return (
    <div className="ventas-container">
      <h2>Cierres diarios</h2>
      <button className="btn btn-outline" onClick={() => navigate('/dashboard')}>volver al menu principal</button>

      {/* NUEVO CIERRE */}
      <section className="section-card">
        <h3>Nuevo cierre</h3>
        <div className="controls-row">
          <label className="label">Fecha</label>
          <input type="date" value={fecha} onChange={e => setFecha(e.target.value)} />
          <button className="btn" onClick={doPreview} disabled={busy}>Calcular</button>
          <button className="btn btn-primary" onClick={doCreate} disabled={busy}>Crear cierre</button>
        </div>
        {preview && (
          <div className="dashboard-kpi" style={{marginTop: 12}}>
            <div className="kpi-card"><h4>Ventas</h4><p>{preview[0]}</p></div>
            <div className="kpi-card"><h4>Total</h4><p>${preview[1].toFixed(2)}</p></div>
            <div className="kpi-card">
              <h4>Pagos</h4>
              <ul style={{margin:0,paddingLeft:16}}>
                {preview[2].map(([fp, m]) => <li key={fp}>{fp}: ${m.toFixed(2)}</li>)}
              </ul>
            </div>
          </div>
        )}
        {msg && <p>{msg}</p>}
      </section>

      {/* HISTORIAL MENSUAL */}
      <section className="section-card">
        <h3>Historial del mes</h3>
        <div className="controls-row">
          <label className="label">Mes</label>
          <input type="month" value={month} onChange={e => setMonth(e.target.value)} />
          <label className="label">Forma de pago</label>
          <select value={formaPago} onChange={e => setFormaPago(e.target.value)}>
            <option value="">Todas</option>
            <option value="efectivo">Efectivo</option>
            <option value="tarjeta">Tarjeta</option>
            <option value="transferencia">Transferencia</option>
            <option value="mercado_pago">Mercado Pago</option>
          </select>
          <button className="btn" onClick={loadList} disabled={busy}>Refrescar</button>
        </div>

        <table className="report-table">
          <thead>
            <tr>
              <th>Fecha</th><th>Ventas</th><th>Total</th>
              <th>Efectivo</th><th>Tarjeta</th><th>Transf.</th><th>M. Pago</th><th>Otros</th>
            </tr>
          </thead>
          <tbody>
            {rows.map(r => (
              <tr key={r.fecha}>
                <td>{r.fecha}</td>
                <td className="quantity">{r.ventas_count}</td>
                <td className="income">${r.total.toFixed(2)}</td>
                <td>${r.efectivo.toFixed(2)}</td>
                <td>${r.tarjeta.toFixed(2)}</td>
                <td>${r.transferencia.toFixed(2)}</td>
                <td>${r.mercado_pago.toFixed(2)}</td>
                <td>${r.otros.toFixed(2)}</td>
              </tr>
            ))}
            {!rows.length && <tr><td colSpan={8} style={{textAlign:'center'}}>Sin datos</td></tr>}
          </tbody>
        </table>
      </section>
    </div>
  );
}
