import { create } from "zustand";

interface AuthState {
  user: string | null;
  rol: string | null;
  user_id: number | null;
  setUser: (u: string | null) => void;
  setRol: (r: string | null) => void;
  setUserId: (id: number | null) => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  rol: null,
  user_id: null,
  setRol: (r) => set({ rol: r }),
  setUser: (u) => set({ user: u }),
  setUserId: (id) => set({ user_id: id }),
}));