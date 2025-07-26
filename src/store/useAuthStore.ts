import { create } from "zustand";

interface AuthState {
  user: string | null;
  rol: string | null;
  setUser: (u: string | null) => void;
  setRol: (r: string | null) => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  rol: null,
  setRol: (r) => set({ rol: r }),
  setUser: (u) => set({ user: u }),
}));