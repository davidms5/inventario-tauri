import { create } from "zustand";

interface AuthState {
  user: string | null;
  rol: string | null;
  user_id: number | null;
  enabled_add_products?: boolean | null;
  setUser: (u: string | null) => void;
  setRol: (r: string | null) => void;
  setUserId: (id: number | null) => void;
  setEnabledAddProducts: (enabled: boolean | null) => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  rol: null,
  user_id: null,
  enabled_add_products: null,
  setEnabledAddProducts: (enabled) => set({ enabled_add_products: enabled }),
  setRol: (r) => set({ rol: r }),
  setUser: (u) => set({ user: u }),
  setUserId: (id) => set({ user_id: id }),
}));