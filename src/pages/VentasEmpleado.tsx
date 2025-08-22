// src/pages/VentaNueva.tsx
import { useNavigate } from 'react-router-dom';
import { useNuevaVenta } from '../modules/ventas/hooks/VentasEmpleadoHooks';
import { confirm as dialogConfirm } from '@tauri-apps/plugin-dialog';
import { useAuthStore } from '../store/useAuthStore';
import "../modules/ventas/styles/ventasEmpleados.css";
import { invoke } from '@tauri-apps/api/core';
import { generateAndSaveSalePdf } from '../modules/ventas/utils/pdf';

export default function VentaNueva() {
  const navigate = useNavigate();
  const userId = useAuthStore(state => state.user_id);
  const {
    combos, products,
    selCombo, setSelCombo, cantCombo, setCantCombo, addCombo,
    selProduct, setSelProduct, cantProd, setCantProd, addProduct,
    cart, removeLine,
    pago, setPago, cashReceived, setCashReceived,
    total,
    confirmSale, confirmDisabled, reset, searchTerm, setSearchTerm, searchProducts
  } = useNuevaVenta({ userId });

  const handleConfirm = async () => {
    //console.log({ userId });
    const seguro = await dialogConfirm("¿Está seguro de confirmar la venta?", {
      title: "Confirmar venta",
      kind: "warning",
      okLabel: "Sí",
      cancelLabel: "Cancelar",
    });
    
    if (!seguro) return;

    try {

      if (pago === 'efectivo') {
        const received = typeof cashReceived === 'number' ? cashReceived : 0;
        if (received < total) {
          alert('El efectivo recibido no puede ser menor al total.');
          return;
        }
      }
      
      const res = await confirmSale();
      if (!res) return;

    
      const msg = pago === 'efectivo' && res.change > 0
        ? `Venta realizada. Cambio: $${res.change.toFixed(2)}`
        : 'Venta realizada exitosamente';
      
        // Traemos la venta con sus items para imprimir
      const sale = await invoke<{ sale: any; items: any[] }>('get_sale', { id: res.sale_id });
      
      alert(msg);
      
      await generateAndSaveSalePdf(sale, res.change);
      reset();
      navigate('/ventas');
    } catch (e) {
      console.error(e);
      alert('Error creando la venta');
    }
  };

  return (
    <div className="ventas-container">
      <h2>Módulo de Venta — {new Date().toLocaleString()}</h2>
      <button className="btn btn-outline" onClick={() => navigate('/dashboard')}>Dashboard</button>
      

      {/* PRODUCTOS */}

        {/* Input de búsqueda */}
        <div className="toolbar">
        <input
          type="text"
          placeholder="Buscar por nombre o SKU"
          value={searchTerm}
          onChange={e => setSearchTerm(e.target.value)}
          style={{ marginRight: 8, width: 240 }}
        />
        <button onClick={searchProducts} className="btn">Buscar</button>
        </div>

      <section className="section-card">
        <div className="controls-row">
        <strong className='label'>Productos</strong>

        {/* Select vacío por defecto; se llena con el resultado */}
        <select
          value={selProduct}
          onChange={e => setSelProduct(e.target.value ? Number(e.target.value) : '')}
          style={{ marginRight: 8, minWidth: 260 }}
        >
          <option value="">Seleccione producto</option>
          {products
            .filter(p => p.quantity > 0)
            .map(p => (
              <option key={p.id} value={p.id}>
                {p.nombre} — ${p.price.toFixed(2)} (stock: {p.quantity})
              </option>
            ))}
        </select>

        <input
          type="number"
          min={1}
          value={cantProd}
          onChange={e => {
            const val = Math.max(1, parseInt(e.target.value || '1'));
            const p = products.find(x => x.id === selProduct);
            const max = p ? p.quantity : val;
            setCantProd(Math.min(val, max));
          }}
          style={{ width: 90, marginRight: 8 }}
        />

        <button
        className="btn btn-primary"
          onClick={addProduct}
          disabled={
            !selProduct ||
            !products.find(p => p.id === selProduct && p.quantity > 0) ||
            cantProd <= 0
          }
        >
          Agregar producto
        </button>
        </div>
      </section>
        
            {/* COMBOS */}
      <section className="section-card">
        <div className="controls-row">
        <strong className='label'>Combos activos</strong>
        <select value={selCombo} onChange={e => setSelCombo(e.target.value ? Number(e.target.value) : '')}>
          <option value="">Seleccione combo</option>
          {combos.map(c => (
            <option key={c.id} value={c.id}>{c.nombre} — ${c.price.toFixed(2)}</option>
          ))}
        </select>
        <input
          type="number"
          min={1}
          value={cantCombo}
          onChange={e => setCantCombo(Math.max(1, parseInt(e.target.value || '1')))}
        />
        <button onClick={addCombo}>Agregar combo</button>
        </div>
      </section>

      {/* Carrito */}
      <table className="report-table">
        <thead>
          <tr>
            <th>Tipo</th><th>Nombre</th><th>Cant.</th><th>Precio U.</th><th>Subtotal</th><th></th>
          </tr>
        </thead>
        <tbody>
          {cart.map((it, idx) => (
            <tr key={idx}>
              <td>{it.kind}</td>
              <td>{it.nombre}</td>
              <td className="quantity">{it.cantidad}</td>
              <td className="price">${it.price.toFixed(2)}</td>
              <td className="income">${(it.price * it.cantidad).toFixed(2)}</td>
              <td><button onClick={() => removeLine(idx)}>Quitar</button></td>
            </tr>
          ))}
          {cart.length === 0 && (
            <tr><td colSpan={6} style={{ textAlign: 'center' }}>Sin ítems</td></tr>
          )}
        </tbody>
        <tfoot>
          <tr>
            <td colSpan={4} style={{ textAlign: 'right' }}><strong>Total:</strong></td>
            <td className="income"><strong>${total.toFixed(2)}</strong></td>
            <td />
          </tr>
        </tfoot>
      </table>

      {/* Pago */}
      <div className='pay-row'>
        <label>Forma de pago: </label>
        <select
          value={pago}
          onChange={e => {
            const v = e.target.value as typeof pago;
            setPago(v);
            if (v !== 'efectivo') setCashReceived('');
          }}
        >
          <option value="efectivo">Efectivo</option>
          <option value="tarjeta">Tarjeta</option>
          <option value="transferencia">Transferencia</option>
          <option value="mercado_pago">Mercado Pago</option>
        </select>

        {pago === 'efectivo' && (
          <span style={{ marginLeft: 12 }}>
            <label>Recibido: </label>
            <input
              type="number"
              step="0.01"
              value={cashReceived === '' ? '' : cashReceived}
              onChange={e => {
                const raw = e.target.value;
                if (raw === '') { setCashReceived(''); return; }
                const parsed = parseFloat(raw.replace(',', '.'));
                if (!Number.isNaN(parsed)) setCashReceived(parsed);
              }}
              style={{ width: 120, marginLeft: 6 }}
            />
            <span style={{ marginLeft: 12 }}>
              {typeof cashReceived === 'number' && cashReceived >= total
                ? <>Cambio: <strong>${(cashReceived - total).toFixed(2)}</strong></>
                : null}
            </span>
          </span>
        )}

        <button style={{ marginLeft: 12 }} disabled={confirmDisabled} onClick={handleConfirm}>
          Confirmar venta
        </button>
      </div>
    </div>
  );
}
