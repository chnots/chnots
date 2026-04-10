import { type EditorView, WidgetType } from '@codemirror/view';
import katex from 'katex';

export class InlineMathWidget extends WidgetType {
  private readonly latex: string;
  private readonly displayMode: boolean;

  constructor(latex: string, displayMode: boolean = false) {
    super();
    this.latex = latex;
    this.displayMode = displayMode;
  }

  eq(other: InlineMathWidget) {
    return other.latex === this.latex && other.displayMode === this.displayMode;
  }

  toDOM(_view: EditorView) {
    const span = document.createElement('span');
    span.className = 'cm-livePreview-math cm-livePreview-math-inline';
    span.setAttribute('aria-hidden', 'true');

    try {
      katex.render(this.latex, span, {
        throwOnError: false,
        displayMode: this.displayMode,
      });
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
      katex.render(this.latex, wrap, {
        throwOnError: false,
        displayMode: true,
      });
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
