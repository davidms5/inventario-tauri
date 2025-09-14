import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";
import styles from "../modules/usuarios/styles/Usuarios.module.css";

type User = {
  id: number;
  username: string;
  rol: string;
  enabled_add_products: boolean;
};

export default function Usuarios() {
  const [users, setUsers] = useState<User[]>([]);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [role, setRole] = useState("empleado");
  const [enabledAddProducts, setEnabledAddProducts] = useState("false");

  const [editUserId, setEditUserId] = useState<number | null>(null);
  const [editPassword, setEditPassword] = useState("");
  const [editRole, setEditRole] = useState("empleado");

  const navigate = useNavigate();

  const fetchUsers = async () => {
    try {
      const result = await invoke<User[]>("list_users");
      console.log("Usuarios cargados:", result);
      setUsers(result);
    } catch (error) {
      console.error("Error cargando usuarios", error);
    }
  };

  const handleCreate = async () => {
    try {
      const realValueEnabledAddProducts = enabledAddProducts === "true";
      await invoke("create_user", { new_username: username, new_password_hash: password, new_rol: role, new_enabled_add_products: realValueEnabledAddProducts });
      setUsername("");
      setPassword("");
      setRole("empleado");
      setEnabledAddProducts("false");
      fetchUsers();
    } catch (error) {
      console.error("Error creando usuario", error);
    }
  };

  const handleDelete = async (id: number) => {
    try {
      await invoke("delete_user", { target_id:id });
      fetchUsers();
    } catch (error) {
      console.error("Error eliminando usuario", error);
    }
  };

  const handleEdit = (user: User) => {
    setEditUserId(user.id);
    setEditPassword(""); // no mostramos hash actual, se cambia si se ingresa
    setEditRole(user.rol);
  };

  
  const handleUpdate = async () => {
    if (!editUserId) return;

    try {
      await invoke("update_user", {
        target_id: editUserId,
        plain_password: editPassword,
        new_rol: editRole,
        new_enabled_add_products: enabledAddProducts === "true",
      });

      setEditUserId(null);
      fetchUsers();
    } catch (error) {
      console.error("Error actualizando usuario", error);
    }
  };

  useEffect(() => {
    fetchUsers();
  }, []);

  return (
    <div className={styles['usuarios-container']}>
      <h2>Gestión de Usuarios</h2>

      {/* Crear usuario */}
      <div className={styles['form-section']}>
        <h3>Crear Usuario</h3>
        <button onClick={() => navigate("/dashboard")}>menu principal</button>
        <hr />
        <input
          type="text"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
        <input
          type="password"
          placeholder="Contraseña"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <select value={role} onChange={(e) => setRole(e.target.value)}>
          <option value="empleado">Empleado</option>
          <option value="admin">Admin</option>
        </select>

        <select onChange={(e) => setEnabledAddProducts(e.target.value)}>
            <option value="false">No puede ver combos</option>
            <option value="true">Puede ver combos</option>
        </select>
        <button onClick={handleCreate}>Crear</button>
      </div>

      {/* Listado */}
      {/* Lista de usuarios */}
      <div className={styles['list-section']}>
        <h3>Lista de Usuarios</h3>
        <ul className={styles['user-list']}>
          {users.map((user) => (
            <li key={user.id}>
              <div className={styles['user-info']}>
                {user.username} ({user.rol}) ver combos: {user.enabled_add_products ? 'sí' : 'no'}
              </div>
              
              <div className={styles['user-actions']}>
              <button onClick={() => handleDelete(user.id)} style={{ marginLeft: "10px" }}>
                Eliminar
              </button>
              <button onClick={() => handleEdit(user)} style={{ marginLeft: "10px" }}>
                Editar
              </button>
              </div>
            </li>
          ))}
        </ul>
      </div>

            {/* Formulario de edición */}
      {editUserId && (
        <div className={styles['edit-section']}>
          <h3>Editar Usuario</h3>
          <input
            type="password"
            placeholder="Nueva contraseña (opcional)"
            value={editPassword}
            onChange={(e) => setEditPassword(e.target.value)}
          />
          <select value={editRole} onChange={(e) => setEditRole(e.target.value)}>
            <option value="empleado">Empleado</option>
            <option value="admin">Admin</option>
          </select>
          <select value={enabledAddProducts} onChange={(e) => setEnabledAddProducts(e.target.value)}>
            <option value="false">No puede ver combos</option>
            <option value="true">Puede ver combos</option>
          </select>
          <button onClick={handleUpdate}>Guardar Cambios</button>
          <button onClick={() => setEditUserId(null)} style={{ marginLeft: "10px" }}>
            Cancelar
          </button>
        </div>
      )}
    </div>
  );
}
