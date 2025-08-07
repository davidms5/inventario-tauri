import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../modules/ventas/styles/combos.css'

type Combo = {
  id: number;
  nombre: string;
  descripcion?: string;
  price: number;
  enabled: boolean;
};

export default function Combos() {
  const [combos, setCombos] = useState<Combo[]>([]);
  const [form, setForm] = useState({ nombre: '', descripcion: '', price: 0, enabled: true });
  const [editId, setEditId] = useState<number | null>(null);

  const load = async () => {
    const result = await invoke<Combo[]>('list_combos');
    setCombos(result);
  };

  useEffect(() => { load(); }, []);

  const handleSubmit = async () => {
    if (editId) {
      await invoke('update_combo', { id: editId, ...form });
    } else {
      await invoke('create_combo', form);
    }
    setEditId(null);
    setForm({ nombre: '', descripcion: '', price: 0, enabled: true });
    load();
  };

  const handleDelete = async (id: number) => {
    await invoke('delete_combo', { id });
    load();
  };

  const startEdit = (c: Combo) => {
    setEditId(c.id);
    setForm({ nombre: c.nombre, descripcion: c.descripcion || '', price: c.price, enabled: c.enabled });
  };

  return (
    <div className="combo-container">
      <h2>Gestión de Combos</h2>
        
      <div className="form-section">
        <h3 style={{color: "black"}}>{editId ? 'Editar Combo' : 'Crear Combo'}</h3>
        <input placeholder="Nombre" value={form.nombre} onChange={e => setForm({ ...form, nombre: e.target.value })} />
        <input placeholder="Descripción" value={form.descripcion} onChange={e => setForm({ ...form, descripcion: e.target.value })} />
        <input type="number" placeholder="Precio total" value={form.price} onChange={e => setForm({ ...form, price: parseFloat(e.target.value) })} />
        <label>
          <input type="checkbox" checked={form.enabled} onChange={e => setForm({ ...form, enabled: e.target.checked })} />
          Activo
        </label>
        <button onClick={handleSubmit}>{editId ? 'Guardar' : 'Crear'}</button>
        {editId && <button onClick={() => {
          setEditId(null);
          setForm({ nombre: '', descripcion: '', price: 0, enabled: true });
        }}>Cancelar</button>}
      </div>

      <ul className="combo-list">
        {combos.map(c => (
          <li key={c.id}>
            <span>{c.nombre} — ${c.price.toFixed(2)} {c.enabled ? '' : '(Inactivo)'}</span>
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
