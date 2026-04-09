import { type EditorView, WidgetType } from '@codemirror/view';

export class InlineMathWidget extends WidgetType {
  private readonly latex: string;

  constructor(latex: string) {
    super();
    this.latex = latex;
  }

  eq(other: InlineMathWidget) {
    return other.latex === this.latex;
  }

  toDOM(_view: EditorView) {
    const span = document.createElement('span');
    span.className = 'cm-livePreview-math cm-livePreview-math-inline';
    span.setAttribute('aria-hidden', 'true');

    try {
      const katex = (window as any).katex;
      if (katex) {
        katex.render(this.latex, span, {
          throwOnError: false,
          displayMode: false,
        });
      } else {
        span.textContent = this.latex;
        span.classList.add('cm-livePreview-math-raw');
      }
    } catch {
      span.textContent = this.latex;
      span.classList.add('cm-livePreview-math-raw');
    }

    return span;
  }

  ignoreEvent() {
    return false;
  }
}

export class BlockMathWidget extends WidgetType {
  private readonly latex: string;

  constructor(latex: string) {
    super();
    this.latex = latex;
  }

  eq(other: BlockMathWidget) {
    return other.latex === this.latex;
  }

  toDOM(_view: EditorView) {
    const wrap = document.createElement('div');
    wrap.className = 'cm-livePreview-math cm-livePreview-math-block';
    wrap.setAttribute('aria-hidden', 'true');

    try {
      const katex = (window as any).katex;
      if (katex) {
        katex.render(this.latex, wrap, {
          throwOnError: false,
          displayMode: true,
        });
      } else {
        wrap.textContent = this.latex;
        wrap.classList.add('cm-livePreview-math-raw');
      }
    } catch {
      wrap.textContent = this.latex;
      wrap.classList.add('cm-livePreview-math-raw');
    }

    return wrap;
  }

  ignoreEvent() {
    return false;
  }
}
