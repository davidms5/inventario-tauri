import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router-dom";

type User = {
  id: number;
  username: string;
  rol: string;
};

export default function Usuarios() {
  const [users, setUsers] = useState<User[]>([]);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [role, setRole] = useState("empleado");

  const [editUserId, setEditUserId] = useState<number | null>(null);
  const [editPassword, setEditPassword] = useState("");
  const [editRole, setEditRole] = useState("empleado");

  const navigate = useNavigate();

  const fetchUsers = async () => {
    try {
      const result = await invoke<User[]>("list_users");
      setUsers(result);
    } catch (error) {
      console.error("Error cargando usuarios", error);
    }
  };

  const handleCreate = async () => {
    try {
      await invoke("create_user", { username, password_hash: password, rol: role });
      setUsername("");
      setPassword("");
      setRole("empleado");
      fetchUsers();
    } catch (error) {
      console.error("Error creando usuario", error);
    }
  };

  const handleDelete = async (id: number) => {
    try {
      await invoke("delete_user", { id });
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
        id: editUserId,
        password_hash: editPassword,
        rol: editRole,
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
    <div>
      <h2>Gestión de Usuarios</h2>

      {/* Crear usuario */}
      <div>
        <h3>Crear Usuario</h3>
        <button onClick={() => navigate("/dashboard")}>dashboard</button>
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
        <button onClick={handleCreate}>Crear</button>
      </div>

      {/* Listado */}
      {/* Lista de usuarios */}
      <div>
        <h3>Lista de Usuarios</h3>
        <ul>
          {users.map((user) => (
            <li key={user.id}>
              {user.username} ({user.rol})
              <button onClick={() => handleDelete(user.id)} style={{ marginLeft: "10px" }}>
                Eliminar
              </button>
              <button onClick={() => handleEdit(user)} style={{ marginLeft: "10px" }}>
                Editar
              </button>
            </li>
          ))}
        </ul>
      </div>

            {/* Formulario de edición */}
      {editUserId && (
        <div>
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
          <button onClick={handleUpdate}>Guardar Cambios</button>
          <button onClick={() => setEditUserId(null)} style={{ marginLeft: "10px" }}>
            Cancelar
          </button>
        </div>
      )}
    </div>
  );
}
