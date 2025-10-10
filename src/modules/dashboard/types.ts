// src/types/ventas.ts
export type PaymentTotal = { forma_pago: string; monto: number };
export type TodaySummary = {
  ventas_count: number;
  total_dia: number;
  por_forma_pago: PaymentTotal[];
};
