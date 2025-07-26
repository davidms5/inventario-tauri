import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
//import { open } from "@tauri-apps/plugin-dialog";
//import { readBinaryFile, writeBinaryFile, BaseDirectory } from "@tauri-apps/plugin-fs";

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
    await invoke("delete_product", { id });
    load();
  };

  const startEdit = (p: Product) => {
    setEditId(p.id);
    setForm({ ...p });
  };

  useEffect(() => { load(); }, []);

    //const downloadCSV = async () => {
    //    try {
    //        const filePath: string = await invoke("export_products_csv");
    //        const dest = await open({
    //        title: "Guardar CSV",
    //        defaultPath: "products_export.csv",
    //        filters: [{ name: "CSV", extensions: ["csv"] }],
    //        multiple: false,
    //        });
    //        if (!dest) return;
    //        const content = await readBinaryFile(filePath, { baseDir: BaseDirectory.AppConfig });
    //        await writeBinaryFile(dest, content);
    //        alert("CSV exportado correctamente.");
    //    } catch (err) {
    //        console.error(err);
    //        alert("Error al exportar CSV");
    //    }
    //};

  return (
    <div>
      <h2>Productos</h2>
        <button onClick={() => navigate("/dashboard")}>dashboard</button>
        <button>descargar csv</button>
        <hr />
      <fieldset>
        <legend>{editId ? "Editar Producto" : "Nuevo Producto"}</legend>
        <input value={form.nombre} placeholder="Nombre" onChange={e => setForm({ ...form, nombre: e.target.value })} />
        <input value={form.sku} placeholder="SKU" onChange={e => setForm({ ...form, sku: e.target.value })} />
        <input value={form.descripcion} placeholder="Descripción" onChange={e => setForm({ ...form, descripcion: e.target.value })} />
        <input type="number" value={form.price} placeholder="Precio" onChange={e => setForm({ ...form, price: parseFloat(e.target.value) })} />
        <input type="number" value={form.quantity} placeholder="Cantidad" onChange={e => setForm({ ...form, quantity: parseInt(e.target.value) })} />
        <input value={form.category} placeholder="Categoría" onChange={e => setForm({ ...form, category: e.target.value })} />
        <button onClick={handleSubmit}>{editId ? "Guardar Cambios" : "Crear"}</button>
        {editId && <button onClick={() => { setEditId(null); setForm({ nombre: "", sku: "", descripcion: "", price: 0, quantity: 0, category: "" }); }}>Cancelar</button>}
      </fieldset>

      <table>
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
  );
}
