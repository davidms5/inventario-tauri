// src/modules/ventas/utils/pdf.ts
import { PDFDocument, StandardFonts } from 'pdf-lib';
import { save } from '@tauri-apps/plugin-dialog';
import { writeFile } from '@tauri-apps/plugin-fs';
import { desktopDir, join } from '@tauri-apps/api/path';

// Tipos mínimos (ajustá a tu shape real)
type Sale = {
  id: number; user_id: number; fecha: string;
  total: number; forma_pago: string; estado: string;
};
type SaleItem = {
  id: number;
  product_id?: number | null;
  combo_id?: number | null;
  cantidad: number;
  precio_unitario: number;
  nombre: string;
};
type SaleWithItems = { sale: Sale; items: SaleItem[] };

// Genera bytes PDF y los guarda donde el usuario elija
export async function generateAndSaveSalePdf(
  sale: SaleWithItems,
  change: number = 0
) {
  const pdf = await PDFDocument.create();
  let page = pdf.addPage([595.28, 841.89]); // A4
  const font = await pdf.embedFont(StandardFonts.Helvetica);
  const fontBold = await pdf.embedFont(StandardFonts.HelveticaBold);

  const margin = 40;
  let y = 812;

  const draw = (text: string, x: number, size = 12, bold = false) => {
    page.drawText(text, { x, y, size, font: bold ? fontBold : font });
  };
  const line = () => {
    y -= 8;
    page.drawLine({ start: { x: margin, y }, end: { x: 595.28 - margin, y }, thickness: 1 });
    y -= 12;
  };

  // Header
  draw('Comprobante de Venta', margin, 18, true); y -= 24;
  draw(`Venta #${sale.sale.id}`, margin); y -= 16;
  draw(`Fecha: ${sale.sale.fecha}`, margin); y -= 16;
  draw(`Pago: ${sale.sale.forma_pago}    Estado: ${sale.sale.estado}`, margin); y -= 10;
  line();

  // Encabezado de tabla
  draw('Tipo', margin, 12, true);
  draw('Nombre', margin + 80, 12, true);
  draw('Cant.', margin + 300, 12, true);
  draw('Precio U.', margin + 360, 12, true);
  draw('Subtotal', margin + 450, 12, true);
  y -= 18;

  // Filas
  for (const it of sale.items) {
    const tipo = it.combo_id ? 'Combo' : 'Producto';
    const nombre = it.combo_id ? `Combo ${it.nombre}` : `Prod ${it.nombre}`;
    const subtotal = (it.precio_unitario * it.cantidad).toFixed(2);

    draw(tipo, margin);
    draw(nombre, margin + 80);
    draw(String(it.cantidad), margin + 300);
    draw(`$${it.precio_unitario.toFixed(2)}`, margin + 360);
    draw(`$${subtotal}`, margin + 450);
    y -= 16;

    if (y < 80) { // nueva página si hace falta
      y = 812;
      const p = pdf.addPage([595.28, 841.89]);
      (page as any) = p;
    }
  }

  line();
  draw(`TOTAL: $${sale.sale.total.toFixed(2)}`, margin + 400, 14, true); y -= 18;
  if (change > 0) {
    draw(`Cambio: $${change.toFixed(2)}`, margin + 400, 12);
    y -= 14;
  }

  const bytes = await pdf.save(); // Uint8Array listo para escribir :contentReference[oaicite:1]{index=1}

  // Sugerimos Desktop y nombre
  const suggested = await join(await desktopDir(), `venta_${sale.sale.id}.pdf`);
  // Abrimos diálogo de guardar: agrega esa ruta al scope automáticamente :contentReference[oaicite:2]{index=2}
  const dest = await save({
    title: 'Guardar comprobante',
    defaultPath: suggested,
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
  });
  if (!dest) return;

  // Escribimos archivo (podés pasar Uint8Array directo) :contentReference[oaicite:3]{index=3}
  await writeFile(dest, bytes);
}
