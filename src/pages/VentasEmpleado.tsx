// src/pages/VentaNueva.tsx
import { useNavigate } from 'react-router-dom';
import { useNuevaVenta } from '../modules/ventas/hooks/VentasEmpleadoHooks';
import { confirm as dialogConfirm } from '@tauri-apps/plugin-dialog';
import { useAuthStore } from '../store/useAuthStore';
import "../modules/ventas/styles/ventasEmpleados.css";
import { invoke } from '@tauri-apps/api/core';
import { generateAndSaveSalePdf } from '../modules/ventas/utils/pdf';
import type { Product } from "../modules/ventas/hooks/VentasEmpleadoHooks";
import { useState, useEffect } from 'react';

export default function VentaNueva() {
  const navigate = useNavigate();
  const userId = useAuthStore(state => state.user_id);
  const enabledAddProducts = useAuthStore(state => state.enabled_add_products);

  const [prodQuery, setProdQuery] = useState("");
  const [prodOptions, setProdOptions] = useState<Product[]>([]);
  const [searchBusy, setSearchBusy] = useState(false);
  const [isClosedToday, setIsClosedToday] = useState<boolean>(false);

    useEffect(() => {
    // chequeo al montar (y podrías re-chequear cada X mins si querés)
    invoke<boolean>("is_date_closed", { dateYmd: null })
      .then(setIsClosedToday)
      .catch(() => setIsClosedToday(false));
  }, []);

  // búsqueda con debounce cuando el usuario escribe
  useEffect(() => {
    const q = prodQuery.trim();

    if (/^#\d+\s/.test(q)) return;

    const t = setTimeout(async () => {
      if (!q) { setProdOptions([]); return; }
      try {
        setSearchBusy(true);
        const res = await invoke<Product[]>("search_products_in_stock", { query: q });
        console.log("Productos encontrados:", res);
        setProdOptions(res);
      } finally {
        setSearchBusy(false);
      }
    }, 250);
    return () => clearTimeout(t);
  }, [prodQuery]);

  // auxiliar: extrae el ID del valor elegido (#123 Nombre)
  const parseId = (txt: string) => {
    const m = txt.match(/^#(\d+)\s/);
    return m ? Number(m[1]) : null;
  };

  const {
    combos, 
    selCombo, setSelCombo, cantCombo, setCantCombo, addCombo,
     cantProd, setCantProd, addProductDirect,
    cart, removeLine,
    pago, setPago, cashReceived, setCashReceived,
    total,
    confirmSale, confirmDisabled, reset
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
    } catch (e: any) {
      console.error(e);
      const msg = String(e);
      if (msg.startsWith("ERR_DAY_CLOSED")) {
        setIsClosedToday(true);
        alert("El cierre diario ya fue realizado. No se pueden crear más ventas hoy.");
        return;
      }
      alert('Error creando la venta: ' + (e as any).toString());
    }
  };

  return (
    <div className="ventas-container">
      <h2>Módulo de Venta — {new Date().toLocaleString()}</h2>
      <button className="btn btn-outline" onClick={() => navigate('/dashboard')}>volver al menu principal</button>
      

      {/* PRODUCTOS */}

        {/* Input de búsqueda */}
        {/*<div className="toolbar">
        <input
          type="text"
          placeholder="Buscar por nombre o SKU"
          value={searchTerm}
          onChange={e => setSearchTerm(e.target.value)}
          style={{ marginRight: 8, width: 240 }}
        />
        <button onClick={searchProducts} className="btn">Buscar</button>
        </div>*/}

<section className="section-card">
  <div className="controls-row">
    <strong className="label">Productos</strong>

    {/* Campo único: al teclear se buscan sugerencias */}
    <input
      list="product-options"
      placeholder="Buscar y seleccionar producto"
      value={prodQuery}
      onChange={(e) => setProdQuery(e.target.value)}
      style={{ minWidth: 260, marginRight: 8 }}
    />
    <datalist id="product-options">
      {prodOptions
        .filter(p => p.quantity > 0) // solo con stock
        .map(p => (
          // mostramos "#id Nombre", así luego podemos recuperar el id
          <option key={p.id} value={`#${p.id} ${p.nombre}`} />
        ))
      }
    </datalist>

    {/* cantidad */}
    <input
      type="number"
      min={1}
      value={cantProd}
      onChange={e => setCantProd(Math.max(1, parseInt(e.target.value || "1")))}
      style={{ width: 90, marginRight: 8 }}
    />

    {/* agregar usando lo escrito/seleccionado en el input */}
    <button
      className="btn btn-primary"
      onClick={async () => {
          const id = parseId(prodQuery);
          if (!id) { alert("Elegí un producto de la lista"); return; }

          // Busca en las opciones actuales
          let p = prodOptions.find(x => x.id === id);

          // Fallback por si el efecto vació prodOptions:
          //if (!p) {
          //  try {
          //    p = await invoke<Product | null>("get_product_in_stock_by_id", { id }) as Product | null;
          //  } catch {}
          //}

          if (!p) { alert("No se encontró el producto seleccionado"); return; }
          if (p.quantity <= 0) { alert("Sin stock"); return; }
          if (cantProd > p.quantity) { alert("Cantidad supera stock"); return; }

          addProductDirect(p, cantProd);
          setProdQuery("");
        }}
        disabled={searchBusy || !prodQuery}
    >
      Agregar producto
    </button>
  </div>
</section>
        
            {/* COMBOS */}
      <section className="section-card">
        <div className="controls-row">
        <strong className='label'>Combos activos</strong>
        <select value={selCombo} onChange={e => setSelCombo(e.target.value ? Number(e.target.value) : '')} disabled={!enabledAddProducts} style={{ marginLeft: 8, minWidth: 260, marginRight: 8 }}>
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
          disabled={!enabledAddProducts}
        />
        <button onClick={addCombo} disabled={!enabledAddProducts}>Agregar combo</button>
        </div>
      </section>

      {/* Carrito */}
      <table className="report-table">
        <thead>
          <tr>
            <th>Nombre</th><th>Cant.</th><th>Precio U.</th><th>Subtotal</th><th></th>
          </tr>
        </thead>
        <tbody>
          {cart.map((it, idx) => (
            <tr key={idx}>
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

        <button style={{ marginLeft: 12 }} disabled={confirmDisabled || isClosedToday} onClick={handleConfirm}>
          Confirmar venta
        </button>
      </div>

        {isClosedToday && (
          <div className="overlay">
            <div className="overlay-card">
              <h3>Ventas cerradas</h3>
              <p>El cierre diario ya fue realizado hoy. No se pueden registrar más ventas.</p>
              <button className="btn" onClick={() => navigate("/dashboard")}>
                Volver al menú
              </button>
            </div>
          </div>
        )}
    </div>
  );
}
