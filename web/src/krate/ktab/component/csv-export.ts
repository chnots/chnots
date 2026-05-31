import type { KTabMeta } from "../po";
import type { KTabRowData } from "./ktab-table";

const csvEscape = (val: unknown): string => {
  const str = val == null ? "" : String(val);
  if (str.includes(",") || str.includes('"') || str.includes("\n")) {
    return `"${str.replace(/"/g, '""')}"`;
  }
  return str;
};

export async function exportCsv(
  meta: KTabMeta,
  fetchData: (
    tableId: number,
    start: number,
    size: number,
  ) => Promise<KTabRowData[]>,
): Promise<void> {
  // Fetch all data
  const allRows = await fetchData(meta.otid, 0, 10000);
  const columns = Object.values(meta.columns).sort(
    (a, b) => a.order_by - b.order_by,
  );

  // Header row
  const header = columns.map((c) => csvEscape(c.name)).join(",");

  // Data rows
  const dataLines = allRows.map((row) =>
    columns.map((col) => csvEscape(row[col.name])).join(","),
  );

  const csv = [header, ...dataLines].join("\n");
  const blob = new Blob([csv], { type: "text/csv;charset=utf-8;" });
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = `${meta.table_name || "ktab-export"}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
