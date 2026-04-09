import { type EditorView, WidgetType } from '@codemirror/view';

export class HeadingWidget extends WidgetType {
  private readonly level: number;
  private readonly text: string;

  constructor(level: number, text: string) {
    super();
    this.level = level;
    this.text = text;
  }

  eq(other: HeadingWidget) {
    return other.level === this.level && other.text === this.text;
  }

  toDOM(_view: EditorView) {
    const wrap = document.createElement('div');
    wrap.className = `cm-livePreview-heading cm-livePreview-h${this.level}`;
    wrap.setAttribute('aria-hidden', 'true');
    wrap.textContent = this.text;
    return wrap;
  }

  ignoreEvent() {
    return false;
  }
}
