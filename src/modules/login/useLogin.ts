// src/modules/login/hooks/useLogin.ts
import { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { LoginSchema } from './loginSchema';
import { useNavigate } from 'react-router-dom';
import { UserInfo } from './types';
import { useAuthStore } from '../../store/useAuthStore';

export function useLogin(onLogin: (username: string) => void) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const navigate = useNavigate();
  const setRol = useAuthStore((state) => state.setRol);
  const setUserId = useAuthStore((state) => state.setUserId);
  const setEnabledAddProducts = useAuthStore((state) => state.setEnabledAddProducts);

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    const formData = {
      username,
      password_hash: password,
    };

    const result = LoginSchema.safeParse(formData);

    if (!result.success) {
      const firstError = result.error.issues[0]?.message || 'Datos inválidos';
      setError(firstError);
      return;
    }

    try {
      const success = await invoke<UserInfo| null>('check_login', result.data);

      if (success) {
        console.log('Login exitoso:', success);
        onLogin(success.username);
        setRol(success.rol);
        setUserId(success.id);
        setEnabledAddProducts(success.enabled_add_products);
        navigate("/dashboard");
      } else {
        setError('Usuario o contraseña incorrectos');
      }

    } catch (err) {
      console.error('Error al comunicarse con Tauri:', err);
      setError('Error interno del sistema');
    }
  };

  return {
    username,
    password,
    error,
    setUsername,
    setPassword,
    handleLogin,
  };
}
