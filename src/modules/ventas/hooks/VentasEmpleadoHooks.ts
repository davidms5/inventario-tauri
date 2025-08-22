// src/modules/ventas/hooks/useNuevaVenta.ts
import { useEffect, useMemo, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

// —— Tipos ——
export type Combo = { id: number; nombre: string; price: number };
export type Product = { id: number; nombre: string; price: number; quantity: number };

export type CartItem =
  | { kind: 'combo'; combo_id: number; nombre: string; cantidad: number; price: number }
  | { kind: 'product'; product_id: number; nombre: string; cantidad: number; price: number };

type Pago = 'efectivo' | 'tarjeta' | 'transferencia' | 'mercado_pago';

// —— Hook ——
export function useNuevaVenta(opts?: { userId?: number| null }) {
  const userId = opts?.userId ?? 1; // trae de tu store si querés

  // catálogos
  const [combos, setCombos] = useState<Combo[]>([]);
  const [products, setProducts] = useState<Product[]>([]);

  // búsqueda
  const [searchTerm, setSearchTerm] = useState('');

  // selección para agregar
  const [selCombo, setSelCombo] = useState<number | ''>('');
  const [selProduct, setSelProduct] = useState<number | ''>('');
  const [cantCombo, setCantCombo] = useState<number>(1);
  const [cantProd, setCantProd] = useState<number>(1);

  // carrito
  const [cart, setCart] = useState<CartItem[]>([]);

  // pago
  const [pago, setPago] = useState<Pago>('efectivo');
  const [cashReceived, setCashReceived] = useState<number | ''>('');

  // carga catálogos
  const loadCombos = async () => {
    const res = await invoke<Combo[]>('list_active_combos');
    setCombos(res);
  };
  // BUSCAR productos
  const searchProducts = async () => {
    const q = searchTerm.trim();
    if (q.length < 1) {         // puedes ajustar el mínimo
      setProducts([]);
      setSelProduct('');
      return;
    }
    const res = await invoke<Product[]>('search_products_in_stock', { query: q });
    setProducts(res);
    setSelProduct('');
    setCantProd(1);
  };

  useEffect(() => {
    loadCombos();
    //loadProducts();
  }, []);

  // totales
  const total = useMemo(
    () => cart.reduce((acc, it) => acc + it.price * it.cantidad, 0),
    [cart]
  );

  const change = useMemo(() => {
    if (pago !== 'efectivo') return 0;
    if (typeof cashReceived !== 'number') return 0;
    return Math.max(0, cashReceived - total);
  }, [pago, cashReceived, total]);

  // agregar líneas
  const addCombo = () => {
    if (!selCombo || cantCombo <= 0) return;
    const c = combos.find(x => x.id === selCombo);
    if (!c) return;

    setCart(prev => {
      const idx = prev.findIndex(it => it.kind === 'combo' && it.combo_id === c.id);
      if (idx >= 0) {
        const clone = [...prev];
        const line = clone[idx] as Extract<CartItem, { kind: 'combo' }>;
        clone[idx] = { ...line, cantidad: line.cantidad + cantCombo };
        return clone;
      }
      return [...prev, { kind: 'combo', combo_id: c.id, nombre: c.nombre, cantidad: cantCombo, price: c.price }];
    });
    setCantCombo(1);
    setSelCombo('');
  };

  const addProduct = () => {
    if (!selProduct || cantProd <= 0) return;
    const p = products.find(x => x.id === selProduct);
    if (!p) return;
    if (cantProd > p.quantity) {
      alert('Cantidad supera el stock disponible');
      return;
    }
    setCart(prev => {
      const idx = prev.findIndex(it => it.kind === 'product' && it.product_id === p.id);
      if (idx >= 0) {
        const clone = [...prev];
        const line = clone[idx] as Extract<CartItem, { kind: 'product' }>;
        clone[idx] = { ...line, cantidad: line.cantidad + cantProd };
        return clone;
      }
      return [...prev, { kind: 'product', product_id: p.id, nombre: p.nombre, cantidad: cantProd, price: p.price }];
    });
    setCantProd(1);
    setSelProduct('');
  };

  const removeLine = (index: number) =>
    setCart(prev => prev.filter((_, i) => i !== index));

  const reset = async () => {
    setCart([]);
    setCashReceived('');
    setPago('efectivo');
    setSelCombo('');
    setSelProduct('');
    setCantCombo(1);
    setCantProd(1);
    //await loadProducts();
  };

  // confirmación
  const confirmDisabled = !cart.length;
    //!cart.length || (pago === 'efectivo' && (typeof cashReceived !== 'number' || cashReceived < total));

  const confirmSale = async () => {

    if (!cart.length) return;

    const items = cart.map(it =>
      it.kind === 'product'
        ? { product_id: it.product_id, combo_id: null, cantidad: it.cantidad }
        : { product_id: null, combo_id: it.combo_id, cantidad: it.cantidad }
    );

    const payload = { user_id: userId, forma_pago: pago, items };
    const sale_id = await invoke<number>('create_sale', { payload });

    return {
      sale_id,
      total,
      change: pago === 'efectivo' ? change : 0,
    };
  };

  return {
    // catálogos
    combos, products, loadCombos,
    // búsqueda
    searchTerm, setSearchTerm, searchProducts,
    // selección
    selCombo, setSelCombo, cantCombo, setCantCombo,
    selProduct, setSelProduct, cantProd, setCantProd,

    // carrito
    cart, addCombo, addProduct, removeLine,

    // totales
    total, change,

    // pago
    pago, setPago, cashReceived, setCashReceived,

    // confirmar/reset
    confirmSale, confirmDisabled, reset,
  };
}
