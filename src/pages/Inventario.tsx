import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import { save } from "@tauri-apps/plugin-dialog";
import { desktopDir, join } from '@tauri-apps/api/path';
import styles from "../modules/productos/styles/Inventario.module.css";

type Product = {
  id: number;
  nombre: string;
  sku?: string;
  descripcion?: string;
  price: number;
  quantity: number;
  category?: string;
};

export default function Inventario() {
  const [products, setProducts] = useState<Product[]>([]);
  const [form, setForm] = useState<Omit<Product, 'id'>>({
    nombre: "",
    sku: "",
    descripcion: "",
    price: 0,
    quantity: 0,
    category: ""
  });

  const [editId, setEditId] = useState<number | null>(null);

  const load = async () => {
    const list = await invoke<Product[]>("list_products");
    setProducts(list);
  };
    const navigate = useNavigate();
  const handleSubmit = async () => {
    const payload = {
      ...form,
      sku: form.sku || null,
      description: form.descripcion || null,
      category: form.category || null
    };
    if (editId) {
      await invoke("update_product", { id: editId, ...payload });
    } else {
      await invoke("create_product", payload);
    }
    setEditId(null);
    setForm({ nombre: "", sku: "", descripcion: "", price: 0, quantity: 0, category: "" });
    load();
  };

  const handleDelete = async (id: number) => {
    await invoke("delete_product", { target_id: id });
    load();
  };

  const startEdit = (p: Product) => {
    setEditId(p.id);
    setForm({ ...p });
  };

  useEffect(() => { load(); }, []);

  const downloadCSV = async () => {
    try {
      const destPath = await save({
        title: 'Guardar CSV',
        defaultPath: await join(await desktopDir(), 'products_export.csv'),
        filters: [{ name: 'CSV', extensions: ['csv'] }],
      });
      if (!destPath) return;

      await invoke('export_table_to_csv', { path: destPath });

      alert('CSV exportado correctamente.');
    } catch (err) {
      console.error(err);
      alert('Error exportando CSV');
    }
  };

  return (
    <div className={styles['inventario-container']}>
      <h2>Productos</h2>
      <div className={styles.toolbar}>
        <button onClick={() => navigate("/dashboard")}>dashboard</button>
       <button onClick={() => downloadCSV()}>descargar csv</button>
      </div>
    
      <fieldset className={styles['form-section']}>
        <legend>{editId ? "Editar Producto" : "Nuevo Producto"}</legend>
        <p>nombre</p>
        <input value={form.nombre} placeholder="Nombre" onChange={e => setForm({ ...form, nombre: e.target.value })} />
        <p>id unico</p>
        <input value={form.sku} placeholder="SKU" onChange={e => setForm({ ...form, sku: e.target.value })} />
        <p>descripcion</p>
        <input value={form.descripcion} placeholder="Descripción" onChange={e => setForm({ ...form, descripcion: e.target.value })} />
        <p>precio</p>
        <input type="number" value={form.price} placeholder="Precio" onChange={e => setForm({ ...form, price: parseFloat(e.target.value) })} />
        <p>cantidad</p>
        <input type="number" value={form.quantity} placeholder="Cantidad" onChange={e => setForm({ ...form, quantity: parseInt(e.target.value) })} />
        <p>categoria</p>
        <input value={form.category} placeholder="Categoría" onChange={e => setForm({ ...form, category: e.target.value })} />
        <button onClick={handleSubmit}>{editId ? "Guardar Cambios" : "Crear"}</button>
        {editId && <button onClick={() => { setEditId(null); setForm({ nombre: "", sku: "", descripcion: "", price: 0, quantity: 0, category: "" }); }}>Cancelar</button>}
      </fieldset>

      <div className={styles['table-section']}>
        <table className={styles['products-table']}>
          <thead>
            <tr><th>Nombre</th><th>SKU</th><th>Precio</th><th>Cantidad</th><th>Categoría</th><th>Acciones</th></tr>
          </thead>
          <tbody>
            {products.map(p => (
              <tr key={p.id}>
                <td>{p.nombre}</td><td>{p.sku}</td><td>${p.price}</td><td>{p.quantity}</td><td>{p.category}</td>
                <td>
                  <button onClick={() => startEdit(p)}>Editar</button>
                  <button onClick={() => handleDelete(p.id)}>Eliminar</button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

    </div>
  );
}
