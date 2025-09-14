// src/pages/Combos.tsx
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../modules/ventas/styles/combos.css';
import { useNavigate } from 'react-router-dom';

type Combo = {
  id: number;
  nombre: string;
  descripcion?: string | null;
  price: number;
  enabled: boolean;
};

type Product = {
  id: number;
  nombre: string;
  price: number;
  quantity: number;
};

type ComboItemInput = { product_id: number; cantidad: number };

export default function Combos() {
  const [combos, setCombos] = useState<Combo[]>([]);
  const [products, setProducts] = useState<Product[]>([]);
  const [form, setForm] = useState({
    nombre: '',
    descripcion: '',
    price: '' as number | string,
    enabled: true,
  });

  const [items, setItems] = useState<ComboItemInput[]>([]); // productos del combo
  const [selProduct, setSelProduct] = useState<number | ''>('');
  const [selCantidad, setSelCantidad] = useState<number>(1);

  const [editId, setEditId] = useState<number | null>(null);
  const navigate = useNavigate();

  const loadCombos = async () => {
    const result = await invoke<Combo[]>('list_combos');
    setCombos(result);
  };
  const loadProducts = async () => {
    const result = await invoke<Product[]>('list_products_in_stock'); // o 'list_products'
    setProducts(result);
  };

  useEffect(() => { loadCombos(); loadProducts(); }, []);

  const addItem = () => {
    if (!selProduct || selCantidad <= 0) return;
    const p = products.find(x => x.id === selProduct);
    if (!p) return;
    // opcional: validar stock indicativo
    if (selCantidad > p.quantity) {
      alert('Cantidad supera el stock disponible');
      return;
    }
    setItems(prev => {
      const idx = prev.findIndex(i => i.product_id === p.id);
      if (idx >= 0) {
        const clone = [...prev];
        clone[idx] = { ...clone[idx], cantidad: clone[idx].cantidad + selCantidad };
        return clone;
      }
      return [...prev, { product_id: p.id, cantidad: selCantidad }];
    });
    setSelProduct('');
    setSelCantidad(1);
  };

  const removeItem = (product_id: number) =>
    setItems(prev => prev.filter(i => i.product_id !== product_id));

  const startEdit = async (c: Combo) => {
    setEditId(c.id);
    setForm({
      nombre: c.nombre,
      descripcion: c.descripcion || '',
      price: c.price,
      enabled: c.enabled,
    });
    // cargar items del combo
    const full = await invoke<{
      id: number; nombre: string; descripcion?: string | null; price: number; enabled: boolean;
      items: { product_id: number; cantidad: number; product_name: string }[];
    }>('get_combo_with_items', { idQuery: c.id });
    setItems(full.items.map(it => ({ product_id: it.product_id, cantidad: it.cantidad })));
  };

  const handleSubmit = async () => {
    // normalizar price
    const priceNum = typeof form.price === 'string' ? parseFloat(form.price) : form.price;
    if (!priceNum || Number.isNaN(priceNum) || priceNum < 0) {
      alert('Precio inválido');
      return;
    }
    if (!form.nombre.trim()) {
      alert('Nombre requerido');
      return;
    }
    if (items.length === 0) {
      alert('Agregá al menos un producto al combo');
      return;
    }

    if (editId) {
      // update con items
      await invoke('update_combo_with_items', {
        payload: {
          id: editId,
          nombre: form.nombre,
          descripcion: form.descripcion || null,
          price: priceNum,
          enabled: form.enabled,
          items, // [{ product_id, cantidad }]
        }
      });
    } else {
      // create con items
      await invoke('create_combo_with_items', {
        payload: {
          combo: {
            nombre: form.nombre,
            descripcion: form.descripcion || null,
            price: priceNum,
            enabled: form.enabled,
          },
          items, // [{ product_id, cantidad }]
        }
      });
    }

    setEditId(null);
    setForm({ nombre: '', descripcion: '', price: '', enabled: true });
    setItems([]);
    await loadCombos();
  };

  const handleDelete = async (id: number) => {
    if (!confirm('¿Eliminar este combo?')) return;
    await invoke('delete_combo', { idToDelete: id });
    await loadCombos();
  };

  return (
    <div className="combo-container">
      <h2>Gestión de Combos</h2>
      <button onClick={() => navigate('/dashboard')} style={{ backgroundColor: "#007bff" }}>
        menu principal
      </button>

      <div className="form-section">
        <h3 style={{ color: "black" }}>{editId ? 'Editar Combo' : 'Crear Combo'}</h3>

        <label>Nombre</label>
        <input
          placeholder="Nombre"
          value={form.nombre}
          onChange={e => setForm({ ...form, nombre: e.target.value })}
        />

        <label>Descripción</label>
        <input
          placeholder="Descripción"
          value={form.descripcion}
          onChange={e => setForm({ ...form, descripcion: e.target.value })}
        />

        <label>Precio total</label>
        <input
          type="number"
          placeholder="Precio total"
          value={form.price}
          onChange={e => setForm({ ...form, price: e.target.value })}
        />

        <label>
          <input
            type="checkbox"
            checked={form.enabled}
            onChange={e => setForm({ ...form, enabled: e.target.checked })}
          />
          Activo
        </label>

        {/* Picker de productos para el combo */}
        <div className="combo-items-picker">
          <h4 style={{ color: "black" }}>Agregar productos al combo</h4>
          <div style={{ display: 'flex', gap: 8, alignItems: 'center', marginBottom: 8 }}>
            <select
              value={selProduct}
              onChange={e => setSelProduct(e.target.value ? Number(e.target.value) : '')}
            >
              <option value="">Seleccione producto</option>
              {products.map(p => (
                <option key={p.id} value={p.id}>
                  {p.nombre} — ${p.price.toFixed(2)} (stock: {p.quantity})
                </option>
              ))}
            </select>
            <input
              type="number"
              min={1}
              value={selCantidad}
              onChange={e => {
                const v = Math.max(1, parseInt(e.target.value || '1'));
                const p = products.find(x => x.id === selProduct);
                const max = p ? p.quantity : v;
                setSelCantidad(Math.min(v, max));
              }}
              style={{ width: 90 }}
            />
            <button onClick={addItem}>Agregar</button>
          </div>

          {/* Lista de items del combo */}
          <table className="ventas-table">
            <thead>
              <tr><th style={{color: "black"}}>Producto</th><th style={{color: "black"}}>Cantidad</th><th></th></tr>
            </thead>
            <tbody>
              {items.map((it) => {
                const p = products.find(x => x.id === it.product_id);
                return (
                  <tr key={it.product_id}>
                    <td style={{color: "black"}}>{p ? p.nombre : it.product_id}</td>
                    <td style={{color: "black"}}>{it.cantidad}</td>
                    <td>
                      <button onClick={() => removeItem(it.product_id)}>Quitar</button>
                    </td>
                  </tr>
                );
              })}
              {items.length === 0 && (
                <tr><td colSpan={3} style={{ textAlign: 'center', color: "darkgrey" }}>Sin productos en el combo</td></tr>
              )}
            </tbody>
          </table>
        </div>

        <div style={{ marginTop: 10 }}>
          <button onClick={handleSubmit}>{editId ? 'Guardar' : 'Crear'}</button>
          {editId && (
            <button onClick={() => {
              setEditId(null);
              setForm({ nombre: '', descripcion: '', price: '', enabled: true });
              setItems([]);
            }} style={{ marginLeft: 8 }}>
              Cancelar
            </button>
          )}
        </div>
      </div>

      <ul className="combo-list">
        {combos.map(c => (
          <li key={c.id}>
            <span style={{ color: "black" }}>
              {c.nombre} — ${c.price.toFixed(2)} {c.enabled ? '' : '(Inactivo)'}
            </span>
            <div className="actions">
              <button onClick={() => startEdit(c)}>Editar</button>
              <button onClick={() => handleDelete(c.id)}>Eliminar</button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
