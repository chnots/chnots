import { type EditorView, WidgetType } from '@codemirror/view';

export type ColumnAlign = 'left' | 'center' | 'right';

export function parseCells(line: string): string[] {
  return line
    .replace(/^\|/, '')
    .replace(/\|$/, '')
    .split('|')
    .map((cell) => cell.trim());
}

export function parseAlignment(delimiterLine: string): ColumnAlign[] {
  return delimiterLine
    .replace(/^\|/, '')
    .replace(/\|$/, '')
    .split('|')
    .map((cell) => {
      const trimmed = cell.trim();
      if (trimmed.startsWith(':') && trimmed.endsWith(':')) return 'center';
      if (trimmed.endsWith(':')) return 'right';
      return 'left';
    });
}

export function isDelimiterRow(line: string): boolean {
  return /^\|?\s*[-:]+[-|\s:]*$/.test(line.trim());
}

export interface TableEditDetail {
  rawText: string;
  from: number;
  to: number;
}

export const TABLE_EDIT_EVENT = 'cm-table-edit';

export class TableWidget extends WidgetType {
  private readonly rawText: string;
  private readonly from: number;
  private readonly to: number;

  constructor(rawText: string, from: number, to: number) {
    super();
    this.rawText = rawText;
    this.from = from;
    this.to = to;
  }

  eq(other: TableWidget) {
    return other.rawText === this.rawText && other.from === this.from;
  }

  toDOM(_view: EditorView) {
    const wrapper = document.createElement('div');
    wrapper.className = 'cm-livePreview-table-wrapper';

    const table = document.createElement('table');
    table.className = 'cm-livePreview-table';
    table.style.cursor = 'pointer';

    const lines = this.rawText.split('\n').filter((line) => line.trim());

    let hasHeader = false;
    let hasBody = false;
    let alignments: ColumnAlign[] = [];

    for (const line of lines) {
      if (isDelimiterRow(line)) {
        alignments = parseAlignment(line);
        continue;
      }

      const cells = parseCells(line);

      if (!hasHeader) {
        const thead = document.createElement('thead');
        const tr = document.createElement('tr');
        for (let i = 0; i < cells.length; i++) {
          const th = document.createElement('th');
          th.textContent = cells[i];
          const align = alignments[i] || 'left';
          if (align !== 'left') th.style.textAlign = align;
          tr.appendChild(th);
        }
        thead.appendChild(tr);
        table.appendChild(thead);
        hasHeader = true;
      } else {
        if (!hasBody) {
          const tbody = document.createElement('tbody');
          table.appendChild(tbody);
          hasBody = true;
        }
        const tbody = table.querySelector('tbody')!;
        const tr = document.createElement('tr');
        for (let i = 0; i < cells.length; i++) {
          const td = document.createElement('td');
          td.textContent = cells[i];
          const align = alignments[i] || 'left';
          if (align !== 'left') td.style.textAlign = align;
          tr.appendChild(td);
        }
        tbody.appendChild(tr);
      }
    }

    wrapper.appendChild(table);

    wrapper.addEventListener('mousedown', (e) => {
      e.preventDefault();
      e.stopPropagation();
    });

    wrapper.addEventListener('click', (e) => {
      e.preventDefault();
      e.stopPropagation();
      wrapper.dispatchEvent(
        new CustomEvent<TableEditDetail>(TABLE_EDIT_EVENT, {
          bubbles: true,
          detail: {
            rawText: this.rawText,
            from: this.from,
            to: this.to,
          },
        }),
      );
    });

    return wrapper;
  }

  ignoreEvent() {
    return true;
  }
}
