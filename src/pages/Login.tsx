// src/components/Login.tsx
import styles from "../modules/login/styles/Login.module.css"
import { useLogin } from '../modules/login/useLogin';

export default function Login({ onLogin }: { onLogin: (username: string) => void }) {

  const {
    username,
    password,
    error,
    setUsername,
    setPassword,
    handleLogin
  } = useLogin(onLogin);

  return (
    <form onSubmit={handleLogin} className={styles["login-form"]}>
      <h2>Iniciar sesión</h2>
      <input
        type="text"
        placeholder="Usuario"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        required
      />
      <input
        type="password"
        placeholder="Contraseña"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        required
      />
      {error && <p className={`${styles['error']} p`}>{error}</p>}
      <button type="submit">Entrar</button>
    </form>
  );
}
