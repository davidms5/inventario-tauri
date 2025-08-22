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

type Page<T> = {
  data: T[];
  total: number;
  total_pages: number;
  current_page: number;
  per_page: number;
};

export default function Inventario() {
  const [products, setProducts] = useState<Product[]>([]);
  const [page, setPage] = useState(1);
  const [perPage, setPerPage] = useState(10);
  const [q, setQ] = useState("");
  const [totalPages, setTotalPages] = useState(1);

  const [form, setForm] = useState<Omit<Product, 'id'>>({
    nombre: "",
    sku: "",
    descripcion: "",
    price: 0,
    quantity: 0,
    category: ""
  });

  const [editId, setEditId] = useState<number | null>(null);

  const load = async (p = page) => {
    const res = await invoke<Page<Product>>('list_products_paginated', {
      page: p,
      perPage,
      q: q || null,
    });
    setProducts(res.data);
    setTotalPages(res.total_pages);
    setPage(res.current_page);
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

  useEffect(() => { load(1); }, [perPage, q]);

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

      <div className={styles['two-col']}>
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

          {/* Controles */}
            <div style={{marginTop: "20px"}}>
            <button disabled={page<=1} onClick={()=>load(page-1)}>« Anterior</button>
            <span>Página {page} de {totalPages}</span>
            <button disabled={page>=totalPages} onClick={()=>load(page+1)}>Siguiente »</button>

            <select style={{marginLeft: "5px"}} value={perPage} onChange={e => setPerPage(parseInt(e.target.value))}>
              <option value={10}>10</option>
              <option value={25}>25</option>
              <option value={50}>50</option>
            </select>
            <input style={{marginTop: "10px"}} placeholder="Buscar..." value={q} onChange={e => setQ(e.target.value)} />
            </div>
        </div>
      </div>
    </div>
  );
}
