import { type EditorView, WidgetType } from '@codemirror/view';

export class ImageWidget extends WidgetType {
  private readonly url: string;
  private readonly alt: string;

  constructor(url: string, alt: string) {
    super();
    this.url = url;
    this.alt = alt;
  }

  eq(other: ImageWidget) {
    return other.url === this.url && other.alt === this.alt;
  }

  toDOM(_view: EditorView) {
    const wrap = document.createElement('div');
    wrap.className = 'cm-livePreview-image';
    wrap.setAttribute('aria-hidden', 'true');

    const img = wrap.appendChild(document.createElement('img'));
    img.src = this.url;
    img.alt = this.alt;
    img.loading = 'lazy';
    img.style.maxWidth = '100%';
    img.style.borderRadius = '4px';
    img.style.display = 'block';

    img.onerror = () => {
      wrap.classList.add('cm-livePreview-image-error');
      const errSpan = wrap.appendChild(document.createElement('span'));
      errSpan.className = 'cm-livePreview-image-errorText';
      errSpan.textContent = this.alt ? `![${this.alt}](${this.url})` : this.url;
      img.style.display = 'none';
    };

    return wrap;
  }

  ignoreEvent() {
    return false;
  }
}

const imageRE = /!\[([^\]]*)\]\(([^)]+)\)/;

export function parseImage(text: string): { alt: string; url: string } | null {
  const m = imageRE.exec(text);
  if (!m) return null;
  return { alt: m[1], url: m[2] };
}
